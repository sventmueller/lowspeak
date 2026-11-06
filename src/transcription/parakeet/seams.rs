//! Clean-up where merged windows meet: duplicated seam words and speech the merge dropped.
//!
//! FluidAudio applies both to the whole transcript; here they only act at seams, so that a genuine
//! "Essen essen" or a long pause inside one window is left alone. A word pair is at a seam when its
//! tokens come from more than one window, since a merge may also splice at punctuation inside a
//! word; a gap is at a seam when it reaches into audio that two neighbouring windows both decoded.

use std::collections::HashSet;
use std::ops::Range;

use anyhow::Result;

use super::chunks::{WindowDecoder, WindowRequest};
use super::energy::SpeechGate;
use super::layout::OVERLAP_SECONDS;
use super::text::{self, Vocabulary};
use super::{MAX_MODEL_SAMPLES, MEL_HOP, SAMPLES_PER_FRAME, SECONDS_PER_FRAME, Token};
use crate::util::numeric::{floor_i64, signed, unsigned};

/// A seam gap is re-decoded when it lasts 1.5 s and holds 0.5 s of speech-like audio.
const SEAM_GAP_MIN_GAP_SECONDS: f64 = 1.5;
const SEAM_GAP_MIN_SPEECH_SECONDS: f64 = 0.5;
const MAX_SEAM_GAP_REPAIRS: usize = 32;
/// Repairs run again for what is left of a partly recovered gap.
const REPAIR_PASSES: usize = 3;
/// Lowspeak's own cap: each probe tries up to two window placements, so this lets every seam be
/// probed once while repairs cost at most twice the decoding of the windows themselves.
const REPAIR_DECODES_PER_SEAM: usize = 2;
/// Recovered words this close to the word bordering the gap are that word heard again.
const EDGE_TOLERANCE_FRAMES: i64 = 6;

/// Drops an adjacent case-only duplicate of a seam word ("…have Have a…").
pub fn collapse_seam_word_duplicates(tokens: Vec<Token>, vocabulary: &Vocabulary) -> Vec<Token> {
    if tokens.len() < 2 {
        return tokens;
    }
    let overlap_frames = floor_i64((OVERLAP_SECONDS / SECONDS_PER_FRAME).round());

    struct Word {
        tokens: Vec<Token>,
        core: String,
        ends_sentence: bool,
    }
    impl Word {
        fn start(&self) -> i64 {
            self.tokens[0].timestamp
        }
    }
    let at_seam = |previous: &Word, current: &Word| {
        let window = previous.tokens[0].window_start;
        previous
            .tokens
            .iter()
            .chain(&current.tokens)
            .any(|token| token.window_start != window)
    };

    let mut words: Vec<Word> = Vec::new();
    for token in tokens {
        match words.last_mut() {
            Some(word) if !text::starts_word(vocabulary.piece(token.id)) => word.tokens.push(token),
            _ => words.push(Word {
                tokens: vec![token],
                core: String::new(),
                ends_sentence: false,
            }),
        }
    }
    for word in &mut words {
        let joined: String = word
            .tokens
            .iter()
            .map(|token| text::strip_word_boundary(vocabulary.piece(token.id)))
            .collect();
        word.core = joined
            .trim_matches(|c| text::is_punctuation(c) || text::is_whitespace(c))
            .to_owned();
        word.ends_sentence = joined.chars().last().is_some_and(|c| ".?!:".contains(c));
    }

    let mut keep = vec![true; words.len()];
    let mut last_kept: Option<usize> = None;
    for index in 0..words.len() {
        let Some(previous_index) = last_kept else {
            last_kept = Some(index);
            continue;
        };
        let previous = &words[previous_index];
        let current = &words[index];
        let is_duplicate = at_seam(previous, current)
            && !previous.core.is_empty()
            && !current.core.is_empty()
            && previous.core != current.core
            && previous.core.to_lowercase() == current.core.to_lowercase()
            && current.core.chars().next().is_some_and(char::is_alphabetic)
            && !previous.ends_sentence
            && current.start() - previous.start() <= overlap_frames;
        if !is_duplicate {
            last_kept = Some(index);
            continue;
        }
        // Keep the lower-case copy, else the earlier one.
        let current_is_lower = current.core == current.core.to_lowercase();
        let previous_is_lower = previous.core == previous.core.to_lowercase();
        if current_is_lower && !previous_is_lower {
            keep[previous_index] = false;
            last_kept = Some(index);
        } else {
            keep[index] = false;
        }
    }
    words
        .into_iter()
        .zip(keep)
        .filter(|(_, keep)| *keep)
        .flat_map(|(word, _)| word.tokens)
        .collect()
}

