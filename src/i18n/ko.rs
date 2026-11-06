use super::Texts;

pub static KO: Texts = Texts {
    recent_dictations: "최근 받아쓰기",
    none_yet: "아직 없음",
    local: "로컬",
    not_installed: |model| format!("{model} (설치되지 않음)"),
    no_key: |provider| format!("{provider} (키 없음)"),
    language: "언어",
    auto_detect: "자동 감지",
    post_processing: "후처리",
    post_processing_off: "끔",
    post_processing_cleanup: "정리 (불필요한 말, 문법)",
    post_processing_format: "서식 지정 (상황에 맞게)",
    vocabulary_menu: |terms| match terms {
        0 => "어휘…".to_owned(),
        terms => format!("어휘 ({terms}개)…"),
    },
    microphone: "마이크",
    system_default: "시스템 기본값",
    not_connected: |microphone| format!("{microphone} (연결되지 않음)"),
    play_sound_when_ready: "준비되면 사운드 재생",
    manage_models: "모델 관리",
    delete_model: |model| format!("{model} 삭제"),
    download_model: |model| format!("{model} 다운로드"),
    set_api_key: "API 키 설정…",
    update_api_key: "API 키 업데이트…",
    open_at_login: "로그인 시 열기",
    help: "도움말",
    quick_reference: "빠른 참조",
    report_problem: "문제 보고…",
    suggest_feature: "기능 제안…",
    on_github: "GitHub의 Lowspeak",
    about: "Lowspeak에 관하여",
    license: "라이선스: MIT 또는 Apache 2.0",
    quit: "Lowspeak 종료",

    reference_hold: "길게 눌러 받아쓰고, 놓으면 붙여넣기",
    reference_double_tap: "빠르게 두 번 눌러 핸즈프리로 받아쓰기",
    reference_shift: "핸즈프리로 받아쓰기",
    reference_stop: "핸즈프리 받아쓰기 종료",
    reference_emoji: "🌐 키가 이모지 및 기호를 열면 키보드 설정에서 끄십시오.",
    keyboard_settings: "키보드 설정",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ".",
    english: "영어",
    languages: |count| format!("{count}개 언어"),
    legacy: "구형",
    fast: "고속",

    set_api_key_title: "API 키 설정",
    update_api_key_title: "API 키 업데이트",
    api_key_info: "API 키를 입력하십시오. 제공업체는 자동으로 감지됩니다.\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... 또는 gsk_...",
    save: "저장",
    cancel: "취소",
    ok: "확인",
    remove_all_keys: "모든 키 제거",
    unknown_key_format: "알 수 없는 키 형식",
    unknown_key_format_info: "sk-(OpenAI) 또는 gsk_(Groq)로 시작하는 키여야 합니다. 다시 시도하십시오.",
    vocabulary_title: "어휘",
    vocabulary_info: "정확한 표기로 받아쓸 이름과 전문 용어를 쉼표로 구분하여 입력합니다. OpenAI, Groq, 이 Mac의 Whisper 및 후처리에서 사용됩니다.",

    recording_failed: "녹음 실패",
    microphone_silent: "마이크 신호 없음",
    microphone_silent_info: "오디오 신호가 감지되지 않았습니다. AirPods 마이크를 사용할 수 있는지 확인하거나 내장 마이크로 전환하십시오.",
    accessibility_required: "손쉬운 사용 권한 필요",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak에서 자동으로 붙여넣을 수 없습니다. 시스템 설정 → 개인정보 보호 및 보안 → {setting}에서 권한을 허용한 다음 ⌘V로 직접 붙여넣으십시오."
        )
    },
    accessibility_list: "손쉬운 사용",
    device_control_list: "기기 제어 및 데이터 접근",
    transcription_failed: "텍스트 변환 실패",
    transcribed_locally: "이 Mac에서 텍스트로 변환함",
    transcribed_locally_info: |reason| format!("{reason} 대신 로컬 모델로 변환했습니다."),
    post_processing_failed: "후처리 실패",
    post_processing_failed_info: |reason| format!("{reason} 후처리 없이 텍스트를 붙여넣었습니다."),
    no_api_key: "API 키 없음",
    no_api_key_info: |provider, menu_item| {
        format!("{provider} API 키가 설정되지 않았습니다. 메뉴의 ‘{menu_item}’에서 추가하십시오.")
    },
    login_item_error: "로그인 항목 오류",
    login_item_error_info: |menu_item| {
        format!(
            "‘{menu_item}’ 설정을 변경하지 못했습니다. 시스템 설정 → 일반 → 로그인 항목 및 확장 프로그램에서 전환해 보십시오."
        )
    },
    api_key_error: "API 키 오류",
    api_key_not_saved: "API 키를 저장할 수 없습니다. 다시 시도하십시오.",
    api_keys_not_removed: "API 키를 제거할 수 없습니다. 다시 시도하십시오.",
    download_failed: "다운로드 실패",
    download_failed_info: |model| {
        format!("{model} 다운로드에 실패했습니다. 인터넷 연결을 확인한 후 다시 시도하십시오.")
    },
    cannot_delete: "삭제할 수 없음",
    cannot_delete_info: "현재 작업이 완료될 때까지 기다린 후 모델을 삭제하십시오.",
    delete_failed: "삭제 실패",
    delete_failed_info: |model| format!("{model} 삭제에 실패했습니다. 다시 시도하십시오."),

    no_api_key_error: |provider| {
        format!("{provider} API 키가 설정되지 않았습니다. 메뉴 막대에서 설정하십시오.")
    },
    invalid_response: "서버 응답이 유효하지 않습니다.",
    authentication_failed: "API 인증에 실패했습니다. API 키를 확인하십시오.",
    rate_limited: "요청 한도를 초과했습니다. 잠시 기다리십시오.",
    server_error: |status| format!("서버 오류({status}). 나중에 다시 시도하십시오."),
    api_error: |status| format!("API 오류({status}). 자세한 내용은 로그를 확인하십시오."),
    model_not_available: |menu_item| {
        format!("로컬 모델을 사용할 수 없습니다. ‘{menu_item}’에서 다운로드하십시오.")
    },
    network_error: |message| format!("네트워크 오류: {message}"),
    key_rejected: |provider| format!("{provider}에서 API 키를 거부했습니다."),
    model_retired: |provider| format!("{provider}에서 후처리 모델을 더 이상 제공하지 않습니다."),
    request_rejected: |provider, status| {
        format!("{provider}에서 후처리 요청을 거부했습니다({status}).")
    },
    microphone_permission_denied: "Lowspeak에서 마이크에 접근할 수 없습니다. 시스템 설정 → 개인정보 보호 및 보안 → 마이크에서 허용하십시오.",
    no_microphone: "연결된 마이크가 없습니다.",
    microphone_error: |detail| format!("마이크를 시작할 수 없습니다: {detail}"),
    paste_failed: "붙여넣기 실패",
    paste_failed_info: "텍스트가 클립보드에 있습니다. ⌘V로 붙여넣으십시오.",
    request_timed_out: "요청 시간이 초과되었습니다.",
    audio_failed: "녹음을 처리할 수 없습니다.",
    local_model_failed: "로컬 모델이 녹음을 텍스트로 변환할 수 없습니다.",
};
