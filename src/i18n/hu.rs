use super::Texts;

pub static HU: Texts = Texts {
    recent_dictations: "Legutóbbi diktálások",
    none_yet: "Még nincs",
    local: "Helyi",
    not_installed: |model| format!("{model} (nincs telepítve)"),
    no_key: |provider| format!("{provider} (nincs kulcs)"),
    language: "Nyelv",
    auto_detect: "Automatikus felismerés",
    post_processing: "Utófeldolgozás",
    post_processing_off: "Ki",
    post_processing_cleanup: "Tisztítás (töltelékszavak, nyelvtan)",
    post_processing_format: "Formázás (környezetfüggő)",
    vocabulary_menu: |terms| match terms {
        0 => "Szókincs…".to_owned(),
        terms => format!("Szókincs ({terms} kifejezés)…"),
    },
    microphone: "Mikrofon",
    system_default: "Rendszer alapértelmezése",
    not_connected: |microphone| format!("{microphone} (nincs csatlakoztatva)"),
    play_sound_when_ready: "Hangjelzés, ha kész",
    manage_models: "Modellek kezelése",
    delete_model: |model| format!("{model} törlése"),
    download_model: |model| format!("{model} letöltése"),
    set_api_key: "API-kulcs megadása…",
    update_api_key: "API-kulcs módosítása…",
    open_at_login: "Megnyitás bejelentkezéskor",
    check_for_updates: "Frissítések keresése…",
    check_updates_automatically: "Frissítések automatikus keresése",
    install_updates_automatically: "Frissítések automatikus letöltése és telepítése",
    help: "Súgó",
    quick_reference: "Gyors áttekintés",
    report_problem: "Probléma jelentése…",
    suggest_feature: "Funkció javaslása…",
    on_github: "A Lowspeak a GitHubon",
    about: "A Lowspeak névjegye",
    license: "Licenc: MIT vagy Apache 2.0",
    quit: "Lowspeak bezárása",

    reference_hold: "Lenyomva tartva diktálhat, felengedve beilleszti a szöveget",
    reference_double_tap: "Kétszer gyorsan lenyomva kéz nélkül diktálhat",
    reference_shift: "Diktálás kéz nélkül",
    reference_stop: "Kéz nélküli diktálás befejezése",
    reference_emoji: "Ha a 🌐 billentyű az Emojik és szimbólumok ablakot nyitja meg, kapcsolja ki ezt a billentyűzetbeállításokban.",
    keyboard_settings: "Billentyűzetbeállítások",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "angol",
    languages: |count| format!("{count} nyelv"),
    legacy: "régebbi modell",
    fast: "gyors",

    set_api_key_title: "API-kulcs megadása",
    update_api_key_title: "API-kulcs módosítása",
    api_key_info: "Adja meg az API-kulcsot. A szolgáltató felismerése automatikus:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... vagy gsk_...",
    save: "Mentés",
    cancel: "Mégsem",
    ok: "OK",
    remove_all_keys: "Összes kulcs eltávolítása",
    unknown_key_format: "Ismeretlen kulcsformátum",
    unknown_key_format_info: "A kulcsnak sk- (OpenAI) vagy gsk_ (Groq) előtaggal kell kezdődnie. Próbálja újra.",
    vocabulary_title: "Szókincs",
    vocabulary_info: "Pontosan így írandó nevek és szakkifejezések, vesszővel elválasztva. Az OpenAI, a Groq, a Whisper ezen a Macen és az utófeldolgozás használja őket.",

    recording_failed: "A felvétel nem sikerült",
    microphone_silent: "A mikrofon néma",
    microphone_silent_info: "Nem érkezik hangjel. Ellenőrizze, hogy elérhető-e az AirPods mikrofonja, vagy váltson a beépített mikrofonra.",
    accessibility_required: "Engedély szükséges a kisegítő lehetőségekhez",
    accessibility_required_info: |setting| {
        format!(
            "A Lowspeak nem tud automatikusan beilleszteni. Adjon engedélyt itt: Rendszerbeállítások → Adatvédelem és biztonság → {setting}, majd illessze be a szöveget kézzel a ⌘V billentyűparanccsal."
        )
    },
    accessibility_list: "Kisegítő lehetőségek",
    device_control_list: "Eszközvezérlés és adathozzáférés",
    transcription_failed: "Az átírás nem sikerült",
    transcribed_locally: "Átírva ezen a Macen",
    transcribed_locally_info: |reason| {
        format!("{reason} Helyette egy helyi modell végezte az átírást.")
    },
    post_processing_failed: "Az utófeldolgozás nem sikerült",
    post_processing_failed_info: |reason| {
        format!("{reason} A Lowspeak utófeldolgozás nélkül illesztette be a szöveget.")
    },
    no_api_key: "Nincs API-kulcs",
    no_api_key_info: |provider, menu_item| {
        format!("Nincs megadva {provider} API-kulcs. Adja meg a menü „{menu_item}” pontjában.")
    },
    login_item_error: "Indítóelem-hiba",
    login_item_error_info: |menu_item| {
        format!(
            "Nem sikerült módosítani a menü „{menu_item}” pontját. Próbálja meg átkapcsolni itt: Rendszerbeállítások → Általános → Indítóelemek és bővítmények."
        )
    },
    api_key_error: "API-kulcshiba",
    api_key_not_saved: "Nem sikerült menteni az API-kulcsot. Próbálja újra.",
    api_keys_not_removed: "Nem sikerült eltávolítani az API-kulcsokat. Próbálja újra.",
    download_failed: "A letöltés nem sikerült",
    download_failed_info: |model| {
        format!(
            "Nem sikerült letölteni a modellt ({model}). Ellenőrizze az internetkapcsolatot, és próbálja újra."
        )
    },
    cannot_delete: "Nem törölhető",
    cannot_delete_info: "Modell törlése előtt várja meg, amíg az aktuális művelet befejeződik.",
    delete_failed: "A törlés nem sikerült",
    delete_failed_info: |model| format!("Nem sikerült törölni a modellt ({model}). Próbálja újra."),

    no_api_key_error: |provider| {
        format!("Nincs megadva {provider} API-kulcs. Adja meg a menüsoron.")
    },
    invalid_response: "Érvénytelen válasz a szervertől.",
    authentication_failed: "Sikertelen API-hitelesítés. Ellenőrizze az API-kulcsot.",
    rate_limited: "Túl sok kérés. Várjon egy kicsit.",
    server_error: |status| format!("Szerverhiba ({status}). Próbálja újra később."),
    api_error: |status| format!("API-hiba ({status}). A részletek a naplóban találhatók."),
    model_not_available: |menu_item| {
        format!("A helyi modell nem érhető el. Töltse le a menü „{menu_item}” pontjában.")
    },
    network_error: |message| format!("Hálózati hiba: {message}"),
    key_rejected: |provider| format!("A szolgáltató ({provider}) elutasította az API-kulcsot."),
    model_retired: |provider| {
        format!("A szolgáltató ({provider}) már nem kínálja az utófeldolgozási modellt.")
    },
    no_post_processing_key: |first, second| {
        format!("Az utófeldolgozáshoz {first} vagy {second} API-kulcs kell. Adja meg a menüsoron.")
    },
    online_model_retired: |provider, model| {
        format!(
            "A szolgáltató ({provider}) már nem kínálja ezt a modellt: {model}. Válasszon másikat a menüben."
        )
    },
    request_rejected: |provider, status| {
        format!("A szolgáltató ({provider}) elutasította az utófeldolgozási kérést ({status}).")
    },
    microphone_permission_denied: "A Lowspeak nem fér hozzá a mikrofonhoz. Engedélyezze itt: Rendszerbeállítások → Adatvédelem és biztonság → Mikrofon.",
    no_microphone: "Nincs csatlakoztatott mikrofon.",
    microphone_error: |detail| format!("Nem sikerült elindítani a mikrofont: {detail}"),
    paste_failed: "A beillesztés nem sikerült",
    paste_failed_info: "A szöveg a vágólapon van. Illessze be a ⌘V billentyűparanccsal.",
    request_timed_out: "A kérés túllépte az időkorlátot.",
    audio_failed: "A felvételt nem sikerült feldolgozni.",
    local_model_failed: "A helyi modell nem tudta átírni a felvételt.",
};
