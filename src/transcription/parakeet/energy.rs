//! Energy of the recording in encoder frames, for window starts and seam repair.
//!
//! Unlike `audio::vad`, which tracks a noise floor from the start of a recording to find where
//! speech begins and ends, these measures follow FluidAudio: they line up with token timestamps and
//! judge speech against the recording's own speech level, also on raw audio the VAD never saw.

use super::{SAMPLES_PER_FRAME, SECONDS_PER_FRAME};
use crate::audio::preprocess::{frame_rms, rms, sum_of_squares};
use crate::util::numeric::floor_usize;

/// ≈ −42 dBFS, FluidAudio's fixed speech gate before it became adaptive; louder frames always count
/// as speech, room noise included.
const SPEECH_RMS_CEILING: f32 = 0.008;
/// ≈ −66 dBFS: above dither and room tone, below quiet speech.
const SPEECH_RMS_FLOOR: f32 = 0.0005;
/// The gate sits ≈ 10.5 dB below the level of the louder quarter of the recording's frames.
const SPEECH_RMS_REFERENCE_SCALE: f32 = 0.3;
const SPEECH_RMS_REFERENCE_PERCENTILE: f64 = 0.75;

/// Mean square of the samples within `half_window` of `center`.
pub fn boundary_energy(samples: &[f32], center: usize, half_window: usize) -> f32 {
    let start = center.saturating_sub(half_window);
    let end = samples.len().min(center + half_window);
    if end <= start {
        return 0.0;
    }
    sum_of_squares(&samples[start..end]) / (end - start) as f32
}

/// End of the last frame louder than the quietest real speech, or the end of an all-quiet
/// recording.
pub fn speech_end(samples: &[f32]) -> usize {
    let mut end = samples.len();
    while end > 0 {
        let start = end.saturating_sub(SAMPLES_PER_FRAME);
        if rms(&samples[start..end]) >= SPEECH_RMS_FLOOR {
            return end;
        }
        end = start;
    }
    samples.len()
}

/// The RMS above which an encoder frame of this recording counts as speech.
pub struct SpeechGate(f32);

impl SpeechGate {
    /// Scaled to the recording's level, since an absolute gate never fires on quiet audio; digital
    /// silence is left out, as it would drag the reference down to the floor.
    pub fn new(samples: &[f32]) -> Self {
        let mut levels: Vec<f32> = frame_rms::<SAMPLES_PER_FRAME>(samples)
            .filter(|&level| level > 0.0)
            .collect();
        if levels.is_empty() {
            return Self(SPEECH_RMS_CEILING);
        }
        levels.sort_by(f32::total_cmp);
        let index = (levels.len() - 1).min(floor_usize(
            levels.len() as f64 * SPEECH_RMS_REFERENCE_PERCENTILE,
        ));
        Self(Self::scaled(levels[index]))
    }

    fn scaled(reference: f32) -> f32 {
        (reference * SPEECH_RMS_REFERENCE_SCALE).clamp(SPEECH_RMS_FLOOR, SPEECH_RMS_CEILING)
    }

    /// Seconds of speech-like frames between two sample offsets.
    pub fn speech_seconds(&self, samples: &[f32], start: usize, end: usize) -> f64 {
        let end = end.min(samples.len());
        if end <= start {
            return 0.0;
        }
        let speech_frames = frame_rms::<SAMPLES_PER_FRAME>(&samples[start..end])
            .filter(|&level| level > self.0)
            .count();
        speech_frames as f64 * SECONDS_PER_FRAME
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECOND: usize = 16_000;

    fn tone(amplitude: f32, range: std::ops::Range<usize>, samples: &mut [f32]) {
        for index in range {
            samples[index] = amplitude * (index as f32 * 0.1).sin();
        }
    }

    #[test]
    fn speech_ends_with_the_last_loud_frame() {
        let mut samples = vec![0.0; 4 * SECOND];
        tone(0.05, 0..2 * SECOND, &mut samples);
        let end = speech_end(&samples);
        assert!(end.abs_diff(2 * SECOND) <= SAMPLES_PER_FRAME, "{end}");
        assert_eq!(speech_end(&vec![0.0; SECOND]), SECOND);
    }

    #[test]
    fn silence_holds_no_speech() {
        let silence = vec![0.0; 4 * SECOND];
        assert_eq!(
            SpeechGate::new(&silence).speech_seconds(&silence, 0, silence.len()),
            0.0
        );
        assert_eq!(SpeechGate::new(&silence).0, SPEECH_RMS_CEILING);
    }

    #[test]
    fn a_loud_second_counts_as_one_second_of_speech() {
        let mut samples = vec![0.0; 4 * SECOND];
        tone(0.1, SECOND..2 * SECOND, &mut samples);
        let seconds = SpeechGate::new(&samples).speech_seconds(&samples, 0, samples.len());
        assert!((seconds - 1.0).abs() < 0.1, "{seconds}");
    }

    #[test]
    fn room_tone_in_a_normal_recording_is_not_speech() {
        let mut samples = vec![0.0; 5 * SECOND];
        tone(0.1, 0..3 * SECOND, &mut samples);
        tone(0.004, 3 * SECOND..5 * SECOND, &mut samples);
        let gate = SpeechGate::new(&samples);
        assert_eq!(
            gate.speech_seconds(&samples, 3 * SECOND, samples.len()),
            0.0
        );
    }

    #[test]
    fn quiet_speech_counts_in_a_quiet_recording() {
        let mut samples = vec![0.0; 4 * SECOND];
        tone(0.005, SECOND..2 * SECOND, &mut samples);
        let seconds = SpeechGate::new(&samples).speech_seconds(&samples, 0, samples.len());
        assert!((seconds - 1.0).abs() < 0.1, "{seconds}");
    }

    #[test]
    fn the_gate_follows_the_level_between_floor_and_ceiling() {
        assert_eq!(SpeechGate::scaled(0.07), SPEECH_RMS_CEILING);
        assert!((SpeechGate::scaled(0.004) - 0.0012).abs() < 1e-4);
        assert_eq!(SpeechGate::scaled(0.0), SPEECH_RMS_FLOOR);
    }

    #[test]
    fn digital_silence_is_left_out_of_the_reference() {
        let mut samples = vec![0.0; 4 * SECOND];
        tone(0.004, 0..4 * SECOND / 5, &mut samples);
        let gate = SpeechGate::new(&samples).0;
        assert!(gate > SPEECH_RMS_FLOOR);
        assert!((gate - 0.0028 * 0.3).abs() < 2e-4, "{gate}");
    }
}
