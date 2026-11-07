//! The menu bar item: icon, status animation and the menu, rebuilt each time it opens.

use std::cell::RefCell;
use std::time::Duration;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{Bool, ProtocolObject, Sel};
use objc2::{DefinedClass as _, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSBezierPath, NSCellImagePosition, NSColor, NSCompositingOperation, NSControlStateValue,
    NSControlStateValueOff, NSControlStateValueOn, NSGraphicsContext, NSImage, NSMenu,
    NSMenuDelegate, NSMenuItem, NSMenuItemBadge, NSStatusBar, NSStatusItem,
    NSVariableStatusItemLength,
};
use objc2_foundation::{
    MainThreadMarker, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString, ns_string,
};

use super::state::{Snapshot, Status};
use super::updates::UpdateMenu;
use super::{Action, menu_needs_update, perform};
use crate::i18n::{self, Texts};
use crate::models::{
    ApiProvider, Language, LocalModel, ModelFamily, OnlineModel, PostProcessingMode,
    TranscriptionMode,
};
use crate::util::numeric::floor_usize;

const ICON_SIZE: f64 = 18.0;
const RING_DIAMETER: f64 = 14.0;
pub const PULSE_INTERVAL: Duration = Duration::from_millis(500);

pub struct TargetIvars {
    /// The actions of the items in the menu, indexed by their tags.
    actions: RefCell<Vec<Action>>,
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements and `MenuTarget` does not implement Drop.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "LowspeakMenuTarget"]
    #[ivars = TargetIvars]
    pub struct MenuTarget;

    unsafe impl NSObjectProtocol for MenuTarget {}

    unsafe impl NSMenuDelegate for MenuTarget {
        #[unsafe(method(menuNeedsUpdate:))]
        fn menu_needs_update(&self, menu: &NSMenu) {
            menu_needs_update(menu);
        }
    }

    impl MenuTarget {
        #[unsafe(method(menuAction:))]
        fn menu_action(&self, sender: &NSMenuItem) {
            let action = usize::try_from(sender.tag())
                .ok()
                .and_then(|index| self.ivars().actions.borrow().get(index).cloned());
            if let Some(action) = action {
                perform(action);
            }
        }
    }
);

impl MenuTarget {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(TargetIvars {
            actions: RefCell::new(Vec::new()),
        });
        // SAFETY: NSObject's init.
        unsafe { msg_send![super(this), init] }
    }
}

fn image(draw: impl Fn() + 'static) -> Retained<NSImage> {
    let handler = RcBlock::new(move |_rect: NSRect| -> Bool {
        draw();
        Bool::YES
    });
    let image = NSImage::imageWithSize_flipped_drawingHandler(
        NSSize::new(ICON_SIZE, ICON_SIZE),
        false,
        &handler,
    );
    image.setTemplate(true);
    image.setAccessibilityDescription(Some(ns_string!("Lowspeak")));
    image
}

fn centered(diameter: f64) -> NSRect {
    let origin = (ICON_SIZE - diameter) / 2.0;
    NSRect::new(
        NSPoint::new(origin, origin),
        NSSize::new(diameter, diameter),
    )
}

fn draw_idle() {
    let color = NSColor::controlTextColor();
    color.setFill();
    color.setStroke();
    let line_width = 1.4;
    let ring = centered(RING_DIAMETER - line_width);
    let path = NSBezierPath::bezierPathWithOvalInRect(ring);
    path.setLineWidth(line_width);
    path.stroke();
    NSBezierPath::bezierPathWithOvalInRect(centered(5.5)).fill();
}

pub fn idle_icon() -> Retained<NSImage> {
    image(draw_idle)
}

