//! The app's one version, `[workspace.package] version` in Cargo.toml, and raising it.

use std::cmp::Ordering;
use std::ffi::c_int;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use anyhow::{Result, bail, ensure};

use crate::*;

const MANIFEST: &str = "Cargo.toml";
const LOCK: &str = "Cargo.lock";
const SECTION: &str = "[workspace.package]";
const KEY: &str = "version = ";

pub(crate) fn is_version(version: &str) -> bool {
    numbers(version).is_some()
}

fn numbers(version: &str) -> Option<[u64; 3]> {
    let parts: Vec<&str> = version.split('.').collect();
    let valid = parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()));
    if !valid {
        return None;
    }
    let mut numbers = [0; 3];
    for (number, part) in numbers.iter_mut().zip(parts) {
        *number = part.parse().ok()?;
    }
    Some(numbers)
}

fn compare(left: &str, right: &str) -> Result<Ordering> {
    let left = numbers(left).with_context(|| format!("{left:?} is not major.minor.build"))?;
    let right = numbers(right).with_context(|| format!("{right:?} is not major.minor.build"))?;
    Ok(left.cmp(&right))
}

pub(crate) fn following(version: &str) -> Result<String> {
    let (head, build) = version.rsplit_once('.').context("no build number")?;
    let build: u64 = build
        .parse()
        .with_context(|| format!("build number {build}"))?;
    Ok(format!("{head}.{}", build + 1))
}

fn section<'a>(manifest: &'a str, header: &str) -> impl Iterator<Item = &'a str> {
    manifest
        .lines()
        .skip_while(move |line| line.trim() != header)
        .skip(1)
        .take_while(|line| !line.starts_with('['))
}

fn is_version_line(line: &str) -> bool {
    line.starts_with(KEY)
}

pub(crate) fn workspace_version(manifest: &str) -> Result<String> {
    let line = section(manifest, SECTION)
        .find(|line| is_version_line(line))
        .with_context(|| format!("{MANIFEST} has no version in {SECTION}"))?;
    let version = line
        .split('"')
        .nth(1)
        .with_context(|| format!("{MANIFEST}: {line}"))?;
    ensure!(
        is_version(version),
        "{MANIFEST}: {version:?} is not major.minor.build"
    );
    Ok(version.to_owned())
}

fn rewrite(manifest: &str, version: Option<&str>) -> String {
    let mut header = "";
    let mut rewritten = String::with_capacity(manifest.len());
    for line in manifest.split_inclusive('\n') {
        if line.starts_with('[') {
            header = line.trim();
        }
        if header == SECTION && is_version_line(line) {
            if let Some(version) = version {
                rewritten.push_str(&format!("{KEY}\"{version}\"\n"));
            }
        } else {
            rewritten.push_str(line);
        }
    }
    rewritten
}

/// The lock entries of the workspace's own packages: the version line follows the name, and only registry and git
/// packages have a source.
fn own_entries<'a>(lock: &'a [&'a str], names: &'a [String]) -> impl Iterator<Item = usize> + 'a {
    (1..lock.len().saturating_sub(1)).filter(move |&i| {
        lock[i - 1].trim_end() == "[[package]]"
            && names
                .iter()
                .any(|name| lock[i].trim_end() == format!("name = \"{name}\""))
            && is_version_line(lock[i + 1])
            && !lock
                .get(i + 2)
                .is_some_and(|line| line.starts_with("source = "))
    })
}

fn rewrite_lock(lock: &str, names: &[String], version: Option<&str>) -> String {
    let lines: Vec<&str> = lock.split_inclusive('\n').collect();
    let own: Vec<usize> = own_entries(&lines, names).map(|i| i + 1).collect();
    let mut rewritten = String::with_capacity(lock.len());
    for (i, line) in lines.iter().enumerate() {
        if !own.contains(&i) {
            rewritten.push_str(line);
        } else if let Some(version) = version {
            rewritten.push_str(&format!("{KEY}\"{version}\"\n"));
        }
    }
    rewritten
}

#[cfg(test)]
fn lock_versions(lock: &str, names: &[String]) -> Vec<(String, String)> {
    let lines: Vec<&str> = lock.lines().collect();
    own_entries(&lines, names)
        .map(|i| {
            let name = lines[i].split('"').nth(1).unwrap_or_default().to_owned();
            let version = lines[i + 1]
                .split('"')
                .nth(1)
                .unwrap_or_default()
                .to_owned();
            (name, version)
        })
        .collect()
}

fn member_manifests(root: &Path, manifest: &str) -> Vec<PathBuf> {
    let members = section(manifest, "[workspace]")
        .find(|line| line.starts_with("members = "))
        .unwrap_or_default();
    std::iter::once(root.join(MANIFEST))
        .chain(
            members
                .split('"')
                .skip(1)
                .step_by(2)
                .map(|member| root.join(member).join(MANIFEST)),
        )
        .collect()
}

