//! Log files in `paths::log_dir()`, rotated at 1 MB.

use std::collections::HashMap;
use std::fs::{File, OpenOptions, Permissions};
use std::io::Write as _;
use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use super::paths;

const MAX_SIZE: u64 = 1024 * 1024;
/// Lines between checks that the file still exists, in case someone deleted or moved it.
const RECHECK_EVERY: u32 = 64;
/// The logs name files, devices and the dictated apps, so only the user may read them.
const MODE: u32 = 0o600;

pub const APP: &str = "appstate.log";
pub const AUDIO: &str = "audio.log";
pub const HOTKEY: &str = "hotkey.log";
pub const CRASH: &str = "crash.log";

struct Log {
    file: File,
    size: u64,
    unchecked_lines: u32,
}

struct Logs {
    dir: PathBuf,
    max_size: u64,
    open: HashMap<&'static str, Log>,
}

static LOGS: Mutex<Option<Logs>> = Mutex::new(None);

#[macro_export]
macro_rules! flog {
    ($file:expr, $($arg:tt)*) => {
        $crate::util::filelog::write($file, &format!($($arg)*))
    };
}

/// UTC timestamp in the form `2026-10-22 15:04:05 +0000`.
pub fn timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format_timestamp(secs)
}

fn format_timestamp(secs: u64) -> String {
    let days = i64::try_from(secs / 86_400).unwrap_or(0);
    let rem = secs % 86_400;
    // Civil date from days since 1970-01-01 (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02} +0000",
        rem / 3_600,
        rem % 3_600 / 60,
        rem % 60
    )
}

fn open(path: &Path) -> Option<Log> {
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .mode(MODE)
        .open(path)
        .ok()?;
    // `mode` applies only to a file this call creates, not to a log that already exists.
    let _ = file.set_permissions(Permissions::from_mode(MODE));
    let size = file.metadata().map_or(0, |meta| meta.len());
    Some(Log {
        file,
        size,
        unchecked_lines: 0,
    })
}

impl Logs {
    fn new(dir: PathBuf, max_size: u64) -> Self {
        Self {
            dir,
            max_size,
            open: HashMap::new(),
        }
    }

    fn write(&mut self, file: &'static str, line: &str) {
        let path = self.dir.join(file);
        let max_size = self.max_size;
        let stale = self.open.get_mut(file).is_none_or(|log| {
            log.unchecked_lines += 1;
            log.size > max_size || (log.unchecked_lines >= RECHECK_EVERY && !path.exists())
        });
        if stale {
            if self
                .open
                .remove(file)
                .is_some_and(|log| log.size > max_size)
            {
                let _ = std::fs::rename(&path, self.dir.join(format!("{file}.1")));
            }
            let Some(log) = open(&path) else { return };
            self.open.insert(file, log);
        }
        if let Some(log) = self.open.get_mut(file)
            && log.file.write_all(line.as_bytes()).is_ok()
        {
            log.size += line.len() as u64;
            if log.unchecked_lines >= RECHECK_EVERY {
                log.unchecked_lines = 0;
            }
        }
    }

    fn flush(&mut self) {
        for log in self.open.values_mut() {
            let _ = log.file.sync_all();
        }
    }
}

pub fn write(file: &'static str, message: &str) {
    let line = format!("{}: {message}\n", timestamp());
    let Ok(mut guard) = LOGS.lock() else {
        return;
    };
    guard
        .get_or_insert_with(|| Logs::new(paths::log_dir(), MAX_SIZE))
        .write(file, &line);
}

pub fn flush_all() {
    if let Ok(mut guard) = LOGS.lock()
        && let Some(logs) = guard.as_mut()
    {
        logs.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("lowspeak-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn timestamps_are_utc_calendar_dates() {
        assert_eq!(format_timestamp(0), "1970-01-01 00:00:00 +0000");
        assert_eq!(format_timestamp(951_782_400), "2000-02-29 00:00:00 +0000");
        assert_eq!(format_timestamp(1_792_681_445), "2026-10-22 15:04:05 +0000");
        assert_eq!(format_timestamp(4_107_542_399), "2100-02-28 23:59:59 +0000");
        assert_eq!(timestamp().len(), 25);
    }

    #[test]
    fn a_full_log_moves_aside_and_a_new_one_starts() {
        let dir = temp_dir("rotation");
        let mut logs = Logs::new(dir.clone(), 100);
        for index in 0..5 {
            logs.write(APP, &format!("line {index} {}\n", "x".repeat(30)));
        }
        let current = std::fs::read_to_string(dir.join(APP)).unwrap();
        let previous = std::fs::read_to_string(dir.join(format!("{APP}.1"))).unwrap();
        assert!(previous.starts_with("line 0") && previous.contains("line 2"));
        assert!(current.starts_with("line 3") && current.contains("line 4"));
        for file in [APP.to_owned(), format!("{APP}.1")] {
            let mode = std::fs::metadata(dir.join(file))
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o777, MODE);
        }
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_deleted_log_is_recreated() {
        let dir = temp_dir("recreate");
        let mut logs = Logs::new(dir.clone(), MAX_SIZE);
        logs.write(HOTKEY, "first\n");
        std::fs::remove_file(dir.join(HOTKEY)).unwrap();
        for _ in 0..RECHECK_EVERY {
            logs.write(HOTKEY, "again\n");
        }
        assert!(
            std::fs::read_to_string(dir.join(HOTKEY))
                .unwrap()
                .contains("again")
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn an_existing_log_becomes_private() {
        let dir = temp_dir("private");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(AUDIO), "old\n").unwrap();
        std::fs::set_permissions(dir.join(AUDIO), Permissions::from_mode(0o644)).unwrap();
        Logs::new(dir.clone(), MAX_SIZE).write(AUDIO, "new\n");
        let mode = std::fs::metadata(dir.join(AUDIO))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, MODE);
        assert_eq!(
            std::fs::read_to_string(dir.join(AUDIO)).unwrap(),
            "old\nnew\n"
        );
        let _ = std::fs::remove_dir_all(dir);
    }
}
