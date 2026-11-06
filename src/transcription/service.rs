//! From recorded samples to the text that gets pasted.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use super::Api;
use super::error::TranscriptionError;
use super::local::LocalEngine;
use super::model_store::ModelStore;
use super::online::{self, Patience, Upload};
use super::postprocess::{self, ActiveApp};
use crate::audio::{MIN_SAMPLES, SAMPLE_RATE, aac, preprocess, vad, wav};
use crate::config::settings::Settings;
use crate::config::vocabulary;
use crate::models::{LocalModel, TranscriptionMode};
use crate::util::filelog;

/// Below the lowest fundamental of speech (about 85 Hz): it removes rumble and mains hum, not voice.
const HIGH_PASS_HZ: f32 = 80.0;
/// About −26 dBFS: loud enough for every engine, with headroom below clipping.
const TARGET_RMS: f32 = 0.05;
/// At most +20 dB, so the noise of a near-silent recording is not raised to the level of speech.
const MAX_GAIN: f32 = 10.0;

/// Everything the pipeline talks to; tests replace each part.
pub struct Services<'a> {
    pub local: &'a dyn LocalEngine,
    pub models: &'a dyn ModelStore,
    pub api: Api<'a>,
}

pub struct Job {
    pub samples: Vec<f32>,
    pub settings: Settings,
    pub active_app: ActiveApp,
    /// Parakeet first gets the untouched recording; the preprocessed pipeline is the fallback.
    pub raw_parakeet_first: bool,
    pub vocabulary: Vec<String>,
}

pub struct Transcript {
    pub text: String,
    /// The online request failed and a local model transcribed instead; why it failed.
    pub local_fallback: Option<TranscriptionError>,
    pub post_processing: postprocess::Status,
}

pub enum Outcome {
    Text(Transcript),
    NoSpeech,
    TooShort,
}

/// The installed model that takes over when an online request fails: the one chosen for local
/// mode, else the installed one with the lowest `fallback_rank`.
pub fn fallback_model(chosen: LocalModel, installed: &HashSet<LocalModel>) -> Option<LocalModel> {
    if installed.contains(&chosen) {
        return Some(chosen);
    }
    installed
        .iter()
        .copied()
        .min_by_key(|model| model.spec().fallback_rank)
}

pub fn run(mut job: Job, services: &Services) -> Result<Outcome, TranscriptionError> {
    if too_short(&job.samples) {
        return Ok(Outcome::TooShort);
    }
    let raw = Arc::new(std::mem::take(&mut job.samples));
    if job.raw_parakeet_first
        && let Some(text) = parakeet_on_raw_audio(&job, Arc::clone(&raw), services)
    {
        return Ok(finish(&job, services.api, &text, None));
    }
    let audio = prepared(&raw, job.settings.preprocessing);
    drop(raw);
    if too_short(&audio) {
        return Ok(Outcome::TooShort);
    }

    let started = Instant::now();
    let seconds = audio.len() as f32 / SAMPLE_RATE as f32;
    let recognised = match job.settings.mode {
        TranscriptionMode::Online => transcribe_online(&job, audio, services)?,
        TranscriptionMode::Local => Recognised {
            text: transcribe_locally(&job, job.settings.local_model, audio, services)?,
            local_fallback: None,
        },
    };
    flog!(
        filelog::APP,
        "transcribe took {:.0}ms for {seconds:.1}s audio → {} chars",
        started.elapsed().as_secs_f64() * 1000.0,
        recognised.text.chars().count()
    );
    if recognised.text.trim().is_empty() {
        return Ok(Outcome::NoSpeech);
    }
    Ok(finish(
        &job,
        services.api,
        &recognised.text,
        recognised.local_fallback,
    ))
}

/// The one length check of the pipeline; no engine gets less than `MIN_SAMPLES`.
fn too_short(samples: &[f32]) -> bool {
    samples.len() < MIN_SAMPLES
}

/// The first try in local mode with Parakeet; `None` hands the dictation on to the preprocessed
/// pipeline.
fn parakeet_on_raw_audio(job: &Job, raw: Arc<Vec<f32>>, services: &Services) -> Option<String> {
    let started = Instant::now();
    let settings = job.settings;
    match services.local.transcribe_shared(
        raw,
        settings.local_model,
        settings.language.code(),
        None,
    ) {
        Ok(text) if !text.trim().is_empty() => {
            flog!(
                filelog::APP,
                "Parakeet on raw audio took {:.0}ms ({} chars)",
                started.elapsed().as_secs_f64() * 1000.0,
                text.chars().count()
            );
            Some(text)
        }
        Ok(_) => {
            flog!(
                filelog::APP,
                "No speech detected on raw audio, trying the preprocessed pipeline"
            );
            None
        }
        Err(error) => {
            flog!(
                filelog::APP,
                "Parakeet on raw audio failed: {error}, trying the preprocessed pipeline"
            );
            None
        }
    }
}

