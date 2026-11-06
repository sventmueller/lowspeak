//! The disk image, and the release: built with the Developer ID, notarized by Apple, stapled and checked.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Result, bail};

use crate::*;

/// Created once with `xcrun notarytool store-credentials notary --apple-id <Apple ID> --team-id <team ID>`.
pub(crate) const NOTARY_PROFILE: &str = "notary";

/// What the release being built brings, written by hand before `cargo xtask dmg`.
const NOTES: &str = "RELEASE_NOTES.md";

const REPORT_LINE: &str = "Something wrong or missing? \
     [Report it on GitHub](https://github.com/sventmueller/lowspeak/issues/new/choose).";

/// Builds the release app signed with the Developer ID, has Apple notarize it, packs it into a disk image with a
/// link to /Applications and has Apple notarize the image too, so that Gatekeeper opens both on other Macs, each with
/// its ticket stapled. Only an image that passed all of this becomes `dist/Lowspeak-<version>.dmg`, next to
/// `dist/Lowspeak-<version>.md` with the notes of `RELEASE_NOTES.md`; it replaces the older ones there, and the build
/// number goes up. A failed run leaves `dist/` and VERSION as they were.
pub(crate) fn release() -> Result<PathBuf> {
    let version = version()?;
    let dist = root().join("dist");
    let notes_path = root().join(NOTES);
    let written = std::fs::read_to_string(&notes_path)
        .with_context(|| format!("reading {}", notes_path.display()))?;
    let notes = release_notes(&written, previous_notes(&dist).as_deref())?;
    let identity = developer_id()?;
    // Checks the saved login before the release build, which takes minutes.
    run(Command::new("xcrun")
        .args(["notarytool", "history", "--keychain-profile", NOTARY_PROFILE])
        .stdout(Stdio::null()))
    .with_context(|| {
        format!(
            "no usable notarization login in the keychain profile \"{NOTARY_PROFILE}\"; create it with \
             `xcrun notarytool store-credentials {NOTARY_PROFILE} --apple-id <Apple ID> --team-id <team ID>`"
        )
    })?;

    let app = assemble(Build::Distribution, &version)?;
    println!("Signing {version} with {}", identity.name);
    sign(&app, Some(&identity))?;
    // The image's ticket reaches Gatekeeper only when the image is opened in the Finder; a copy taken out otherwise
    // would need a network connection on first launch.
    let archive = app.with_extension("zip");
    run(Command::new("ditto")
        .args(["-c", "-k", "--keepParent"])
        .arg(&app)
        .arg(&archive))?;
    notarize(&archive)?;
    std::fs::remove_file(&archive)?;
    run(Command::new("xcrun").args(["stapler", "staple"]).arg(&app))?;

    let image = disk_image(&app)?;
    run(Command::new("codesign")
        .args(["--force", "--sign", &identity.hash, "--timestamp"])
        .arg(&image))?;
    notarize(&image)?;
    run(Command::new("xcrun")
        .args(["stapler", "staple"])
        .arg(&image))?;
    run(Command::new("spctl")
        .args([
            "--assess",
            "--type",
            "open",
            "--context",
            "context:primary-signature",
            "--verbose",
        ])
        .arg(&image))?;

    std::fs::create_dir_all(&dist)?;
    let name = format!("{APP_NAME}-{version}");
    let published = dist.join(format!("{name}.dmg"));
    // Copied rather than renamed: target/ may be a link to another volume.
    std::fs::copy(&image, &published)
        .with_context(|| format!("copying to {}", published.display()))?;
    std::fs::remove_file(&image)?;
    std::fs::write(dist.join(format!("{name}.md")), &notes)?;
    for entry in std::fs::read_dir(&dist)?.flatten() {
        let file = entry.file_name().to_string_lossy().into_owned();
        if released_version(&file).is_some_and(|other| other != version) {
            run(Command::new("trash").arg(entry.path()))?;
        }
    }
    next_build(&version)?;
    Ok(published)
}

