use super::Texts;

pub static EL: Texts = Texts {
    recent_dictations: "Πρόσφατες υπαγορεύσεις",
    none_yet: "Καμία ακόμη",
    local: "Τοπικά",
    not_installed: |model| format!("{model} (μη εγκατεστημένο)"),
    no_key: |provider| format!("{provider} (χωρίς κλειδί)"),
    language: "Γλώσσα",
    auto_detect: "Αυτόματη ανίχνευση",
    post_processing: "Μετεπεξεργασία",
    post_processing_off: "Ανενεργή",
    post_processing_cleanup: "Καθαρισμός (λέξεις-γεμίσματα, γραμματική)",
    post_processing_format: "Μορφοποίηση (ανάλογα με το πλαίσιο)",
    vocabulary_menu: |terms| match terms {
        0 => "Λεξιλόγιο…".to_owned(),
        1 => "Λεξιλόγιο (1 όρος)…".to_owned(),
        terms => format!("Λεξιλόγιο ({terms} όροι)…"),
    },
    microphone: "Μικρόφωνο",
    system_default: "Προεπιλογή συστήματος",
    not_connected: |microphone| format!("{microphone} (μη συνδεδεμένο)"),
    play_sound_when_ready: "Αναπαραγωγή ήχου όταν είναι έτοιμο",
    manage_models: "Διαχείριση μοντέλων",
    delete_model: |model| format!("Διαγραφή {model}"),
    download_model: |model| format!("Λήψη {model}"),
    set_api_key: "Ορισμός κλειδιού API…",
    update_api_key: "Ενημέρωση κλειδιού API…",
    open_at_login: "Άνοιγμα κατά την είσοδο",
    check_for_updates: "Έλεγχος για ενημερώσεις…",
    check_updates_automatically: "Αυτόματος έλεγχος για ενημερώσεις",
    install_updates_automatically: "Αυτόματη λήψη και εγκατάσταση ενημερώσεων",
    help: "Βοήθεια",
    quick_reference: "Σύντομος οδηγός",
    report_problem: "Αναφορά προβλήματος…",
    suggest_feature: "Πρόταση λειτουργίας…",
    on_github: "Το Lowspeak στο GitHub",
    about: "Πληροφορίες για το Lowspeak",
    license: "Άδεια: MIT ή Apache 2.0",
    quit: "Τερματισμός Lowspeak",

    reference_hold: "Κρατήστε πατημένο για υπαγόρευση, αφήστε για επικόλληση",
    reference_double_tap: "Πατήστε δύο φορές γρήγορα για υπαγόρευση χωρίς χέρια",
    reference_shift: "Υπαγόρευση χωρίς χέρια",
    reference_stop: "Τέλος υπαγόρευσης χωρίς χέρια",
    reference_emoji: "Αν το πλήκτρο 🌐 ανοίγει τα Emoji και σύμβολα, απενεργοποιήστε το στις ρυθμίσεις πληκτρολογίου.",
    keyboard_settings: "Ρυθμίσεις πληκτρολογίου",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "Αγγλικά",
    languages: |count| match count {
        1 => "1 γλώσσα".to_owned(),
        count => format!("{count} γλώσσες"),
    },
    legacy: "παλαιότερο",
    fast: "γρήγορο",

    set_api_key_title: "Ορισμός κλειδιού API",
    update_api_key_title: "Ενημέρωση κλειδιού API",
    api_key_info: "Εισαγάγετε το κλειδί API. Ο πάροχος αναγνωρίζεται αυτόματα:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... ή gsk_...",
    save: "Αποθήκευση",
    cancel: "Ακύρωση",
    ok: "OK",
    remove_all_keys: "Αφαίρεση όλων των κλειδιών",
    unknown_key_format: "Άγνωστη μορφή κλειδιού",
    unknown_key_format_info: "Αναμενόταν κλειδί που ξεκινά με sk- (OpenAI) ή gsk_ (Groq). Δοκιμάστε ξανά.",
    vocabulary_title: "Λεξιλόγιο",
    vocabulary_info: "Ονόματα και τεχνικοί όροι που πρέπει να γράφονται ακριβώς έτσι, χωρισμένοι με κόμματα. Χρησιμοποιούνται από το OpenAI, το Groq, το Whisper σε αυτό το Mac και τη μετεπεξεργασία.",

    recording_failed: "Η εγγραφή απέτυχε",
    microphone_silent: "Κανένας ήχος από το μικρόφωνο",
    microphone_silent_info: "Δεν ανιχνεύθηκε σήμα ήχου. Ελέγξτε αν το μικρόφωνο των AirPods είναι διαθέσιμο ή μεταβείτε στο ενσωματωμένο μικρόφωνο.",
    accessibility_required: "Απαιτείται άδεια προσβασιμότητας",
    accessibility_required_info: |setting| {
        format!(
            "Το Lowspeak δεν μπορεί να κάνει αυτόματη επικόλληση. Δώστε άδεια στις Ρυθμίσεις συστήματος → Απόρρητο και ασφάλεια → {setting} και μετά επικολλήστε χειροκίνητα με ⌘V."
        )
    },
    accessibility_list: "Προσβασιμότητα",
    device_control_list: "Έλεγχος συσκευής και πρόσβαση σε δεδομένα",
    transcription_failed: "Η μεταγραφή απέτυχε",
    transcribed_locally: "Μεταγράφηκε σε αυτό το Mac",
    transcribed_locally_info: |reason| {
        format!("{reason} Αντ' αυτού, τη μεταγραφή έκανε ένα τοπικό μοντέλο.")
    },
    post_processing_failed: "Η μετεπεξεργασία απέτυχε",
    post_processing_failed_info: |reason| {
        format!("{reason} Το κείμενο επικολλήθηκε χωρίς μετεπεξεργασία.")
    },
    no_api_key: "Δεν υπάρχει κλειδί API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Δεν έχει οριστεί κλειδί API για το {provider}. Προσθέστε ένα μέσω της επιλογής «{menu_item}» στο μενού."
        )
    },
    login_item_error: "Σφάλμα στοιχείου εισόδου",
    login_item_error_info: |menu_item| {
        format!(
            "Δεν ήταν δυνατή η ενημέρωση της επιλογής «{menu_item}». Δοκιμάστε να την αλλάξετε στις Ρυθμίσεις συστήματος → Γενικά → Στοιχεία εισόδου και επεκτάσεις."
        )
    },
    api_key_error: "Σφάλμα κλειδιού API",
    api_key_not_saved: "Δεν ήταν δυνατή η αποθήκευση του κλειδιού API. Δοκιμάστε ξανά.",
    api_keys_not_removed: "Δεν ήταν δυνατή η αφαίρεση των κλειδιών API. Δοκιμάστε ξανά.",
    download_failed: "Η λήψη απέτυχε",
    download_failed_info: |model| {
        format!(
            "Δεν ήταν δυνατή η λήψη του {model}. Ελέγξτε τη σύνδεση στο διαδίκτυο και δοκιμάστε ξανά."
        )
    },
    cannot_delete: "Δεν είναι δυνατή η διαγραφή",
    cannot_delete_info: "Περιμένετε να ολοκληρωθεί η τρέχουσα εργασία πριν διαγράψετε ένα μοντέλο.",
    delete_failed: "Η διαγραφή απέτυχε",
    delete_failed_info: |model| format!("Δεν ήταν δυνατή η διαγραφή του {model}. Δοκιμάστε ξανά."),

    no_api_key_error: |provider| {
        format!("Δεν έχει οριστεί κλειδί API για το {provider}. Ορίστε το από τη γραμμή μενού.")
    },
    invalid_response: "Μη έγκυρη απόκριση από τον διακομιστή.",
    authentication_failed: "Ο έλεγχος ταυτότητας API απέτυχε. Ελέγξτε το κλειδί API.",
    rate_limited: "Υπέρβαση ορίου αιτημάτων. Περιμένετε λίγο.",
    server_error: |status| format!("Σφάλμα διακομιστή ({status}). Δοκιμάστε ξανά αργότερα."),
    api_error: |status| {
        format!("Σφάλμα API ({status}). Δείτε το αρχείο καταγραφής για λεπτομέρειες.")
    },
    model_not_available: |menu_item| {
        format!("Το τοπικό μοντέλο δεν είναι διαθέσιμο. Κάντε λήψη από το μενού «{menu_item}».")
    },
    network_error: |message| format!("Σφάλμα δικτύου: {message}"),
    key_rejected: |provider| format!("Το {provider} απέρριψε το κλειδί API."),
    model_retired: |provider| {
        format!("Το {provider} δεν προσφέρει πλέον το μοντέλο μετεπεξεργασίας.")
    },
    no_post_processing_key: |first, second| {
        format!(
            "Η μετεπεξεργασία χρειάζεται κλειδί API από το {first} ή το {second}. Ορίστε το από τη γραμμή μενού."
        )
    },
    online_model_retired: |provider, model| {
        format!("Το {provider} δεν προσφέρει πλέον το {model}. Επιλέξτε άλλο μοντέλο από το μενού.")
    },
    request_rejected: |provider, status| {
        format!("Το {provider} απέρριψε το αίτημα μετεπεξεργασίας ({status}).")
    },
    microphone_permission_denied: "Το Lowspeak δεν έχει πρόσβαση στο μικρόφωνο. Δώστε άδεια στις Ρυθμίσεις συστήματος → Απόρρητο και ασφάλεια → Μικρόφωνο.",
    no_microphone: "Δεν είναι συνδεδεμένο μικρόφωνο.",
    microphone_error: |detail| format!("Δεν ήταν δυνατή η εκκίνηση του μικροφώνου: {detail}"),
    paste_failed: "Η επικόλληση απέτυχε",
    paste_failed_info: "Το κείμενο βρίσκεται στο πρόχειρο. Επικολλήστε το με ⌘V.",
    request_timed_out: "Το αίτημα έληξε.",
    audio_failed: "Δεν ήταν δυνατή η επεξεργασία της εγγραφής.",
    local_model_failed: "Το τοπικό μοντέλο δεν μπόρεσε να μεταγράψει την εγγραφή.",
};
