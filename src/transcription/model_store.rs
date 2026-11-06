//! Where local models live, and downloading and deleting them.

use std::collections::HashSet;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::AtomicBool;

use anyhow::{Context as _, Result, ensure};

use super::catalog::{self, Engine, WHISPER_REPO};
use super::hf::{Hub, RemoteFile};
use super::parakeet::{self, Version};
use crate::models::LocalModel;
use crate::util::{filelog, paths};

/// Which local models are ready to load.
pub trait ModelStore {
    fn installed(&self) -> HashSet<LocalModel>;
}

pub struct Disk;

impl ModelStore for Disk {
    fn installed(&self) -> HashSet<LocalModel> {
        installed_models()
    }
}

/// The model directories below one root, filled from one hub.
#[derive(Clone, Debug)]
pub struct Library {
    whisper_dir: PathBuf,
    parakeet_root: PathBuf,
    hub: Hub,
}

impl ModelStore for Library {
    fn installed(&self) -> HashSet<LocalModel> {
        Library::installed(self)
    }
}

impl Library {
    /// The directories in `~/Library/Application Support`, filled from Hugging Face.
    pub fn system() -> io::Result<Self> {
        Ok(Self::under(
            &paths::try_application_support()?,
            Hub::hugging_face(),
        ))
    }

    /// The layout of `~/Library/Application Support` below `root`: Whisper in the app's own
    /// directory, Parakeet in FluidAudio's cache, so models FluidAudio downloaded are reused.
    pub fn under(root: &Path, hub: Hub) -> Self {
        Self {
            whisper_dir: root.join("Lowspeak").join("Models").join("whisper"),
            parakeet_root: root.join("FluidAudio").join("Models"),
            hub,
        }
    }

    /// The model file (Whisper) or directory (Parakeet).
    pub fn location(&self, model: LocalModel) -> PathBuf {
        match catalog::engine(model) {
            Engine::Whisper(file) => self.whisper_dir.join(file.name),
            Engine::Parakeet(version) => self.parakeet_root.join(version.folder_name()),
        }
    }

    /// Checks presence only, so copies from downloads made before checksums were verified still
    /// count and are not fetched again.
    pub fn is_installed(&self, model: LocalModel) -> bool {
        let location = self.location(model);
        match catalog::engine(model) {
            Engine::Whisper(_) => location.is_file(),
            Engine::Parakeet(version) => parakeet::is_installed(&location, version),
        }
    }

    pub fn installed(&self) -> HashSet<LocalModel> {
        LocalModel::ALL
            .into_iter()
            .filter(|&model| self.is_installed(model))
            .collect()
    }

    /// Downloads a model, reporting progress as a fraction from 0 to 1.
    pub fn download(
        &self,
        model: LocalModel,
        cancel: &AtomicBool,
        progress: &mut dyn FnMut(f64),
    ) -> Result<()> {
        flog!(filelog::APP, "Starting download: {}", model.raw_value());
        let location = self.location(model);
        match catalog::engine(model) {
            Engine::Whisper(file) => {
                self.hub.download(
                    WHISPER_REPO,
                    &file.remote(),
                    &location,
                    cancel,
                    &mut |bytes| progress(fraction(bytes, file.size)),
                )?;
            }
            Engine::Parakeet(version) => {
                self.download_parakeet(version, &location, cancel, progress)?;
            }
        }
        progress(1.0);
        flog!(filelog::APP, "Downloaded {}", model.raw_value());
        Ok(())
    }

    fn download_parakeet(
        &self,
        version: Version,
        dir: &Path,
        cancel: &AtomicBool,
        progress: &mut dyn FnMut(f64),
    ) -> Result<()> {
        let repo = catalog::parakeet_repo(version);
        let files = parakeet_files(version, self.hub.list_files(repo)?);
        let total = files.iter().map(|file| file.size).sum::<u64>().max(1);
        let mut done = 0u64;
        for file in &files {
            self.hub.download(
                repo,
                file,
                &dir.join(contained(&file.path)?),
                cancel,
                &mut |bytes| progress(fraction(done + bytes, total)),
            )?;
            done += file.size;
        }
        ensure!(
            parakeet::is_installed(dir, version),
            "download of {} is incomplete",
            repo.name
        );
        Ok(())
    }

    pub fn delete(&self, model: LocalModel) -> Result<()> {
        let path = self.location(model);
        if path.is_dir() {
            std::fs::remove_dir_all(&path)
                .with_context(|| format!("deleting {}", path.display()))?;
        } else if path.symlink_metadata().is_ok() {
            std::fs::remove_file(&path).with_context(|| format!("deleting {}", path.display()))?;
        }
        flog!(filelog::APP, "Deleted model {}", model.raw_value());
        Ok(())
    }
}

/// The library of this Mac for reading, which needs no home directory to answer "not installed".
fn this_mac() -> Library {
    Library::under(&paths::application_support(), Hub::hugging_face())
}

/// The model file (Whisper) or directory (Parakeet) on this Mac.
pub fn location(model: LocalModel) -> PathBuf {
    this_mac().location(model)
}

