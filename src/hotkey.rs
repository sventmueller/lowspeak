//! Detects the fn (Globe) and Shift keys through a Quartz event tap on its own thread.

use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use block2::RcBlock;
use objc2_application_services::AXIsProcessTrusted;
use objc2_core_foundation::{
    CFAbsoluteTimeGetCurrent, CFMachPort, CFRetained, CFRunLoop, CFRunLoopRunResult,
    CFRunLoopTimer, kCFRunLoopCommonModes, kCFRunLoopDefaultMode,
};
use objc2_core_graphics::{
    CGEvent, CGEventField, CGEventFlags, CGEventTapLocation, CGEventTapOptions,
    CGEventTapPlacement, CGEventTapProxy, CGEventType,
};

use crate::util::filelog;

const FN_KEYCODE: i64 = 63;
const SHIFT_KEYCODES: [i64; 2] = [56, 60];
const WATCHDOG_INTERVAL: f64 = 30.0;
const MAX_TAP_RETRIES: u32 = 10;
const TAP_RETRY_DELAY: Duration = Duration::from_secs(2);
/// How long the tap thread runs its loop before it looks at `cancelled` again.
const RUN_SLICE: f64 = 1.0;

/// The event tap's port, shared with other threads to enable, check or invalidate it.
struct TapPort(CFRetained<CFMachPort>);
// SAFETY: the port is used only through CGEventTapEnable, CGEventTapIsEnabled, CFMachPortIsValid
// and CFMachPortInvalidate, which Quartz and Core Foundation allow on any thread, and retaining or
// releasing a Core Foundation object is thread-safe.
unsafe impl Send for TapPort {}
// SAFETY: as for Send; concurrent calls of those functions share no Rust-side state.
unsafe impl Sync for TapPort {}

/// The tap thread's run loop, shared with other threads only to stop it.
struct TapRunLoop(CFRetained<CFRunLoop>);
// SAFETY: CFRunLoopStop may be called from any thread, and it is the only call made through
// this type.
unsafe impl Send for TapRunLoop {}
// SAFETY: as for Send; the call takes no Rust-side state.
unsafe impl Sync for TapRunLoop {}

struct Installed {
    port: TapPort,
    run_loop: TapRunLoop,
}

struct Inner {
    fn_held: AtomicBool,
    tap: Mutex<Option<Installed>>,
    cancelled: AtomicBool,
    event_count: AtomicUsize,
    /// Lines for `hotkey.log`; written on another thread, since every modifier key press in the
    /// system waits for the tap's callback.
    log: Sender<String>,
    on_key: Box<dyn Fn(Key) + Send + Sync>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    FnDown { shift: bool },
    FnUp,
    ShiftDown,
}

pub struct Hotkey {
    inner: Arc<Inner>,
}

impl Inner {
    fn log(&self, message: impl Into<String>) {
        let _ = self.log.send(message.into());
    }
}

fn spawn_logger() -> Sender<String> {
    let (sender, lines) = mpsc::channel::<String>();
    let spawned = std::thread::Builder::new()
        .name("lowspeak.hotkey-log".into())
        .spawn(move || {
            for line in lines {
                filelog::write(filelog::HOTKEY, &line);
            }
        });
    if let Err(error) = spawned {
        filelog::write(
            filelog::HOTKEY,
            &format!("[Hotkey] No log thread, hotkey events go unlogged: {error}"),
        );
    }
    sender
}

/// What a flagsChanged event means for dictation, given whether fn was down before it.
fn key_event(keycode: i64, flags: CGEventFlags, fn_was_held: bool) -> Option<Key> {
    if SHIFT_KEYCODES.contains(&keycode) {
        return flags
            .contains(CGEventFlags::MaskShift)
            .then_some(Key::ShiftDown);
    }
    if keycode != FN_KEYCODE {
        return None;
    }
    match (fn_was_held, flags.contains(CGEventFlags::MaskSecondaryFn)) {
        (false, true) => Some(Key::FnDown {
            shift: flags.contains(CGEventFlags::MaskShift),
        }),
        (true, false) => Some(Key::FnUp),
        _ => None,
    }
}

