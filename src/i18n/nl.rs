use super::Texts;

pub static NL: Texts = Texts {
    recent_dictations: "Recente dictaten",
    none_yet: "Nog geen",
    local: "Lokaal",
    not_installed: |model| format!("{model} (niet geïnstalleerd)"),
    no_key: |provider| format!("{provider} (geen sleutel)"),
    language: "Taal",
    auto_detect: "Automatisch herkennen",
    post_processing: "Nabewerking",
    post_processing_off: "Uit",
    post_processing_cleanup: "Opschonen (stopwoorden, grammatica)",
    post_processing_format: "Opmaken (contextbewust)",
    vocabulary_menu: |terms| match terms {
        0 => "Woordenlijst…".to_owned(),
        1 => "Woordenlijst (1 term)…".to_owned(),
        terms => format!("Woordenlijst ({terms} termen)…"),
    },
    microphone: "Microfoon",
    system_default: "Systeemstandaard",
    not_connected: |microphone| format!("{microphone} (niet verbonden)"),
    play_sound_when_ready: "Speel geluid af zodra gereed",
    manage_models: "Beheer modellen",
    delete_model: |model| format!("Verwijder {model}"),
    download_model: |model| format!("Download {model}"),
    set_api_key: "Stel API-sleutel in…",
    update_api_key: "Wijzig API-sleutel…",
    open_at_login: "Open bij inloggen",
    check_for_updates: "Zoek naar updates…",
    check_updates_automatically: "Zoek automatisch naar updates",
    install_updates_automatically: "Download en installeer updates automatisch",
    help: "Help",
    quick_reference: "Beknopt overzicht",
    report_problem: "Meld een probleem…",
    suggest_feature: "Stel een functie voor…",
    on_github: "Lowspeak op GitHub",
    about: "Over Lowspeak",
    license: "Licentie: MIT of Apache 2.0",
    quit: "Stop Lowspeak",

    reference_hold: "Houd ingedrukt om te dicteren, laat los om te plakken",
    reference_double_tap: "Druk twee keer snel om handsfree te dicteren",
    reference_shift: "Handsfree dicteren",
    reference_stop: "Handsfree dicteren beëindigen",
    reference_emoji: "Opent de 🌐-toets Emoji's en symbolen, zet dit dan uit in de toetsenbordinstellingen.",
    keyboard_settings: "Instellingen toetsenbord",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "Engels",
    languages: |count| match count {
        1 => "1 taal".to_owned(),
        count => format!("{count} talen"),
    },
    legacy: "ouder model",
    fast: "snel",

    set_api_key_title: "Stel API-sleutel in",
    update_api_key_title: "Wijzig API-sleutel",
    api_key_info: "Voer je API-sleutel in. De aanbieder wordt automatisch herkend:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... of gsk_...",
    save: "Bewaar",
    cancel: "Annuleer",
    ok: "OK",
    remove_all_keys: "Verwijder alle sleutels",
    unknown_key_format: "Onbekende sleutelindeling",
    unknown_key_format_info: "De sleutel moet beginnen met sk- (OpenAI) of gsk_ (Groq). Probeer het opnieuw.",
    vocabulary_title: "Woordenlijst",
    vocabulary_info: "Namen en vaktermen die precies zo gespeld moeten worden, gescheiden door komma's. OpenAI, Groq, Whisper op deze Mac en de nabewerking gebruiken ze.",

    recording_failed: "Opname mislukt",
    microphone_silent: "Geen geluid van microfoon",
    microphone_silent_info: "Geen audiosignaal gedetecteerd. Controleer of de microfoon van je AirPods beschikbaar is of schakel over naar de ingebouwde microfoon.",
    accessibility_required: "Toestemming voor toegankelijkheid vereist",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak kan niet automatisch plakken. Geef toestemming in Systeeminstellingen → Privacy en beveiliging → {setting} en plak daarna handmatig met ⌘V."
        )
    },
    accessibility_list: "Toegankelijkheid",
    device_control_list: "Apparaatbeheer en gegevenstoegang",
    transcription_failed: "Transcriptie mislukt",
    transcribed_locally: "Getranscribeerd op deze Mac",
    transcribed_locally_info: |reason| {
        format!("{reason} In plaats daarvan heeft een lokaal model getranscribeerd.")
    },
    post_processing_failed: "Nabewerking mislukt",
    post_processing_failed_info: |reason| {
        format!("{reason} De tekst is zonder nabewerking geplakt.")
    },
    no_api_key: "Geen API-sleutel",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Geen API-sleutel voor {provider} ingesteld. Voeg er een toe via ‘{menu_item}’ in het menu."
        )
    },
    login_item_error: "Fout bij inlogonderdeel",
    login_item_error_info: |menu_item| {
        format!(
            "Kan ‘{menu_item}’ niet wijzigen. Probeer het aan of uit te zetten in Systeeminstellingen → Algemeen → Inlogonderdelen en extensies."
        )
    },
    api_key_error: "Fout bij API-sleutel",
    api_key_not_saved: "Kan de API-sleutel niet bewaren. Probeer het opnieuw.",
    api_keys_not_removed: "Kan de API-sleutels niet verwijderen. Probeer het opnieuw.",
    download_failed: "Download mislukt",
    download_failed_info: |model| {
        format!(
            "Kan {model} niet downloaden. Controleer de internetverbinding en probeer het opnieuw."
        )
    },
    cannot_delete: "Kan niet verwijderen",
    cannot_delete_info: "Wacht tot de huidige bewerking klaar is voordat je een model verwijdert.",
    delete_failed: "Verwijderen mislukt",
    delete_failed_info: |model| format!("Kan {model} niet verwijderen. Probeer het opnieuw."),

    no_api_key_error: |provider| {
        format!("Geen API-sleutel voor {provider} ingesteld. Stel deze in via de menubalk.")
    },
    invalid_response: "Ongeldig antwoord van de server.",
    authentication_failed: "API-authenticatie mislukt. Controleer je API-sleutel.",
    rate_limited: "Limiet voor verzoeken bereikt. Wacht even.",
    server_error: |status| format!("Serverfout ({status}). Probeer het later opnieuw."),
    api_error: |status| format!("API-fout ({status}). Zie het logbestand voor details."),
    model_not_available: |menu_item| {
        format!("Lokaal model niet beschikbaar. Download het via ‘{menu_item}’.")
    },
    network_error: |message| format!("Netwerkfout: {message}"),
    key_rejected: |provider| format!("{provider} heeft de API-sleutel geweigerd."),
    model_retired: |provider| format!("{provider} biedt het nabewerkingsmodel niet meer aan."),
    no_post_processing_key: |first, second| {
        format!(
            "Nabewerking heeft een API-sleutel van {first} of {second} nodig. Stel deze in via de menubalk."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} biedt {model} niet meer aan. Kies een ander model in het menu.")
    },
    request_rejected: |provider, status| {
        format!("{provider} heeft het nabewerkingsverzoek geweigerd ({status}).")
    },
    microphone_permission_denied: "Lowspeak heeft geen toegang tot de microfoon. Sta dit toe in Systeeminstellingen → Privacy en beveiliging → Microfoon.",
    no_microphone: "Er is geen microfoon aangesloten.",
    microphone_error: |detail| format!("De microfoon kon niet worden gestart: {detail}"),
    paste_failed: "Plakken mislukt",
    paste_failed_info: "De tekst staat op het klembord. Plak hem met ⌘V.",
    request_timed_out: "Er is een time-out opgetreden voor het verzoek.",
    audio_failed: "De opname kon niet worden verwerkt.",
    local_model_failed: "Het lokale model kon de opname niet transcriberen.",
};
