use super::Texts;

fn plural(n: usize, one: &'static str, few: &'static str, many: &'static str) -> &'static str {
    if n % 10 == 1 && n % 100 != 11 {
        one
    } else if (2..=4).contains(&(n % 10)) && !(12..=14).contains(&(n % 100)) {
        few
    } else {
        many
    }
}

pub static HR: Texts = Texts {
    recent_dictations: "Nedavni diktati",
    none_yet: "Još nema",
    retry_failed_dictation: "Ponovi neuspjeli diktat",
    local: "Lokalno",
    not_installed: |model| format!("{model} (nije instaliran)"),
    no_key: |provider| format!("{provider} (nema ključa)"),
    language: "Jezik",
    auto_detect: "Automatsko prepoznavanje",
    post_processing: "Naknadna obrada",
    post_processing_off: "Isključeno",
    post_processing_cleanup: "Čišćenje (poštapalice, gramatika)",
    post_processing_format: "Oblikovanje (prema kontekstu)",
    vocabulary_menu: |terms| match terms {
        0 => "Rječnik…".to_owned(),
        terms => format!(
            "Rječnik ({terms} {})…",
            plural(terms, "pojam", "pojma", "pojmova")
        ),
    },
    microphone: "Mikrofon",
    system_default: "Standardno podešenje sustava",
    not_connected: |microphone| format!("{microphone} (nije povezan)"),
    play_sound_when_ready: "Reproduciraj zvuk kad je spremno",
    manage_models: "Upravljaj modelima",
    delete_model: |model| format!("Obriši {model}"),
    download_model: |model| format!("Preuzmi {model}"),
    set_api_key: "Postavi API ključ…",
    update_api_key: "Promijeni API ključ…",
    open_at_login: "Otvori kod prijave",
    check_for_updates: "Potraži ažuriranja…",
    check_updates_automatically: "Automatski traži ažuriranja",
    install_updates_automatically: "Automatski preuzmi i instaliraj ažuriranja",
    help: "Pomoć",
    quick_reference: "Kratki pregled",
    report_problem: "Prijavi problem…",
    suggest_feature: "Predloži značajku…",
    on_github: "Lowspeak na GitHubu",
    about: "Više o aplikaciji Lowspeak",
    license: "Licenca: MIT ili Apache 2.0",
    quit: "Zatvori Lowspeak",

    reference_hold: "Držite za diktiranje, otpustite za lijepljenje",
    reference_double_tap: "Dvaput brzo pritisnite za diktiranje bez ruku",
    reference_shift: "Diktiranje bez ruku",
    reference_stop: "Završetak diktiranja bez ruku",
    reference_emoji: "Ako tipka 🌐 otvara Emoji i simbole, isključite to u postavkama tipkovnice.",
    keyboard_settings: "Postavke tipkovnice",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "engleski",
    languages: |count| format!("{count} {}", plural(count, "jezik", "jezika", "jezika")),
    legacy: "stariji model",
    fast: "brzi",

    set_api_key_title: "Postavi API ključ",
    update_api_key_title: "Promijeni API ključ",
    api_key_info: "Unesite svoj API ključ. Pružatelj usluge prepoznaje se automatski:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... ili gsk_...",
    save: "Spremi",
    cancel: "Poništi",
    ok: "OK",
    remove_all_keys: "Ukloni sve ključeve",
    unknown_key_format: "Nepoznat format ključa",
    unknown_key_format_info: "Očekuje se ključ koji počinje sa sk- (OpenAI) ili gsk_ (Groq). Pokušajte ponovno.",
    vocabulary_title: "Rječnik",
    vocabulary_info: "Imena i stručni pojmovi koji se trebaju pisati točno ovako, odvojeni zarezima. Koriste ih OpenAI, Groq, Whisper i Parakeet na ovom Macu te naknadna obrada.",

    recording_failed: "Snimanje nije uspjelo",
    microphone_silent: "Mikrofon bez signala",
    microphone_silent_info: "Nije otkriven zvučni signal. Provjerite je li dostupan mikrofon AirPods slušalica ili prijeđite na ugrađeni mikrofon.",
    accessibility_required: "Potrebna je dozvola za pristupačnost",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak ne može automatski zalijepiti tekst. Dodijelite dozvolu u Postavkama sustava → Privatnost i sigurnost → {setting}, a zatim tekst zalijepite ručno pomoću ⌘V."
        )
    },
    accessibility_list: "Pristupačnost",
    device_control_list: "Upravljanje uređajem i pristup podacima",
    transcription_failed: "Transkripcija nije uspjela",
    retry_failed_dictation_info: |menu_item| {
        format!(
            "Snimka je sačuvana: odaberite „{menu_item}” u izborniku da biste je ponovno transkribirali."
        )
    },
    transcribed_locally: "Transkribirano na ovom Macu",
    transcribed_locally_info: |reason| {
        format!("{reason} Umjesto toga transkribirao je lokalni model.")
    },
    post_processing_failed: "Naknadna obrada nije uspjela",
    post_processing_failed_info: |reason| {
        format!("{reason} Tekst je zalijepljen bez naknadne obrade.")
    },
    no_api_key: "Nema API ključa",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Nije postavljen API ključ za {provider}. Dodajte ga putem stavke „{menu_item}” u izborniku."
        )
    },
    login_item_error: "Pogreška stavke za prijavu",
    login_item_error_info: |menu_item| {
        format!(
            "Nije moguće promijeniti „{menu_item}”. Pokušajte to uključiti ili isključiti u Postavkama sustava → Općenito → Stavke za prijavu i ekstenzije."
        )
    },
    api_key_error: "Pogreška API ključa",
    api_key_not_saved: "Nije moguće spremiti API ključ. Pokušajte ponovno.",
    api_keys_not_removed: "Nije moguće ukloniti API ključeve. Pokušajte ponovno.",
    download_failed: "Preuzimanje nije uspjelo",
    download_failed_info: |model| {
        format!("Nije moguće preuzeti {model}. Provjerite internetsku vezu i pokušajte ponovno.")
    },
    cannot_delete: "Brisanje nije moguće",
    cannot_delete_info: "Prije brisanja modela pričekajte da završi trenutačna radnja.",
    delete_failed: "Brisanje nije uspjelo",
    delete_failed_info: |model| format!("Nije moguće izbrisati {model}. Pokušajte ponovno."),

    no_api_key_error: |provider| {
        format!("Nije postavljen API ključ za {provider}. Postavite ga u traci s izbornicima.")
    },
    invalid_response: "Nevažeći odgovor poslužitelja.",
    authentication_failed: "Provjera autentičnosti API-ja nije uspjela. Provjerite API ključ.",
    rate_limited: "Prekoračeno je ograničenje broja zahtjeva. Pričekajte trenutak.",
    server_error: |status| format!("Pogreška poslužitelja ({status}). Pokušajte ponovno kasnije."),
    api_error: |status| format!("Pogreška API-ja ({status}). Pojedinosti potražite u zapisniku."),
    model_not_available: |menu_item| {
        format!("Lokalni model nije dostupan. Preuzmite ga putem stavke „{menu_item}”.")
    },
    network_error: |message| format!("Pogreška mreže: {message}"),
    key_rejected: |provider| format!("Usluga {provider} odbila je API ključ."),
    quota_exhausted: |provider| {
        format!(
            "Usluga {provider} javlja da na računu više nema kredita. Dodajte kredit na svoj {provider} račun."
        )
    },
    model_retired: |provider| format!("Usluga {provider} više ne nudi model za naknadnu obradu."),
    no_post_processing_key: |first, second| {
        format!(
            "Za naknadnu obradu potreban je API ključ za {first} ili {second}. Postavite ga u traci s izbornicima."
        )
    },
    online_model_retired: |provider, model| {
        format!("Usluga {provider} više ne nudi {model}. Odaberite drugi model u izborniku.")
    },
    request_rejected: |provider, status| {
        format!("Usluga {provider} odbila je zahtjev za naknadnu obradu ({status}).")
    },
    microphone_permission_denied: "Lowspeak nema pristup mikrofonu. Dopustite pristup u Postavkama sustava → Privatnost i sigurnost → Mikrofon.",
    no_microphone: "Nije priključen nijedan mikrofon.",
    microphone_error: |detail| format!("Mikrofon nije moguće pokrenuti: {detail}"),
    paste_failed: "Lijepljenje nije uspjelo",
    paste_failed_info: "Tekst je u međuspremniku. Zalijepite ga pomoću ⌘V.",
    not_pasted: "Diktat nije zalijepljen",
    not_pasted_info: "Druga aplikacija došla je u prvi plan prije nego što je tekst bio spreman. Tekst je u međuspremniku. Zalijepite ga pomoću ⌘V.",
    request_timed_out: "Isteklo je vrijeme čekanja na zahtjev.",
    audio_failed: "Snimku nije moguće obraditi.",
    local_model_failed: "Lokalni model nije mogao transkribirati snimku.",
};