/// The idle icon with a dot at the top right, cut out of the ring, while an update waits.
pub fn update_icon() -> Retained<NSImage> {
    image(|| {
        draw_idle();
        let dot = |diameter: f64| {
            let origin = ICON_SIZE - 3.0 - diameter / 2.0;
            NSBezierPath::bezierPathWithOvalInRect(NSRect::new(
                NSPoint::new(origin, origin),
                NSSize::new(diameter, diameter),
            ))
        };
        if let Some(context) = NSGraphicsContext::currentContext() {
            context.setCompositingOperation(NSCompositingOperation::Clear);
            dot(9.0).fill();
            context.setCompositingOperation(NSCompositingOperation::SourceOver);
        }
        dot(6.0).fill();
    })
}

pub fn recording_icon() -> Retained<NSImage> {
    image(|| {
        NSColor::controlTextColor().setFill();
        NSBezierPath::bezierPathWithOvalInRect(centered(RING_DIAMETER)).fill();
    })
}

/// Hands-free recording: a square, the stop symbol, since the next fn or Shift press ends it.
pub fn hands_free_icon() -> Retained<NSImage> {
    image(|| {
        NSColor::controlTextColor().setFill();
        let side = RING_DIAMETER - 2.0;
        NSBezierPath::bezierPathWithRoundedRect_xRadius_yRadius(centered(side), 2.5, 2.5).fill();
    })
}

