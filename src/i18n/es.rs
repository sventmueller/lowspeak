use super::Texts;

pub static ES: Texts = Texts {
    recent_dictations: "Dictados recientes",
    none_yet: "Ninguno todavía",
    local: "Local",
    not_installed: |model| format!("{model} (no instalado)"),
    no_key: |provider| format!("{provider} (sin clave)"),
    language: "Idioma",
    auto_detect: "Detección automática",
    post_processing: "Posprocesado",
    post_processing_off: "Desactivado",
    post_processing_cleanup: "Limpieza (muletillas, gramática)",
    post_processing_format: "Formato (según el contexto)",
    vocabulary_menu: |terms| match terms {
        0 => "Vocabulario…".to_owned(),
        1 => "Vocabulario (1 término)…".to_owned(),
        terms => format!("Vocabulario ({terms} términos)…"),
    },
    microphone: "Micrófono",
    system_default: "Ajuste por omisión del sistema",
    not_connected: |microphone| format!("{microphone} (desconectado)"),
    play_sound_when_ready: "Reproducir sonido al estar listo",
    manage_models: "Gestionar modelos",
    delete_model: |model| format!("Eliminar {model}"),
    download_model: |model| format!("Descargar {model}"),
    set_api_key: "Configurar clave de API…",
    update_api_key: "Actualizar clave de API…",
    open_at_login: "Abrir al iniciar sesión",
    help: "Ayuda",
    quick_reference: "Referencia rápida",
    report_problem: "Informar de un problema…",
    suggest_feature: "Sugerir una función…",
    on_github: "Lowspeak en GitHub",
    about: "Acerca de Lowspeak",
    license: "Licencia: MIT o Apache 2.0",
    quit: "Salir de Lowspeak",

    reference_hold: "Mantén pulsada para dictar, suelta para pegar",
    reference_double_tap: "Pulsa dos veces rápido para dictar sin manos",
    reference_shift: "Dictar sin manos",
    reference_stop: "Terminar el dictado sin manos",
    reference_emoji: "Si la tecla 🌐 abre Emojis y símbolos, desactívalo en los ajustes de Teclado.",
    keyboard_settings: "Ajustes del teclado",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "inglés",
    languages: |count| match count {
        1 => "1 idioma".to_owned(),
        count => format!("{count} idiomas"),
    },
    legacy: "modelo antiguo",
    fast: "rápido",

    set_api_key_title: "Configurar clave de API",
    update_api_key_title: "Actualizar clave de API",
    api_key_info: "Introduce tu clave de API. El proveedor se detecta automáticamente:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... o gsk_...",
    save: "Guardar",
    cancel: "Cancelar",
    ok: "Aceptar",
    remove_all_keys: "Eliminar todas las claves",
    unknown_key_format: "Formato de clave desconocido",
    unknown_key_format_info: "La clave debe empezar por sk- (OpenAI) o gsk_ (Groq). Inténtalo de nuevo.",
    vocabulary_title: "Vocabulario",
    vocabulary_info: "Nombres y términos técnicos que deben escribirse tal cual, separados por comas. Los usan OpenAI, Groq, Whisper en este Mac y el posprocesado.",

    recording_failed: "Error de grabación",
    microphone_silent: "Micrófono sin señal",
    microphone_silent_info: "No se detecta ninguna señal de audio. Comprueba si el micrófono de los AirPods está disponible o cambia al micrófono integrado.",
    accessibility_required: "Se necesita permiso de accesibilidad",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak no puede pegar automáticamente. Concede el permiso en Ajustes del Sistema → Privacidad y seguridad → {setting} y pega manualmente con ⌘V."
        )
    },
    accessibility_list: "Accesibilidad",
    device_control_list: "Control de dispositivos y acceso a datos",
    transcription_failed: "Error de transcripción",
    transcribed_locally: "Transcrito en este Mac",
    transcribed_locally_info: |reason| {
        format!("{reason} Ha transcrito un modelo local en su lugar.")
    },
    post_processing_failed: "Error de posprocesado",
    post_processing_failed_info: |reason| {
        format!("{reason} El texto se ha pegado sin posprocesar.")
    },
    no_api_key: "Sin clave de API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "No hay ninguna clave de API configurada para {provider}. Añade una desde “{menu_item}” en el menú."
        )
    },
    login_item_error: "Error del ítem de inicio",
    login_item_error_info: |menu_item| {
        format!(
            "No se ha podido cambiar “{menu_item}”. Prueba a activarlo o desactivarlo en Ajustes del Sistema → General → Ítems de inicio y extensiones."
        )
    },
    api_key_error: "Error de clave de API",
    api_key_not_saved: "No se ha podido guardar la clave de API. Inténtalo de nuevo.",
    api_keys_not_removed: "No se han podido eliminar las claves de API. Inténtalo de nuevo.",
    download_failed: "Error de descarga",
    download_failed_info: |model| {
        format!(
            "No se ha podido descargar {model}. Comprueba la conexión a internet e inténtalo de nuevo."
        )
    },
    cannot_delete: "No se puede eliminar",
    cannot_delete_info: "Espera a que termine la operación en curso antes de eliminar un modelo.",
    delete_failed: "Error al eliminar",
    delete_failed_info: |model| format!("No se ha podido eliminar {model}. Inténtalo de nuevo."),

    no_api_key_error: |provider| {
        format!(
            "No hay ninguna clave de API configurada para {provider}. Configúrala desde la barra de menús."
        )
    },
    invalid_response: "Respuesta no válida del servidor.",
    authentication_failed: "Error de autenticación de la API. Comprueba tu clave de API.",
    rate_limited: "Se ha superado el límite de solicitudes. Espera un momento.",
    server_error: |status| format!("Error del servidor ({status}). Inténtalo de nuevo más tarde."),
    api_error: |status| {
        format!("Error de la API ({status}). Consulta el registro para ver los detalles.")
    },
    model_not_available: |menu_item| {
        format!("El modelo local no está disponible. Descárgalo desde “{menu_item}”.")
    },
    network_error: |message| format!("Error de red: {message}"),
    key_rejected: |provider| format!("{provider} ha rechazado la clave de API."),
    model_retired: |provider| format!("{provider} ya no ofrece el modelo de posprocesado."),
    request_rejected: |provider, status| {
        format!("{provider} ha rechazado la solicitud de posprocesado ({status}).")
    },
    microphone_permission_denied: "Lowspeak no tiene acceso al micrófono. Permítelo en Ajustes del Sistema → Privacidad y seguridad → Micrófono.",
    no_microphone: "No hay ningún micrófono conectado.",
    microphone_error: |detail| format!("No se ha podido iniciar el micrófono: {detail}"),
    paste_failed: "Error al pegar",
    paste_failed_info: "El texto está en el portapapeles. Pégalo con ⌘V.",
    request_timed_out: "Se ha agotado el tiempo de espera de la solicitud.",
    audio_failed: "No se ha podido procesar la grabación.",
    local_model_failed: "El modelo local no ha podido transcribir la grabación.",
};
