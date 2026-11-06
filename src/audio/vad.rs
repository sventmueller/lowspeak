//! Energy-based voice activity detection that trims leading and trailing silence. Its thresholds
//! are Lowspeak's own tuning.

use super::preprocess::{frame_rms, rms};
use crate::util::filelog;
use crate::util::numeric::floor_usize;

/// 0.3 s: enough ramp-up for a word onset.
pub const MARGIN_SAMPLES: usize = 4_800;

/// 30 ms.
const FRAME_SAMPLES: usize = 480;
const HANGOVER_FRAMES: usize = 10;
const MIN_SPEECH_FRAMES: usize = 3;
/// The noise floor starts from the first 1.5 s of sound.
const CALIBRATION_FRAMES: usize = 50;
/// The quieter tail of the calibration frames estimates the noise floor even when speech starts at
/// once.
const NOISE_FLOOR_PERCENTILE: f64 = 0.2;
/// Frames this quiet are digital silence, which Bluetooth headsets deliver while they switch
/// profiles; they say nothing about the room's noise.
const DIGITAL_SILENCE_RMS: f32 = 1e-6;
/// The tracked noise floor follows louder noise by 0.5 % per frame, so speech cannot drag it up.
const NOISE_FLOOR_RISE: f32 = 0.005;
const ENERGY_MULTIPLIER: f32 = 3.0;
const MIN_DYNAMIC_THRESHOLD: f32 = 0.005;
const MAX_DYNAMIC_THRESHOLD: f32 = 0.02;
/// Zero crossings per sample: hum crosses rarely, hiss almost every sample, speech in between.
const MIN_VOICE_ZCR: f32 = 0.02;
const MAX_VOICE_ZCR: f32 = 0.6;
/// A recording this loud overall is kept whole when no frame passes as speech, instead of dropped.
const MIN_FALLBACK_RMS: f32 = 0.008;

fn zero_crossing_rate(frame: &[f32]) -> f32 {
    let mut crossings = 0;
    let mut previous = frame[0] > 0.0;
    for &sample in &frame[1..] {
        let sign = sample > 0.0;
        if sign != previous {
            crossings += 1;
            previous = sign;
        }
    }
    crossings as f32 / frame.len() as f32
}

fn is_digital_silence(energy: f32) -> bool {
    energy <= DIGITAL_SILENCE_RMS
}

fn initial_noise_floor(energies: &[f32]) -> f32 {
    let mut calibration: Vec<f32> = energies
        .iter()
        .copied()
        .filter(|&energy| !is_digital_silence(energy))
        .take(CALIBRATION_FRAMES)
        .collect();
    if calibration.is_empty() {
        return DIGITAL_SILENCE_RMS;
    }
    calibration.sort_by(f32::total_cmp);
    calibration[floor_usize((calibration.len() - 1) as f64 * NOISE_FLOOR_PERCENTILE)]
}

