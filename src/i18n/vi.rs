use super::Texts;

pub static VI: Texts = Texts {
    recent_dictations: "Đọc chính tả gần đây",
    none_yet: "Chưa có",
    local: "Cục bộ",
    not_installed: |model| format!("{model} (Chưa cài đặt)"),
    no_key: |provider| format!("{provider} (Không có khóa)"),
    language: "Ngôn ngữ",
    auto_detect: "Tự động phát hiện",
    post_processing: "Hậu xử lý",
    post_processing_off: "Tắt",
    post_processing_cleanup: "Làm gọn (từ đệm, ngữ pháp)",
    post_processing_format: "Định dạng (theo ngữ cảnh)",
    vocabulary_menu: |terms| match terms {
        0 => "Từ vựng…".to_owned(),
        terms => format!("Từ vựng ({terms} thuật ngữ)…"),
    },
    microphone: "Micrô",
    system_default: "Mặc định hệ thống",
    not_connected: |microphone| format!("{microphone} (Không kết nối)"),
    play_sound_when_ready: "Phát âm thanh khi sẵn sàng",
    manage_models: "Quản lý mô hình",
    delete_model: |model| format!("Xóa {model}"),
    download_model: |model| format!("Tải về {model}"),
    set_api_key: "Đặt khóa API…",
    update_api_key: "Cập nhật khóa API…",
    open_at_login: "Mở khi đăng nhập",
    check_for_updates: "Kiểm tra Bản cập nhật…",
    check_updates_automatically: "Tự động Kiểm tra Bản cập nhật",
    install_updates_automatically: "Tự động Tải về và Cài đặt Bản cập nhật",
    help: "Trợ giúp",
    quick_reference: "Tham khảo nhanh",
    report_problem: "Báo cáo sự cố…",
    suggest_feature: "Đề xuất tính năng…",
    on_github: "Lowspeak trên GitHub",
    about: "Giới thiệu về Lowspeak",
    license: "Giấy phép: MIT hoặc Apache 2.0",
    quit: "Thoát Lowspeak",

    reference_hold: "Giữ để đọc chính tả, thả ra để dán",
    reference_double_tap: "Nhấn nhanh hai lần để đọc chính tả rảnh tay",
    reference_shift: "Đọc chính tả rảnh tay",
    reference_stop: "Kết thúc đọc chính tả rảnh tay",
    reference_emoji: "Nếu phím 🌐 mở Biểu tượng & Ký hiệu, hãy đặt phím này thành “Không làm gì” trong cài đặt Bàn phím.",
    keyboard_settings: "Cài đặt bàn phím",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "Tiếng Anh",
    languages: |count| format!("{count} ngôn ngữ"),
    legacy: "Cũ",
    fast: "Nhanh",

    set_api_key_title: "Đặt khóa API",
    update_api_key_title: "Cập nhật khóa API",
    api_key_info: "Nhập khóa API của bạn. Nhà cung cấp được nhận diện tự động:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... hoặc gsk_...",
    save: "Lưu",
    cancel: "Hủy",
    ok: "OK",
    remove_all_keys: "Xóa tất cả khóa",
    unknown_key_format: "Định dạng khóa không xác định",
    unknown_key_format_info: "Khóa phải bắt đầu bằng sk- (OpenAI) hoặc gsk_ (Groq). Vui lòng thử lại.",
    vocabulary_title: "Từ vựng",
    vocabulary_info: "Tên và thuật ngữ chuyên môn cần viết chính xác, phân tách bằng dấu phẩy. OpenAI, Groq, Whisper trên máy Mac này và tính năng hậu xử lý sẽ sử dụng chúng.",

    recording_failed: "Ghi âm không thành công",
    microphone_silent: "Micrô không có tín hiệu",
    microphone_silent_info: "Không phát hiện thấy tín hiệu âm thanh. Hãy kiểm tra xem micrô AirPods có khả dụng không hoặc chuyển sang micrô tích hợp.",
    accessibility_required: "Cần quyền Trợ năng",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak không thể tự động dán. Hãy cấp quyền trong Cài đặt hệ thống → Quyền riêng tư & Bảo mật → {setting}, sau đó dán thủ công bằng ⌘V."
        )
    },
    accessibility_list: "Trợ năng",
    device_control_list: "Điều khiển thiết bị và truy cập dữ liệu",
    transcription_failed: "Chép lại không thành công",
    transcribed_locally: "Đã chép lại trên máy Mac này",
    transcribed_locally_info: |reason| {
        format!("{reason} Một mô hình cục bộ đã chép lại bản ghi âm thay thế.")
    },
    post_processing_failed: "Hậu xử lý không thành công",
    post_processing_failed_info: |reason| {
        format!("{reason} Văn bản đã được dán mà không qua hậu xử lý.")
    },
    no_api_key: "Không có khóa API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Chưa định cấu hình khóa API {provider}. Hãy thêm khóa qua “{menu_item}” trong menu."
        )
    },
    login_item_error: "Lỗi mục đăng nhập",
    login_item_error_info: |menu_item| {
        format!(
            "Không thể cập nhật “{menu_item}”. Hãy thử bật hoặc tắt mục này trong Cài đặt hệ thống → Cài đặt chung → Mục đăng nhập & Phần mở rộng."
        )
    },
    api_key_error: "Lỗi khóa API",
    api_key_not_saved: "Không thể lưu khóa API. Hãy thử lại.",
    api_keys_not_removed: "Không thể xóa các khóa API. Hãy thử lại.",
    download_failed: "Tải về không thành công",
    download_failed_info: |model| {
        format!("Không thể tải về {model}. Hãy kiểm tra kết nối Internet rồi thử lại.")
    },
    cannot_delete: "Không thể xóa",
    cannot_delete_info: "Hãy đợi thao tác hiện tại hoàn tất trước khi xóa mô hình.",
    delete_failed: "Xóa không thành công",
    delete_failed_info: |model| format!("Không thể xóa {model}. Hãy thử lại."),

    no_api_key_error: |provider| {
        format!("Chưa định cấu hình khóa API {provider}. Hãy đặt khóa qua thanh menu.")
    },
    invalid_response: "Phản hồi từ máy chủ không hợp lệ.",
    authentication_failed: "Xác thực API không thành công. Hãy kiểm tra khóa API của bạn.",
    rate_limited: "Đã vượt quá giới hạn yêu cầu. Vui lòng đợi một lát.",
    server_error: |status| format!("Lỗi máy chủ ({status}). Hãy thử lại sau."),
    api_error: |status| format!("Lỗi API ({status}). Hãy xem nhật ký để biết chi tiết."),
    model_not_available: |menu_item| {
        format!("Mô hình cục bộ không khả dụng. Hãy tải về qua “{menu_item}”.")
    },
    network_error: |message| format!("Lỗi mạng: {message}"),
    key_rejected: |provider| format!("{provider} đã từ chối khóa API."),
    model_retired: |provider| format!("{provider} không còn cung cấp mô hình hậu xử lý."),
    request_rejected: |provider, status| {
        format!("{provider} đã từ chối yêu cầu hậu xử lý ({status}).")
    },
    microphone_permission_denied: "Lowspeak không có quyền truy cập micrô. Hãy cho phép trong Cài đặt hệ thống → Quyền riêng tư & Bảo mật → Micrô.",
    no_microphone: "Không có micrô nào được kết nối.",
    microphone_error: |detail| format!("Không thể khởi động micrô: {detail}"),
    paste_failed: "Dán không thành công",
    paste_failed_info: "Văn bản đang ở trong bảng nhớ tạm. Hãy dán bằng ⌘V.",
    request_timed_out: "Yêu cầu đã hết thời gian chờ.",
    audio_failed: "Không thể xử lý bản ghi âm.",
    local_model_failed: "Mô hình cục bộ không thể chép lại bản ghi âm.",
};
