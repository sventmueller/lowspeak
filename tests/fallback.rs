//! A failed online request falls back to the local model: `cargo test --release --test fallback -- --ignored`.
//! A test binary of its own, because it points the whole process to an invalid API key.

use std::path::Path;

use lowspeak::audio::wav;
use lowspeak::config::settings::Settings;
use lowspeak::models::{Language, LocalModel, OnlineModel, PostProcessingMode, TranscriptionMode};
use lowspeak::transcription::Api;
use lowspeak::transcription::error::TranscriptionError;
use lowspeak::transcription::local::LocalTranscriber;
use lowspeak::transcription::model_store::{self, Disk};
use lowspeak::transcription::postprocess::ActiveApp;
use lowspeak::transcription::service::{self, Job, Outcome, Services};

#[test]
#[ignore = "sends one request with an invalid key to Groq; needs Parakeet Ultra installed"]
fn rejected_online_request_is_transcribed_locally() {
    assert!(
        model_store::is_installed(LocalModel::ParakeetUltra),
        "parakeet-ultra is not installed; download it in the app first"
    );
    let dir = std::env::temp_dir().join(format!("lowspeak-fallback-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let key_file = dir.join(".api-key");
    std::fs::write(&key_file, "gsk_invalid_key_for_the_fallback_test\n").unwrap();
    // SAFETY: set before any thread of this test binary reads the environment.
    unsafe { std::env::set_var("LOWSPEAK_API_KEY_FILE", &key_file) };

    let bytes =
        std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/builtin.wav"))
            .unwrap();
    let job = Job {
        samples: wav::decode(&bytes).unwrap().samples,
        settings: Settings {
            mode: TranscriptionMode::Online,
            local_model: LocalModel::ParakeetUltra,
            online_model: OnlineModel::GroqWhisperLargeV3Turbo,
            language: Language::Auto,
            post_processing: PostProcessingMode::Off,
            preprocessing: true,
            start_sound: false,
        },
        active_app: ActiveApp::default(),
        raw_parakeet_first: false,
        vocabulary: Vec::new(),
    };
    let local = LocalTranscriber::spawn();
    let services = Services {
        local: &local,
        models: &Disk,
        api: Api::SYSTEM,
    };
    let outcome = service::run(job, &services);
    let _ = std::fs::remove_dir_all(&dir);
    match outcome.unwrap() {
        Outcome::Text(transcript) => {
            println!("{} ({:?})", transcript.text, transcript.local_fallback);
            assert!(transcript.text.starts_with("Test"), "{}", transcript.text);
            assert!(
                matches!(
                    transcript.local_fallback,
                    Some(TranscriptionError::Api { status: 401, .. })
                ),
                "{:?}",
                transcript.local_fallback
            );
        }
        _ => panic!("expected text"),
    }
}
