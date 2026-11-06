use objc2_foundation::{NSBundle, NSString, ns_string};

/// The version from the bundle's Info.plist, which `cargo xtask bundle` takes from VERSION; "dev" outside a bundle.
pub fn app_version() -> String {
    NSBundle::mainBundle()
        .objectForInfoDictionaryKey(ns_string!("CFBundleShortVersionString"))
        .and_then(|value| value.downcast::<NSString>().ok())
        .map_or_else(|| "dev".to_owned(), |value| value.to_string())
}
