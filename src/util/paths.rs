use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// `.cargo/config.toml` sets it, so tests, examples and `cargo run` keep out of the app's logs.
const LOG_DIR_ENV: &str = "LOWSPEAK_LOG_DIR";
const SYSTEM_APPLICATION_SUPPORT: &str = "/Library/Application Support";

/// `~/Library/Application Support`.
pub fn try_application_support() -> io::Result<PathBuf> {
    std::env::home_dir()
        .filter(|home| home.is_absolute())
        .map(|home| home.join("Library").join("Application Support"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no home directory"))
}

/// `try_application_support` for callers without a way to report the error: they get the
/// system-wide directory, where reading finds nothing of Lowspeak's and writing fails.
pub fn application_support() -> PathBuf {
    try_application_support().unwrap_or_else(|error| {
        report_once(&error);
        PathBuf::from(SYSTEM_APPLICATION_SUPPORT)
    })
}

/// `~/Library/Application Support/Lowspeak`, created if missing.
pub fn try_app_dir() -> io::Result<PathBuf> {
    let dir = try_application_support()?.join("Lowspeak");
    create(&dir)?;
    Ok(dir)
}

/// `try_app_dir` for callers that see the failure later, when they use the directory.
pub fn app_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        try_app_dir().unwrap_or_else(|error| {
            report_once(&error);
            application_support().join("Lowspeak")
        })
    })
    .clone()
}

/// The directory of the log files, created on first use.
pub fn log_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let dir = std::env::var_os(LOG_DIR_ENV)
            .filter(|dir| !dir.is_empty())
            .map_or_else(app_dir, PathBuf::from);
        if let Err(error) = create(&dir) {
            report_once(&error);
        }
        dir
    })
    .clone()
}

fn create(dir: &Path) -> io::Result<()> {
    std::fs::create_dir_all(dir).map_err(|error| {
        io::Error::new(error.kind(), format!("creating {}: {error}", dir.display()))
    })
}

/// Goes to stderr: the log files live in these directories, so a failure here cannot be logged.
fn report_once(error: &io::Error) {
    static REPORTED: OnceLock<()> = OnceLock::new();
    REPORTED.get_or_init(|| eprintln!("Lowspeak: {error}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_support_is_below_the_home_directory() {
        let dir = try_application_support().unwrap();
        assert!(dir.is_absolute());
        assert!(dir.ends_with("Library/Application Support"));
        assert_eq!(application_support(), dir);
    }

    #[test]
    fn a_directory_that_cannot_be_created_is_an_error_naming_it() {
        let blocker = std::env::temp_dir().join(format!("lowspeak-paths-{}", std::process::id()));
        std::fs::write(&blocker, b"").unwrap();
        let error = create(&blocker.join("child")).unwrap_err();
        let _ = std::fs::remove_file(&blocker);
        assert!(error.to_string().contains("lowspeak-paths-"), "{error}");
    }
}