fn package_name(manifest: &str) -> Option<String> {
    section(manifest, "[package]")
        .find(|line| line.starts_with("name = "))
        .and_then(|line| line.split('"').nth(1))
        .map(str::to_owned)
}

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

fn package_names(root: &Path, manifest: &str) -> Result<Vec<String>> {
    member_manifests(root, manifest)
        .iter()
        .map(|path| {
            package_name(&read(path)?).with_context(|| format!("{}: no name", path.display()))
        })
        .collect()
}

pub(crate) fn version() -> Result<String> {
    workspace_version(&read(&root().join(MANIFEST))?)
}

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

extern "C" fn note_interruption(_: c_int) {
    INTERRUPTED.store(true, AtomicOrdering::SeqCst);
}

unsafe extern "C" {
    fn signal(number: c_int, handler: extern "C" fn(c_int)) -> usize;
}

/// Whether Control-C, a closed terminal or `kill` asked xtask to stop since a bump started.
pub(crate) fn interrupted() -> bool {
    INTERRUPTED.load(AtomicOrdering::SeqCst)
}

/// Keeps xtask alive on SIGHUP, SIGINT and SIGTERM so that a bump is undone: the programs it runs still stop, since
/// a handler, unlike an ignored signal, does not pass on to them, and `run` fails once they did.
fn catch_interruptions() {
    for number in [1, 2, 15] {
        // SAFETY: the handler only stores to an atomic, which is safe in a signal handler.
        unsafe { signal(number, note_interruption) };
    }
}

/// A raised version in Cargo.toml and Cargo.lock: `commit` keeps it as a commit of these two files alone, dropping it
/// without one puts both files back as they were.
pub(crate) struct Bump {
    version: String,
    saved: Vec<(PathBuf, String)>,
    kept: bool,
}

impl Bump {
    /// Raises the build number past the committed version, or takes a higher version set by hand. Cargo.toml and
    /// Cargo.lock may differ from the last commit only in the version, so that the commit carries nothing else.
    pub(crate) fn start() -> Result<Self> {
        let root = root();
        let manifest = read(&root.join(MANIFEST))?;
        let lock = read(&root.join(LOCK))?;
        let committed = |file: &str| {
            output(
                Command::new("git")
                    .current_dir(&root)
                    .args(["show", &format!("HEAD:{file}")]),
            )
        };
        let committed_manifest = committed(MANIFEST)?;
        let committed_lock = committed(LOCK)?;
        let names = package_names(&root, &manifest)?;
        ensure!(
            rewrite(&manifest, None) == rewrite(&committed_manifest, None)
                && rewrite_lock(&lock, &names, None) == rewrite_lock(&committed_lock, &names, None),
            "{MANIFEST} or {LOCK} has changes besides the version; commit them first, the version commit carries the \
             version alone"
        );
        let last = workspace_version(&committed_manifest)?;
        let written = workspace_version(&manifest)?;
        let version = match compare(&written, &last)? {
            Ordering::Greater => written,
            Ordering::Equal => following(&last)?,
            Ordering::Less => bail!("{MANIFEST} says {written}, below the committed {last}"),
        };
        catch_interruptions();
        let bump = Bump {
            saved: vec![(root.join(MANIFEST), manifest), (root.join(LOCK), lock)],
            version,
            kept: false,
        };
        std::fs::write(
            &bump.saved[0].0,
            rewrite(&bump.saved[0].1, Some(&bump.version)),
        )?;
        std::fs::write(
            &bump.saved[1].0,
            rewrite_lock(&bump.saved[1].1, &names, Some(&bump.version)),
        )?;
        run(Command::new(env!("CARGO"))
            .current_dir(&root)
            .args(["metadata", "--locked", "--offline", "--format-version", "1"])
            .stdout(Stdio::null()))
        .with_context(|| format!("{LOCK} does not match {MANIFEST} after the version change"))?;
        println!("Version {}", bump.version);
        Ok(bump)
    }

    pub(crate) fn version(&self) -> &str {
        &self.version
    }

    pub(crate) fn commit(mut self) -> Result<()> {
        run(Command::new("git")
            .current_dir(root())
            .args(["commit", "--quiet", "--only", MANIFEST, LOCK, "-m"])
            .arg(format!("Raise the version to {}", self.version)))?;
        self.kept = true;
        println!("Committed version {}", self.version);
        Ok(())
    }
}

