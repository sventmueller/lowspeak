//! The app bundle: Info.plist, the signed bundle, and installing it.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::Result;

use crate::*;

/// The interface languages, named as the README folders and the folders of `resources/`. macOS shows the texts of
/// frameworks such as Sparkle and AppKit only in a language the app declares.
pub(crate) const LOCALIZATIONS: [&str; 35] = [
    "ar", "ca", "cs", "da", "de", "el", "en", "es", "fi", "fr", "he", "hi", "hr", "hu", "id", "it",
    "ja", "ko", "ms", "nl", "no", "pl", "pt_BR", "pt_PT", "ro", "ru", "sk", "sl", "sv", "th", "tr",
    "uk", "vi", "zh-Hans", "zh-Hant",
];

/// `resources/<language>.lproj/InfoPlist.strings`: the property list's texts macOS shows, translated; of those, only
/// the microphone prompt shows the app's own words.
pub(crate) const STRINGS: &str = "InfoPlist.strings";

/// `updates` adds what Sparkle reads: where the feed is and the key every update must be signed with.
pub(crate) fn info_plist(version: &str, updates: bool) -> String {
    let localizations: String = LOCALIZATIONS
        .iter()
        .map(|language| format!("        <string>{language}</string>\n"))
        .collect();
    let updates = if updates {
        format!(
            "    <key>SUFeedURL</key>\n    <string>{}</string>\n    <key>SUPublicEDKey</key>\n    <string>{UPDATE_KEY}</string>\n",
            feed_url()
        )
    } else {
        String::new()
    };
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleLocalizations</key>
    <array>
{localizations}    </array>
    <key>CFBundleExecutable</key>
    <string>{APP_NAME}</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>{BUNDLE_ID}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>{APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>{version}</string>
    <key>CFBundleVersion</key>
    <string>{version}</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.productivity</string>
    <key>LSMinimumSystemVersion</key>
    <string>{MINIMUM_SYSTEM}</string>
    <key>LSUIElement</key>
    <true/>
    <key>NSMicrophoneUsageDescription</key>
    <string>Lowspeak needs microphone access to record speech for transcription.</string>
    <key>NSAccessibilityUsageDescription</key>
    <string>Lowspeak needs accessibility permission to detect the fn (Globe) key and type transcribed text.</string>
    <key>NSHumanReadableCopyright</key>
    <string>© 2026 Sven Müller</string>
{updates}</dict>
</plist>
"#
    )
}

/// The targets of the universal program, with the names `lipo -archs` prints for them.
pub(crate) const UNIVERSAL: [(&str, &str); 2] = [
    ("aarch64-apple-darwin", "arm64"),
    ("x86_64-apple-darwin", "x86_64"),
];

/// What a build is for: debugging, the copy installed on this Mac, or the notarized disk image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Build {
    Debug,
    Release,
    Distribution,
}

impl Build {
    /// Only the disk image carries Sparkle: an update must not replace a build of one's own.
    pub(crate) fn updates(self) -> bool {
        self == Build::Distribution
    }
}

