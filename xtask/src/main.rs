//! Build tasks in the language of the project: bundling, signing, installing, the notarized disk image and the icon.
//! The Makefile names the entry points.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context as _, Result, bail, ensure};

mod assets;
mod bundle;
mod licenses;
#[cfg(test)]
mod readme;
mod release;
mod signing;

pub(crate) use assets::*;
pub(crate) use bundle::*;
pub(crate) use licenses::*;
pub(crate) use release::*;
pub(crate) use signing::*;

pub(crate) const APP_NAME: &str = "Lowspeak";
pub(crate) const BUNDLE_ID: &str = "com.lowspeak.app";
pub(crate) const MINIMUM_SYSTEM: &str = "14.0";

const USAGE: &str = "usage: cargo xtask bundle [--debug] | deploy [--debug] | dmg | universal | intel-tests | icon | signing-identity
  bundle            target/bundle/Lowspeak.app, signed for this Mac; raises the build number
  deploy            bundle, replace the installed copy, launch it, pack target/bundle/Lowspeak-<version>.dmg (not notarized)
  dmg               the release: dist/Lowspeak-<version>.dmg, universal, signed with the Developer ID and notarized
  universal         target/universal/Lowspeak for Apple Silicon and Intel, unsigned; needs rustup
  intel-tests       the tests as Intel code under Rosetta, built as the universal program
  icon              assets/AppIcon.icns and assets/readme-icon.png from assets/icon-1024.png
  signing-identity  a local code signing certificate, so updates keep the permissions";

pub(crate) fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives in the workspace")
        .to_path_buf()
}

pub(crate) fn run(command: &mut Command) -> Result<()> {
    let status = command
        .status()
        .with_context(|| format!("starting {command:?}"))?;
    ensure!(status.success(), "{command:?} failed with {status}");
    Ok(())
}

pub(crate) fn output(command: &mut Command) -> Result<String> {
    let output = command
        .stderr(Stdio::inherit())
        .output()
        .with_context(|| format!("starting {command:?}"))?;
    ensure!(
        output.status.success(),
        "{command:?} failed with {}",
        output.status
    );
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Major, minor and build number, digits only.
pub(crate) fn is_version(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
}

/// The version the next build gets, from VERSION: major and minor are raised by hand, the build
/// number by every release build.
pub(crate) fn version() -> Result<String> {
    let path = root().join("VERSION");
    let text =
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let version = text.trim();
    ensure!(
        is_version(version),
        "{}: {version:?} is not major.minor.build",
        path.display()
    );
    Ok(version.to_owned())
}

pub(crate) fn following(version: &str) -> Result<String> {
    let (head, build) = version.rsplit_once('.').context("no build number")?;
    let build: u64 = build
        .parse()
        .with_context(|| format!("build number {build}"))?;
    Ok(format!("{head}.{}", build + 1))
}

/// Raises the build number in VERSION past `built`, the version just built.
pub(crate) fn next_build(built: &str) -> Result<()> {
    let next = following(built)?;
    std::fs::write(root().join("VERSION"), format!("{next}\n"))?;
    println!("Built {built}, next build {next}");
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(unknown) = args.iter().skip(1).find(|arg| arg.as_str() != "--debug") {
        bail!("unknown option {unknown}\n{USAGE}");
    }
    let build = if args.iter().any(|arg| arg == "--debug") {
        Build::Debug
    } else {
        Build::Release
    };
    match args.first().map(String::as_str) {
        Some("bundle") => bundle(build).map(|_| ()),
        Some("deploy") => deploy(build),
        Some("dmg") if build == Build::Debug => bail!("dmg builds the release only\n{USAGE}"),
        Some("dmg") => release().map(|image| println!("{}", image.display())),
        Some("intel-tests") => intel_tests(),
        Some("universal") => universal_program().map(|program| println!("{}", program.display())),
        Some("icon") => icon(),
        Some("signing-identity") => create_signing_identity(),
        _ => bail!("{USAGE}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_program_is_universal_only_with_the_code_for_both_processors() {
        assert!(missing_architectures("x86_64 arm64\n").is_empty());
        assert_eq!(missing_architectures("arm64\n"), ["x86_64"]);
        assert_eq!(
            missing_architectures("arm64e x86_64h\n"),
            ["arm64", "x86_64"]
        );
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
    fn the_bundle_carries_the_version_twice() {
        let plist = info_plist("1.2.3");
        assert!(
            plist.contains("<key>CFBundleShortVersionString</key>\n    <string>1.2.3</string>")
        );
        assert!(plist.contains("<key>CFBundleVersion</key>\n    <string>1.2.3</string>"));
    }
}
