//! A host app's own log directory, set before the first log line; later attempts change nothing.

use lowspeak::util::{filelog, paths};

#[test]
fn a_host_puts_the_logs_into_its_own_directory_before_the_first_line() {
    let dir = std::env::temp_dir().join(format!("lowspeak-host-logs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    assert!(paths::use_log_dir(dir.clone()));
    lowspeak::flog!(filelog::APP, "written by the host");
    assert_eq!(paths::log_dir(), dir);
    assert!(dir.join(filelog::APP).exists());
    assert!(
        !paths::use_log_dir(std::env::temp_dir()),
        "the directory stays once chosen"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
