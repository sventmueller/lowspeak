//! Clean-up before recognition: resampling to 16 kHz, a high-pass filter against rumble, then RMS
//! normalisation.

use std::f64::consts::{FRAC_1_SQRT_2, PI};

use anyhow::Result;
use rubato::audioadapter_buffers::direct::InterleavedSlice;
use rubato::{Fft, FixedSync, Resampler as _};

use super::SAMPLE_RATE;

/// The quality factor of a second-order Butterworth filter: maximally flat, −3 dB at the cutoff.
const BUTTERWORTH_Q: f64 = FRAC_1_SQRT_2;
const RESAMPLER_CHUNK_FRAMES: usize = 1_024;

pub fn sum_of_squares(samples: &[f32]) -> f32 {
    samples
        .iter()
        .fold(0.0f32, |sum, &sample| sum + sample * sample)
}

pub fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    (sum_of_squares(samples) / samples.len() as f32).sqrt()
}

/// RMS of each whole frame of `FRAME` samples; a shorter remainder at the end is skipped.
pub fn frame_rms<const FRAME: usize>(samples: &[f32]) -> impl Iterator<Item = f32> + '_ {
    samples
        .as_chunks::<FRAME>()
        .0
        .iter()
        .map(|frame| rms(frame))
}

/// Second-order Butterworth high-pass filter.
pub fn high_pass_filter(samples: &[f32], sample_rate: u32, cutoff_hz: f32) -> Vec<f32> {
    if samples.len() <= 2 {
        return samples.to_vec();
    }
    // The biquad of the Audio EQ Cookbook (Robert Bristow-Johnson).
    let w0 = 2.0 * PI * f64::from(cutoff_hz) / f64::from(sample_rate);
    let alpha = w0.sin() / (2.0 * BUTTERWORTH_Q);
    let cos = w0.cos();
    let a0 = 1.0 + alpha;
    let b0 = ((1.0 + cos) / 2.0) / a0;
    let b1 = -(1.0 + cos) / a0;
    let b2 = b0;
    let a1 = (-2.0 * cos) / a0;
    let a2 = (1.0 - alpha) / a0;

    let (mut x1, mut x2, mut y1, mut y2) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    samples
        .iter()
        .map(|&sample| {
            let x0 = f64::from(sample);
            let y0 = b0 * x0 + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
            (x2, x1, y2, y1) = (x1, x0, y1, y0);
            #[expect(
                clippy::cast_possible_truncation,
                reason = "the filter state runs in f64 for stability; its output is f32 audio"
            )]
            let output = y0 as f32;
            output
        })
        .collect()
}

/// Scales to a target RMS (gain capped) and soft-clips peaks with tanh.
pub fn normalize(samples: &[f32], target_rms: f32, max_gain: f32) -> Vec<f32> {
    let level = rms(samples);
    if level <= 1e-6 {
        return samples.to_vec();
    }
    let gain = (target_rms / level).min(max_gain);
    samples
        .iter()
        .map(|sample| (sample * gain).tanh())
        .collect()
}

/// Resamples mono audio from `from_rate` to [`SAMPLE_RATE`].
pub fn resample(samples: &[f32], from_rate: u32) -> Result<Vec<f32>> {
    if from_rate == SAMPLE_RATE || samples.is_empty() {
        return Ok(samples.to_vec());
    }
    let mut resampler = Fft::<f32>::new(
        usize::try_from(from_rate)?,
        usize::try_from(SAMPLE_RATE)?,
        RESAMPLER_CHUNK_FRAMES,
        1,
        FixedSync::Both,
    )?;
    let input = InterleavedSlice::new(samples, 1, samples.len())?;
    let output = resampler.process_all(&input, samples.len(), None)?;
    Ok(output.take_data())
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, PI};

    use super::*;
    use crate::util::numeric::floor_usize;

    const RATE: f32 = 16_000.0;

    fn sine(frequency: f32, seconds: f32) -> Vec<f32> {
        (0..floor_usize(f64::from(RATE * seconds)))
            .map(|i| 0.5 * (2.0 * PI * frequency * i as f32 / RATE).sin())
            .collect()
    }

    /// Steady-state gain of the 80 Hz filter at `frequency`, in dB.
    fn gain_db(frequency: f32) -> f32 {
        let input = sine(frequency, 3.0);
        let output = high_pass_filter(&input, SAMPLE_RATE, 80.0);
        let settled = floor_usize(f64::from(RATE));
        20.0 * (rms(&output[settled..]) / rms(&input[settled..])).log10()
    }

    #[test]
    fn high_pass_has_the_butterworth_magnitude_response() {
        // |H(f)|² = 1 / (1 + (fc / f)⁴) for a second-order Butterworth high-pass.
        for (frequency, expected) in [
            (20.0, -24.10),
            (60.0, -6.19),
            (80.0, -3.01),
            (200.0, -0.11),
            (1_000.0, 0.0),
        ] {
            let gain = gain_db(frequency);
            assert!(
                (gain - expected).abs() < 0.15,
                "{frequency} Hz: {gain:.2} dB, expected {expected} dB"
            );
        }
    }

    #[test]
    fn high_pass_does_not_boost_near_the_cutoff() {
        for frequency in [70.0, 80.0, 92.0, 110.0, 150.0] {
            assert!(gain_db(frequency) < 0.0, "{frequency} Hz is boosted");
        }
    }

    #[test]
    fn high_pass_removes_rumble_and_keeps_speech_band() {
        let rumble = high_pass_filter(&sine(20.0, 1.0), SAMPLE_RATE, 80.0);
        let voice = high_pass_filter(&sine(1_000.0, 1.0), SAMPLE_RATE, 80.0);
        assert!(rms(&rumble[8_000..]) < 0.03);
        assert!((rms(&voice[8_000..]) - rms(&sine(1_000.0, 0.5))).abs() < 0.01);
    }

    #[test]
    fn normalize_reaches_target_level_and_leaves_silence_alone() {
        let quiet: Vec<f32> = sine(440.0, 1.0).iter().map(|s| s * 0.05).collect();
        assert!((rms(&normalize(&quiet, 0.05, 10.0)) - 0.05).abs() < 0.005);
        let silence = vec![0.0; 1_000];
        assert_eq!(normalize(&silence, 0.05, 10.0), silence);
    }

    #[test]
    fn frame_rms_measures_whole_frames_only() {
        let samples = [vec![0.5; 4], vec![0.0; 4], vec![1.0; 3]].concat();
        assert_eq!(frame_rms::<4>(&samples).collect::<Vec<_>>(), [0.5, 0.0]);
    }

    #[test]
    fn resampling_keeps_duration_and_tone() {
        let tone: Vec<f32> = (0..48_000)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 48_000.0).sin())
            .collect();
        let out = resample(&tone, 48_000).unwrap();
        assert!(out.len().abs_diff(16_000) <= 2, "got {} samples", out.len());
        let level = rms(&out[1_000..15_000]);
        assert!((level - FRAC_1_SQRT_2).abs() < 0.02, "rms {level}");
        assert_eq!(resample(&tone, SAMPLE_RATE).unwrap(), tone);
    }
}
