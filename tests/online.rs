//! Calls the paid APIs with the keys stored by the app: `cargo test --release --test online -- --ignored`.

use std::path::Path;

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
        samples,
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
    for (provider, model) in [
        (ApiProvider::OpenAi, OnlineModel::GptTranscribe),
        (ApiProvider::Groq, OnlineModel::GroqWhisperLargeV3),
    ] {
        require_key(provider);
        let text = transcribe_fixture(model, Language::De, PostProcessingMode::Off, &vocabulary);
        assert!(
            text.to_lowercase().contains("test"),
            "unexpected text from {}: {text}",
            model.raw_value()
        );
    }
}
