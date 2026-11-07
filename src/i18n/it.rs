use super::Texts;

pub static IT: Texts = Texts {
    recent_dictations: "Dettature recenti",
    none_yet: "Nessuna per ora",
    local: "Locale",
    not_installed: |model| format!("{model} (non installato)"),
    no_key: |provider| format!("{provider} (nessuna chiave)"),
    language: "Lingua",
    auto_detect: "Rilevamento automatico",
    post_processing: "Post-elaborazione",
    post_processing_off: "Disattivata",
    post_processing_cleanup: "Pulizia (intercalari, grammatica)",
    post_processing_format: "Formattazione (in base al contesto)",
    vocabulary_menu: |terms| match terms {
        0 => "Vocabolario…".to_owned(),
        1 => "Vocabolario (1 termine)…".to_owned(),
        terms => format!("Vocabolario ({terms} termini)…"),
    },
    microphone: "Microfono",
    system_default: "Default di sistema",
    not_connected: |microphone| format!("{microphone} (non connesso)"),
    play_sound_when_ready: "Segnale acustico quando è pronto",
    manage_models: "Gestisci modelli",
    delete_model: |model| format!("Elimina {model}"),
    download_model: |model| format!("Scarica {model}"),
    set_api_key: "Imposta chiave API…",
    update_api_key: "Aggiorna chiave API…",
    open_at_login: "Apri al login",
    check_for_updates: "Verifica aggiornamenti…",
    check_updates_automatically: "Verifica automaticamente gli aggiornamenti",
    install_updates_automatically: "Scarica e installa automaticamente gli aggiornamenti",
    help: "Aiuto",
    quick_reference: "Riferimento rapido",
    report_problem: "Segnala un problema…",
    suggest_feature: "Suggerisci una funzione…",
    on_github: "Lowspeak su GitHub",
    about: "Informazioni su Lowspeak",
    license: "Licenza: MIT o Apache 2.0",
    quit: "Esci da Lowspeak",

    reference_hold: "Tieni premuto per dettare, rilascia per incollare",
    reference_double_tap: "Premi due volte rapidamente per dettare a mani libere",
    reference_shift: "Detta a mani libere",
    reference_stop: "Termina la dettatura a mani libere",
    reference_emoji: "Se il tasto 🌐 apre Emoji e simboli, disattivalo nelle impostazioni Tastiera.",
    keyboard_settings: "Impostazioni tastiera",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "inglese",
    languages: |count| match count {
        1 => "1 lingua".to_owned(),
        count => format!("{count} lingue"),
    },
    legacy: "modello precedente",
    fast: "veloce",

    set_api_key_title: "Imposta chiave API",
    update_api_key_title: "Aggiorna chiave API",
    api_key_info: "Inserisci la chiave API. Il fornitore viene rilevato automaticamente:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... o gsk_...",
    save: "Salva",
    cancel: "Annulla",
    ok: "OK",
    remove_all_keys: "Rimuovi tutte le chiavi",
    unknown_key_format: "Formato chiave sconosciuto",
    unknown_key_format_info: "La chiave deve iniziare con sk- (OpenAI) o gsk_ (Groq). Riprova.",
    vocabulary_title: "Vocabolario",
    vocabulary_info: "Nomi e termini tecnici da scrivere esattamente così, separati da virgole. Li usano OpenAI, Groq, Whisper su questo Mac e la post-elaborazione.",

    recording_failed: "Registrazione non riuscita",
    microphone_silent: "Microfono silenzioso",
    microphone_silent_info: "Nessun segnale audio rilevato. Verifica che il microfono degli AirPods sia disponibile o passa al microfono integrato.",
    accessibility_required: "Autorizzazione di accessibilità richiesta",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak non può incollare automaticamente. Concedi l’autorizzazione in Impostazioni di Sistema → Privacy e sicurezza → {setting}, poi incolla manualmente con ⌘V."
        )
    },
    accessibility_list: "Accessibilità",
    device_control_list: "Controllo del dispositivo e accesso ai dati",
    transcription_failed: "Trascrizione non riuscita",
    transcribed_locally: "Trascritto su questo Mac",
    transcribed_locally_info: |reason| {
        format!("{reason} La trascrizione è stata eseguita da un modello locale.")
    },
    post_processing_failed: "Post-elaborazione non riuscita",
    post_processing_failed_info: |reason| {
        format!("{reason} Il testo è stato incollato senza post-elaborazione.")
    },
    no_api_key: "Nessuna chiave API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Nessuna chiave API configurata per {provider}. Aggiungine una con “{menu_item}” nel menu."
        )
    },
    login_item_error: "Errore elemento login",
    login_item_error_info: |menu_item| {
        format!(
            "Impossibile aggiornare “{menu_item}”. Prova ad attivarlo o disattivarlo in Impostazioni di Sistema → Generali → Elementi login ed estensioni."
        )
    },
    api_key_error: "Errore chiave API",
    api_key_not_saved: "Impossibile salvare la chiave API. Riprova.",
    api_keys_not_removed: "Impossibile rimuovere le chiavi API. Riprova.",
    download_failed: "Download non riuscito",
    download_failed_info: |model| {
        format!("Impossibile scaricare {model}. Controlla la connessione a Internet e riprova.")
    },
    cannot_delete: "Impossibile eliminare",
    cannot_delete_info: "Attendi il termine dell’operazione in corso prima di eliminare un modello.",
    delete_failed: "Eliminazione non riuscita",
    delete_failed_info: |model| format!("Impossibile eliminare {model}. Riprova."),

    no_api_key_error: |provider| {
        format!("Nessuna chiave API configurata per {provider}. Impostala dalla barra dei menu.")
    },
    invalid_response: "Risposta del server non valida.",
    authentication_failed: "Autenticazione API non riuscita. Controlla la chiave API.",
    rate_limited: "Limite di richieste superato. Attendi un momento.",
    server_error: |status| format!("Errore del server ({status}). Riprova più tardi."),
    api_error: |status| format!("Errore API ({status}). Controlla il registro per i dettagli."),
    model_not_available: |menu_item| {
        format!("Modello locale non disponibile. Scaricalo da “{menu_item}”.")
    },
    network_error: |message| format!("Errore di rete: {message}"),
    key_rejected: |provider| format!("{provider} ha rifiutato la chiave API."),
    model_retired: |provider| format!("{provider} non offre più il modello di post-elaborazione."),
    request_rejected: |provider, status| {
        format!("{provider} ha rifiutato la richiesta di post-elaborazione ({status}).")
    },
    microphone_permission_denied: "Lowspeak non ha accesso al microfono. Consenti l’accesso in Impostazioni di Sistema → Privacy e sicurezza → Microfono.",
    no_microphone: "Nessun microfono collegato.",
    microphone_error: |detail| format!("Impossibile avviare il microfono: {detail}"),
    paste_failed: "Incolla non riuscito",
    paste_failed_info: "Il testo si trova negli appunti. Incollalo con ⌘V.",
    request_timed_out: "La richiesta è scaduta.",
    audio_failed: "Impossibile elaborare la registrazione.",
    local_model_failed: "Il modello locale non è riuscito a trascrivere la registrazione.",
};
