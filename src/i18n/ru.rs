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

pub static RU: Texts = Texts {
    recent_dictations: "Последние диктовки",
    none_yet: "Пока нет",
    local: "Локально",
    not_installed: |model| format!("{model} (не установлена)"),
    no_key: |provider| format!("{provider} (нет ключа)"),
    language: "Язык",
    auto_detect: "Автоопределение",
    post_processing: "Постобработка",
    post_processing_off: "Выкл.",
    post_processing_cleanup: "Очистка (слова-паразиты, грамматика)",
    post_processing_format: "Форматирование (по контексту)",
    vocabulary_menu: |terms| match terms {
        0 => "Словарь…".to_owned(),
        terms => format!(
            "Словарь ({terms} {})…",
            plural(terms, "термин", "термина", "терминов")
        ),
    },
    microphone: "Микрофон",
    system_default: "Системный по умолчанию",
    not_connected: |microphone| format!("{microphone} (не подключен)"),
    play_sound_when_ready: "Звук готовности",
    manage_models: "Управление моделями",
    delete_model: |model| format!("Удалить {model}"),
    download_model: |model| format!("Загрузить {model}"),
    set_api_key: "Задать ключ API…",
    update_api_key: "Изменить ключ API…",
    open_at_login: "Открывать при входе",
    help: "Справка",
    quick_reference: "Краткая справка",
    report_problem: "Сообщить о проблеме…",
    suggest_feature: "Предложить функцию…",
    on_github: "Lowspeak на GitHub",
    about: "О приложении Lowspeak",
    license: "Лицензия: MIT или Apache 2.0",
    quit: "Завершить Lowspeak",

    reference_hold: "Удерживайте, чтобы диктовать; отпустите, чтобы вставить",
    reference_double_tap: "Нажмите дважды быстро, чтобы диктовать без рук",
    reference_shift: "Диктовать без рук",
    reference_stop: "Завершить диктовку без рук",
    reference_emoji: "Если клавиша 🌐 открывает «Эмодзи и символы», отключите это в настройках клавиатуры.",
    keyboard_settings: "Настройки клавиатуры",

    megabytes: "МБ",
    gigabytes: "ГБ",
    decimal_separator: ",",
    english: "английский",
    languages: |count| format!("{count} {}", plural(count, "язык", "языка", "языков")),
    legacy: "устаревшая модель",
    fast: "быстрая",

    set_api_key_title: "Задать ключ API",
    update_api_key_title: "Изменить ключ API",
    api_key_info: "Введите ключ API. Сервис определяется автоматически:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... или gsk_...",
    save: "Сохранить",
    cancel: "Отменить",
    ok: "OK",
    remove_all_keys: "Удалить все ключи",
    unknown_key_format: "Неизвестный формат ключа",
    unknown_key_format_info: "Ключ должен начинаться с sk- (OpenAI) или gsk_ (Groq). Повторите попытку.",
    vocabulary_title: "Словарь",
    vocabulary_info: "Имена, названия и термины, которые нужно писать именно так, через запятую. Их используют OpenAI, Groq, Whisper на этом Mac и постобработка.",

    recording_failed: "Ошибка записи",
    microphone_silent: "Нет сигнала с микрофона",
    microphone_silent_info: "Аудиосигнал не обнаружен. Проверьте, доступен ли микрофон AirPods, или переключитесь на встроенный микрофон.",
    accessibility_required: "Требуется разрешение на универсальный доступ",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak не может вставлять текст автоматически. Предоставьте разрешение: Системные настройки → Конфиденциальность и безопасность → {setting}. Затем вставьте текст вручную с помощью ⌘V."
        )
    },
    accessibility_list: "Универсальный доступ",
    device_control_list: "Управление устройством и доступ к данным",
    transcription_failed: "Ошибка распознавания",
    transcribed_locally: "Распознано на этом Mac",
    transcribed_locally_info: |reason| {
        format!("{reason} Вместо этого текст распознала локальная модель.")
    },
    post_processing_failed: "Ошибка постобработки",
    post_processing_failed_info: |reason| format!("{reason} Текст вставлен без постобработки."),
    no_api_key: "Нет ключа API",
    no_api_key_info: |provider, menu_item| {
        format!("Не задан ключ API для {provider}. Добавьте его в меню через «{menu_item}».")
    },
    login_item_error: "Ошибка объекта входа",
    login_item_error_info: |menu_item| {
        format!(
            "Не удалось изменить «{menu_item}». Попробуйте переключить этот параметр вручную: Системные настройки → Основные → Объекты входа и расширения."
        )
    },
    api_key_error: "Ошибка ключа API",
    api_key_not_saved: "Не удалось сохранить ключ API. Повторите попытку.",
    api_keys_not_removed: "Не удалось удалить ключи API. Повторите попытку.",
    download_failed: "Ошибка загрузки",
    download_failed_info: |model| {
        format!(
            "Не удалось загрузить модель {model}. Проверьте подключение к интернету и повторите попытку."
        )
    },
    cannot_delete: "Удаление невозможно",
    cannot_delete_info: "Перед удалением модели дождитесь завершения текущей операции.",
    delete_failed: "Ошибка удаления",
    delete_failed_info: |model| format!("Не удалось удалить модель {model}. Повторите попытку."),

    no_api_key_error: |provider| {
        format!("Не задан ключ API для {provider}. Задайте его в строке меню.")
    },
    invalid_response: "Некорректный ответ сервера.",
    authentication_failed: "Ошибка аутентификации API. Проверьте ключ API.",
    rate_limited: "Превышен лимит запросов. Подождите немного.",
    server_error: |status| format!("Ошибка сервера ({status}). Повторите попытку позже."),
    api_error: |status| format!("Ошибка API ({status}). Подробности в журнале."),
    model_not_available: |menu_item| {
        format!("Локальная модель недоступна. Загрузите ее через «{menu_item}».")
    },
    network_error: |message| format!("Ошибка сети: {message}"),
    key_rejected: |provider| format!("Сервис {provider} отклонил ключ API."),
    model_retired: |provider| {
        format!("Сервис {provider} больше не предоставляет модель для постобработки.")
    },
    request_rejected: |provider, status| {
        format!("Сервис {provider} отклонил запрос на постобработку ({status}).")
    },
    microphone_permission_denied: "У Lowspeak нет доступа к микрофону. Разрешите доступ: Системные настройки → Конфиденциальность и безопасность → Микрофон.",
    no_microphone: "Микрофон не подключён.",
    microphone_error: |detail| format!("Не удалось запустить микрофон: {detail}"),
    paste_failed: "Не удалось вставить",
    paste_failed_info: "Текст находится в буфере обмена. Вставьте его с помощью ⌘V.",
    request_timed_out: "Истекло время ожидания запроса.",
    audio_failed: "Не удалось обработать запись.",
    local_model_failed: "Локальной модели не удалось распознать запись.",
};
