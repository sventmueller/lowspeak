//! The About window: macOS's standard panel with the project's address and its license.

use objc2::AnyThread as _;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{
    NSAboutPanelOptionCredits, NSAboutPanelOptionVersion, NSApplication, NSFont,
    NSFontAttributeName, NSLinkAttributeName, NSMutableParagraphStyle,
    NSParagraphStyleAttributeName, NSTextAlignment,
};
use objc2_foundation::{
    MainThreadMarker, NSAttributedString, NSDictionary, NSMutableAttributedString, NSRange,
    NSString, NSURL,
};

use super::feedback::REPOSITORY;
use crate::i18n::Texts;

pub fn show(mtm: MainThreadMarker, t: &Texts) {
    // The version ends in the build number already, which the panel would repeat in parentheses.
    let version = NSString::new();
    let credits = credits(t);
    // SAFETY: framework constants.
    let keys = unsafe { [NSAboutPanelOptionVersion, NSAboutPanelOptionCredits] };
    let values: [&AnyObject; 2] = [&version, &credits];
    let options = NSDictionary::from_slices(&keys, &values);
    super::bring_to_front(mtm);
    // SAFETY: the panel takes a string for the version and an attributed string for the credits.
    unsafe {
        NSApplication::sharedApplication(mtm).orderFrontStandardAboutPanelWithOptions(&options);
    };
}

/// The repository's address and the license, each license name linked to its text.
fn credits(t: &Texts) -> Retained<NSAttributedString> {
    let address = REPOSITORY.trim_start_matches("https://");
    let shown = format!("{address}\n{}", t.license);
    let centered = NSMutableParagraphStyle::new();
    centered.setAlignment(NSTextAlignment::Center);
    let font = NSFont::systemFontOfSize(NSFont::smallSystemFontSize());
    // SAFETY: framework constants.
    let keys = unsafe { [NSFontAttributeName, NSParagraphStyleAttributeName] };
    let values: [&AnyObject; 2] = [&font, &centered];
    let attributes = NSDictionary::from_slices(&keys, &values);
    // SAFETY: the attributes map keys to values of the documented types.
    let credits = unsafe {
        NSMutableAttributedString::initWithString_attributes(
            NSMutableAttributedString::alloc(),
            &NSString::from_str(&shown),
            Some(&attributes),
        )
    };
    let links = [
        (address, REPOSITORY.to_owned()),
        ("MIT", format!("{REPOSITORY}/blob/main/LICENSE-MIT")),
        (
            "Apache 2.0",
            format!("{REPOSITORY}/blob/main/LICENSE-APACHE"),
        ),
    ];
    for (name, target) in links {
        let (Some(start), Some(url)) = (
            shown.find(name),
            NSURL::URLWithString(&NSString::from_str(&target)),
        ) else {
            continue;
        };
        let range = NSRange::new(
            shown[..start].encode_utf16().count(),
            name.encode_utf16().count(),
        );
        // SAFETY: a link attribute takes an NSURL; the range lies within the string.
        unsafe { credits.addAttribute_value_range(NSLinkAttributeName, &url, range) };
    }
    Retained::into_super(credits)
}
