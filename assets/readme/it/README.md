<p><img src="../../readme-icon.png" alt="Icona dell’app Lowspeak" width="168" height="168"></p>

# Lowspeak

**La dettatura per Mac che scrive dove si trova il cursore: tieni premuto fn, parla, rilascia.**

Lowspeak vive nella barra dei menu. Tieni premuto il tasto fn (Globo) e parla; quando lo rilasci, le tue parole compaiono nell’app in cui stai scrivendo, come se le avessi digitate tu. Trascrive online con OpenAI o Groq, oppure interamente sul tuo Mac con Whisper o NVIDIA Parakeet, che trasforma 40 secondi di parlato in testo in circa un terzo di secondo. Se la rete o il servizio online non funzionano, subentra un modello locale installato, così nessuna dettatura va persa. Insegnagli i nomi e i termini che usi, lascia che un modello linguistico tolga gli intercalari, se vuoi, e blocca una registrazione con Shift per dettare testi più lunghi a mani libere. È scritto in Rust su veri controlli AppKit, parla 35 lingue e invia la tua voce solo al fornitore che hai scelto.

[![Scarica Lowspeak per macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Scarica Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: firmato e notarizzato da Apple, per macOS 14 o successivo su Apple Silicon e Intel.

**Hai trovato un bug?** [Apri una issue](https://github.com/sventmueller/lowspeak/issues/new/choose) oppure scegli **Aiuto › Segnala un problema…** nel menu di Lowspeak: il modulo si apre con la tua versione già compilata. Se Lowspeak ti è utile, una [stella su GitHub](https://github.com/sventmueller/lowspeak) aiuta altri a trovarlo.

[![Rust edizione 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 o successivo](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universale: Apple Silicon e Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit tramite objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp su Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet sul Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online con OpenAI o Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 lingue dell’interfaccia](https://img.shields.io/badge/languages-35-4C9A2A)
![Firmato con Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licenza: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licenza)

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
**Italiano** ·
[Magyar](../hu/README.md) ·
[Nederlands](../nl/README.md) ·
[Norsk](../no/README.md) ·
[Polski](../pl/README.md) ·
[Português (Brasil)](../pt_BR/README.md) ·
[Português (Portugal)](../pt_PT/README.md) ·
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

## Indice

- [Funzioni in breve](#funzioni-in-breve)
- [Requisiti](#requisiti)
- [Installazione](#installazione)
- [Per iniziare](#per-iniziare)
  - [Autorizzazioni](#autorizzazioni)
  - [Scegliere come trascrivere](#scegliere-come-trascrivere)
  - [Il menu](#il-menu)
  - [Riferimento rapido](#riferimento-rapido)
- [Dettare](#dettare)
  - [Tenere premuto per parlare](#tenere-premuto-per-parlare)
  - [Bloccare con Shift o con una doppia pressione](#bloccare-con-shift-o-con-una-doppia-pressione)
  - [Dove finisce il testo](#dove-finisce-il-testo)
  - [Dettature recenti](#dettature-recenti)
- [Trascrizione](#trascrizione)
  - [Quale modello fa per te](#quale-modello-fa-per-te)
  - [Online: OpenAI e Groq](#online-openai-e-groq)
  - [Sul tuo Mac: Whisper e Parakeet](#sul-tuo-mac-whisper-e-parakeet)
  - [Quando il servizio online non funziona](#quando-il-servizio-online-non-funziona)
  - [Come viene preparata la registrazione](#come-viene-preparata-la-registrazione)
  - [Lingue per modello](#lingue-per-modello)
  - [Mac con Intel](#mac-con-intel)
- [Vocabolario](#vocabolario)
- [Post-elaborazione](#post-elaborazione)
- [Microfono](#microfono)
- [Chiavi API](#chiavi-api)
- [Notifiche](#notifiche)
- [Aspetto e accessibilità](#aspetto-e-accessibilità)
- [Lingue](#lingue)
- [Tasti e gesti](#tasti-e-gesti)
- [Privacy e sicurezza](#privacy-e-sicurezza)
- [Velocità](#velocità)
- [Limiti noti](#limiti-noti)
- [Risoluzione dei problemi](#risoluzione-dei-problemi)
- [Per sviluppatori](#per-sviluppatori)
  - [Componenti di terze parti](#componenti-di-terze-parti)
- [Licenza](#licenza)

## Funzioni in breve

- **Tieni premuto fn, parla, rilascia.** Il testo viene scritto nell’app in primo piano, ovunque si trovi il cursore: posta, chat, editor, terminale o browser. Nessuna finestra in cui passare, nessun pulsante da cliccare.
- **Bloccala per i testi più lunghi.** Premi Shift mentre tieni premuto fn, oppure premi fn due volte, e la registrazione continua senza tenere premuto alcun tasto, fino a cinque minuti. Una pressione su fn o Shift la termina.
- **Il modello giusto per ogni esigenza.** Groq per la velocità senza occupare la memoria del tuo Mac, GPT Transcribe di OpenAI per il risultato migliore, Whisper o NVIDIA Parakeet sul tuo Mac per lavorare offline e tenere privata la tua voce. Si cambia con un clic.
- **Nessuna dettatura persa.** Se la rete o il servizio online non funzionano, un modello locale installato trascrive la stessa registrazione sul tuo Mac.
- **Le tue parole, scritte giuste.** Un vocabolario di nomi e termini tecnici guida OpenAI, Groq, Whisper e la post-elaborazione.
- **Testo pulito, se lo vuoi.** La post-elaborazione facoltativa rimuove gli intercalari, corregge la grammatica e, quando ti correggi da solo, tiene solo la correzione, oppure formatta anche il testo in base al tipo di app in cui detti.
- **Continua a parlare.** Avvia la dettatura successiva mentre la precedente è ancora in trascrizione; i testi arrivano nell’ordine in cui li hai pronunciati.
- **Gli appunti restano tuoi.** Lowspeak incolla tramite gli appunti e un attimo dopo rimette al suo posto ciò che contenevano. Le ultime dieci dettature restano nel menu per copiarle di nuovo.
- **Nativo fino in fondo.** Un vero elemento della barra dei menu con un menu AppKit, avvisi e notifiche standard. Niente Electron, nessuna finestra, nessuna icona nel Dock.
- **Riservato.** Nessun account, nessuna telemetria. L’audio va solo al fornitore che hai scelto, o da nessuna parte in modalità locale.
- **Nella tua lingua.** La dettatura funziona in decine di lingue, e menu, finestre di dialogo e notifiche sono disponibili in 35.

## Requisiti

- macOS 14 Sonoma o successivo. Lowspeak è sviluppato e testato su macOS 27.
- Un Mac con Apple Silicon o con processore Intel. La trascrizione locale è pensata per Apple Silicon, dove Whisper gira sulla GPU e Parakeet sul Neural Engine; sui Mac con Intel entrambi girano sul processore, molto più lentamente, vedi [Mac con Intel](#mac-con-intel).
- Circa 17 MB per l’app, più da 75 MB a 3 GB per ogni modello locale che scarichi.
- Per la trascrizione online, una chiave API di [OpenAI](https://platform.openai.com/api-keys) o di [Groq](https://console.groq.com/keys); entrambi fanno pagare ciò che trascrivi. La trascrizione locale non richiede alcun account.

## Installazione

**Dall’immagine disco.** Scarica [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), aprila e trascina Lowspeak nella cartella Applicazioni. L’immagine è firmata con un Developer ID e notarizzata da Apple, quindi si apre senza avvisi.

**Con Homebrew.** Il cask si trova in un tap a parte:

```sh
brew install --cask sventmueller/tap/lowspeak
```

`brew uninstall --zap --cask lowspeak` rimuove anche le impostazioni, i modelli, i file di registro e le cache di Lowspeak; le chiavi API restano nel tuo portachiavi di login.

**Aggiornamenti.** Lowspeak, installato dall’immagine disco o con Homebrew, si mantiene aggiornato da solo con [Sparkle](https://sparkle-project.org). Al secondo avvio chiede se può cercare aggiornamenti per conto suo, una volta al giorno. **Verifica aggiornamenti…** nel menu li cerca subito; **Verifica automaticamente gli aggiornamenti** e **Scarica e installa automaticamente gli aggiornamenti**, appena sotto, cambiano la tua scelta in seguito. Se una verifica trova una nuova versione mentre lavori in un’altra app, sull’anello nella barra dei menu compare un punto e **Verifica aggiornamenti…** porta l’aggiornamento in primo piano. Lowspeak installa un aggiornamento solo se porta la firma dello sviluppatore; impostazioni, chiavi e modelli restano. `brew upgrade --cask lowspeak` aggiorna anche una copia installata con Homebrew. La versione 1.0.0 non ha l’aggiornamento automatico: installa una volta sopra di essa una versione più recente, dalla [pagina delle release](https://github.com/sventmueller/lowspeak/releases). Nemmeno una build compilata da te ha l’aggiornamento automatico.

**Dal codice sorgente.** Per i prerequisiti vedi [Compilazione e distribuzione](../../../docs/development.md#build--deploy), poi:

```sh
make deploy
```

Questo compila Lowspeak, sostituisce la copia nella tua cartella Applicazioni e lo avvia.

**Apri al login** nel menu avvia Lowspeak insieme al Mac, tramite gli elementi login di macOS stesso. Gira una sola copia alla volta; una seconda si chiude subito.

## Per iniziare

### Autorizzazioni

Al primo avvio Lowspeak chiede tre autorizzazioni:

- **Accessibilità** (Impostazioni di Sistema › Privacy e sicurezza › Accessibilità; su macOS 27 l’elenco si chiama Controllo del dispositivo e accesso ai dati). Lowspeak ne ha bisogno per accorgersi del tasto fn e per incollare con ⌘V. macOS propone di aprire l’elenco; attiva l’interruttore accanto a Lowspeak. I messaggi di Lowspeak chiamano l’elenco come lo mostra il tuo macOS.
- **Microfono.** Viene chiesto una sola volta; senza, Lowspeak non registra nulla.
- **Notifiche.** Lowspeak le usa per i problemi che altrimenti non vedresti, come un microfono silenzioso o una trascrizione non riuscita.

### Scegliere come trascrivere

Configura almeno un modo; puoi cambiare in qualsiasi momento dal menu.

- **Online:** scegli **Imposta chiave API…** e incolla una chiave di OpenAI (inizia con `sk-`) o di Groq (`gsk_`). Lowspeak riconosce il fornitore dalla chiave. Poi scegli un modello sotto **OpenAI** o **Groq**.
- **Sul tuo Mac:** scegli **Gestisci modelli** › **Scarica Parakeet Ultra** o un modello Whisper, poi selezionalo sotto **Locale**.

[Quale modello fa per te](#quale-modello-fa-per-te) aiuta nella scelta. Poi tieni premuto fn, di’ qualcosa e rilascia.

### Il menu

Fai clic sull’anello nella barra dei menu:

| Voce | Che cosa fa |
|---|---|
| **Dettature recenti** | gli ultimi dieci testi; un clic ne copia di nuovo uno negli appunti |
| **Locale** | i modelli sul tuo Mac, raggruppati in Whisper e Parakeet; quelli non scaricati sono in grigio |
| **OpenAI**, **Groq** | i modelli online; senza una chiave sono in grigio e il menu offre **Imposta chiave API…** |
| **Lingua** | la lingua in cui detti, oppure **Rilevamento automatico**; il menu la segue |
| **Post-elaborazione** | **Disattivata**, **Pulizia** o **Formattazione**, vedi [Post-elaborazione](#post-elaborazione) |
| **Vocabolario…** | nomi e termini da scrivere esattamente; il menu mostra quanti ne hai inseriti |
| **Microfono** | **Default di sistema** o un microfono fisso, e **Segnale acustico quando è pronto** |
| **Gestisci modelli** | scarica ed elimina i modelli locali, mostrando l’avanzamento |
| **Imposta chiave API…** / **Aggiorna chiave API…** | aggiunge, sostituisce o rimuove le chiavi |
| **Apri al login** | avvia Lowspeak insieme al Mac; il segno di spunta indica che è attivo |
| **Verifica aggiornamenti…** | cerca subito una nuova versione; le due voci sotto decidono se Lowspeak cerca una volta al giorno e installa gli aggiornamenti senza chiedere. Solo nell’app dall’immagine disco |
| **Aiuto** | **Riferimento rapido**, **Segnala un problema…**, **Suggerisci una funzione…**, **Lowspeak su GitHub** |
| **Informazioni su Lowspeak** | versione, copyright, pagina del progetto e licenza |
| **Esci da Lowspeak** | ⌘Q mentre il menu è aperto |

Il segno di spunta sotto **Locale**, **OpenAI** e **Groq** indica il modello in uso. L’icona mostra che cosa sta facendo Lowspeak:

| Icona | Significato |
|---|---|
| Anello con un punto | Pronto |
| Cerchio pieno | Registrazione mentre tieni premuto fn |
| Quadrato arrotondato | Registrazione bloccata; la pressione successiva su fn o Shift la termina |
| Anello lampeggiante | Trascrizione in corso |
| Anello con un punto in alto a destra | Una nuova versione è in attesa, vedi **Verifica aggiornamenti…** |
| ↓ 42% | Download di un modello |

Lowspeak non ha una finestra delle impostazioni: è tutto in questo menu, che ricorda ogni scelta.

### Riferimento rapido

**Aiuto › Riferimento rapido** mostra i gesti di fn in una piccola finestra, nella tua lingua, con un pulsante che apre le impostazioni della tastiera. **Segnala un problema…** apre un modulo su GitHub con le versioni di Lowspeak e di macOS già compilate, **Suggerisci una funzione…** una discussione per le idee, **Lowspeak su GitHub** la pagina del progetto.

## Dettare

### Tenere premuto per parlare

Tieni premuto fn, parla, rilascia. L’icona di registrazione compare nel momento in cui il microfono inizia a fornire audio; inizia a parlare allora, perché ciò che dici prima non viene registrato. Il rilascio di fn termina la registrazione e il testo arriva un attimo dopo.

Un breve tocco su fn non fa nulla, così fn mantiene i suoi altri usi. Una registrazione che resta vuota o dura meno di 0,3 secondi viene scartata.

### Bloccare con Shift o con una doppia pressione

Per i testi più lunghi, blocca la registrazione in modo che continui senza tenere premuto alcun tasto. Ci sono tre modi:

- **Shift mentre tieni premuto fn.** Inizia con fn come al solito e premi Shift durante la registrazione: si blocca e puoi rilasciare fn. Niente di ciò che hai detto prima va perso, la registrazione semplicemente continua.
- **Shift, poi fn.** Tieni premuto Shift e premi fn: la registrazione parte già bloccata.
- **fn due volte.** Tocca brevemente fn (meno di 0,3 secondi) e premilo di nuovo entro 0,4 secondi. La registrazione parte dalla prima pressione.

L’icona diventa un quadrato arrotondato. Premi **fn** o **Shift** una volta per fermarla; il testo arriva come al solito. Se il tuo tasto 🌐 è impostato per aprire Emoji e simboli, ferma la registrazione con Shift: una breve pressione su fn aprirebbe il visore.

Una registrazione si ferma da sola dopo due minuti se tieni premuto fn e dopo cinque minuti se è bloccata, e viene poi trascritta come al solito.

### Dove finisce il testo

Lowspeak incolla il testo nell’app che era in primo piano quando hai smesso di parlare, nel punto del cursore, tramite gli appunti e ⌘V. Salva tutto ciò che si trovava prima negli appunti e lo rimette al suo posto mezzo secondo dopo che l’app ha letto il testo, o dopo dieci secondi se non lo legge mai, a meno che nel frattempo un’altra app non abbia modificato gli appunti. Il testo è contrassegnato come transitorio, quindi le app di cronologia degli appunti che rispettano questo contrassegno non lo registrano.

Puoi avviare la dettatura successiva mentre la precedente è ancora in trascrizione. I testi vengono incollati nell’ordine in cui li hai pronunciati, e mai mentre stai registrando o tieni premuto Shift, così un testo incollato non può finire a metà della tua frase successiva.

Senza l’autorizzazione Accessibilità, o quando il ⌘V simulato non può essere inviato, il testo resta negli appunti e una notifica lo segnala; incollalo tu con ⌘V.

### Dettature recenti

**Dettature recenti** nel menu elenca gli ultimi dieci testi, dal più recente, ciascuno accorciato a una riga; passandoci sopra con il puntatore si vede il testo completo. Un clic lo copia negli appunti, per quando un testo è finito nel posto sbagliato o ti serve due volte. L’elenco esiste solo in memoria ed è vuoto dopo la chiusura di Lowspeak.

## Trascrizione

### Quale modello fa per te

| Vuoi | Scegli | Perché |
|---|---|---|
| **Velocità senza impegnare il Mac** | **Groq › Whisper Large V3 Turbo** | l’opzione online più veloce; non si scarica nulla e il Mac conserva memoria e spazio su disco |
| **Il risultato migliore** | **OpenAI › GPT Transcribe** (predefinito) | il più preciso tra i modelli offerti da Lowspeak; riceve il tuo vocabolario come parole chiave |
| **Offline e riservato** | **Locale › Parakeet Ultra** per le lingue europee, **Locale › Whisper Large v3 Turbo** per tutte le altre | funziona senza rete e la tua voce non lascia mai il Mac; Parakeet è il più veloce di tutti |
| **Nessun account, nessun costo** | qualsiasi modello **Locale** | gira sul tuo Mac; non viene addebitato nulla |
| **Affidabilità e velocità insieme** | un modello online, più Parakeet Ultra o un modello Whisper installato | il modello locale subentra quando il servizio online non funziona |
| **Un Mac con Intel** | **Groq** o **OpenAI** | lì i modelli locali girano sul processore, vedi [Mac con Intel](#mac-con-intel) |

I modelli locali occupano su disco la loro dimensione di download e, mentre sono selezionati, circa altrettanta memoria; Lowspeak tiene il modello caricato perché la dettatura parta senza attesa. Su un Mac con 8 GB di memoria, preferisci Parakeet Ultra o un modello online ai modelli Whisper grandi.

### Online: OpenAI e Groq

| Modello | Fornitore | Note |
|---|---|---|
| **GPT Transcribe** (predefinito) | OpenAI | il più preciso; riceve il tuo vocabolario come parole chiave |
| **Whisper** | OpenAI | il modello Whisper originale |
| **Whisper Large V3** | Groq | molto veloce |
| **Whisper Large V3 Turbo** | Groq | il più veloce online |

Prima dell’invio la registrazione viene compressa in AAC, circa 6 KB per secondo di parlato, così la dettatura funziona anche con connessioni lente. Mentre tieni premuto fn, Lowspeak apre già la connessione al fornitore, così l’invio parte nel momento in cui rilasci.

### Sul tuo Mac: Whisper e Parakeet

| Modello | Dimensione | Gira su | Lingue |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 lingue europee |
| **Parakeet v2** | ~600 MB | Neural Engine | inglese |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | circa 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | circa 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | circa 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | circa 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | circa 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | circa 99 |

**Gestisci modelli** li scarica e li elimina; durante un download la barra dei menu ne mostra l’avanzamento, e uscire da Lowspeak lo annulla. Il primissimo utilizzo di un modello lo prepara per il tuo Mac: circa 10 secondi per Parakeet Ultra, fino a 30 secondi per Whisper; dopo si carica in una frazione di secondo. Se il modello che hai scelto viene eliminato, Lowspeak passa a un altro modello installato.

**Parakeet** è la scelta più veloce: trascrive 40 secondi di parlato in circa 0,3 secondi. Lowspeak lo esegue con gli stessi modelli e la stessa decodifica di [FluidAudio](https://github.com/FluidInference/FluidAudio) e li conserva nella cartella di FluidAudio (`~/Library/Application Support/FluidAudio/Models`), così le app basate su FluidAudio li condividono. Le registrazioni più lunghe di 15 secondi vengono trascritte in finestre sovrapposte, unite senza perdere parole nei punti di giunzione. **Whisper** capisce più lingue e legge il tuo vocabolario; i modelli grandi sono i più precisi, quelli piccoli molto più deboli al di fuori dell’inglese. I modelli Whisper sono conservati in `~/Library/Application Support/Lowspeak/Models`.

### Quando il servizio online non funziona

Se una trascrizione online non riesce, perché la rete non c’è, il fornitore ha un problema o la chiave è sbagliata, un modello locale installato trascrive la stessa registrazione: quello scelto sotto **Locale**, altrimenti Parakeet Ultra, poi Whisper Large v3 Turbo, Large v3 e i modelli Whisper più piccoli, con Parakeet v2 per ultimo. Una notifica te lo dice una volta; torna solo dopo che nel frattempo una dettatura online è riuscita.

Con un modello locale di riserva, Lowspeak concede alla richiesta online 20 secondi; senza, fa tre tentativi di al massimo un minuto ciascuno. In modalità online, Lowspeak prepara il modello Parakeet di riserva una volta all’avvio, così il primo ricorso alla riserva non deve aspettarlo.

### Come viene preparata la registrazione

Lowspeak registra alla frequenza di campionamento propria del microfono e, quando ti fermi, converte ai 16 kHz che i modelli si aspettano. Prima della trascrizione rimuove il rombo grave sotto gli 80 Hz, taglia il silenzio all’inizio e alla fine e uniforma il volume, così chi parla piano e chi parla forte ottengono lo stesso risultato. Una registrazione senza parlato non viene inviata affatto. In modalità locale, Parakeet riceve prima la registrazione intatta e quella preparata solo se la prima non produce testo.

### Lingue per modello

La lingua scelta sotto **Lingua** viene passata ai modelli online e a Whisper. Con **Rilevamento automatico** il modello individua la lingua da solo.

| Modello | Lingue in cui puoi dettare |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, sul tuo Mac) | circa 99, tra cui tutte le 35 del menu |
| Parakeet Ultra | bulgaro, ceco, croato, danese, estone, finlandese, francese, greco, inglese, italiano, lettone, lituano, maltese, olandese, polacco, portoghese, rumeno, russo, slovacco, sloveno, spagnolo, svedese, tedesco, ucraino, ungherese |
| Parakeet v2 | inglese |

Parakeet riconosce la lingua da solo e ignora la scelta nel menu. **Cinese (tradizionale)** fa sì che GPT Transcribe scriva i caratteri tradizionali; Whisper scrive il cinese in uno qualsiasi dei due sistemi di scrittura e il portoghese senza badare alla variante.

### Mac con Intel

Lowspeak è un’app universale e funziona anche sui Mac con Intel con macOS 14. Tutto ciò che è online funziona allo stesso modo. Lì però i modelli locali non hanno l’hardware per cui sono pensati: Whisper gira sul processore invece che sulla GPU, e Parakeet sul processore invece che sul Neural Engine. Entrambi funzionano, ma impiegano molte volte più tempo che su Apple Silicon, i modelli Whisper grandi decisamente troppo per dettare. Su un Mac con Intel, trascrivi online e tieni Whisper Small o Parakeet Ultra come [riserva](#quando-il-servizio-online-non-funziona).

## Vocabolario

**Vocabolario…** accoglie i nomi e i termini che Lowspeak deve scrivere esattamente, separati da virgole: nomi di prodotti, colleghi, termini tecnici. Fino a 100 termini di massimo 60 caratteri ciascuno; `<` e `>` vengono rimossi.

- **GPT Transcribe** li riceve come parole chiave.
- **Whisper** (OpenAI, Groq e sul tuo Mac) li legge come suggerimento prima della registrazione, fino a 500 caratteri.
- La **post-elaborazione** riceve l’istruzione di scriverli esattamente così come sono scritti.
- **Parakeet** non li usa.

## Post-elaborazione

Dopo la trascrizione, un modello linguistico può rielaborare il testo. Scegli sotto **Post-elaborazione**:

- **Disattivata** (predefinita) — il testo così come è stato trascritto.
- **Pulizia** — rimuove intercalari come «ehm», «tipo» o «praticamente», corregge grammatica e punteggiatura e, quando ti correggi da solo, tiene solo la correzione. Non aggiunge nulla e non cambia il significato.
- **Formattazione** — come Pulizia, e in più adatta il testo all’app in cui detti: termini in stile codice negli editor di codice, righe di comando nei terminali, un tono informale nelle app di chat, uno professionale nelle email.

La post-elaborazione usa Groq (`openai/gpt-oss-120b`) se hai una chiave Groq, altrimenti OpenAI (`gpt-4o-mini`), anche quando trascrivi in locale. Con una lingua scelta sotto **Lingua**, mantiene quella lingua. Se impiega più di otto secondi, non riesce o restituisce una risposta troncata o molto più lunga di quello che hai detto, Lowspeak incolla il testo non elaborato. Se il fornitore rifiuta la richiesta o non è impostata nessuna chiave, una notifica lo segnala una volta.

## Microfono

Sotto **Microfono**, scegli un microfono fisso o **Default di sistema**. Lowspeak ricorda il dispositivo; finché non è collegato, il menu lo mostra come **(non connesso)** e Lowspeak registra con il microfono di default del sistema.

Il microfono viene acceso solo mentre detti. Prepararlo richiede circa 0,07 secondi con il microfono integrato, circa mezzo secondo con uno Studio Display e da uno a due secondi con gli AirPods, che prima passano alla modalità headset. L’icona di registrazione compare appena arriva l’audio; **Segnale acustico quando è pronto** aggiunge un breve suono in quel momento. È disattivato per impostazione predefinita, perché su alcuni altoparlanti il suono stesso arriva con mezzo secondo di ritardo.

Se un microfono fornisce solo silenzio per cinque secondi, come fanno gli AirPods quando un altro dispositivo tiene occupato il loro microfono, Lowspeak te lo segnala.

## Chiavi API

**Imposta chiave API…** chiede una chiave e riconosce il fornitore da come inizia: `sk-` per OpenAI, `gsk_` per Groq. Una chiave in un altro formato viene rifiutata con una spiegazione. Inserisci una chiave per fornitore; con le chiavi di entrambi, sono disponibili tutti i modelli online. **Aggiorna chiave API…** sostituisce una chiave oppure, con **Rimuovi tutte le chiavi**, le elimina.

Le chiavi sono conservate nel tuo portachiavi di login sotto `com.lowspeak.app`. Il file delle chiavi di una versione precedente (`~/Library/Application Support/Lowspeak/.api-key`) viene spostato lì al successivo avvio ed eliminato non appena ogni chiave può essere riletta dal portachiavi.

## Notifiche

Lowspeak lavora in background, quindi segnala i problemi con le notifiche:

| Notifica | Che cosa è successo |
|---|---|
| **Registrazione non riuscita** | nessun accesso al microfono, nessun microfono collegato, oppure il microfono non è riuscito ad avviarsi |
| **Microfono silenzioso** | cinque secondi di silenzio, vedi [Microfono](#microfono) |
| **Autorizzazione di accessibilità richiesta** | il testo è negli appunti; consenti l’accesso in Accessibilità per incollare automaticamente |
| **Incolla non riuscito** | il testo è negli appunti; incollalo con ⌘V |
| **Trascrizione non riuscita** | con il motivo: nessuna chiave, chiave rifiutata, limite di richieste, errore del server o di rete, oppure nessun modello locale |
| **Trascritto su questo Mac** | il servizio online non ha funzionato ed è subentrato un modello locale |
| **Post-elaborazione non riuscita** | il fornitore rifiuta la post-elaborazione o non è impostata nessuna chiave; il testo è stato incollato senza |
| **Nessuna chiave API** | hai scelto un modello online senza una chiave per il suo fornitore |
| **Download non riuscito**, **Eliminazione non riuscita**, **Impossibile eliminare** | la gestione di un modello locale non ha funzionato, oppure un’altra operazione era ancora in corso |
| **Errore elemento login**, **Errore chiave API** | macOS o il portachiavi ha rifiutato la modifica |

Le notifiche non contengono mai il testo che hai dettato. Un servizio online che non funziona e un fornitore di post-elaborazione che rifiuta le richieste vengono segnalati una volta, non dopo ogni dettatura, finché non tornano a funzionare.

## Aspetto e accessibilità

- **L’icona nella barra dei menu** è un’immagine modello che si adatta a una barra dei menu chiara, scura o colorata.
- **VoiceOver** legge l’icona come «Lowspeak», e menu, avvisi e notifiche sono controlli standard di macOS.
- **Nessun tasto va tenuto premuto.** Blocca una registrazione con Shift o con una doppia pressione di fn, e la pressione successiva la termina.
- **Il Riferimento rapido** è una semplice finestra con etichette e un pulsante, raggiungibile da tastiera; Esc la chiude.

Le lacune note sono elencate in [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver non annuncia l’inizio di una registrazione, e Lowspeak non è ancora stato verificato con VoiceOver o con Controllo interruttori.

## Lingue

Menu, finestre di dialogo e notifiche sono tradotti in 35 lingue:

Arabo, catalano, ceco, cinese (semplificato), cinese (tradizionale), coreano, croato, danese, ebraico, finlandese, francese, giapponese, greco, hindi, indonesiano, inglese, italiano, malese, norvegese, olandese, polacco, portoghese (Brasile), portoghese (Portogallo), rumeno, russo, slovacco, sloveno, spagnolo, svedese, tailandese, tedesco, turco, ucraino, ungherese e vietnamita.

La lingua che scegli sotto **Lingua** è anche la lingua del menu. Con **Rilevamento automatico**, il menu segue l’ordine delle lingue in **Impostazioni di Sistema › Generali › Lingua e zona**, e usa l’inglese dove manca una traduzione. I termini dei menu seguono le traduzioni di Apple, così Lowspeak si legge come il resto del tuo Mac. A parte l’inglese, le traduzioni sono automatiche e non sono ancora state riviste da madrelingua; le correzioni sono benvenute come [issue](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Tasti e gesti

| Azione | Tasti |
|---|---|
| Dettare tenendo premuto | tieni premuto fn |
| Bloccare una registrazione in corso | Shift mentre tieni premuto fn |
| Avviare una registrazione bloccata | Shift+fn, oppure fn due volte rapidamente |
| Terminare una registrazione bloccata | fn o Shift |
| Uscire da Lowspeak (menu aperto) | ⌘Q |
| Chiudere il Riferimento rapido | Esc |

Lowspeak impedisce che le pressioni di fn che usa raggiungano altre app; Shift passa come al solito.

## Privacy e sicurezza

Lowspeak sente ciò che dici mentre tieni premuto fn, può scrivere in qualsiasi app e custodisce le tue chiavi API, quindi con tutto questo fa il meno possibile.

- **La tua voce va in un solo posto.** In modalità online la registrazione, compressa, va al fornitore che hai scelto, OpenAI o Groq, insieme al tuo vocabolario. In modalità locale nulla lascia il tuo Mac. Se una richiesta online non riesce, un modello locale trascrive la stessa registrazione sul tuo Mac.
- **La post-elaborazione**, se la attivi, invia il testo trascritto e il tuo vocabolario a Groq o OpenAI. In modalità Formattazione aggiunge il tipo di app in cui detti (editor di codice, terminale, chat o email); il nome dell’app non viene inviato.
- **Nessuna telemetria, nessun account.** A parte trascrizione e post-elaborazione, Lowspeak va online solo per aprire la connessione al tuo fornitore mentre tieni premuto fn, in modalità online o con la post-elaborazione attiva, per scaricare un modello quando lo chiedi, da Hugging Face tramite HTTPS, in una versione fissa e controllando il checksum di ogni file, e per cercare una nuova versione su GitHub, una volta al giorno se lo consenti. Quella richiesta contiene la versione di Lowspeak e nient’altro. **Aiuto › Segnala un problema…** apre un modulo su GitHub nel browser; il suo indirizzo contiene la versione di Lowspeak e quella di macOS, nulla di ciò che hai dettato.
- **Le chiavi API** restano nel tuo portachiavi di login. Vengono inviate solo al rispettivo fornitore e rimosse dai messaggi di errore prima che qualcosa venga scritto nel registro.
- **Il testo dettato non viene salvato.** Le ultime dieci dettature restano in memoria per **Dettature recenti** e spariscono quando Lowspeak viene chiuso. I file di registro annotano lunghezze e tempi, mai il testo.
- **Gli appunti** vengono salvati prima di incollare e ripristinati non appena l’app ha letto il testo, a meno che nel frattempo un’altra app non li abbia modificati. La dettatura è contrassegnata come transitoria, quindi le app di cronologia degli appunti che rispettano questo contrassegno la ignorano.
- **Firma del codice** con Hardened Runtime; l’app chiede accesso solo a microfono, Accessibilità, notifiche e rete. Un aggiornamento deve portare la firma dello sviluppatore, altrimenti Sparkle lo rifiuta.

Per segnalare una vulnerabilità, segui la [politica di sicurezza](../../../SECURITY.md).

## Velocità

Misurata su Apple Silicon:

- **Parakeet Ultra** trascrive 40 secondi di parlato in circa 0,3 secondi.
- **Online:** la prima richiesta dopo una pausa richiede circa 0,4 secondi con Groq, perché Lowspeak apre la connessione mentre stai ancora parlando; le richieste successive la riutilizzano, tramite HTTP/3 dove il fornitore lo offre.
- **Dimensione dell’invio:** circa 6 KB per secondo di parlato, in AAC a 48 kbit/s.
- **Preparazione del microfono:** circa 0,07 secondi con quello integrato, mezzo secondo con uno Studio Display, da uno a due secondi con gli AirPods.

## Limiti noti

- **I modelli locali sui Mac con Intel** girano sul processore e sono molto più lenti; lì la trascrizione online è la scelta migliore.
- **Ciò che dici prima che il microfono sia pronto va perso.** Aspetta l’icona di registrazione, soprattutto con gli AirPods.
- **Il tasto fn può essere conteso.** Se l’impostazione del tasto 🌐 di macOS, Warp o la Dettatura di macOS si prendono fn per primi, Lowspeak non lo vede; vedi [Risoluzione dei problemi](#risoluzione-dei-problemi).
- **Parakeet** ignora il vocabolario e la scelta della lingua, e conosce solo lingue europee.
- **Sistema di scrittura e variante:** solo a GPT Transcribe si può chiedere di scrivere in cinese tradizionale; a nessun modello si può chiedere di scrivere in portoghese europeo anziché brasiliano.
- **Limiti di tempo:** due minuti se tieni premuto fn, cinque minuti in modalità bloccata.
- **Modalità microfono:** Centro di Controllo offre Isolamento vocale e Ampio spettro per Lowspeak e li indica come attivi, ma Lowspeak registra senza elaborazione della voce, quindi la scelta non ha effetto.
- Le **Dettature recenti** esistono solo in memoria e spariscono dopo l’uscita.
- **Per scelta:** niente finestra delle impostazioni, niente icona nel Dock, non è nell’App Store.

## Risoluzione dei problemi

**Non succede nulla quando tengo premuto fn.** Controlla tre cose: Lowspeak è consentito in **Impostazioni di Sistema › Privacy e sicurezza › Accessibilità** (su macOS 27: **Controllo del dispositivo e accesso ai dati**); **Impostazioni di Sistema › Tastiera › Premi il tasto 🌐 per** è impostato su **Nessuna azione**; e nessun’altra app si prende fn. Warp lo fa di default; disattivalo con `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` e riavvia Warp. Anche la Dettatura di macOS può essere impostata per partire con fn.

**Dopo un aggiornamento, Lowspeak non reagisce più a fn.** macOS lega l’autorizzazione Accessibilità alla firma dell’app. Se la firma è cambiata, ad esempio passando da una build compilata da te all’app scaricata, rimuovi Lowspeak dall’elenco Accessibilità (su macOS 27: Controllo del dispositivo e accesso ai dati) e aggiungilo di nuovo.

**Mancano le prime parole.** Inizia a parlare quando compare l’icona di registrazione. Con gli AirPods il microfono ha bisogno di uno o due secondi; un microfono integrato o USB scelto sotto **Microfono** è pronto quasi subito.

**«Microfono silenzioso».** Il microfono non ha fornito audio per cinque secondi. Con gli AirPods, un altro dispositivo potrebbe usare il loro microfono; passa al microfono integrato o ricollegali. Anche un MacBook con il coperchio chiuso silenzia il microfono integrato.

**Le dettature risultano vuote.** Il volume di ingresso potrebbe essere troppo basso: app come Teams, Zoom o Meet lo cambiano durante le chiamate, e macOS lo memorizza per ogni dispositivo. Alzalo in **Impostazioni di Sistema › Suono › Ingresso**.

**Il testo è finito nel posto sbagliato.** **Dettature recenti** nel menu lo copia di nuovo negli appunti.

**«Nessuna chiave API».** Scegli **Imposta chiave API…** nel menu, oppure passa a un modello locale.

**La prima trascrizione locale richiede molto tempo.** Ogni modello viene preparato per il tuo Mac al primo utilizzo; da allora in poi si carica in una frazione di secondo.

**Lowspeak si chiude subito dopo l’avvio.** È già in esecuzione un’altra copia di Lowspeak; chiudi prima quella.

**Non compare nessuna notifica.** Consentile in **Impostazioni di Sistema › Notifiche › Lowspeak**.

**Altro.** I file di registro in `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) descrivono che cosa è successo, senza il testo che hai dettato. Allega le righe rilevanti a una [segnalazione di problema](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Per sviluppatori

La compilazione, i test, la struttura del codice e le regole che segue sono descritti in [docs/development.md](../../../docs/development.md), i contributi in [CONTRIBUTING.md](../../../CONTRIBUTING.md) (entrambi in inglese).

### Componenti di terze parti

| Componente | Versione | Usato per | Licenza |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) tramite [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | Whisper locale sulla GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | decodifica di Parakeet e unione delle registrazioni lunghe, tradotte in Rust in `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) e i suoi crate per i framework | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation e altri framework di sistema | MIT; Zlib, Apache-2.0 o MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | acquisizione dal microfono tramite CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | ricampionamento a 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) con rustls | 3.5 | download dei modelli | MIT o Apache-2.0; rustls e ring sotto ISC e Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | checksum dei download dei modelli | MIT o Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | chiavi API nel portachiavi | MIT o Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, errori, il vocabolario di Parakeet | MIT o Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | aggiornamenti dell’app dall’immagine disco | MIT, con parti sotto licenze BSD e zlib |

I testi delle licenze di tutto ciò che è contenuto nell’app sono inclusi nell’app come `Contents/Resources/Acknowledgements.txt`.

I modelli non fanno parte di Lowspeak; li scarica quando lo chiedi:

| Modello | Fonte | Licenza |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper di OpenAI in formato ggml) | MIT |
| Parakeet v2, Parakeet Ultra | conversioni CoreML di [FluidInference](https://huggingface.co/FluidInference) del Parakeet TDT di NVIDIA | CC-BY-4.0 |

## Licenza

Lowspeak è distribuito con una delle seguenti licenze

- Licenza Apache, versione 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licenza MIT ([LICENSE-MIT](../../../LICENSE-MIT))

a tua scelta.

Salvo diversa indicazione esplicita, qualsiasi contributo inviato intenzionalmente da te per l’inclusione in Lowspeak, come definito nella licenza Apache-2.0, sarà distribuito con la doppia licenza di cui sopra, senza termini o condizioni aggiuntivi.
