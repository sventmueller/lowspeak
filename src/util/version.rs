/// The version of `[workspace.package]` in Cargo.toml, which xtask also writes into the bundle's Info.plist.
pub fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_app_reports_the_workspace_version() {
        let manifest = include_str!("../../Cargo.toml");
        let declared = manifest
            .split("[workspace.package]")
            .nth(1)
            .and_then(|section| section.lines().find(|line| line.starts_with("version = ")))
            .and_then(|line| line.split('"').nth(1))
            .expect("Cargo.toml declares the version in [workspace.package]");
        assert_eq!(app_version(), declared);
        assert_ne!(app_version(), "0.0.0");
    }
}
