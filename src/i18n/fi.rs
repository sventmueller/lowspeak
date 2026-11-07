use super::Texts;

pub static FI: Texts = Texts {
    recent_dictations: "Viimeaikaiset sanelut",
    none_yet: "Ei vielä yhtään",
    local: "Paikallinen",
    not_installed: |model| format!("{model} (ei asennettu)"),
    no_key: |provider| format!("{provider} (ei avainta)"),
    language: "Kieli",
    auto_detect: "Tunnista automaattisesti",
    post_processing: "Jälkikäsittely",
    post_processing_off: "Pois",
    post_processing_cleanup: "Siisti (täytesanat, kielioppi)",
    post_processing_format: "Muotoile (asiayhteyden mukaan)",
    vocabulary_menu: |terms| match terms {
        0 => "Sanasto…".to_owned(),
        1 => "Sanasto (1 termi)…".to_owned(),
        terms => format!("Sanasto ({terms} termiä)…"),
    },
    microphone: "Mikrofoni",
    system_default: "Järjestelmän oletus",
    not_connected: |microphone| format!("{microphone} (ei yhdistetty)"),
    play_sound_when_ready: "Toista ääni, kun valmis",
    manage_models: "Hallitse malleja",
    delete_model: |model| format!("Poista {model}"),
    download_model: |model| format!("Lataa {model}"),
    set_api_key: "Aseta API-avain…",
    update_api_key: "Vaihda API-avain…",
    open_at_login: "Avaa sisäänkirjauduttaessa",
    check_for_updates: "Etsi päivityksiä…",
    check_updates_automatically: "Etsi päivityksiä automaattisesti",
    install_updates_automatically: "Lataa ja asenna päivitykset automaattisesti",
    help: "Ohje",
    quick_reference: "Pikaopas",
    report_problem: "Ilmoita ongelmasta…",
    suggest_feature: "Ehdota ominaisuutta…",
    on_github: "Lowspeak GitHubissa",
    about: "Tietoja: Lowspeak",
    license: "Lisenssi: MIT tai Apache 2.0",
    quit: "Lopeta Lowspeak",

    reference_hold: "Sanele pitämällä painettuna, liitä vapauttamalla",
    reference_double_tap: "Paina kahdesti nopeasti sanellaksesi handsfree-tilassa",
    reference_shift: "Sanele handsfree-tilassa",
    reference_stop: "Lopeta handsfree-sanelu",
    reference_emoji: "Jos 🌐-näppäin avaa Emojit ja symbolit, poista se käytöstä näppäimistöasetuksissa.",
    keyboard_settings: "Näppäimistöasetukset",

    megabytes: "Mt",
    gigabytes: "Gt",
    decimal_separator: ",",
    english: "englanti",
    languages: |count| match count {
        1 => "1 kieli".to_owned(),
        count => format!("{count} kieltä"),
    },
    legacy: "vanhempi malli",
    fast: "nopea",

    set_api_key_title: "Aseta API-avain",
    update_api_key_title: "Vaihda API-avain",
    api_key_info: "Syötä API-avaimesi. Palveluntarjoaja tunnistetaan automaattisesti:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... tai gsk_...",
    save: "Tallenna",
    cancel: "Kumoa",
    ok: "OK",
    remove_all_keys: "Poista kaikki avaimet",
    unknown_key_format: "Tuntematon avainmuoto",
    unknown_key_format_info: "Avaimen pitää alkaa merkeillä sk- (OpenAI) tai gsk_ (Groq). Yritä uudelleen.",
    vocabulary_title: "Sanasto",
    vocabulary_info: "Nimet ja erikoistermit, jotka kirjoitetaan täsmälleen oikein, pilkuilla erotettuina. Niitä käyttävät OpenAI, Groq, Whisper tässä Macissa ja jälkikäsittely.",

    recording_failed: "Äänitys epäonnistui",
    microphone_silent: "Mikrofonista ei kuulu ääntä",
    microphone_silent_info: "Äänisignaalia ei havaittu. Tarkista, onko AirPodsien mikrofoni käytettävissä, tai vaihda sisäiseen mikrofoniin.",
    accessibility_required: "Käyttöapuoikeus vaaditaan",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak ei voi sijoittaa tekstiä automaattisesti. Anna lupa kohdassa Järjestelmäasetukset → Tietosuoja ja suojaus → {setting} ja sijoita teksti sitten itse painamalla ⌘V."
        )
    },
    accessibility_list: "Käyttöapu",
    device_control_list: "Laitehallinta ja tietojen käyttö",
    transcription_failed: "Litterointi epäonnistui",
    transcribed_locally: "Litteroitu tässä Macissa",
    transcribed_locally_info: |reason| {
        format!("{reason} Litteroinnin teki sen sijaan paikallinen malli.")
    },
    post_processing_failed: "Jälkikäsittely epäonnistui",
    post_processing_failed_info: |reason| {
        format!("{reason} Teksti sijoitettiin ilman jälkikäsittelyä.")
    },
    no_api_key: "Ei API-avainta",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Palvelulle {provider} ei ole asetettu API-avainta. Lisää se valikon kohdasta ”{menu_item}”."
        )
    },
    login_item_error: "Kirjautumiskohteen virhe",
    login_item_error_info: |menu_item| {
        format!(
            "Kohdetta ”{menu_item}” ei voitu muuttaa. Kokeile kytkeä se päälle tai pois kohdassa Järjestelmäasetukset → Yleiset → Kirjautumiskohteet ja laajennukset."
        )
    },
    api_key_error: "API-avaimen virhe",
    api_key_not_saved: "API-avaimen tallentaminen epäonnistui. Yritä uudelleen.",
    api_keys_not_removed: "API-avainten poistaminen epäonnistui. Yritä uudelleen.",
    download_failed: "Lataus epäonnistui",
    download_failed_info: |model| {
        format!("Mallin {model} lataus epäonnistui. Tarkista internetyhteys ja yritä uudelleen.")
    },
    cannot_delete: "Poistaminen ei onnistu",
    cannot_delete_info: "Odota, että käynnissä oleva toiminto päättyy, ennen kuin poistat mallin.",
    delete_failed: "Poistaminen epäonnistui",
    delete_failed_info: |model| format!("Mallin {model} poistaminen epäonnistui. Yritä uudelleen."),

    no_api_key_error: |provider| {
        format!("Palvelulle {provider} ei ole asetettu API-avainta. Aseta se valikkorivin kautta.")
    },
    invalid_response: "Palvelimen vastaus oli virheellinen.",
    authentication_failed: "API-todennus epäonnistui. Tarkista API-avaimesi.",
    rate_limited: "Pyyntöjen enimmäismäärä ylittyi. Odota hetki.",
    server_error: |status| format!("Palvelinvirhe ({status}). Yritä myöhemmin uudelleen."),
    api_error: |status| format!("API-virhe ({status}). Katso lisätiedot lokista."),
    model_not_available: |menu_item| {
        format!("Paikallinen malli ei ole käytettävissä. Lataa se kohdasta ”{menu_item}”.")
    },
    network_error: |message| format!("Verkkovirhe: {message}"),
    key_rejected: |provider| format!("{provider} hylkäsi API-avaimen."),
    model_retired: |provider| format!("{provider} ei enää tarjoa jälkikäsittelymallia."),
    no_post_processing_key: |first, second| {
        format!(
            "Jälkikäsittely tarvitsee API-avaimen palvelulta {first} tai {second}. Aseta se valikkorivin kautta."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} ei enää tarjoa mallia {model}. Valitse valikosta toinen malli.")
    },
    request_rejected: |provider, status| {
        format!("{provider} hylkäsi jälkikäsittelypyynnön ({status}).")
    },
    microphone_permission_denied: "Lowspeakilla ei ole pääsyä mikrofoniin. Salli se kohdassa Järjestelmäasetukset → Tietosuoja ja suojaus → Mikrofoni.",
    no_microphone: "Mikrofonia ei ole liitetty.",
    microphone_error: |detail| format!("Mikrofonin käynnistäminen epäonnistui: {detail}"),
    paste_failed: "Sijoittaminen epäonnistui",
    paste_failed_info: "Teksti on leikepöydällä. Sijoita se painamalla ⌘V.",
    request_timed_out: "Pyyntö aikakatkaistiin.",
    audio_failed: "Äänitystä ei voitu käsitellä.",
    local_model_failed: "Paikallinen malli ei pystynyt litteroimaan äänitystä.",
};
