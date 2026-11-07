use super::Texts;

pub static FR: Texts = Texts {
    recent_dictations: "Dictées récentes",
    none_yet: "Aucune pour l’instant",
    local: "Local",
    not_installed: |model| format!("{model} (non installé)"),
    no_key: |provider| format!("{provider} (aucune clé)"),
    language: "Langue",
    auto_detect: "Détection automatique",
    post_processing: "Post-traitement",
    post_processing_off: "Désactivé",
    post_processing_cleanup: "Nettoyage (mots parasites, grammaire)",
    post_processing_format: "Mise en forme (selon le contexte)",
    vocabulary_menu: |terms| match terms {
        0 => "Vocabulaire…".to_owned(),
        1 => "Vocabulaire (1 terme)…".to_owned(),
        terms => format!("Vocabulaire ({terms} termes)…"),
    },
    microphone: "Micro",
    system_default: "Réglage par défaut du système",
    not_connected: |microphone| format!("{microphone} (non connecté)"),
    play_sound_when_ready: "Signal sonore une fois prêt",
    manage_models: "Gérer les modèles",
    delete_model: |model| format!("Supprimer {model}"),
    download_model: |model| format!("Télécharger {model}"),
    set_api_key: "Définir la clé API…",
    update_api_key: "Modifier la clé API…",
    open_at_login: "Ouvrir avec la session",
    check_for_updates: "Rechercher les mises à jour…",
    check_updates_automatically: "Rechercher automatiquement les mises à jour",
    install_updates_automatically: "Télécharger et installer automatiquement les mises à jour",
    help: "Aide",
    quick_reference: "Aide-mémoire",
    report_problem: "Signaler un problème…",
    suggest_feature: "Suggérer une fonction…",
    on_github: "Lowspeak sur GitHub",
    about: "À propos de Lowspeak",
    license: "Licence : MIT ou Apache 2.0",
    quit: "Quitter Lowspeak",

    reference_hold: "Maintenir pour dicter, relâcher pour coller",
    reference_double_tap: "Appuyer deux fois rapidement pour dicter en mains libres",
    reference_shift: "Dicter en mains libres",
    reference_stop: "Terminer la dictée en mains libres",
    reference_emoji: "Si la touche 🌐 ouvre Emoji et symboles, désactivez cette action dans les réglages Clavier.",
    keyboard_settings: "Réglages Clavier",

    megabytes: "Mo",
    gigabytes: "Go",
    decimal_separator: ",",
    english: "anglais",
    languages: |count| match count {
        0 | 1 => format!("{count} langue"),
        _ => format!("{count} langues"),
    },
    legacy: "ancien modèle",
    fast: "rapide",

    set_api_key_title: "Définir la clé API",
    update_api_key_title: "Modifier la clé API",
    api_key_info: "Saisissez votre clé API. Le fournisseur est détecté automatiquement :\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... ou gsk_...",
    save: "Enregistrer",
    cancel: "Annuler",
    ok: "OK",
    remove_all_keys: "Supprimer toutes les clés",
    unknown_key_format: "Format de clé inconnu",
    unknown_key_format_info: "La clé doit commencer par sk- (OpenAI) ou gsk_ (Groq). Veuillez réessayer.",
    vocabulary_title: "Vocabulaire",
    vocabulary_info: "Noms et termes techniques à écrire exactement ainsi, séparés par des virgules. OpenAI, Groq, Whisper sur ce Mac et le post-traitement les utilisent.",

    recording_failed: "Échec de l’enregistrement",
    microphone_silent: "Micro silencieux",
    microphone_silent_info: "Aucun signal audio détecté. Vérifiez que le micro de vos AirPods est disponible ou passez au micro intégré.",
    accessibility_required: "Autorisation d’accessibilité requise",
    accessibility_required_info: |setting| {
        format!(
            "Lowspeak ne peut pas coller automatiquement. Accordez l’autorisation dans Réglages Système → Confidentialité et sécurité → {setting}, puis collez manuellement avec ⌘V."
        )
    },
    accessibility_list: "Accessibilité",
    device_control_list: "Contrôle de l’appareil et accès aux données",
    transcription_failed: "Échec de la transcription",
    transcribed_locally: "Transcrit sur ce Mac",
    transcribed_locally_info: |reason| format!("{reason} Un modèle local a pris le relais."),
    post_processing_failed: "Échec du post-traitement",
    post_processing_failed_info: |reason| {
        format!("{reason} Le texte a été collé sans post-traitement.")
    },
    no_api_key: "Aucune clé API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Aucune clé API n’est configurée pour {provider}. Ajoutez-en une via « {menu_item} » dans le menu."
        )
    },
    login_item_error: "Erreur d’ouverture avec la session",
    login_item_error_info: |menu_item| {
        format!(
            "Impossible de modifier « {menu_item} ». Essayez de l’activer ou de le désactiver dans Réglages Système → Général → Ouverture et extensions."
        )
    },
    api_key_error: "Erreur de clé API",
    api_key_not_saved: "Impossible d’enregistrer la clé API. Réessayez.",
    api_keys_not_removed: "Impossible de supprimer les clés API. Réessayez.",
    download_failed: "Échec du téléchargement",
    download_failed_info: |model| {
        format!(
            "Impossible de télécharger {model}. Vérifiez la connexion à Internet, puis réessayez."
        )
    },
    cannot_delete: "Suppression impossible",
    cannot_delete_info: "Attendez la fin de l’opération en cours avant de supprimer un modèle.",
    delete_failed: "Échec de la suppression",
    delete_failed_info: |model| format!("Impossible de supprimer {model}. Réessayez."),

    no_api_key_error: |provider| {
        format!(
            "Aucune clé API n’est configurée pour {provider}. Définissez-la depuis la barre des menus."
        )
    },
    invalid_response: "Réponse du serveur non valide.",
    authentication_failed: "Échec de l’authentification API. Vérifiez votre clé API.",
    rate_limited: "Limite de requêtes dépassée. Veuillez patienter un instant.",
    server_error: |status| format!("Erreur du serveur ({status}). Réessayez plus tard."),
    api_error: |status| {
        format!("Erreur API ({status}). Consultez le journal pour plus de détails.")
    },
    model_not_available: |menu_item| {
        format!("Modèle local non disponible. Téléchargez-le via « {menu_item} ».")
    },
    network_error: |message| format!("Erreur réseau : {message}"),
    key_rejected: |provider| format!("{provider} a refusé la clé API."),
    model_retired: |provider| format!("{provider} ne propose plus le modèle de post-traitement."),
    no_post_processing_key: |first, second| {
        format!(
            "Le post-traitement nécessite une clé API de {first} ou de {second}. Définissez-la depuis la barre des menus."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} ne propose plus {model}. Choisissez un autre modèle dans le menu.")
    },
    request_rejected: |provider, status| {
        format!("{provider} a refusé la requête de post-traitement ({status}).")
    },
    microphone_permission_denied: "Lowspeak n’a pas accès au micro. Autorisez-le dans Réglages Système → Confidentialité et sécurité → Micro.",
    no_microphone: "Aucun micro n’est connecté.",
    microphone_error: |detail| format!("Impossible de démarrer le micro : {detail}"),
    paste_failed: "Échec du collage",
    paste_failed_info: "Le texte se trouve dans le presse-papiers. Collez-le avec ⌘V.",
    request_timed_out: "La requête a expiré.",
    audio_failed: "L’enregistrement n’a pas pu être traité.",
    local_model_failed: "Le modèle local n’a pas pu transcrire l’enregistrement.",
};
