use super::Texts;

pub static ID: Texts = Texts {
    recent_dictations: "Dikte Terbaru",
    none_yet: "Belum Ada",
    local: "Lokal",
    not_installed: |model| format!("{model} (Tidak Terinstal)"),
    no_key: |provider| format!("{provider} (Tanpa Kunci)"),
    language: "Bahasa",
    auto_detect: "Deteksi Otomatis",
    post_processing: "Pascapemrosesan",
    post_processing_off: "Mati",
    post_processing_cleanup: "Rapikan (Kata Pengisi, Tata Bahasa)",
    post_processing_format: "Format (Sesuai Konteks)",
    vocabulary_menu: |terms| match terms {
        0 => "Kosakata…".to_owned(),
        terms => format!("Kosakata ({terms} Istilah)…"),
    },
    microphone: "Mikrofon",
    system_default: "Default Sistem",
    not_connected: |microphone| format!("{microphone} (Tidak Terhubung)"),
    play_sound_when_ready: "Putar Bunyi Saat Siap",
    manage_models: "Kelola Model",
    delete_model: |model| format!("Hapus {model}"),
    download_model: |model| format!("Unduh {model}"),
    set_api_key: "Atur Kunci API…",
    update_api_key: "Perbarui Kunci API…",
    open_at_login: "Buka saat Masuk",
    check_for_updates: "Periksa Pembaruan…",
    check_updates_automatically: "Periksa Pembaruan Secara Otomatis",
    install_updates_automatically: "Unduh dan Instal Pembaruan Secara Otomatis",
    help: "Bantuan",
    quick_reference: "Referensi Cepat",
    report_problem: "Laporkan Masalah…",
    suggest_feature: "Usulkan Fitur…",
    on_github: "Lowspeak di GitHub",
    about: "Mengenai Lowspeak",
    license: "Lisensi: MIT atau Apache 2.0",
    quit: "Tutup Lowspeak",

    reference_hold: "Tahan untuk mendikte, lepaskan untuk menempel",
    reference_double_tap: "Tekan dua kali dengan cepat untuk mendikte tanpa tangan",
    reference_shift: "Dikte tanpa tangan",
    reference_stop: "Akhiri dikte tanpa tangan",
    reference_emoji: "Jika tombol 🌐 membuka Emoji & Simbol, atur agar tidak melakukan apa pun di pengaturan Papan Ketik.",
    keyboard_settings: "Pengaturan Papan Ketik",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "Bahasa Inggris",
    languages: |count| format!("{count} Bahasa"),
    legacy: "Lama",
    fast: "Cepat",

    set_api_key_title: "Atur Kunci API",
    update_api_key_title: "Perbarui Kunci API",
    api_key_info: "Masukkan kunci API Anda. Penyedia terdeteksi secara otomatis:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... atau gsk_...",
    save: "Simpan",
    cancel: "Batalkan",
    ok: "OKE",
    remove_all_keys: "Hapus Semua Kunci",
    unknown_key_format: "Format Kunci Tidak Dikenal",
    unknown_key_format_info: "Kunci harus diawali dengan sk- (OpenAI) atau gsk_ (Groq). Coba lagi.",
    vocabulary_title: "Kosakata",
    vocabulary_info: "Nama dan istilah teknis yang harus dieja persis seperti ini, dipisahkan dengan koma. OpenAI, Groq, Whisper di Mac ini, dan pascapemrosesan menggunakannya.",

    recording_failed: "Perekaman Gagal",
    microphone_silent: "Mikrofon Senyap",
    microphone_silent_info: "Tidak ada sinyal audio yang terdeteksi. Periksa apakah mikrofon AirPods Anda tersedia, atau beralih ke mikrofon internal.",
    accessibility_required: "Izin Aksesibilitas Diperlukan",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak tidak dapat menempel secara otomatis. Berikan izin di Pengaturan Sistem → Privasi & Keamanan → {setting}, lalu tempel secara manual dengan ⌘V."
        )
    },
    accessibility_list: "Aksesibilitas",
    device_control_list: "Kontrol Perangkat dan Akses Data",
    transcription_failed: "Transkripsi Gagal",
    transcribed_locally: "Ditranskripsi di Mac Ini",
    transcribed_locally_info: |reason| {
        format!("{reason} Model lokal melakukan transkripsi sebagai gantinya.")
    },
    post_processing_failed: "Pascapemrosesan Gagal",
    post_processing_failed_info: |reason| format!("{reason} Teks ditempel tanpa pascapemrosesan."),
    no_api_key: "Tidak Ada Kunci API",
    no_api_key_info: |provider, menu_item| {
        format!("Kunci API {provider} belum diatur. Tambahkan melalui “{menu_item}” di menu.")
    },
    login_item_error: "Kesalahan Item Masuk",
    login_item_error_info: |menu_item| {
        format!(
            "Gagal memperbarui “{menu_item}”. Coba ubah di Pengaturan Sistem → Umum → Item Masuk & Ekstensi."
        )
    },
    api_key_error: "Kesalahan Kunci API",
    api_key_not_saved: "Kunci API tidak dapat disimpan. Coba lagi.",
    api_keys_not_removed: "Kunci API tidak dapat dihapus. Coba lagi.",
    download_failed: "Pengunduhan Gagal",
    download_failed_info: |model| {
        format!("Gagal mengunduh {model}. Periksa koneksi internet, lalu coba lagi.")
    },
    cannot_delete: "Tidak Dapat Menghapus",
    cannot_delete_info: "Tunggu hingga operasi saat ini selesai sebelum menghapus model.",
    delete_failed: "Penghapusan Gagal",
    delete_failed_info: |model| format!("Gagal menghapus {model}. Coba lagi."),

    no_api_key_error: |provider| {
        format!("Kunci API {provider} belum diatur. Atur melalui bar menu.")
    },
    invalid_response: "Respons tidak valid dari server.",
    authentication_failed: "Autentikasi API gagal. Periksa kunci API Anda.",
    rate_limited: "Batas permintaan terlampaui. Harap tunggu sebentar.",
    server_error: |status| format!("Kesalahan server ({status}). Coba lagi nanti."),
    api_error: |status| format!("Kesalahan API ({status}). Lihat log untuk detailnya."),
    model_not_available: |menu_item| {
        format!("Model lokal tidak tersedia. Unduh melalui “{menu_item}”.")
    },
    network_error: |message| format!("Kesalahan jaringan: {message}"),
    key_rejected: |provider| format!("{provider} menolak kunci API."),
    model_retired: |provider| format!("{provider} tidak lagi menyediakan model pascapemrosesan."),
    no_post_processing_key: |first, second| {
        format!(
            "Pascapemrosesan memerlukan kunci API {first} atau {second}. Atur melalui bar menu."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} tidak lagi menyediakan {model}. Pilih model lain di menu.")
    },
    request_rejected: |provider, status| {
        format!("{provider} menolak permintaan pascapemrosesan ({status}).")
    },
    microphone_permission_denied: "Lowspeak tidak memiliki akses ke mikrofon. Izinkan di Pengaturan Sistem → Privasi & Keamanan → Mikrofon.",
    no_microphone: "Tidak ada mikrofon yang terhubung.",
    microphone_error: |detail| format!("Mikrofon tidak dapat dimulai: {detail}"),
    paste_failed: "Penempelan Gagal",
    paste_failed_info: "Teks ada di papan klip. Tempel dengan ⌘V.",
    request_timed_out: "Waktu permintaan habis.",
    audio_failed: "Rekaman tidak dapat diproses.",
    local_model_failed: "Model lokal tidak dapat mentranskripsi rekaman.",
};