/// Keeps the item's width while the transcribing pulse hides the icon.
pub fn blank_icon() -> Retained<NSImage> {
    image(|| {})
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    Idle,
    Recording,
    HandsFree,
    /// The idle icon blinking while dictations are transcribed.
    Pulsing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Look {
    pub icon: Icon,
    /// The download progress, next to the icon.
    pub title: String,
}

pub fn look(status: Status, hands_free: bool, download: Option<f64>) -> Look {
    let icon = match status {
        Status::Idle | Status::Starting => Icon::Idle,
        Status::Recording if hands_free => Icon::HandsFree,
        Status::Recording => Icon::Recording,
        Status::Transcribing => Icon::Pulsing,
    };
    let title = download.map_or_else(String::new, |progress| {
        format!("↓ {}%", floor_usize(progress * 100.0))
    });
    Look { icon, title }
}

struct Icons {
    idle: Retained<NSImage>,
    update: Retained<NSImage>,
    recording: Retained<NSImage>,
    hands_free: Retained<NSImage>,
    blank: Retained<NSImage>,
}

fn checkmark(on: bool) -> NSControlStateValue {
    if on {
        NSControlStateValueOn
    } else {
        NSControlStateValueOff
    }
}

pub struct StatusUi {
    mtm: MainThreadMarker,
    item: Retained<NSStatusItem>,
    target: Retained<MenuTarget>,
    icons: Icons,
    shown: Option<Look>,
    pulse_generation: u64,
    pulse_visible: bool,
    update_waiting: bool,
}

impl StatusUi {
    pub fn new(mtm: MainThreadMarker) -> Self {
        let item = NSStatusBar::systemStatusBar().statusItemWithLength(NSVariableStatusItemLength);
        let target = MenuTarget::new(mtm);
        let menu = NSMenu::new(mtm);
        menu.setAutoenablesItems(false);
        menu.setDelegate(Some(ProtocolObject::from_ref(&*target)));
        item.setMenu(Some(&menu));
        if let Some(button) = item.button(mtm) {
            button.setImagePosition(NSCellImagePosition::ImageLeading);
        }
        let mut ui = Self {
            mtm,
            item,
            target,
            icons: Icons {
                idle: idle_icon(),
                update: update_icon(),
                recording: recording_icon(),
                hands_free: hands_free_icon(),
                blank: blank_icon(),
            },
            shown: None,
            pulse_generation: 0,
            pulse_visible: true,
            update_waiting: false,
        };
        ui.show(Status::Idle, false, None);
        ui
    }

    /// The status item's menu, for rebuilding it while it is open.
    pub fn menu(&self) -> Option<Retained<NSMenu>> {
        self.item.menu(self.mtm)
    }

    /// Returns the generation to pass to `pulse` after `PULSE_INTERVAL` when the pulse starts.
    pub fn show(&mut self, status: Status, hands_free: bool, download: Option<f64>) -> Option<u64> {
        let look = look(status, hands_free, download);
        if self.shown.as_ref() == Some(&look) {
            return None;
        }
        let was_pulsing = self
            .shown
            .as_ref()
            .is_some_and(|shown| shown.icon == Icon::Pulsing);
        let pulsing = look.icon == Icon::Pulsing;
        self.shown = Some(look);
        if pulsing != was_pulsing {
            self.pulse_generation += 1;
            self.pulse_visible = true;
        }
        self.draw();
        (pulsing && !was_pulsing).then_some(self.pulse_generation)
    }

    /// Returns whether to call it again after `PULSE_INTERVAL`.
    pub fn pulse(&mut self, generation: u64) -> bool {
        if generation != self.pulse_generation {
            return false;
        }
        self.pulse_visible = !self.pulse_visible;
        self.draw();
        true
    }

    /// Marks the idle icon while an update waits for the user, see `updates`.
    pub fn show_update(&mut self, waiting: bool) {
        self.update_waiting = waiting;
        self.draw();
    }

    fn draw(&self) {
        let (Some(look), Some(button)) = (&self.shown, self.item.button(self.mtm)) else {
            return;
        };
        let idle = if self.update_waiting {
            &self.icons.update
        } else {
            &self.icons.idle
        };
        let image = match look.icon {
            Icon::Idle => idle,
            Icon::Recording => &self.icons.recording,
            Icon::HandsFree => &self.icons.hands_free,
            Icon::Pulsing if self.pulse_visible => idle,
            Icon::Pulsing => &self.icons.blank,
        };
        button.setImage(Some(image));
        button.setTitle(&NSString::from_str(&look.title));
    }

    fn item(&self, menu: &NSMenu, title: &str, action: Option<Sel>) -> Retained<NSMenuItem> {
        // SAFETY: the selector, if any, is implemented by `MenuTarget`, which `action_item` sets as
        // the target.
        let item = unsafe {
            NSMenuItem::initWithTitle_action_keyEquivalent(
                NSMenuItem::alloc(self.mtm),
                &NSString::from_str(title),
                action,
                &NSString::new(),
            )
        };
        menu.addItem(&item);
        item
    }

    fn action_item(&self, menu: &NSMenu, title: &str, action: Action) -> Retained<NSMenuItem> {
        let item = self.item(menu, title, Some(sel!(menuAction:)));
        // SAFETY: the target lives as long as the status item.
        unsafe { item.setTarget(Some(&self.target)) };
        let mut actions = self.target.ivars().actions.borrow_mut();
        item.setTag(isize::try_from(actions.len()).unwrap_or(isize::MAX));
        actions.push(action);
        item
    }

    fn label(&self, menu: &NSMenu, title: &str) -> Retained<NSMenuItem> {
        let item = self.item(menu, title, None);
        item.setEnabled(false);
        item
    }

    fn header(&self, menu: &NSMenu, title: &str) {
        menu.addItem(&NSMenuItem::sectionHeaderWithTitle(
            &NSString::from_str(title),
            self.mtm,
        ));
    }

    fn submenu(&self, menu: &NSMenu, title: &str) -> Retained<NSMenu> {
        let item = self.item(menu, title, None);
        let submenu = NSMenu::new(self.mtm);
        submenu.setAutoenablesItems(false);
        item.setSubmenu(Some(&submenu));
        submenu
    }

    fn separator(&self, menu: &NSMenu) {
        menu.addItem(&NSMenuItem::separatorItem(self.mtm));
    }

    pub fn rebuild(&self, menu: &NSMenu, state: &Snapshot) {
        menu.removeAllItems();
        self.target.ivars().actions.borrow_mut().clear();
        let t = i18n::texts(state.language);

        self.add_recent(menu, state, t);
        self.separator(menu);
        self.add_local_models(menu, state, t);
        self.add_providers(menu, state, t);
        self.separator(menu);
        self.add_dictation_settings(menu, state, t);
        self.add_microphones(menu, state, t);
        self.separator(menu);
        self.add_model_management(menu, state, t);
        self.separator(menu);
        let login = self.action_item(menu, t.open_at_login, Action::ToggleLogin);
        login.setState(checkmark(state.login_item));
        if let Some(updates) = state.updates {
            self.separator(menu);
            self.add_updates(menu, updates, t);
        }
        self.separator(menu);
        self.add_help(menu, t);
    }

    fn add_updates(&self, menu: &NSMenu, updates: UpdateMenu, t: &Texts) {
        let check = self.action_item(menu, t.check_for_updates, Action::CheckForUpdates);
        check.setEnabled(updates.can_check);
        if updates.waiting {
            check.setBadge(Some(&NSMenuItemBadge::updatesWithCount(1)));
        }
        let checks = self.action_item(
            menu,
            t.check_updates_automatically,
            Action::ToggleUpdateChecks,
        );
        checks.setState(checkmark(updates.checks_automatically));
        let installs = self.action_item(
            menu,
            t.install_updates_automatically,
            Action::ToggleUpdateInstalls,
        );
        installs.setState(checkmark(updates.installs_automatically));
        installs.setEnabled(updates.can_install_automatically);
    }

    fn add_recent(&self, menu: &NSMenu, state: &Snapshot, t: &Texts) {
        let submenu = self.submenu(menu, t.recent_dictations);
        if state.recent.is_empty() {
            self.label(&submenu, t.none_yet);
        }
        for text in &state.recent {
            let item = self.action_item(&submenu, &preview(text), Action::CopyRecent(text.clone()));
            item.setToolTip(Some(&NSString::from_str(text)));
        }
    }

    fn add_local_models(&self, menu: &NSMenu, state: &Snapshot, t: &Texts) {
        let local = self.submenu(menu, t.local);
        for (family, title) in [
            (ModelFamily::Whisper, "Whisper"),
            (ModelFamily::Parakeet, "Parakeet"),
        ] {
            if family == ModelFamily::Parakeet {
                self.separator(&local);
            }
            self.header(&local, title);
            for model in LocalModel::of_family(family) {
                let installed = state.downloaded.contains(&model);
                let label = if installed {
                    t.local_model(model)
                } else {
                    (t.not_installed)(&t.local_model(model))
                };
                let item = self.action_item(&local, &label, Action::SelectLocal(model));
                item.setState(checkmark(
                    state.mode == TranscriptionMode::Local && state.local_model == model,
                ));
                item.setEnabled(installed);
            }
        }
    }

    fn add_providers(&self, menu: &NSMenu, state: &Snapshot, t: &Texts) {
        for provider in ApiProvider::ALL {
            let has_key = state.providers.contains(&provider);
            let title = if has_key {
                provider.display_name().to_owned()
            } else {
                (t.no_key)(provider.display_name())
            };
            let submenu = self.submenu(menu, &title);
            for model in OnlineModel::of_provider(provider) {
                let item = self.action_item(
                    &submenu,
                    &t.online_model(model),
                    Action::SelectOnline(model),
                );
                item.setState(checkmark(
                    has_key
                        && state.mode == TranscriptionMode::Online
                        && state.online_model == model,
                ));
                item.setEnabled(has_key);
            }
            if !has_key {
                self.separator(&submenu);
                self.action_item(&submenu, t.set_api_key, Action::ApiKey);
            }
        }
    }

    fn add_dictation_settings(&self, menu: &NSMenu, state: &Snapshot, t: &Texts) {
        let languages = self.submenu(menu, t.language);
        for language in Language::ALL {
            let item = self.action_item(
                &languages,
                t.language_name(language),
                Action::SelectLanguage(language),
            );
            item.setState(checkmark(state.language == language));
        }
        let post = self.submenu(menu, t.post_processing);
        for mode in PostProcessingMode::ALL {
            let item = self.action_item(
                &post,
                t.post_processing_mode(mode),
                Action::SelectPostProcessing(mode),
            );
            item.setState(checkmark(state.post_processing == mode));
        }
        self.action_item(
            menu,
            &(t.vocabulary_menu)(state.vocabulary_terms),
            Action::Vocabulary,
        );
    }

    fn add_microphones(&self, menu: &NSMenu, state: &Snapshot, t: &Texts) {
        let submenu = self.submenu(menu, t.microphone);
        let chosen = state
            .microphone
            .as_ref()
            .map(|microphone| microphone.id.as_str());
        let default = self.action_item(&submenu, t.system_default, Action::SelectMicrophone(None));
        default.setState(checkmark(chosen.is_none()));
        self.separator(&submenu);
        for microphone in &state.microphones {
            let item = self.action_item(
                &submenu,
                &microphone.name,
                Action::SelectMicrophone(Some(microphone.clone())),
            );
            item.setState(checkmark(chosen == Some(microphone.id.as_str())));
        }
        if let Some(missing) = state.microphone.as_ref().filter(|chosen| {
            !state
                .microphones
                .iter()
                .any(|microphone| microphone.id == chosen.id)
        }) {
            let item = self.label(&submenu, &(t.not_connected)(&missing.name));
            item.setState(NSControlStateValueOn);
        }
        self.separator(&submenu);
        let sound = self.action_item(&submenu, t.play_sound_when_ready, Action::ToggleStartSound);
        sound.setState(checkmark(state.start_sound));
    }

    fn add_model_management(&self, menu: &NSMenu, state: &Snapshot, t: &Texts) {
        let manage = self.submenu(menu, t.manage_models);
        for model in LocalModel::ALL {
            let name = t.local_model(model);
            match (state.downloading, state.deleting) {
                (Some((downloading, progress)), _) if downloading == model => {
                    let title = format!("↓ {name} — {}%", floor_usize(progress * 100.0));
                    self.label(&manage, &title);
                }
                (_, Some(deleting)) if deleting == model => {
                    self.label(&manage, &(t.delete_model)(&name));
                }
                _ if state.downloaded.contains(&model) => {
                    self.action_item(&manage, &(t.delete_model)(&name), Action::Delete(model));
                }
                _ => {
                    self.action_item(&manage, &(t.download_model)(&name), Action::Download(model));
                }
            }
        }
        let api_title = if state.providers.is_empty() {
            t.set_api_key
        } else {
            t.update_api_key
        };
        self.action_item(menu, api_title, Action::ApiKey);
    }

    fn add_help(&self, menu: &NSMenu, t: &Texts) {
        let help = self.submenu(menu, t.help);
        self.action_item(&help, t.quick_reference, Action::QuickReference);
        self.separator(&help);
        self.action_item(&help, t.report_problem, Action::ReportProblem);
        self.action_item(&help, t.suggest_feature, Action::SuggestFeature);
        self.action_item(&help, t.on_github, Action::OpenRepository);
        self.action_item(menu, t.about, Action::About);
        let quit = self.action_item(menu, t.quit, Action::Quit);
        quit.setKeyEquivalent(ns_string!("q"));
    }
}

fn preview(text: &str) -> String {
    const MAX_CHARS: usize = 50;
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if line.chars().count() <= MAX_CHARS {
        return line;
    }
    let cut: String = line.chars().take(MAX_CHARS - 1).collect();
    format!("{}…", cut.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_download_shows_next_to_every_status() {
        assert_eq!(
            look(Status::Recording, false, Some(0.427)),
            Look {
                icon: Icon::Recording,
                title: "↓ 42%".into()
            }
        );
        assert_eq!(look(Status::Recording, true, None).icon, Icon::HandsFree);
        assert_eq!(look(Status::Starting, false, None).icon, Icon::Idle);
        assert_eq!(
            look(Status::Transcribing, false, Some(1.0)),
            Look {
                icon: Icon::Pulsing,
                title: "↓ 100%".into()
            }
        );
        assert_eq!(look(Status::Idle, false, None).title, "");
    }

    #[test]
    fn previews_fit_on_one_line() {
        assert_eq!(preview("Kurz.\n"), "Kurz.");
        let long = preview(&"word ".repeat(30));
        assert_eq!(long.chars().count(), 50);
        assert!(long.ends_with("word…"));
    }
}
