use super::Texts;

pub static MS: Texts = Texts {
    recent_dictations: "Perencanaan Terbaharu",
    none_yet: "Belum Ada",
    local: "Setempat",
    not_installed: |model| format!("{model} (Tidak Dipasang)"),
    no_key: |provider| format!("{provider} (Tiada Kunci)"),
    language: "Bahasa",
    auto_detect: "Kesan Automatik",
    post_processing: "Pascapemprosesan",
    post_processing_off: "Tak Aktif",
    post_processing_cleanup: "Bersihkan (Kata Pengisi, Tatabahasa)",
    post_processing_format: "Formatkan (Ikut Konteks)",
    vocabulary_menu: |terms| match terms {
        0 => "Kosa Kata…".to_owned(),
        terms => format!("Kosa Kata ({terms} Istilah)…"),
    },
    microphone: "Mikrofon",
    system_default: "Lalai Sistem",
    not_connected: |microphone| format!("{microphone} (Tidak Disambung)"),
    play_sound_when_ready: "Mainkan Bunyi Apabila Sedia",
    manage_models: "Urus Model",
    delete_model: |model| format!("Padam {model}"),
    download_model: |model| format!("Muat Turun {model}"),
    set_api_key: "Setkan Kunci API…",
    update_api_key: "Kemas Kini Kunci API…",
    open_at_login: "Buka apabila Log Masuk",
    help: "Bantuan",
    quick_reference: "Rujukan Pantas",
    report_problem: "Laporkan Masalah…",
    suggest_feature: "Cadangkan Ciri…",
    on_github: "Lowspeak di GitHub",
    about: "Perihal Lowspeak",
    license: "Lesen: MIT atau Apache 2.0",
    quit: "Keluar daripada Lowspeak",

    reference_hold: "Tahan untuk perencanaan, lepaskan untuk menampal",
    reference_double_tap: "Tekan dua kali dengan pantas untuk perencanaan bebas tangan",
    reference_shift: "Perencanaan bebas tangan",
    reference_stop: "Tamatkan perencanaan bebas tangan",
    reference_emoji: "Jika kekunci 🌐 membuka Emoji & Simbol, setkannya kepada “Jangan Buat Apa-apa” dalam seting Papan Kekunci.",
    keyboard_settings: "Seting Papan Kekunci",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ".",
    english: "Bahasa Inggeris",
    languages: |count| format!("{count} Bahasa"),
    legacy: "Lama",
    fast: "Cepat",

    set_api_key_title: "Setkan Kunci API",
    update_api_key_title: "Kemas Kini Kunci API",
    api_key_info: "Masukkan kunci API anda. Penyedia dikesan secara automatik:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... atau gsk_...",
    save: "Simpan",
    cancel: "Batal",
    ok: "OK",
    remove_all_keys: "Keluarkan Semua Kunci",
    unknown_key_format: "Format Kunci Tidak Diketahui",
    unknown_key_format_info: "Kunci mesti bermula dengan sk- (OpenAI) atau gsk_ (Groq). Sila cuba lagi.",
    vocabulary_title: "Kosa Kata",
    vocabulary_info: "Nama dan istilah teknikal yang perlu dieja dengan tepat, dipisahkan dengan koma. OpenAI, Groq, Whisper pada Mac ini dan pascapemprosesan menggunakannya.",

    recording_failed: "Rakaman Gagal",
    microphone_silent: "Mikrofon Senyap",
    microphone_silent_info: "Tiada isyarat audio dikesan. Semak sama ada mikrofon AirPods anda tersedia atau tukar kepada mikrofon terbina dalam.",
    accessibility_required: "Keizinan Kebolehcapaian Diperlukan",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak tidak dapat menampal secara automatik. Berikan keizinan dalam Seting Sistem → Privasi & Keselamatan → {setting}, kemudian tampal secara manual dengan ⌘V."
        )
    },
    accessibility_list: "Kebolehcapaian",
    device_control_list: "Kawalan Peranti dan Akses Data",
    transcription_failed: "Transkripsi Gagal",
    transcribed_locally: "Ditranskripsi pada Mac Ini",
    transcribed_locally_info: |reason| {
        format!("{reason} Model setempat telah mentranskripsi rakaman sebagai gantinya.")
    },
    post_processing_failed: "Pascapemprosesan Gagal",
    post_processing_failed_info: |reason| {
        format!("{reason} Teks telah ditampal tanpa pascapemprosesan.")
    },
    no_api_key: "Tiada Kunci API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Tiada kunci API {provider} dikonfigurasikan. Tambahkan satu melalui “{menu_item}” dalam menu."
        )
    },
    login_item_error: "Ralat Item Log Masuk",
    login_item_error_info: |menu_item| {
        format!(
            "Gagal mengemas kini “{menu_item}”. Cuba hidupkan atau matikannya dalam Seting Sistem → Umum → Item Log Masuk & Sambungan."
        )
    },
    api_key_error: "Ralat Kunci API",
    api_key_not_saved: "Kunci API tidak dapat disimpan. Cuba lagi.",
    api_keys_not_removed: "Kunci API tidak dapat dikeluarkan. Cuba lagi.",
    download_failed: "Muat Turun Gagal",
    download_failed_info: |model| {
        format!("Gagal memuat turun {model}. Semak sambungan internet, kemudian cuba lagi.")
    },
    cannot_delete: "Tidak Dapat Memadam",
    cannot_delete_info: "Tunggu sehingga operasi semasa selesai sebelum memadam model.",
    delete_failed: "Pemadaman Gagal",
    delete_failed_info: |model| format!("Gagal memadam {model}. Cuba lagi."),

    no_api_key_error: |provider| {
        format!("Tiada kunci API {provider} dikonfigurasikan. Setkannya melalui bar menu.")
    },
    invalid_response: "Respons tidak sah daripada pelayan.",
    authentication_failed: "Pengesahan API gagal. Semak kunci API anda.",
    rate_limited: "Had kadar permintaan telah dilampaui. Sila tunggu sebentar.",
    server_error: |status| format!("Ralat pelayan ({status}). Cuba lagi kemudian."),
    api_error: |status| format!("Ralat API ({status}). Semak log untuk butiran."),
    model_not_available: |menu_item| {
        format!("Model setempat tidak tersedia. Muat turunnya melalui “{menu_item}”.")
    },
    network_error: |message| format!("Ralat rangkaian: {message}"),
    key_rejected: |provider| format!("{provider} menolak kunci API."),
    model_retired: |provider| format!("{provider} tidak lagi menawarkan model pascapemprosesan."),
    request_rejected: |provider, status| {
        format!("{provider} menolak permintaan pascapemprosesan ({status}).")
    },
    microphone_permission_denied: "Lowspeak tidak mempunyai akses kepada mikrofon. Benarkannya dalam Seting Sistem → Privasi & Keselamatan → Mikrofon.",
    no_microphone: "Tiada mikrofon disambungkan.",
    microphone_error: |detail| format!("Mikrofon tidak dapat dimulakan: {detail}"),
    paste_failed: "Penampalan Gagal",
    paste_failed_info: "Teks berada dalam papan klip. Tampalkannya dengan ⌘V.",
    request_timed_out: "Permintaan telah tamat masa.",
    audio_failed: "Rakaman tidak dapat diproses.",
    local_model_failed: "Model setempat tidak dapat mentranskripsi rakaman.",
};
