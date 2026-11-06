//! Help → Quick Reference: the fn gestures in a small window.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSBackingStoreType, NSButton, NSColor, NSFont, NSGridView, NSLayoutAttribute, NSPanel,
    NSStackView, NSTextField, NSUserInterfaceLayoutOrientation, NSView, NSWindowStyleMask,
};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSEdgeInsets, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    NSString,
};

use super::feedback;
use crate::i18n::Texts;

const KEYBOARD_SETTINGS: &str = "x-apple.systempreferences:com.apple.Keyboard-Settings.extension";
const MIN_TEXT_WIDTH: f64 = 320.0;

define_class!(
    // SAFETY: NSObject has no subclassing requirements and `ReferenceTarget` does not implement Drop.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "LowspeakReferenceTarget"]
    struct ReferenceTarget;

    unsafe impl NSObjectProtocol for ReferenceTarget {}

    impl ReferenceTarget {
        #[unsafe(method(openKeyboardSettings:))]
        fn open_keyboard_settings(&self, _sender: Option<&AnyObject>) {
            feedback::open(KEYBOARD_SETTINGS);
        }
    }
);

impl ReferenceTarget {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: NSObject's init.
        unsafe { msg_send![super(this), init] }
    }
}

thread_local! {
    /// The button holds its target weakly, so the target lives as long as the window.
    static WINDOW: RefCell<Option<(Retained<NSPanel>, Retained<ReferenceTarget>)>> =
        const { RefCell::new(None) };
}

/// Opens the window anew, so it follows a change of the interface language.
pub fn show(mtm: MainThreadMarker, t: &Texts) {
    let previous = WINDOW.with_borrow_mut(Option::take);
    if let Some((panel, _)) = previous {
        panel.close();
    }
    let target = ReferenceTarget::new(mtm);
    let panel = window(mtm, t, &target);
    super::bring_to_front(mtm);
    panel.makeKeyAndOrderFront(None);
    WINDOW.with_borrow_mut(|window| *window = Some((panel, target)));
}

fn window(mtm: MainThreadMarker, t: &Texts, target: &ReferenceTarget) -> Retained<NSPanel> {
    let gestures = [
        ("fn", t.reference_hold),
        ("fn fn", t.reference_double_tap),
        ("⇧ fn", t.reference_shift),
        ("fn / ⇧", t.reference_stop),
    ];
    let rows: Vec<Retained<NSArray<NSView>>> = gestures
        .iter()
        .map(|(keys, gesture)| {
            let keys = NSTextField::labelWithString(&NSString::from_str(keys), mtm);
            keys.setFont(Some(
                &NSFont::boldSystemFontOfSize(NSFont::systemFontSize()),
            ));
            let gesture = NSTextField::labelWithString(&NSString::from_str(gesture), mtm);
            let row: [&NSView; 2] = [&keys, &gesture];
            NSArray::from_slice(&row)
        })
        .collect();
    let rows: Vec<&NSArray<NSView>> = rows.iter().map(|row| &**row).collect();
    let grid = NSGridView::gridViewWithViews(&NSArray::from_slice(&rows), mtm);
    grid.setRowSpacing(8.0);
    grid.setColumnSpacing(16.0);

    let width = grid.fittingSize().width.max(MIN_TEXT_WIDTH);
    let tip = NSTextField::wrappingLabelWithString(&NSString::from_str(t.reference_emoji), mtm);
    // A stack view lets a wrapping label grow past its preferred width, which then runs past the window's edge.
    tip.setPreferredMaxLayoutWidth(width);
    tip.widthAnchor()
        .constraintEqualToConstant(width)
        .setActive(true);
    tip.setTextColor(Some(&NSColor::secondaryLabelColor()));
    // SAFETY: `ReferenceTarget` implements the action; WINDOW keeps the target alive with the window.
    let button = unsafe {
        NSButton::buttonWithTitle_target_action(
            &NSString::from_str(t.keyboard_settings),
            Some(target),
            Some(sel!(openKeyboardSettings:)),
            mtm,
        )
    };

    let views: [&NSView; 3] = [&grid, &tip, &button];
    let stack = NSStackView::stackViewWithViews(&NSArray::from_slice(&views), mtm);
    stack.setOrientation(NSUserInterfaceLayoutOrientation::Vertical);
    stack.setAlignment(NSLayoutAttribute::Leading);
    stack.setSpacing(16.0);
    stack.setEdgeInsets(NSEdgeInsets {
        top: 20.0,
        left: 20.0,
        bottom: 20.0,
        right: 20.0,
    });

    let panel = NSPanel::initWithContentRect_styleMask_backing_defer(
        NSPanel::alloc(mtm),
        NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(width + 40.0, 200.0)),
        NSWindowStyleMask::Titled | NSWindowStyleMask::Closable,
        NSBackingStoreType::Buffered,
        false,
    );
    // SAFETY: WINDOW owns the panel, so closing it must not release it a second time.
    unsafe { panel.setReleasedWhenClosed(false) };
    // Panels hide when their app deactivates, which for a menu bar app is as soon as the user clicks elsewhere.
    panel.setHidesOnDeactivate(false);
    panel.setTitle(&NSString::from_str(t.quick_reference));
    panel.setContentView(Some(&stack));
    panel.setContentSize(stack.fittingSize());
    panel.center();
    panel
}
