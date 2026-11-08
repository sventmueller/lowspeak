use super::Texts;

pub static TR: Texts = Texts {
    recent_dictations: "Son dikteler",
    none_yet: "Henüz yok",
    retry_failed_dictation: "Başarısız dikteyi yeniden dene",
    local: "Yerel",
    not_installed: |model| format!("{model} (yüklü değil)"),
    no_key: |provider| format!("{provider} (anahtar yok)"),
    language: "Dil",
    auto_detect: "Otomatik algıla",
    post_processing: "Son işleme",
    post_processing_off: "Kapalı",
    post_processing_cleanup: "Temizleme (dolgu sözcükleri, dil bilgisi)",
    post_processing_format: "Biçimlendirme (bağlama duyarlı)",
    vocabulary_menu: |terms| match terms {
        0 => "Sözlük…".to_owned(),
        terms => format!("Sözlük ({terms} terim)…"),
    },
    microphone: "Mikrofon",
    system_default: "Sistem varsayılanı",
    not_connected: |microphone| format!("{microphone} (bağlı değil)"),
    play_sound_when_ready: "Hazır olduğunda ses çal",
    manage_models: "Modelleri yönet",
    delete_model: |model| format!("{model} modelini sil"),
    download_model: |model| format!("{model} modelini indir"),
    set_api_key: "API anahtarını ayarla…",
    update_api_key: "API anahtarını güncelle…",
    open_at_login: "Oturum Açıldığında Aç",
    check_for_updates: "Güncellemeleri Denetle…",
    check_updates_automatically: "Güncellemeleri Otomatik Olarak Denetle",
    install_updates_automatically: "Güncellemeleri Otomatik Olarak İndir ve Yükle",
    help: "Yardım",
    quick_reference: "Hızlı Başvuru",
    report_problem: "Sorun Bildir…",
    suggest_feature: "Özellik Öner…",
    on_github: "GitHub’da Lowspeak",
    about: "Lowspeak Hakkında",
    license: "Lisans: MIT veya Apache 2.0",
    quit: "Lowspeak’ten Çık",

    reference_hold: "Dikte etmek için basılı tutun, yapıştırmak için bırakın",
    reference_double_tap: "Eller serbest dikte için hızlıca iki kez basın",
    reference_shift: "Eller serbest dikte et",
    reference_stop: "Eller serbest dikteyi bitir",
    reference_emoji: "🌐 tuşu Emoji ve Semboller’i açıyorsa bunu Klavye ayarlarından kapatın.",
    keyboard_settings: "Klavye Ayarları",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "İngilizce",
    languages: |count| format!("{count} dil"),
    legacy: "eski",
    fast: "hızlı",

    set_api_key_title: "API anahtarını ayarla",
    update_api_key_title: "API anahtarını güncelle",
    api_key_info: "API anahtarınızı girin. Sağlayıcı otomatik olarak algılanır:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... veya gsk_...",
    save: "Kaydet",
    cancel: "Vazgeç",
    ok: "Tamam",
    remove_all_keys: "Tüm anahtarları kaldır",
    unknown_key_format: "Bilinmeyen anahtar biçimi",
    unknown_key_format_info: "sk- (OpenAI) veya gsk_ (Groq) ile başlayan bir anahtar bekleniyordu. Lütfen yeniden deneyin.",
    vocabulary_title: "Sözlük",
    vocabulary_info: "Tam olarak bu şekilde yazılması gereken adlar ve teknik terimler; virgülle ayırın. OpenAI, Groq, bu Mac'teki Whisper ve Parakeet ile son işleme bunları kullanır.",

    recording_failed: "Kayıt başarısız oldu",
    microphone_silent: "Mikrofon sessiz",
    microphone_silent_info: "Ses sinyali algılanmadı. AirPods mikrofonunun kullanılabilir olduğunu kontrol edin veya dahili mikrofona geçin.",
    accessibility_required: "Erişilebilirlik izni gerekli",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak otomatik olarak yapıştıramıyor. Sistem Ayarları → Gizlilik ve Güvenlik → {setting} bölümünden izin verin, ardından ⌘V ile elle yapıştırın."
        )
    },
    accessibility_list: "Erişilebilirlik",
    device_control_list: "Aygıt Denetimi ve Veri Erişimi",
    transcription_failed: "Metne dönüştürme başarısız oldu",
    retry_failed_dictation_info: |menu_item| {
        format!(
            "Kayıt saklandı: Yeniden metne dönüştürmek için menüde “{menu_item}” öğesini seçin."
        )
    },
    transcribed_locally: "Bu Mac'te metne dönüştürüldü",
    transcribed_locally_info: |reason| {
        format!("{reason} Bunun yerine yerel bir model metne dönüştürdü.")
    },
    post_processing_failed: "Son işleme başarısız oldu",
    post_processing_failed_info: |reason| {
        format!("{reason} Metin son işleme yapılmadan yapıştırıldı.")
    },
    no_api_key: "API anahtarı yok",
    no_api_key_info: |provider, menu_item| {
        format!("{provider} API anahtarı ayarlanmamış. Menüdeki “{menu_item}” öğesiyle ekleyin.")
    },
    login_item_error: "Oturum açma öğesi hatası",
    login_item_error_info: |menu_item| {
        format!(
            "“{menu_item}” güncellenemedi. Sistem Ayarları → Genel → Oturum Açma Öğeleri ve Genişletmeler bölümünden açıp kapatmayı deneyin."
        )
    },
    api_key_error: "API anahtarı hatası",
    api_key_not_saved: "API anahtarı kaydedilemedi. Yeniden deneyin.",
    api_keys_not_removed: "API anahtarları kaldırılamadı. Yeniden deneyin.",
    download_failed: "İndirme başarısız oldu",
    download_failed_info: |model| {
        format!("{model} indirilemedi. İnternet bağlantısını denetleyip yeniden deneyin.")
    },
    cannot_delete: "Silinemiyor",
    cannot_delete_info: "Bir modeli silmeden önce devam eden işlemin bitmesini bekleyin.",
    delete_failed: "Silme başarısız oldu",
    delete_failed_info: |model| format!("{model} silinemedi. Yeniden deneyin."),

    no_api_key_error: |provider| {
        format!("{provider} API anahtarı ayarlanmamış. Menü çubuğundan ayarlayın.")
    },
    invalid_response: "Sunucudan geçersiz yanıt.",
    authentication_failed: "API kimlik doğrulaması başarısız oldu. API anahtarınızı kontrol edin.",
    rate_limited: "İstek sınırı aşıldı. Lütfen biraz bekleyin.",
    server_error: |status| format!("Sunucu hatası ({status}). Daha sonra yeniden deneyin."),
    api_error: |status| format!("API hatası ({status}). Ayrıntılar için günlüğe bakın."),
    model_not_available: |menu_item| {
        format!("Yerel model kullanılamıyor. “{menu_item}” menüsünden indirin.")
    },
    network_error: |message| format!("Ağ hatası: {message}"),
    key_rejected: |provider| format!("{provider} API anahtarını reddetti."),
    quota_exhausted: |provider| {
        format!(
            "{provider}, hesapta kredi kalmadığını bildiriyor. {provider} hesabınıza kredi ekleyin."
        )
    },
    model_retired: |provider| format!("{provider} artık son işleme modelini sunmuyor."),
    no_post_processing_key: |first, second| {
        format!(
            "Son işleme için {first} veya {second} API anahtarı gerekiyor. Menü çubuğundan ayarlayın."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} artık {model} modelini sunmuyor. Menüden başka bir model seçin.")
    },
    request_rejected: |provider, status| {
        format!("{provider} son işleme isteğini reddetti ({status}).")
    },
    microphone_permission_denied: "Lowspeak mikrofona erişemiyor. Sistem Ayarları → Gizlilik ve Güvenlik → Mikrofon bölümünden izin verin.",
    no_microphone: "Bağlı mikrofon yok.",
    microphone_error: |detail| format!("Mikrofon başlatılamadı: {detail}"),
    paste_failed: "Yapıştırma başarısız",
    paste_failed_info: "Metin panoda. ⌘V ile yapıştırın.",
    not_pasted: "Dikte yapıştırılmadı",
    not_pasted_info: "Metin hazır olmadan önce başka bir uygulama öne geldi. Metin panoda. ⌘V ile yapıştırın.",
    request_timed_out: "İstek zaman aşımına uğradı.",
    audio_failed: "Kayıt işlenemedi.",
    local_model_failed: "Yerel model kaydı yazıya dökemedi.",
};
