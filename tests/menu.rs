//! Builds the status item menu and icons on the main thread and checks what the user would see;
//! also offers a dictation on a pasteboard of its own, since AppKit wants the main thread there too.

use std::sync::{Arc, Mutex};

use objc2::msg_send;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSApplication, NSControlStateValueOn, NSImage, NSMenu, NSMenuItem, NSPasteboard,
    NSPasteboardTypeString,
};
use objc2_foundation::{MainThreadMarker, ns_string};

use lowspeak::app::menu::{self, StatusUi};
use lowspeak::app::{Snapshot, Status};
use lowspeak::app::{about, reference};
use lowspeak::audio::recorder::Microphone;
use lowspeak::config::settings::ChosenMicrophone;
use lowspeak::i18n;
use lowspeak::models::{
    ApiProvider, Language, LocalModel, OnlineModel, PostProcessingMode, TranscriptionMode,
};
use lowspeak::paste::{self, Provided};

fn items(menu: &NSMenu) -> Vec<Retained<NSMenuItem>> {
    (0..menu.numberOfItems())
        .filter_map(|index| menu.itemAtIndex(index))
        .collect()
}

fn titles(menu: &NSMenu) -> Vec<String> {
    items(menu)
        .iter()
        .map(|item| item.title().to_string())
        .collect()
}

fn find(menu: &NSMenu, title: &str) -> Retained<NSMenuItem> {
    items(menu)
        .into_iter()
        .find(|item| item.title().to_string() == title)
        .unwrap_or_else(|| panic!("no menu item {title:?} in {:?}", titles(menu)))
}

fn submenu(menu: &NSMenu, title: &str) -> Retained<NSMenu> {
    find(menu, title)
        .submenu()
        .unwrap_or_else(|| panic!("{title} has no submenu"))
}

fn checked(item: &NSMenuItem) -> bool {
    item.state() == NSControlStateValueOn
}

fn renders(image: &NSImage) -> bool {
    // SAFETY: rendering the image at its own size, which runs its drawing handler.
    unsafe { image.CGImageForProposedRect_context_hints(std::ptr::null_mut(), None, None) }
        .is_some()
}

fn strings(pasteboard: &NSPasteboard) -> Vec<String> {
    pasteboard
        .types()
        .map(|types| types.iter().map(|kind| kind.to_string()).collect())
        .unwrap_or_default()
}

/// The dictation is handed out on the first read only, marked for clipboard history apps to skip,
/// and the previous content comes back whole.
fn clipboard_offers_the_dictation_lazily_and_restores_what_was_there() {
    let pasteboard = NSPasteboard::pasteboardWithUniqueName();
    // SAFETY: framework constant.
    let text = unsafe { NSPasteboardTypeString };
    pasteboard.clearContents();
    pasteboard.setString_forType(ns_string!("previous"), text);
    let saved = paste::save(&pasteboard);

    let events = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&events);
    let offer = paste::offer(&pasteboard, "dictated", move |event| {
        seen.lock().unwrap().push(event);
    })
    .expect("the pasteboard takes the offer");
    let offered = strings(&pasteboard);
    assert!(
        offered.contains(&paste::TRANSIENT_TYPE.to_owned()) && offered.contains(&text.to_string()),
        "{offered:?}"
    );
    assert!(
        events.lock().unwrap().is_empty(),
        "read before anyone asked"
    );

    let read = |pasteboard: &NSPasteboard| {
        pasteboard
            .stringForType(text)
            .map(|string| string.to_string())
    };
    assert_eq!(read(&pasteboard).as_deref(), Some("dictated"));
    assert_eq!(read(&pasteboard).as_deref(), Some("dictated"));
    let reads = events
        .lock()
        .unwrap()
        .iter()
        .filter(|event| **event == Provided::Read)
        .count();
    assert_eq!(reads, 1);

    paste::restore(&pasteboard, &saved);
    assert_eq!(read(&pasteboard).as_deref(), Some("previous"));
    assert!(!strings(&pasteboard).contains(&paste::TRANSIENT_TYPE.to_owned()));
    drop(offer);
    // SAFETY: releases the pasteboard server's resources for this test's own pasteboard.
    let () = unsafe { msg_send![&*pasteboard, releaseGlobally] };
}