pub fn is_installed(model: LocalModel) -> bool {
    this_mac().is_installed(model)
}

pub fn installed_models() -> HashSet<LocalModel> {
    this_mac().installed()
}

/// Downloads a model to this Mac; see [`Library::download`].
pub fn download(
    model: LocalModel,
    cancel: &AtomicBool,
    progress: &mut dyn FnMut(f64),
) -> Result<()> {
    Library::system()?.download(model, cancel, progress)
}

pub fn delete(model: LocalModel) -> Result<()> {
    Library::system()?.delete(model)
}

/// The files of the listing that the model needs. The vocabulary comes last: `is_installed` looks
/// for it, so an interrupted download never looks complete.
fn parakeet_files(version: Version, listing: Vec<RemoteFile>) -> Vec<RemoteFile> {
    let bundles = version.required_bundles();
    let mut files: Vec<RemoteFile> = listing
        .into_iter()
        .filter(|file| {
            bundles
                .iter()
                .any(|bundle| file.path.starts_with(&format!("{bundle}/")))
                || (!file.path.contains('/') && file.path.ends_with(".json"))
        })
        .collect();
    files.sort_by_key(|file| file.path == Version::VOCABULARY_FILE);
    files
}

/// A path from a repository listing, accepted only if it stays inside the model directory.
fn contained(path: &str) -> Result<&Path> {
    let relative = Path::new(path);
    ensure!(
        !path.is_empty()
            && relative
                .components()
                .all(|component| matches!(component, Component::Normal(_))),
        "refusing file path {path:?} from the model repository"
    );
    Ok(relative)
}

