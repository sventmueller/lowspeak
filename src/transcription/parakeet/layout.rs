//! Where the windows of a long recording start and how much audio each takes.

use super::energy::boundary_energy;
use super::{MAX_MODEL_SAMPLES, MEL_HOP, SAMPLES_PER_FRAME};
use crate::audio::SAMPLE_RATE;
use crate::util::numeric::{floor_usize, signed, unsigned};

pub const OVERLAP_SECONDS: f64 = 2.0;
/// A window start is looked for in silence up to 4 s from its regular place, else in an energy
/// valley up to 0.5 s from it.
const SILENCE_SEARCH_SECONDS: f64 = 4.0;
const VALLEY_SEARCH_SECONDS: f64 = 0.5;
/// Silence is a boundary below 5 % of the median energy around it, a valley one below 35 %.
const SILENCE_RATIO: f32 = 0.05;
const VALLEY_RATIO: f32 = 0.35;
/// Boundary energy is measured over one frame on either side.
const BOUNDARY_HALF_WINDOW: usize = SAMPLES_PER_FRAME;
/// Windows keep 480 ms of overlap however far a start moves, so the merge has context.
const MIN_OVERLAP_SAMPLES: usize = 6 * SAMPLES_PER_FRAME;

/// How a model's windows are laid out; FluidAudio decides by `melChunkContext` and `isV3Family`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Windowing {
    /// Parakeet v2: regular starts, and every window after the first begins with the previous 80 ms
    /// as context for the encoder's convolutions.
    MelContext,
    /// The v3 family: starts moved onto nearby silence, and a short last window filled backwards
    /// with real audio instead of padding.
    SilenceAligned,
}

pub struct Layout {
    pub chunk_samples: usize,
    pub stride_samples: usize,
    pub mel_context_samples: usize,
}

impl Layout {
    pub fn new(windowing: Windowing) -> Self {
        let reserved = match windowing {
            Windowing::MelContext => SAMPLES_PER_FRAME,
            Windowing::SilenceAligned => 0,
        };
        let raw = (MAX_MODEL_SAMPLES - reserved - MEL_HOP).max(SAMPLES_PER_FRAME);
        let chunk_samples = raw / SAMPLES_PER_FRAME * SAMPLES_PER_FRAME;
        let requested = floor_usize(OVERLAP_SECONDS * f64::from(SAMPLE_RATE));
        let overlap = requested.min(chunk_samples / 2) / SAMPLES_PER_FRAME * SAMPLES_PER_FRAME;
        let stride = (chunk_samples - overlap).max(SAMPLES_PER_FRAME) / SAMPLES_PER_FRAME
            * SAMPLES_PER_FRAME;
        Self {
            chunk_samples,
            stride_samples: stride,
            mel_context_samples: reserved,
        }
    }
}

pub fn regular_starts(total: usize, stride: usize) -> Vec<usize> {
    let mut starts = vec![0];
    let mut start = stride;
    while start < total {
        starts.push(start);
        start += stride;
    }
    starts
}

struct Candidate {
    start: usize,
    score: f32,
    median: f32,
}

fn below(candidate: &Candidate, ratio: f32) -> bool {
    let threshold = if candidate.median > 0.0 {
        candidate.median * ratio
    } else {
        0.0
    };
    candidate.score <= threshold
}

/// Window starts moved onto nearby silence (or at least an energy valley) where possible.
pub fn silence_aligned_starts(samples: &[f32], chunk_samples: usize, stride: usize) -> Vec<usize> {
    let frame = SAMPLES_PER_FRAME;
    let radius = |seconds: f64| floor_usize((seconds * f64::from(SAMPLE_RATE)) / frame as f64);
    let silence_radius = radius(SILENCE_SEARCH_SECONDS).max(1);
    let valley_radius = radius(VALLEY_SEARCH_SECONDS).max(1);
    let total = samples.len();

    let mut starts = vec![0];
    let mut previous = 0usize;
    let mut target = stride;
    while target < total {
        let target_frame = target / frame;
        let latest_covered = previous + chunk_samples - MIN_OVERLAP_SAMPLES;
        let target_start = (target_frame * frame)
            .max(previous + frame)
            .min(latest_covered);
        let search =
            |radius| best_boundary(samples, target_frame, radius, previous, latest_covered);

        let silence = search(silence_radius);
        let mut best = if below(&silence, SILENCE_RATIO) {
            silence.start
        } else {
            let valley = search(valley_radius);
            if below(&valley, VALLEY_RATIO) {
                valley.start
            } else {
                target_start
            }
        };
        if best <= previous {
            best = (previous + stride).min(total);
        }
        starts.push(best);
        previous = best;
        target += stride;
    }
    starts
}

