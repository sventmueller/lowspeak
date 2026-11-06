use super::Texts;

pub static SV: Texts = Texts {
    recent_dictations: "Senaste dikteringar",
    none_yet: "Inga ännu",
    local: "Lokalt",
    not_installed: |model| format!("{model} (inte installerad)"),
    no_key: |provider| format!("{provider} (ingen nyckel)"),
    language: "Språk",
    auto_detect: "Identifiera automatiskt",
    post_processing: "Efterbearbetning",
    post_processing_off: "Av",
    post_processing_cleanup: "Rensa (fyllnadsord, grammatik)",
    post_processing_format: "Formatera (efter sammanhang)",
    vocabulary_menu: |terms| match terms {
        0 => "Ordlista…".to_owned(),
        1 => "Ordlista (1 term)…".to_owned(),
        terms => format!("Ordlista ({terms} termer)…"),
    },
    microphone: "Mikrofon",
    system_default: "Systemstandard",
    not_connected: |microphone| format!("{microphone} (inte ansluten)"),
    play_sound_when_ready: "Spela upp ljud när redo",
    manage_models: "Hantera modeller",
    delete_model: |model| format!("Radera {model}"),
    download_model: |model| format!("Hämta {model}"),
    set_api_key: "Ange API-nyckel…",
    update_api_key: "Ändra API-nyckel…",
    open_at_login: "Öppna vid inloggning",
    help: "Hjälp",
    quick_reference: "Snabbreferens",
    report_problem: "Rapportera ett problem…",
    suggest_feature: "Föreslå en funktion…",
    on_github: "Lowspeak på GitHub",
    about: "Om Lowspeak",
    license: "Licens: MIT eller Apache 2.0",
    quit: "Avsluta Lowspeak",

    reference_hold: "Håll ned för att diktera, släpp för att klistra in",
    reference_double_tap: "Tryck två gånger snabbt för att diktera handsfree",
    reference_shift: "Diktera handsfree",
    reference_stop: "Avsluta handsfree-diktering",
    reference_emoji: "Om 🌐-tangenten öppnar Emojier och symboler kan du stänga av det i tangentbordsinställningarna.",
    keyboard_settings: "Tangentbordsinställningar",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "Engelska",
    languages: |count| format!("{count} språk"),
    legacy: "äldre modell",
    fast: "snabb",

    set_api_key_title: "Ange API-nyckel",
    update_api_key_title: "Ändra API-nyckel",
    api_key_info: "Ange din API-nyckel. Leverantören identifieras automatiskt:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... eller gsk_...",
    save: "Spara",
    cancel: "Avbryt",
    ok: "OK",
    remove_all_keys: "Ta bort alla nycklar",
    unknown_key_format: "Okänt nyckelformat",
    unknown_key_format_info: "Nyckeln ska börja med sk- (OpenAI) eller gsk_ (Groq). Försök igen.",
    vocabulary_title: "Ordlista",
    vocabulary_info: "Namn och facktermer som ska stavas exakt, åtskilda med kommatecken. OpenAI, Groq, Whisper på den här Macen och efterbearbetningen använder dem.",

    recording_failed: "Inspelningen misslyckades",
    microphone_silent: "Mikrofonen är tyst",
    microphone_silent_info: "Ingen ljudsignal upptäcktes. Kontrollera om mikrofonen i dina AirPods är tillgänglig eller byt till den inbyggda mikrofonen.",
    accessibility_required: "Behörighet för hjälpmedel krävs",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak kan inte klistra in automatiskt. Ge behörighet i Systeminställningar → Integritet och säkerhet → {setting} och klistra sedan in manuellt med ⌘V."
        )
    },
    accessibility_list: "Hjälpmedel",
    device_control_list: "Enhetskontroll och dataåtkomst",
    transcription_failed: "Transkriberingen misslyckades",
    transcribed_locally: "Transkriberat på den här Macen",
    transcribed_locally_info: |reason| {
        format!("{reason} En lokal modell transkriberade i stället.")
    },
    post_processing_failed: "Efterbearbetningen misslyckades",
    post_processing_failed_info: |reason| {
        format!("{reason} Texten klistrades in utan efterbearbetning.")
    },
    no_api_key: "Ingen API-nyckel",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Ingen API-nyckel för {provider} har angetts. Lägg till en via ”{menu_item}” i menyn."
        )
    },
    login_item_error: "Fel med startobjekt",
    login_item_error_info: |menu_item| {
        format!(
            "Det gick inte att ändra ”{menu_item}”. Försök slå på eller av det i Systeminställningar → Allmänt → Startobjekt och tillägg."
        )
    },
    api_key_error: "Fel med API-nyckel",
    api_key_not_saved: "Det gick inte att spara API-nyckeln. Försök igen.",
    api_keys_not_removed: "Det gick inte att ta bort API-nycklarna. Försök igen.",
    download_failed: "Hämtningen misslyckades",
    download_failed_info: |model| {
        format!(
            "Det gick inte att hämta {model}. Kontrollera internetanslutningen och försök igen."
        )
    },
    cannot_delete: "Kan inte radera",
    cannot_delete_info: "Vänta tills den pågående åtgärden är klar innan du raderar en modell.",
    delete_failed: "Raderingen misslyckades",
    delete_failed_info: |model| format!("Det gick inte att radera {model}. Försök igen."),

    no_api_key_error: |provider| {
        format!("Ingen API-nyckel för {provider} har angetts. Ange den via menyraden.")
    },
    invalid_response: "Ogiltigt svar från servern.",
    authentication_failed: "API-autentiseringen misslyckades. Kontrollera din API-nyckel.",
    rate_limited: "Gränsen för antal förfrågningar har nåtts. Vänta en stund.",
    server_error: |status| format!("Serverfel ({status}). Försök igen senare."),
    api_error: |status| format!("API-fel ({status}). Se loggen för mer information."),
    model_not_available: |menu_item| {
        format!("Den lokala modellen är inte tillgänglig. Hämta den via ”{menu_item}”.")
    },
    network_error: |message| format!("Nätverksfel: {message}"),
    key_rejected: |provider| format!("{provider} avvisade API-nyckeln."),
    model_retired: |provider| {
        format!("{provider} erbjuder inte längre modellen för efterbearbetning.")
    },
    request_rejected: |provider, status| {
        format!("{provider} avvisade begäran om efterbearbetning ({status}).")
    },
    microphone_permission_denied: "Lowspeak har inte åtkomst till mikrofonen. Tillåt det i Systeminställningar → Integritet och säkerhet → Mikrofon.",
    no_microphone: "Ingen mikrofon är ansluten.",
    microphone_error: |detail| format!("Det gick inte att starta mikrofonen: {detail}"),
    paste_failed: "Inklistring misslyckades",
    paste_failed_info: "Texten finns i urklippet. Klistra in den med ⌘V.",
    request_timed_out: "Tidsgränsen för begäran överskreds.",
    audio_failed: "Det gick inte att bearbeta inspelningen.",
    local_model_failed: "Den lokala modellen kunde inte transkribera inspelningen.",
};