/// What goes with the release: the notes written for it and the way to report a problem.
pub(crate) fn release_notes(written: &str, previous: Option<&str>) -> Result<String> {
    let written = written.trim();
    if written.is_empty() {
        bail!("{NOTES} is empty; write what this release brings");
    }
    if previous.is_some_and(|notes| notes.contains(written)) {
        bail!("{NOTES} still holds the notes of the previous release");
    }
    Ok(format!("{written}\n\n{REPORT_LINE}\n"))
}

/// The notes of the release `dist/` holds, if any.
fn previous_notes(dist: &Path) -> Option<String> {
    std::fs::read_dir(dist)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            path.extension().is_some_and(|ext| ext == "md")
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .and_then(released_version)
                    .is_some()
        })
        .and_then(|path| std::fs::read_to_string(path).ok())
}

/// The version in the name of a released image or its notes, `Lowspeak-<version>.dmg` or `.md`.
pub(crate) fn released_version(file: &str) -> Option<&str> {
    let rest = file.strip_prefix(APP_NAME)?.strip_prefix('-')?;
    let version = rest
        .strip_suffix(".dmg")
        .or_else(|| rest.strip_suffix(".md"))?;
    is_version(version).then_some(version)
}

/// Packs the app with a link to /Applications into a compressed, read-only `Lowspeak.dmg` next to it.
pub(crate) fn disk_image(app: &Path) -> Result<PathBuf> {
    let out = app.parent().context("the app has no parent directory")?;
    let staging = out.join("dmg-staging");
    let writable = out.join(format!("{APP_NAME}-writable.dmg"));
    let image = out.join(format!("{APP_NAME}.dmg"));
    for file in [&writable, &image] {
        if file.exists() {
            std::fs::remove_file(file)?;
        }
    }
    if staging.exists() {
        std::fs::remove_dir_all(&staging)?;
    }
    std::fs::create_dir_all(&staging)?;
    run(Command::new("ditto")
        .arg(app)
        .arg(staging.join(format!("{APP_NAME}.app"))))?;
    std::os::unix::fs::symlink("/Applications", staging.join("Applications"))?;
    run(Command::new("hdiutil")
        .args([
            "create",
            "-volname",
            APP_NAME,
            "-format",
            "UDRW",
            "-ov",
            "-srcfolder",
        ])
        .arg(&staging)
        .arg(&writable)
        .stdout(Stdio::null()))?;
    let attached = output(
        Command::new("hdiutil")
            .args(["attach", "-readwrite", "-noverify", "-noautoopen"])
            .arg(&writable),
    )?;
    let mount = attached
        .lines()
        .find_map(|line| {
            line.split('\t')
                .map(str::trim)
                .find(|field| field.starts_with("/Volumes/"))
        })
        .map(PathBuf::from)
        .context("hdiutil attach reported no mount point")?;
    let laid_out = lay_out(&mount);
    let detached = run(Command::new("hdiutil")
        .arg("detach")
        .arg(&mount)
        .stdout(Stdio::null()));
    if detached.is_err() {
        run(Command::new("hdiutil")
            .args(["detach", "-force"])
            .arg(&mount)
            .stdout(Stdio::null()))?;
    }
    if let Err(error) = laid_out {
        println!("Finder layout skipped, the image opens with default icon positions: {error:#}");
    }
    run(Command::new("hdiutil")
        .args([
            "convert",
            "-format",
            "UDZO",
            "-imagekey",
            "zlib-level=9",
            "-o",
        ])
        .arg(&image)
        .arg(&writable)
        .stdout(Stdio::null()))?;
    std::fs::remove_file(&writable)?;
    std::fs::remove_dir_all(&staging)?;
    Ok(image)
}

