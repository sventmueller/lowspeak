use super::Texts;

pub static NO: Texts = Texts {
    recent_dictations: "Nylige dikteringer",
    none_yet: "Ingen ennå",
    local: "Lokalt",
    not_installed: |model| format!("{model} (ikke installert)"),
    no_key: |provider| format!("{provider} (ingen nøkkel)"),
    language: "Språk",
    auto_detect: "Gjenkjenn automatisk",
    post_processing: "Etterbehandling",
    post_processing_off: "Av",
    post_processing_cleanup: "Rydd opp (fyllord, grammatikk)",
    post_processing_format: "Formater (etter kontekst)",
    vocabulary_menu: |terms| match terms {
        0 => "Ordliste…".to_owned(),
        1 => "Ordliste (1 term)…".to_owned(),
        terms => format!("Ordliste ({terms} termer)…"),
    },
    microphone: "Mikrofon",
    system_default: "Systemstandard",
    not_connected: |microphone| format!("{microphone} (ikke tilkoblet)"),
    play_sound_when_ready: "Spill av lyd når klar",
    manage_models: "Administrer modeller",
    delete_model: |model| format!("Slett {model}"),
    download_model: |model| format!("Last ned {model}"),
    set_api_key: "Angi API-nøkkel…",
    update_api_key: "Endre API-nøkkel…",
    open_at_login: "Åpne ved pålogging",
    check_for_updates: "Søk etter oppdateringer…",
    check_updates_automatically: "Søk etter oppdateringer automatisk",
    install_updates_automatically: "Last ned og installer oppdateringer automatisk",
    help: "Hjelp",
    quick_reference: "Hurtigoversikt",
    report_problem: "Rapporter et problem…",
    suggest_feature: "Foreslå en funksjon…",
    on_github: "Lowspeak på GitHub",
    about: "Om Lowspeak",
    license: "Lisens: MIT eller Apache 2.0",
    quit: "Avslutt Lowspeak",

    reference_hold: "Hold inne for å diktere, slipp for å lime inn",
    reference_double_tap: "Trykk to ganger raskt for å diktere håndfritt",
    reference_shift: "Dikter håndfritt",
    reference_stop: "Avslutt håndfri diktering",
    reference_emoji: "Hvis 🌐-tasten åpner Emoji og symboler, kan du slå det av i tastaturinnstillingene.",
    keyboard_settings: "Tastaturinnstillinger",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "Engelsk",
    languages: |count| format!("{count} språk"),
    legacy: "eldre modell",
    fast: "rask",

    set_api_key_title: "Angi API-nøkkel",
    update_api_key_title: "Endre API-nøkkel",
    api_key_info: "Skriv inn API-nøkkelen din. Leverandøren gjenkjennes automatisk:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... eller gsk_...",
    save: "Arkiver",
    cancel: "Avbryt",
    ok: "OK",
    remove_all_keys: "Fjern alle nøkler",
    unknown_key_format: "Ukjent nøkkelformat",
    unknown_key_format_info: "Nøkkelen må starte med sk- (OpenAI) eller gsk_ (Groq). Prøv igjen.",
    vocabulary_title: "Ordliste",
    vocabulary_info: "Navn og faguttrykk som skal staves nøyaktig, skilt med komma. OpenAI, Groq, Whisper på denne Macen og etterbehandlingen bruker dem.",

    recording_failed: "Opptaket mislyktes",
    microphone_silent: "Ingen lyd fra mikrofonen",
    microphone_silent_info: "Fant ikke noe lydsignal. Kontroller om AirPods-mikrofonen er tilgjengelig, eller bytt til den innebygde mikrofonen.",
    accessibility_required: "Tillatelse for tilgjengelighet kreves",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak kan ikke lime inn automatisk. Gi tillatelse i Systeminnstillinger → Personvern og sikkerhet → {setting}, og lim deretter inn manuelt med ⌘V."
        )
    },
    accessibility_list: "Tilgjengelighet",
    device_control_list: "Enhetskontroll og datatilgang",
    transcription_failed: "Transkriberingen mislyktes",
    transcribed_locally: "Transkribert på denne Macen",
    transcribed_locally_info: |reason| format!("{reason} En lokal modell transkriberte i stedet."),
    post_processing_failed: "Etterbehandlingen mislyktes",
    post_processing_failed_info: |reason| {
        format!("{reason} Teksten ble limt inn uten etterbehandling.")
    },
    no_api_key: "Ingen API-nøkkel",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Ingen API-nøkkel for {provider} er angitt. Legg til en via «{menu_item}» i menyen."
        )
    },
    login_item_error: "Feil med påloggingsobjekt",
    login_item_error_info: |menu_item| {
        format!(
            "Kunne ikke endre «{menu_item}». Prøv å slå det av eller på i Systeminnstillinger → Generelt → Påloggingsobjekter og utvidelser."
        )
    },
    api_key_error: "Feil med API-nøkkel",
    api_key_not_saved: "Kunne ikke arkivere API-nøkkelen. Prøv igjen.",
    api_keys_not_removed: "Kunne ikke fjerne API-nøklene. Prøv igjen.",
    download_failed: "Nedlastingen mislyktes",
    download_failed_info: |model| {
        format!("Kunne ikke laste ned {model}. Kontroller internettilkoblingen og prøv igjen.")
    },
    cannot_delete: "Kan ikke slette",
    cannot_delete_info: "Vent til den pågående handlingen er fullført før du sletter en modell.",
    delete_failed: "Slettingen mislyktes",
    delete_failed_info: |model| format!("Kunne ikke slette {model}. Prøv igjen."),

    no_api_key_error: |provider| {
        format!("Ingen API-nøkkel for {provider} er angitt. Angi den via menylinjen.")
    },
    invalid_response: "Ugyldig svar fra serveren.",
    authentication_failed: "API-autentiseringen mislyktes. Kontroller API-nøkkelen din.",
    rate_limited: "Grensen for antall forespørsler er nådd. Vent litt.",
    server_error: |status| format!("Serverfeil ({status}). Prøv igjen senere."),
    api_error: |status| format!("API-feil ({status}). Se loggen for detaljer."),
    model_not_available: |menu_item| {
        format!("Den lokale modellen er ikke tilgjengelig. Last den ned via «{menu_item}».")
    },
    network_error: |message| format!("Nettverksfeil: {message}"),
    key_rejected: |provider| format!("{provider} avviste API-nøkkelen."),
    model_retired: |provider| {
        format!("{provider} tilbyr ikke lenger modellen for etterbehandling.")
    },
    request_rejected: |provider, status| {
        format!("{provider} avviste forespørselen om etterbehandling ({status}).")
    },
    microphone_permission_denied: "Lowspeak har ikke tilgang til mikrofonen. Gi tillatelse i Systeminnstillinger → Personvern og sikkerhet → Mikrofon.",
    no_microphone: "Ingen mikrofon er tilkoblet.",
    microphone_error: |detail| format!("Kunne ikke starte mikrofonen: {detail}"),
    paste_failed: "Innliming mislyktes",
    paste_failed_info: "Teksten ligger på utklippstavlen. Lim den inn med ⌘V.",
    request_timed_out: "Forespørselen fikk tidsavbrudd.",
    audio_failed: "Opptaket kunne ikke behandles.",
    local_model_failed: "Den lokale modellen kunne ikke transkribere opptaket.",
};
