use super::Texts;

fn plural(n: usize, one: &'static str, few: &'static str, other: &'static str) -> &'static str {
    match n {
        1 => one,
        2..=4 => few,
        _ => other,
    }
}

pub static SK: Texts = Texts {
    recent_dictations: "Nedávne diktáty",
    none_yet: "Zatiaľ žiadne",
    local: "Lokálne",
    not_installed: |model| format!("{model} (nenainštalované)"),
    no_key: |provider| format!("{provider} (bez kľúča)"),
    language: "Jazyk",
    auto_detect: "Rozpoznať automaticky",
    post_processing: "Následné spracovanie",
    post_processing_off: "Vypnuté",
    post_processing_cleanup: "Vyčistenie (výplňové slová, gramatika)",
    post_processing_format: "Formátovanie (podľa kontextu)",
    vocabulary_menu: |terms| match terms {
        0 => "Slovník…".to_owned(),
        terms => format!(
            "Slovník ({terms} {})…",
            plural(terms, "výraz", "výrazy", "výrazov")
        ),
    },
    microphone: "Mikrofón",
    system_default: "Systémová predvoľba",
    not_connected: |microphone| format!("{microphone} (nepripojené)"),
    play_sound_when_ready: "Prehrať zvuk po pripravení",
    manage_models: "Spravovať modely",
    delete_model: |model| format!("Vymazať {model}"),
    download_model: |model| format!("Stiahnuť {model}"),
    set_api_key: "Nastaviť kľúč API…",
    update_api_key: "Zmeniť kľúč API…",
    open_at_login: "Otvoriť po prihlásení",
    check_for_updates: "Vyhľadať aktualizácie…",
    check_updates_automatically: "Automaticky vyhľadávať aktualizácie",
    install_updates_automatically: "Automaticky sťahovať a inštalovať aktualizácie",
    help: "Pomocník",
    quick_reference: "Stručný prehľad",
    report_problem: "Nahlásiť problém…",
    suggest_feature: "Navrhnúť funkciu…",
    on_github: "Lowspeak na GitHube",
    about: "O aplikácii Lowspeak",
    license: "Licencia: MIT alebo Apache 2.0",
    quit: "Ukončiť Lowspeak",

    reference_hold: "Podržaním diktujete, uvoľnením vložíte text",
    reference_double_tap: "Dvojitým rýchlym stlačením diktujete bez rúk",
    reference_shift: "Diktovať bez rúk",
    reference_stop: "Ukončiť diktovanie bez rúk",
    reference_emoji: "Ak kláves 🌐 otvára Emoji a symboly, vypnite to v nastaveniach klávesnice.",
    keyboard_settings: "Nastavenia klávesnice",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "angličtina",
    languages: |count| format!("{count} {}", plural(count, "jazyk", "jazyky", "jazykov")),
    legacy: "starší model",
    fast: "rýchly",

    set_api_key_title: "Nastaviť kľúč API",
    update_api_key_title: "Zmeniť kľúč API",
    api_key_info: "Zadajte kľúč API. Služba sa rozpozná automaticky:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... alebo gsk_...",
    save: "Uložiť",
    cancel: "Zrušiť",
    ok: "Potvrdiť",
    remove_all_keys: "Odstrániť všetky kľúče",
    unknown_key_format: "Neznámy formát kľúča",
    unknown_key_format_info: "Očakáva sa kľúč začínajúci na sk- (OpenAI) alebo gsk_ (Groq). Skúste to znova.",
    vocabulary_title: "Slovník",
    vocabulary_info: "Mená a odborné výrazy, ktoré sa majú písať presne takto, oddelené čiarkami. Používajú ich OpenAI, Groq, Whisper na tomto Macu a následné spracovanie.",

    recording_failed: "Nahrávanie zlyhalo",
    microphone_silent: "Mikrofón bez signálu",
    microphone_silent_info: "Nezistil sa žiadny zvukový signál. Skontrolujte, či je mikrofón AirPods dostupný, alebo prepnite na vstavaný mikrofón.",
    accessibility_required: "Vyžaduje sa povolenie na prístupnosť",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak nemôže vkladať automaticky. Udeľte povolenie v Systémových nastaveniach → Súkromie a bezpečnosť → {setting} a potom vložte text ručne pomocou ⌘V."
        )
    },
    accessibility_list: "Prístupnosť",
    device_control_list: "Ovládanie zariadenia a prístup k dátam",
    transcription_failed: "Prepis zlyhal",
    transcribed_locally: "Prepísané na tomto Macu",
    transcribed_locally_info: |reason| {
        format!("{reason} Prepis namiesto toho vykonal lokálny model.")
    },
    post_processing_failed: "Následné spracovanie zlyhalo",
    post_processing_failed_info: |reason| {
        format!("{reason} Text bol vložený bez následného spracovania.")
    },
    no_api_key: "Chýba kľúč API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Pre {provider} nie je nastavený kľúč API. Pridajte ho cez položku „{menu_item}“ v menu."
        )
    },
    login_item_error: "Chyba otvárania po prihlásení",
    login_item_error_info: |menu_item| {
        format!(
            "Voľbu „{menu_item}“ sa nepodarilo zmeniť. Skúste ju prepnúť v Systémových nastaveniach → Všeobecné → Položky a rozšírenia spúšťané pri prihlásení."
        )
    },
    api_key_error: "Chyba kľúča API",
    api_key_not_saved: "Kľúč API sa nepodarilo uložiť. Skúste to znova.",
    api_keys_not_removed: "Kľúče API sa nepodarilo odstrániť. Skúste to znova.",
    download_failed: "Sťahovanie zlyhalo",
    download_failed_info: |model| {
        format!(
            "Model {model} sa nepodarilo stiahnuť. Skontrolujte pripojenie na internet a skúste to znova."
        )
    },
    cannot_delete: "Nedá sa vymazať",
    cannot_delete_info: "Pred vymazaním modelu počkajte na dokončenie prebiehajúcej operácie.",
    delete_failed: "Vymazanie zlyhalo",
    delete_failed_info: |model| format!("Model {model} sa nepodarilo vymazať. Skúste to znova."),

    no_api_key_error: |provider| {
        format!("Pre {provider} nie je nastavený kľúč API. Nastavte ho v lište menu.")
    },
    invalid_response: "Neplatná odpoveď servera.",
    authentication_failed: "Overenie API zlyhalo. Skontrolujte kľúč API.",
    rate_limited: "Prekročil sa limit požiadaviek. Chvíľu počkajte.",
    server_error: |status| format!("Chyba servera ({status}). Skúste to neskôr."),
    api_error: |status| format!("Chyba API ({status}). Podrobnosti nájdete v protokole."),
    model_not_available: |menu_item| {
        format!("Lokálny model nie je k dispozícii. Stiahnite ho cez „{menu_item}“.")
    },
    network_error: |message| format!("Chyba siete: {message}"),
    key_rejected: |provider| format!("Služba {provider} odmietla kľúč API."),
    model_retired: |provider| {
        format!("Služba {provider} už neponúka model na následné spracovanie.")
    },
    request_rejected: |provider, status| {
        format!("Služba {provider} odmietla požiadavku na následné spracovanie ({status}).")
    },
    microphone_permission_denied: "Lowspeak nemá prístup k mikrofónu. Povoľte ho v Systémových nastaveniach → Súkromie a bezpečnosť → Mikrofón.",
    no_microphone: "Nie je pripojený žiadny mikrofón.",
    microphone_error: |detail| format!("Mikrofón sa nepodarilo spustiť: {detail}"),
    paste_failed: "Vloženie zlyhalo",
    paste_failed_info: "Text je v schránke. Vložte ho pomocou ⌘V.",
    request_timed_out: "Časový limit požiadavky vypršal.",
    audio_failed: "Nahrávku sa nepodarilo spracovať.",
    local_model_failed: "Lokálny model nedokázal nahrávku prepísať.",
};
