use super::Texts;

pub static HE: Texts = Texts {
    recent_dictations: "הכתבות אחרונות",
    none_yet: "אין עדיין",
    local: "מקומי",
    not_installed: |model| format!("{model} (לא מותקן)"),
    no_key: |provider| format!("{provider} (אין מפתח)"),
    language: "שפה",
    auto_detect: "זיהוי אוטומטי",
    post_processing: "עיבוד המשך",
    post_processing_off: "כבוי",
    post_processing_cleanup: "ניקוי (מילות מילוי, דקדוק)",
    post_processing_format: "עיצוב (לפי ההקשר)",
    vocabulary_menu: |terms| match terms {
        0 => "אוצר מילים…".to_owned(),
        1 => "אוצר מילים (1 מונח)…".to_owned(),
        terms => format!("אוצר מילים ({terms} מונחים)…"),
    },
    microphone: "מיקרופון",
    system_default: "ברירת המחדל של המערכת",
    not_connected: |microphone| format!("{microphone} (לא מחובר)"),
    play_sound_when_ready: "השמעת צליל כשמוכן",
    manage_models: "ניהול מודלים",
    delete_model: |model| format!("מחיקת {model}"),
    download_model: |model| format!("הורדת {model}"),
    set_api_key: "הגדרת מפתח API…",
    update_api_key: "עדכון מפתח API…",
    open_at_login: "פתיחה בעת ההתחברות",
    check_for_updates: "חיפוש עדכונים…",
    check_updates_automatically: "חיפוש עדכונים באופן אוטומטי",
    install_updates_automatically: "הורדה והתקנה של עדכונים באופן אוטומטי",
    help: "עזרה",
    quick_reference: "מדריך מהיר",
    report_problem: "דיווח על בעיה…",
    suggest_feature: "הצעת תכונה…",
    on_github: "Lowspeak ב-GitHub",
    about: "מידע על Lowspeak",
    license: "רישיון: MIT או Apache 2.0",
    quit: "סיום Lowspeak",

    reference_hold: "החזק כדי להכתיב, שחרר כדי להדביק",
    reference_double_tap: "לחץ פעמיים במהירות כדי להכתיב ללא ידיים",
    reference_shift: "הכתב ללא ידיים",
    reference_stop: "סיים הכתבה ללא ידיים",
    reference_emoji: "אם מקש ה-🌐 פותח את \"אמוג׳י וסמלים\", כבה זאת בהגדרות המקלדת.",
    keyboard_settings: "הגדרות מקלדת",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ".",
    english: "אנגלית",
    languages: |count| format!("{count} שפות"),
    legacy: "דור קודם",
    fast: "מהיר",

    set_api_key_title: "הגדרת מפתח API",
    update_api_key_title: "עדכון מפתח API",
    api_key_info: "הזן את מפתח ה-API שלך. הספק מזוהה אוטומטית:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... או gsk_...",
    save: "שמירה",
    cancel: "ביטול",
    ok: "אישור",
    remove_all_keys: "הסרת כל המפתחות",
    unknown_key_format: "תבנית מפתח לא מוכרת",
    unknown_key_format_info: "נדרש מפתח שמתחיל ב-sk- (OpenAI) או ב-gsk_ (Groq). נסה שוב.",
    vocabulary_title: "אוצר מילים",
    vocabulary_info: "שמות ומונחים טכניים שיש לאיית בדיוק כך, מופרדים בפסיקים. הם משמשים את OpenAI, את Groq, את Whisper ב-Mac זה ואת עיבוד ההמשך.",

    recording_failed: "ההקלטה נכשלה",
    microphone_silent: "אין אות מהמיקרופון",
    microphone_silent_info: "לא זוהה אות שמע. בדוק אם המיקרופון של ה-AirPods זמין, או עבור למיקרופון המובנה.",
    accessibility_required: "נדרשת הרשאת נגישות",
    accessibility_required_info: |setting| {
        format!(
            "ל-Lowspeak אין אפשרות להדביק אוטומטית. הענק הרשאה בהגדרות המערכת ← פרטיות ואבטחה ← {setting}, ולאחר מכן הדבק ידנית באמצעות \u{200E}⌘V."
        )
    },
    accessibility_list: "נגישות",
    device_control_list: "שליטה במכשיר וגישה לנתונים",
    transcription_failed: "התמלול נכשל",
    transcribed_locally: "תומלל ב-Mac זה",
    transcribed_locally_info: |reason| format!("{reason} מודל מקומי ביצע את התמלול במקום זאת."),
    post_processing_failed: "עיבוד ההמשך נכשל",
    post_processing_failed_info: |reason| format!("{reason} הטקסט הודבק ללא עיבוד המשך."),
    no_api_key: "אין מפתח API",
    no_api_key_info: |provider, menu_item| {
        format!("לא הוגדר מפתח API עבור {provider}. הוסף מפתח דרך \"{menu_item}\" בתפריט.")
    },
    login_item_error: "שגיאה בפריט התחברות",
    login_item_error_info: |menu_item| {
        format!(
            "לא ניתן היה לעדכן את \"{menu_item}\". נסה להפעיל או לכבות זאת בהגדרות המערכת ← כללי ← פריטי התחברות והרחבות."
        )
    },
    api_key_error: "שגיאה במפתח API",
    api_key_not_saved: "לא ניתן היה לשמור את מפתח ה-API. נסה שוב.",
    api_keys_not_removed: "לא ניתן היה להסיר את מפתחות ה-API. נסה שוב.",
    download_failed: "ההורדה נכשלה",
    download_failed_info: |model| {
        format!("לא ניתן היה להוריד את {model}. בדוק את החיבור לאינטרנט ונסה שוב.")
    },
    cannot_delete: "לא ניתן למחוק",
    cannot_delete_info: "המתן לסיום הפעולה הנוכחית לפני מחיקת מודל.",
    delete_failed: "המחיקה נכשלה",
    delete_failed_info: |model| format!("לא ניתן היה למחוק את {model}. נסה שוב."),

    no_api_key_error: |provider| {
        format!("לא הוגדר מפתח API עבור {provider}. הגדר אותו דרך שורת התפריטים.")
    },
    invalid_response: "תגובה לא חוקית מהשרת.",
    authentication_failed: "אימות ה-API נכשל. בדוק את מפתח ה-API.",
    rate_limited: "חריגה ממגבלת הבקשות. המתן רגע.",
    server_error: |status| format!("שגיאת שרת ({status}). נסה שוב מאוחר יותר."),
    api_error: |status| format!("שגיאת API ({status}). פרטים נוספים ביומן."),
    model_not_available: |menu_item| {
        format!("המודל המקומי אינו זמין. הורד אותו דרך \"{menu_item}\".")
    },
    network_error: |message| format!("שגיאת רשת: {message}"),
    key_rejected: |provider| format!("השירות {provider} דחה את מפתח ה-API."),
    model_retired: |provider| format!("השירות {provider} כבר לא מציע את מודל עיבוד ההמשך."),
    request_rejected: |provider, status| {
        format!("השירות {provider} דחה את בקשת עיבוד ההמשך ({status}).")
    },
    microphone_permission_denied: "ל-Lowspeak אין גישה למיקרופון. הענק גישה בהגדרות המערכת ← פרטיות ואבטחה ← מיקרופון.",
    no_microphone: "אין מיקרופון מחובר.",
    microphone_error: |detail| format!("לא ניתן היה להפעיל את המיקרופון: {detail}"),
    paste_failed: "ההדבקה נכשלה",
    paste_failed_info: "הטקסט נמצא בלוח הגזירים. הדבק אותו באמצעות \u{200E}⌘V.",
    request_timed_out: "תם הזמן הקצוב לבקשה.",
    audio_failed: "לא ניתן היה לעבד את ההקלטה.",
    local_model_failed: "המודל המקומי לא הצליח לתמלל את ההקלטה.",
};
