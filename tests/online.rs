//! Calls the paid APIs with the keys stored by the app: `cargo test --release --test online -- --ignored`.

use std::path::Path;
use std::sync::Arc;

use lowspeak::audio::wav;
use lowspeak::config::api_keys;
use lowspeak::config::settings::Settings;
use lowspeak::models::{
    ApiProvider, Language, LocalModel, OnlineModel, PostProcessingMode, TranscriptionMode,
};
use lowspeak::transcription::Api;
use lowspeak::transcription::local::LocalTranscriber;
use lowspeak::transcription::model_store::Disk;
use lowspeak::transcription::postprocess::{self, ActiveApp};
use lowspeak::transcription::service::{self, Job, Outcome, Services};
use lowspeak::transcription::spelling::{self, Lexicon};

fn require_key(provider: ApiProvider) {
    assert!(
        api_keys::load(provider).is_some(),
        "no {} key stored; set it in the app first",
        provider.display_name()
    );
}

#[test]
#[ignore = "sends text to Groq with the stored API key"]
fn groq_cleans_up_dictation() {
    require_key(ApiProvider::Groq);
    let text = postprocess::post_process(
        Api::SYSTEM,
        "ähm also das Meeting ist am Dienstag nein am Mittwoch",
        PostProcessingMode::Cleanup,
        Language::De,
        &ActiveApp::default(),
        &[],
    )
    .unwrap();
    println!("{text}");
    assert!(
        text.contains("Mittwoch") && !text.contains("ähm"),
        "unexpected text: {text}"
    );
}

#[test]
#[ignore = "sends text to Groq with the stored API key"]
fn groq_post_processing_spells_vocabulary_terms() {
    require_key(ApiProvider::Groq);
    let text = postprocess::post_process(
        Api::SYSTEM,
        "die migration auf post gres und träfik läuft morgen",
        PostProcessingMode::Cleanup,
        Language::De,
        &ActiveApp::default(),
        &["Postgres".into(), "Traefik".into()],
    )
    .unwrap();
    println!("{text}");
    assert!(
        text.contains("Postgres") && text.contains("Traefik"),
        "unexpected text: {text}"
    );
}

fn transcribe_fixture(
    online_model: OnlineModel,
    language: Language,
    post_processing: PostProcessingMode,
    vocabulary: &[&str],
) -> String {
    let bytes =
        std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/builtin.wav"))
            .unwrap();
    let samples = wav::decode(&bytes).unwrap().samples;
    transcribe(samples, online_model, language, post_processing, vocabulary)
}

/// 16 kHz speech from macOS's speech synthesis.
fn spoken(voice: &str, text: &str) -> Vec<f32> {
    let path = std::env::temp_dir().join(format!(
        "lowspeak-online-say-{}-{}.wav",
        std::process::id(),
        text.len()
    ));
    let status = std::process::Command::new("say")
        .args([
            "-v",
            voice,
            "--file-format=WAVE",
            "--data-format=LEI16@16000",
            "-o",
        ])
        .arg(&path)
        .arg(text)
        .status()
        .unwrap();
    assert!(status.success(), "say failed");
    let audio = wav::decode(&std::fs::read(&path).unwrap()).unwrap();
    std::fs::remove_file(&path).unwrap();
    audio.samples
}

fn transcribe(
    samples: Vec<f32>,
    online_model: OnlineModel,
    language: Language,
    post_processing: PostProcessingMode,
    vocabulary: &[&str],
) -> String {
    let settings = Settings {
        mode: TranscriptionMode::Online,
        local_model: LocalModel::ParakeetUltra,
        online_model,
        language,
        post_processing,
        preprocessing: true,
        start_sound: false,
    };
    let job = Job {
        samples: Arc::new(samples),
        settings,
        active_app: ActiveApp::default(),
        raw_parakeet_first: false,
        vocabulary: vocabulary.iter().map(|term| term.to_string()).collect(),
    };
    let local = LocalTranscriber::spawn();
    let services = Services {
        local: &local,
        models: &Disk,
        api: Api::SYSTEM,
        lexicon: &MacosWords,
    };
    match service::run(job, &services).unwrap() {
        Outcome::Text(transcript) => {
            println!("{}", transcript.text);
            assert!(
                transcript.local_fallback.is_none(),
                "the online request failed: {:?}",
                transcript.local_fallback
            );
            transcript.text
        }
        _ => panic!("expected text"),
    }
}

