use super::Texts;

pub static RO: Texts = Texts {
    recent_dictations: "Dictări recente",
    none_yet: "Niciuna deocamdată",
    local: "Local",
    not_installed: |model| format!("{model} (neinstalat)"),
    no_key: |provider| format!("{provider} (fără cheie)"),
    language: "Limbă",
    auto_detect: "Detectare automată",
    post_processing: "Postprocesare",
    post_processing_off: "Dezactivată",
    post_processing_cleanup: "Curățare (cuvinte de umplutură, gramatică)",
    post_processing_format: "Formatare (în funcție de context)",
    vocabulary_menu: |terms| match terms {
        0 => "Vocabular…".to_owned(),
        1 => "Vocabular (1 termen)…".to_owned(),
        _ if (1..=19).contains(&(terms % 100)) => format!("Vocabular ({terms} termeni)…"),
        _ => format!("Vocabular ({terms} de termeni)…"),
    },
    microphone: "Microfon",
    system_default: "Implicit pentru sistem",
    not_connected: |microphone| format!("{microphone} (neconectat)"),
    play_sound_when_ready: "Semnal sonor când este gata",
    manage_models: "Gestionați modelele",
    delete_model: |model| format!("Ștergeți {model}"),
    download_model: |model| format!("Descărcați {model}"),
    set_api_key: "Configurați cheia API…",
    update_api_key: "Actualizați cheia API…",
    open_at_login: "Deschideți la login",
    check_for_updates: "Căutați actualizări…",
    check_updates_automatically: "Căutați automat actualizări",
    install_updates_automatically: "Descărcați și instalați automat actualizările",
    help: "Ajutor",
    quick_reference: "Referință rapidă",
    report_problem: "Raportați o problemă…",
    suggest_feature: "Sugerați o funcție…",
    on_github: "Lowspeak pe GitHub",
    about: "Despre Lowspeak",
    license: "Licență: MIT sau Apache 2.0",
    quit: "Închideți aplicația Lowspeak",

    reference_hold: "Țineți apăsat pentru a dicta, eliberați pentru a lipi",
    reference_double_tap: "Apăsați de două ori rapid pentru a dicta fără mâini",
    reference_shift: "Dictați fără mâini",
    reference_stop: "Încheiați dictarea fără mâini",
    reference_emoji: "Dacă tasta 🌐 deschide Emoji și simboluri, dezactivați acest lucru în configurările tastaturii.",
    keyboard_settings: "Configurări tastatură",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "engleză",
    languages: |count| match count {
        1 => "1 limbă".to_owned(),
        _ if count == 0 || (1..=19).contains(&(count % 100)) => format!("{count} limbi"),
        _ => format!("{count} de limbi"),
    },
    legacy: "model mai vechi",
    fast: "rapid",

    set_api_key_title: "Configurați cheia API",
    update_api_key_title: "Actualizați cheia API",
    api_key_info: "Introduceți cheia API. Furnizorul este detectat automat:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... sau gsk_...",
    save: "Salvați",
    cancel: "Anulați",
    ok: "OK",
    remove_all_keys: "Eliminați toate cheile",
    unknown_key_format: "Format de cheie necunoscut",
    unknown_key_format_info: "Cheia trebuie să înceapă cu sk- (OpenAI) sau gsk_ (Groq). Încercați din nou.",
    vocabulary_title: "Vocabular",
    vocabulary_info: "Nume și termeni tehnici care trebuie scriși exact așa, separați prin virgule. Sunt folosiți de OpenAI, Groq, Whisper pe acest Mac și de postprocesare.",

    recording_failed: "Înregistrarea a eșuat",
    microphone_silent: "Microfon fără semnal",
    microphone_silent_info: "Nu a fost detectat niciun semnal audio. Verificați dacă microfonul AirPods este disponibil sau treceți la microfonul încorporat.",
    accessibility_required: "Este necesară permisiunea de accesibilitate",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak nu poate lipi automat. Acordați permisiunea în Configurări sistem → Intimitate și securitate → {setting}, apoi lipiți manual cu ⌘V."
        )
    },
    accessibility_list: "Accesibilitate",
    device_control_list: "Control al dispozitivului și acces la date",
    transcription_failed: "Transcrierea a eșuat",
    transcribed_locally: "Transcris pe acest Mac",
    transcribed_locally_info: |reason| {
        format!("{reason} Transcrierea a fost făcută de un model local.")
    },
    post_processing_failed: "Postprocesarea a eșuat",
    post_processing_failed_info: |reason| {
        format!("{reason} Textul a fost lipit fără postprocesare.")
    },
    no_api_key: "Nicio cheie API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Nu este configurată nicio cheie API pentru {provider}. Adăugați una prin „{menu_item}” din meniu."
        )
    },
    login_item_error: "Eroare de lansare la login",
    login_item_error_info: |menu_item| {
        format!(
            "Nu s-a putut actualiza „{menu_item}”. Încercați să-l activați sau să-l dezactivați în Configurări sistem → General → Lansare la login și extensii."
        )
    },
    api_key_error: "Eroare la cheia API",
    api_key_not_saved: "Cheia API nu a putut fi salvată. Încercați din nou.",
    api_keys_not_removed: "Cheile API nu au putut fi eliminate. Încercați din nou.",
    download_failed: "Descărcarea a eșuat",
    download_failed_info: |model| {
        format!(
            "Nu s-a putut descărca {model}. Verificați conexiunea la internet și încercați din nou."
        )
    },
    cannot_delete: "Nu se poate șterge",
    cannot_delete_info: "Așteptați finalizarea operațiunii curente înainte de a șterge un model.",
    delete_failed: "Ștergerea a eșuat",
    delete_failed_info: |model| format!("Nu s-a putut șterge {model}. Încercați din nou."),

    no_api_key_error: |provider| {
        format!(
            "Nu este configurată nicio cheie API pentru {provider}. Configurați-o din bara de meniuri."
        )
    },
    invalid_response: "Răspuns nevalid de la server.",
    authentication_failed: "Autentificarea API a eșuat. Verificați cheia API.",
    rate_limited: "Limita de cereri a fost depășită. Așteptați puțin.",
    server_error: |status| format!("Eroare de server ({status}). Încercați din nou mai târziu."),
    api_error: |status| format!("Eroare API ({status}). Consultați jurnalul pentru detalii."),
    model_not_available: |menu_item| {
        format!("Modelul local nu este disponibil. Descărcați-l din „{menu_item}”.")
    },
    network_error: |message| format!("Eroare de rețea: {message}"),
    key_rejected: |provider| format!("{provider} a respins cheia API."),
    model_retired: |provider| format!("{provider} nu mai oferă modelul de postprocesare."),
    no_post_processing_key: |first, second| {
        format!(
            "Postprocesarea necesită o cheie API de la {first} sau {second}. Configurați-o din bara de meniuri."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} nu mai oferă {model}. Alegeți alt model din meniu.")
    },
    request_rejected: |provider, status| {
        format!("{provider} a respins cererea de postprocesare ({status}).")
    },
    microphone_permission_denied: "Lowspeak nu are acces la microfon. Permiteți accesul în Configurări sistem → Intimitate și securitate → Microfon.",
    no_microphone: "Niciun microfon conectat.",
    microphone_error: |detail| format!("Microfonul nu a putut fi pornit: {detail}"),
    paste_failed: "Lipirea a eșuat",
    paste_failed_info: "Textul se află în clipboard. Lipiți-l cu ⌘V.",
    request_timed_out: "Solicitarea a expirat.",
    audio_failed: "Înregistrarea nu a putut fi procesată.",
    local_model_failed: "Modelul local nu a putut transcrie înregistrarea.",
};
