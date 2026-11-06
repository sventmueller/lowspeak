//! Records fatal signals, panics and uncaught Objective-C exceptions in `crash.log`.
//! macOS additionally writes `.ips` reports to `~/Library/Logs/DiagnosticReports/`.

use std::ffi::{CString, c_int, c_void};
use std::sync::atomic::{AtomicI32, Ordering};

use objc2::rc::Retained;
use objc2_foundation::{NSException, NSSetUncaughtExceptionHandler};

use super::{filelog, paths};

static CRASH_FD: AtomicI32 = AtomicI32::new(-1);
/// Written at every launch; it separates the sessions in `crash.log`.
const INSTALLED: &str = "Crash reporter installed";

fn write_all(fd: c_int, mut bytes: &[u8]) {
    while !bytes.is_empty() {
        // SAFETY: write(2) is async-signal-safe; the buffer is valid for its length.
        let written = unsafe { libc::write(fd, bytes.as_ptr().cast(), bytes.len()) };
        match usize::try_from(written) {
            Ok(count) if count > 0 => bytes = &bytes[count..],
            _ => return,
        }
    }
}

/// Only async-signal-safe calls: no allocation, no locks.
extern "C" fn on_signal(signal: c_int) {
    let fd = CRASH_FD.load(Ordering::Relaxed);
    if fd >= 0 {
        let mut digits = [0u8; 12];
        let mut length = 0;
        let mut n = u32::try_from(signal).unwrap_or(0);
        loop {
            digits[digits.len() - 1 - length] = b'0' + (n % 10) as u8;
            length += 1;
            n /= 10;
            if n == 0 || length == digits.len() {
                break;
            }
        }
        write_all(fd, b"\nFATAL SIGNAL: ");
        write_all(fd, &digits[digits.len() - length..]);
        write_all(fd, b" (see .ips report for stack trace)\n");
    }
    // SAFETY: restoring the default action and re-raising lets macOS write its crash report.
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        libc::raise(signal);
    }
}

unsafe extern "C" fn on_exception(exception: *mut NSException) {
    // SAFETY: Foundation passes the uncaught exception, which stays alive while the handler runs.
    let Some(exception) = (unsafe { Retained::retain(exception) }) else {
        return;
    };
    let symbols: Vec<String> = exception
        .callStackSymbols()
        .iter()
        .map(|symbol| symbol.to_string())
        .collect();
    flog!(
        filelog::CRASH,
        "UNCAUGHT EXCEPTION: {}\nReason: {}\nStack: {}",
        exception.name(),
        exception
            .reason()
            .map(|reason| reason.to_string())
            .unwrap_or_else(|| "nil".into()),
        symbols.join("\n")
    );
    filelog::flush_all();
}

pub fn install() {
    let path = paths::log_dir().join(filelog::CRASH);
    if let Ok(path) = CString::new(path.to_string_lossy().as_bytes()) {
        // SAFETY: plain open(2) of a path we own.
        let fd = unsafe {
            libc::open(
                path.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_APPEND,
                0o600,
            )
        };
        CRASH_FD.store(fd, Ordering::Relaxed);
    }

    // SAFETY: the handler only uses async-signal-safe calls.
    unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = on_signal as extern "C" fn(c_int) as usize;
        action.sa_flags = libc::SA_RESETHAND;
        libc::sigemptyset(&mut action.sa_mask);
        for signal in [
            libc::SIGABRT,
            libc::SIGSEGV,
            libc::SIGBUS,
            libc::SIGFPE,
            libc::SIGILL,
            libc::SIGTRAP,
        ] {
            libc::sigaction(signal, &action, std::ptr::null_mut());
        }
    }

    // SAFETY: the handler has the signature Foundation expects.
    unsafe {
        NSSetUncaughtExceptionHandler(
            on_exception as unsafe extern "C" fn(*mut NSException) as *mut c_void,
        );
    }

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        flog!(
            filelog::CRASH,
            "PANIC: {info}\n{}",
            std::backtrace::Backtrace::force_capture()
        );
        filelog::flush_all();
        default_hook(info);
    }));

    flog!(filelog::CRASH, "{INSTALLED}");
}

pub fn check_previous_crash() {
    let Ok(log) = std::fs::read_to_string(paths::log_dir().join(filelog::CRASH)) else {
        return;
    };
    let crashes = crashes_in_previous_session(&log);
    if crashes > 0 {
        flog!(
            filelog::APP,
            "The previous session logged {crashes} crash(es) — see crash.log for details"
        );
    }
}

/// Crash entries between the last two launches; `install` has already marked the current one.
fn crashes_in_previous_session(log: &str) -> usize {
    let previous = log.rsplit(INSTALLED).nth(1).unwrap_or_default();
    ["FATAL SIGNAL", "UNCAUGHT EXCEPTION", "PANIC:"]
        .iter()
        .map(|marker| previous.matches(marker).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_session_before_this_launch_counts() {
        let log = format!(
            "{INSTALLED}\nFATAL SIGNAL: 11\n{INSTALLED}\nPANIC: a\nPANIC: b\n{INSTALLED}\n"
        );
        assert_eq!(crashes_in_previous_session(&log), 2);
        assert_eq!(
            crashes_in_previous_session(&format!("{log}{INSTALLED}\n")),
            0
        );
    }

    #[test]
    fn a_log_without_launch_markers_reports_nothing() {
        assert_eq!(crashes_in_previous_session(""), 0);
        assert_eq!(crashes_in_previous_session("FATAL SIGNAL: 11\n"), 0);
    }
}
