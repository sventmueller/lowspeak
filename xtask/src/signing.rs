//! Code signing: the identity to sign with, and the local certificate that keeps the permissions across updates.

use std::os::unix::fs::OpenOptionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Result, ensure};

use crate::*;

#[derive(Clone)]
pub(crate) struct Identity {
    pub(crate) hash: String,
    pub(crate) name: String,
    /// Set for the local identity, which lives in its own keychain.
    pub(crate) keychain: Option<PathBuf>,
}

pub(crate) const LOCAL_IDENTITY: &str = "Lowspeak Local Signing";

/// Keychain and password file of the self-signed identity created by `signing-identity`.
pub(crate) fn local_signing_files() -> Result<(PathBuf, PathBuf)> {
    let home = std::env::home_dir().context("no home directory")?;
    let dir = home.join("Library/Application Support/Lowspeak/Signing");
    Ok((
        dir.join("signing.keychain-db"),
        dir.join("keychain-password"),
    ))
}

pub(crate) fn parse_identities(listing: &str) -> Vec<Identity> {
    listing
        .lines()
        .take_while(|line| !line.contains("Valid identities only"))
        .filter_map(|line| {
            let (_, rest) = line.trim().split_once(") ")?;
            let (hash, name) = rest.split_once(' ')?;
            let name = name.split('"').nth(1)?;
            Some(Identity {
                hash: hash.to_owned(),
                name: name.to_owned(),
                keychain: None,
            })
        })
        .collect()
}

pub(crate) fn local_identity() -> Result<Option<Identity>> {
    let (keychain, password_file) = local_signing_files()?;
    if !keychain.exists() || !password_file.exists() {
        return Ok(None);
    }
    let password = std::fs::read_to_string(&password_file)?;
    run(Command::new("security")
        .args(["unlock-keychain", "-p", password.trim()])
        .arg(&keychain))?;
    // Self-signed, so it is listed among the matching but not the "valid" identities.
    let listing = output(
        Command::new("security")
            .args(["find-identity", "-p", "codesigning"])
            .arg(&keychain),
    )?;
    Ok(parse_identities(&listing)
        .into_iter()
        .find(|identity| identity.name == LOCAL_IDENTITY)
        .map(|identity| Identity {
            keychain: Some(keychain),
            ..identity
        }))
}

/// Developer ID, then Apple Development, then the local identity, else ad hoc. The installed copy keeps its
/// permissions only while the identity stays the same.
pub(crate) fn signing_identity() -> Result<Option<Identity>> {
    let listing =
        output(Command::new("security").args(["find-identity", "-v", "-p", "codesigning"]))?;
    let identities = parse_identities(&listing);
    let apple = ["Developer ID Application", "Apple Development"]
        .iter()
        .find_map(|kind| {
            identities
                .iter()
                .find(|identity| identity.name.starts_with(kind))
                .cloned()
        });
    match apple {
        Some(identity) => Ok(Some(identity)),
        None => local_identity(),
    }
}

/// The first valid Developer ID Application identity in the keychains, which notarization requires.
pub(crate) fn developer_id() -> Result<Identity> {
    let listing =
        output(Command::new("security").args(["find-identity", "-v", "-p", "codesigning"]))?;
    parse_identities(&listing)
        .into_iter()
        .find(|identity| identity.name.starts_with("Developer ID Application:"))
        .context("no Developer ID Application certificate in the keychain")
}

