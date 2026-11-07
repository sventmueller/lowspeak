use super::Texts;

fn plural(n: usize, one: &'static str, few: &'static str, other: &'static str) -> &'static str {
    match n {
        1 => one,
        2..=4 => few,
        _ => other,
    }
}

pub static CS: Texts = Texts {
    recent_dictations: "Poslední diktáty",
    none_yet: "Zatím žádné",
    local: "Místní",
    not_installed: |model| format!("{model} (nenainstalováno)"),
    no_key: |provider| format!("{provider} (bez klíče)"),
    language: "Jazyk",
    auto_detect: "Rozpoznat automaticky",
    post_processing: "Následné zpracování",
    post_processing_off: "Vypnuto",
    post_processing_cleanup: "Vyčištění (vycpávková slova, gramatika)",
    post_processing_format: "Formátování (podle kontextu)",
    vocabulary_menu: |terms| match terms {
        0 => "Slovník…".to_owned(),
        terms => format!(
            "Slovník ({terms} {})…",
            plural(terms, "výraz", "výrazy", "výrazů")
        ),
    },
    microphone: "Mikrofon",
    system_default: "Výchozí systémový",
    not_connected: |microphone| format!("{microphone} (nepřipojeno)"),
    play_sound_when_ready: "Zvuk při připravenosti",
    manage_models: "Spravovat modely",
    delete_model: |model| format!("Smazat {model}"),
    download_model: |model| format!("Stáhnout {model}"),
    set_api_key: "Nastavit klíč API…",
    update_api_key: "Změnit klíč API…",
    open_at_login: "Otevřít při přihlášení",
    check_for_updates: "Vyhledat aktualizace…",
    check_updates_automatically: "Vyhledávat aktualizace automaticky",
    install_updates_automatically: "Stahovat a instalovat aktualizace automaticky",
    help: "Nápověda",
    quick_reference: "Stručný přehled",
    report_problem: "Nahlásit problém…",
    suggest_feature: "Navrhnout funkci…",
    on_github: "Lowspeak na GitHubu",
    about: "O aplikaci Lowspeak",
    license: "Licence: MIT nebo Apache 2.0",
    quit: "Ukončit Lowspeak",

    reference_hold: "Podržením diktujete, uvolněním vložíte text",
    reference_double_tap: "Dvojím rychlým stisknutím diktujete bez rukou",
    reference_shift: "Diktovat bez rukou",
    reference_stop: "Ukončit diktování bez rukou",
    reference_emoji: "Pokud klávesa 🌐 otevírá Emotikony a symboly, vypněte to v nastavení klávesnice.",
    keyboard_settings: "Nastavení klávesnice",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "angličtina",
    languages: |count| format!("{count} {}", plural(count, "jazyk", "jazyky", "jazyků")),
    legacy: "starší model",
    fast: "rychlý",

    set_api_key_title: "Nastavit klíč API",
    update_api_key_title: "Změnit klíč API",
    api_key_info: "Zadejte klíč API. Služba se rozpozná automaticky:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... nebo gsk_...",
    save: "Uložit",
    cancel: "Zrušit",
    ok: "OK",
    remove_all_keys: "Odstranit všechny klíče",
    unknown_key_format: "Neznámý formát klíče",
    unknown_key_format_info: "Očekává se klíč začínající sk- (OpenAI) nebo gsk_ (Groq). Zkuste to znovu.",
    vocabulary_title: "Slovník",
    vocabulary_info: "Vlastní jména a odborné výrazy, které se mají psát přesně takto, oddělené čárkami. Používají je OpenAI, Groq, Whisper na tomto Macu a následné zpracování.",

    recording_failed: "Nahrávání se nezdařilo",
    microphone_silent: "Mikrofon bez signálu",
    microphone_silent_info: "Nebyl zjištěn žádný zvukový signál. Zkontrolujte, zda je mikrofon AirPods dostupný, nebo přepněte na vestavěný mikrofon.",
    accessibility_required: "Je vyžadováno oprávnění pro zpřístupnění",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak nemůže vkládat automaticky. Udělte oprávnění v Nastavení systému → Soukromí a zabezpečení → {setting} a potom vložte text ručně pomocí ⌘V."
        )
    },
    accessibility_list: "Zpřístupnění",
    device_control_list: "Ovládání zařízení a přístup k datům",
    transcription_failed: "Přepis se nezdařil",
    transcribed_locally: "Přepsáno na tomto Macu",
    transcribed_locally_info: |reason| format!("{reason} Přepis místo toho provedl místní model."),
    post_processing_failed: "Následné zpracování se nezdařilo",
    post_processing_failed_info: |reason| {
        format!("{reason} Text byl vložen bez následného zpracování.")
    },
    no_api_key: "Chybí klíč API",
    no_api_key_info: |provider, menu_item| {
        format!("Pro {provider} není nastaven klíč API. Přidejte ho v nabídce přes „{menu_item}“.")
    },
    login_item_error: "Chyba spouštění při přihlášení",
    login_item_error_info: |menu_item| {
        format!(
            "Volbu „{menu_item}“ se nepodařilo změnit. Zkuste ji přepnout v Nastavení systému → Obecné → Aktivace položek a rozšíření při přihlášení."
        )
    },
    api_key_error: "Chyba klíče API",
    api_key_not_saved: "Klíč API se nepodařilo uložit. Zkuste to znovu.",
    api_keys_not_removed: "Klíče API se nepodařilo odstranit. Zkuste to znovu.",
    download_failed: "Stahování se nezdařilo",
    download_failed_info: |model| {
        format!(
            "Model {model} se nepodařilo stáhnout. Zkontrolujte připojení k internetu a zkuste to znovu."
        )
    },
    cannot_delete: "Nelze smazat",
    cannot_delete_info: "Před smazáním modelu počkejte na dokončení probíhající operace.",
    delete_failed: "Smazání se nezdařilo",
    delete_failed_info: |model| format!("Model {model} se nepodařilo smazat. Zkuste to znovu."),

    no_api_key_error: |provider| {
        format!("Pro {provider} není nastaven klíč API. Nastavte ho na řádku nabídek.")
    },
    invalid_response: "Neplatná odpověď serveru.",
    authentication_failed: "Ověření API se nezdařilo. Zkontrolujte klíč API.",
    rate_limited: "Překročen limit požadavků. Chvíli počkejte.",
    server_error: |status| format!("Chyba serveru ({status}). Zkuste to později."),
    api_error: |status| format!("Chyba API ({status}). Podrobnosti najdete v protokolu."),
    model_not_available: |menu_item| {
        format!("Místní model není k dispozici. Stáhněte ho přes „{menu_item}“.")
    },
    network_error: |message| format!("Chyba sítě: {message}"),
    key_rejected: |provider| format!("Služba {provider} odmítla klíč API."),
    model_retired: |provider| {
        format!("Služba {provider} už nenabízí model pro následné zpracování.")
    },
    request_rejected: |provider, status| {
        format!("Služba {provider} odmítla požadavek na následné zpracování ({status}).")
    },
    microphone_permission_denied: "Lowspeak nemá přístup k mikrofonu. Povolte ho v Nastavení systému → Soukromí a zabezpečení → Mikrofon.",
    no_microphone: "Není připojen žádný mikrofon.",
    microphone_error: |detail| format!("Mikrofon se nepodařilo spustit: {detail}"),
    paste_failed: "Vložení se nezdařilo",
    paste_failed_info: "Text je ve schránce. Vložte ho pomocí ⌘V.",
    request_timed_out: "Časový limit požadavku vypršel.",
    audio_failed: "Nahrávku se nepodařilo zpracovat.",
    local_model_failed: "Místní model nedokázal nahrávku přepsat.",
};
