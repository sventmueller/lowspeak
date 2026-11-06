//! Downloads from Hugging Face model repositories, checked against the object ids that git and
//! Git LFS keep for every file.

use std::fmt;
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::{Context as _, Result, bail, ensure};
use serde::Deserialize;
use sha1::Sha1;
use sha2::{Digest as _, Sha256};

use crate::util::http;

const HUGGING_FACE: &str = "https://huggingface.co";
/// A transfer that delivers nothing for this long has stalled; the wait also bounds how late a
/// cancel takes effect.
const READ_TIMEOUT: Duration = Duration::from_secs(30);
const LISTING_TIMEOUT: Duration = Duration::from_secs(60);
const BUFFER_BYTES: usize = 256 * 1024;

/// A repository at a fixed commit, so a push to its `main` branch cannot change what gets
/// installed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Repo {
    pub name: &'static str,
    pub revision: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Checksum {
    /// The SHA-256 of the content, which Git LFS uses as the object id.
    Sha256([u8; 32]),
    /// The SHA-1 of the git blob, the object id of a file stored in git itself.
    GitBlob([u8; 20]),
}

impl Checksum {
    /// Lowercase hex, as Hugging Face lists object ids.
    pub const fn parse_hex<const N: usize>(hex: &str) -> Option<[u8; N]> {
        const fn nibble(digit: u8) -> Option<u8> {
            match digit {
                b'0'..=b'9' => Some(digit - b'0'),
                b'a'..=b'f' => Some(digit - b'a' + 10),
                _ => None,
            }
        }
        let hex = hex.as_bytes();
        if hex.len() != 2 * N {
            return None;
        }
        let mut digest = [0; N];
        let mut i = 0;
        while i < N {
            let (Some(high), Some(low)) = (nibble(hex[2 * i]), nibble(hex[2 * i + 1])) else {
                return None;
            };
            digest[i] = (high << 4) | low;
            i += 1;
        }
        Some(digest)
    }
}

impl fmt::Debug for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (kind, digest): (&str, &[u8]) = match self {
            Self::Sha256(digest) => ("sha256", digest),
            Self::GitBlob(digest) => ("git blob", digest),
        };
        write!(f, "{kind} ")?;
        digest.iter().try_for_each(|byte| write!(f, "{byte:02x}"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemoteFile {
    pub path: String,
    pub size: u64,
    pub checksum: Checksum,
}

#[derive(Debug)]
pub struct Cancelled;

impl fmt::Display for Cancelled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("download cancelled")
    }
}

impl std::error::Error for Cancelled {}

/// Where repositories are downloaded from.
#[derive(Clone, Debug)]
pub struct Hub {
    base_url: String,
    read_timeout: Duration,
}

impl Hub {
    pub fn hugging_face() -> Self {
        Self::new(HUGGING_FACE, READ_TIMEOUT)
    }

    /// A server with Hugging Face's URL layout, such as a local one in tests.
    pub fn new(base_url: impl Into<String>, read_timeout: Duration) -> Self {
        Self {
            base_url: base_url.into(),
            read_timeout,
        }
    }

    pub fn resolve_url(&self, repo: Repo, path: &str) -> String {
        format!(
            "{}/{}/resolve/{}/{path}",
            self.base_url, repo.name, repo.revision
        )
    }

    /// All files of a repository, following the API's pagination.
    pub fn list_files(&self, repo: Repo) -> Result<Vec<RemoteFile>> {
        let agent = http::agent(Some(LISTING_TIMEOUT), self.read_timeout);
        let mut url = Some(format!(
            "{}/api/models/{}/tree/{}?recursive=true",
            self.base_url, repo.name, repo.revision
        ));
        let mut files = Vec::new();
        let mut visited = std::collections::HashSet::new();
        while let Some(page) = url.take() {
            if !visited.insert(page.clone()) {
                break;
            }
            let mut response = agent
                .get(&page)
                .call()
                .with_context(|| format!("listing {}", repo.name))?;
            let status = response.status().as_u16();
            if status != 200 {
                bail!("listing {} failed with HTTP {status}", repo.name);
            }
            url = response
                .headers()
                .get("link")
                .and_then(|link| link.to_str().ok())
                .and_then(next_page);
            let body = response.body_mut().read_to_vec()?;
            files.extend(
                parse_listing(&body)
                    .with_context(|| format!("parsing the file list of {}", repo.name))?,
            );
        }
        Ok(files)
    }