fn fraction(done: u64, total: u64) -> f64 {
    (done as f64 / total as f64).min(1.0)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::time::Duration;

    use super::*;
    use crate::transcription::hf::{self, Checksum};
    use crate::transcription::test_server::{self, Reply, Server};

    #[test]
    fn repository_paths_must_stay_inside_the_model_directory() {
        for path in [
            "Encoder.mlmodelc/coremldata.bin",
            "parakeet_vocab.json",
            "Encoder.mlmodelc/weights/weight.bin",
        ] {
            assert!(contained(path).is_ok(), "{path}");
        }
        for path in [
            "Encoder.mlmodelc/../../../Library/LaunchAgents/x.plist",
            "/etc/passwd",
            "./config.json",
            "",
        ] {
            assert!(contained(path).is_err(), "{path}");
        }
    }

    #[test]
    fn this_mac_uses_the_directories_parakeet_loads_from() {
        for version in [Version::V2, Version::Ultra] {
            let model = match version {
                Version::V2 => LocalModel::ParakeetV2,
                Version::Ultra => LocalModel::ParakeetUltra,
            };
            assert_eq!(location(model), parakeet::model_dir(version));
        }
        assert_eq!(
            location(LocalModel::Tiny),
            paths::application_support().join("Lowspeak/Models/whisper/ggml-tiny.bin")
        );
    }

    struct Scratch(PathBuf);

    impl Scratch {
        fn new(name: &str) -> Self {
            let dir =
                std::env::temp_dir().join(format!("lowspeak-models-{name}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            Self(dir)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn hex(checksum: Checksum) -> String {
        format!("{checksum:?}")
            .rsplit(' ')
            .next()
            .unwrap_or_default()
            .to_owned()
    }

    /// A Parakeet v2 repository with the four bundles, the JSON files and parts the model does not
    /// need; `tamper` changes the served content of one path.
    fn parakeet_server(extra: &[&str], tamper: Option<&'static str>) -> Server {
        let mut paths: Vec<String> = Version::V2
            .required_bundles()
            .iter()
            .flat_map(|bundle| {
                [
                    format!("{bundle}/coremldata.bin"),
                    format!("{bundle}/weights/weight.bin"),
                ]
            })
            .collect();
        paths.extend(
            [
                "config.json",
                "parakeet_vocab.json",
                "Melspectogram.mlmodelc/coremldata.bin",
                "README.md",
            ]
            .map(str::to_owned),
        );
        paths.extend(extra.iter().map(|path| (*path).to_owned()));
        let files: BTreeMap<String, Vec<u8>> = paths
            .into_iter()
            .map(|path| {
                let content = format!("content of {path}").into_bytes();
                (path, content)
            })
            .collect();
        let listing: Vec<serde_json::Value> = files
            .iter()
            .map(|(path, content)| {
                let (sha256, git_blob) = hf::checksums_of(content);
                let mut entry = serde_json::json!({
                    "type": "file",
                    "path": path,
                    "size": content.len(),
                    "oid": hex(git_blob),
                });
                if path.ends_with(".bin") {
                    entry["lfs"] = serde_json::json!({"oid": hex(sha256), "size": content.len()});
                }
                entry
            })
            .collect();
        let listing = serde_json::to_vec(&listing).unwrap();
        let repo = catalog::parakeet_repo(Version::V2);
        let resolve = format!("/{}/resolve/{}/", repo.name, repo.revision);
        test_server::serve(move |target| {
            if target.starts_with("/api/models/") {
                return Reply::Full(200, listing.clone());
            }
            match target
                .strip_prefix(&resolve)
                .and_then(|path| files.get(path))
            {
                Some(_) if tamper.is_some_and(|path| target.ends_with(path)) => {
                    Reply::Full(200, b"tampered content".to_vec())
                }
                Some(content) => Reply::Full(200, content.clone()),
                None => Reply::Full(404, Vec::new()),
            }
        })
    }

    fn library(scratch: &Scratch, server: &Server) -> Library {
        Library::under(
            &scratch.0,
            Hub::new(&server.base_url, Duration::from_secs(5)),
        )
    }

    #[test]
    fn a_parakeet_download_fetches_the_needed_files_verified_and_the_vocabulary_last() {
        let server = parakeet_server(&[], None);
        let scratch = Scratch::new("parakeet");
        let library = library(&scratch, &server);
        assert!(library.installed().is_empty());
        let mut progress = Vec::new();
        library
            .download(
                LocalModel::ParakeetV2,
                &AtomicBool::new(false),
                &mut |fraction| progress.push(fraction),
            )
            .unwrap();

        assert_eq!(library.installed(), HashSet::from([LocalModel::ParakeetV2]));
        assert!(progress.is_sorted() && progress.last() == Some(&1.0));
        let requests = server.requests();
        assert!(requests[0].starts_with("/api/models/"));
        assert!(requests.last().unwrap().ends_with("/parakeet_vocab.json"));
        assert_eq!(requests.len(), 1 + 4 * 2 + 2, "{requests:#?}");
        let dir = library.location(LocalModel::ParakeetV2);
        assert!(dir.join("config.json").is_file());
        assert!(!dir.join("README.md").exists());
        assert!(!dir.join("Melspectogram.mlmodelc").exists());

        library.delete(LocalModel::ParakeetV2).unwrap();
        assert!(!dir.exists());
        assert!(library.installed().is_empty());
    }

    #[test]
    fn a_tampered_file_stops_the_download_before_the_model_counts_as_installed() {
        let server = parakeet_server(&[], Some("Encoder.mlmodelc/weights/weight.bin"));
        let scratch = Scratch::new("tampered");
        let library = library(&scratch, &server);
        let error = library
            .download(LocalModel::ParakeetV2, &AtomicBool::new(false), &mut |_| {})
            .unwrap_err();
        assert!(format!("{error:#}").contains("Encoder.mlmodelc/weights/weight.bin"));
        assert!(!library.is_installed(LocalModel::ParakeetV2));
        let weights = library
            .location(LocalModel::ParakeetV2)
            .join("Encoder.mlmodelc/weights");
        assert!(!weights.join("weight.bin").exists());
        assert!(!weights.join("weight.bin.partial").exists());
    }

    #[test]
    fn a_listing_with_a_path_outside_the_model_directory_is_refused() {
        let server = parakeet_server(&["Encoder.mlmodelc/../../escape.json"], None);
        let scratch = Scratch::new("escape");
        let library = library(&scratch, &server);
        let error = library
            .download(LocalModel::ParakeetV2, &AtomicBool::new(false), &mut |_| {})
            .unwrap_err();
        assert!(error.to_string().contains("refusing"), "{error:#}");
        assert!(!scratch.0.join("FluidAudio/escape.json").exists());
        assert!(!library.is_installed(LocalModel::ParakeetV2));
    }

    #[test]
    fn a_whisper_file_is_fetched_from_the_pinned_commit_and_rejected_unless_it_matches() {
        let server = test_server::serve(|_| Reply::Full(200, b"not a ggml file".to_vec()));
        let scratch = Scratch::new("whisper");
        let library = library(&scratch, &server);
        assert!(
            library
                .download(LocalModel::Tiny, &AtomicBool::new(false), &mut |_| {})
                .is_err()
        );
        assert!(!library.is_installed(LocalModel::Tiny));
        assert_eq!(
            server.requests(),
            [format!(
                "/ggerganov/whisper.cpp/resolve/{}/ggml-tiny.bin",
                WHISPER_REPO.revision
            )]
        );
    }

    #[test]
    fn deleting_a_whisper_model_removes_only_its_file() {
        let scratch = Scratch::new("delete");
        let library = Library::under(&scratch.0, Hub::hugging_face());
        let tiny = library.location(LocalModel::Tiny);
        let base = library.location(LocalModel::Base);
        std::fs::create_dir_all(tiny.parent().unwrap()).unwrap();
        std::fs::write(&tiny, b"tiny").unwrap();
        std::fs::write(&base, b"base").unwrap();
        assert_eq!(
            library.installed(),
            HashSet::from([LocalModel::Tiny, LocalModel::Base])
        );
        library.delete(LocalModel::Tiny).unwrap();
        library.delete(LocalModel::Tiny).unwrap();
        assert_eq!(library.installed(), HashSet::from([LocalModel::Base]));
    }
}
