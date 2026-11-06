//! User notifications for errors that would otherwise go unnoticed in a menu bar app.

use block2::RcBlock;
use objc2::runtime::Bool;
use objc2_foundation::{NSBundle, NSError, NSString, NSUUID};
use objc2_user_notifications::{
    UNAuthorizationOptions, UNMutableNotificationContent, UNNotificationRequest,
    UNNotificationSound, UNUserNotificationCenter,
};

use crate::util::filelog;

/// The notification center throws for processes without a bundle, e.g. under `cargo run`.
fn available() -> bool {
    NSBundle::mainBundle().bundleIdentifier().is_some()
}

pub fn request_permission() {
    if !available() {
        return;
    }
    let handler = RcBlock::new(|granted: Bool, _error: *mut NSError| {
        flog!(
            filelog::APP,
            "Notification permission {}",
            if granted.as_bool() {
                "granted"
            } else {
                "denied"
            }
        );
    });
    UNUserNotificationCenter::currentNotificationCenter()
        .requestAuthorizationWithOptions_completionHandler(
            UNAuthorizationOptions::Alert | UNAuthorizationOptions::Sound,
            &handler,
        );
}

pub fn send(title: &str, message: &str) {
    flog!(filelog::APP, "Notification: {title} — {message}");
    if !available() {
        return;
    }
    // macOS shows the app's icon with every notification, so the title names the event.
    let content = UNMutableNotificationContent::new();
    content.setTitle(&NSString::from_str(title));
    content.setBody(&NSString::from_str(message));
    content.setSound(Some(&UNNotificationSound::defaultSound()));
    let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
        &NSUUID::new().UUIDString(),
        &content,
        None,
    );
    UNUserNotificationCenter::currentNotificationCenter()
        .addNotificationRequest_withCompletionHandler(&request, None);
}