/// Builds the program and lays out `target/bundle/Lowspeak.app` around it, unsigned. The distribution carries the
/// universal program and Sparkle, the other builds the program for this Mac.
pub(crate) fn assemble(build: Build, version: &str) -> Result<PathBuf> {
    let root = root();
    let release = build != Build::Debug;
    let mut cargo = Command::new("cargo");
    cargo
        .current_dir(&root)
        .args(["build", "--package", "lowspeak", "--bin", APP_NAME]);
    if release {
        cargo.arg("--release");
    }
    let binary = if build == Build::Distribution {
        universal_program()?
    } else {
        run(&mut cargo)?;
        let profile = if release { "release" } else { "debug" };
        root.join("target").join(profile).join(APP_NAME)
    };
    let app = root
        .join("target")
        .join("bundle")
        .join(format!("{APP_NAME}.app"));
    if app.exists() {
        std::fs::remove_dir_all(&app)
            .with_context(|| format!("removing the old {}", app.display()))?;
    }
    let contents = app.join("Contents");
    std::fs::create_dir_all(contents.join("MacOS"))?;
    std::fs::create_dir_all(contents.join("Resources"))?;
    std::fs::copy(&binary, contents.join("MacOS").join(APP_NAME))
        .with_context(|| format!("copying {}", binary.display()))?;
    std::fs::copy(
        root.join("assets").join("AppIcon.icns"),
        contents.join("Resources").join("AppIcon.icns"),
    )
    .context("copying the icon (run `cargo xtask icon` first)")?;
    for language in LOCALIZATIONS {
        let lproj = format!("{language}.lproj");
        let target = contents.join("Resources").join(&lproj);
        std::fs::create_dir_all(&target)?;
        let strings = root.join("resources").join(&lproj).join(STRINGS);
        std::fs::copy(&strings, target.join(STRINGS))
            .with_context(|| format!("copying {}", strings.display()))?;
    }
    std::fs::write(
        contents.join("Info.plist"),
        info_plist(version, build.updates()),
    )?;
    let mut bundled_sparkle = None;
    if build.updates() {
        let sparkle = sparkle()?;
        let frameworks = contents.join("Frameworks");
        std::fs::create_dir_all(&frameworks)?;
        let framework = frameworks.join("Sparkle.framework");
        run(Command::new("ditto")
            .arg(sparkle.join("Sparkle.framework"))
            .arg(&framework))?;
        // Only a sandboxed app needs Sparkle's XPC services.
        std::fs::remove_dir_all(framework.join("Versions/B/XPCServices"))?;
        std::fs::remove_file(framework.join("XPCServices"))?;
        let code = framework.join("Versions/B");
        for program in [
            code.join("Sparkle"),
            code.join("Autoupdate"),
            code.join("Updater.app/Contents/MacOS/Updater"),
        ] {
            check_universal(&program)?;
        }
        bundled_sparkle = Some(sparkle);
    }
    std::fs::write(
        contents.join("Resources").join(ACKNOWLEDGEMENTS),
        acknowledgements(bundled_sparkle.as_deref())?,
    )?;
    run(Command::new("plutil")
        .arg("-lint")
        .arg(contents.join("Info.plist"))
        .stdout(Stdio::null()))?;
    Ok(app)
}

/// The release program for Apple Silicon and Intel in one file, through rustup with the toolchain rust-toolchain.toml
/// names, since Homebrew's Rust ships the standard library for the host processor alone.
pub(crate) fn universal_program() -> Result<PathBuf> {
    let mut join = Command::new("lipo");
    join.arg("-create");
    for (target, _) in UNIVERSAL {
        let mut build = cargo_for(target)?;
        run(build
            .args([
                "build",
                "--package",
                "lowspeak",
                "--bin",
                APP_NAME,
                "--release",
            ])
            .args(["--target", target]))?;
        join.arg(universal_dir().join(target).join("release").join(APP_NAME));
    }
    let program = universal_dir().join(APP_NAME);
    run(join.arg("-output").arg(&program))?;
    check_universal(&program)?;
    Ok(program)
}

/// The tests of `make check` as Intel code, which Rosetta runs on Apple Silicon, built as the release builds it.
pub(crate) fn intel_tests() -> Result<()> {
    let (target, _) = UNIVERSAL[1];
    run(cargo_for(target)?.args(["test", "--release", "--workspace", "--target", target]))
}

/// The universal program builds in a target directory of its own: whisper-rs-sys passes the `GGML_*` settings to
/// whisper.cpp but does not rebuild when they change, so other builds for these targets must not share its output.
fn universal_dir() -> PathBuf {
    root().join("target").join("universal")
}

/// cargo of rust-toolchain.toml with what a build of the distributed program for `target` needs.
fn cargo_for(target: &str) -> Result<Command> {
    let mut cargo = rustup_cargo()?;
    cargo
        .env("CARGO_TARGET_DIR", universal_dir())
        // whisper.cpp otherwise tunes itself for the processor it is built on, which other Macs may lack.
        .env("GGML_NATIVE", "OFF");
    if target.starts_with("x86_64") {
        // Every Intel Mac that runs macOS 14 has these.
        for flag in [
            "GGML_SSE42",
            "GGML_AVX",
            "GGML_AVX2",
            "GGML_BMI2",
            "GGML_FMA",
            "GGML_F16C",
        ] {
            cargo.env(flag, "ON");
        }
    }
    Ok(cargo)
}

