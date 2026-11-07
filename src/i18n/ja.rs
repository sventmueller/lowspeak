use super::Texts;

pub static JA: Texts = Texts {
    recent_dictations: "最近の音声入力",
    none_yet: "まだありません",
    local: "ローカル",
    not_installed: |model| format!("{model} (未インストール)"),
    no_key: |provider| format!("{provider} (キーなし)"),
    language: "言語",
    auto_detect: "自動検出",
    post_processing: "後処理",
    post_processing_off: "オフ",
    post_processing_cleanup: "クリーンアップ (つなぎ言葉、文法)",
    post_processing_format: "整形 (文脈に応じて)",
    vocabulary_menu: |terms| match terms {
        0 => "用語集…".to_owned(),
        terms => format!("用語集 ({terms}件)…"),
    },
    microphone: "マイク",
    system_default: "システムのデフォルト",
    not_connected: |microphone| format!("{microphone} (未接続)"),
    play_sound_when_ready: "準備ができたらサウンドを再生",
    manage_models: "モデルを管理",
    delete_model: |model| format!("{model}を削除"),
    download_model: |model| format!("{model}をダウンロード"),
    set_api_key: "APIキーを設定…",
    update_api_key: "APIキーを更新…",
    open_at_login: "ログイン時に開く",
    check_for_updates: "アップデートを確認…",
    check_updates_automatically: "アップデートを自動的に確認",
    install_updates_automatically: "アップデートを自動的にダウンロードしてインストール",
    help: "ヘルプ",
    quick_reference: "クイックリファレンス",
    report_problem: "問題を報告…",
    suggest_feature: "機能を提案…",
    on_github: "GitHubのLowspeak",
    about: "Lowspeakについて",
    license: "ライセンス: MIT または Apache 2.0",
    quit: "Lowspeakを終了",

    reference_hold: "押し続けて音声入力、離すとペースト",
    reference_double_tap: "すばやく2回押すとハンズフリーで音声入力",
    reference_shift: "ハンズフリーで音声入力",
    reference_stop: "ハンズフリー音声入力を終了",
    reference_emoji: "🌐キーで絵文字と記号が開く場合は、キーボード設定でオフにしてください。",
    keyboard_settings: "キーボード設定",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ".",
    english: "英語",
    languages: |count| format!("{count}言語"),
    legacy: "旧モデル",
    fast: "高速",

    set_api_key_title: "APIキーを設定",
    update_api_key_title: "APIキーを更新",
    api_key_info: "APIキーを入力してください。プロバイダは自動的に判別されます：\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... または gsk_...",
    save: "保存",
    cancel: "キャンセル",
    ok: "OK",
    remove_all_keys: "すべてのキーを削除",
    unknown_key_format: "不明なキー形式",
    unknown_key_format_info: "sk-（OpenAI）またはgsk_（Groq）で始まるキーが必要です。もう一度お試しください。",
    vocabulary_title: "用語集",
    vocabulary_info: "正確に表記したい名前や専門用語をカンマで区切って入力します。OpenAI、Groq、このMacのWhisper、および後処理で使用されます。",

    recording_failed: "録音に失敗しました",
    microphone_silent: "マイクが無音です",
    microphone_silent_info: "音声信号が検出されません。AirPodsのマイクが使用可能か確認するか、内蔵マイクに切り替えてください。",
    accessibility_required: "アクセシビリティの許可が必要です",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeakは自動的にペーストできません。システム設定 → プライバシーとセキュリティ → {setting}で許可してから、⌘Vを押して手動でペーストしてください。"
        )
    },
    accessibility_list: "アクセシビリティ",
    device_control_list: "デバイスの制御とデータへのアクセス",
    transcription_failed: "文字起こしに失敗しました",
    transcribed_locally: "このMacで文字起こししました",
    transcribed_locally_info: |reason| {
        format!("{reason}代わりにローカルモデルで文字起こししました。")
    },
    post_processing_failed: "後処理に失敗しました",
    post_processing_failed_info: |reason| {
        format!("{reason}テキストは後処理せずにペーストされました。")
    },
    no_api_key: "APIキーがありません",
    no_api_key_info: |provider, menu_item| {
        format!(
            "{provider}のAPIキーが設定されていません。メニューの「{menu_item}」から追加してください。"
        )
    },
    login_item_error: "ログイン項目のエラー",
    login_item_error_info: |menu_item| {
        format!(
            "「{menu_item}」を変更できませんでした。システム設定 → 一般 → ログイン項目と機能拡張で切り替えてみてください。"
        )
    },
    api_key_error: "APIキーのエラー",
    api_key_not_saved: "APIキーを保存できませんでした。もう一度お試しください。",
    api_keys_not_removed: "APIキーを削除できませんでした。もう一度お試しください。",
    download_failed: "ダウンロードに失敗しました",
    download_failed_info: |model| {
        format!(
            "{model}をダウンロードできませんでした。インターネット接続を確認して、もう一度お試しください。"
        )
    },
    cannot_delete: "削除できません",
    cannot_delete_info: "現在の処理が完了してからモデルを削除してください。",
    delete_failed: "削除に失敗しました",
    delete_failed_info: |model| format!("{model}を削除できませんでした。もう一度お試しください。"),

    no_api_key_error: |provider| {
        format!("{provider}のAPIキーが設定されていません。メニューバーから設定してください。")
    },
    invalid_response: "サーバからの応答が無効です。",
    authentication_failed: "APIの認証に失敗しました。APIキーを確認してください。",
    rate_limited: "リクエストの上限を超えました。しばらくお待ちください。",
    server_error: |status| {
        format!("サーバエラー（{status}）。しばらくしてからもう一度お試しください。")
    },
    api_error: |status| format!("APIエラー（{status}）。詳細はログを確認してください。"),
    model_not_available: |menu_item| {
        format!("ローカルモデルを使用できません。「{menu_item}」からダウンロードしてください。")
    },
    network_error: |message| format!("ネットワークエラー：{message}"),
    key_rejected: |provider| format!("{provider}がAPIキーを拒否しました。"),
    model_retired: |provider| format!("{provider}は後処理用モデルの提供を終了しました。"),
    no_post_processing_key: |first, second| {
        format!(
            "後処理には{first}または{second}のAPIキーが必要です。メニューバーから設定してください。"
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider}は{model}の提供を終了しました。メニューで別のモデルを選択してください。")
    },
    request_rejected: |provider, status| {
        format!("{provider}が後処理のリクエストを拒否しました（{status}）。")
    },
    microphone_permission_denied: "Lowspeakはマイクにアクセスできません。システム設定 → プライバシーとセキュリティ → マイクで許可してください。",
    no_microphone: "マイクが接続されていません。",
    microphone_error: |detail| format!("マイクを開始できませんでした：{detail}"),
    paste_failed: "ペーストに失敗しました",
    paste_failed_info: "テキストはクリップボードにあります。⌘Vでペーストしてください。",
    request_timed_out: "リクエストがタイムアウトしました。",
    audio_failed: "録音を処理できませんでした。",
    local_model_failed: "ローカルモデルで録音を文字起こしできませんでした。",
};
