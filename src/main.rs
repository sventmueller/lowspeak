use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSWorkspace,
};
use objc2_application_services::{
    AXIsProcessTrusted, AXIsProcessTrustedWithOptions, kAXTrustedCheckOptionPrompt,
};
use objc2_core_foundation::CFDictionary;
use objc2_foundation::{
    MainThreadMarker, NSBundle, NSDictionary, NSNotification, NSNumber, NSObject, NSObjectProtocol,
    NSString,
};

use lowspeak::app;
use lowspeak::util::{crash, filelog, version};

define_class!(
    // SAFETY: NSObject has no subclassing requirements and `Delegate` does not implement Drop.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "LowspeakAppDelegate"]
    struct Delegate;

    unsafe impl NSObjectProtocol for Delegate {}

    unsafe impl NSApplicationDelegate for Delegate {
        #[unsafe(method(applicationWillTerminate:))]
        fn will_terminate(&self, _notification: &NSNotification) {
            app::shutdown();
        }
    }
);

impl Delegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: NSObject's init.
        unsafe { msg_send![super(this), init] }
    }
}

fn another_instance_running() -> bool {
    let Some(bundle_id) = NSBundle::mainBundle().bundleIdentifier() else {
        return false;
    };
    let own_pid = std::process::id();
    NSWorkspace::sharedWorkspace()
        .runningApplications()
        .iter()
        .any(|running| {
            u32::try_from(running.processIdentifier()).ok() != Some(own_pid)
                && running.bundleIdentifier().is_some_and(|id| id == bundle_id)
        })
}

/// Shows the system prompt that leads to the Accessibility settings.
fn prompt_for_accessibility() {
    // SAFETY: plain permission query.
    if unsafe { AXIsProcessTrusted() } {
        return;
    }
    // SAFETY: framework constant; CFString and NSString are toll-free bridged.
    let key: &NSString =
        unsafe { &*(std::ptr::from_ref(kAXTrustedCheckOptionPrompt).cast::<NSString>()) };
    let options = NSDictionary::from_slices(&[key], &[&*NSNumber::new_bool(true)]);
    // SAFETY: NSDictionary and CFDictionary are toll-free bridged.
    let options: &CFDictionary = unsafe { &*(Retained::as_ptr(&options).cast::<CFDictionary>()) };
    // SAFETY: the options dictionary has the documented key and a boolean value.
    unsafe { AXIsProcessTrustedWithOptions(Some(options)) };
}

fn main() {
    let mtm = MainThreadMarker::new().expect("Lowspeak must start on the main thread");
    crash::install();
    crash::check_previous_crash();

    let application = NSApplication::sharedApplication(mtm);
    application.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
    if another_instance_running() {
        filelog::write(filelog::APP, "Another instance already running — quitting");
        return;
    }

    let delegate = Delegate::new(mtm);
    application.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
    app::launch(mtm);
    prompt_for_accessibility();
    filelog::write(
        filelog::APP,
        &format!("Lowspeak {} launched", version::app_version()),
    );
    application.run();
}