fn main() {
    let mtm = MainThreadMarker::new().expect("the menu test must run on the main thread");
    let _application = NSApplication::sharedApplication(mtm);
    clipboard_offers_the_dictation_lazily_and_restores_what_was_there();
    let mut ui = StatusUi::new(mtm);

    let state = Snapshot {
        mode: TranscriptionMode::Local,
        local_model: LocalModel::ParakeetUltra,
        online_model: OnlineModel::GroqWhisperLargeV3,
        language: Language::En,
        post_processing: PostProcessingMode::Off,
        downloaded: vec![LocalModel::Base, LocalModel::ParakeetUltra],
        downloading: Some((LocalModel::Tiny, 0.42)),
        deleting: Some(LocalModel::Small),
        providers: vec![ApiProvider::Groq],
        microphones: vec![
            Microphone {
                id: "coreaudio:BuiltIn".into(),
                name: "MacBook Air-Mikrofon".into(),
            },
            Microphone {
                id: "coreaudio:Display".into(),
                name: "Studio Display-Mikrofon".into(),
            },
        ],
        microphone: Some(ChosenMicrophone {
            id: "coreaudio:AirPods".into(),
            name: "AirPods".into(),
        }),
        start_sound: true,
        recent: vec![
            "Kurz.".into(),
            "Ein sehr langes Diktat, das im Menü auf eine Zeile gekürzt wird,\nmit Umbruch.".into(),
        ],
        vocabulary_terms: 3,
        login_item: true,
    };
    let root = NSMenu::new(mtm);
    ui.rebuild(&root, &state);

    let top = titles(&root);
    assert_eq!(
        &top[..4],
        ["Recent Dictations", "", "Local", "OpenAI (No Key)"]
    );
    assert!(top.contains(&"Vocabulary (3 Terms)…".to_owned()), "{top:?}");
    assert!(top.contains(&"Update API Key…".to_owned()), "{top:?}");
    assert_eq!(
        &top[top.len() - 3..],
        ["Help", "About Lowspeak", "Quit Lowspeak"]
    );
    let quit = find(&root, "Quit Lowspeak");
    assert!(quit.isEnabled());
    assert_eq!(quit.keyEquivalent().to_string(), "q");
    assert_eq!(
        titles(&submenu(&root, "Help")),
        [
            "Quick Reference",
            "",
            "Report Problem…",
            "Suggest Feature…",
            "Lowspeak on GitHub"
        ]
    );
    assert!(
        items(&submenu(&root, "Help"))
            .iter()
            .all(|item| item.isSeparatorItem() || item.isEnabled())
    );
    assert!(find(&root, "About Lowspeak").isEnabled());
    let login = find(&root, "Open at Login");
    assert!(login.isEnabled() && checked(&login) && !login.title().to_string().contains('✓'));

    let local = submenu(&root, "Local");
    let parakeet = find(&local, "Parakeet Ultra (~630 MB, 25 Languages)");
    assert!(checked(&parakeet) && parakeet.isEnabled());
    let tiny = find(&local, "Tiny (~75 MB) (Not Installed)");
    assert!(!tiny.isEnabled() && !checked(&tiny));
    assert!(find(&local, "Base (~150 MB)").isEnabled());
    assert!(
        find(&local, "Whisper").isSectionHeader() && find(&local, "Parakeet").isSectionHeader()
    );

    let openai = submenu(&root, "OpenAI (No Key)");
    let openai_items = items(&openai);
    let (set_key, models) = openai_items.split_last().unwrap();
    assert_eq!(set_key.title().to_string(), "Set API Key…");
    assert!(set_key.isEnabled());
    assert!(
        models
            .iter()
            .all(|item| item.isSeparatorItem() || (!item.isEnabled() && !checked(item))),
        "{:?}",
        titles(&openai)
    );
    let groq = submenu(&root, "Groq");
    assert!(
        !checked(&find(&groq, "Whisper Large V3")),
        "online model must not be checked in local mode"
    );

    assert!(checked(&find(&submenu(&root, "Language"), "English")));
    assert!(checked(&find(&submenu(&root, "Post-Processing"), "Off")));

    let microphones = submenu(&root, "Microphone");
    assert!(!checked(&find(&microphones, "System Default")));
    assert!(!checked(&find(&microphones, "Studio Display-Mikrofon")));
    let missing = find(&microphones, "AirPods (Not Connected)");
    assert!(checked(&missing) && !missing.isEnabled());
    assert!(checked(&find(&microphones, "Play Sound When Ready")));
    assert!(find(&microphones, "MacBook Air-Mikrofon").isEnabled());

    let recent = submenu(&root, "Recent Dictations");
    assert_eq!(
        titles(&recent),
        [
            "Kurz.",
            "Ein sehr langes Diktat, das im Menü auf eine Zeil…"
        ]
    );
    let long = find(
        &recent,
        "Ein sehr langes Diktat, das im Menü auf eine Zeil…",
    );
    assert!(long.isEnabled());
    assert_eq!(long.toolTip().unwrap().to_string(), state.recent[1]);

    let manage = submenu(&root, "Manage Models");
    assert!(!find(&manage, "↓ Tiny (~75 MB) — 42%").isEnabled());
    assert!(find(&manage, "Delete Parakeet Ultra (~630 MB, 25 Languages)").isEnabled());
    assert!(find(&manage, "Download Medium (~1.5 GB)").isEnabled());
    assert!(!find(&manage, "Delete Small (~470 MB)").isEnabled());

    // Rebuilding replaces the items and the action table instead of appending to them.
    let count = root.numberOfItems();
    ui.rebuild(&root, &state);
    assert_eq!(root.numberOfItems(), count);

    let empty = Snapshot {
        recent: Vec::new(),
        vocabulary_terms: 0,
        ..state
    };
    ui.rebuild(&root, &empty);
    let none = find(&submenu(&root, "Recent Dictations"), "None Yet");
    assert!(!none.isEnabled());
    assert!(titles(&root).contains(&"Vocabulary…".to_owned()));

    let german = Snapshot {
        language: Language::De,
        vocabulary_terms: 3,
        ..empty
    };
    ui.rebuild(&root, &german);
    let top = titles(&root);
    assert_eq!(
        &top[..4],
        ["Letzte Diktate", "", "Lokal", "OpenAI (kein Schlüssel)"]
    );
    assert!(
        top.contains(&"Vokabular (3 Begriffe) …".to_owned()),
        "{top:?}"
    );
    assert_eq!(
        &top[top.len() - 3..],
        ["Hilfe", "Über Lowspeak", "Lowspeak beenden"]
    );
    assert_eq!(
        titles(&submenu(&root, "Hilfe")),
        [
            "Kurzreferenz",
            "",
            "Problem melden …",
            "Funktion vorschlagen …",
            "Lowspeak auf GitHub"
        ]
    );
    let lokal = submenu(&root, "Lokal");
    assert!(!find(&lokal, "Medium (~1,5 GB) (nicht installiert)").isEnabled());
    let sprache = submenu(&root, "Sprache");
    assert!(checked(&find(&sprache, "Deutsch")));
    assert!(!checked(&find(&sprache, "Automatisch erkennen")));
    assert!(
        find(
            &submenu(&root, "Modelle verwalten"),
            "Base (~150 MB) löschen"
        )
        .isEnabled()
    );

    for (status, hands_free) in [
        (Status::Idle, false),
        (Status::Starting, false),
        (Status::Recording, false),
        (Status::Recording, true),
    ] {
        assert_eq!(ui.show(status, hands_free, None), None);
    }
    assert_eq!(ui.show(Status::Recording, true, Some(0.5)), None);
    let pulse = ui
        .show(Status::Transcribing, false, Some(0.5))
        .expect("transcribing starts the pulse");
    assert_eq!(ui.show(Status::Transcribing, false, Some(0.6)), None);
    assert!(ui.pulse(pulse) && ui.pulse(pulse));
    assert_eq!(ui.show(Status::Idle, false, None), None);
    assert!(!ui.pulse(pulse), "the pulse ends with the transcription");
    assert!(
        renders(&menu::idle_icon())
            && renders(&menu::recording_icon())
            && renders(&menu::hands_free_icon())
            && renders(&menu::blank_icon())
    );

    // Help → Quick Reference opens one window, in every interface language, at a size that fits next to other windows.
    for language in Language::ALL {
        let texts = i18n::texts(language);
        reference::show(mtm, texts);
        let windows = NSApplication::sharedApplication(mtm).windows();
        let open: Vec<_> = windows
            .iter()
            .filter(|window| {
                window.isVisible() && window.title().to_string() == texts.quick_reference
            })
            .collect();
        assert_eq!(open.len(), 1, "{}", language.raw_value());
        let size = open[0].frame().size;
        assert!(
            (300.0..600.0).contains(&size.width) && size.height < 400.0,
            "{}: {size:?}",
            language.raw_value()
        );
    }

    about::show(mtm, i18n::texts(Language::De));

    println!("menu test passed");
}
