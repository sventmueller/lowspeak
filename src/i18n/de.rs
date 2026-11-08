use super::Texts;

pub static DE: Texts = Texts {
    recent_dictations: "Letzte Diktate",
    none_yet: "Noch keine",
    retry_failed_dictation: "Fehlgeschlagenes Diktat erneut transkribieren",
    local: "Lokal",
    not_installed: |model| format!("{model} (nicht installiert)"),
    no_key: |provider| format!("{provider} (kein Schlüssel)"),
    language: "Sprache",
    auto_detect: "Automatisch erkennen",
    post_processing: "Nachbearbeitung",
    post_processing_off: "Aus",
    post_processing_cleanup: "Bereinigen (Füllwörter, Grammatik)",
    post_processing_format: "Formatieren (kontextabhängig)",
    vocabulary_menu: |terms| match terms {
        0 => "Vokabular …".to_owned(),
        1 => "Vokabular (1 Begriff) …".to_owned(),
        terms => format!("Vokabular ({terms} Begriffe) …"),
    },
    microphone: "Mikrofon",
    system_default: "Systemstandard",
    not_connected: |microphone| format!("{microphone} (nicht verbunden)"),
    play_sound_when_ready: "Signalton, sobald bereit",
    manage_models: "Modelle verwalten",
    delete_model: |model| format!("{model} löschen"),
    download_model: |model| format!("{model} laden"),
    set_api_key: "API-Schlüssel festlegen …",
    update_api_key: "API-Schlüssel ändern …",
    open_at_login: "Bei der Anmeldung öffnen",
    check_for_updates: "Nach Updates suchen …",
    check_updates_automatically: "Automatisch nach Updates suchen",
    install_updates_automatically: "Updates automatisch laden und installieren",
    help: "Hilfe",
    quick_reference: "Kurzreferenz",
    report_problem: "Problem melden …",
    suggest_feature: "Funktion vorschlagen …",
    on_github: "Lowspeak auf GitHub",
    about: "Über Lowspeak",
    license: "Lizenz: MIT oder Apache 2.0",
    quit: "Lowspeak beenden",

    reference_hold: "Halten zum Diktieren, loslassen zum Einfügen",
    reference_double_tap: "Zweimal kurz drücken: freihändig diktieren",
    reference_shift: "Freihändig diktieren",
    reference_stop: "Freihändiges Diktat beenden",
    reference_emoji: "Falls die 🌐-Taste „Emoji & Symbole“ öffnet: in den Tastatureinstellungen abschalten.",
    keyboard_settings: "Tastatureinstellungen",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "Englisch",
    languages: |count| format!("{count} Sprachen"),
    legacy: "älteres Modell",
    fast: "schnell",

    set_api_key_title: "API-Schlüssel festlegen",
    update_api_key_title: "API-Schlüssel ändern",
    api_key_info: "API-Schlüssel eingeben. Der Anbieter wird automatisch erkannt:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... oder gsk_...",
    save: "Sichern",
    cancel: "Abbrechen",
    ok: "OK",
    remove_all_keys: "Alle Schlüssel entfernen",
    unknown_key_format: "Unbekanntes Schlüsselformat",
    unknown_key_format_info: "Erwartet wird ein Schlüssel, der mit sk- (OpenAI) oder gsk_ (Groq) beginnt. Bitte erneut versuchen.",
    vocabulary_title: "Vokabular",
    vocabulary_info: "Namen und Fachbegriffe, die genau so geschrieben werden sollen, durch Kommas getrennt. OpenAI, Groq, Whisper und Parakeet auf diesem Mac sowie die Nachbearbeitung verwenden sie.",

    recording_failed: "Aufnahme fehlgeschlagen",
    microphone_silent: "Mikrofon stumm",
    microphone_silent_info: "Kein Audiosignal erkannt. Prüfen, ob das AirPods-Mikrofon verfügbar ist, oder zum eingebauten Mikrofon wechseln.",
    accessibility_required: "Zugriff auf Bedienungshilfen erforderlich",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak kann nicht automatisch einfügen. Zugriff erlauben unter Systemeinstellungen → Datenschutz & Sicherheit → {setting}, dann manuell mit ⌘V einfügen."
        )
    },
    accessibility_list: "Bedienungshilfen",
    device_control_list: "Gerätesteuerung und Datenzugriff",
    transcription_failed: "Transkription fehlgeschlagen",
    retry_failed_dictation_info: |menu_item| {
        format!("Die Aufnahme bleibt erhalten: Über „{menu_item}“ im Menü erneut transkribieren.")
    },
    transcribed_locally: "Auf diesem Mac transkribiert",
    transcribed_locally_info: |reason| {
        format!("{reason} Stattdessen hat ein lokales Modell transkribiert.")
    },
    post_processing_failed: "Nachbearbeitung fehlgeschlagen",
    post_processing_failed_info: |reason| {
        format!("{reason} Der Text wurde ohne Nachbearbeitung eingefügt.")
    },
    no_api_key: "Kein API-Schlüssel",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Kein API-Schlüssel für {provider} hinterlegt. Über „{menu_item}“ im Menü hinzufügen."
        )
    },
    login_item_error: "Anmeldeobjekt nicht geändert",
    login_item_error_info: |menu_item| {
        format!(
            "„{menu_item}“ konnte nicht geändert werden. Unter Systemeinstellungen → Allgemein → Anmeldeobjekte & Erweiterungen umschalten."
        )
    },
    api_key_error: "Fehler beim API-Schlüssel",
    api_key_not_saved: "Der API-Schlüssel konnte nicht gesichert werden. Erneut versuchen.",
    api_keys_not_removed: "Die API-Schlüssel konnten nicht entfernt werden. Erneut versuchen.",
    download_failed: "Download fehlgeschlagen",
    download_failed_info: |model| {
        format!(
            "{model} konnte nicht geladen werden. Internetverbindung prüfen und erneut versuchen."
        )
    },
    cannot_delete: "Löschen nicht möglich",
    cannot_delete_info: "Vor dem Löschen eines Modells warten, bis der laufende Vorgang abgeschlossen ist.",
    delete_failed: "Löschen fehlgeschlagen",
    delete_failed_info: |model| format!("{model} konnte nicht gelöscht werden. Erneut versuchen."),

    no_api_key_error: |provider| {
        format!("Kein API-Schlüssel für {provider} hinterlegt. Über die Menüleiste festlegen.")
    },
    invalid_response: "Ungültige Antwort vom Server.",
    authentication_failed: "API-Anmeldung fehlgeschlagen. API-Schlüssel prüfen.",
    rate_limited: "Anfragelimit überschritten. Bitte kurz warten.",
    server_error: |status| format!("Serverfehler ({status}). Später erneut versuchen."),
    api_error: |status| format!("API-Fehler ({status}). Details stehen im Log."),
    model_not_available: |menu_item| {
        format!("Lokales Modell nicht verfügbar. Über „{menu_item}“ laden.")
    },
    network_error: |message| format!("Netzwerkfehler: {message}"),
    key_rejected: |provider| format!("{provider} hat den API-Schlüssel abgelehnt."),
    quota_exhausted: |provider| {
        format!(
            "{provider} meldet, dass das Konto kein Guthaben mehr hat. Guthaben im {provider}-Konto aufladen."
        )
    },
    model_retired: |provider| {
        format!("{provider} bietet das Modell für die Nachbearbeitung nicht mehr an.")
    },
    no_post_processing_key: |first, second| {
        format!(
            "Für die Nachbearbeitung fehlt ein API-Schlüssel von {first} oder {second}. Über die Menüleiste festlegen."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} bietet {model} nicht mehr an. Im Menü ein anderes Modell wählen.")
    },
    request_rejected: |provider, status| {
        format!("{provider} hat die Anfrage zur Nachbearbeitung abgelehnt ({status}).")
    },
    microphone_permission_denied: "Lowspeak hat keinen Zugriff auf das Mikrofon. Zugriff erlauben unter Systemeinstellungen → Datenschutz & Sicherheit → Mikrofon.",
    no_microphone: "Kein Mikrofon angeschlossen.",
    microphone_error: |detail| format!("Das Mikrofon konnte nicht gestartet werden: {detail}"),
    paste_failed: "Einfügen fehlgeschlagen",
    paste_failed_info: "Der Text liegt in der Zwischenablage. Mit ⌘V einfügen.",
    not_pasted: "Diktat nicht eingefügt",
    not_pasted_info: "Eine andere App kam nach vorn, bevor der Text fertig war. Der Text liegt in der Zwischenablage. Mit ⌘V einfügen.",
    request_timed_out: "Zeitüberschreitung bei der Anfrage.",
    audio_failed: "Die Aufnahme konnte nicht verarbeitet werden.",
    local_model_failed: "Das lokale Modell konnte die Aufnahme nicht transkribieren.",
};
