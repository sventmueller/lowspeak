//! Merging the tokens of two overlapping windows into one stream.

use std::collections::{HashMap, HashSet};

use super::layout::OVERLAP_SECONDS;
use super::{SECONDS_PER_FRAME, Token};

#[derive(Clone, Copy)]
struct Indexed {
    index: usize,
    token: Token,
    start: f64,
}

fn start_time(token: &Token) -> f64 {
    token.timestamp as f64 * SECONDS_PER_FRAME
}

fn token_ids_match(left: usize, right: usize, case_variants: &HashMap<usize, usize>) -> bool {
    left == right
        || matches!(
            (case_variants.get(&left), case_variants.get(&right)),
            (Some(lhs), Some(rhs)) if lhs == rhs
        )
}

/// Pairs `(left, right)` of the longest run of consecutive matches.
fn find_contiguous_matches<T>(
    left: &[T],
    right: &[T],
    matcher: impl Fn(&T, &T) -> bool,
) -> Vec<(usize, usize)> {
    let mut best: Vec<(usize, usize)> = Vec::new();
    for i in 0..left.len() {
        for j in 0..right.len() {
            if !matcher(&left[i], &right[j]) {
                continue;
            }
            let mut current = Vec::new();
            let (mut k, mut l) = (i, j);
            while k < left.len() && l < right.len() && matcher(&left[k], &right[l]) {
                current.push((k, l));
                k += 1;
                l += 1;
            }
            if current.len() > best.len() {
                best = current;
            }
        }
    }
    best
}

