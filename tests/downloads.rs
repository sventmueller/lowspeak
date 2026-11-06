//! Network tests against Hugging Face: `cargo test --release --test downloads -- --ignored`.
//! Downloads go to a temporary directory, never into the app's model directories.

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use lowspeak::models::LocalModel;
use lowspeak::transcription::catalog::{self, Engine, WHISPER_REPO};
use lowspeak::transcription::hf::Hub;
use lowspeak::transcription::model_store::Library;
use lowspeak::transcription::parakeet::Version;

struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let dir =
            std::env::temp_dir().join(format!("lowspeak-downloads-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        Self(dir)
    }

    fn library(&self) -> Library {
        Library::under(&self.0, Hub::hugging_face())
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
#[ignore = "needs network access"]
fn parakeet_repositories_contain_the_required_files() {
    for version in [Version::V2, Version::Ultra] {
        let repo = catalog::parakeet_repo(version);
        let files = Hub::hugging_face().list_files(repo).unwrap();
        for bundle in version.required_bundles() {
            for part in ["coremldata.bin", "model.mil", "weights/weight.bin"] {
                let path = format!("{bundle}/{part}");
                assert!(
                    files.iter().any(|file| file.path == path),
                    "{} lacks {path}",
                    repo.name
                );
            }
        }
        assert!(
            files
                .iter()
                .any(|file| file.path == Version::VOCABULARY_FILE),
            "{} lacks the vocabulary",
            repo.name
        );
    }
}

#[test]
#[ignore = "needs network access"]
fn whisper_files_match_the_pinned_listing() {
    let files = Hub::hugging_face().list_files(WHISPER_REPO).unwrap();
    for model in LocalModel::ALL {
        if let Engine::Whisper(file) = catalog::engine(model) {
            assert!(
                files.contains(&file.remote()),
                "{} differs from the listing",
                file.name
            );
        }
    }
}

#[test]
#[ignore = "downloads 75 MB"]
fn downloads_whisper_tiny() {
    let scratch = Scratch::new("tiny");
    let library = scratch.library();
    let mut last = 0.0;
    library
        .download(LocalModel::Tiny, &AtomicBool::new(false), &mut |progress| {
            assert!(progress >= last, "progress went backwards");
            last = progress;
        })
        .unwrap();
    assert_eq!(last, 1.0);
    assert!(library.is_installed(LocalModel::Tiny));
}

#[test]
#[ignore = "downloads 630 MB"]
fn downloads_parakeet_ultra() {
    let scratch = Scratch::new("ultra");
    let library = scratch.library();
    let mut last = 0.0;
    library
        .download(
            LocalModel::ParakeetUltra,
            &AtomicBool::new(false),
            &mut |progress| {
                assert!(progress >= last, "progress went backwards");
                last = progress;
            },
        )
        .unwrap();
    assert!(library.is_installed(LocalModel::ParakeetUltra));
}
