//! NVIDIA Parakeet TDT speech recognition on the CoreML models published by FluidAudio.
//!
//! This module is a Rust translation of parts of FluidAudio 0.17.7 (Swift, Apache-2.0; the license is in
//! `LICENSE-FluidAudio` next to this file): the decoding, the long-form windows and their merge, and the vocabulary.
//! Thresholds and limits are FluidAudio's unless their documentation calls them Lowspeak's own tuning. Not translated:
//! FluidAudio's retries of a window that decodes to nothing (its issue #909).

mod chunks;
mod coreml;
mod decoder;
mod energy;
mod layout;
mod merge;
mod networks;
mod seams;
mod text;

use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, ensure};

use self::chunks::{WindowDecoder, WindowRequest};
use self::coreml::{ComputeUnits, Model};
use self::decoder::{EncoderFrames, Placement, Window};
use self::layout::Windowing;
use self::networks::CoreMlNetworks;
use self::text::Vocabulary;
use crate::audio::{MIN_SAMPLES, SAMPLE_RATE};

/// The encoder takes exactly 15 s of audio per call.
pub const MAX_MODEL_SAMPLES: usize = 240_000;
const MEL_HOP: usize = 160;
const ENCODER_SUBSAMPLING: usize = 8;
const SAMPLES_PER_FRAME: usize = MEL_HOP * ENCODER_SUBSAMPLING;
const SECONDS_PER_FRAME: f64 = SAMPLES_PER_FRAME as f64 / SAMPLE_RATE as f64;
const ENCODER_HIDDEN: usize = 1_024;
const DECODER_HIDDEN: usize = 640;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    V2,
    /// Parakeet Ultra, a post-trained v3 with v3's files, vocabulary and decoding.
    Ultra,
}

impl Version {
    fn blank_id(self) -> usize {
        match self {
            Self::V2 => 1_024,
            Self::Ultra => 8_192,
        }
    }

    fn joint_file(self) -> &'static str {
        match self {
            Self::V2 => "JointDecision.mlmodelc",
            Self::Ultra => "JointDecisionv3.mlmodelc",
        }
    }

    fn windowing(self) -> Windowing {
        match self {
            Self::V2 => Windowing::MelContext,
            Self::Ultra => Windowing::SilenceAligned,
        }
    }

    pub fn repo(self) -> &'static str {
        match self {
            Self::V2 => "FluidInference/parakeet-tdt-0.6b-v2-coreml",
            Self::Ultra => "FluidInference/parakeet-ultra-coreml",
        }
    }

    /// Directory under FluidAudio's cache, so models downloaded by FluidAudio itself are reused.
    pub fn folder_name(self) -> &'static str {
        match self {
            Self::V2 => "parakeet-tdt-0.6b-v2",
            Self::Ultra => "parakeet-ultra",
        }
    }

    pub fn required_bundles(self) -> [&'static str; 4] {
        [
            "Preprocessor.mlmodelc",
            "Encoder.mlmodelc",
            "Decoder.mlmodelc",
            self.joint_file(),
        ]
    }

    pub const VOCABULARY_FILE: &'static str = "parakeet_vocab.json";
}

/// `~/Library/Application Support/FluidAudio/Models`.
pub fn models_root() -> PathBuf {
    crate::util::paths::application_support()
        .join("FluidAudio")
        .join("Models")
}

pub fn model_dir(version: Version) -> PathBuf {
    models_root().join(version.folder_name())
}

pub fn is_installed(dir: &Path, version: Version) -> bool {
    version
        .required_bundles()
        .iter()
        .all(|bundle| dir.join(bundle).join("coremldata.bin").is_file())
        && dir.join(Version::VOCABULARY_FILE).is_file()
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub id: usize,
    pub timestamp: i64,
    pub confidence: f32,
    pub duration: i64,
    /// Frame at which the decoded window starts; tokens of different windows meet at a seam.
    pub window_start: i64,
}

pub struct Parakeet {
    version: Version,
    preprocessor: Model,
    encoder: Model,
    decoder: Model,
    joint: Model,
    vocabulary: Vocabulary,
}

impl Parakeet {
    pub fn load(dir: &Path, version: Version) -> Result<Self> {
        let [preprocessor, encoder, decoder, joint] = version.required_bundles();
        Ok(Self {
            version,
            // FluidAudio measured every preprocessor op on the CPU anyway.
            preprocessor: Model::load(&dir.join(preprocessor), ComputeUnits::CPUOnly)?,
            encoder: Model::load(&dir.join(encoder), ComputeUnits::CPUAndNeuralEngine)?,
            decoder: Model::load(&dir.join(decoder), ComputeUnits::CPUAndNeuralEngine)?,
            joint: Model::load(&dir.join(joint), ComputeUnits::CPUAndNeuralEngine)?,
            vocabulary: Vocabulary::load(&dir.join(Version::VOCABULARY_FILE))?,
        })
    }