    /// Streams `file` into `destination` via a `.partial` file, which is removed unless the size
    /// and checksum match; `on_bytes` gets the bytes written so far.
    pub fn download(
        &self,
        repo: Repo,
        file: &RemoteFile,
        destination: &Path,
        cancel: &AtomicBool,
        on_bytes: &mut dyn FnMut(u64),
    ) -> Result<()> {
        let url = self.resolve_url(repo, &file.path);
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        let response = http::agent(None, self.read_timeout)
            .get(&url)
            .call()
            .with_context(|| format!("downloading {url}"))?;
        let status = response.status().as_u16();
        if status != 200 {
            bail!("downloading {url} failed with HTTP {status}");
        }
        let partial = partial_path(destination);
        let result = (|| {
            let mut reader = response.into_body().into_reader();
            let mut output = std::fs::File::create(&partial)
                .with_context(|| format!("creating {}", partial.display()))?;
            let mut hasher = Hasher::new(file);
            let mut buffer = vec![0u8; BUFFER_BYTES];
            let mut written = 0u64;
            loop {
                if cancel.load(Ordering::Relaxed) {
                    return Err(Cancelled.into());
                }
                let count = match reader.read(&mut buffer) {
                    Ok(count) => count,
                    Err(_) if cancel.load(Ordering::Relaxed) => return Err(Cancelled.into()),
                    Err(error) => {
                        return Err(error).with_context(|| format!("reading {url}"));
                    }
                };
                if count == 0 {
                    break;
                }
                written += count as u64;
                ensure!(
                    written <= file.size,
                    "{} is larger than the expected {} bytes",
                    file.path,
                    file.size
                );
                hasher.update(&buffer[..count]);
                output.write_all(&buffer[..count])?;
                on_bytes(written);
            }
            ensure!(
                written == file.size,
                "{} ended after {written} of {} bytes",
                file.path,
                file.size
            );
            let actual = hasher.finish();
            ensure!(
                actual == file.checksum,
                "{} does not match its checksum: expected {:?}, got {actual:?}",
                file.path,
                file.checksum
            );
            output.sync_all()?;
            std::fs::rename(&partial, destination)
                .with_context(|| format!("moving {} into place", destination.display()))
        })();
        if result.is_err() {
            let _ = std::fs::remove_file(&partial);
        }
        result
    }
}

fn partial_path(destination: &Path) -> PathBuf {
    destination.with_extension(match destination.extension() {
        Some(extension) => format!("{}.partial", extension.to_string_lossy()),
        None => "partial".into(),
    })
}

/// `rel="next"` target of an HTTP `Link` header.
fn next_page(link: &str) -> Option<String> {
    link.split(',').find_map(|part| {
        let (url, params) = part.split_once(';')?;
        params.contains("rel=\"next\"").then(|| {
            url.trim()
                .trim_start_matches('<')
                .trim_end_matches('>')
                .to_owned()
        })
    })
}

#[derive(Deserialize)]
struct Entry {
    #[serde(rename = "type")]
    kind: String,
    path: String,
    #[serde(default)]
    size: u64,
    oid: Option<String>,
    lfs: Option<LfsPointer>,
}

#[derive(Deserialize)]
struct LfsPointer {
    oid: String,
}

