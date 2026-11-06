use super::Texts;

fn plural(n: usize, one: &'static str, few: &'static str, many: &'static str) -> &'static str {
    if n == 1 {
        one
    } else if (2..=4).contains(&(n % 10)) && !(12..=14).contains(&(n % 100)) {
        few
    } else {
        many
    }
}

pub static PL: Texts = Texts {
    recent_dictations: "Ostatnio podyktowane",
    none_yet: "Na razie brak",
    local: "Lokalnie",
    not_installed: |model| format!("{model} (nie zainstalowano)"),
    no_key: |provider| format!("{provider} (brak klucza)"),
    language: "Język",
    auto_detect: "Wykrywaj automatycznie",
    post_processing: "Przetwarzanie końcowe",
    post_processing_off: "Wyłączone",
    post_processing_cleanup: "Oczyszczanie (przerywniki, gramatyka)",
    post_processing_format: "Formatowanie (zależne od kontekstu)",
    vocabulary_menu: |terms| match terms {
        0 => "Słownictwo…".to_owned(),
        terms => format!(
            "Słownictwo ({terms} {})…",
            plural(terms, "termin", "terminy", "terminów")
        ),
    },
    microphone: "Mikrofon",
    system_default: "Domyślny systemowy",
    not_connected: |microphone| format!("{microphone} (niepodłączony)"),
    play_sound_when_ready: "Dźwięk gotowości",
    manage_models: "Zarządzaj modelami",
    delete_model: |model| format!("Usuń {model}"),
    download_model: |model| format!("Pobierz {model}"),
    set_api_key: "Ustaw klucz API…",
    update_api_key: "Zmień klucz API…",
    open_at_login: "Otwieraj podczas logowania",
    help: "Pomoc",
    quick_reference: "Skrócona instrukcja",
    report_problem: "Zgłoś problem…",
    suggest_feature: "Zaproponuj funkcję…",
    on_github: "Lowspeak w serwisie GitHub",
    about: "Lowspeak – informacje",
    license: "Licencja: MIT lub Apache 2.0",
    quit: "Zakończ Lowspeak",

    reference_hold: "Przytrzymaj, aby dyktować; puść, aby wkleić",
    reference_double_tap: "Naciśnij dwa razy szybko, aby dyktować bez użycia rąk",
    reference_shift: "Dyktuj bez użycia rąk",
    reference_stop: "Zakończ dyktowanie bez użycia rąk",
    reference_emoji: "Jeśli klawisz 🌐 otwiera Emoji i symbole, wyłącz to w ustawieniach klawiatury.",
    keyboard_settings: "Ustawienia klawiatury",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "angielski",
    languages: |count| format!("{count} {}", plural(count, "język", "języki", "języków")),
    legacy: "starszy model",
    fast: "szybki",

    set_api_key_title: "Ustaw klucz API",
    update_api_key_title: "Zmień klucz API",
    api_key_info: "Wprowadź klucz API. Usługa zostanie rozpoznana automatycznie:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... lub gsk_...",
    save: "Zachowaj",
    cancel: "Anuluj",
    ok: "OK",
    remove_all_keys: "Usuń wszystkie klucze",
    unknown_key_format: "Nieznany format klucza",
    unknown_key_format_info: "Klucz powinien zaczynać się od sk- (OpenAI) lub gsk_ (Groq). Spróbuj ponownie.",
    vocabulary_title: "Słownictwo",
    vocabulary_info: "Nazwy własne i terminy fachowe do zapisywania dokładnie w tej formie, rozdzielone przecinkami. Korzystają z nich OpenAI, Groq, Whisper na tym Macu i przetwarzanie końcowe.",

    recording_failed: "Nagrywanie nie powiodło się",
    microphone_silent: "Brak sygnału z mikrofonu",
    microphone_silent_info: "Nie wykryto sygnału audio. Sprawdź, czy mikrofon AirPods jest dostępny, lub przełącz na wbudowany mikrofon.",
    accessibility_required: "Wymagane uprawnienie do funkcji dostępności",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak nie może wklejać automatycznie. Nadaj uprawnienie w Ustawieniach systemowych → Prywatność i ochrona → {setting}, a następnie wklej tekst ręcznie, naciskając ⌘V."
        )
    },
    accessibility_list: "Dostępność",
    device_control_list: "Sterowanie urządzeniami i dostęp do danych",
    transcription_failed: "Transkrypcja nie powiodła się",
    transcribed_locally: "Transkrypcja wykonana na tym Macu",
    transcribed_locally_info: |reason| {
        format!("{reason} Zamiast tego transkrypcję wykonał model lokalny.")
    },
    post_processing_failed: "Przetwarzanie końcowe nie powiodło się",
    post_processing_failed_info: |reason| {
        format!("{reason} Tekst wklejono bez przetwarzania końcowego.")
    },
    no_api_key: "Brak klucza API",
    no_api_key_info: |provider, menu_item| {
        format!("Brak klucza API dla {provider}. Dodaj go w menu, wybierając „{menu_item}”.")
    },
    login_item_error: "Błąd uruchamiania podczas logowania",
    login_item_error_info: |menu_item| {
        format!(
            "Nie udało się zmienić opcji „{menu_item}”. Spróbuj przełączyć ją w Ustawieniach systemowych → Ogólne → Rzeczy i rozszerzenia otwierane podczas logowania."
        )
    },
    api_key_error: "Błąd klucza API",
    api_key_not_saved: "Nie udało się zachować klucza API. Spróbuj ponownie.",
    api_keys_not_removed: "Nie udało się usunąć kluczy API. Spróbuj ponownie.",
    download_failed: "Pobieranie nie powiodło się",
    download_failed_info: |model| {
        format!(
            "Nie udało się pobrać modelu {model}. Sprawdź połączenie z internetem i spróbuj ponownie."
        )
    },
    cannot_delete: "Nie można usunąć",
    cannot_delete_info: "Przed usunięciem modelu zaczekaj na zakończenie bieżącej operacji.",
    delete_failed: "Usuwanie nie powiodło się",
    delete_failed_info: |model| format!("Nie udało się usunąć modelu {model}. Spróbuj ponownie."),

    no_api_key_error: |provider| format!("Brak klucza API dla {provider}. Ustaw go na pasku menu."),
    invalid_response: "Nieprawidłowa odpowiedź serwera.",
    authentication_failed: "Uwierzytelnianie API nie powiodło się. Sprawdź klucz API.",
    rate_limited: "Przekroczono limit zapytań. Zaczekaj chwilę.",
    server_error: |status| format!("Błąd serwera ({status}). Spróbuj ponownie później."),
    api_error: |status| format!("Błąd API ({status}). Szczegóły znajdziesz w dzienniku."),
    model_not_available: |menu_item| {
        format!("Model lokalny jest niedostępny. Pobierz go, wybierając „{menu_item}”.")
    },
    network_error: |message| format!("Błąd sieci: {message}"),
    key_rejected: |provider| format!("Usługa {provider} odrzuciła klucz API."),
    model_retired: |provider| {
        format!("Usługa {provider} nie oferuje już modelu do przetwarzania końcowego.")
    },
    request_rejected: |provider, status| {
        format!("Usługa {provider} odrzuciła żądanie przetwarzania końcowego ({status}).")
    },
    microphone_permission_denied: "Lowspeak nie ma dostępu do mikrofonu. Zezwól na dostęp w Ustawieniach systemowych → Prywatność i ochrona → Mikrofon.",
    no_microphone: "Nie podłączono żadnego mikrofonu.",
    microphone_error: |detail| format!("Nie można uruchomić mikrofonu: {detail}"),
    paste_failed: "Wklejanie nie powiodło się",
    paste_failed_info: "Tekst jest w schowku. Wklej go, naciskając ⌘V.",
    request_timed_out: "Przekroczono limit czasu żądania.",
    audio_failed: "Nie można przetworzyć nagrania.",
    local_model_failed: "Model lokalny nie mógł przepisać nagrania.",
};