unsafe extern "C-unwind" fn callback(
    _proxy: CGEventTapProxy,
    event_type: CGEventType,
    event: NonNull<CGEvent>,
    user_info: *mut c_void,
) -> *mut CGEvent {
    // SAFETY: `user_info` is the `Inner` the tap was created with; the tap thread holds an `Arc`
    // of it until its run loop, which delivers these calls, has ended.
    let inner = unsafe { &*user_info.cast::<Inner>() };
    // SAFETY: Quartz passes a valid event for the duration of the callback.
    let event_ref = unsafe { event.as_ref() };
    if event_type == CGEventType::TapDisabledByTimeout
        || event_type == CGEventType::TapDisabledByUserInput
    {
        if let Ok(tap) = inner.tap.lock()
            && let Some(tap) = tap.as_ref()
        {
            inner.log("[Hotkey] Event tap was disabled by macOS — re-enabling");
            CGEvent::tap_enable(&tap.port.0, true);
        }
        return event.as_ptr();
    }

    let keycode = CGEvent::integer_value_field(Some(event_ref), CGEventField::KeyboardEventKeycode);
    let flags = CGEvent::flags(Some(event_ref));
    if inner.event_count.fetch_add(1, Ordering::Relaxed) < 5 {
        inner.log(format!(
            "[Hotkey] type={} keycode={keycode} flags=0x{:x}",
            event_type.0, flags.0
        ));
    }
    if event_type != CGEventType::FlagsChanged {
        return event.as_ptr();
    }
    let fn_was_held = keycode == FN_KEYCODE
        && inner.fn_held.swap(
            flags.contains(CGEventFlags::MaskSecondaryFn),
            Ordering::AcqRel,
        );
    match key_event(keycode, flags, fn_was_held) {
        Some(Key::ShiftDown) => {
            (inner.on_key)(Key::ShiftDown);
            event.as_ptr()
        }
        Some(key) => {
            inner.log(match key {
                Key::FnDown { shift: true } => "[Hotkey] fn DOWN with Shift",
                Key::FnDown { shift: false } => "[Hotkey] fn DOWN",
                Key::FnUp | Key::ShiftDown => "[Hotkey] fn UP",
            });
            (inner.on_key)(key);
            std::ptr::null_mut()
        }
        None => event.as_ptr(),
    }
}

/// Sleeps in short steps so `stop` does not wait for a whole retry delay.
fn sleep_unless_cancelled(inner: &Inner, duration: Duration) {
    let step = Duration::from_millis(500);
    let mut remaining = duration;
    while !remaining.is_zero() && !inner.cancelled.load(Ordering::Acquire) {
        let nap = remaining.min(step);
        std::thread::sleep(nap);
        remaining -= nap;
    }
}

fn create_tap(inner: &Arc<Inner>) -> Option<CFRetained<CFMachPort>> {
    // Only flagsChanged: adding keyDown/keyUp makes macOS stop delivering fn events.
    let mask = 1u64 << CGEventType::FlagsChanged.0;
    // SAFETY: the callback matches CGEventTapCallBack and `inner` outlives the tap (see `user_info`
    // in `callback`).
    unsafe {
        CGEvent::tap_create(
            CGEventTapLocation::SessionEventTap,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::Default,
            mask,
            Some(callback),
            Arc::as_ptr(inner).cast_mut().cast(),
        )
    }
}

/// Accessibility permission may be granted only after launch, so this keeps retrying.
fn create_tap_until_cancelled(inner: &Arc<Inner>) -> Option<CFRetained<CFMachPort>> {
    loop {
        for attempt in 1..=MAX_TAP_RETRIES {
            if inner.cancelled.load(Ordering::Acquire) {
                inner.log("[Hotkey] Cancelled, aborting tap creation");
                return None;
            }
            if let Some(tap) = create_tap(inner) {
                inner.log(format!("[Hotkey] Event tap created on attempt {attempt}"));
                return Some(tap);
            }
            // SAFETY: plain permission query.
            let trusted = unsafe { AXIsProcessTrusted() };
            inner.log(format!(
                "[Hotkey] Attempt {attempt}/{MAX_TAP_RETRIES} failed, AXTrusted={trusted}"
            ));
            sleep_unless_cancelled(inner, TAP_RETRY_DELAY);
        }
        inner.log("[Hotkey] All attempts failed — retrying in 10s");
        sleep_unless_cancelled(inner, Duration::from_secs(10));
    }
}

/// Publishes the tap for `stop`, unless `stop` already ran: it sets `cancelled` before it takes the
/// tap, so checking under the same lock leaves no gap between the two.
fn install(inner: &Inner, tap: &CFRetained<CFMachPort>, run_loop: &CFRetained<CFRunLoop>) -> bool {
    let Ok(mut slot) = inner.tap.lock() else {
        return false;
    };
    if inner.cancelled.load(Ordering::Acquire) {
        return false;
    }
    *slot = Some(Installed {
        port: TapPort(tap.clone()),
        run_loop: TapRunLoop(run_loop.clone()),
    });
    true
}

