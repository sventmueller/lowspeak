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
mod updates;
mod version;

pub(crate) use assets::*;
pub(crate) use bundle::*;
pub(crate) use licenses::*;
pub(crate) use release::*;
pub(crate) use signing::*;
pub(crate) use updates::*;
pub(crate) use version::*;

pub(crate) const APP_NAME: &str = "Lowspeak";
pub(crate) const BUNDLE_ID: &str = "com.lowspeak.app";
pub(crate) const MINIMUM_SYSTEM: &str = "14.0";

const USAGE: &str = "usage: cargo xtask bundle [--debug] | deploy [--debug] | dmg | universal | intel-tests | icon | signing-identity
  bundle            target/bundle/Lowspeak.app at the version in Cargo.toml, signed for this Mac
  deploy            raise the build number, bundle, replace the installed copy, launch it, pack
                    target/bundle/Lowspeak-<version>.dmg (not notarized), commit the version
  dmg               the release: raise the build number, dist/Lowspeak-<version>.dmg, universal, signed with the
                    Developer ID and notarized, commit the version
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
    ensure!(!interrupted(), "interrupted");
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
    ensure!(!interrupted(), "interrupted");
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
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
        Some("bundle") => bundle(build, &version()?).map(|_| ()),
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
    fn the_bundle_carries_the_version_twice() {
        let plist = info_plist("1.2.3", false);
        assert!(
            plist.contains("<key>CFBundleShortVersionString</key>\n    <string>1.2.3</string>")
        );
        assert!(plist.contains("<key>CFBundleVersion</key>\n    <string>1.2.3</string>"));
        assert!(plist.contains(
            "<key>CFBundleLocalizations</key>\n    <array>\n        <string>ar</string>\n"
        ));
        assert!(plist.contains("        <string>zh-Hant</string>\n    </array>\n"));
    }

    #[test]
    fn every_language_explains_the_microphone_prompt() {
        let plist = info_plist("1.2.3", false);
        let english = plist
            .split("<key>NSMicrophoneUsageDescription</key>\n    <string>")
            .nth(1)
            .and_then(|rest| rest.split("</string>").next())
            .unwrap();
        let mut folders: Vec<String> = std::fs::read_dir(root().join("resources"))
            .unwrap()
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        folders.sort_unstable();
        let expected: Vec<String> = LOCALIZATIONS.iter().map(|l| format!("{l}.lproj")).collect();
        assert_eq!(folders, expected);
        for language in LOCALIZATIONS {
            let path = root().join(format!("resources/{language}.lproj/{STRINGS}"));
            let text = std::fs::read_to_string(&path).unwrap();
            let value = text
                .strip_prefix("\"NSMicrophoneUsageDescription\" = \"")
                .and_then(|rest| rest.strip_suffix("\";\n"))
                .unwrap_or_else(|| panic!("{}: {text:?}", path.display()));
            assert!(
                value.contains("Lowspeak") && !value.contains('"'),
                "{language}: {value}"
            );
            if language == "en" {
                assert_eq!(value, english);
            } else {
                assert_ne!(value, english, "{language} is not translated");
            }
        }
    }

    #[test]
    fn only_the_disk_image_looks_for_updates() {
        assert!(!info_plist("1.2.3", false).contains("SUFeedURL"));
        let published = info_plist("1.2.3", true);
        assert!(published.contains(&format!(
            "<key>SUFeedURL</key>\n    <string>{}</string>",
            feed_url()
        )));
        assert!(published.contains(&format!(
            "<key>SUPublicEDKey</key>\n    <string>{UPDATE_KEY}</string>"
        )));
        assert!(
            Build::Distribution.updates() && !Build::Release.updates() && !Build::Debug.updates()
        );
    }
}
