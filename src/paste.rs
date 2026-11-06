//! Types text into the frontmost app: clipboard plus a simulated ⌘V, then restores the clipboard
//! once the app has read the text.

use std::cell::RefCell;
use std::time::{Duration, Instant};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{AnyThread as _, DefinedClass as _, define_class, msg_send};
use objc2_app_kit::{
    NSPasteboard, NSPasteboardItem, NSPasteboardItemDataProvider, NSPasteboardType,
    NSPasteboardTypeString, NSPasteboardWriting,
};
use objc2_application_services::AXIsProcessTrusted;
use objc2_core_foundation::CFRetained;
use objc2_core_graphics::{CGEvent, CGEventFlags, CGEventTapLocation};
use objc2_foundation::{NSArray, NSData, NSObject, NSObjectProtocol, NSString};

use crate::util::{filelog, main_thread};

const V_KEYCODE: u16 = 0x09;
/// Asks clipboard history apps not to record the item (nspasteboard.org).
pub const TRANSIENT_TYPE: &str = "org.nspasteboard.TransientType";
/// After the first read, for apps that read the clipboard more than once or that a clipboard
/// manager beat to it.
pub const READ_GRACE: Duration = Duration::from_millis(500);
/// For an app that never reads the clipboard, e.g. because no text field had the focus.
pub const LONGEST_WAIT: Duration = Duration::from_secs(10);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Restore,
    /// Another app wrote to the clipboard meanwhile; its content wins.
    Leave,
    WaitUntil(Instant),
}

/// When the clipboard gets its previous content back: a little after the dictation was first read,
/// at the latest after `LONGEST_WAIT`.
#[derive(Clone, Copy, Debug)]
pub struct RestoreClock {
    posted: Instant,
    first_read: Option<Instant>,
}

impl RestoreClock {
    pub fn new(posted: Instant) -> Self {
        Self {
            posted,
            first_read: None,
        }
    }

    /// The pasteboard asks for promised data once and keeps it, so only the first read is seen.
    pub fn read(&mut self, at: Instant) {
        self.first_read.get_or_insert(at);
    }

    pub fn due(&self) -> Instant {
        let latest = self.posted + LONGEST_WAIT;
        self.first_read
            .map_or(latest, |read| (read + READ_GRACE).min(latest))
    }

