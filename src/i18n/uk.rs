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

pub static UK: Texts = Texts {
    recent_dictations: "Останні диктування",
    none_yet: "Поки немає",
    retry_failed_dictation: "Повторити невдале диктування",
    local: "Локально",
    not_installed: |model| format!("{model} (не встановлена)"),
    no_key: |provider| format!("{provider} (немає ключа)"),
    language: "Мова",
    auto_detect: "Автовизначення",
    post_processing: "Постобробка",
    post_processing_off: "Вимк.",
    post_processing_cleanup: "Очищення (слова-паразити, граматика)",
    post_processing_format: "Форматування (за контекстом)",
    vocabulary_menu: |terms| match terms {
        0 => "Словник…".to_owned(),
        terms => format!(
            "Словник ({terms} {})…",
            plural(terms, "термін", "терміни", "термінів")
        ),
    },
    microphone: "Мікрофон",
    system_default: "Як у системі",
    not_connected: |microphone| format!("{microphone} (не підключено)"),
    play_sound_when_ready: "Звук готовності",
    manage_models: "Керування моделями",
    delete_model: |model| format!("Видалити {model}"),
    download_model: |model| format!("Завантажити {model}"),
    set_api_key: "Задати ключ API…",
    update_api_key: "Змінити ключ API…",
    open_at_login: "Відкривати під час входу в систему",
    check_for_updates: "Перевірити наявність оновлень…",
    check_updates_automatically: "Автоматично перевіряти наявність оновлень",
    install_updates_automatically: "Автоматично завантажувати й інсталювати оновлення",
    help: "Довідка",
    quick_reference: "Коротка довідка",
    report_problem: "Повідомити про проблему…",
    suggest_feature: "Запропонувати функцію…",
    on_github: "Lowspeak на GitHub",
    about: "Про Lowspeak",
    license: "Ліцензія: MIT або Apache 2.0",
    quit: "Завершити Lowspeak",

    reference_hold: "Утримуйте, щоб диктувати; відпустіть, щоб вставити",
    reference_double_tap: "Натисніть двічі швидко, щоб диктувати без рук",
    reference_shift: "Диктувати без рук",
    reference_stop: "Завершити диктування без рук",
    reference_emoji: "Якщо клавіша 🌐 відкриває «Емодзі та символи», вимкніть це в параметрах клавіатури.",
    keyboard_settings: "Параметри клавіатури",

    megabytes: "МБ",
    gigabytes: "ГБ",
    decimal_separator: ",",
    english: "англійська",
    languages: |count| format!("{count} {}", plural(count, "мова", "мови", "мов")),
    legacy: "застаріла модель",
    fast: "швидка",

    set_api_key_title: "Задати ключ API",
    update_api_key_title: "Змінити ключ API",
    api_key_info: "Введіть ключ API. Сервіс визначається автоматично:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... або gsk_...",
    save: "Зберегти",
    cancel: "Скасувати",
    ok: "OK",
    remove_all_keys: "Видалити всі ключі",
    unknown_key_format: "Невідомий формат ключа",
    unknown_key_format_info: "Ключ має починатися з sk- (OpenAI) або gsk_ (Groq). Спробуйте ще раз.",
    vocabulary_title: "Словник",
    vocabulary_info: "Імена, назви й терміни, які слід писати саме так, через кому. Їх використовують OpenAI, Groq, Whisper і Parakeet на цьому Mac і постобробка.",

    recording_failed: "Помилка запису",
    microphone_silent: "Немає сигналу з мікрофона",
    microphone_silent_info: "Аудіосигнал не виявлено. Перевірте, чи доступний мікрофон AirPods, або перейдіть на вбудований мікрофон.",
    accessibility_required: "Потрібен дозвіл для функцій доступності",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak не може вставляти текст автоматично. Надайте дозвіл: Системні параметри → Приватність і безпека → {setting}. Потім вставте текст вручну за допомогою ⌘V."
        )
    },
    accessibility_list: "Доступність",
    device_control_list: "Керування пристроями й доступ до даних",
    transcription_failed: "Помилка розпізнавання",
    retry_failed_dictation_info: |menu_item| {
        format!("Запис збережено: виберіть у меню «{menu_item}», щоб розпізнати його знову.")
    },
    transcribed_locally: "Розпізнано на цьому Mac",
    transcribed_locally_info: |reason| {
        format!("{reason} Натомість текст розпізнала локальна модель.")
    },
    post_processing_failed: "Помилка постобробки",
    post_processing_failed_info: |reason| format!("{reason} Текст вставлено без постобробки."),
    no_api_key: "Немає ключа API",
    no_api_key_info: |provider, menu_item| {
        format!("Не задано ключ API для {provider}. Додайте його в меню через «{menu_item}».")
    },
    login_item_error: "Помилка автозапуску",
    login_item_error_info: |menu_item| {
        format!(
            "Не вдалося змінити «{menu_item}». Спробуйте перемкнути цей параметр вручну: Системні параметри → Загальні → Автозапуск і розширення."
        )
    },
    api_key_error: "Помилка ключа API",
    api_key_not_saved: "Не вдалося зберегти ключ API. Повторіть спробу.",
    api_keys_not_removed: "Не вдалося видалити ключі API. Повторіть спробу.",
    download_failed: "Помилка завантаження",
    download_failed_info: |model| {
        format!(
            "Не вдалося завантажити модель {model}. Перевірте підключення до інтернету й повторіть спробу."
        )
    },
    cannot_delete: "Неможливо видалити",
    cannot_delete_info: "Перед видаленням моделі дочекайтеся завершення поточної операції.",
    delete_failed: "Помилка видалення",
    delete_failed_info: |model| format!("Не вдалося видалити модель {model}. Повторіть спробу."),

    no_api_key_error: |provider| {
        format!("Не задано ключ API для {provider}. Задайте його через смугу меню.")
    },
    invalid_response: "Некоректна відповідь сервера.",
    authentication_failed: "Помилка автентифікації API. Перевірте ключ API.",
    rate_limited: "Перевищено ліміт запитів. Трохи зачекайте.",
    server_error: |status| format!("Помилка сервера ({status}). Спробуйте пізніше."),
    api_error: |status| format!("Помилка API ({status}). Подробиці в журналі."),
    model_not_available: |menu_item| {
        format!("Локальна модель недоступна. Завантажте її через «{menu_item}».")
    },
    network_error: |message| format!("Помилка мережі: {message}"),
    key_rejected: |provider| format!("Сервіс {provider} відхилив ключ API."),
    quota_exhausted: |provider| {
        format!(
            "Сервіс {provider} повідомляє, що на рахунку не залишилося коштів. Поповніть баланс у своєму обліковому записі {provider}."
        )
    },
    model_retired: |provider| format!("Сервіс {provider} більше не надає модель для постобробки."),
    no_post_processing_key: |first, second| {
        format!(
            "Для постобробки потрібен ключ API {first} або {second}. Задайте його через смугу меню."
        )
    },
    online_model_retired: |provider, model| {
        format!("Сервіс {provider} більше не надає {model}. Виберіть іншу модель у меню.")
    },
    request_rejected: |provider, status| {
        format!("Сервіс {provider} відхилив запит на постобробку ({status}).")
    },
    microphone_permission_denied: "Lowspeak не має доступу до мікрофона. Надайте дозвіл: Системні параметри → Приватність і безпека → Мікрофон.",
    no_microphone: "Мікрофон не під’єднано.",
    microphone_error: |detail| format!("Не вдалося запустити мікрофон: {detail}"),
    paste_failed: "Не вдалося вставити",
    paste_failed_info: "Текст у буфері обміну. Вставте його за допомогою ⌘V.",
    not_pasted: "Диктування не вставлено",
    not_pasted_info: "Інша програма перейшла на передній план, перш ніж текст був готовий. Текст у буфері обміну. Вставте його за допомогою ⌘V.",
    request_timed_out: "Час очікування запиту минув.",
    audio_failed: "Не вдалося обробити запис.",
    local_model_failed: "Локальній моделі не вдалося розпізнати запис.",
};
