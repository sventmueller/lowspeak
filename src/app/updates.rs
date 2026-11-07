//! Updates through Sparkle, which only the distributed disk image carries. Sparkle asks on the second launch whether
//! it may check on its own; until then, and in builds without it, Lowspeak makes no request for updates.
//!
//! Away from launch, Sparkle shows the update a scheduled check found behind the other apps' windows, since a menu
//! bar app is in the background; the dot on the status item and the badge in the menu point to it until the user
//! turns to it.

use std::cell::{Cell, OnceCell};

use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, Bool};
use objc2::{MainThreadOnly, define_class, msg_send};
use objc2_foundation::{MainThreadMarker, NSBundle, NSObject, NSObjectProtocol, ns_string};

/// Sparkle's settings as the menu shows them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpdateMenu {
    /// False while a check runs.
    pub can_check: bool,
    pub checks_automatically: bool,
    pub installs_automatically: bool,
    /// Sparkle downloads and installs on its own only with automatic checks.
    pub can_install_automatically: bool,
    /// A scheduled check found an update the user has not turned to yet.
    pub waiting: bool,
}

struct Updater {
    controller: Retained<AnyObject>,
    /// Sparkle holds its delegates weakly.
    _reminder: Retained<Reminder>,
}

thread_local! {
    static UPDATER: OnceCell<Option<Updater>> = const { OnceCell::new() };
    static WAITING: Cell<bool> = const { Cell::new(false) };
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements and `Reminder` does not implement Drop.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "LowspeakUpdateReminder"]
    struct Reminder;

    unsafe impl NSObjectProtocol for Reminder {}

    /// Gentle reminders of `SPUStandardUserDriverDelegate`, whose methods Sparkle looks up one by one.
    impl Reminder {
        #[unsafe(method(supportsGentleScheduledUpdateReminders))]
        fn supports_gentle_reminders(&self) -> Bool {
            Bool::YES
        }

        #[unsafe(method(standardUserDriverWillHandleShowingUpdate:forUpdate:state:))]
        fn will_show_update(&self, _handled: Bool, _update: &AnyObject, state: &AnyObject) {
            // SAFETY: SPUUserUpdateState's BOOL property.
            let user_initiated: bool = unsafe { msg_send![state, userInitiated] };
            if !user_initiated {
                set_waiting(true);
            }
        }

        #[unsafe(method(standardUserDriverDidReceiveUserAttentionForUpdate:))]
        fn did_receive_attention(&self, _update: &AnyObject) {
            set_waiting(false);
        }

        #[unsafe(method(standardUserDriverWillFinishUpdateSession))]
        fn will_finish_session(&self) {
            set_waiting(false);
        }
    }
);

impl Reminder {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: NSObject's init.
        unsafe { msg_send![super(this), init] }
    }
}

fn set_waiting(waiting: bool) {
    if WAITING.replace(waiting) != waiting {
        super::dispatch(move |app| app.ui.show_update(waiting));
    }
}

/// Loads Sparkle from the app's Frameworks folder and starts the updater, which runs the checks the user allowed.
/// Without Sparkle, as in local builds, nothing happens.
pub fn start(mtm: MainThreadMarker) {
    UPDATER.with(|cell| {
        cell.get_or_init(|| load(mtm));
    });
}

fn load(mtm: MainThreadMarker) -> Option<Updater> {
    let url = NSBundle::mainBundle()
        .privateFrameworksURL()?
        .URLByAppendingPathComponent(ns_string!("Sparkle.framework"))?;
    let bundle = NSBundle::bundleWithURL(&url)?;
    // SAFETY: loading runs no initializers that need the app. The hardened runtime refuses the framework unless it
    // carries the app's Team ID, which only the Developer ID signature gives it.
    if !unsafe { bundle.load() } {
        return None;
    }
    let class = AnyClass::get(c"SPUStandardUpdaterController")?;
    let reminder = Reminder::new(mtm);
    // SAFETY: Sparkle's documented initializer; the updater delegate is optional.
    let controller: Option<Retained<AnyObject>> = unsafe {
        let allocated: Allocated<AnyObject> = msg_send![class, alloc];
        msg_send![
            allocated,
            initWithStartingUpdater: true,
            updaterDelegate: None::<&AnyObject>,
            userDriverDelegate: Some(&*reminder)
        ]
    };
    Some(Updater {
        controller: controller?,
        _reminder: reminder,
    })
}

fn controller() -> Option<Retained<AnyObject>> {
    UPDATER.with(|cell| {
        cell.get()
            .and_then(Option::as_ref)
            .map(|updater| updater.controller.clone())
    })
}

fn updater() -> Option<Retained<AnyObject>> {
    // SAFETY: SPUStandardUpdaterController's `updater` property, never nil once initialized.
    controller().map(|controller| unsafe { msg_send![&*controller, updater] })
}

/// What the menu shows about updates, or `None` without Sparkle.
pub fn menu() -> Option<UpdateMenu> {
    let updater = updater()?;
    // SAFETY: SPUUpdater's BOOL properties, read on the main thread as Sparkle requires.
    unsafe {
        Some(UpdateMenu {
            can_check: msg_send![&*updater, canCheckForUpdates],
            checks_automatically: msg_send![&*updater, automaticallyChecksForUpdates],
            installs_automatically: msg_send![&*updater, automaticallyDownloadsUpdates],
            can_install_automatically: msg_send![&*updater, allowsAutomaticUpdates],
            waiting: WAITING.get(),
        })
    }
}

/// Looks for an update and shows what it finds, or brings the update a scheduled check found to the front.
pub fn check() {
    if let Some(controller) = controller() {
        // SAFETY: SPUStandardUpdaterController's action method; the sender is unused.
        let () = unsafe { msg_send![&*controller, checkForUpdates: None::<&AnyObject>] };
    }
}

/// Sparkle keeps the choice in the app's user defaults.
pub fn toggle_automatic_checks() {
    if let Some(updater) = updater() {
        // SAFETY: SPUUpdater's BOOL property, read and set on the main thread as Sparkle requires.
        unsafe {
            let on: bool = msg_send![&*updater, automaticallyChecksForUpdates];
            let () = msg_send![&*updater, setAutomaticallyChecksForUpdates: !on];
        }
    }
}

pub fn toggle_automatic_installs() {
    if let Some(updater) = updater() {
        // SAFETY: SPUUpdater's BOOL property, read and set on the main thread as Sparkle requires.
        unsafe {
            let on: bool = msg_send![&*updater, automaticallyDownloadsUpdates];
            let () = msg_send![&*updater, setAutomaticallyDownloadsUpdates: !on];
        }
    }
}