/// Signs with the hardened runtime, the app with the entitlements; `None` signs ad hoc. Sparkle's helpers are signed
/// first, inside out, as its documentation prescribes instead of `--deep`.
pub(crate) fn sign(app: &Path, identity: Option<&Identity>) -> Result<()> {
    match identity {
        Some(identity) => println!("Signing with {}", identity.name),
        None => println!(
            "WARNING: no signing certificate — signing ad hoc. macOS then asks for the \
             Accessibility permission again after every install. `cargo xtask signing-identity` \
             creates a local certificate that avoids this."
        ),
    }
    let sparkle = app.join("Contents/Frameworks/Sparkle.framework");
    let mut parts = Vec::new();
    if sparkle.exists() {
        parts.extend([
            sparkle.join("Versions/B/Autoupdate"),
            sparkle.join("Versions/B/Updater.app"),
            sparkle,
        ]);
    }
    parts.push(app.to_path_buf());
    for part in parts {
        let mut command = Command::new("codesign");
        command.args(["--force", "--options", "runtime"]);
        if part == app {
            command
                .arg("--entitlements")
                .arg(root().join("assets").join("Lowspeak.entitlements"));
        }
        match identity {
            Some(identity) => {
                match &identity.keychain {
                    Some(keychain) => {
                        command
                            .args(["--timestamp=none", "--keychain"])
                            .arg(keychain);
                    }
                    None => {
                        command.arg("--timestamp");
                    }
                }
                command.args(["--sign", &identity.hash]);
            }
            None => {
                command.args(["--sign", "-"]);
            }
        }
        run(command.arg(&part))?;
    }
    run(Command::new("codesign")
        .args(["--verify", "--strict"])
        .arg(app))
}

pub(crate) fn random_password() -> Result<String> {
    let mut bytes = [0u8; 24];
    std::io::Read::read_exact(&mut std::fs::File::open("/dev/urandom")?, &mut bytes)?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

/// Creates a self-signed code signing certificate in a keychain of its own. Builds signed with
/// it keep a stable designated requirement, so macOS keeps the Accessibility and microphone
/// permissions across updates.
pub(crate) fn create_signing_identity() -> Result<()> {
    let (keychain, password_file) = local_signing_files()?;
    ensure!(!keychain.exists(), "{} already exists", keychain.display());
    let dir = keychain.parent().expect("keychain path has a parent");
    std::fs::create_dir_all(dir)?;
    let work = dir.join("tmp");
    std::fs::create_dir_all(&work)?;
    let result = (|| -> Result<()> {
        let (key, cert, bundle) = (
            work.join("key.pem"),
            work.join("cert.pem"),
            work.join("identity.p12"),
        );
        run(Command::new("/usr/bin/openssl")
            .args([
                "req", "-x509", "-newkey", "rsa:2048", "-nodes", "-days", "3650",
            ])
            .args(["-subj", &format!("/CN={LOCAL_IDENTITY}")])
            .args(["-addext", "basicConstraints=critical,CA:false"])
            .args(["-addext", "keyUsage=critical,digitalSignature"])
            .args(["-addext", "extendedKeyUsage=critical,codeSigning"])
            .arg("-keyout")
            .arg(&key)
            .arg("-out")
            .arg(&cert)
            .stderr(Stdio::null()))?;
        let transfer = random_password()?;
        run(Command::new("/usr/bin/openssl")
            .args(["pkcs12", "-export", "-inkey"])
            .arg(&key)
            .arg("-in")
            .arg(&cert)
            .arg("-out")
            .arg(&bundle)
            .args(["-passout", &format!("pass:{transfer}")]))?;

        let password = random_password()?;
        run(Command::new("security")
            .args(["create-keychain", "-p", &password])
            .arg(&keychain))?;
        // No automatic locking; `local_identity` unlocks it before every signing anyway.
        run(Command::new("security")
            .arg("set-keychain-settings")
            .arg(&keychain))?;
        run(Command::new("security")
            .args(["unlock-keychain", "-p", &password])
            .arg(&keychain))?;
        run(Command::new("security")
            .arg("import")
            .arg(&bundle)
            .arg("-k")
            .arg(&keychain)
            .args(["-P", &transfer, "-T", "/usr/bin/codesign"])
            .stdout(Stdio::null()))?;
        // Lets codesign use the key without a password dialog.
        run(Command::new("security")
            .args([
                "set-key-partition-list",
                "-S",
                "apple-tool:,apple:,codesign:",
                "-s",
                "-k",
                &password,
            ])
            .arg(&keychain)
            .stdout(Stdio::null()))?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&password_file)?;
        std::io::Write::write_all(&mut file, password.as_bytes())?;
        Ok(())
    })();
    std::fs::remove_dir_all(&work)?;
    result?;
    let identity = local_identity()?.context("the new identity is not usable for code signing")?;
    println!(
        "Created {} ({}) in {}",
        identity.name,
        identity.hash,
        keychain.display()
    );
    Ok(())
}
