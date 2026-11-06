use super::Texts;

fn counted(count: usize, [one, two, few, many, other]: [&str; 5]) -> String {
    let noun = match (count, count % 100) {
        (1, _) => one,
        (2, _) => two,
        (_, 3..=10) => few,
        (_, 11..=99) => many,
        _ => other,
    };
    format!("{count} {noun}")
}

pub static AR: Texts = Texts {
    recent_dictations: "الإملاءات الأخيرة",
    none_yet: "لا شيء بعد",
    local: "محلي",
    not_installed: |model| format!("{model} (غير مثبت)"),
    no_key: |provider| format!("{provider} (بلا مفتاح)"),
    language: "اللغة",
    auto_detect: "اكتشاف تلقائي",
    post_processing: "المعالجة اللاحقة",
    post_processing_off: "إيقاف",
    post_processing_cleanup: "تنقيح (كلمات الحشو، القواعد)",
    post_processing_format: "تنسيق (حسب السياق)",
    vocabulary_menu: |terms| match terms {
        0 => "المفردات…".to_owned(),
        terms => format!(
            "المفردات ({})…",
            counted(terms, ["مصطلح", "مصطلحان", "مصطلحات", "مصطلحًا", "مصطلح"])
        ),
    },
    microphone: "الميكروفون",
    system_default: "الافتراضي للنظام",
    not_connected: |microphone| format!("{microphone} (غير متصل)"),
    play_sound_when_ready: "تشغيل صوت عند الجاهزية",
    manage_models: "إدارة النماذج",
    delete_model: |model| format!("حذف {model}"),
    download_model: |model| format!("تنزيل {model}"),
    set_api_key: "تعيين مفتاح API…",
    update_api_key: "تحديث مفتاح API…",
    open_at_login: "فتح عند الدخول",
    help: "مساعدة",
    quick_reference: "مرجع سريع",
    report_problem: "الإبلاغ عن مشكلة…",
    suggest_feature: "اقتراح ميزة…",
    on_github: "Lowspeak على GitHub",
    about: "حول Lowspeak",
    license: "الترخيص: MIT أو Apache 2.0",
    quit: "إنهاء Lowspeak",

    reference_hold: "اضغط مطولًا للإملاء، وارفع إصبعك للصق",
    reference_double_tap: "اضغط مرتين بسرعة للإملاء دون استخدام اليدين",
    reference_shift: "الإملاء دون استخدام اليدين",
    reference_stop: "إنهاء الإملاء دون استخدام اليدين",
    reference_emoji: "إذا كان مفتاح 🌐 يفتح إيموجي والرموز، فأوقف ذلك في إعدادات لوحة المفاتيح.",
    keyboard_settings: "إعدادات لوحة المفاتيح",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ".",
    english: "الإنجليزية",
    languages: |count| counted(count, ["لغة", "لغتان", "لغات", "لغةً", "لغة"]),
    legacy: "قديم",
    fast: "سريع",

    set_api_key_title: "تعيين مفتاح API",
    update_api_key_title: "تحديث مفتاح API",
    api_key_info: "أدخل مفتاح API. يُكتشف المزوّد تلقائيًا:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... أو gsk_...",
    save: "حفظ",
    cancel: "إلغاء",
    ok: "موافق",
    remove_all_keys: "إزالة كل المفاتيح",
    unknown_key_format: "تنسيق مفتاح غير معروف",
    unknown_key_format_info: "يجب أن يبدأ المفتاح بـ sk- (OpenAI) أو gsk_ (Groq). حاول مرة أخرى.",
    vocabulary_title: "المفردات",
    vocabulary_info: "أسماء ومصطلحات تقنية تُكتب كما هي تمامًا، مفصولة بفواصل. يستخدمها OpenAI وGroq وWhisper على جهاز Mac هذا والمعالجة اللاحقة.",

    recording_failed: "فشل التسجيل",
    microphone_silent: "الميكروفون صامت",
    microphone_silent_info: "لم تُكتشف أي إشارة صوتية. تحقق من أن ميكروفون AirPods متاح أو انتقل إلى الميكروفون المدمج.",
    accessibility_required: "يلزم إذن تسهيلات الاستخدام",
    accessibility_required_info: |setting| {
        format!(
            "يتعذر على Lowspeak اللصق تلقائيًا. امنح الإذن في إعدادات النظام ← الخصوصية والأمن ← {setting}، ثم الصق يدويًا باستخدام \u{200E}⌘V."
        )
    },
    accessibility_list: "تسهيلات الاستخدام",
    device_control_list: "التحكم في الأجهزة والوصول إلى البيانات",
    transcription_failed: "فشل النسخ",
    transcribed_locally: "تم النسخ على جهاز Mac هذا",
    transcribed_locally_info: |reason| format!("{reason} تولّى نموذج محلي النسخ بدلًا من ذلك."),
    post_processing_failed: "فشلت المعالجة اللاحقة",
    post_processing_failed_info: |reason| format!("{reason} تم لصق النص دون معالجة لاحقة."),
    no_api_key: "لا يوجد مفتاح API",
    no_api_key_info: |provider, menu_item| {
        format!("لم يُعيَّن مفتاح API لخدمة {provider}. أضف مفتاحًا عبر \"{menu_item}\" في القائمة.")
    },
    login_item_error: "خطأ في عنصر الدخول",
    login_item_error_info: |menu_item| {
        format!(
            "تعذّر تحديث \"{menu_item}\". جرّب تبديله في إعدادات النظام ← عام ← عناصر الدخول والملحقات."
        )
    },
    api_key_error: "خطأ في مفتاح API",
    api_key_not_saved: "تعذّر حفظ مفتاح API. حاول مرة أخرى.",
    api_keys_not_removed: "تعذّرت إزالة مفاتيح API. حاول مرة أخرى.",
    download_failed: "فشل التنزيل",
    download_failed_info: |model| {
        format!("تعذّر تنزيل {model}. تحقق من الاتصال بالإنترنت ثم حاول مرة أخرى.")
    },
    cannot_delete: "يتعذر الحذف",
    cannot_delete_info: "انتظر حتى تنتهي العملية الجارية قبل حذف أي نموذج.",
    delete_failed: "فشل الحذف",
    delete_failed_info: |model| format!("تعذّر حذف {model}. حاول مرة أخرى."),

    no_api_key_error: |provider| {
        format!("لم يُعيَّن مفتاح API لخدمة {provider}. عيّنه من شريط القوائم.")
    },
    invalid_response: "استجابة غير صالحة من الخادم.",
    authentication_failed: "فشلت مصادقة API. تحقق من مفتاح API.",
    rate_limited: "تم تجاوز حد الطلبات. يُرجى الانتظار قليلًا.",
    server_error: |status| format!("خطأ في الخادم ({status}). حاول مرة أخرى لاحقًا."),
    api_error: |status| format!("خطأ في API ({status}). راجع السجل لمعرفة التفاصيل."),
    model_not_available: |menu_item| format!("النموذج المحلي غير متوفر. نزّله عبر \"{menu_item}\"."),
    network_error: |message| format!("خطأ في الشبكة: {message}"),
    key_rejected: |provider| format!("رفضت خدمة {provider} مفتاح API."),
    model_retired: |provider| format!("لم تعد خدمة {provider} توفر نموذج المعالجة اللاحقة."),
    request_rejected: |provider, status| {
        format!("رفضت خدمة {provider} طلب المعالجة اللاحقة ({status}).")
    },
    microphone_permission_denied: "لا يمكن لـ Lowspeak الوصول إلى الميكروفون. امنح الإذن في إعدادات النظام ← الخصوصية والأمن ← الميكروفون.",
    no_microphone: "لا يوجد ميكروفون متصل.",
    microphone_error: |detail| format!("تعذر تشغيل الميكروفون: {detail}"),
    paste_failed: "فشل اللصق",
    paste_failed_info: "النص موجود في الحافظة. الصقه باستخدام \u{200E}⌘V.",
    request_timed_out: "انتهت مهلة الطلب.",
    audio_failed: "تعذرت معالجة التسجيل.",
    local_model_failed: "تعذر على النموذج المحلي نسخ التسجيل.",
};