    pub fn decide(&self, now: Instant, clipboard_changed: bool) -> Decision {
        if clipboard_changed {
            Decision::Leave
        } else if now >= self.due() {
            Decision::Restore
        } else {
            Decision::WaitUntil(self.due())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Provided {
    Read,
    /// The pasteboard dropped the item, because it was cleared or written over.
    Released,
}

pub struct ProviderIvars {
    text: String,
    on_event: Box<dyn Fn(Provided) + Send + Sync>,
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements and `TextProvider` does not implement Drop.
    #[unsafe(super = NSObject)]
    #[name = "LowspeakTextProvider"]
    #[ivars = ProviderIvars]
    pub struct TextProvider;

    unsafe impl NSObjectProtocol for TextProvider {}

    unsafe impl NSPasteboardItemDataProvider for TextProvider {
        #[unsafe(method(pasteboard:item:provideDataForType:))]
        fn provide(
            &self,
            _pasteboard: Option<&NSPasteboard>,
            item: &NSPasteboardItem,
            kind: &NSPasteboardType,
        ) {
            item.setString_forType(&NSString::from_str(&self.ivars().text), kind);
            (self.ivars().on_event)(Provided::Read);
        }

        #[unsafe(method(pasteboardFinishedWithDataProvider:))]
        fn finished(&self, _pasteboard: &NSPasteboard) {
            (self.ivars().on_event)(Provided::Released);
        }
    }
);

/// Keeps the provider of a dictation alive while the pasteboard may still ask it for the text.
pub struct Offer {
    _provider: Retained<TextProvider>,
}

/// Puts `text` on `pasteboard` as a promise kept on the first read, marked as transient.
pub fn offer(
    pasteboard: &NSPasteboard,
    text: &str,
    on_event: impl Fn(Provided) + Send + Sync + 'static,
) -> Option<Offer> {
    let provider = TextProvider::alloc().set_ivars(ProviderIvars {
        text: text.to_owned(),
        on_event: Box::new(on_event),
    });
    // SAFETY: NSObject's init.
    let provider: Retained<TextProvider> = unsafe { msg_send![super(provider), init] };
    let item = NSPasteboardItem::new();
    let transient = NSString::from_str(TRANSIENT_TYPE);
    // SAFETY: framework constant.
    let string_type = unsafe { NSPasteboardTypeString };
    let promised = item.setDataProvider_forTypes(
        ProtocolObject::from_ref(&*provider),
        &NSArray::from_slice(&[string_type]),
    );
    if !promised || !item.setData_forType(&NSData::new(), &transient) {
        return None;
    }
    pasteboard.clearContents();
    let items: [Retained<ProtocolObject<dyn NSPasteboardWriting>>; 1] =
        [ProtocolObject::from_retained(item)];
    pasteboard
        .writeObjects(&NSArray::from_retained_slice(&items))
        .then_some(Offer {
            _provider: provider,
        })
}

pub struct Saved(Vec<Vec<(String, Vec<u8>)>>);

pub fn save(pasteboard: &NSPasteboard) -> Saved {
    let Some(items) = pasteboard.pasteboardItems() else {
        return Saved(Vec::new());
    };
    Saved(
        items
            .iter()
            .map(|item| {
                item.types()
                    .iter()
                    .filter_map(|kind| Some((kind.to_string(), item.dataForType(&kind)?.to_vec())))
                    .collect()
            })
            .collect(),
    )
}

/// Puts the saved items back; an empty clipboard stays as it is now.
pub fn restore(pasteboard: &NSPasteboard, saved: &Saved) {
    if saved.0.is_empty() {
        return;
    }
    pasteboard.clearContents();
    let items: Vec<Retained<ProtocolObject<dyn NSPasteboardWriting>>> = saved
        .0
        .iter()
        .map(|entries| {
            let item = NSPasteboardItem::new();
            for (kind, bytes) in entries {
                item.setData_forType(&NSData::with_bytes(bytes), &NSString::from_str(kind));
            }
            ProtocolObject::from_retained(item)
        })
        .collect();
    pasteboard.writeObjects(&NSArray::from_retained_slice(&items));
}

/// Puts text on the clipboard for the user to paste.
pub fn copy(text: &str) {
    let pasteboard = NSPasteboard::generalPasteboard();
    pasteboard.clearContents();
    // SAFETY: framework constant.
    pasteboard.setString_forType(&NSString::from_str(text), unsafe { NSPasteboardTypeString });
}

pub enum Pasted {
    Yes,
    /// Without Accessibility permission the text is left on the clipboard for a manual ⌘V.
    NeedsAccessibility,
    /// The ⌘V keystroke could not be created; the text is left on the clipboard.
    Failed,
}

/// A paste whose clipboard is not restored yet.
struct Pending {
    paste: u64,
    saved: Saved,
    change_count: isize,
    clock: RestoreClock,
    restored: Box<dyn FnOnce()>,
}

thread_local! {
    static PENDING: RefCell<Option<Pending>> = const { RefCell::new(None) };
    /// Kept until the next paste, since the dictation may stay on the clipboard.
    static OFFER: RefCell<Option<Offer>> = const { RefCell::new(None) };
    static PASTES: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

fn command_v() -> Option<[CFRetained<CGEvent>; 2]> {
    let key = |key_down| {
        let event = CGEvent::new_keyboard_event(None, V_KEYCODE, key_down)?;
        CGEvent::set_flags(Some(&event), CGEventFlags::MaskCommand);
        Some(event)
    };
    Some([key(true)?, key(false)?])
}

/// Call on the main thread. `restored` runs there once the clipboard holds its previous content
/// again, and only for `Pasted::Yes`: a paste before that would save this text as the content to
/// restore.
pub fn paste(text: &str, restored: impl FnOnce() + 'static) -> Pasted {
    // SAFETY: plain permission query.
    if !unsafe { AXIsProcessTrusted() } {
        copy(text);
        flog!(
            filelog::APP,
            "Accessibility permission not granted — text left in clipboard"
        );
        return Pasted::NeedsAccessibility;
    }
    let Some(keys) = command_v() else {
        copy(text);
        flog!(
            filelog::APP,
            "Failed to create keyboard events — text left in clipboard"
        );
        return Pasted::Failed;
    };
    if let Some(earlier) = PENDING.take() {
        finish(earlier, Decision::Restore);
    }
    let pasteboard = NSPasteboard::generalPasteboard();
    let saved = save(&pasteboard);
    let id = PASTES.get() + 1;
    PASTES.set(id);
    let Some(offer) = offer(&pasteboard, text, move |event| {
        main_thread::run(move || provided(id, event));
    }) else {
        copy(text);
        flog!(
            filelog::APP,
            "The clipboard refused the dictation — text left in clipboard"
        );
        return Pasted::Failed;
    };
    OFFER.set(Some(offer));
    let change_count = pasteboard.changeCount();
    for key in &keys {
        CGEvent::post(CGEventTapLocation::HIDEventTap, Some(key));
    }
    let clock = RestoreClock::new(Instant::now());
    PENDING.set(Some(Pending {
        paste: id,
        saved,
        change_count,
        clock,
        restored: Box::new(restored),
    }));
    check_at(id, clock.due());
    flog!(filelog::APP, "Pasted {} characters", text.chars().count());
    Pasted::Yes
}

fn check_at(paste: u64, when: Instant) {
    let delay = when.saturating_duration_since(Instant::now());
    main_thread::after(delay, move || check(paste));
}

fn provided(paste: u64, event: Provided) {
    let due = PENDING.with_borrow_mut(|pending| {
        let pending = pending.as_mut().filter(|pending| pending.paste == paste)?;
        if event == Provided::Read {
            pending.clock.read(Instant::now());
            flog!(filelog::APP, "The dictation was read from the clipboard");
        }
        Some(pending.clock.due())
    });
    match (event, due) {
        (Provided::Read, Some(due)) => check_at(paste, due),
        (Provided::Released, Some(_)) => check(paste),
        (_, None) => {}
    }
}

fn check(paste: u64) {
    let Some(pending) = PENDING.take() else {
        return;
    };
    if pending.paste != paste {
        PENDING.set(Some(pending));
        return;
    }
    let changed = NSPasteboard::generalPasteboard().changeCount() != pending.change_count;
    match pending.clock.decide(Instant::now(), changed) {
        Decision::WaitUntil(when) => {
            PENDING.set(Some(pending));
            check_at(paste, when);
        }
        decision => finish(pending, decision),
    }
}

fn finish(pending: Pending, decision: Decision) {
    if decision == Decision::Restore {
        restore(&NSPasteboard::generalPasteboard(), &pending.saved);
    }
    flog!(
        filelog::APP,
        "Clipboard {} after {:.2}s",
        if decision == Decision::Restore {
            "restored"
        } else {
            "left to the app that wrote to it"
        },
        pending.clock.posted.elapsed().as_secs_f64()
    );
    (pending.restored)();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(start: Instant, millis: u64) -> Instant {
        start + Duration::from_millis(millis)
    }

    #[test]
    fn the_clipboard_is_restored_shortly_after_the_first_read() {
        let start = Instant::now();
        let mut clock = RestoreClock::new(start);
        clock.read(ms(start, 40));
        clock.read(ms(start, 300));
        assert_eq!(clock.due(), ms(start, 40) + READ_GRACE);
        assert_eq!(
            clock.decide(ms(start, 100), false),
            Decision::WaitUntil(ms(start, 40) + READ_GRACE)
        );
        assert_eq!(
            clock.decide(ms(start, 40) + READ_GRACE, false),
            Decision::Restore
        );
    }

    #[test]
    fn a_slow_reader_keeps_the_dictation_until_it_reads() {
        let start = Instant::now();
        let mut clock = RestoreClock::new(start);
        assert!(matches!(
            clock.decide(ms(start, 3_000), false),
            Decision::WaitUntil(_)
        ));
        clock.read(ms(start, 4_000));
        assert_eq!(
            clock.decide(ms(start, 4_100), false),
            Decision::WaitUntil(ms(start, 4_000) + READ_GRACE)
        );
    }

    #[test]
    fn an_app_that_never_reads_gets_the_clipboard_back_after_the_longest_wait() {
        let start = Instant::now();
        let mut clock = RestoreClock::new(start);
        assert_eq!(clock.due(), start + LONGEST_WAIT);
        assert_eq!(clock.decide(start + LONGEST_WAIT, false), Decision::Restore);
        clock.read(start + LONGEST_WAIT - Duration::from_millis(10));
        assert_eq!(clock.due(), start + LONGEST_WAIT);
    }

    #[test]
    fn content_another_app_wrote_is_left_alone() {
        let start = Instant::now();
        let mut clock = RestoreClock::new(start);
        assert_eq!(clock.decide(ms(start, 50), true), Decision::Leave);
        clock.read(ms(start, 10));
        assert_eq!(clock.decide(start + LONGEST_WAIT, true), Decision::Leave);
    }
}
