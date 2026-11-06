use super::Texts;

pub static PT_PT: Texts = Texts {
    recent_dictations: "Ditados recentes",
    none_yet: "Ainda nenhum",
    local: "Local",
    not_installed: |model| format!("{model} (não instalado)"),
    no_key: |provider| format!("{provider} (sem chave)"),
    language: "Idioma",
    auto_detect: "Detetar automaticamente",
    post_processing: "Pós-processamento",
    post_processing_off: "Desativado",
    post_processing_cleanup: "Limpar (bordões de linguagem, gramática)",
    post_processing_format: "Formatar (consoante o contexto)",
    vocabulary_menu: |terms| match terms {
        0 => "Vocabulário…".to_owned(),
        1 => "Vocabulário (1 termo)…".to_owned(),
        terms => format!("Vocabulário ({terms} termos)…"),
    },
    microphone: "Microfone",
    system_default: "Predefinição do sistema",
    not_connected: |microphone| format!("{microphone} (não ligado)"),
    play_sound_when_ready: "Reproduzir som quando estiver pronto",
    manage_models: "Gerir modelos",
    delete_model: |model| format!("Apagar {model}"),
    download_model: |model| format!("Descarregar {model}"),
    set_api_key: "Definir chave de API…",
    update_api_key: "Atualizar chave de API…",
    open_at_login: "Abrir ao iniciar sessão",
    help: "Ajuda",
    quick_reference: "Referência rápida",
    report_problem: "Comunicar problema…",
    suggest_feature: "Sugerir funcionalidade…",
    on_github: "Lowspeak no GitHub",
    about: "Acerca do Lowspeak",
    license: "Licença: MIT ou Apache 2.0",
    quit: "Sair do Lowspeak",

    reference_hold: "Manter premida para ditar, soltar para colar",
    reference_double_tap: "Premir duas vezes rapidamente para ditar em modo mãos-livres",
    reference_shift: "Ditar em modo mãos-livres",
    reference_stop: "Terminar o ditado em modo mãos-livres",
    reference_emoji: "Se a tecla 🌐 abrir “Emoji e símbolos”, defina-a como “Não fazer nada” nas definições de Teclado.",
    keyboard_settings: "Definições de Teclado",

    megabytes: "MB",
    gigabytes: "GB",
    decimal_separator: ",",
    english: "inglês",
    languages: |count| match count {
        1 => "1 idioma".to_owned(),
        count => format!("{count} idiomas"),
    },
    legacy: "modelo antigo",
    fast: "rápido",

    set_api_key_title: "Definir chave de API",
    update_api_key_title: "Atualizar chave de API",
    api_key_info: "Digite a chave de API. O fornecedor é detetado automaticamente:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... ou gsk_...",
    save: "Guardar",
    cancel: "Cancelar",
    ok: "OK",
    remove_all_keys: "Remover todas as chaves",
    unknown_key_format: "Formato de chave desconhecido",
    unknown_key_format_info: "A chave deve começar por sk- (OpenAI) ou gsk_ (Groq). Volte a tentar.",
    vocabulary_title: "Vocabulário",
    vocabulary_info: "Nomes e termos técnicos a escrever exatamente assim, separados por vírgulas. OpenAI, Groq, Whisper neste Mac e o pós-processamento utilizam-nos.",

    recording_failed: "Falha na gravação",
    microphone_silent: "Microfone sem sinal",
    microphone_silent_info: "Não foi detetado nenhum sinal de áudio. Verifique se o microfone dos AirPods está disponível ou mude para o microfone integrado.",
    accessibility_required: "Permissão de acessibilidade necessária",
    accessibility_required_info: |setting| {
        format!(
            "O Lowspeak não consegue colar automaticamente. Conceda a permissão em Definições do Sistema → Privacidade e segurança → {setting} e, depois, cole manualmente com ⌘V."
        )
    },
    accessibility_list: "Acessibilidade",
    device_control_list: "Controlo do dispositivo e acesso a dados",
    transcription_failed: "Falha na transcrição",
    transcribed_locally: "Transcrito neste Mac",
    transcribed_locally_info: |reason| {
        format!("{reason} Em vez disso, a transcrição foi feita por um modelo local.")
    },
    post_processing_failed: "Falha no pós-processamento",
    post_processing_failed_info: |reason| {
        format!("{reason} O texto foi colado sem pós-processamento.")
    },
    no_api_key: "Sem chave de API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Não há nenhuma chave de API configurada para {provider}. Adicione uma através de “{menu_item}” no menu."
        )
    },
    login_item_error: "Erro no elemento de início de sessão",
    login_item_error_info: |menu_item| {
        format!(
            "Não foi possível atualizar “{menu_item}”. Experimente ativar ou desativar esta opção em Definições do Sistema → Geral → Início de sessão e extensões."
        )
    },
    api_key_error: "Erro na chave de API",
    api_key_not_saved: "Não foi possível guardar a chave de API. Volte a tentar.",
    api_keys_not_removed: "Não foi possível remover as chaves de API. Volte a tentar.",
    download_failed: "Falha ao descarregar",
    download_failed_info: |model| {
        format!(
            "Não foi possível descarregar {model}. Verifique a ligação à Internet e volte a tentar."
        )
    },
    cannot_delete: "Não é possível apagar",
    cannot_delete_info: "Aguarde até a operação atual terminar antes de apagar um modelo.",
    delete_failed: "Falha ao apagar",
    delete_failed_info: |model| format!("Não foi possível apagar {model}. Volte a tentar."),

    no_api_key_error: |provider| {
        format!(
            "Não há nenhuma chave de API configurada para {provider}. Defina-a na barra de menus."
        )
    },
    invalid_response: "Resposta inválida do servidor.",
    authentication_failed: "Falha na autenticação da API. Verifique a chave de API.",
    rate_limited: "Limite de pedidos excedido. Aguarde um momento.",
    server_error: |status| format!("Erro do servidor ({status}). Volte a tentar mais tarde."),
    api_error: |status| {
        format!("Erro da API ({status}). Consulte o registo para obter mais detalhes.")
    },
    model_not_available: |menu_item| {
        format!("Modelo local indisponível. Descarregue-o através de “{menu_item}”.")
    },
    network_error: |message| format!("Erro de rede: {message}"),
    key_rejected: |provider| format!("{provider} recusou a chave de API."),
    model_retired: |provider| {
        format!("{provider} já não disponibiliza o modelo de pós-processamento.")
    },
    request_rejected: |provider, status| {
        format!("{provider} recusou o pedido de pós-processamento ({status}).")
    },
    microphone_permission_denied: "O Lowspeak não tem acesso ao microfone. Permita o acesso em Definições do Sistema → Privacidade e segurança → Microfone.",
    no_microphone: "Não está ligado nenhum microfone.",
    microphone_error: |detail| format!("Não foi possível iniciar o microfone: {detail}"),
    paste_failed: "Falha ao colar",
    paste_failed_info: "O texto está no clipboard. Cole-o com ⌘V.",
    request_timed_out: "O pedido excedeu o tempo limite.",
    audio_failed: "Não foi possível processar a gravação.",
    local_model_failed: "O modelo local não conseguiu transcrever a gravação.",
};
