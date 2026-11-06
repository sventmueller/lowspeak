<p><img src="../../readme-icon.png" alt="Ícone da aplicação Lowspeak" width="168" height="168"></p>

# Lowspeak

**Ditado para o Mac que escreve onde está o cursor: mantenha fn premida, fale, solte.**

O Lowspeak vive na barra de menus. Mantenha premida a tecla fn (Globo) e fale; quando a soltar, as suas palavras aparecem na aplicação em que estiver a escrever, como se as tivesse escrito à mão. Transcreve online com a OpenAI ou a Groq, ou inteiramente no seu Mac com o Whisper ou o NVIDIA Parakeet, que transforma 40 segundos de fala em texto em cerca de um terço de segundo. Quando a rede ou o serviço online falha, um modelo local instalado assume o trabalho, para que nenhum ditado se perca. Ensine-lhe os nomes e termos que utiliza, deixe um modelo de linguagem retirar os bordões de linguagem, se quiser, e bloqueie uma gravação com Shift para ditar textos mais longos em modo mãos-livres. Está escrito em Rust sobre controlos AppKit genuínos, fala 35 idiomas e envia a sua voz apenas ao fornecedor que escolheu.

[![Descarregar o Lowspeak para macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Descarregar Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: assinado e notarizado pela Apple, para macOS 14 ou posterior em Apple Silicon e Intel.

**Encontrou um erro?** [Abra uma issue](https://github.com/sventmueller/lowspeak/issues/new/choose) ou escolha **Ajuda › Comunicar problema…** no menu do Lowspeak: o formulário abre com a sua versão já preenchida. Se o Lowspeak lhe for útil, uma [estrela no GitHub](https://github.com/sventmueller/lowspeak) ajuda outras pessoas a encontrá-lo.

[![Rust, edição 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 ou posterior](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon e Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit através de objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp no Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet no Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online com a OpenAI ou a Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 idiomas da interface](https://img.shields.io/badge/languages-35-4C9A2A)
![Assinado com Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licença: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licença)

[Bahasa Indonesia](../id/README.md) ·
[Bahasa Melayu](../ms/README.md) ·
[Català](../ca/README.md) ·
[Čeština](../cs/README.md) ·
[Dansk](../da/README.md) ·
[Deutsch](../de/README.md) ·
[English](../../../README.md) ·
[Español](../es/README.md) ·
[Français](../fr/README.md) ·
[Hrvatski](../hr/README.md) ·
[Italiano](../it/README.md) ·
[Magyar](../hu/README.md) ·
[Nederlands](../nl/README.md) ·
[Norsk](../no/README.md) ·
[Polski](../pl/README.md) ·
[Português (Brasil)](../pt_BR/README.md) ·
**Português (Portugal)** ·
[Română](../ro/README.md) ·
[Slovenčina](../sk/README.md) ·
[Slovenščina](../sl/README.md) ·
[Suomi](../fi/README.md) ·
[Svenska](../sv/README.md) ·
[Tiếng Việt](../vi/README.md) ·
[Türkçe](../tr/README.md) ·
[Ελληνικά](../el/README.md) ·
[Русский](../ru/README.md) ·
[Українська](../uk/README.md) ·
[עברית](../he/README.md)‎ ·
[العربية](../ar/README.md)‎ ·
[हिन्दी](../hi/README.md) ·
[ไทย](../th/README.md) ·
[한국어](../ko/README.md) ·
[日本語](../ja/README.md) ·
[简体中文](../zh-Hans/README.md) ·
[繁體中文](../zh-Hant/README.md)

## Índice

- [Funcionalidades em resumo](#funcionalidades-em-resumo)
- [Requisitos](#requisitos)
- [Instalação](#instalação)
- [Primeiros passos](#primeiros-passos)
  - [Permissões](#permissões)
  - [Escolher como transcrever](#escolher-como-transcrever)
  - [O menu](#o-menu)
  - [Referência rápida](#referência-rápida)
- [Ditar](#ditar)
  - [Manter premida para falar](#manter-premida-para-falar)
  - [Bloquear com Shift ou premindo duas vezes](#bloquear-com-shift-ou-premindo-duas-vezes)
  - [Para onde vai o texto](#para-onde-vai-o-texto)
  - [Ditados recentes](#ditados-recentes)
- [Transcrição](#transcrição)
  - [Que modelo lhe convém](#que-modelo-lhe-convém)
  - [Online: OpenAI e Groq](#online-openai-e-groq)
  - [No seu Mac: Whisper e Parakeet](#no-seu-mac-whisper-e-parakeet)
  - [Quando o serviço online falha](#quando-o-serviço-online-falha)
  - [Como a gravação é preparada](#como-a-gravação-é-preparada)
  - [Idiomas por modelo](#idiomas-por-modelo)
  - [Macs com Intel](#macs-com-intel)
- [Vocabulário](#vocabulário)
- [Pós-processamento](#pós-processamento)
- [Microfone](#microfone)
- [Chaves de API](#chaves-de-api)
- [Notificações](#notificações)
- [Aspeto e acessibilidade](#aspeto-e-acessibilidade)
- [Idiomas](#idiomas)
- [Teclas e gestos](#teclas-e-gestos)
- [Privacidade e segurança](#privacidade-e-segurança)
- [Velocidade](#velocidade)
- [Limitações conhecidas](#limitações-conhecidas)
- [Resolução de problemas](#resolução-de-problemas)
- [Para programadores](#para-programadores)
  - [Componentes de terceiros](#componentes-de-terceiros)
- [Licença](#licença)

## Funcionalidades em resumo

- **Mantenha fn premida, fale, solte.** O texto é escrito na aplicação em primeiro plano, onde quer que esteja o cursor: correio, chat, editor, terminal ou navegador. Sem janela para onde mudar, sem botão para clicar.
- **Bloqueie para textos mais longos.** Prima Shift enquanto mantém fn premida, ou prima fn duas vezes, e a gravação continua sem nenhuma tecla premida, até cinco minutos. Um toque em fn ou em Shift termina-a.
- **O modelo certo para cada tarefa.** A Groq para rapidez sem ocupar a memória do seu Mac, o GPT Transcribe da OpenAI para o melhor resultado, o Whisper ou o NVIDIA Parakeet no seu Mac para trabalhar offline e manter a sua voz privada. Mude com um clique.
- **Nenhum ditado perdido.** Quando a rede ou o serviço online falha, um modelo local instalado transcreve a mesma gravação no seu Mac.
- **As suas palavras, bem escritas.** Um vocabulário de nomes e termos técnicos orienta a OpenAI, a Groq, o Whisper e o pós-processamento.
- **Texto limpo, se o quiser.** O pós-processamento opcional remove bordões de linguagem, corrige a gramática e, quando se corrige a si próprio, mantém apenas a correção, ou formata também o texto para o tipo de aplicação em que está a ditar.
- **Continue a falar.** Comece o ditado seguinte enquanto o anterior ainda está a ser transcrito; os textos chegam pela ordem em que os disse.
- **O seu clipboard continua a ser seu.** O Lowspeak cola através do clipboard e, pouco depois, repõe o que lá estava. Os dez últimos ditados ficam no menu para os voltar a copiar.
- **Nativo até ao último pormenor.** Um verdadeiro item da barra de menus com um menu AppKit, alertas e notificações padrão. Sem Electron, sem janela, sem ícone na Dock.
- **Privado.** Sem conta, sem telemetria. O áudio vai apenas para o fornecedor que escolheu, ou para lado nenhum no modo local.
- **No seu idioma.** O ditado funciona em dezenas de idiomas, e o menu, as caixas de diálogo e as notificações existem em 35.

## Requisitos

- macOS 14 Sonoma ou posterior. O Lowspeak é desenvolvido e testado no macOS 27.
- Um Mac com Apple Silicon ou com processador Intel. A transcrição local foi concebida para Apple Silicon, onde o Whisper corre na GPU e o Parakeet no Neural Engine; nos Macs com Intel, ambos correm no processador, muito mais devagar, consulte [Macs com Intel](#macs-com-intel).
- Cerca de 10 MB para a aplicação, mais 75 MB a 3 GB por cada modelo local que descarregar.
- Para a transcrição online, uma chave de API da [OpenAI](https://platform.openai.com/api-keys) ou da [Groq](https://console.groq.com/keys); ambas cobram pelo que transcrever. A transcrição local não precisa de conta.

## Instalação

**A partir da imagem de disco.** Descarregue [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), abra-a e arraste o Lowspeak para a pasta Aplicações. A imagem está assinada com um Developer ID e notarizada pela Apple, pelo que abre sem avisos.

**Atualizações.** O Lowspeak não se atualiza sozinho. As novas versões aparecem na [página de versões](https://github.com/sventmueller/lowspeak/releases); **Ajuda › Lowspeak no GitHub** leva até lá. Instale a versão nova por cima da antiga; as suas definições, chaves e modelos mantêm-se.

**A partir do código-fonte.** Consulte [Compilação e implementação](../../../docs/development.md#build--deploy) para os pré-requisitos e depois execute:

```sh
make deploy
```

Isto compila o Lowspeak, substitui a cópia na sua pasta Aplicações e abre-o.

**Abrir ao iniciar sessão** no menu inicia o Lowspeak com o Mac, através dos próprios elementos de início de sessão do macOS. Só corre uma cópia de cada vez; uma segunda termina de imediato.

## Primeiros passos

### Permissões

No primeiro arranque, o Lowspeak pede três permissões:

- **Acessibilidade** (Definições do Sistema › Privacidade e segurança › Acessibilidade; no macOS 27, a lista chama-se Controlo do dispositivo e acesso a dados). O Lowspeak precisa dela para detetar a tecla fn e para colar com ⌘V. O macOS propõe abrir a lista; ative o interruptor do Lowspeak. As mensagens do próprio Lowspeak dão à lista o nome que o seu macOS mostra.
- **Microfone.** Pedida uma única vez; sem ela, o Lowspeak não grava nada.
- **Notificações.** O Lowspeak usa-as para problemas que de outro modo não veria, como um microfone sem sinal ou uma transcrição falhada.

### Escolher como transcrever

Configure pelo menos uma forma; pode mudar em qualquer momento no menu.

- **Online:** escolha **Definir chave de API…** e cole uma chave da OpenAI (começa por `sk-`) ou da Groq (`gsk_`). O Lowspeak reconhece o fornecedor pela chave. Depois escolha um modelo em **OpenAI** ou **Groq**.
- **No seu Mac:** escolha **Gerir modelos** › **Descarregar Parakeet Ultra** ou um modelo Whisper e depois selecione-o em **Local**.

[Que modelo lhe convém](#que-modelo-lhe-convém) ajuda na escolha. Depois mantenha fn premida, diga qualquer coisa e solte.

### O menu

Clique no anel na barra de menus:

| Item | O que faz |
|---|---|
| **Ditados recentes** | os dez últimos textos; um clique volta a copiar um deles para o clipboard |
| **Local** | os modelos no seu Mac, agrupados em Whisper e Parakeet; os que não foram descarregados aparecem a cinzento |
| **OpenAI**, **Groq** | os modelos online; sem chave aparecem a cinzento e o menu oferece **Definir chave de API…** |
| **Idioma** | o idioma em que dita, ou **Detetar automaticamente**; o menu acompanha-o |
| **Pós-processamento** | **Desativado**, **Limpar** ou **Formatar**, consulte [Pós-processamento](#pós-processamento) |
| **Vocabulário…** | nomes e termos a escrever exatamente assim; o menu mostra quantos introduziu |
| **Microfone** | **Predefinição do sistema** ou um microfone fixo, e **Reproduzir som quando estiver pronto** |
| **Gerir modelos** | descarregar e apagar modelos locais, com indicação do progresso |
| **Definir chave de API…** / **Atualizar chave de API…** | adicionar, substituir ou remover chaves |
| **Abrir ao iniciar sessão** | iniciar o Lowspeak com o Mac; a marca de verificação indica que está ativo |
| **Ajuda** | **Referência rápida**, **Comunicar problema…**, **Sugerir funcionalidade…**, **Lowspeak no GitHub** |
| **Acerca do Lowspeak** | versão, copyright, página do projeto e licença |
| **Sair do Lowspeak** | ⌘Q com o menu aberto |

A marca de verificação em **Local**, **OpenAI** e **Groq** assinala o modelo em uso. O ícone mostra o que o Lowspeak está a fazer:

| Ícone | Significado |
|---|---|
| Anel com um ponto | Pronto |
| Círculo preenchido | A gravar enquanto fn está premida |
| Quadrado arredondado | Gravação bloqueada; o toque seguinte em fn ou Shift termina-a |
| Anel a piscar | A transcrever |
| ↓ 42% | A descarregar um modelo |

O Lowspeak não tem janela de definições: está tudo neste menu, que memoriza cada escolha.

### Referência rápida

**Ajuda › Referência rápida** mostra os gestos de fn numa pequena janela, no seu idioma, com um botão que abre as definições de teclado. **Comunicar problema…** abre um formulário no GitHub com as versões do Lowspeak e do macOS já preenchidas, **Sugerir funcionalidade…** uma discussão para ideias e **Lowspeak no GitHub** a página do projeto.

## Ditar

### Manter premida para falar

Mantenha fn premida, fale, solte. O ícone de gravação aparece no momento em que o microfone começa a captar som; comece a falar nessa altura, porque o que disser antes não é gravado. Soltar fn termina a gravação, e o texto surge pouco depois.

Um toque breve em fn não faz nada, por isso fn mantém as suas outras utilizações. Uma gravação que fique vazia ou dure menos de 0,3 segundos é descartada.

### Bloquear com Shift ou premindo duas vezes

Para textos mais longos, bloqueie a gravação para que continue sem nenhuma tecla premida. Há três formas:

- **Shift enquanto mantém fn premida.** Comece com fn como habitualmente e prima Shift durante a gravação: esta fica bloqueada e pode soltar fn. Nada do que disse antes se perde; a gravação simplesmente continua.
- **Shift e depois fn.** Mantenha Shift premida e prima fn: a gravação começa já bloqueada.
- **fn duas vezes.** Toque brevemente em fn (menos de 0,3 segundos) e prima-a novamente nos 0,4 segundos seguintes. A gravação conta a partir do primeiro toque.

O ícone passa a um quadrado arredondado. Prima **fn** ou **Shift** uma vez para parar; o texto surge como habitualmente. Se a sua tecla 🌐 estiver definida para abrir “Emoji e símbolos”, pare com Shift: um toque breve em fn abriria o visualizador.

Uma gravação para sozinha ao fim de dois minutos com fn premida e ao fim de cinco minutos quando bloqueada, e é depois transcrita como habitualmente.

### Para onde vai o texto

O Lowspeak cola o texto na aplicação que estava em primeiro plano quando parou de falar, na posição do cursor, através do clipboard e de ⌘V. Guarda tudo o que estava antes no clipboard e repõe-no meio segundo depois de a aplicação ler o texto, ou ao fim de dez segundos se nunca o ler, a menos que entretanto outra aplicação tenha alterado o clipboard. O texto é marcado como transitório, pelo que as aplicações de histórico do clipboard que respeitam essa marca não o registam.

Pode começar o ditado seguinte enquanto o anterior ainda está a ser transcrito. Os textos são colados pela ordem em que os disse, e nunca enquanto está a gravar ou a manter Shift premida, para que um texto colado não caia a meio da sua frase seguinte.

Sem a permissão de Acessibilidade, ou quando o ⌘V simulado não pode ser enviado, o texto fica no clipboard e uma notificação avisa; cole-o manualmente com ⌘V.

### Ditados recentes

**Ditados recentes** no menu lista os dez últimos textos, do mais recente para o mais antigo, cada um encurtado a uma linha; passar o cursor por cima mostra o texto completo. Um clique copia-o para o clipboard, para quando um texto foi parar ao sítio errado ou precisa dele duas vezes. A lista existe apenas na memória e fica vazia depois de o Lowspeak terminar.

## Transcrição

### Que modelo lhe convém

| Pretende | Escolha | Porquê |
|---|---|---|
| **Rapidez sem sobrecarregar o Mac** | **Groq › Whisper Large V3 Turbo** | a opção online mais rápida; nada é descarregado, e o Mac conserva a memória e o espaço em disco |
| **O melhor resultado** | **OpenAI › GPT Transcribe** (predefinido) | o mais preciso dos modelos que o Lowspeak oferece; recebe o seu vocabulário como palavras-chave |
| **Offline e privado** | **Local › Parakeet Ultra** para idiomas europeus, **Local › Whisper Large v3 Turbo** para todos os outros | funciona sem rede, e a sua voz nunca sai do Mac; o Parakeet é o mais rápido de todos |
| **Sem conta, sem custos** | qualquer modelo **Local** | corre no seu Mac; nada é cobrado |
| **Fiabilidade e rapidez ao mesmo tempo** | um modelo online, mais o Parakeet Ultra ou um modelo Whisper instalado | o modelo local entra em ação quando o serviço online falha |
| **Um Mac com Intel** | **Groq** ou **OpenAI** | aí os modelos locais correm no processador, consulte [Macs com Intel](#macs-com-intel) |

Os modelos locais ocupam no disco o tamanho da transferência e, enquanto estão selecionados, aproximadamente o mesmo em memória; o Lowspeak mantém o modelo carregado para que o ditado comece sem demora. Num Mac com 8 GB de memória, prefira o Parakeet Ultra ou um modelo online aos modelos Whisper grandes.

### Online: OpenAI e Groq

| Modelo | Fornecedor | Notas |
|---|---|---|
| **GPT Transcribe** (predefinido) | OpenAI | o mais preciso; recebe o seu vocabulário como palavras-chave |
| **Whisper** | OpenAI | o modelo Whisper original |
| **Whisper Large V3** | Groq | muito rápido |
| **Whisper Large V3 Turbo** | Groq | o mais rápido online |

A gravação é comprimida em AAC antes do envio, cerca de 6 KB por segundo de fala, para que o ditado funcione também em ligações lentas. Enquanto mantém fn premida, o Lowspeak já abre a ligação ao fornecedor, para que o envio comece no instante em que solta.

### No seu Mac: Whisper e Parakeet

| Modelo | Tamanho | Corre em | Idiomas |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 idiomas europeus |
| **Parakeet v2** | ~600 MB | Neural Engine | inglês |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | cerca de 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | cerca de 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | cerca de 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | cerca de 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | cerca de 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | cerca de 99 |

**Gerir modelos** descarrega-os e apaga-os; durante uma transferência, a barra de menus mostra o progresso, e sair do Lowspeak cancela-a. A primeiríssima utilização de um modelo prepara-o para o seu Mac: cerca de 10 segundos para o Parakeet Ultra, até 30 segundos para o Whisper; depois disso, carrega numa fração de segundo. Se o modelo que escolheu for apagado, o Lowspeak muda para outro que esteja instalado.

O **Parakeet** é a escolha mais rápida: transcreve 40 segundos de fala em cerca de 0,3 segundos. O Lowspeak executa-o com os mesmos modelos e a mesma descodificação do [FluidAudio](https://github.com/FluidInference/FluidAudio) e guarda-os na pasta do FluidAudio (`~/Library/Application Support/FluidAudio/Models`), para que as aplicações baseadas no FluidAudio os partilhem. As gravações com mais de 15 segundos são transcritas em janelas sobrepostas, unidas sem perder palavras nas junções. O **Whisper** compreende mais idiomas e lê o seu vocabulário; os modelos grandes são os mais precisos, os pequenos bastante mais fracos fora do inglês. Os modelos Whisper são guardados em `~/Library/Application Support/Lowspeak/Models`.

### Quando o serviço online falha

Se uma transcrição online falhar, porque a rede está em baixo, o fornecedor tem um problema ou a chave está errada, um modelo local instalado transcreve a mesma gravação: o escolhido em **Local**, caso contrário o Parakeet Ultra, depois o Whisper Large v3 Turbo, o Large v3 e os modelos Whisper mais pequenos, com o Parakeet v2 em último lugar. Uma notificação informa-o uma vez; só volta a aparecer depois de, entretanto, um ditado online ter sido bem-sucedido.

Com um modelo local de reserva, o Lowspeak dá 20 segundos ao pedido online; sem ele, tenta três vezes, até um minuto de cada vez. No modo online, o Lowspeak prepara o modelo Parakeet de reserva uma vez no arranque, para que a primeira substituição não tenha de esperar por ele.

### Como a gravação é preparada

O Lowspeak grava à taxa de amostragem própria do seu microfone e, quando para, converte para os 16 kHz que os modelos esperam. Antes da transcrição, remove o ruído grave abaixo de 80 Hz, corta o silêncio no início e no fim e uniformiza o volume, para que quem fala baixo e quem fala alto fique igual. Uma gravação sem fala nem sequer é enviada. No modo local, o Parakeet recebe primeiro a gravação intacta e só recebe a preparada se aquela não produzir texto.

### Idiomas por modelo

O idioma escolhido em **Idioma** é passado aos modelos online e ao Whisper. Com **Detetar automaticamente**, o modelo identifica o idioma por si.

| Modelo | Idiomas em que pode ditar |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, no seu Mac) | cerca de 99, entre eles todos os 35 do menu |
| Parakeet Ultra | alemão, búlgaro, checo, croata, dinamarquês, eslovaco, esloveno, espanhol, estónio, finlandês, francês, grego, húngaro, inglês, italiano, letão, lituano, maltês, neerlandês, polaco, português, romeno, russo, sueco, ucraniano |
| Parakeet v2 | inglês |

O Parakeet deteta o idioma por si e ignora a escolha no menu. **Chinês (tradicional)** faz com que o GPT Transcribe escreva em caracteres tradicionais; o Whisper escreve chinês em qualquer uma das duas escritas e português sem atender à variante.

### Macs com Intel

O Lowspeak é uma aplicação universal e também funciona em Macs com Intel com o macOS 14. Tudo o que é online funciona da mesma forma. Aí, porém, os modelos locais não dispõem do hardware para o qual foram concebidos: o Whisper corre no processador em vez da GPU, e o Parakeet no processador em vez do Neural Engine. Ambos funcionam, mas demoram muitas vezes mais do que em Apple Silicon, e os modelos Whisper grandes demasiado para ditar. Num Mac com Intel, transcreva online e mantenha o Whisper Small ou o Parakeet Ultra como [reserva](#quando-o-serviço-online-falha).

## Vocabulário

**Vocabulário…** recebe os nomes e termos que o Lowspeak deve escrever exatamente assim, separados por vírgulas: nomes de produtos, colegas, termos técnicos. Até 100 termos de até 60 caracteres cada; `<` e `>` são removidos.

- O **GPT Transcribe** recebe-os como palavras-chave.
- O **Whisper** (OpenAI, Groq e no seu Mac) lê-os como uma indicação antes da gravação, até 500 caracteres.
- O **pós-processamento** recebe a instrução de os escrever exatamente como estão escritos.
- O **Parakeet** não os utiliza.

## Pós-processamento

Depois da transcrição, um modelo de linguagem pode retrabalhar o texto. Escolha em **Pós-processamento**:

- **Desativado** (predefinido) — o texto tal como foi transcrito.
- **Limpar** — remove bordões de linguagem como “hum”, “pronto” ou “basicamente”, corrige a gramática e a pontuação e, quando se corrige a si próprio, mantém apenas a correção. Não acrescenta nada e não altera o sentido.
- **Formatar** — como Limpar, e adapta ainda o texto à aplicação em que está a ditar: termos em estilo de código nos editores de código, linhas de comandos nos terminais, um tom descontraído nas aplicações de chat, um tom profissional no correio eletrónico.

O pós-processamento usa a Groq (`openai/gpt-oss-120b`) se tiver uma chave da Groq, caso contrário a OpenAI (`gpt-4o-mini`), também quando transcreve localmente. Com um idioma escolhido em **Idioma**, mantém esse idioma. Se demorar mais de oito segundos, falhar ou devolver uma resposta cortada ou muito mais longa do que aquilo que disse, o Lowspeak cola o texto sem processamento. Se o fornecedor recusar o pedido ou não houver nenhuma chave definida, uma notificação avisa uma vez.

## Microfone

Em **Microfone**, escolha um microfone fixo ou **Predefinição do sistema**. O Lowspeak memoriza o dispositivo; enquanto não estiver ligado, o menu mostra-o como **(não ligado)** e o Lowspeak grava com a predefinição do sistema.

O microfone só é ativado enquanto dita. Prepará-lo demora cerca de 0,07 segundos com o microfone integrado, cerca de meio segundo com um Studio Display e um a dois segundos com AirPods, que primeiro passam para o modo de auricular. O ícone de gravação aparece assim que chega som; **Reproduzir som quando estiver pronto** acrescenta um som curto nesse momento. Está desativado por predefinição, porque em alguns altifalantes o próprio som chega com meio segundo de atraso.

Se um microfone fornecer apenas silêncio durante cinco segundos, como fazem os AirPods quando outro dispositivo ocupa o microfone deles, o Lowspeak avisa-o.

## Chaves de API

**Definir chave de API…** pede uma chave e reconhece o fornecedor pelo início: `sk-` para a OpenAI, `gsk_` para a Groq. Uma chave noutro formato é recusada com uma explicação. Introduza uma chave por fornecedor; com chaves de ambos, ficam disponíveis todos os modelos online. **Atualizar chave de API…** substitui uma chave ou, com **Remover todas as chaves**, apaga-as.

As chaves são guardadas no seu porta-chaves de início de sessão, sob `com.lowspeak.app`. O ficheiro das chaves de uma versão anterior (`~/Library/Application Support/Lowspeak/.api-key`) é movido para lá no próximo arranque e apagado assim que todas as chaves puderem ser lidas de novo a partir do porta-chaves.

## Notificações

O Lowspeak trabalha em segundo plano, por isso comunica os problemas através de notificações:

| Notificação | O que aconteceu |
|---|---|
| **Falha na gravação** | sem acesso ao microfone, nenhum microfone ligado, ou não foi possível iniciar o microfone |
| **Microfone sem sinal** | cinco segundos de silêncio, consulte [Microfone](#microfone) |
| **Permissão de acessibilidade necessária** | o texto está no clipboard; autorize o acesso em Acessibilidade para colar automaticamente |
| **Falha ao colar** | o texto está no clipboard; cole-o com ⌘V |
| **Falha na transcrição** | com o motivo: sem chave, chave recusada, limite de pedidos, erro do servidor ou de rede, ou nenhum modelo local |
| **Transcrito neste Mac** | o serviço online falhou e um modelo local assumiu o trabalho |
| **Falha no pós-processamento** | o fornecedor recusa o pós-processamento ou não há nenhuma chave definida; o texto foi colado sem ele |
| **Sem chave de API** | escolheu um modelo online sem chave para o respetivo fornecedor |
| **Falha ao descarregar**, **Falha ao apagar**, **Não é possível apagar** | a gestão de um modelo local não funcionou, ou outra operação ainda estava em curso |
| **Erro no elemento de início de sessão**, **Erro na chave de API** | o macOS ou o porta-chaves recusou a alteração |

As notificações nunca contêm o texto que ditou. Um serviço online com falhas e um fornecedor de pós-processamento que recusa os pedidos são comunicados uma vez, e não após cada ditado, até voltarem a funcionar.

## Aspeto e acessibilidade

- **O ícone da barra de menus** é uma imagem modelo que acompanha uma barra de menus clara, escura ou colorida.
- O **VoiceOver** lê o ícone como “Lowspeak”, e o menu, os alertas e as notificações são controlos padrão do macOS.
- **Nenhuma tecla tem de ficar premida.** Bloqueie uma gravação com Shift ou premindo fn duas vezes, e o toque seguinte termina-a.
- **A Referência rápida** é uma janela simples com etiquetas e um botão, acessível pelo teclado; Esc fecha-a.

As lacunas conhecidas estão listadas em [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): o VoiceOver não anuncia quando uma gravação começa, e o Lowspeak ainda não foi verificado com o VoiceOver nem com o Controlo por manípulos.

## Idiomas

O menu, as caixas de diálogo e as notificações estão traduzidos em 35 idiomas:

Alemão, árabe, catalão, checo, chinês (simplificado), chinês (tradicional), coreano, croata, dinamarquês, eslovaco, esloveno, espanhol, finlandês, francês, grego, hebraico, hindi, húngaro, indonésio, inglês, italiano, japonês, malaio, neerlandês, norueguês, polaco, português (Brasil), português (Portugal), romeno, russo, sueco, tailandês, turco, ucraniano e vietnamita.

O idioma que escolher em **Idioma** é também o idioma do menu. Com **Detetar automaticamente**, o menu segue a ordem de idiomas em **Definições do Sistema › Geral › Idioma e região**, e usa o inglês quando não existe tradução. Os termos dos menus seguem as traduções da própria Apple, para que o Lowspeak se leia como o resto do seu Mac. À exceção do inglês, as traduções são automáticas e ainda não foram revistas por falantes nativos; as correções são bem-vindas como [issues](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Teclas e gestos

| Ação | Teclas |
|---|---|
| Ditar enquanto a mantém premida | manter fn premida |
| Bloquear uma gravação em curso | Shift enquanto mantém fn premida |
| Iniciar uma gravação bloqueada | Shift+fn, ou fn duas vezes rapidamente |
| Terminar uma gravação bloqueada | fn ou Shift |
| Sair do Lowspeak (menu aberto) | ⌘Q |
| Fechar a Referência rápida | Esc |

O Lowspeak impede que os toques em fn que utiliza cheguem a outras aplicações; o Shift passa normalmente.

## Privacidade e segurança

O Lowspeak ouve o que diz enquanto fn está premida, pode escrever em qualquer aplicação e guarda as suas chaves de API, por isso faz o mínimo possível com tudo isto.

- **A sua voz vai para um único sítio.** No modo online, a gravação, comprimida, vai para o fornecedor que escolheu, OpenAI ou Groq, juntamente com o seu vocabulário. No modo local, nada sai do seu Mac. Quando um pedido online falha, um modelo local transcreve a mesma gravação no seu Mac.
- **O pós-processamento**, se o ativar, envia o texto transcrito e o seu vocabulário para a Groq ou a OpenAI. No modo Formatar, acrescenta o tipo de aplicação em que está a ditar (editor de código, terminal, chat ou correio eletrónico); o nome da aplicação não é enviado.
- **Sem telemetria, sem conta.** Além da transcrição e do pós-processamento, o Lowspeak só acede à internet para abrir a ligação ao seu fornecedor enquanto fn está premida, no modo online ou com o pós-processamento ativo, e para descarregar um modelo quando o pedir, a partir do Hugging Face por HTTPS, numa versão fixa e confirmando a soma de verificação de cada ficheiro. **Ajuda › Comunicar problema…** abre um formulário no GitHub no seu navegador; o endereço contém a versão do Lowspeak e a do macOS, nada do que ditou.
- **As chaves de API** ficam no seu porta-chaves de início de sessão. São enviadas apenas ao respetivo fornecedor e removidas das mensagens de erro antes de qualquer coisa ser escrita no registo.
- **O texto ditado não é guardado.** Os dez últimos ditados ficam na memória para **Ditados recentes** e desaparecem quando o Lowspeak termina. Os ficheiros de registo guardam comprimentos e tempos, nunca o texto.
- **O clipboard** é guardado antes de colar e reposto assim que a aplicação tiver lido o texto, a menos que outra aplicação o tenha alterado entretanto. O ditado é marcado como transitório, pelo que as aplicações de histórico do clipboard que respeitam essa marca o ignoram.
- **Assinatura de código** com o Hardened Runtime; a aplicação pede acesso apenas ao microfone, à Acessibilidade, às notificações e à rede.

Para comunicar uma vulnerabilidade, siga a [política de segurança](../../../SECURITY.md).

## Velocidade

Medido em Apple Silicon:

- O **Parakeet Ultra** transcreve 40 segundos de fala em cerca de 0,3 segundos.
- **Online:** o primeiro pedido após uma pausa demora cerca de 0,4 segundos na Groq, porque o Lowspeak abre a ligação enquanto ainda está a falar; os pedidos seguintes reutilizam-na, por HTTP/3 quando o fornecedor o disponibiliza.
- **Tamanho do envio:** cerca de 6 KB por segundo de fala, em AAC a 48 kbit/s.
- **Preparar o microfone:** cerca de 0,07 segundos no integrado, meio segundo num Studio Display, um a dois segundos com AirPods.

## Limitações conhecidas

- **Os modelos locais em Macs com Intel** correm no processador e são muito mais lentos; aí, a transcrição online é a melhor escolha.
- **O que disser antes de o microfone estar pronto perde-se.** Aguarde pelo ícone de gravação, sobretudo com AirPods.
- **A tecla fn pode ser tomada por outros.** Quando a definição da tecla 🌐 do macOS, o Warp ou o Ditado do macOS se apropriam primeiro de fn, o Lowspeak não a vê; consulte [Resolução de problemas](#resolução-de-problemas).
- O **Parakeet** ignora o vocabulário e a escolha de idioma, e só conhece idiomas europeus.
- **Escrita e variante:** só ao GPT Transcribe se pode pedir que escreva em chinês tradicional; a nenhum modelo se pode pedir que escreva em português europeu em vez de português do Brasil.
- **Limites de tempo:** dois minutos com fn premida, cinco minutos quando bloqueada.
- **Modo do microfone:** a Central de controlo oferece Isolamento de voz e Espectro amplo para o Lowspeak e indica-os como ativos, mas o Lowspeak grava sem processamento de voz, pelo que a escolha não tem efeito.
- Os **Ditados recentes** existem apenas na memória e desaparecem depois de sair.
- **Por opção de conceção:** sem janela de definições, sem ícone na Dock, sem atualizações automáticas, fora da App Store.

## Resolução de problemas

**Não acontece nada quando mantenho fn premida.** Verifique três coisas: o Lowspeak está autorizado em **Definições do Sistema › Privacidade e segurança › Acessibilidade** (no macOS 27: **Controlo do dispositivo e acesso a dados**); **Definições do Sistema › Teclado › Premir tecla 🌐 para** está definido como **Não fazer nada**; e nenhuma outra aplicação se apropria de fn. O Warp fá-lo por predefinição; desative-o com `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` e reinicie o Warp. O Ditado do macOS também pode estar definido para começar com fn.

**Depois de uma atualização, o Lowspeak deixou de reagir a fn.** O macOS associa a permissão de Acessibilidade à assinatura da aplicação. Se a assinatura mudou, por exemplo de uma compilação própria para a aplicação descarregada, remova o Lowspeak da lista de Acessibilidade (no macOS 27: Controlo do dispositivo e acesso a dados) e adicione-o novamente.

**Faltam as primeiras palavras.** Comece a falar quando o ícone de gravação aparecer. Com AirPods, o microfone precisa de um a dois segundos; um microfone integrado ou USB escolhido em **Microfone** fica pronto quase de imediato.

**“Microfone sem sinal.”** O microfone não forneceu som durante cinco segundos. Com AirPods, outro dispositivo pode estar a usar o microfone deles; mude para o microfone integrado ou volte a ligá-los. A tampa fechada de um MacBook também silencia o microfone integrado.

**Os ditados saem vazios.** O volume de entrada pode estar demasiado baixo: aplicações como Teams, Zoom ou Meet alteram-no durante as chamadas, e o macOS guarda-o por dispositivo. Aumente-o em **Definições do Sistema › Som › Entrada**.

**O texto foi parar ao sítio errado.** **Ditados recentes** no menu volta a copiá-lo para o clipboard.

**“Sem chave de API.”** Escolha **Definir chave de API…** no menu ou mude para um modelo local.

**A primeira transcrição local demora muito.** Cada modelo é preparado para o seu Mac na primeira utilização; a partir daí, carrega numa fração de segundo.

**O Lowspeak termina logo após o arranque.** Já está em execução outra cópia do Lowspeak; termine essa primeiro.

**Não aparece nenhuma notificação.** Autorize-as em **Definições do Sistema › Notificações › Lowspeak**.

**Outra coisa.** Os registos em `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) descrevem o que aconteceu, sem o texto que ditou. Anexe as linhas relevantes a uma [comunicação de problema](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Para programadores

A compilação, os testes, a estrutura do código e as regras que segue estão descritos em [docs/development.md](../../../docs/development.md); as contribuições, em [CONTRIBUTING.md](../../../CONTRIBUTING.md) (ambos em inglês).

### Componentes de terceiros

| Componente | Versão | Utilização | Licença |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) através de [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | Whisper local na GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | descodificação do Parakeet e junção de gravações longas, traduzidas para Rust em `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) e os seus crates de frameworks | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation e outras frameworks do sistema | MIT; Zlib, Apache-2.0 ou MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | captura do microfone através do CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | reamostragem para 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) com rustls | 3.5 | transferência de modelos | MIT ou Apache-2.0; rustls e ring sob ISC e Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | somas de verificação das transferências de modelos | MIT ou Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | chaves de API no porta-chaves | MIT ou Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, erros, o vocabulário do Parakeet | MIT ou Apache-2.0; Apache-2.0 |

Os textos das licenças de tudo o que está dentro da aplicação seguem com ela em `Contents/Resources/Acknowledgements.txt`.

Os modelos não fazem parte do Lowspeak; este descarrega-os quando o pedir:

| Modelo | Origem | Licença |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (o Whisper da OpenAI em formato ggml) | MIT |
| Parakeet v2, Parakeet Ultra | conversões para CoreML, feitas pela [FluidInference](https://huggingface.co/FluidInference), do Parakeet TDT da NVIDIA | CC-BY-4.0 |

## Licença

O Lowspeak está licenciado ao abrigo de uma das seguintes licenças

- Licença Apache, Versão 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licença MIT ([LICENSE-MIT](../../../LICENSE-MIT))

à sua escolha.

Salvo indicação explícita em contrário, qualquer contribuição intencionalmente submetida por si para inclusão no Lowspeak, tal como definida na licença Apache-2.0, será licenciada em licença dupla como acima, sem quaisquer termos ou condições adicionais.