/// The files of one page of the tree API; a file without a usable object id is an error, so
/// nothing gets installed unchecked.
fn parse_listing(json: &[u8]) -> Result<Vec<RemoteFile>> {
    let entries: Vec<Entry> = serde_json::from_slice(json)?;
    entries
        .into_iter()
        .filter(|entry| entry.kind == "file")
        .map(|entry| {
            let checksum = match (&entry.lfs, &entry.oid) {
                (Some(lfs), _) => Checksum::parse_hex(&lfs.oid).map(Checksum::Sha256),
                (None, Some(oid)) => Checksum::parse_hex(oid).map(Checksum::GitBlob),
                (None, None) => None,
            }
            .with_context(|| format!("no valid object id for {}", entry.path))?;
            Ok(RemoteFile {
                path: entry.path,
                size: entry.size,
                checksum,
            })
        })
        .collect()
}

enum Hasher {
    Sha256(Sha256),
    GitBlob(Sha1),
}

impl Hasher {
    fn new(file: &RemoteFile) -> Self {
        match file.checksum {
            Checksum::Sha256(_) => Self::Sha256(Sha256::new()),
            Checksum::GitBlob(_) => {
                Self::GitBlob(Sha1::new_with_prefix(format!("blob {}\0", file.size)))
            }
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        match self {
            Self::Sha256(hasher) => hasher.update(bytes),
            Self::GitBlob(hasher) => hasher.update(bytes),
        }
    }

    fn finish(self) -> Checksum {
        match self {
            Self::Sha256(hasher) => {
                let mut digest = [0; 32];
                digest.copy_from_slice(&hasher.finalize());
                Checksum::Sha256(digest)
            }
            Self::GitBlob(hasher) => {
                let mut digest = [0; 20];
                digest.copy_from_slice(&hasher.finalize());
                Checksum::GitBlob(digest)
            }
        }
    }
}

#[cfg(test)]
pub(crate) fn checksums_of(content: &[u8]) -> (Checksum, Checksum) {
    let file = |checksum| RemoteFile {
        path: String::new(),
        size: content.len() as u64,
        checksum,
    };
    let hash = |file: RemoteFile| {
        let mut hasher = Hasher::new(&file);
        hasher.update(content);
        hasher.finish()
    };
    (
        hash(file(Checksum::Sha256([0; 32]))),
        hash(file(Checksum::GitBlob([0; 20]))),
    )
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use crate::transcription::test_server::{self, Reply};

    const REPO: Repo = Repo {
        name: "owner/model",
        revision: "0123456789abcdef0123456789abcdef01234567",
    };

    #[test]
    fn next_page_is_read_from_link_header() {
        let link = r#"<https://huggingface.co/api/models/x/tree/main?cursor=abc>; rel="next""#;
        assert_eq!(
            next_page(link).as_deref(),
            Some("https://huggingface.co/api/models/x/tree/main?cursor=abc")
        );
        assert_eq!(next_page(r#"<https://a>; rel="prev""#), None);
    }

    #[test]
    fn urls_name_the_pinned_commit() {
        assert_eq!(
            Hub::hugging_face().resolve_url(REPO, "a/b.bin"),
            "https://huggingface.co/owner/model/resolve/0123456789abcdef0123456789abcdef01234567/a/b.bin"
        );
    }

    #[test]
    fn checksums_follow_git_and_git_lfs() {
        let (sha256, git_blob) = checksums_of(b"{}\n");
        assert_eq!(
            git_blob,
            Checksum::GitBlob(
                Checksum::parse_hex("0967ef424bce6791893e9a57bb952f80fd536e93").unwrap()
            )
        );
        assert_eq!(
            checksums_of(b"").0,
            Checksum::Sha256(
                Checksum::parse_hex(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                )
                .unwrap()
            )
        );
        assert_ne!(sha256, checksums_of(b"{}").0);
        assert_eq!(Checksum::parse_hex::<2>("abcd"), Some([0xab, 0xcd]));
        assert_eq!(Checksum::parse_hex::<2>("ABCD"), None);
        assert_eq!(Checksum::parse_hex::<2>("abc"), None);
    }

    #[test]
    fn the_listing_yields_lfs_and_git_checksums_and_skips_directories() {
        let listing = br#"[
            {"type":"directory","oid":"5a0e3f8bd6a9b38ef9b6e2e0a4f52f5c3bd6e0e1","size":0,"path":"Encoder.mlmodelc"},
            {"type":"file","oid":"e016ed32c20e1563399f15adad451a442644b3d7","size":485,
             "lfs":{"oid":"4def7aa848599ad0e17a8b9a982edcdbf33cf92e1f4b798de32e2ca0bc74b030","size":485,"pointerSize":132},
             "xetHash":"0000","path":"Encoder.mlmodelc/coremldata.bin"},
            {"type":"file","oid":"0967ef424bce6791893e9a57bb952f80fd536e93","size":3,"path":"config.json"}
        ]"#;
        let files = parse_listing(listing).unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].path, "Encoder.mlmodelc/coremldata.bin");
        assert_eq!(files[0].size, 485);
        assert!(matches!(files[0].checksum, Checksum::Sha256(digest) if digest[0] == 0x4d));
        assert_eq!(files[1].checksum, checksums_of(b"{}\n").1);
        assert!(parse_listing(br#"[{"type":"file","size":3,"path":"x.json"}]"#).is_err());
        assert!(
            parse_listing(br#"[{"type":"file","oid":"zz","size":3,"path":"x.json"}]"#).is_err()
        );
    }

    struct Scratch(PathBuf);

    impl Scratch {
        fn new(name: &str) -> Self {
            let dir =
                std::env::temp_dir().join(format!("lowspeak-hf-{name}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            Self(dir)
        }

        fn destination(&self) -> PathBuf {
            self.0.join("models").join("file.bin")
        }

        fn is_empty(&self) -> bool {
            std::fs::read_dir(self.0.join("models"))
                .map(|mut entries| entries.next().is_none())
                .unwrap_or(true)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn remote(content: &[u8]) -> RemoteFile {
        RemoteFile {
            path: "dir/file.bin".into(),
            size: content.len() as u64,
            checksum: checksums_of(content).0,
        }
    }

    fn fetch(hub: &Hub, file: &RemoteFile, scratch: &Scratch, cancel: &AtomicBool) -> Result<()> {
        hub.download(REPO, file, &scratch.destination(), cancel, &mut |_| {})
    }

    #[test]
    fn a_verified_download_is_moved_into_place() {
        let content = b"weights".repeat(100_000);
        let served = content.clone();
        let server = test_server::serve(move |_| Reply::Full(200, served.clone()));
        let scratch = Scratch::new("ok");
        let mut progress = Vec::new();
        Hub::new(&server.base_url, Duration::from_secs(5))
            .download(
                REPO,
                &remote(&content),
                &scratch.destination(),
                &AtomicBool::new(false),
                &mut |bytes| progress.push(bytes),
            )
            .unwrap();
        assert_eq!(std::fs::read(scratch.destination()).unwrap(), content);
        assert_eq!(progress.last().copied(), Some(content.len() as u64));
        assert!(progress.is_sorted());
        assert_eq!(
            server.requests(),
            [format!(
                "/owner/model/resolve/{}/dir/file.bin",
                REPO.revision
            )]
        );
    }

    #[test]
    fn a_wrong_checksum_or_size_leaves_nothing_behind() {
        let server = test_server::serve(|_| Reply::Full(200, b"tampered".to_vec()));
        let hub = Hub::new(&server.base_url, Duration::from_secs(5));
        let scratch = Scratch::new("checksum");
        let cancel = AtomicBool::new(false);
        let error = fetch(&hub, &remote(b"original"), &scratch, &cancel).unwrap_err();
        assert!(format!("{error:#}").contains("checksum"), "{error:#}");
        assert!(scratch.is_empty());
        let error = fetch(&hub, &remote(b"longer original"), &scratch, &cancel).unwrap_err();
        assert!(format!("{error:#}").contains("ended after 8"), "{error:#}");
        let error = fetch(&hub, &remote(b"short"), &scratch, &cancel).unwrap_err();
        assert!(format!("{error:#}").contains("larger than"), "{error:#}");
        assert!(scratch.is_empty());
    }

    #[test]
    fn an_error_status_fails_without_creating_files() {
        let server = test_server::serve(|_| Reply::Full(404, b"Entry not found".to_vec()));
        let scratch = Scratch::new("status");
        let error = fetch(
            &Hub::new(&server.base_url, Duration::from_secs(5)),
            &remote(b"x"),
            &scratch,
            &AtomicBool::new(false),
        )
        .unwrap_err();
        assert!(error.to_string().contains("HTTP 404"), "{error:#}");
        assert!(scratch.is_empty());
    }

    #[test]
    fn a_stalled_transfer_times_out_and_removes_the_partial_file() {
        let content = vec![7u8; 1_000_000];
        let server = test_server::serve(|_| Reply::Stall {
            length: 1_000_000,
            sent: vec![7u8; 1_000],
        });
        let scratch = Scratch::new("stall");
        let started = Instant::now();
        let error = fetch(
            &Hub::new(&server.base_url, Duration::from_millis(300)),
            &remote(&content),
            &scratch,
            &AtomicBool::new(false),
        )
        .unwrap_err();
        assert!(started.elapsed() < Duration::from_secs(5), "{error:#}");
        assert!(
            format!("{error:#}").to_lowercase().contains("timeout"),
            "{error:#}"
        );
        assert!(error.downcast_ref::<Cancelled>().is_none());
        assert!(scratch.is_empty());
    }

    #[test]
    fn cancelling_stops_a_running_transfer_and_removes_the_partial_file() {
        let content = vec![1u8; 1_000_000];
        let served = content.clone();
        let server = test_server::serve(move |_| Reply::Trickle {
            body: served.clone(),
            chunk: 1_000,
            pause: Duration::from_millis(20),
        });
        let scratch = Scratch::new("cancel");
        let cancel = AtomicBool::new(false);
        let started = Instant::now();
        let error = Hub::new(&server.base_url, Duration::from_secs(5))
            .download(
                REPO,
                &remote(&content),
                &scratch.destination(),
                &cancel,
                &mut |_| cancel.store(true, Ordering::Relaxed),
            )
            .unwrap_err();
        assert!(error.downcast_ref::<Cancelled>().is_some(), "{error:#}");
        assert!(started.elapsed() < Duration::from_secs(5));
        assert!(scratch.is_empty());
    }

    #[test]
    fn a_cancel_during_a_stall_is_reported_as_cancelled() {
        let server = test_server::serve(|_| Reply::Stall {
            length: 10,
            sent: vec![0; 2],
        });
        let scratch = Scratch::new("cancel-stall");
        let hub = Hub::new(&server.base_url, Duration::from_millis(300));
        let cancel = AtomicBool::new(false);
        let error = std::thread::scope(|scope| {
            scope.spawn(|| {
                std::thread::sleep(Duration::from_millis(100));
                cancel.store(true, Ordering::Relaxed);
            });
            fetch(&hub, &remote(&[0; 10]), &scratch, &cancel).unwrap_err()
        });
        assert!(error.downcast_ref::<Cancelled>().is_some(), "{error:#}");
        assert!(scratch.is_empty());
    }

    #[test]
    fn listings_are_read_at_the_pinned_commit() {
        let server = test_server::serve(|path| {
            assert!(path.contains(REPO.revision), "{path}");
            Reply::Full(
                200,
                br#"[{"type":"file","oid":"0967ef424bce6791893e9a57bb952f80fd536e93","size":3,"path":"config.json"}]"#
                    .to_vec(),
            )
        });
        let files = Hub::new(&server.base_url, Duration::from_secs(5))
            .list_files(REPO)
            .unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            server.requests(),
            [format!(
                "/api/models/owner/model/tree/{}?recursive=true",
                REPO.revision
            )]
        );
    }
}