fn run_tap_loop(inner: Arc<Inner>) {
    // SAFETY: plain permission query.
    inner.log(format!("[Hotkey] AXIsProcessTrusted: {}", unsafe {
        AXIsProcessTrusted()
    }));
    let Some(tap) = create_tap_until_cancelled(&inner) else {
        return;
    };
    let Some(run_loop) = CFRunLoop::current() else {
        inner.log("[Hotkey] No run loop on the tap thread");
        tap.invalidate();
        return;
    };
    if !install(&inner, &tap, &run_loop) {
        inner.log("[Hotkey] Stopped before the tap was installed");
        tap.invalidate();
        return;
    }
    // SAFETY: framework constant.
    let common_modes = unsafe { kCFRunLoopCommonModes };
    let source = CFMachPort::new_run_loop_source(None, Some(&tap), 0);
    run_loop.add_source(source.as_deref(), common_modes);
    CGEvent::tap_enable(&tap, true);
    inner.log(format!(
        "[Hotkey] Tap enabled: {}, valid: {}",
        CGEvent::tap_is_enabled(&tap),
        tap.is_valid()
    ));

    let watched = Arc::clone(&inner);
    let watchdog = RcBlock::new(move |_timer: *mut CFRunLoopTimer| {
        let Ok(tap) = watched.tap.lock() else { return };
        if let Some(tap) = tap.as_ref()
            && tap.port.0.is_valid()
            && !CGEvent::tap_is_enabled(&tap.port.0)
        {
            watched.log("[Hotkey] Watchdog: event tap was disabled by macOS — re-enabling");
            CGEvent::tap_enable(&tap.port.0, true);
        }
    });
    // SAFETY: repeating timer with a block that only touches the shared tap.
    let timer = unsafe {
        CFRunLoopTimer::with_handler(
            None,
            CFAbsoluteTimeGetCurrent() + WATCHDOG_INTERVAL,
            WATCHDOG_INTERVAL,
            0,
            0,
            Some(&watchdog),
        )
    };
    run_loop.add_timer(timer.as_deref(), common_modes);

    inner.log("[Hotkey] Starting CFRunLoop...");
    // SAFETY: framework constant.
    let default_mode = unsafe { kCFRunLoopDefaultMode };
    // A `stop` between `install` and here finds no running loop to stop, hence the slices.
    while !inner.cancelled.load(Ordering::Acquire) {
        if CFRunLoop::run_in_mode(default_mode, RUN_SLICE, false) == CFRunLoopRunResult::Finished {
            break;
        }
    }
    if let Some(timer) = timer {
        timer.invalidate();
    }
    inner.log("[Hotkey] CFRunLoop exited");
}

impl Hotkey {
    pub fn start(on_key: impl Fn(Key) + Send + Sync + 'static) -> Self {
        let inner = Arc::new(Inner {
            fn_held: AtomicBool::new(false),
            tap: Mutex::new(None),
            cancelled: AtomicBool::new(false),
            event_count: AtomicUsize::new(0),
            log: spawn_logger(),
            on_key: Box::new(on_key),
        });
        let thread_inner = Arc::clone(&inner);
        std::thread::Builder::new()
            .name("lowspeak.eventtap".into())
            .spawn(move || run_tap_loop(thread_inner))
            .expect("spawning the event tap thread");
        Self { inner }
    }

    /// Forgets a held fn key, for when the app rejected the recording it started.
    pub fn reset_fn_state(&self) {
        self.inner.fn_held.store(false, Ordering::Release);
    }

    pub fn stop(&self) {
        self.inner.cancelled.store(true, Ordering::Release);
        let installed = self.inner.tap.lock().ok().and_then(|mut slot| slot.take());
        if let Some(installed) = installed {
            CGEvent::tap_enable(&installed.port.0, false);
            installed.port.0.invalidate();
            installed.run_loop.0.stop();
        }
        self.reset_fn_state();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEFT_SHIFT: i64 = 56;
    const RIGHT_SHIFT: i64 = 60;
    const COMMAND: i64 = 55;

    #[test]
    fn fn_is_reported_on_its_edges_only() {
        let fn_flag = CGEventFlags::MaskSecondaryFn;
        assert_eq!(
            key_event(FN_KEYCODE, fn_flag, false),
            Some(Key::FnDown { shift: false })
        );
        assert_eq!(key_event(FN_KEYCODE, fn_flag, true), None);
        assert_eq!(
            key_event(FN_KEYCODE, CGEventFlags::empty(), true),
            Some(Key::FnUp)
        );
        assert_eq!(key_event(FN_KEYCODE, CGEventFlags::empty(), false), None);
    }

    #[test]
    fn fn_pressed_with_shift_held_says_so() {
        let flags = CGEventFlags::MaskSecondaryFn | CGEventFlags::MaskShift;
        assert_eq!(
            key_event(FN_KEYCODE, flags, false),
            Some(Key::FnDown { shift: true })
        );
    }

    #[test]
    fn shift_is_reported_when_pressed_not_when_released() {
        for keycode in [LEFT_SHIFT, RIGHT_SHIFT] {
            assert_eq!(
                key_event(keycode, CGEventFlags::MaskShift, false),
                Some(Key::ShiftDown)
            );
            assert_eq!(
                key_event(
                    keycode,
                    CGEventFlags::MaskShift | CGEventFlags::MaskSecondaryFn,
                    true
                ),
                Some(Key::ShiftDown)
            );
            assert_eq!(key_event(keycode, CGEventFlags::empty(), true), None);
        }
    }

    #[test]
    fn other_modifiers_are_ignored_even_with_fn_flags() {
        assert_eq!(
            key_event(
                COMMAND,
                CGEventFlags::MaskCommand | CGEventFlags::MaskSecondaryFn,
                false
            ),
            None
        );
    }
}
