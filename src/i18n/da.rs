use super::Texts;

pub static DA: Texts = Texts {
    recent_dictations: "Seneste dikteringer",
    none_yet: "Ingen endnu",
    retry_failed_dictation: "Prøv mislykket diktering igen",
    local: "Lokalt",
    not_installed: |model| format!("{model} (ikke installeret)"),
    no_key: |provider| format!("{provider} (ingen nøgle)"),
    language: "Sprog",
    auto_detect: "Registrer automatisk",
    post_processing: "Efterbehandling",
    post_processing_off: "Fra",
    post_processing_cleanup: "Ryd op (fyldord, grammatik)",
    post_processing_format: "Formater (efter kontekst)",
    vocabulary_menu: |terms| match terms {
        0 => "Ordliste…".to_owned(),
        1 => "Ordliste (1 term)…".to_owned(),
        terms => format!("Ordliste ({terms} termer)…"),
    },
    microphone: "Mikrofon",
    system_default: "Systemstandard",
    not_connected: |microphone| format!("{microphone} (ikke tilsluttet)"),
    play_sound_when_ready: "Afspil lyd, når klar",
    manage_models: "Administrer modeller",
    delete_model: |model| format!("Slet {model}"),
    download_model: |model| format!("Hent {model}"),
    set_api_key: "Indstil API-nøgle…",
    update_api_key: "Skift API-nøgle…",
    open_at_login: "Åbn ved login",
    check_for_updates: "Søg efter opdateringer…",
    check_updates_automatically: "Søg automatisk efter opdateringer",
    install_updates_automatically: "Hent og installer opdateringer automatisk",
    help: "Hjælp",
    quick_reference: "Hurtig oversigt",
    report_problem: "Rapporter et problem…",
    suggest_feature: "Foreslå en funktion…",
    on_github: "Lowspeak på GitHub",
    about: "Om Lowspeak",
    license: "Licens: MIT eller Apache 2.0",
    quit: "Slut Lowspeak",

    reference_hold: "Hold nede for at diktere, slip for at indsætte",
    reference_double_tap: "Tryk to gange hurtigt for at diktere håndfrit",
    reference_shift: "Diktér håndfrit",
    reference_stop: "Afslut håndfri diktering",
    reference_emoji: "Hvis 🌐-tasten åbner Emojis og symboler, kan du slå det fra i tastaturindstillingerne.",
    keyboard_settings: "Tastaturindstillinger",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "Engelsk",
    languages: |count| format!("{count} sprog"),
    legacy: "ældre model",
    fast: "hurtig",

    set_api_key_title: "Indstil API-nøgle",
    update_api_key_title: "Skift API-nøgle",
    api_key_info: "Indtast din API-nøgle. Udbyderen registreres automatisk:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... eller gsk_...",
    save: "Gem",
    cancel: "Annuller",
    ok: "OK",
    remove_all_keys: "Fjern alle nøgler",
    unknown_key_format: "Ukendt nøgleformat",
    unknown_key_format_info: "Nøglen skal starte med sk- (OpenAI) eller gsk_ (Groq). Prøv igen.",
    vocabulary_title: "Ordliste",
    vocabulary_info: "Navne og fagudtryk, der skal staves præcist, adskilt med komma. OpenAI, Groq, Whisper og Parakeet på denne Mac og efterbehandlingen bruger dem.",

    recording_failed: "Optagelse mislykkedes",
    microphone_silent: "Ingen lyd fra mikrofonen",
    microphone_silent_info: "Intet lydsignal registreret. Kontroller, om mikrofonen i dine AirPods er tilgængelig, eller skift til den indbyggede mikrofon.",
    accessibility_required: "Tilladelse til tilgængelighed kræves",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak kan ikke indsætte automatisk. Giv tilladelse i Systemindstillinger → Anonymitet & sikkerhed → {setting}, og indsæt derefter manuelt med ⌘V."
        )
    },
    accessibility_list: "Tilgængelighed",
    device_control_list: "Styring af enhed og adgang til data",
    transcription_failed: "Transskription mislykkedes",
    retry_failed_dictation_info: |menu_item| {
        format!("Optagelsen er gemt: Vælg ”{menu_item}” i menuen for at transskribere den igen.")
    },
    transcribed_locally: "Transskriberet på denne Mac",
    transcribed_locally_info: |reason| format!("{reason} En lokal model transskriberede i stedet."),
    post_processing_failed: "Efterbehandling mislykkedes",
    post_processing_failed_info: |reason| {
        format!("{reason} Teksten blev indsat uden efterbehandling.")
    },
    no_api_key: "Ingen API-nøgle",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Der er ikke angivet nogen API-nøgle til {provider}. Tilføj en via ”{menu_item}” i menuen."
        )
    },
    login_item_error: "Fejl ved loginemne",
    login_item_error_info: |menu_item| {
        format!(
            "Kunne ikke ændre ”{menu_item}”. Prøv at slå det til eller fra i Systemindstillinger → Generelt → Loginemner & udvidelser."
        )
    },
    api_key_error: "Fejl ved API-nøgle",
    api_key_not_saved: "API-nøglen kunne ikke gemmes. Prøv igen.",
    api_keys_not_removed: "API-nøglerne kunne ikke fjernes. Prøv igen.",
    download_failed: "Hentning mislykkedes",
    download_failed_info: |model| {
        format!("{model} kunne ikke hentes. Kontroller internetforbindelsen, og prøv igen.")
    },
    cannot_delete: "Kan ikke slette",
    cannot_delete_info: "Vent, til den igangværende handling er færdig, før du sletter en model.",
    delete_failed: "Sletning mislykkedes",
    delete_failed_info: |model| format!("{model} kunne ikke slettes. Prøv igen."),

    no_api_key_error: |provider| {
        format!("Der er ikke angivet nogen API-nøgle til {provider}. Indstil den via menulinjen.")
    },
    invalid_response: "Ugyldigt svar fra serveren.",
    authentication_failed: "API-godkendelse mislykkedes. Kontroller din API-nøgle.",
    rate_limited: "Grænsen for antal forespørgsler er nået. Vent et øjeblik.",
    server_error: |status| format!("Serverfejl ({status}). Prøv igen senere."),
    api_error: |status| format!("API-fejl ({status}). Se loggen for detaljer."),
    model_not_available: |menu_item| {
        format!("Den lokale model er ikke tilgængelig. Hent den via ”{menu_item}”.")
    },
    network_error: |message| format!("Netværksfejl: {message}"),
    key_rejected: |provider| format!("{provider} afviste API-nøglen."),
    quota_exhausted: |provider| {
        format!(
            "{provider} oplyser, at kontoen ikke har mere kredit. Tilføj kredit på din {provider}-konto."
        )
    },
    model_retired: |provider| {
        format!("{provider} tilbyder ikke længere modellen til efterbehandling.")
    },
    no_post_processing_key: |first, second| {
        format!(
            "Efterbehandling kræver en API-nøgle fra {first} eller {second}. Indstil den via menulinjen."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} tilbyder ikke længere {model}. Vælg en anden model i menuen.")
    },
    request_rejected: |provider, status| {
        format!("{provider} afviste anmodningen om efterbehandling ({status}).")
    },
    microphone_permission_denied: "Lowspeak har ikke adgang til mikrofonen. Giv tilladelse i Systemindstillinger → Anonymitet & sikkerhed → Mikrofon.",
    no_microphone: "Der er ikke tilsluttet nogen mikrofon.",
    microphone_error: |detail| format!("Mikrofonen kunne ikke startes: {detail}"),
    paste_failed: "Indsætning mislykkedes",
    paste_failed_info: "Teksten ligger i udklipsholderen. Indsæt den med ⌘V.",
    not_pasted: "Dikteringen blev ikke indsat",
    not_pasted_info: "En anden app kom i forgrunden, før teksten var klar. Teksten ligger i udklipsholderen. Indsæt den med ⌘V.",
    request_timed_out: "Anmodningen fik timeout.",
    audio_failed: "Optagelsen kunne ikke behandles.",
    local_model_failed: "Den lokale model kunne ikke transskribere optagelsen.",
};
