//! Sparkle, the updater only the disk image carries, and the update feed it reads.

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Result, bail, ensure};

use crate::*;

/// Sparkle's GitHub release, pinned by the checksum of its archive.
pub(crate) const SPARKLE_VERSION: &str = "2.10.1";

const SPARKLE_SHA256: &str = "1a856d2a9edb8e628de88121d7be04cf0e2167f8641650b78fca00b14244cf27";

/// Public half of the EdDSA key that signs updates, Splat's as well: Sparkle's `generate_keys` keeps the private half
/// in the login keychain, and installed copies accept only updates signed with it, so losing it ends their updates.
pub(crate) const UPDATE_KEY: &str = "21sUy80rKTKlgQLPk+T7UrAkoqVQ+VV+6rfHKVAL4EU=";

const RELEASES: &str = "https://github.com/sventmueller/lowspeak/releases";

/// The feed is an asset of the latest GitHub release, so creating the release publishes the update.
pub(crate) fn feed_url() -> String {
    format!("{RELEASES}/latest/download/{FEED}")
}

/// The feed's file name in `dist/` and among the release's assets.
pub(crate) const FEED: &str = "appcast.xml";

/// Sparkle's release archive, downloaded once into target/, checked against its pinned checksum and unpacked: the
/// framework and the `bin/` tools that sign updates.
pub(crate) fn sparkle() -> Result<PathBuf> {
    let target = root().join("target");
    let dir = target.join(format!("sparkle-{SPARKLE_VERSION}"));
    if dir.join("Sparkle.framework").exists() {
        return Ok(dir);
    }
    std::fs::create_dir_all(&target)?;
    let archive = target.join(format!("Sparkle-{SPARKLE_VERSION}.tar.xz"));
    let url = format!(
        "https://github.com/sparkle-project/Sparkle/releases/download/{SPARKLE_VERSION}/Sparkle-{SPARKLE_VERSION}.tar.xz"
    );
    run(Command::new("curl")
        .args([
            "-fsSL",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "-o",
        ])
        .arg(&archive)
        .arg(&url))?;
    let listed = output(Command::new("shasum").args(["-a", "256"]).arg(&archive))?;
    let actual = listed.split_whitespace().next().unwrap_or_default();
    if actual != SPARKLE_SHA256 {
        std::fs::remove_file(&archive)?;
        bail!("{url} has checksum {actual}, expected {SPARKLE_SHA256}");
    }
    let unpacking = target.join(format!("sparkle-{SPARKLE_VERSION}.partial"));
    if unpacking.exists() {
        std::fs::remove_dir_all(&unpacking)?;
    }
    std::fs::create_dir_all(&unpacking)?;
    run(Command::new("tar")
        .arg("-xf")
        .arg(&archive)
        .arg("-C")
        .arg(&unpacking))?;
    std::fs::rename(&unpacking, &dir)?;
    std::fs::remove_file(&archive)?;
    Ok(dir)
}

/// Stops unless the login keychain holds the private half of `UPDATE_KEY`.
pub(crate) fn check_update_key() -> Result<()> {
    let generate_keys = sparkle()?.join("bin/generate_keys");
    let key = output(Command::new(&generate_keys).arg("-p")).with_context(|| {
        format!(
            "no update signing key in the keychain; import its backup with `{} -f <file>`",
            generate_keys.display()
        )
    })?;
    ensure!(
        key.trim() == UPDATE_KEY,
        "the keychain's update signing key {} is not UPDATE_KEY {UPDATE_KEY}",
        key.trim()
    );
    Ok(())
}

/// The update feed: the release's disk image with the EdDSA signature and length `sign_update` printed for it, and
/// its notes as Markdown. Macs older than `MINIMUM_SYSTEM` are not offered it.
pub(crate) fn appcast(version: &str, signature: &str, date: &str, notes: &str) -> Result<String> {
    ensure!(
        signature.starts_with("sparkle:edSignature=\"") && signature.contains(" length=\""),
        "sign_update printed {signature:?}"
    );
    ensure!(
        !notes.contains("]]>"),
        "the notes contain ]]>, which would end the feed's text early"
    );
    Ok(format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0" xmlns:sparkle="http://www.andymatuschak.org/xml-namespaces/sparkle">
  <channel>
    <title>{APP_NAME}</title>
    <item>
      <title>{APP_NAME} {version}</title>
      <pubDate>{date}</pubDate>
      <sparkle:version>{version}</sparkle:version>
      <sparkle:shortVersionString>{version}</sparkle:shortVersionString>
      <sparkle:minimumSystemVersion>{MINIMUM_SYSTEM}</sparkle:minimumSystemVersion>
      <description sparkle:format="markdown"><![CDATA[{notes}]]></description>
      <enclosure url="{RELEASES}/download/v{version}/{APP_NAME}-{version}.dmg" {signature} type="application/octet-stream"/>
    </item>
  </channel>
</rss>
"#
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_feed_offers_the_signed_image_of_its_release() {
        let feed = appcast(
            "1.0.4",
            "sparkle:edSignature=\"c2ln\" length=\"42\"",
            "Sat, 07 Nov 2026 12:00:00 +0000",
            "- New.",
        )
        .unwrap();
        assert!(feed.contains(
            "<enclosure url=\"https://github.com/sventmueller/lowspeak/releases/download/v1.0.4/Lowspeak-1.0.4.dmg\" \
             sparkle:edSignature=\"c2ln\" length=\"42\" type=\"application/octet-stream\"/>"
        ));
        assert!(feed.contains("<sparkle:version>1.0.4</sparkle:version>"));
        assert!(feed.contains("<sparkle:minimumSystemVersion>14.0</sparkle:minimumSystemVersion>"));
        assert!(
            feed.contains(
                "<description sparkle:format=\"markdown\"><![CDATA[- New.]]></description>"
            )
        );
        assert!(appcast("1.0.4", "ERROR: no key", "", "").is_err());
        assert!(
            appcast(
                "1.0.4",
                "sparkle:edSignature=\"c2ln\" length=\"42\"",
                "",
                "a ]]> b"
            )
            .is_err()
        );
        assert_eq!(
            feed_url(),
            "https://github.com/sventmueller/lowspeak/releases/latest/download/appcast.xml"
        );
    }
}