    /// Transcribes 16 kHz mono audio of at least 0.3 s.
    pub fn transcribe(&self, samples: &[f32]) -> Result<String> {
        ensure!(samples.len() >= MIN_SAMPLES, "audio shorter than 0.3 s");
        let tokens = if samples.len() <= MAX_MODEL_SAMPLES {
            let aligned = samples.len().div_ceil(SAMPLES_PER_FRAME) * SAMPLES_PER_FRAME;
            let length = if aligned <= MAX_MODEL_SAMPLES {
                aligned
            } else {
                samples.len()
            };
            self.infer(
                samples,
                length,
                Placement {
                    actual_audio_frames: length.div_ceil(SAMPLES_PER_FRAME),
                    context_frame_adjustment: 0,
                    is_last_chunk: true,
                    global_frame_offset: 0,
                    emit_tokens_after_global_frame: None,
                    initial_time_index_override: None,
                },
            )?
        } else {
            chunks::transcribe(samples, self, &self.vocabulary, self.version.windowing())?
        };
        let ids: Vec<usize> = tokens.iter().map(|token| token.id).collect();
        Ok(self.vocabulary.decode(&ids))
    }

    fn infer(
        &self,
        samples: &[f32],
        original_length: usize,
        placement: Placement,
    ) -> Result<Vec<Token>> {
        ensure!(
            samples.len() <= MAX_MODEL_SAMPLES,
            "a window of {} samples exceeds the encoder's {MAX_MODEL_SAMPLES}",
            samples.len()
        );
        let mut padded = samples.to_vec();
        padded.resize(MAX_MODEL_SAMPLES, 0.0);
        let audio = coreml::f32_array(&[1, MAX_MODEL_SAMPLES], &padded)?;
        let length = coreml::i32_array(&[1], &[i32::try_from(original_length)?])?;
        let mel = self
            .preprocessor
            .predict(&[("audio_signal", &audio), ("audio_length", &length)])?;
        let (features, feature_length) = (mel.array("mel")?, mel.array("mel_length")?);
        let encoded = self
            .encoder
            .predict(&[("mel", &features), ("mel_length", &feature_length)])?;
        let encoder_length =
            coreml::read_i32(&*encoded.array("encoder_length")?, "encoder length")?;
        let encoder_length = encoder_length
            .data
            .first()
            .copied()
            .context("empty encoder length")?;
        let encoder_sequence_length = usize::try_from(encoder_length)
            .with_context(|| format!("negative encoder length {encoder_length}"))?;
        if encoder_sequence_length == 0 {
            return Ok(Vec::new());
        }
        let output = coreml::read_f32(&*encoded.array("encoder")?, "encoder output")?;
        let frames = EncoderFrames::new(&output, encoder_sequence_length)?;
        let networks = CoreMlNetworks::new(&self.decoder, &self.joint)?;
        decoder::decode(
            &networks,
            self.version.blank_id(),
            &Window {
                frames: &frames,
                encoder_sequence_length,
                placement,
            },
        )
    }
}

impl WindowDecoder for Parakeet {
    fn decode_window(&self, request: WindowRequest) -> Result<Vec<Token>> {
        if request.samples.is_empty() {
            return Ok(Vec::new());
        }
        self.infer(request.samples, request.samples.len(), placement(&request)?)
    }
}

/// Where a window's encoder frames sit in the recording. Its first frame is the mel context, if any,
/// so a token's frame counts from there; FluidAudio 0.17.7 counts from after it and stamps v2 tokens
/// one frame late.
fn placement(request: &WindowRequest) -> Result<Placement> {
    let actual_samples = request
        .samples
        .len()
        .checked_sub(request.context_samples)
        .context("window context is longer than the window")?;
    let first_sample = request
        .chunk_start
        .checked_sub(request.context_samples)
        .context("window context reaches before the recording")?;
    Ok(Placement {
        actual_audio_frames: actual_samples.div_ceil(SAMPLES_PER_FRAME),
        context_frame_adjustment: request.context_samples / SAMPLES_PER_FRAME,
        is_last_chunk: request.is_last_chunk,
        global_frame_offset: i64::try_from(first_sample / SAMPLES_PER_FRAME)?,
        emit_tokens_after_global_frame: request.emit_tokens_after_frame,
        initial_time_index_override: request.initial_time_index_override,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_window_with_mel_context_counts_its_frames_from_the_context() {
        let samples = vec![0.0; 11 * SAMPLES_PER_FRAME];
        let request = WindowRequest {
            samples: &samples,
            context_samples: SAMPLES_PER_FRAME,
            chunk_start: 100 * SAMPLES_PER_FRAME,
            is_last_chunk: true,
            emit_tokens_after_frame: None,
            initial_time_index_override: None,
        };
        let at = placement(&request).unwrap();
        assert_eq!(
            (
                at.global_frame_offset,
                at.context_frame_adjustment,
                at.actual_audio_frames
            ),
            (99, 1, 10)
        );
        let first = WindowRequest {
            context_samples: 0,
            chunk_start: 0,
            ..request
        };
        assert_eq!(placement(&first).unwrap().global_frame_offset, 0);
        let impossible = WindowRequest {
            chunk_start: 0,
            ..request
        };
        assert!(placement(&impossible).is_err());
    }

    #[test]
    fn an_encoder_frame_is_80_ms() {
        assert_eq!(SAMPLES_PER_FRAME, 1_280);
        assert!((SECONDS_PER_FRAME - 0.08).abs() < 1e-12);
    }
}