/// Without leading and trailing silence and, with preprocessing on, filtered and normalised.
fn prepared(raw: &[f32], preprocessing: bool) -> Vec<f32> {
    if !preprocessing {
        return vad::trim_silence(raw);
    }
    let trimmed = vad::trim_silence(&preprocess::high_pass_filter(
        raw,
        SAMPLE_RATE,
        HIGH_PASS_HZ,
    ));
    // After trimming, so the gain does not lift the noise floor the VAD measures.
    preprocess::normalize(&trimmed, TARGET_RMS, MAX_GAIN)
}

struct Recognised {
    text: String,
    local_fallback: Option<TranscriptionError>,
}

fn transcribe_online(
    job: &Job,
    audio: Vec<f32>,
    services: &Services,
) -> Result<Recognised, TranscriptionError> {
    let settings = job.settings;
    let encoded = Encoded::new(&audio);
    let fallback = fallback_model(settings.local_model, &services.models.installed());
    let patience = if fallback.is_some() {
        Patience::Brief
    } else {
        Patience::Full
    };
    let error = match online::transcribe(
        services.api,
        &encoded.upload(),
        settings.online_model,
        settings.language,
        &job.vocabulary,
        patience,
    ) {
        Ok(text) => {
            return Ok(Recognised {
                text,
                local_fallback: None,
            });
        }
        Err(error) => error,
    };
    let Some(model) = fallback else {
        return Err(error);
    };
    flog!(
        filelog::APP,
        "Online transcription failed ({error}), transcribing with {} instead",
        model.raw_value()
    );
    match transcribe_locally(job, model, audio, services) {
        Ok(text) => Ok(Recognised {
            text,
            local_fallback: Some(error),
        }),
        Err(local_error) => {
            flog!(filelog::APP, "Local fallback failed too: {local_error}");
            Err(error)
        }
    }
}

fn transcribe_locally(
    job: &Job,
    model: LocalModel,
    audio: Vec<f32>,
    services: &Services,
) -> Result<String, TranscriptionError> {
    services.local.transcribe(
        audio,
        model,
        job.settings.language.code(),
        vocabulary::prompt(&job.vocabulary),
    )
}

/// The recording as uploaded: AAC, or WAV where the system encoder fails.
struct Encoded {
    data: Vec<u8>,
    file_name: &'static str,
    mime_type: &'static str,
}

impl Encoded {
    fn new(samples: &[f32]) -> Self {
        match aac::compress_to_m4a(samples, SAMPLE_RATE) {
            Ok(data) => Self {
                data,
                file_name: "audio.m4a",
                mime_type: "audio/mp4",
            },
            Err(error) => {
                flog!(filelog::AUDIO, "AAC compression failed: {error:#}");
                let data = wav::encode(samples, SAMPLE_RATE);
                flog!(
                    filelog::APP,
                    "Sending {} KB WAV (AAC compression unavailable)",
                    data.len() / 1024
                );
                Self {
                    data,
                    file_name: "audio.wav",
                    mime_type: "audio/wav",
                }
            }
        }
    }

    fn upload(&self) -> Upload<'_> {
        Upload {
            data: &self.data,
            file_name: self.file_name,
            mime_type: self.mime_type,
        }
    }
}

fn finish(job: &Job, api: Api, text: &str, local_fallback: Option<TranscriptionError>) -> Outcome {
    let settings = job.settings;
    let processed = postprocess::process(
        api,
        text,
        settings.post_processing,
        settings.language,
        &job.active_app,
        &job.vocabulary,
    );
    Outcome::Text(Transcript {
        text: without_trailing_line_breaks(processed.text),
        local_fallback,
        post_processing: processed.status,
    })
}

/// Pasted into a terminal, a trailing line break would run the text as a command.
fn without_trailing_line_breaks(mut text: String) -> String {
    let end = text.trim_end_matches(['\n', '\r']).len();
    text.truncate(end);
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_prefers_the_chosen_model_then_the_best_multilingual_one() {
        let installed = |models: &[LocalModel]| models.iter().copied().collect::<HashSet<_>>();
        assert_eq!(
            fallback_model(
                LocalModel::Base,
                &installed(&[LocalModel::Base, LocalModel::ParakeetUltra])
            ),
            Some(LocalModel::Base)
        );
        assert_eq!(
            fallback_model(
                LocalModel::LargeV3,
                &installed(&[LocalModel::ParakeetV2, LocalModel::Small])
            ),
            Some(LocalModel::Small)
        );
        assert_eq!(
            fallback_model(
                LocalModel::Tiny,
                &installed(&[LocalModel::Medium, LocalModel::ParakeetUltra])
            ),
            Some(LocalModel::ParakeetUltra)
        );
        assert_eq!(fallback_model(LocalModel::Tiny, &installed(&[])), None);
    }

    #[test]
    fn pasted_text_ends_without_a_line_break() {
        assert_eq!(
            without_trailing_line_breaks("ls -la\n\r\n".into()),
            "ls -la"
        );
        assert_eq!(without_trailing_line_breaks("a\nb ".into()), "a\nb ");
    }
}