pub fn trim_silence(samples: &[f32]) -> Vec<f32> {
    if samples.len() <= FRAME_SAMPLES {
        return samples.to_vec();
    }
    let frames = samples.as_chunks::<FRAME_SAMPLES>().0;
    let energies: Vec<f32> = frame_rms::<FRAME_SAMPLES>(samples).collect();

    let mut noise_floor = initial_noise_floor(&energies);
    let mut speech = vec![false; frames.len()];
    let mut consecutive = 0;
    for (index, (frame, &energy)) in frames.iter().zip(&energies).enumerate() {
        if !is_digital_silence(energy) {
            noise_floor = (noise_floor + NOISE_FLOOR_RISE * noise_floor).min(energy);
        }
        let threshold =
            (noise_floor * ENERGY_MULTIPLIER).clamp(MIN_DYNAMIC_THRESHOLD, MAX_DYNAMIC_THRESHOLD);
        let zcr = zero_crossing_rate(frame);
        if energy > threshold && zcr > MIN_VOICE_ZCR && zcr < MAX_VOICE_ZCR {
            consecutive += 1;
            if consecutive >= MIN_SPEECH_FRAMES {
                speech[index + 1 - consecutive..=index].fill(true);
            }
        } else {
            consecutive = 0;
        }
    }

    let mut hangover = 0;
    for flag in &mut speech {
        if *flag {
            hangover = HANGOVER_FRAMES;
        } else if hangover > 0 {
            *flag = true;
            hangover -= 1;
        }
    }

    let (Some(first), Some(last)) = (
        speech.iter().position(|&s| s),
        speech.iter().rposition(|&s| s),
    ) else {
        let overall = rms(samples);
        if overall >= MIN_FALLBACK_RMS {
            flog!(
                filelog::AUDIO,
                "VAD fallback engaged (overall RMS {overall:.4})"
            );
            return samples.to_vec();
        }
        return Vec::new();
    };
    let start = (first * FRAME_SAMPLES).saturating_sub(MARGIN_SAMPLES);
    let end = samples
        .len()
        .min((last + 1) * FRAME_SAMPLES + MARGIN_SAMPLES);
    samples[start..end].to_vec()
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    const SAMPLE_RATE: f32 = 16_000.0;

    fn samples_for(seconds: f32) -> usize {
        floor_usize(f64::from(seconds * SAMPLE_RATE))
    }

    fn sine(amplitude: f32, seconds: f32) -> Vec<f32> {
        (0..samples_for(seconds))
            .map(|i| amplitude * (2.0 * PI * 220.0 * i as f32 / SAMPLE_RATE).sin())
            .collect()
    }

    /// White noise of the given RMS from a fixed xorshift sequence.
    fn noise(level: f32, seconds: f32, seed: u64) -> Vec<f32> {
        let mut state = seed.max(1);
        let amplitude = level * 3f32.sqrt();
        (0..samples_for(seconds))
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let uniform = (state >> 40) as f32 / (1u64 << 24) as f32;
                amplitude * (2.0 * uniform - 1.0)
            })
            .collect()
    }

    /// A voiced sound: a 250 Hz tone with falling harmonics under a 4 Hz syllable envelope.
    fn voice(level: f32, seconds: f32) -> Vec<f32> {
        (0..samples_for(seconds))
            .map(|i| {
                let t = i as f32 / SAMPLE_RATE;
                let envelope = 0.6 - 0.4 * (2.0 * PI * 4.0 * t).cos();
                let tone: f32 = (1..=5)
                    .map(|harmonic| {
                        let harmonic = harmonic as f32;
                        (2.0 * PI * 250.0 * harmonic * t).sin() / harmonic
                    })
                    .sum();
                level * envelope * tone
            })
            .collect()
    }

    fn mix(signal: &[f32], noise: &[f32]) -> Vec<f32> {
        signal.iter().zip(noise).map(|(a, b)| a + b).collect()
    }

    #[test]
    fn keeps_speech_that_starts_immediately() {
        let trimmed = trim_silence(&sine(0.12, 2.0));
        assert!(trimmed.len() > samples_for(1.5));
    }

    #[test]
    fn trims_leading_and_trailing_silence() {
        let silence = vec![0.0; samples_for(0.8)];
        let clip = [silence.clone(), sine(0.10, 1.5), silence].concat();
        let trimmed = trim_silence(&clip);
        assert!(trimmed.len() < clip.len());
        assert!(trimmed.len() > samples_for(1.2));
    }

    #[test]
    fn drops_pure_silence() {
        assert!(trim_silence(&vec![0.0; 16_000]).is_empty());
    }

    /// 1 s of room noise, 1.5 s of speech in that noise, 1.5 s of room noise.
    fn speech_in_room_noise() -> Vec<f32> {
        let speech = voice(0.08, 1.5);
        [
            noise(0.01, 1.0, 1),
            mix(&speech, &noise(0.01, 1.5, 2)),
            noise(0.01, 1.5, 3),
        ]
        .concat()
    }

    #[test]
    fn trims_room_noise_around_speech() {
        let clip = speech_in_room_noise();
        let trimmed = trim_silence(&clip);
        assert!(trimmed.len() >= samples_for(1.5), "speech was cut");
        assert!(
            trimmed.len() < samples_for(3.0),
            "kept {:.2} s of 4 s",
            trimmed.len() as f32 / SAMPLE_RATE
        );
    }

    #[test]
    fn digital_silence_does_not_pin_the_noise_floor() {
        let clip = speech_in_room_noise();
        let dropout = vec![0.0; FRAME_SAMPLES * 2];
        let middle = samples_for(0.5);
        let with_zeros = [
            vec![0.0; samples_for(0.3)],
            clip[..middle].to_vec(),
            dropout,
            clip[middle..].to_vec(),
        ]
        .concat();
        let trimmed = trim_silence(&with_zeros);
        assert!(trimmed.len() >= samples_for(1.5), "speech was cut");
        assert!(
            trimmed.len() < samples_for(3.0),
            "kept {:.2} s of {:.2} s",
            trimmed.len() as f32 / SAMPLE_RATE,
            with_zeros.len() as f32 / SAMPLE_RATE
        );
    }

    #[test]
    fn room_noise_alone_is_dropped() {
        assert!(trim_silence(&noise(0.003, 2.0, 7)).is_empty());
    }
}
