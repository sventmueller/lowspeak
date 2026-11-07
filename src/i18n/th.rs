use super::Texts;

pub static TH: Texts = Texts {
    recent_dictations: "การป้อนตามคำบอกล่าสุด",
    none_yet: "ยังไม่มี",
    local: "ในเครื่อง",
    not_installed: |model| format!("{model} (ไม่ได้ติดตั้ง)"),
    no_key: |provider| format!("{provider} (ไม่มีคีย์)"),
    language: "ภาษา",
    auto_detect: "ตรวจหาอัตโนมัติ",
    post_processing: "การประมวลผลภายหลัง",
    post_processing_off: "ปิด",
    post_processing_cleanup: "ขัดเกลา (คำติดปาก, ไวยากรณ์)",
    post_processing_format: "จัดรูปแบบ (ตามบริบท)",
    vocabulary_menu: |terms| match terms {
        0 => "คำศัพท์…".to_owned(),
        terms => format!("คำศัพท์ ({terms} คำ)…"),
    },
    microphone: "ไมโครโฟน",
    system_default: "ค่าเริ่มต้นของระบบ",
    not_connected: |microphone| format!("{microphone} (ไม่ได้เชื่อมต่อ)"),
    play_sound_when_ready: "เล่นเสียงเมื่อพร้อม",
    manage_models: "จัดการโมเดล",
    delete_model: |model| format!("ลบ {model}"),
    download_model: |model| format!("ดาวน์โหลด {model}"),
    set_api_key: "ตั้งค่าคีย์ API…",
    update_api_key: "อัปเดตคีย์ API…",
    open_at_login: "เปิดเมื่อเข้าสู่ระบบ",
    check_for_updates: "ตรวจหารายการอัปเดต…",
    check_updates_automatically: "ตรวจหารายการอัปเดตโดยอัตโนมัติ",
    install_updates_automatically: "ดาวน์โหลดและติดตั้งรายการอัปเดตโดยอัตโนมัติ",
    help: "วิธีใช้",
    quick_reference: "ข้อมูลอ้างอิงด่วน",
    report_problem: "รายงานปัญหา…",
    suggest_feature: "เสนอคุณสมบัติ…",
    on_github: "Lowspeak บน GitHub",
    about: "เกี่ยวกับ Lowspeak",
    license: "สัญญาอนุญาต: MIT หรือ Apache 2.0",
    quit: "ออกจาก Lowspeak",

    reference_hold: "กดค้างไว้เพื่อป้อนตามคำบอก ปล่อยเพื่อวาง",
    reference_double_tap: "กดสองครั้งอย่างรวดเร็วเพื่อป้อนตามคำบอกแบบแฮนด์ฟรี",
    reference_shift: "ป้อนตามคำบอกแบบแฮนด์ฟรี",
    reference_stop: "สิ้นสุดการป้อนตามคำบอกแบบแฮนด์ฟรี",
    reference_emoji: "หากปุ่ม 🌐 เปิดอิโมจิและสัญลักษณ์ ให้ตั้งค่าเป็น “ไม่ดำเนินการใดๆ” ในการตั้งค่าแป้นพิมพ์",
    keyboard_settings: "การตั้งค่าแป้นพิมพ์",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ".",
    english: "ภาษาอังกฤษ",
    languages: |count| format!("{count} ภาษา"),
    legacy: "รุ่นเก่า",
    fast: "เร็ว",

    set_api_key_title: "ตั้งค่าคีย์ API",
    update_api_key_title: "อัปเดตคีย์ API",
    api_key_info: "ป้อนคีย์ API ของคุณ ระบบจะตรวจหาผู้ให้บริการโดยอัตโนมัติ:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... หรือ gsk_...",
    save: "บันทึก",
    cancel: "ยกเลิก",
    ok: "ตกลง",
    remove_all_keys: "เอาคีย์ทั้งหมดออก",
    unknown_key_format: "รูปแบบคีย์ที่ไม่รู้จัก",
    unknown_key_format_info: "คีย์ต้องขึ้นต้นด้วย sk- (OpenAI) หรือ gsk_ (Groq) โปรดลองอีกครั้ง",
    vocabulary_title: "คำศัพท์",
    vocabulary_info: "ชื่อและศัพท์เฉพาะที่ต้องสะกดให้ตรงทุกตัวอักษร คั่นด้วยเครื่องหมายจุลภาค OpenAI, Groq, Whisper บน Mac เครื่องนี้ และการประมวลผลภายหลังจะใช้คำเหล่านี้",

    recording_failed: "บันทึกเสียงไม่สำเร็จ",
    microphone_silent: "ไมโครโฟนไม่มีเสียง",
    microphone_silent_info: "ไม่พบสัญญาณเสียง โปรดตรวจสอบว่าไมโครโฟนของ AirPods ใช้งานได้หรือไม่ หรือสลับไปใช้ไมโครโฟนในตัว",
    accessibility_required: "ต้องได้รับอนุญาตการช่วยการเข้าถึง",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak ไม่สามารถวางโดยอัตโนมัติได้ ให้อนุญาตในการตั้งค่าระบบ → ความเป็นส่วนตัวและความปลอดภัย → {setting} จากนั้นวางด้วยตนเองโดยใช้ ⌘V"
        )
    },
    accessibility_list: "การช่วยการเข้าถึง",
    device_control_list: "การควบคุมอุปกรณ์และการเข้าถึงข้อมูล",
    transcription_failed: "ถอดเสียงไม่สำเร็จ",
    transcribed_locally: "ถอดเสียงบน Mac เครื่องนี้แล้ว",
    transcribed_locally_info: |reason| format!("{reason} โมเดลในเครื่องถอดเสียงแทนแล้ว"),
    post_processing_failed: "ประมวลผลภายหลังไม่สำเร็จ",
    post_processing_failed_info: |reason| format!("{reason} ข้อความถูกวางโดยไม่ผ่านการประมวลผลภายหลัง"),
    no_api_key: "ไม่มีคีย์ API",
    no_api_key_info: |provider, menu_item| {
        format!("ยังไม่ได้ตั้งค่าคีย์ API ของ {provider} เพิ่มคีย์ได้ผ่าน “{menu_item}” ในเมนู")
    },
    login_item_error: "ข้อผิดพลาดของรายการเข้าสู่ระบบ",
    login_item_error_info: |menu_item| {
        format!(
            "ไม่สามารถอัปเดต “{menu_item}” ได้ ลองเปิดหรือปิดในการตั้งค่าระบบ → ทั่วไป → รายการเข้าสู่ระบบและส่วนขยาย"
        )
    },
    api_key_error: "ข้อผิดพลาดของคีย์ API",
    api_key_not_saved: "ไม่สามารถบันทึกคีย์ API ได้ โปรดลองอีกครั้ง",
    api_keys_not_removed: "ไม่สามารถเอาคีย์ API ออกได้ โปรดลองอีกครั้ง",
    download_failed: "ดาวน์โหลดไม่สำเร็จ",
    download_failed_info: |model| {
        format!("ไม่สามารถดาวน์โหลด {model} ได้ โปรดตรวจสอบการเชื่อมต่ออินเทอร์เน็ตแล้วลองอีกครั้ง")
    },
    cannot_delete: "ไม่สามารถลบได้",
    cannot_delete_info: "โปรดรอให้การดำเนินการปัจจุบันเสร็จสิ้นก่อนลบโมเดล",
    delete_failed: "ลบไม่สำเร็จ",
    delete_failed_info: |model| format!("ไม่สามารถลบ {model} ได้ โปรดลองอีกครั้ง"),

    no_api_key_error: |provider| format!("ยังไม่ได้ตั้งค่าคีย์ API ของ {provider} ตั้งค่าได้จากแถบเมนู"),
    invalid_response: "การตอบกลับจากเซิร์ฟเวอร์ไม่ถูกต้อง",
    authentication_failed: "การตรวจสอบสิทธิ์ API ล้มเหลว โปรดตรวจสอบคีย์ API ของคุณ",
    rate_limited: "เกินขีดจำกัดจำนวนคำขอ โปรดรอสักครู่",
    server_error: |status| format!("ข้อผิดพลาดของเซิร์ฟเวอร์ ({status}) โปรดลองอีกครั้งในภายหลัง"),
    api_error: |status| format!("ข้อผิดพลาดของ API ({status}) ดูรายละเอียดได้ในบันทึก"),
    model_not_available: |menu_item| format!("ไม่มีโมเดลในเครื่องที่ใช้งานได้ ดาวน์โหลดได้ผ่าน “{menu_item}”"),
    network_error: |message| format!("ข้อผิดพลาดของเครือข่าย: {message}"),
    key_rejected: |provider| format!("{provider} ปฏิเสธคีย์ API"),
    model_retired: |provider| format!("{provider} เลิกให้บริการโมเดลสำหรับการประมวลผลภายหลังแล้ว"),
    no_post_processing_key: |first, second| {
        format!("การประมวลผลภายหลังต้องใช้คีย์ API ของ {first} หรือ {second} ตั้งค่าได้จากแถบเมนู")
    },
    online_model_retired: |provider, model| {
        format!("{provider} เลิกให้บริการ {model} แล้ว เลือกโมเดลอื่นในเมนู")
    },
    request_rejected: |provider, status| {
        format!("{provider} ปฏิเสธคำขอการประมวลผลภายหลัง ({status})")
    },
    microphone_permission_denied: "Lowspeak ไม่มีสิทธิ์เข้าถึงไมโครโฟน ให้อนุญาตในการตั้งค่าระบบ → ความเป็นส่วนตัวและความปลอดภัย → ไมโครโฟน",
    no_microphone: "ไม่มีไมโครโฟนเชื่อมต่ออยู่",
    microphone_error: |detail| format!("ไม่สามารถเริ่มไมโครโฟนได้: {detail}"),
    paste_failed: "วางไม่สำเร็จ",
    paste_failed_info: "ข้อความอยู่ในคลิปบอร์ดแล้ว วางได้ด้วย ⌘V",
    request_timed_out: "คำขอหมดเวลา",
    audio_failed: "ไม่สามารถประมวลผลการบันทึกเสียงได้",
    local_model_failed: "โมเดลในเครื่องไม่สามารถถอดเสียงจากการบันทึกได้",
};