/// cargo of the toolchain rust-toolchain.toml names, with its targets.
pub(crate) fn rustup_cargo() -> Result<Command> {
    let root = root();
    let rustup = rustup()?;
    run(Command::new(&rustup)
        .current_dir(&root)
        .args(["toolchain", "install"]))?;
    let tool = |name: &str| {
        output(
            Command::new(&rustup)
                .current_dir(&root)
                .args(["which", name]),
        )
        .map(|path| path.trim().to_owned())
    };
    let mut cargo = Command::new(tool("cargo")?);
    // Cargo would otherwise run the rustc and rustdoc first on the PATH, which may be Homebrew's.
    cargo
        .current_dir(&root)
        .env("RUSTC", tool("rustc")?)
        .env("RUSTDOC", tool("rustdoc")?);
    Ok(cargo)
}

/// rustup from the PATH, or from Homebrew, which keeps it off the PATH beside its own Rust.
fn rustup() -> Result<PathBuf> {
    let on_path = Command::new("rustup")
        .arg("--version")
        .output()
        .is_ok_and(|out| out.status.success());
    if on_path {
        return Ok("rustup".into());
    }
    output(Command::new("brew").args(["--prefix", "rustup"]))
        .map(|prefix| Path::new(prefix.trim()).join("bin/rustup"))
        .ok()
        .filter(|path| path.exists())
        .context("this needs rustup; install it with `brew install rustup`")
}

/// The architectures of `UNIVERSAL` that are not in the output of `lipo -archs`.
pub(crate) fn missing_architectures(listed: &str) -> Vec<&'static str> {
    UNIVERSAL
        .iter()
        .map(|(_, arch)| *arch)
        .filter(|arch| !listed.split_whitespace().any(|name| name == *arch))
        .collect()
}

/// A program without the code for one processor goes unnoticed on the other and keeps the app from starting there.
pub(crate) fn check_universal(program: &Path) -> Result<()> {
    let missing = missing_architectures(&output(Command::new("lipo").arg("-archs").arg(program))?);
    ensure!(
        missing.is_empty(),
        "{} lacks {}",
        program.display(),
        missing.join(" and ")
    );
    Ok(())
}

/// Builds and signs the app for this Mac. A release build raises the build number in VERSION once it is signed.
pub(crate) fn bundle(build: Build) -> Result<(PathBuf, String)> {
    let version = version()?;
    let app = assemble(build, &version)?;
    sign(&app, signing_identity()?.as_ref())?;
    println!("Bundle: {} ({version})", app.display());
    if build == Build::Release {
        next_build(&version)?;
    }
    Ok((app, version))
}

/// The installed copy, wherever under /Applications it lives.
pub(crate) fn installed_location() -> PathBuf {
    let found =
        output(Command::new("mdfind").arg(format!("kMDItemCFBundleIdentifier == '{BUNDLE_ID}'")))
            .unwrap_or_default();
    found
        .lines()
        .map(PathBuf::from)
        .find(|path| {
            path.starts_with("/Applications") && path.extension().is_some_and(|ext| ext == "app")
        })
        .unwrap_or_else(|| PathBuf::from(format!("/Applications/{APP_NAME}.app")))
}

/// Installs the build and packs it as `target/bundle/Lowspeak-<version>.dmg`, not notarized, to pass on for testing.
pub(crate) fn deploy(build: Build) -> Result<()> {
    let (app, version) = bundle(build)?;
    install(&app)?;
    let image = disk_image(&app, Layout::Optional)?;
    let named = image.with_file_name(format!("{APP_NAME}-{version}.dmg"));
    std::fs::rename(&image, &named)?;
    println!("DMG: {}", named.display());
    Ok(())
}

pub(crate) fn install(app: &Path) -> Result<()> {
    let target = installed_location();
    println!("Installing to {}", target.display());
    let running = || {
        Command::new("pgrep")
            .args(["-x", APP_NAME])
            .stdout(Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    };
    if running() {
        let _ = Command::new("osascript")
            .args([
                "-e",
                &format!("tell application id \"{BUNDLE_ID}\" to quit"),
            ])
            .status();
        for _ in 0..20 {
            if !running() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
        if running() {
            let _ = Command::new("pkill").args(["-x", APP_NAME]).status();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
    // The previous installation goes to the Trash rather than being deleted.
    if target.exists() {
        run(Command::new("trash").arg(&target))
            .with_context(|| format!("moving {} to the Trash", target.display()))?;
    }
    run(Command::new("ditto").arg(app).arg(&target))?;
    // `open` passes its environment on to the app, including the log directory Cargo sets.
    run(Command::new("open")
        .arg(&target)
        .env_remove("LOWSPEAK_LOG_DIR"))?;
    Ok(())
}