/// Finder keeps window size, icon positions and background in the image's .DS_Store, so only Finder itself can
/// write them, on the mounted image. Needs the Automation permission for Finder.
fn lay_out(mount: &Path) -> Result<()> {
    let background = root().join("assets").join("dmg-background.png");
    std::fs::create_dir_all(mount.join(".background"))?;
    std::fs::copy(&background, mount.join(".background/background.png"))
        .with_context(|| format!("copying {}", background.display()))?;
    let disk = mount
        .file_name()
        .and_then(|name| name.to_str())
        .context("mount point without a name")?;
    let script = format!(
        r#"tell application "Finder"
  tell disk "{disk}"
    open
    set current view of container window to icon view
    set toolbar visible of container window to false
    set statusbar visible of container window to false
    set bounds of container window to {{400, 200, 900, 500}}
    set viewOptions to the icon view options of container window
    set arrangement of viewOptions to not arranged
    set icon size of viewOptions to 96
    set background picture of viewOptions to file ".background:background.png"
    set position of item "{APP_NAME}.app" of container window to {{120, 150}}
    set position of item "Applications" of container window to {{380, 150}}
    close
    open
    update without registering applications
    delay 1
    close
  end tell
end tell"#
    );
    run(Command::new("osascript").args(["-e", &script]))
}

/// Uploads a disk image or zip archive to Apple's notary service and waits for the verdict; on a rejection it prints
/// the service's log, which names each problem.
fn notarize(file: &Path) -> Result<()> {
    println!("Waiting for Apple's notary service, usually a few minutes");
    let mut submit = Command::new("xcrun");
    submit
        .args([
            "notarytool",
            "submit",
            "--keychain-profile",
            NOTARY_PROFILE,
            "--wait",
            "--output-format",
            "json",
        ])
        .arg(file)
        .stderr(Stdio::inherit());
    let out = submit
        .output()
        .with_context(|| format!("starting {submit:?}"))?;
    let result = String::from_utf8_lossy(&out.stdout);
    let status = json_string(&result, "status");
    if status == Some("Accepted") && out.status.success() {
        return Ok(());
    }
    if let Some(id) = json_string(&result, "id") {
        let _ = run(Command::new("xcrun").args([
            "notarytool",
            "log",
            id,
            "--keychain-profile",
            NOTARY_PROFILE,
        ]));
    }
    bail!(
        "notarization failed: status {}, notarytool {}",
        status.unwrap_or("unknown"),
        out.status
    )
}

/// The value of a top-level string field in notarytool's flat JSON output.
fn json_string<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let rest = &json[json.find(&format!("\"{key}\""))? + key.len() + 2..];
    let rest = rest
        .trim_start()
        .strip_prefix(':')?
        .trim_start()
        .strip_prefix('"')?;
    rest.split('"').next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn released_files_are_named_with_their_version() {
        assert_eq!(released_version("Lowspeak-1.0.3.dmg"), Some("1.0.3"));
        assert_eq!(released_version("Lowspeak-1.0.3.md"), Some("1.0.3"));
        assert_eq!(released_version("Lowspeak.dmg"), None);
        assert_eq!(released_version("Lowspeak-notes.md"), None);
        assert_eq!(released_version("Other-1.0.3.dmg"), None);
    }

    #[test]
    fn the_notes_are_new_and_name_the_way_to_report_a_problem() {
        let notes = release_notes("- New.\n", None).unwrap();
        assert_eq!(notes, format!("- New.\n\n{REPORT_LINE}\n"));
        assert!(release_notes(" \n", None).is_err());
        assert!(release_notes("- New.\n", Some(&notes)).is_err());
        assert!(release_notes("- Newer.\n", Some(&notes)).is_ok());
    }

    #[test]
    fn the_notary_verdict_is_read_from_its_json() {
        let json = r#"{"id":"2b1c-77","message":"Processing complete","status":"Invalid"}"#;
        assert_eq!(json_string(json, "status"), Some("Invalid"));
        assert_eq!(json_string(json, "id"), Some("2b1c-77"));
        assert_eq!(json_string(json, "missing"), None);
    }
}
