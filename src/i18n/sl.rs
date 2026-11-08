use super::Texts;

fn plural(n: usize, [one, two, few, other]: [&'static str; 4]) -> &'static str {
    match n % 100 {
        1 => one,
        2 => two,
        3 | 4 => few,
        _ => other,
    }
}

pub static SL: Texts = Texts {
    recent_dictations: "Nedavni nareki",
    none_yet: "Še nobenega",
    retry_failed_dictation: "Znova poskusi neuspeli narek",
    local: "Lokalno",
    not_installed: |model| format!("{model} (ni nameščen)"),
    no_key: |provider| format!("{provider} (brez ključa)"),
    language: "Jezik",
    auto_detect: "Samodejno zaznaj",
    post_processing: "Naknadna obdelava",
    post_processing_off: "Izklopljeno",
    post_processing_cleanup: "Čiščenje (mašila, slovnica)",
    post_processing_format: "Oblikovanje (glede na kontekst)",
    vocabulary_menu: |terms| match terms {
        0 => "Besedišče…".to_owned(),
        terms => format!(
            "Besedišče ({terms} {})…",
            plural(terms, ["izraz", "izraza", "izrazi", "izrazov"])
        ),
    },
    microphone: "Mikrofon",
    system_default: "Privzeto sistemsko",
    not_connected: |microphone| format!("{microphone} (ni povezan)"),
    play_sound_when_ready: "Predvajaj zvok ob pripravljenosti",
    manage_models: "Upravljaj modele",
    delete_model: |model| format!("Izbriši {model}"),
    download_model: |model| format!("Prenesi {model}"),
    set_api_key: "Nastavi ključ API…",
    update_api_key: "Posodobi ključ API…",
    open_at_login: "Odpiranje ob prijavi",
    check_for_updates: "Preveri, ali so na voljo posodobitve…",
    check_updates_automatically: "Samodejno preverjaj, ali so na voljo posodobitve",
    install_updates_automatically: "Samodejno prenesi in namesti posodobitve",
    help: "Pomoč",
    quick_reference: "Hitri pregled",
    report_problem: "Prijavi težavo…",
    suggest_feature: "Predlagaj funkcijo…",
    on_github: "Lowspeak na GitHubu",
    about: "O aplikaciji Lowspeak",
    license: "Licenca: MIT ali Apache 2.0",
    quit: "Zapri aplikacijo Lowspeak",

    reference_hold: "Držite za narekovanje, spustite za lepljenje",
    reference_double_tap: "Dvakrat hitro pritisnite za prostoročno narekovanje",
    reference_shift: "Prostoročno narekovanje",
    reference_stop: "Konec prostoročnega narekovanja",
    reference_emoji: "Če tipka 🌐 odpre »Emojiji in simboli«, to izklopite v nastavitvah tipkovnice.",
    keyboard_settings: "Nastavitve tipkovnice",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "angleščina",
    languages: |count| {
        format!(
            "{count} {}",
            plural(count, ["jezik", "jezika", "jeziki", "jezikov"])
        )
    },
    legacy: "starejši model",
    fast: "hiter",

    set_api_key_title: "Nastavi ključ API",
    update_api_key_title: "Posodobi ključ API",
    api_key_info: "Vnesite ključ API. Ponudnik je zaznan samodejno:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... ali gsk_...",
    save: "Shrani",
    cancel: "Prekliči",
    ok: "V redu",
    remove_all_keys: "Odstrani vse ključe",
    unknown_key_format: "Neznana oblika ključa",
    unknown_key_format_info: "Pričakovan je ključ, ki se začne s sk- (OpenAI) ali z gsk_ (Groq). Poskusite znova.",
    vocabulary_title: "Besedišče",
    vocabulary_info: "Imena in strokovni izrazi, ki naj bodo zapisani natanko tako, ločeni z vejicami. Uporabljajo jih OpenAI, Groq, Whisper in Parakeet na tem Macu ter naknadna obdelava.",

    recording_failed: "Snemanje ni uspelo",
    microphone_silent: "Mikrofon brez signala",
    microphone_silent_info: "Zvočni signal ni bil zaznan. Preverite, ali je mikrofon slušalk AirPods na voljo, ali preklopite na vgrajeni mikrofon.",
    accessibility_required: "Potrebno je dovoljenje za dostopnost",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak ne more samodejno prilepiti besedila. Dovoljenje podelite v Nastavitvah sistema → Zasebnost in varnost → {setting}, nato besedilo prilepite ročno s ⌘V."
        )
    },
    accessibility_list: "Dostopnost",
    device_control_list: "Upravljanje naprave in dostop do podatkov",
    transcription_failed: "Prepis ni uspel",
    retry_failed_dictation_info: |menu_item| {
        format!("Posnetek je shranjen: v meniju izberite »{menu_item}«, da ga znova prepišete.")
    },
    transcribed_locally: "Prepisano na tem Macu",
    transcribed_locally_info: |reason| format!("{reason} Namesto tega je prepisal lokalni model."),
    post_processing_failed: "Naknadna obdelava ni uspela",
    post_processing_failed_info: |reason| {
        format!("{reason} Besedilo je bilo prilepljeno brez naknadne obdelave.")
    },
    no_api_key: "Ni ključa API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Ključ API za {provider} ni nastavljen. Dodajte ga z ukazom »{menu_item}« v meniju."
        )
    },
    login_item_error: "Napaka elementa za prijavo",
    login_item_error_info: |menu_item| {
        format!(
            "Možnosti »{menu_item}« ni bilo mogoče posodobiti. Poskusite jo preklopiti v Nastavitvah sistema → Splošno → Elementi prijave in razširitve."
        )
    },
    api_key_error: "Napaka ključa API",
    api_key_not_saved: "Ključa API ni bilo mogoče shraniti. Poskusite znova.",
    api_keys_not_removed: "Ključev API ni bilo mogoče odstraniti. Poskusite znova.",
    download_failed: "Prenos ni uspel",
    download_failed_info: |model| {
        format!(
            "Modela {model} ni bilo mogoče prenesti. Preverite internetno povezavo in poskusite znova."
        )
    },
    cannot_delete: "Brisanje ni mogoče",
    cannot_delete_info: "Pred brisanjem modela počakajte, da se trenutno opravilo konča.",
    delete_failed: "Brisanje ni uspelo",
    delete_failed_info: |model| {
        format!("Modela {model} ni bilo mogoče izbrisati. Poskusite znova.")
    },

    no_api_key_error: |provider| {
        format!("Ključ API za {provider} ni nastavljen. Nastavite ga v menijski vrstici.")
    },
    invalid_response: "Neveljaven odziv strežnika.",
    authentication_failed: "Preverjanje pristnosti API ni uspelo. Preverite ključ API.",
    rate_limited: "Omejitev zahtev je presežena. Počakajte trenutek.",
    server_error: |status| format!("Napaka strežnika ({status}). Poskusite znova pozneje."),
    api_error: |status| format!("Napaka API ({status}). Podrobnosti so v dnevniku."),
    model_not_available: |menu_item| {
        format!("Lokalni model ni na voljo. Prenesite ga v meniju »{menu_item}«.")
    },
    network_error: |message| format!("Omrežna napaka: {message}"),
    key_rejected: |provider| format!("Storitev {provider} je zavrnila ključ API."),
    quota_exhausted: |provider| {
        format!(
            "Storitev {provider} sporoča, da na računu ni več dobroimetja. Dobroimetje dodajte v svojem računu {provider}."
        )
    },
    model_retired: |provider| {
        format!("Storitev {provider} ne ponuja več modela za naknadno obdelavo.")
    },
    no_post_processing_key: |first, second| {
        format!(
            "Za naknadno obdelavo potrebujete ključ API za {first} ali {second}. Nastavite ga v menijski vrstici."
        )
    },
    online_model_retired: |provider, model| {
        format!("Storitev {provider} ne ponuja več modela {model}. V meniju izberite drug model.")
    },
    request_rejected: |provider, status| {
        format!("Storitev {provider} je zavrnila zahtevo za naknadno obdelavo ({status}).")
    },
    microphone_permission_denied: "Lowspeak nima dostopa do mikrofona. Dovolite ga v Nastavitvah sistema → Zasebnost in varnost → Mikrofon.",
    no_microphone: "Priključen ni noben mikrofon.",
    microphone_error: |detail| format!("Mikrofona ni bilo mogoče zagnati: {detail}"),
    paste_failed: "Lepljenje ni uspelo",
    paste_failed_info: "Besedilo je v odložišču. Prilepite ga s ⌘V.",
    not_pasted: "Narek ni bil prilepljen",
    not_pasted_info: "Preden je bilo besedilo pripravljeno, je v ospredje prišla druga aplikacija. Besedilo je v odložišču. Prilepite ga s ⌘V.",
    request_timed_out: "Časovna omejitev zahteve je potekla.",
    audio_failed: "Posnetka ni bilo mogoče obdelati.",
    local_model_failed: "Lokalni model posnetka ni mogel prepisati.",
};
