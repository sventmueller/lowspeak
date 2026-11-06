use super::Texts;

pub static EN: Texts = Texts {
    recent_dictations: "Recent Dictations",
    none_yet: "None Yet",
    local: "Local",
    not_installed: |model| format!("{model} (Not Installed)"),
    no_key: |provider| format!("{provider} (No Key)"),
    language: "Language",
    auto_detect: "Auto-Detect",
    post_processing: "Post-Processing",
    post_processing_off: "Off",
    post_processing_cleanup: "Clean Up (Filler Words, Grammar)",
    post_processing_format: "Format (Context-Aware)",
    vocabulary_menu: |terms| match terms {
        0 => "Vocabulary…".to_owned(),
        1 => "Vocabulary (1 Term)…".to_owned(),
        terms => format!("Vocabulary ({terms} Terms)…"),
    },
    microphone: "Microphone",
    system_default: "System Default",
    not_connected: |microphone| format!("{microphone} (Not Connected)"),
    play_sound_when_ready: "Play Sound When Ready",
    manage_models: "Manage Models",
    delete_model: |model| format!("Delete {model}"),
    download_model: |model| format!("Download {model}"),
    set_api_key: "Set API Key…",
    update_api_key: "Update API Key…",
    open_at_login: "Open at Login",
    help: "Help",
    quick_reference: "Quick Reference",
    report_problem: "Report Problem…",
    suggest_feature: "Suggest Feature…",
    on_github: "Lowspeak on GitHub",
    about: "About Lowspeak",
    license: "License: MIT or Apache 2.0",
    quit: "Quit Lowspeak",

    reference_hold: "Hold to dictate, release to paste",
    reference_double_tap: "Press twice quickly to dictate hands-free",
    reference_shift: "Dictate hands-free",
    reference_stop: "End hands-free dictation",
    reference_emoji: "If the 🌐 key opens Emoji & Symbols, set it to do nothing in Keyboard settings.",
    keyboard_settings: "Keyboard Settings",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ".",
    english: "English",
    languages: |count| format!("{count} Languages"),
    legacy: "Legacy",
    fast: "Fast",

    set_api_key_title: "Set API Key",
    update_api_key_title: "Update API Key",
    api_key_info: "Enter your API key. Provider is detected automatically:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... or gsk_...",
    save: "Save",
    cancel: "Cancel",
    ok: "OK",
    remove_all_keys: "Remove All Keys",
    unknown_key_format: "Unknown Key Format",
    unknown_key_format_info: "Expected a key starting with sk- (OpenAI) or gsk_ (Groq). Please try again.",
    vocabulary_title: "Vocabulary",
    vocabulary_info: "Names and technical terms to spell exactly, separated by commas. OpenAI, Groq, Whisper on this Mac and post-processing use them.",

    recording_failed: "Recording Failed",
    microphone_silent: "Microphone Silent",
    microphone_silent_info: "No audio signal detected. Check if your AirPods microphone is available or switch to the built-in mic.",
    accessibility_required: "Accessibility Permission Required",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak can't paste automatically. Grant permission in System Settings → Privacy & Security → {setting}, then paste manually with ⌘V."
        )
    },
    accessibility_list: "Accessibility",
    device_control_list: "Device Control and Data Access",
    transcription_failed: "Transcription Failed",
    transcribed_locally: "Transcribed on This Mac",
    transcribed_locally_info: |reason| format!("{reason} A local model transcribed instead."),
    post_processing_failed: "Post-Processing Failed",
    post_processing_failed_info: |reason| {
        format!("{reason} The text was pasted without post-processing.")
    },
    no_api_key: "No API Key",
    no_api_key_info: |provider, menu_item| {
        format!("No {provider} API key configured. Add one via '{menu_item}' in the menu.")
    },
    login_item_error: "Login Item Error",
    login_item_error_info: |menu_item| {
        format!(
            "Failed to update '{menu_item}'. Try toggling it in System Settings → General → Login Items & Extensions."
        )
    },
    api_key_error: "API Key Error",
    api_key_not_saved: "The API key could not be saved. Try again.",
    api_keys_not_removed: "The API keys could not be removed. Try again.",
    download_failed: "Download Failed",
    download_failed_info: |model| {
        format!("{model} could not be downloaded. Check the internet connection and try again.")
    },
    cannot_delete: "Cannot Delete",
    cannot_delete_info: "Wait for the current operation to finish before deleting a model.",
    delete_failed: "Delete Failed",
    delete_failed_info: |model| format!("{model} could not be deleted. Try again."),

    no_api_key_error: |provider| {
        format!("No {provider} API key configured. Set it via the menu bar.")
    },
    invalid_response: "Invalid response from server.",
    authentication_failed: "API authentication failed. Check your API key.",
    rate_limited: "Rate limit exceeded. Please wait a moment.",
    server_error: |status| format!("Server error ({status}). Try again later."),
    api_error: |status| format!("API error ({status}). Check the log for details."),
    model_not_available: |menu_item| {
        format!("Local model not available. Download it via '{menu_item}'.")
    },
    network_error: |message| format!("Network error: {message}"),
    key_rejected: |provider| format!("{provider} rejected the API key."),
    model_retired: |provider| format!("{provider} no longer offers the post-processing model."),
    request_rejected: |provider, status| {
        format!("{provider} rejected the post-processing request ({status}).")
    },
    microphone_permission_denied: "Lowspeak has no access to the microphone. Allow it in System Settings → Privacy & Security → Microphone.",
    no_microphone: "No microphone is connected.",
    microphone_error: |detail| format!("The microphone could not be started: {detail}"),
    paste_failed: "Paste Failed",
    paste_failed_info: "The text is on the clipboard. Paste it with ⌘V.",
    request_timed_out: "The request timed out.",
    audio_failed: "The recording could not be processed.",
    local_model_failed: "The local model could not transcribe the recording.",
};