fn best_boundary(
    samples: &[f32],
    target_frame: usize,
    radius: usize,
    previous: usize,
    latest_covered: usize,
) -> Candidate {
    let frame = SAMPLES_PER_FRAME;
    let lower = target_frame.saturating_sub(radius).max(1);
    let upper = ((samples.len() - 1) / frame).min(target_frame + radius);
    let target_start = (target_frame * frame)
        .max(previous + frame)
        .min(latest_covered);

    let mut best_start = target_start;
    let mut best_score = f32::MAX;
    let mut scores = Vec::new();
    if lower <= upper {
        for frame_index in lower..=upper {
            let candidate = frame_index * frame;
            if candidate <= previous || candidate > latest_covered {
                continue;
            }
            let score = boundary_energy(samples, candidate, BOUNDARY_HALF_WINDOW);
            scores.push(score);
            if score < best_score {
                best_score = score;
                best_start = candidate;
            }
        }
    }
    if scores.is_empty() {
        return Candidate {
            start: target_start,
            score: f32::MAX,
            median: 0.0,
        };
    }
    scores.sort_by(f32::total_cmp);
    Candidate {
        start: best_start,
        score: best_score,
        median: scores[scores.len() / 2],
    }
}

/// Real audio before a short last window that fills it instead of zero padding.
pub fn last_chunk_warmup(
    total: usize,
    chunk_start: usize,
    chunk_samples: usize,
    speech_end: usize,
) -> usize {
    let frame = SAMPLES_PER_FRAME;
    let visible = frame.max(chunk_samples);
    let is_last = chunk_start + visible >= total;
    let remaining = signed(speech_end.min(total)) - signed(chunk_start);
    if !is_last || remaining <= 0 || chunk_start == 0 {
        return 0;
    }
    let fill = (signed(chunk_samples) - remaining) / signed(frame) * signed(frame);
    if fill <= 0 {
        return 0;
    }
    let available = chunk_start / frame * frame;
    available.min(unsigned(fill))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRAME: usize = SAMPLES_PER_FRAME;

    fn seconds(value: f64) -> usize {
        floor_usize(value * f64::from(SAMPLE_RATE))
    }

    #[test]
    fn v3_layout_matches_fluidaudio() {
        let layout = Layout::new(Windowing::SilenceAligned);
        assert_eq!(
            (
                layout.chunk_samples,
                layout.stride_samples,
                layout.mel_context_samples
            ),
            (239_360, 207_360, 0)
        );
        let layout = Layout::new(Windowing::MelContext);
        assert_eq!(layout.mel_context_samples, 1_280);
        assert_eq!(layout.stride_samples, 206_080);
    }

    #[test]
    fn starts_prefer_nearby_silence() {
        let layout = Layout::new(Windowing::SilenceAligned);
        let mut audio = vec![0.02; seconds(45.0)];
        let silent = layout.stride_samples + 3 * FRAME;
        audio[silent - FRAME..silent + FRAME].fill(0.0);
        let starts = silence_aligned_starts(&audio, layout.chunk_samples, layout.stride_samples);
        assert_eq!(starts[1], silent);
    }

    #[test]
    fn flat_audio_keeps_the_regular_starts() {
        let layout = Layout::new(Windowing::SilenceAligned);
        let audio = vec![0.02; seconds(45.0)];
        let starts = silence_aligned_starts(&audio, layout.chunk_samples, layout.stride_samples);
        assert_eq!(
            starts,
            regular_starts(audio.len(), layout.stride_samples),
            "a flat stretch is no boundary"
        );
    }

    #[test]
    fn moved_starts_keep_enough_overlap_for_the_merge() {
        let layout = Layout::new(Windowing::SilenceAligned);
        let mut audio = vec![0.02; seconds(90.0)];
        let radius = floor_usize(SILENCE_SEARCH_SECONDS * f64::from(SAMPLE_RATE)) / FRAME;
        let first = layout.stride_samples - radius * FRAME;
        audio[first - FRAME..first + FRAME].fill(0.0);
        let second = first + layout.chunk_samples - FRAME;
        audio[second - FRAME..second + FRAME].fill(0.0);
        let starts = silence_aligned_starts(&audio, layout.chunk_samples, layout.stride_samples);
        assert_eq!(starts[1], first);
        assert!(starts[2] <= starts[1] + layout.chunk_samples - MIN_OVERLAP_SAMPLES);
        assert!(starts.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn short_final_window_is_filled_backwards_with_real_audio() {
        let chunk = Layout::new(Windowing::MelContext).chunk_samples;
        let total = seconds(17.28);
        let start = 206_080;
        let warmup = last_chunk_warmup(total, start, chunk, total);
        assert_eq!(warmup % FRAME, 0);
        assert!(warmup > 0 && warmup <= start);
        let filled = warmup + (total - start);
        assert!(filled + FRAME >= chunk && filled <= chunk + FRAME);

        assert_eq!(last_chunk_warmup(2 * chunk, chunk, chunk, 2 * chunk), 0);
        assert_eq!(last_chunk_warmup(chunk / 2, 0, chunk, chunk / 2), 0);
    }

    #[test]
    fn trailing_silence_grows_the_backfill() {
        let chunk = Layout::new(Windowing::SilenceAligned).chunk_samples;
        let total = seconds(20.0);
        let start = 207_360;
        let to_the_end = last_chunk_warmup(total, start, chunk, total);
        let to_the_speech = last_chunk_warmup(total, start, chunk, total - 40 * FRAME);
        assert_eq!(to_the_speech, to_the_end + 40 * FRAME);
    }
}