#[test]
#[ignore = "sends audio to Groq with the stored API key"]
fn groq_transcribes_compressed_upload_and_post_processes() {
    require_key(ApiProvider::Groq);
    let text = transcribe_fixture(
        OnlineModel::GroqWhisperLargeV3Turbo,
        Language::Auto,
        PostProcessingMode::Cleanup,
        &[],
    );
    assert!(
        text.to_lowercase().contains("test"),
        "unexpected text: {text}"
    );
}

#[test]
#[ignore = "sends audio to OpenAI with the stored API key"]
fn openai_transcribes_compressed_upload_with_language() {
    require_key(ApiProvider::OpenAi);
    let text = transcribe_fixture(
        OnlineModel::GptTranscribe,
        Language::De,
        PostProcessingMode::Off,
        &[],
    );
    assert!(
        text.to_lowercase().contains("test"),
        "unexpected text: {text}"
    );
}

#[test]
#[ignore = "sends audio to OpenAI and Groq with the stored API keys"]
fn both_providers_accept_the_vocabulary() {
    let vocabulary = ["Postgres", "Traefik", "Lowspeak"];
    // Every model gets its request, so one provider's failure does not hide the other's result.
    let failed: Vec<&str> = [
        (ApiProvider::OpenAi, OnlineModel::GptTranscribe),
        (ApiProvider::OpenAi, OnlineModel::Whisper1),
        (ApiProvider::Groq, OnlineModel::GroqWhisperLargeV3),
    ]
    .into_iter()
    .filter(|&(provider, model)| {
        std::panic::catch_unwind(|| {
            require_key(provider);
            let text =
                transcribe_fixture(model, Language::De, PostProcessingMode::Off, &vocabulary);
            assert!(
                text.to_lowercase().contains("test"),
                "unexpected text from {}: {text}",
                model.raw_value()
            );
        })
        .is_err()
    })
    .map(|(_, model)| model.raw_value())
    .collect();
    assert!(failed.is_empty(), "failed: {failed:?}");
}

/// After the vocabulary as a list without a full stop, Groq's Whisper dropped one of two sentences
/// that begin alike.
#[test]
#[ignore = "sends audio to Groq with the stored API key; speaks through macOS's speech synthesis"]
fn whisper_keeps_repeated_sentences_with_the_vocabulary() {
    require_key(ApiProvider::Groq);
    let vocabulary = ["Postgres", "Traefik", "Lowspeak", "Kubernetes", "Grafana"];
    for (model, voice, language, text, repeated) in [
        (
            OnlineModel::GroqWhisperLargeV3Turbo,
            "Anna",
            Language::De,
            "Ich schicke dir den Bericht heute Abend. Ich schicke dir den Bericht heute Abend noch einmal.",
            "bericht heute abend",
        ),
        (
            OnlineModel::GroqWhisperLargeV3,
            "Samantha",
            Language::En,
            "The quarterly numbers look good. The quarterly numbers look really good.",
            "quarterly numbers",
        ),
    ] {
        let transcript = transcribe(
            spoken(voice, text),
            model,
            language,
            PostProcessingMode::Off,
            &vocabulary,
        );
        assert_eq!(
            transcript.to_lowercase().matches(repeated).count(),
            2,
            "{}: {transcript}",
            model.raw_value()
        );
    }
}

/// The macOS spell checker, asked on the test's own thread: the test harness runs no main queue.
struct MacosWords;

impl Lexicon for MacosWords {
    fn is_word(&self, word: &str, language: Option<&str>) -> bool {
        spelling::known_to_macos(word, language)
    }
}
