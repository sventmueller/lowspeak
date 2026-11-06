//! End-to-end transcription with locally installed models:
//! `cargo test --release --test transcription -- --ignored` (filter with `parakeet` or `whisper`).
//! The expected Parakeet texts are the output of FluidAudio 0.17.7 for the same audio.

use std::path::Path;

use lowspeak::audio::wav;
use lowspeak::config::settings::Settings;
use lowspeak::models::{Language, LocalModel, OnlineModel, PostProcessingMode, TranscriptionMode};
use lowspeak::transcription::Api;
use lowspeak::transcription::local::LocalTranscriber;
use lowspeak::transcription::model_store::{self, Disk};
use lowspeak::transcription::parakeet::{Parakeet, Version};
use lowspeak::transcription::postprocess::ActiveApp;
use lowspeak::transcription::service::{self, Job, Outcome, Services};

fn fixture() -> Vec<f32> {
    let bytes =
        std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/builtin.wav"))
            .unwrap();
    let audio = wav::decode(&bytes).unwrap();
    assert_eq!(audio.sample_rate, 16_000);
    audio.samples
}

fn require(model: LocalModel) {
    assert!(
        model_store::is_installed(model),
        "{} is not installed; download it in the app first",
        model.raw_value()
    );
}

fn parakeet_ultra() -> Parakeet {
    require(LocalModel::ParakeetUltra);
    Parakeet::load(
        &model_store::location(LocalModel::ParakeetUltra),
        Version::Ultra,
    )
    .unwrap()
}

#[test]
#[ignore = "needs Parakeet Ultra installed"]
fn parakeet_ultra_matches_fluidaudio_on_short_audio() {
    assert_eq!(
        parakeet_ultra().transcribe(&fixture()).unwrap(),
        "Test 12345. Test 1234."
    );
}

#[test]
#[ignore = "needs Parakeet Ultra installed"]
fn parakeet_ultra_matches_fluidaudio_on_audio_longer_than_one_window() {
    let samples = fixture().repeat(4);
    assert_eq!(
        parakeet_ultra().transcribe(&samples).unwrap(),
        "Test 12345. Test 1, 2, 3. Test 1, 2, 3, 4, 5. Test 1, 2, 3. Test 1, 2, 3, 4, 5. \
         Test 1, 2, 3. Test 1, 2, 3, 4, 5. Test 1, 2, 3, 5."
    );
}

fn local_settings(model: LocalModel) -> Settings {
    Settings {
        mode: TranscriptionMode::Local,
        local_model: model,
        online_model: OnlineModel::GptTranscribe,
        language: Language::Auto,
        post_processing: PostProcessingMode::Off,
        preprocessing: true,
        start_sound: false,
    }
}

fn run_pipeline(
    model: LocalModel,
    samples: Vec<f32>,
    raw_parakeet_first: bool,
    vocabulary: Vec<String>,
) -> Outcome {
    let local = LocalTranscriber::spawn();
    let job = Job {
        samples,
        settings: local_settings(model),
        active_app: ActiveApp::default(),
        raw_parakeet_first,
        vocabulary,
    };
    let services = Services {
        local: &local,
        models: &Disk,
        api: Api::SYSTEM,
    };
    service::run(job, &services).unwrap()
}

#[test]
#[ignore = "needs Parakeet Ultra installed"]
fn pipeline_transcribes_with_parakeet() {
    require(LocalModel::ParakeetUltra);
    for raw_first in [true, false] {
        match run_pipeline(LocalModel::ParakeetUltra, fixture(), raw_first, Vec::new()) {
            Outcome::Text(transcript) => {
                assert!(
                    transcript.text.starts_with("Test 12345."),
                    "unexpected text: {}",
                    transcript.text
                );
                assert!(transcript.local_fallback.is_none());
            }
            _ => panic!("expected text"),
        }
    }
}

#[test]
#[ignore = "needs Parakeet Ultra installed"]
fn pipeline_with_parakeet_reports_silence_and_short_audio() {
    require(LocalModel::ParakeetUltra);
    assert!(matches!(
        run_pipeline(
            LocalModel::ParakeetUltra,
            vec![0.0; 32_000],
            true,
            Vec::new()
        ),
        Outcome::TooShort
    ));
    assert!(matches!(
        run_pipeline(
            LocalModel::ParakeetUltra,
            fixture()[..2_000].to_vec(),
            true,
            Vec::new()
        ),
        Outcome::TooShort
    ));
}

#[test]
#[ignore = "needs a Whisper model installed (Tiny, Base, Small or Large v3 Turbo)"]
fn pipeline_transcribes_with_whisper() {
    let model = [
        LocalModel::Tiny,
        LocalModel::Base,
        LocalModel::Small,
        LocalModel::LargeV3Turbo,
    ]
    .into_iter()
    .find(|&model| model_store::is_installed(model))
    .expect("no Whisper model is installed; download one in the app first");
    for vocabulary in [
        Vec::new(),
        vec!["Testlauf".to_owned(), "Lowspeak".to_owned()],
    ] {
        match run_pipeline(model, fixture(), false, vocabulary) {
            Outcome::Text(transcript) => {
                let lower = transcript.text.to_lowercase();
                assert!(
                    lower.contains("test")
                        && (lower.contains('3')
                            || lower.contains("drei")
                            || lower.contains("three")),
                    "unexpected text: {}",
                    transcript.text
                );
            }
            _ => panic!("expected text"),
        }
    }
}
