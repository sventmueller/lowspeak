use super::Texts;

pub static PT: Texts = Texts {
    recent_dictations: "Ditados Recentes",
    none_yet: "Nenhum Ainda",
    local: "Local",
    not_installed: |model| format!("{model} (não instalado)"),
    no_key: |provider| format!("{provider} (sem chave)"),
    language: "Idioma",
    auto_detect: "Detecção Automática",
    post_processing: "Pós-Processamento",
    post_processing_off: "Desativado",
    post_processing_cleanup: "Limpeza (vícios de linguagem, gramática)",
    post_processing_format: "Formatação (conforme o contexto)",
    vocabulary_menu: |terms| match terms {
        0 => "Vocabulário…".to_owned(),
        1 => "Vocabulário (1 termo)…".to_owned(),
        terms => format!("Vocabulário ({terms} termos)…"),
    },
    microphone: "Microfone",
    system_default: "Padrão do Sistema",
    not_connected: |microphone| format!("{microphone} (não conectado)"),
    play_sound_when_ready: "Tocar Som ao Ficar Pronto",
    manage_models: "Gerenciar Modelos",
    delete_model: |model| format!("Apagar {model}"),
    download_model: |model| format!("Baixar {model}"),
    set_api_key: "Definir Chave de API…",
    update_api_key: "Atualizar Chave de API…",
    open_at_login: "Abrir no Início da Sessão",
    check_for_updates: "Buscar Atualizações…",
    check_updates_automatically: "Buscar Atualizações Automaticamente",
    install_updates_automatically: "Baixar e Instalar Atualizações Automaticamente",
    help: "Ajuda",
    quick_reference: "Referência Rápida",
    report_problem: "Relatar um Problema…",
    suggest_feature: "Sugerir um Recurso…",
    on_github: "Lowspeak no GitHub",
    about: "Sobre o Lowspeak",
    license: "Licença: MIT ou Apache 2.0",
    quit: "Encerrar Lowspeak",

    reference_hold: "Mantenha pressionada para ditar, solte para colar",
    reference_double_tap: "Pressione duas vezes rapidamente para ditar sem as mãos",
    reference_shift: "Ditar sem as mãos",
    reference_stop: "Encerrar o ditado sem as mãos",
    reference_emoji: "Se a tecla 🌐 abrir Emoji e Símbolos, desative isso nos Ajustes de Teclado.",
    keyboard_settings: "Ajustes de Teclado",

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

    set_api_key_title: "Definir Chave de API",
    update_api_key_title: "Atualizar Chave de API",
    api_key_info: "Digite sua chave de API. O provedor é detectado automaticamente:\n• sk-... → OpenAI\n• gsk_... → Groq",
    api_key_placeholder: "sk-... ou gsk_...",
    save: "Salvar",
    cancel: "Cancelar",
    ok: "OK",
    remove_all_keys: "Remover Todas as Chaves",
    unknown_key_format: "Formato de Chave Desconhecido",
    unknown_key_format_info: "A chave deve começar com sk- (OpenAI) ou gsk_ (Groq). Tente novamente.",
    vocabulary_title: "Vocabulário",
    vocabulary_info: "Nomes e termos técnicos que devem ser escritos exatamente assim, separados por vírgulas. OpenAI, Groq, Whisper neste Mac e o pós-processamento os utilizam.",

    recording_failed: "Falha na Gravação",
    microphone_silent: "Microfone sem Sinal",
    microphone_silent_info: "Nenhum sinal de áudio detectado. Verifique se o microfone dos AirPods está disponível ou mude para o microfone embutido.",
    accessibility_required: "Permissão de Acessibilidade Necessária",
    accessibility_required_info: |setting| {
        format!(
            "O Lowspeak não pode colar automaticamente. Conceda a permissão em Ajustes do Sistema → Privacidade e Segurança → {setting} e cole manualmente com ⌘V."
        )
    },
    accessibility_list: "Acessibilidade",
    device_control_list: "Controle do Dispositivo e Acesso a Dados",
    transcription_failed: "Falha na Transcrição",
    transcribed_locally: "Transcrito neste Mac",
    transcribed_locally_info: |reason| format!("{reason} Um modelo local fez a transcrição."),
    post_processing_failed: "Falha no Pós-Processamento",
    post_processing_failed_info: |reason| {
        format!("{reason} O texto foi colado sem pós-processamento.")
    },
    no_api_key: "Nenhuma Chave de API",
    no_api_key_info: |provider, menu_item| {
        format!(
            "Nenhuma chave de API configurada para {provider}. Adicione uma em “{menu_item}” no menu."
        )
    },
    login_item_error: "Erro no Item de Início",
    login_item_error_info: |menu_item| {
        format!(
            "Não foi possível alterar “{menu_item}”. Tente ativá-lo ou desativá-lo em Ajustes do Sistema → Geral → Itens de Início de Sessão e Extensões."
        )
    },
    api_key_error: "Erro na Chave de API",
    api_key_not_saved: "Não foi possível salvar a chave de API. Tente novamente.",
    api_keys_not_removed: "Não foi possível remover as chaves de API. Tente novamente.",
    download_failed: "Falha no Download",
    download_failed_info: |model| {
        format!(
            "Não foi possível baixar {model}. Verifique a conexão com a internet e tente novamente."
        )
    },
    cannot_delete: "Não É Possível Apagar",
    cannot_delete_info: "Aguarde o término da operação atual antes de apagar um modelo.",
    delete_failed: "Falha ao Apagar",
    delete_failed_info: |model| format!("Não foi possível apagar {model}. Tente novamente."),

    no_api_key_error: |provider| {
        format!("Nenhuma chave de API configurada para {provider}. Defina-a pela barra de menus.")
    },
    invalid_response: "Resposta inválida do servidor.",
    authentication_failed: "Falha na autenticação da API. Verifique sua chave de API.",
    rate_limited: "Limite de solicitações excedido. Aguarde um momento.",
    server_error: |status| format!("Erro no servidor ({status}). Tente novamente mais tarde."),
    api_error: |status| {
        format!("Erro da API ({status}). Consulte o registro para ver os detalhes.")
    },
    model_not_available: |menu_item| {
        format!("Modelo local indisponível. Baixe-o em “{menu_item}”.")
    },
    network_error: |message| format!("Erro de rede: {message}"),
    key_rejected: |provider| format!("{provider} recusou a chave de API."),
    model_retired: |provider| format!("{provider} não oferece mais o modelo de pós-processamento."),
    no_post_processing_key: |first, second| {
        format!(
            "O pós-processamento precisa de uma chave de API do {first} ou do {second}. Defina-a pela barra de menus."
        )
    },
    online_model_retired: |provider, model| {
        format!("{provider} não oferece mais o {model}. Escolha outro modelo no menu.")
    },
    request_rejected: |provider, status| {
        format!("{provider} recusou a solicitação de pós-processamento ({status}).")
    },
    microphone_permission_denied: "O Lowspeak não tem acesso ao microfone. Permita em Ajustes do Sistema → Privacidade e Segurança → Microfone.",
    no_microphone: "Nenhum microfone conectado.",
    microphone_error: |detail| format!("Não foi possível iniciar o microfone: {detail}"),
    paste_failed: "Falha ao Colar",
    paste_failed_info: "O texto está na área de transferência. Cole com ⌘V.",
    request_timed_out: "A solicitação expirou.",
    audio_failed: "Não foi possível processar a gravação.",
    local_model_failed: "O modelo local não conseguiu transcrever a gravação.",
};