/// Re-decodes gaps at seams that still contain speech after merging, each with a fresh window, and
/// splices in what that window hears inside the gap. `seams` are the frames that two neighbouring
/// windows both decoded.
pub fn repair_seam_gaps(
    samples: &[f32],
    tokens: Vec<Token>,
    decoder: &dyn WindowDecoder,
    vocabulary: &Vocabulary,
    splice_safe: &HashSet<usize>,
    seams: &[Range<i64>],
) -> Result<Vec<Token>> {
    let frame = SAMPLES_PER_FRAME;
    let total = samples.len();
    let min_gap_frames = 2.max(floor_i64(SEAM_GAP_MIN_GAP_SECONDS / SECONDS_PER_FRAME));
    let window_samples = frame.max((MAX_MODEL_SAMPLES - MEL_HOP) / frame * frame);
    let gate = SpeechGate::new(samples);

    let mut working = tokens;
    let mut probes = 0;
    let mut decodes_left = seams.len() * REPAIR_DECODES_PER_SEAM;
    let mut probed_gap_starts = HashSet::new();
    for _ in 0..REPAIR_PASSES {
        let mut inserts = Vec::new();
        for index in 0..working.len().saturating_sub(1) {
            if probes >= MAX_SEAM_GAP_REPAIRS || decodes_left == 0 {
                break;
            }
            let (current, next) = (working[index], working[index + 1]);
            let gap_start_frame = current.timestamp + current.duration.max(1);
            let gap_end_frame = next.timestamp;
            if gap_end_frame - gap_start_frame < min_gap_frames
                || probed_gap_starts.contains(&gap_start_frame)
                || !seams
                    .iter()
                    .any(|seam| gap_start_frame < seam.end && seam.start < gap_end_frame)
            {
                continue;
            }
            let gap_start = unsigned(gap_start_frame) * frame;
            let gap_end = (unsigned(gap_end_frame) * frame).min(total);
            if gap_end <= gap_start
                || gate.speech_seconds(samples, gap_start, gap_end) < SEAM_GAP_MIN_SPEECH_SECONDS
            {
                continue;
            }
            probed_gap_starts.insert(gap_start_frame);
            probes += 1;

            // Cold-start at the gap first; replaying the noise before it can blank the decoder again.
            let center = (gap_start + gap_end) / 2;
            let placements = [
                signed(gap_start),
                signed(center) - signed(window_samples / 2),
            ];
            let lead = word_neighbor(&working, index, Direction::Back, vocabulary);
            let tail = word_neighbor(&working, index + 1, Direction::Forward, vocabulary);
            for placement in placements {
                if decodes_left == 0 {
                    break;
                }
                let latest = signed(total) - signed(window_samples);
                let window_start = unsigned(placement.min(latest).max(0)) / frame * frame;
                let window_end = (window_start + window_samples).min(total);
                if window_end <= window_start {
                    continue;
                }
                decodes_left -= 1;
                let window_tokens = decoder.decode_window(WindowRequest {
                    samples: &samples[window_start..window_end],
                    context_samples: 0,
                    chunk_start: window_start,
                    is_last_chunk: window_end >= total,
                    emit_tokens_after_frame: None,
                    initial_time_index_override: None,
                })?;
                let candidate = splice_candidate(
                    &window_tokens,
                    gap_start_frame,
                    gap_end_frame,
                    lead,
                    tail,
                    splice_safe,
                    vocabulary,
                );
                if !candidate.is_empty() {
                    inserts.extend(candidate);
                    break;
                }
            }
        }
        if inserts.is_empty() {
            break;
        }
        working.extend(inserts);
        working.sort_by_key(|token| token.timestamp);
    }
    Ok(working)
}