impl Drop for Bump {
    fn drop(&mut self) {
        if self.kept {
            return;
        }
        for (path, text) in &self.saved {
            if let Err(error) = std::fs::write(path, text) {
                eprintln!("restoring {}: {error}", path.display());
            }
        }
        eprintln!(
            "{MANIFEST} and {LOCK} put back, {} is not kept",
            self.version
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST_TEXT: &str = "[package]\nname = \"app\"\nversion.workspace = true\n\n[workspace]\nmembers = \
                                 [\"tool\"]\n\n[workspace.package]\nedition = \"2024\"\nversion = \"1.2.3\"\n\n\
                                 [dependencies]\nfoo = { version = \"1\" }\n";
    const LOCK_TEXT: &str = "[[package]]\nname = \"app\"\nversion = \"1.2.3\"\ndependencies = [\n \"tool\",\n]\n\n\
                             [[package]]\nname = \"tool\"\nversion = \"9.9.9\"\nsource = \
                             \"registry+https://github.com/rust-lang/crates.io-index\"\n\n[[package]]\nname = \
                             \"tool\"\nversion = \"1.2.3\"\n";

    fn names() -> Vec<String> {
        vec!["app".to_owned(), "tool".to_owned()]
    }

    #[test]
    fn versions_have_three_numbers() {
        assert!(is_version("1.0.0") && is_version("12.3.456"));
        assert!(
            !is_version("1.0")
                && !is_version("1.0.x")
                && !is_version("1..1")
                && !is_version("1.0.1.2")
        );
    }

    #[test]
    fn a_release_build_raises_only_the_build_number() {
        assert_eq!(following("1.0.0").unwrap(), "1.0.1");
        assert_eq!(following("2.3.99").unwrap(), "2.3.100");
    }

    #[test]
    fn versions_compare_by_number() {
        assert_eq!(compare("1.0.10", "1.0.9").unwrap(), Ordering::Greater);
        assert_eq!(compare("1.1.0", "1.0.99").unwrap(), Ordering::Greater);
        assert_eq!(compare("1.0.6", "1.0.6").unwrap(), Ordering::Equal);
    }

    #[test]
    fn only_the_workspace_version_changes() {
        assert_eq!(workspace_version(MANIFEST_TEXT).unwrap(), "1.2.3");
        let raised = rewrite(MANIFEST_TEXT, Some("1.2.4"));
        assert_eq!(
            raised,
            MANIFEST_TEXT.replace("version = \"1.2.3\"", "version = \"1.2.4\"")
        );
        assert_eq!(rewrite(&raised, None), rewrite(MANIFEST_TEXT, None));
        assert!(rewrite(MANIFEST_TEXT, None).contains("foo = { version = \"1\" }"));
    }

    #[test]
    fn the_lock_changes_only_for_the_workspace_packages() {
        let raised = rewrite_lock(LOCK_TEXT, &names(), Some("1.2.4"));
        assert_eq!(
            lock_versions(&raised, &names()),
            [
                ("app".into(), "1.2.4".into()),
                ("tool".into(), "1.2.4".into())
            ]
        );
        assert!(raised.contains("name = \"tool\"\nversion = \"9.9.9\"\nsource"));
        assert_eq!(
            rewrite_lock(&raised, &names(), None),
            rewrite_lock(LOCK_TEXT, &names(), None)
        );
    }

    #[test]
    fn the_members_are_found() {
        let root = Path::new("/r");
        assert_eq!(
            member_manifests(root, MANIFEST_TEXT),
            [root.join("Cargo.toml"), root.join("tool/Cargo.toml")]
        );
    }

    /// The version may appear nowhere but in `[workspace.package]`, and every other place must follow it.
    #[test]
    fn the_project_has_one_version() {
        let root = root();
        assert!(
            !root.join("VERSION").exists(),
            "the version lives in Cargo.toml"
        );
        let manifest = read(&root.join(MANIFEST)).unwrap();
        let version = workspace_version(&manifest).unwrap();
        assert_ne!(version, "0.0.0");

        let manifests = member_manifests(&root, &manifest);
        assert!(manifests.len() > 1);
        for path in &manifests {
            let text = read(path).unwrap();
            let package: Vec<&str> = section(&text, "[package]").collect();
            assert!(
                package.contains(&"version.workspace = true")
                    && !package.iter().any(|line| is_version_line(line)),
                "{} must take the workspace version",
                path.display()
            );
        }

        let names = package_names(&root, &manifest).unwrap();
        let locked = lock_versions(&read(&root.join(LOCK)).unwrap(), &names);
        assert_eq!(locked.len(), names.len(), "{LOCK} lists {locked:?}");
        for (name, locked) in locked {
            assert_eq!(locked, version, "{LOCK} has {name} at {locked}");
        }

        assert!(info_plist(&version, false).contains(&format!(
            "<key>CFBundleShortVersionString</key>\n    <string>{version}</string>"
        )));

        let feed = read(&root.join("dist").join(FEED)).unwrap();
        let mut released: Vec<String> = feed
            .split("<sparkle:version>")
            .skip(1)
            .filter_map(|rest| rest.split('<').next())
            .map(str::to_owned)
            .collect();
        assert!(!released.is_empty());
        for entry in std::fs::read_dir(root.join("dist")).unwrap().flatten() {
            let file = entry.file_name().to_string_lossy().into_owned();
            if let Some(notes) = released_version(&file) {
                assert_eq!(
                    notes, released[0],
                    "dist/{file} is not the release in {FEED}"
                );
            }
        }
        released.dedup();
        for release in released {
            assert_ne!(
                compare(&release, &version).unwrap(),
                Ordering::Greater,
                "{FEED} offers {release}, past the version {version} in {MANIFEST}"
            );
        }
    }
}
