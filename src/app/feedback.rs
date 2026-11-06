//! The way from the Help menu to the project on GitHub.

use objc2_app_kit::NSWorkspace;
use objc2_foundation::{NSProcessInfo, NSString, NSURL};

use crate::util::{filelog, version};

pub const REPOSITORY: &str = "https://github.com/sventmueller/lowspeak";

pub fn open(address: &str) {
    match NSURL::URLWithString(&NSString::from_str(address)) {
        Some(url) => {
            NSWorkspace::sharedWorkspace().openURL(&url);
        }
        None => flog!(filelog::APP, "Cannot open {address}: not a URL"),
    }
}

pub fn report_problem() {
    let system = NSProcessInfo::processInfo().operatingSystemVersion();
    let macos = format!(
        "{}.{}.{}",
        system.majorVersion, system.minorVersion, system.patchVersion
    );
    open(&problem_address(&version::app_version(), &macos));
}

pub fn suggest_feature() {
    open(&format!("{REPOSITORY}/discussions/new?category=ideas"));
}

pub fn show_repository() {
    open(REPOSITORY);
}

/// GitHub fills the fields of `.github/ISSUE_TEMPLATE/bug.yml` from the query by their ids.
fn problem_address(version: &str, macos: &str) -> String {
    format!("{REPOSITORY}/issues/new?template=bug.yml&version={version}&macos={macos}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_problem_form_is_filled_in_by_the_ids_of_its_fields() {
        assert_eq!(
            problem_address("1.0.3", "26.1.0"),
            "https://github.com/sventmueller/lowspeak/issues/new?template=bug.yml&version=1.0.3&macos=26.1.0"
        );
        let form = include_str!("../../.github/ISSUE_TEMPLATE/bug.yml");
        for id in ["id: version", "id: macos"] {
            assert!(form.contains(id), "{id}");
        }
    }
}