#[derive(Clone, Copy)]
enum Direction {
    Back,
    Forward,
}

/// The token at `index`, or the nearest one in `direction` past punctuation and symbols.
fn word_neighbor(
    stream: &[Token],
    index: usize,
    direction: Direction,
    vocabulary: &Vocabulary,
) -> Token {
    let mut neighbor = index;
    while text::is_punctuation_or_symbol_piece(vocabulary.piece(stream[neighbor].id)) {
        let next = match direction {
            Direction::Back => neighbor.checked_sub(1),
            Direction::Forward => Some(neighbor + 1).filter(|&next| next < stream.len()),
        };
        let Some(next) = next else { break };
        neighbor = next;
    }
    stream[neighbor]
}

/// The tokens of a re-decoded window that may go into the gap: strictly inside it, starting at a
/// word, without the bordering words heard again.
fn splice_candidate(
    window: &[Token],
    gap_start_frame: i64,
    gap_end_frame: i64,
    lead: Token,
    tail: Token,
    splice_safe: &HashSet<usize>,
    vocabulary: &Vocabulary,
) -> Vec<Token> {
    let same_piece = |a: usize, b: usize| {
        a == b || vocabulary.piece(a).to_lowercase() == vocabulary.piece(b).to_lowercase()
    };
    let mut candidate: Vec<Token> = window
        .iter()
        .copied()
        .filter(|token| token.timestamp > gap_start_frame && token.timestamp < gap_end_frame - 1)
        .collect();

    let leading_unsafe = candidate
        .iter()
        .take_while(|token| !splice_safe.contains(&token.id))
        .count();
    candidate.drain(..leading_unsafe);
    while let Some(first) = candidate.first()
        && same_piece(first.id, lead.id)
        && (first.timestamp - lead.timestamp).abs() <= EDGE_TOLERANCE_FRAMES
    {
        candidate.remove(0);
    }
    while let Some(last) = candidate.last()
        && same_piece(last.id, tail.id)
        && (tail.timestamp - last.timestamp).abs() <= EDGE_TOLERANCE_FRAMES
    {
        candidate.pop();
    }
    // Dropping a re-heard word can expose its continuation pieces or punctuation.
    while let Some(first) = candidate.first()
        && (!splice_safe.contains(&first.id)
            || text::is_punctuation_or_symbol_piece(vocabulary.piece(first.id)))
    {
        candidate.remove(0);
    }
    candidate
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use proptest::prelude::*;

    use super::*;

    fn token(id: usize, timestamp: i64, window_start: i64) -> Token {
        Token {
            id,
            timestamp,
            confidence: 0.9,
            duration: 1,
            window_start,
        }
    }

    fn ids(tokens: &[Token]) -> Vec<usize> {
        tokens.iter().map(|token| token.id).collect()
    }

    fn subword_vocabulary() -> Vocabulary {
        Vocabulary::from_entries(&[
            (1, "▁ha"),
            (2, "ve"),
            (3, "▁Ha"),
            (4, "▁ei"),
            (5, "ther"),
            (6, "▁Ei"),
            (7, "▁a"),
            (9, "."),
            (10, "▁Das"),
            (11, "▁Essen"),
            (12, "▁essen"),
            (13, "▁wir"),
        ])
    }

    /// A word of `pieces` from the window starting at `window_start`, one frame per piece.
    fn word(pieces: &[usize], start: i64, window_start: i64) -> Vec<Token> {
        pieces
            .iter()
            .zip(start..)
            .map(|(&id, timestamp)| token(id, timestamp, window_start))
            .collect()
    }

    fn collapse(tokens: Vec<Token>) -> Vec<usize> {
        ids(&collapse_seam_word_duplicates(
            tokens,
            &subword_vocabulary(),
        ))
    }

    proptest! {
        #[test]
        fn collapsing_seam_duplicates_only_drops_tokens(
            words in prop::collection::vec(
                (prop::sample::select(&[1usize, 2, 3, 4, 5, 6, 7, 9][..]), 0i64..3, 0i64..2),
                0..24,
            ),
        ) {
            let mut timestamp = 0;
            let tokens: Vec<Token> = words
                .into_iter()
                .map(|(id, gap, window)| {
                    timestamp += gap;
                    token(id, timestamp, window)
                })
                .collect();
            let collapsed = collapse_seam_word_duplicates(tokens.clone(), &subword_vocabulary());
            let mut remaining = tokens.iter();
            for token in &collapsed {
                prop_assert!(
                    remaining.any(|input| input.id == token.id && input.timestamp == token.timestamp),
                    "collapsing reordered or invented {:?}", token
                );
            }
        }
    }

    #[test]
    fn capitalised_seam_duplicate_is_dropped() {
        assert_eq!(
            collapse(
                [
                    word(&[1, 2], 100, 0),
                    word(&[3, 2], 102, 90),
                    word(&[7], 104, 90)
                ]
                .concat()
            ),
            [1, 2, 7]
        );
        assert_eq!(
            collapse(
                [
                    word(&[6, 5], 100, 0),
                    word(&[4, 5], 102, 90),
                    word(&[7], 104, 90)
                ]
                .concat()
            ),
            [4, 5, 7]
        );
    }

    #[test]
    fn case_only_repeats_inside_one_window_are_kept() {
        let sentence = [
            word(&[10], 100, 0),
            word(&[11], 102, 0),
            word(&[12], 104, 0),
            word(&[13], 106, 0),
        ]
        .concat();
        assert_eq!(collapse(sentence), [10, 11, 12, 13]);
        assert_eq!(
            collapse([word(&[1, 2], 100, 0), word(&[3, 2], 102, 0)].concat()),
            [1, 2, 3, 2]
        );
    }

    #[test]
    fn genuine_repeats_distant_repeats_and_sentence_starts_are_kept() {
        assert_eq!(
            collapse([word(&[1, 2], 100, 0), word(&[1, 2], 102, 90)].concat()),
            [1, 2, 1, 2]
        );
        assert_eq!(
            collapse([word(&[1, 2], 100, 0), word(&[3, 2], 200, 90)].concat()),
            [1, 2, 3, 2]
        );
        assert_eq!(
            collapse(
                [
                    word(&[1, 2, 9], 100, 0),
                    word(&[3, 2], 103, 90),
                    word(&[7], 105, 90)
                ]
                .concat()
            ),
            [1, 2, 9, 3, 2, 7]
        );
    }

    fn splice_vocabulary() -> Vocabulary {
        Vocabulary::from_entries(&[
            (1, "▁for"),
            (2, "▁For"),
            (3, "▁else"),
            (4, "."),
            (5, "▁and"),
            (6, "ing"),
            (7, "▁science"),
            (8, "▁I"),
            (9, "▁So"),
            (10, "▁examples"),
        ])
    }

    /// `splice_candidate` for the gap between frames 100 and 200.
    fn splice(
        window: &[(usize, i64)],
        lead: (usize, i64),
        tail: (usize, i64),
    ) -> Vec<(usize, i64)> {
        let vocabulary = splice_vocabulary();
        let window: Vec<Token> = window.iter().map(|&(id, at)| token(id, at, 0)).collect();
        splice_candidate(
            &window,
            100,
            200,
            token(lead.0, lead.1, 0),
            token(tail.0, tail.1, 0),
            &vocabulary.splice_safe_ids(),
            &vocabulary,
        )
        .iter()
        .map(|token| (token.id, token.timestamp))
        .collect()
    }

    #[test]
    fn only_tokens_strictly_inside_the_gap_are_spliced() {
        let window = [(5, 100), (5, 101), (5, 150), (5, 198), (5, 199), (5, 200)];
        assert_eq!(
            splice(&window, (10, 95), (10, 205)),
            [(5, 101), (5, 150), (5, 198)]
        );
        assert!(splice(&[(5, 80), (7, 90), (5, 210)], (10, 95), (10, 205)).is_empty());
    }

    #[test]
    fn a_splice_starts_at_a_word() {
        assert_eq!(
            splice(&[(6, 110), (5, 120), (7, 130)], (10, 95), (10, 205)),
            [(5, 120), (7, 130)]
        );
    }

    #[test]
    fn bordering_words_heard_again_are_not_spliced() {
        assert_eq!(
            splice(&[(2, 104), (10, 120)], (1, 100), (10, 205)),
            [(10, 120)]
        );
        assert_eq!(
            splice(&[(10, 120), (9, 196)], (10, 95), (9, 201)),
            [(10, 120)]
        );
        assert!(splice(&[(1, 104), (4, 106)], (1, 100), (10, 205)).is_empty());
        assert_eq!(
            splice(&[(8, 108), (10, 130)], (8, 100), (10, 205)),
            [(8, 108), (10, 130)],
            "a repeat beyond the tolerance is a real one"
        );
    }

    #[test]
    fn the_word_bordering_a_gap_is_found_past_punctuation() {
        let vocabulary = splice_vocabulary();
        let stream = [token(5, 90, 0), token(3, 98, 0), token(4, 99, 0)];
        assert_eq!(
            word_neighbor(&stream, 2, Direction::Back, &vocabulary).id,
            3
        );
        assert_eq!(
            word_neighbor(&stream, 2, Direction::Forward, &vocabulary).id,
            4
        );
    }

    /// Records which windows it decodes and hears nothing.
    #[derive(Default)]
    struct Deaf {
        calls: Cell<usize>,
    }

    impl WindowDecoder for Deaf {
        fn decode_window(&self, _: WindowRequest) -> Result<Vec<Token>> {
            self.calls.set(self.calls.get() + 1);
            Ok(Vec::new())
        }
    }

    /// Number of windows decoded to repair `tokens` of 30 s of speech-like audio.
    fn repair(tokens: Vec<Token>, seams: &[Range<i64>]) -> usize {
        let samples = vec![0.1; 30 * 16_000];
        let vocabulary = Vocabulary::from_entries(&[(1, "▁a"), (2, "▁b")]);
        let decoder = Deaf::default();
        let repaired = repair_seam_gaps(
            &samples,
            tokens.clone(),
            &decoder,
            &vocabulary,
            &vocabulary.splice_safe_ids(),
            seams,
        )
        .unwrap();
        assert_eq!(ids(&repaired), ids(&tokens));
        decoder.calls.get()
    }

    #[test]
    fn only_gaps_reaching_into_a_seam_are_decoded_again() {
        let tokens = vec![token(1, 10, 0), token(2, 60, 0)];
        for (seam, decodes) in [
            (100..120, 0),
            (0..11, 0),
            (60..80, 0),
            (55..80, 2),
            (0..12, 2),
        ] {
            assert_eq!(
                repair(tokens.clone(), std::slice::from_ref(&seam)),
                decodes,
                "{seam:?}"
            );
        }
    }

    #[test]
    fn repairs_stop_at_two_decodes_per_seam() {
        let gaps: Vec<Token> = (0..6)
            .map(|index| token(1 + index % 2, 10 + 40 * signed(index), 0))
            .collect();
        assert_eq!(repair(gaps.clone(), std::slice::from_ref(&(0..300))), 2);
        assert_eq!(repair(gaps, &[0..100, 100..300]), 4);
    }

    #[test]
    fn duplicates_split_by_a_punctuation_splice_are_at_a_seam() {
        let vocabulary = Vocabulary::from_entries(&[(1, "▁have"), (2, ","), (3, "▁Have")]);
        let tokens = vec![token(1, 100, 0), token(2, 101, 90), token(3, 102, 90)];
        let collapsed = collapse_seam_word_duplicates(tokens, &vocabulary);
        assert_eq!(ids(&collapsed), [1, 2]);
    }
}
