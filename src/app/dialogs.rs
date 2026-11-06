use objc2::MainThreadOnly as _;
use objc2_app_kit::{
    NSAlert, NSAlertFirstButtonReturn, NSAlertStyle, NSAlertThirdButtonReturn, NSSecureTextField,
    NSTextField,
};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString};

use super::bring_to_front;
use crate::i18n::Texts;
use crate::models::ApiProvider;

/// Asks for an API key. `Some("")` means "remove all keys", `None` means cancelled.
pub fn prompt_api_key(mtm: MainThreadMarker, has_key: bool, t: &Texts) -> Option<String> {
    loop {
        let alert = NSAlert::new(mtm);
        alert.setMessageText(&NSString::from_str(if has_key {
            t.update_api_key_title
        } else {
            t.set_api_key_title
        }));
        alert.setInformativeText(&NSString::from_str(t.api_key_info));
        alert.setAlertStyle(NSAlertStyle::Informational);
        alert.addButtonWithTitle(&NSString::from_str(t.save));
        alert.addButtonWithTitle(&NSString::from_str(t.cancel));
        if has_key {
            alert
                .addButtonWithTitle(&NSString::from_str(t.remove_all_keys))
                .setHasDestructiveAction(true);
        }
        let input = NSSecureTextField::initWithFrame(
            NSSecureTextField::alloc(mtm),
            NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(300.0, 24.0)),
        );
        input.setPlaceholderString(Some(&NSString::from_str(t.api_key_placeholder)));
        alert.setAccessoryView(Some(&input));
        alert.window().setInitialFirstResponder(Some(&input));

        bring_to_front(mtm);
        let response = alert.runModal();
        if response == NSAlertFirstButtonReturn {
            let key = input.stringValue().to_string().trim().to_owned();
            if key.is_empty() {
                return None;
            }
            if ApiProvider::from_api_key(&key).is_some() {
                return Some(key);
            }
            let warning = NSAlert::new(mtm);
            warning.setMessageText(&NSString::from_str(t.unknown_key_format));
            warning.setInformativeText(&NSString::from_str(t.unknown_key_format_info));
            warning.setAlertStyle(NSAlertStyle::Warning);
            warning.addButtonWithTitle(&NSString::from_str(t.ok));
            warning.runModal();
        } else if response == NSAlertThirdButtonReturn {
            return Some(String::new());
        } else {
            return None;
        }
    }
}

/// Asks for the vocabulary as a comma-separated list; `None` means cancelled.
pub fn prompt_vocabulary(mtm: MainThreadMarker, current: &str, t: &Texts) -> Option<String> {
    let alert = NSAlert::new(mtm);
    alert.setMessageText(&NSString::from_str(t.vocabulary_title));
    alert.setInformativeText(&NSString::from_str(t.vocabulary_info));
    alert.setAlertStyle(NSAlertStyle::Informational);
    alert.addButtonWithTitle(&NSString::from_str(t.save));
    alert.addButtonWithTitle(&NSString::from_str(t.cancel));
    let input = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(380.0, 24.0)),
    );
    input.setStringValue(&NSString::from_str(current));
    input.setPlaceholderString(Some(&NSString::from_str("Postgres, Traefik, Kubernetes")));
    alert.setAccessoryView(Some(&input));
    alert.window().setInitialFirstResponder(Some(&input));

    bring_to_front(mtm);
    (alert.runModal() == NSAlertFirstButtonReturn).then(|| input.stringValue().to_string())
}
