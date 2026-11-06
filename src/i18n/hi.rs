use super::Texts;

pub static HI: Texts = Texts {
    recent_dictations: "हाल के डिक्टेशन",
    none_yet: "अभी तक कोई नहीं",
    local: "लोकल",
    not_installed: |model| format!("{model} (इंस्टॉल नहीं है)"),
    no_key: |provider| format!("{provider} (कोई कुंजी नहीं)"),
    language: "भाषा",
    auto_detect: "स्वतः पहचानें",
    post_processing: "पोस्ट-प्रोसेसिंग",
    post_processing_off: "बंद",
    post_processing_cleanup: "सफ़ाई (फ़िलर शब्द, व्याकरण)",
    post_processing_format: "फ़ॉर्मैट (संदर्भ के अनुसार)",
    vocabulary_menu: |terms| match terms {
        0 => "शब्दावली…".to_owned(),
        terms => format!("शब्दावली ({terms} शब्द)…"),
    },
    microphone: "माइक्रोफ़ोन",
    system_default: "सिस्टम डिफ़ॉल्ट",
    not_connected: |microphone| format!("{microphone} (कनेक्ट नहीं है)"),
    play_sound_when_ready: "तैयार होने पर ध्वनि चलाएँ",
    manage_models: "मॉडल प्रबंधित करें",
    delete_model: |model| format!("{model} डिलीट करें"),
    download_model: |model| format!("{model} डाउनलोड करें"),
    set_api_key: "API कुंजी सेट करें…",
    update_api_key: "API कुंजी अपडेट करें…",
    open_at_login: "लॉगइन पर खोलें",
    help: "सहायता",
    quick_reference: "त्वरित संदर्भ",
    report_problem: "समस्या रिपोर्ट करें…",
    suggest_feature: "सुविधा सुझाएँ…",
    on_github: "GitHub पर Lowspeak",
    about: "Lowspeak परिचय",
    license: "लाइसेंस: MIT या Apache 2.0",
    quit: "Lowspeak बंद करें",

    reference_hold: "बोलने के लिए दबाए रखें, पेस्ट करने के लिए छोड़ें",
    reference_double_tap: "हैंड्स-फ़्री डिक्टेशन के लिए जल्दी से दो बार दबाएँ",
    reference_shift: "हैंड्स-फ़्री डिक्टेट करें",
    reference_stop: "हैंड्स-फ़्री डिक्टेशन समाप्त करें",
    reference_emoji: "अगर 🌐 कुंजी ईमोजी और चिह्न खोलती है, तो इसे कीबोर्ड सेटिंग में बंद करें।",
    keyboard_settings: "कीबोर्ड सेटिंग",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ".",
    english: "अंग्रेज़ी",
    languages: |count| format!("{count} भाषाएँ"),
    legacy: "पुराना",
    fast: "तेज़",

    set_api_key_title: "API कुंजी सेट करें",
    update_api_key_title: "API कुंजी अपडेट करें",
    api_key_info: "अपनी API कुंजी दर्ज करें। प्रदाता अपने आप पहचाना जाता है:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... या gsk_...",
    save: "सहेजें",
    cancel: "रद्द करें",
    ok: "ठीक है",
    remove_all_keys: "सभी कुंजियाँ हटाएँ",
    unknown_key_format: "अज्ञात कुंजी फ़ॉर्मैट",
    unknown_key_format_info: "sk- (OpenAI) या gsk_ (Groq) से शुरू होने वाली कुंजी अपेक्षित है। कृपया फिर से कोशिश करें।",
    vocabulary_title: "शब्दावली",
    vocabulary_info: "ऐसे नाम और तकनीकी शब्द जिनकी वर्तनी ठीक यही रहनी चाहिए, अल्पविराम से अलग करके दर्ज करें। OpenAI, Groq, इस Mac पर Whisper और पोस्ट-प्रोसेसिंग इनका उपयोग करते हैं।",

    recording_failed: "रिकॉर्डिंग विफल",
    microphone_silent: "माइक्रोफ़ोन से कोई आवाज़ नहीं",
    microphone_silent_info: "कोई ऑडियो सिग्नल नहीं मिला। जाँचें कि आपके AirPods का माइक्रोफ़ोन उपलब्ध है या नहीं, या बिल्ट-इन माइक्रोफ़ोन पर स्विच करें।",
    accessibility_required: "ऐक्सेसिबिलिटी अनुमति आवश्यक",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak अपने आप पेस्ट नहीं कर सकता। सिस्टम सेटिंग → गोपनीयता और सुरक्षा → {setting} में अनुमति दें, फिर ⌘V से मैन्युअल रूप से पेस्ट करें।"
        )
    },
    accessibility_list: "ऐक्सेसिबिलिटी",
    device_control_list: "डिवाइस नियंत्रण और डेटा ऐक्सेस",
    transcription_failed: "ट्रांसक्रिप्शन विफल",
    transcribed_locally: "इस Mac पर ट्रांसक्राइब किया गया",
    transcribed_locally_info: |reason| format!("{reason} इसके बजाय एक लोकल मॉडल ने ट्रांसक्राइब किया।"),
    post_processing_failed: "पोस्ट-प्रोसेसिंग विफल",
    post_processing_failed_info: |reason| {
        format!("{reason} टेक्स्ट बिना पोस्ट-प्रोसेसिंग के पेस्ट किया गया।")
    },
    no_api_key: "कोई API कुंजी नहीं",
    no_api_key_info: |provider, menu_item| {
        format!("{provider} API कुंजी कॉन्फ़िगर नहीं है। मेनू में “{menu_item}” से कुंजी जोड़ें।")
    },
    login_item_error: "लॉगइन आइटम त्रुटि",
    login_item_error_info: |menu_item| {
        format!(
            "“{menu_item}” अपडेट नहीं किया जा सका। सिस्टम सेटिंग → सामान्य → लॉगइन आइटम और एक्सटेंशन में इसे बंद और फिर चालू करके देखें।"
        )
    },
    api_key_error: "API कुंजी त्रुटि",
    api_key_not_saved: "API कुंजी सहेजी नहीं जा सकी। फिर से कोशिश करें।",
    api_keys_not_removed: "API कुंजियाँ हटाई नहीं जा सकीं। फिर से कोशिश करें।",
    download_failed: "डाउनलोड विफल",
    download_failed_info: |model| {
        format!("{model} डाउनलोड नहीं हो सका। इंटरनेट कनेक्शन जाँचें और फिर से कोशिश करें।")
    },
    cannot_delete: "डिलीट नहीं किया जा सकता",
    cannot_delete_info: "मॉडल डिलीट करने से पहले मौजूदा प्रक्रिया पूरी होने तक प्रतीक्षा करें।",
    delete_failed: "डिलीट विफल",
    delete_failed_info: |model| format!("{model} डिलीट नहीं हो सका। फिर से कोशिश करें।"),

    no_api_key_error: |provider| format!("{provider} API कुंजी कॉन्फ़िगर नहीं है। इसे मेनू बार से सेट करें।"),
    invalid_response: "सर्वर से अमान्य प्रतिक्रिया मिली।",
    authentication_failed: "API प्रमाणीकरण विफल। अपनी API कुंजी जाँचें।",
    rate_limited: "अनुरोध सीमा पार हो गई। कृपया थोड़ी देर प्रतीक्षा करें।",
    server_error: |status| format!("सर्वर त्रुटि ({status})। बाद में फिर से कोशिश करें।"),
    api_error: |status| format!("API त्रुटि ({status})। विवरण के लिए लॉग देखें।"),
    model_not_available: |menu_item| {
        format!("लोकल मॉडल उपलब्ध नहीं है। इसे “{menu_item}” से डाउनलोड करें।")
    },
    network_error: |message| format!("नेटवर्क त्रुटि: {message}"),
    key_rejected: |provider| format!("{provider} ने API कुंजी अस्वीकार कर दी।"),
    model_retired: |provider| format!("{provider} अब पोस्ट-प्रोसेसिंग मॉडल उपलब्ध नहीं कराता।"),
    request_rejected: |provider, status| {
        format!("{provider} ने पोस्ट-प्रोसेसिंग अनुरोध अस्वीकार कर दिया ({status})।")
    },
    microphone_permission_denied: "Lowspeak के पास माइक्रोफ़ोन की ऐक्सेस नहीं है। सिस्टम सेटिंग → गोपनीयता और सुरक्षा → माइक्रोफ़ोन में अनुमति दें।",
    no_microphone: "कोई माइक्रोफ़ोन कनेक्ट नहीं है।",
    microphone_error: |detail| format!("माइक्रोफ़ोन शुरू नहीं हो सका: {detail}"),
    paste_failed: "पेस्ट विफल",
    paste_failed_info: "टेक्स्ट क्लिपबोर्ड पर है। ⌘V से पेस्ट करें।",
    request_timed_out: "अनुरोध का समय समाप्त हो गया।",
    audio_failed: "रिकॉर्डिंग प्रोसेस नहीं हो सकी।",
    local_model_failed: "लोकल मॉडल रिकॉर्डिंग को ट्रांसक्राइब नहीं कर सका।",
};