fn find_longest_common_subsequence<T>(
    left: &[T],
    right: &[T],
    matcher: impl Fn(&T, &T) -> bool,
) -> Vec<(usize, usize)> {
    let (n, m) = (left.len(), right.len());
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if matcher(&left[i - 1], &right[j - 1]) {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    let mut matches = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        if matcher(&left[i - 1], &right[j - 1]) {
            matches.push((i - 1, j - 1));
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    matches.reverse();
    matches
}

/// Merges the tokens of a window into those of the windows before it. `splice_safe` holds the ids
/// that may start a splice (word-initial or punctuation pieces), `case_variants` maps ids of
/// case-only twins to a shared id.
pub fn merge_chunks(
    left: &[Token],
    right: &[Token],
    splice_safe: &HashSet<usize>,
    case_variants: &HashMap<usize, usize>,
) -> Vec<Token> {
    let (Some(left_last), Some(right_first)) = (left.last(), right.first()) else {
        return if left.is_empty() {
            right.to_vec()
        } else {
            left.to_vec()
        };
    };
    let left_end_time = start_time(left_last) + SECONDS_PER_FRAME;
    let right_start_time = start_time(right_first);
    if left_end_time <= right_start_time {
        return [left, right].concat();
    }

    let indexed = |tokens: &[Token]| -> Vec<Indexed> {
        tokens
            .iter()
            .enumerate()
            .map(|(index, &token)| Indexed {
                index,
                token,
                start: start_time(&token),
            })
            .collect()
    };
    let overlap_left: Vec<Indexed> = indexed(left)
        .into_iter()
        .filter(|item| item.start + SECONDS_PER_FRAME > right_start_time - OVERLAP_SECONDS)
        .collect();
    let overlap_right: Vec<Indexed> = indexed(right)
        .into_iter()
        .filter(|item| item.start < left_end_time + OVERLAP_SECONDS)
        .collect();
    if overlap_left.len() < 2 || overlap_right.len() < 2 {
        return merge_by_midpoint(left, right, left_end_time, right_start_time, splice_safe);
    }

    let minimum_pairs = (overlap_left.len() / 2).max(1);
    let half_overlap = OVERLAP_SECONDS / 2.0;
    let matcher = |l: &Indexed, r: &Indexed| {
        token_ids_match(l.token.id, r.token.id, case_variants)
            && (l.start - r.start).abs() < half_overlap
    };
    let contiguous = find_contiguous_matches(&overlap_left, &overlap_right, matcher);
    let pairs = if contiguous.len() >= minimum_pairs {
        contiguous
    } else {
        let lcs = find_longest_common_subsequence(&overlap_left, &overlap_right, matcher);
        if lcs.is_empty() {
            return merge_by_midpoint(left, right, left_end_time, right_start_time, splice_safe);
        }
        lcs
    };
    let left_indices: Vec<usize> = pairs.iter().map(|&(l, _)| overlap_left[l].index).collect();
    let right_indices: Vec<usize> = pairs.iter().map(|&(_, r)| overlap_right[r].index).collect();
    merge_using_matches(left, right, &left_indices, &right_indices, splice_safe)
}

fn merge_using_matches(
    left: &[Token],
    right: &[Token],
    left_indices: &[usize],
    right_indices: &[usize],
    splice_safe: &HashSet<usize>,
) -> Vec<Token> {
    let mut result = Vec::new();
    if let Some(&first) = left_indices.first() {
        result.extend_from_slice(&left[..first]);
    }
    for idx in 0..left_indices.len() {
        let (left_index, right_index) = (left_indices[idx], right_indices[idx]);
        result.push(left[left_index]);
        if idx + 1 == left_indices.len() {
            continue;
        }
        let (next_left, next_right) = (left_indices[idx + 1], right_indices[idx + 1]);
        let gap_left = &left[(left_index + 1).min(next_left)..next_left];
        let gap_right = &right[(right_index + 1).min(next_right)..next_right];
        result.extend_from_slice(if gap_right.len() > gap_left.len() {
            gap_right
        } else {
            gap_left
        });
    }

    let Some(&last_right) = right_indices.last() else {
        return result;
    };
    let tail = &right[last_right + 1..];
    let Some(tail_first) = tail.first() else {
        return result;
    };
    if splice_safe.contains(&tail_first.id) {
        result.extend_from_slice(tail);
        return result;
    }
    // The splice lands mid-word: re-splice at a word boundary so one window owns the seam word.
    let right_word_start = (0..=last_right)
        .rev()
        .find(|&index| splice_safe.contains(&right[index].id));
    let left_word_start = result
        .iter()
        .rposition(|token| splice_safe.contains(&token.id));
    if let (Some(word_start), Some(cut)) = (right_word_start, left_word_start) {
        result.truncate(cut);
        result.extend_from_slice(&right[word_start..]);
        return result;
    }
    if let Some(&last_left) = left_indices.last() {
        result.extend(
            left[last_left + 1..]
                .iter()
                .take_while(|token| !splice_safe.contains(&token.id)),
        );
    }
    let resume = tail
        .iter()
        .position(|token| splice_safe.contains(&token.id))
        .unwrap_or(0);
    result.extend_from_slice(&tail[resume..]);
    result
}

fn merge_by_midpoint(
    left: &[Token],
    right: &[Token],
    left_end_time: f64,
    right_start_time: f64,
    splice_safe: &HashSet<usize>,
) -> Vec<Token> {
    let cutoff = (left_end_time + right_start_time) / 2.0;
    let after_cutoff = |token: &Token| start_time(token) >= cutoff;
    let mut left_end = left.iter().position(after_cutoff).unwrap_or(left.len());
    let mut right_start = right.iter().position(after_cutoff).unwrap_or(right.len());
    if left_end > 0 {
        while left_end < left.len() && !splice_safe.contains(&left[left_end].id) {
            left_end += 1;
        }
    }
    if let Some(offset) = right[right_start..]
        .iter()
        .position(|token| splice_safe.contains(&token.id))
    {
        right_start += offset;
    }
    [&left[..left_end], &right[right_start..]].concat()
}

/// Clamps timestamps to be non-decreasing without reordering: the merge order is the text order.
pub fn enforce_monotonic_timestamps(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut last = match tokens.first() {
        Some(token) => token.timestamp,
        None => return tokens,
    };
    for token in tokens.iter_mut().skip(1) {
        if token.timestamp < last {
            token.timestamp = last;
        } else {
            last = token.timestamp;
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::super::text::{self, Vocabulary};
    use super::*;

    const SPLICE_IDS: [usize; 13] = [10, 20, 21, 22, 24, 25, 26, 27, 28, 30, 40, 50, 60];

    fn token(id: usize, timestamp: i64) -> Token {
        Token {
            id,
            timestamp,
            confidence: 0.95,
            duration: 1,
            window_start: 0,
        }
    }

    fn ids(tokens: &[Token]) -> Vec<usize> {
        tokens.iter().map(|token| token.id).collect()
    }

    fn tokens(entries: &[(usize, i64)]) -> Vec<Token> {
        entries
            .iter()
            .map(|&(id, timestamp)| token(id, timestamp))
            .collect()
    }

    /// Tokens with ascending timestamps from `start`, each `gap` frames after the previous one.
    fn stream(start: i64) -> impl Strategy<Value = Vec<Token>> {
        prop::collection::vec((prop::sample::select(&SPLICE_IDS[..]), 0i64..4), 0..24).prop_map(
            move |entries| {
                let mut timestamp = start;
                entries
                    .into_iter()
                    .map(|(id, gap)| {
                        timestamp += gap;
                        token(id, timestamp)
                    })
                    .collect()
            },
        )
    }

    fn windows() -> impl Strategy<Value = (Vec<Token>, Vec<Token>)> {
        (100i64..140).prop_flat_map(|right_start| (stream(100), stream(right_start)))
    }

    fn splice_vocabulary() -> Vocabulary {
        Vocabulary::from_entries(&[
            (10, "▁hello"),
            (20, "▁wor"),
            (21, "ld"),
            (22, "ldo"),
            (24, "▁Gre"),
            (25, "nl"),
            (26, "and"),
            (27, "▁Green"),
            (28, "andia"),
            (30, "▁there"),
            (40, "▁friend"),
            (50, "ne"),
            (60, "▁o"),
        ])
    }

    /// What the merge knows about ids without a vocabulary: every id starts a word, none has a
    /// case twin.
    fn without_vocabulary() -> HashSet<usize> {
        (0..1_000).collect()
    }

    fn merge(left: &[(usize, i64)], right: &[(usize, i64)], vocabulary: bool) -> Vec<usize> {
        let safe = if vocabulary {
            splice_vocabulary().splice_safe_ids()
        } else {
            without_vocabulary()
        };
        ids(&merge_chunks(
            &tokens(left),
            &tokens(right),
            &safe,
            &HashMap::new(),
        ))
    }

    proptest! {
        #[test]
        fn merging_only_keeps_tokens_from_the_two_windows(
            (left, right) in windows(),
            with_vocabulary in any::<bool>(),
        ) {
            let vocabulary = splice_vocabulary();
            let (safe, variants) = if with_vocabulary {
                (vocabulary.splice_safe_ids(), vocabulary.case_variant_ids())
            } else {
                (without_vocabulary(), HashMap::new())
            };
            let merged = merge_chunks(&left, &right, &safe, &variants);
            prop_assert!(merged.len() <= left.len() + right.len());
            for token in &merged {
                prop_assert!(
                    left.iter().chain(&right).any(|input| input.id == token.id
                        && input.timestamp == token.timestamp),
                    "token {:?} came from neither window", token
                );
            }
            let disjoint = match (left.last(), right.first()) {
                (Some(last), Some(first)) => last.timestamp + 1 < first.timestamp,
                _ => true,
            };
            if disjoint {
                prop_assert_eq!(ids(&merged), ids(&[left.clone(), right.clone()].concat()));
            }
        }

        #[test]
        fn timestamps_end_up_in_order_with_the_same_tokens(tokens in stream(0)) {
            let mut shuffled = tokens.clone();
            shuffled.reverse();
            let ordered = enforce_monotonic_timestamps(shuffled.clone());
            prop_assert_eq!(ids(&ordered), ids(&shuffled));
            prop_assert!(ordered.windows(2).all(|pair| pair[0].timestamp <= pair[1].timestamp));
        }
    }

    #[test]
    fn gap_of_equal_length_keeps_the_older_window() {
        let left = [(100, 120), (200, 130), (901, 131), (300, 132)];
        let right = [(200, 130), (902, 131), (300, 132), (400, 133)];
        assert_eq!(merge(&left, &right, false), [100, 200, 901, 300, 400]);
    }

    #[test]
    fn seam_word_takes_the_right_windows_segmentation_when_it_heard_the_whole_word() {
        let left = [(10, 120), (24, 130), (25, 131), (26, 132)];
        let right = [(27, 130), (25, 131), (28, 132), (30, 134)];
        assert_eq!(merge(&left, &right, true), [10, 27, 25, 28, 30]);
    }

    #[test]
    fn seam_word_stays_with_the_left_window_when_the_right_starts_mid_word() {
        let left = [(10, 120), (24, 130), (25, 131), (26, 132)];
        let right = [(25, 131), (28, 132), (30, 134)];
        assert_eq!(merge(&left, &right, true), [10, 24, 25, 26, 30]);
    }

    #[test]
    fn word_initial_tail_is_appended_verbatim() {
        let left = [(10, 120), (20, 130), (21, 131)];
        let right = [(21, 131), (30, 133), (40, 134)];
        assert_eq!(merge(&left, &right, true), [10, 20, 21, 30, 40]);
    }

    #[test]
    fn midpoint_merge_does_not_cut_words() {
        let left = [(10, 120), (20, 133), (21, 135)];
        let right = [(60, 134), (50, 136), (30, 138)];
        assert_eq!(merge(&left, &right, true), [10, 20, 21, 30]);
        assert_eq!(merge(&left, &right, false), [10, 20, 50, 30]);
    }

    #[test]
    fn windows_without_overlap_are_concatenated() {
        assert_eq!(
            merge(&[(10, 100), (20, 101)], &[(30, 200)], false),
            [10, 20, 30]
        );
        assert_eq!(merge(&[], &[(30, 200)], false), [30]);
    }

    #[test]
    fn case_twins_match_across_the_seam() {
        let vocabulary = Vocabulary::from_entries(&[
            (1, "▁we"),
            (2, "▁meet"),
            (3, "▁Meet"),
            (4, "▁at"),
            (5, "▁noon"),
        ]);
        let left = tokens(&[(1, 120), (2, 125), (4, 130)]);
        let right = tokens(&[(3, 125), (4, 130), (5, 135)]);
        let merged = merge_chunks(
            &left,
            &right,
            &vocabulary.splice_safe_ids(),
            &vocabulary.case_variant_ids(),
        );
        assert_eq!(ids(&merged), [1, 2, 4, 5]);
    }

    #[test]
    fn splice_safe_pieces() {
        for piece in ["▁word", "▁", " word", ",", "...", "?"] {
            assert!(text::is_splice_safe_piece(piece), "{piece}");
        }
        for piece in ["ld", "ня", "", "<unk>"] {
            assert!(!text::is_splice_safe_piece(piece), "{piece}");
        }
    }
}
