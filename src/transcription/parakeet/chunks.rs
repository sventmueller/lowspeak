//! Long-form transcription: splits audio over 15 s into overlapping windows, decodes each and
//! merges their tokens, after the default path of FluidAudio's `ChunkProcessor`.

use std::ops::Range;

use anyhow::Result;

use super::energy;
use super::layout::{self, Layout, Windowing};
use super::merge::{enforce_monotonic_timestamps, merge_chunks};
use super::seams::{collapse_seam_word_duplicates, repair_seam_gaps};
use super::text::Vocabulary;
use super::{SAMPLES_PER_FRAME, Token};
use crate::util::numeric::signed;

/// Decodes one window of audio, as `AsrManager.executeMLInferenceWithTimings` does.
pub trait WindowDecoder {
    fn decode_window(&self, request: WindowRequest) -> Result<Vec<Token>>;
}

pub struct WindowRequest<'a> {
    pub samples: &'a [f32],
    pub context_samples: usize,
    pub chunk_start: usize,
    pub is_last_chunk: bool,
    pub emit_tokens_after_frame: Option<i64>,
    pub initial_time_index_override: Option<usize>,
}

pub fn transcribe(
    samples: &[f32],
    decoder: &dyn WindowDecoder,
    vocabulary: &Vocabulary,
    windowing: Windowing,
) -> Result<Vec<Token>> {
    let total = samples.len();
    let layout = Layout::new(windowing);
    let end_aligned = windowing == Windowing::SilenceAligned;
    let starts = match windowing {
        Windowing::MelContext => layout::regular_starts(total, layout.stride_samples),
        Windowing::SilenceAligned => {
            layout::silence_aligned_starts(samples, layout.chunk_samples, layout.stride_samples)
        }
    };
    let speech_end = if end_aligned {
        energy::speech_end(samples)
    } else {
        total
    };

    let mut outputs: Vec<Vec<Token>> = Vec::new();
    let mut spans: Vec<Range<i64>> = Vec::new();
    let mut chunk_start = starts[0];
    while chunk_start < total {
        let warmup = if end_aligned {
            layout::last_chunk_warmup(total, chunk_start, layout.chunk_samples, speech_end)
        } else {
            0
        };
        let visible = SAMPLES_PER_FRAME.max(layout.chunk_samples - warmup);
        let candidate_end = chunk_start + visible;
        let is_last = candidate_end >= total;
        let chunk_end = if is_last { total } else { candidate_end };
        let audio_end = if is_last && end_aligned {
            chunk_end.min(speech_end)
        } else {
            chunk_end
        };
        if audio_end <= chunk_start {
            break;
        }

        let context_samples = if warmup == 0 && !outputs.is_empty() {
            layout.mel_context_samples
        } else {
            0
        };
        let context_start = chunk_start - warmup.max(context_samples);
        let emit_after = (warmup > 0).then_some(signed(chunk_start / SAMPLES_PER_FRAME));
        outputs.push(decoder.decode_window(WindowRequest {
            samples: &samples[context_start..audio_end],
            context_samples,
            chunk_start: if warmup > 0 {
                context_start
            } else {
                chunk_start
            },
            is_last_chunk: is_last,
            emit_tokens_after_frame: emit_after,
            initial_time_index_override: emit_after.map(|_| 0),
        })?);
        spans.push(
            signed(chunk_start / SAMPLES_PER_FRAME)..signed(audio_end.div_ceil(SAMPLES_PER_FRAME)),
        );

        if is_last {
            break;
        }
        chunk_start = starts
            .get(outputs.len())
            .copied()
            .unwrap_or(chunk_start + layout.stride_samples);
    }

    let seams: Vec<Range<i64>> = spans
        .windows(2)
        .map(|pair| pair[1].start..pair[0].end)
        .collect();
    let mut outputs = outputs.into_iter();
    let Some(first) = outputs.next() else {
        return Ok(Vec::new());
    };
    if seams.is_empty() {
        return Ok(enforce_monotonic_timestamps(first));
    }
    let splice_safe = vocabulary.splice_safe_ids();
    let case_variants = vocabulary.case_variant_ids();
    let merged = outputs.fold(first, |merged, chunk| {
        merge_chunks(&merged, &chunk, &splice_safe, &case_variants)
    });
    let merged = enforce_monotonic_timestamps(merged);
    let merged = collapse_seam_word_duplicates(merged, vocabulary);
    if merged.len() < 2 {
        return Ok(merged);
    }
    repair_seam_gaps(samples, merged, decoder, vocabulary, &splice_safe, &seams)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::audio::SAMPLE_RATE;
    use crate::util::numeric::unsigned;

    const FRAME: usize = SAMPLES_PER_FRAME;
    const WORD_SPACING: i64 = 5;

    /// A speaker who says a word every 5 frames and a decoder that hears them, except where it is
    /// deaf: in `deaf_to_all` always, in `deaf_after_late_start` when its window started more than
    /// 8 frames before, as a decoder that blanks out at a window's end.
    struct Speaker {
        frames: i64,
        deaf_to_all: Range<i64>,
        deaf_after_late_start: Range<i64>,
        windows: RefCell<Vec<(usize, usize, usize)>>,
    }

    impl Speaker {
        fn new(seconds: usize) -> Self {
            Self {
                frames: signed(seconds * usize::try_from(SAMPLE_RATE).unwrap() / FRAME),
                deaf_to_all: 0..0,
                deaf_after_late_start: 0..0,
                windows: RefCell::new(Vec::new()),
            }
        }

        fn samples(&self) -> Vec<f32> {
            vec![0.1; unsigned(self.frames) * FRAME]
        }

        fn word(frame: i64) -> usize {
            100 + unsigned(frame / WORD_SPACING % 40)
        }

        fn script(&self) -> Vec<usize> {
            (0..self.frames)
                .step_by(unsigned(WORD_SPACING))
                .map(Self::word)
                .collect()
        }

        fn vocabulary() -> Vocabulary {
            let pieces: Vec<(usize, String)> =
                (100..140).map(|id| (id, format!("▁w{id}"))).collect();
            let entries: Vec<(usize, &str)> = pieces
                .iter()
                .map(|(id, piece)| (*id, piece.as_str()))
                .collect();
            Vocabulary::from_entries(&entries)
        }

        fn transcribe(&self, windowing: Windowing) -> Vec<usize> {
            let tokens = transcribe(&self.samples(), self, &Self::vocabulary(), windowing).unwrap();
            tokens.iter().map(|token| token.id).collect()
        }
    }

    impl WindowDecoder for Speaker {
        fn decode_window(&self, request: WindowRequest) -> Result<Vec<Token>> {
            self.windows.borrow_mut().push((
                request.chunk_start,
                request.samples.len(),
                request.context_samples,
            ));
            let first = signed(request.chunk_start / FRAME);
            let end = first + signed((request.samples.len() - request.context_samples) / FRAME);
            let heard = |frame: &i64| {
                !self.deaf_to_all.contains(frame)
                    && !(self.deaf_after_late_start.contains(frame)
                        && first < self.deaf_after_late_start.start - 8)
                    && request
                        .emit_tokens_after_frame
                        .is_none_or(|after| *frame >= after)
            };
            Ok((first..end)
                .filter(|frame| frame % WORD_SPACING == 0)
                .filter(heard)
                .map(|frame| Token {
                    id: Self::word(frame),
                    timestamp: frame,
                    confidence: 0.9,
                    duration: 1,
                    window_start: first,
                })
                .collect())
        }
    }

    #[test]
    fn overlapping_windows_merge_into_the_whole_recording() {
        for windowing in [Windowing::SilenceAligned, Windowing::MelContext] {
            let speaker = Speaker::new(40);
            assert_eq!(
                speaker.transcribe(windowing),
                speaker.script(),
                "{windowing:?}"
            );
            assert_eq!(speaker.windows.borrow().len(), 3, "{windowing:?}");
        }
    }

    #[test]
    fn v2_windows_after_the_first_carry_80_ms_of_context() {
        let speaker = Speaker::new(40);
        speaker.transcribe(Windowing::MelContext);
        let layout = Layout::new(Windowing::MelContext);
        for (index, &(start, length, context)) in speaker.windows.borrow().iter().enumerate() {
            assert_eq!(start, index * layout.stride_samples);
            assert_eq!(context, if index == 0 { 0 } else { FRAME });
            assert!(length <= layout.chunk_samples + context);
        }
    }

    #[test]
    fn the_short_last_v3_window_is_filled_with_earlier_audio() {
        let speaker = Speaker::new(40);
        speaker.transcribe(Windowing::SilenceAligned);
        let layout = Layout::new(Windowing::SilenceAligned);
        let windows = speaker.windows.borrow();
        let &(start, length, context) = windows.last().unwrap();
        assert_eq!(context, 0);
        assert_eq!(start + length, speaker.samples().len());
        assert!(length + FRAME > layout.chunk_samples && length <= layout.chunk_samples);
    }

    #[test]
    fn speech_a_seam_dropped_is_decoded_again() {
        let mut speaker = Speaker::new(30);
        speaker.deaf_after_late_start = 175..195;
        assert_eq!(
            speaker.transcribe(Windowing::SilenceAligned),
            speaker.script()
        );
        assert_eq!(speaker.windows.borrow().len(), 3 + 1);
    }

    #[test]
    fn a_pause_inside_a_window_is_not_decoded_again() {
        let mut speaker = Speaker::new(30);
        speaker.deaf_to_all = 50..75;
        let mut expected = speaker.script();
        expected.drain(10..15);
        assert_eq!(speaker.transcribe(Windowing::SilenceAligned), expected);
        assert_eq!(speaker.windows.borrow().len(), 3);
    }

    #[test]
    fn audio_that_fits_one_window_needs_no_merge() {
        let speaker = Speaker::new(10);
        assert_eq!(
            speaker.transcribe(Windowing::SilenceAligned),
            speaker.script()
        );
        assert_eq!(speaker.windows.borrow().len(), 1);
    }
}
