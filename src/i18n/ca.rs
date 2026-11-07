use super::Texts;

pub static CA: Texts = Texts {
    recent_dictations: "Dictats recents",
    none_yet: "Encara no n’hi ha cap",
    local: "Local",
    not_installed: |model| format!("{model} (no instal·lat)"),
    no_key: |provider| format!("{provider} (sense clau)"),
    language: "Idioma",
    auto_detect: "Detecció automàtica",
    post_processing: "Postprocessament",
    post_processing_off: "Desactivat",
    post_processing_cleanup: "Neteja (crosses, gramàtica)",
    post_processing_format: "Format (segons el context)",
    vocabulary_menu: |terms| match terms {
        0 => "Vocabulari…".to_owned(),
        1 => "Vocabulari (1 terme)…".to_owned(),
        terms => format!("Vocabulari ({terms} termes)…"),
    },
    microphone: "Micròfon",
    system_default: "Valor per omissió del sistema",
    not_connected: |microphone| format!("{microphone} (no connectat)"),
    play_sound_when_ready: "Reprodueix un so quan estigui a punt",
    manage_models: "Gestiona els models",
    delete_model: |model| format!("Elimina {model}"),
    download_model: |model| format!("Descarrega {model}"),
    set_api_key: "Configura la clau d’API…",
    update_api_key: "Actualitza la clau d’API…",
    open_at_login: "Obre en iniciar la sessió",
    check_for_updates: "Cerca actualitzacions…",
    check_updates_automatically: "Cerca actualitzacions automàticament",
    install_updates_automatically: "Descarrega i instal·la actualitzacions automàticament",
    help: "Ajuda",
    quick_reference: "Referència ràpida",
    report_problem: "Informa d’un problema…",
    suggest_feature: "Suggereix una funció…",
    on_github: "Lowspeak a GitHub",
    about: "Sobre el Lowspeak",
    license: "Llicència: MIT o Apache 2.0",
    quit: "Surt del Lowspeak",

    reference_hold: "Mantén premuda per dictar, deixa anar per enganxar",
    reference_double_tap: "Prem dues vegades ràpidament per dictar amb mans lliures",
    reference_shift: "Dictar amb mans lliures",
    reference_stop: "Acabar el dictat amb mans lliures",
    reference_emoji: "Si la tecla 🌐 obre Emojis i símbols, configura-la perquè no faci res a la configuració del teclat.",
    keyboard_settings: "Configuració del teclat",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "anglès",
    languages: |count| match count {
        1 => "1 idioma".to_owned(),
        count => format!("{count} idiomes"),
    },
    legacy: "model antic",
    fast: "ràpid",

    set_api_key_title: "Configurar la clau d’API",
    update_api_key_title: "Actualitzar la clau d’API",
    api_key_info: "Introdueix la teva clau d’API. El proveïdor es detecta automàticament:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... o gsk_...",
    save: "Desa",
    cancel: "Cancel·la",
    ok: "D’acord",
    remove_all_keys: "Elimina totes les claus",
    unknown_key_format: "Format de clau desconegut",
    unknown_key_format_info: "S’esperava una clau que comencés per sk- (OpenAI) o gsk_ (Groq). Torna-ho a provar.",
    vocabulary_title: "Vocabulari",
    vocabulary_info: "Noms i termes tècnics que s’han d’escriure exactament així, separats per comes. Els utilitzen OpenAI, Groq, Whisper en aquest Mac i el postprocessament.",

    recording_failed: "Error d’enregistrament",
    microphone_silent: "Micròfon en silenci",
    microphone_silent_info: "No s’ha detectat cap senyal d’àudio. Comprova si el micròfon dels AirPods està disponible o canvia al micròfon integrat.",
    accessibility_required: "Cal el permís d’accessibilitat",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak no pot enganxar automàticament. Concedeix el permís a Configuració del Sistema → Privacitat i seguretat → {setting} i, després, enganxa manualment amb ⌘V."
        )
    },
    accessibility_list: "Accessibilitat",
    device_control_list: "Control de dispositius i accés a les dades",
    transcription_failed: "Error de transcripció",
    transcribed_locally: "Transcrit en aquest Mac",
    transcribed_locally_info: |reason| format!("{reason} Un model local ha fet la transcripció."),
    post_processing_failed: "Error de postprocessament",
    post_processing_failed_info: |reason| {
        format!("{reason} El text s’ha enganxat sense postprocessament.")
    },
    no_api_key: "No hi ha cap clau d’API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "No hi ha cap clau d’API configurada per a {provider}. Afegeix-ne una amb «{menu_item}» al menú."
        )
    },
    login_item_error: "Error de l’ítem d’inici",
    login_item_error_info: |menu_item| {
        format!(
            "No s’ha pogut canviar «{menu_item}». Prova d’activar-ho o desactivar-ho a Configuració del Sistema → General → Ítems d’inici de sessió i extensions."
        )
    },
    api_key_error: "Error de la clau d’API",
    api_key_not_saved: "No s’ha pogut desar la clau d’API. Torna-ho a provar.",
    api_keys_not_removed: "No s’han pogut eliminar les claus d’API. Torna-ho a provar.",
    download_failed: "Error de descàrrega",
    download_failed_info: |model| {
        format!(
            "No s’ha pogut descarregar {model}. Comprova la connexió a internet i torna-ho a provar."
        )
    },
    cannot_delete: "No es pot eliminar",
    cannot_delete_info: "Espera que acabi l’operació en curs abans d’eliminar un model.",
    delete_failed: "Error en eliminar",
    delete_failed_info: |model| format!("No s’ha pogut eliminar {model}. Torna-ho a provar."),

    no_api_key_error: |provider| {
        format!(
            "No hi ha cap clau d’API configurada per a {provider}. Configura-la des de la barra de menús."
        )
    },
    invalid_response: "Resposta no vàlida del servidor.",
    authentication_failed: "Ha fallat l’autenticació de l’API. Comprova la clau d’API.",
    rate_limited: "S’ha superat el límit de sol·licituds. Espera un moment.",
    server_error: |status| format!("Error del servidor ({status}). Torna-ho a provar més tard."),
    api_error: |status| {
        format!("Error de l’API ({status}). Consulta el registre per a més detalls.")
    },
    model_not_available: |menu_item| {
        format!("El model local no està disponible. Descarrega’l amb «{menu_item}».")
    },
    network_error: |message| format!("Error de xarxa: {message}"),
    key_rejected: |provider| format!("{provider} ha rebutjat la clau d’API."),
    model_retired: |provider| format!("{provider} ja no ofereix el model de postprocessament."),
    no_post_processing_key: |first, second| {
        format!(
            "El postprocessament necessita una clau d’API de {first} o {second}. Configura-la des de la barra de menús."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} ja no ofereix {model}. Tria un altre model al menú.")
    },
    request_rejected: |provider, status| {
        format!("{provider} ha rebutjat la sol·licitud de postprocessament ({status}).")
    },
    microphone_permission_denied: "Lowspeak no té accés al micròfon. Concedeix-li l’accés a Configuració del Sistema → Privacitat i seguretat → Micròfon.",
    no_microphone: "No hi ha cap micròfon connectat.",
    microphone_error: |detail| format!("No s’ha pogut iniciar el micròfon: {detail}"),
    paste_failed: "Error en enganxar",
    paste_failed_info: "El text és al porta-retalls. Enganxa’l amb ⌘V.",
    request_timed_out: "S’ha esgotat el temps d’espera de la sol·licitud.",
    audio_failed: "No s’ha pogut processar l’enregistrament.",
    local_model_failed: "El model local no ha pogut transcriure l’enregistrament.",
};
