<p><img src="../../readme-icon.png" alt="Lowspeak-appikon" width="168" height="168"></p>

# Lowspeak

**Diktering for Mac som skriver der markøren står: hold inne fn, snakk, slipp.**

Lowspeak bor i menylinjen. Hold inne fn-tasten (Globus) og snakk; når du slipper, dukker ordene dine opp i den appen du skriver i, som om du hadde skrevet dem selv. Appen transkriberer på nett med OpenAI eller Groq, eller helt og holdent på Macen din med Whisper eller NVIDIA Parakeet, som gjør 40 sekunder tale om til tekst på omtrent en tredjedels sekund. Hvis nettverket eller nettjenesten svikter, tar en installert lokal modell over, så ingen diktering går tapt. Lær den navnene og begrepene du bruker, la en språkmodell fjerne fyllord hvis du vil, og lås et opptak med Shift for å diktere lengre tekster håndfritt. Lowspeak er skrevet i Rust på ekte AppKit-kontroller, snakker 35 språk og sender stemmen din bare til leverandøren du har valgt.

[![Last ned Lowspeak for macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Last ned Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: signert og notarisert av Apple, for macOS 14 eller nyere på Apple Silicon og Intel.

**Fant du en feil?** [Opprett et issue](https://github.com/sventmueller/lowspeak/issues/new/choose), eller velg **Hjelp › Rapporter et problem…** i Lowspeak-menyen: skjemaet åpnes med versjonen din allerede fylt inn. Hvis du har nytte av Lowspeak, hjelper en [stjerne på GitHub](https://github.com/sventmueller/lowspeak) andre med å finne det.

[![Rust 2024 edition](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 eller nyere](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universell: Apple Silicon og Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit via objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp på Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet på Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![På nett med OpenAI eller Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 grensesnittspråk](https://img.shields.io/badge/languages-35-4C9A2A)
![Signert med Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Lisens: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#lisens)

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
**Norsk** ·
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

## Innhold

- [Funksjoner i korte trekk](#funksjoner-i-korte-trekk)
- [Krav](#krav)
- [Installasjon](#installasjon)
- [Kom i gang](#kom-i-gang)
  - [Tillatelser](#tillatelser)
  - [Velg hvordan du vil transkribere](#velg-hvordan-du-vil-transkribere)
  - [Menyen](#menyen)
  - [Hurtigoversikt](#hurtigoversikt)
- [Diktere](#diktere)
  - [Hold inne og snakk](#hold-inne-og-snakk)
  - [Lås med Shift eller et dobbelttrykk](#lås-med-shift-eller-et-dobbelttrykk)
  - [Hvor teksten havner](#hvor-teksten-havner)
  - [Nylige dikteringer](#nylige-dikteringer)
- [Transkribering](#transkribering)
  - [Hvilken modell passer for deg](#hvilken-modell-passer-for-deg)
  - [På nett: OpenAI og Groq](#på-nett-openai-og-groq)
  - [På Macen din: Whisper og Parakeet](#på-macen-din-whisper-og-parakeet)
  - [Når nettjenesten svikter](#når-nettjenesten-svikter)
  - [Slik klargjøres opptaket](#slik-klargjøres-opptaket)
  - [Språk per modell](#språk-per-modell)
  - [Mac-maskiner med Intel](#mac-maskiner-med-intel)
- [Ordliste](#ordliste)
- [Etterbehandling](#etterbehandling)
- [Mikrofon](#mikrofon)
- [API-nøkler](#api-nøkler)
- [Varslinger](#varslinger)
- [Utseende og tilgjengelighet](#utseende-og-tilgjengelighet)
- [Språk](#språk)
- [Taster og bevegelser](#taster-og-bevegelser)
- [Personvern og sikkerhet](#personvern-og-sikkerhet)
- [Hastighet](#hastighet)
- [Kjente begrensninger](#kjente-begrensninger)
- [Feilsøking](#feilsøking)
- [For utviklere](#for-utviklere)
  - [Tredjepartskomponenter](#tredjepartskomponenter)
- [Lisens](#lisens)

## Funksjoner i korte trekk

- **Hold inne fn, snakk, slipp.** Teksten skrives inn i appen som ligger foran, der markøren står: e-post, chat, redigeringsprogram, terminal eller nettleser. Ingen vinduer å bytte til, ingen knapp å klikke på.
- **Lås opptaket for lengre tekster.** Trykk på Shift mens du holder inne fn, eller trykk to ganger på fn, så fortsetter opptaket uten at du holder inne noen tast, i opptil fem minutter. Ett trykk på fn eller Shift avslutter det.
- **Riktig modell for jobben.** Groq for fart uten å bruke minnet på Macen din, OpenAIs GPT Transcribe for det beste resultatet, Whisper eller NVIDIA Parakeet på Macen din for å jobbe uten nett og holde stemmen din privat. Bytt med ett klikk.
- **Ingen diktering går tapt.** Hvis nettverket eller nettjenesten svikter, transkriberer en installert lokal modell det samme opptaket på Macen din.
- **Ordene dine, riktig stavet.** En ordliste med navn og faguttrykk styrer OpenAI, Groq, Whisper og etterbehandlingen.
- **Ryddig tekst hvis du vil.** Valgfri etterbehandling fjerner fyllord, retter grammatikken og beholder bare rettelsen når du retter deg selv, eller formaterer i tillegg teksten etter hva slags app du dikterer i.
- **Bare fortsett å snakke.** Start neste diktering mens den forrige fortsatt transkriberes; tekstene kommer i den rekkefølgen du sa dem.
- **Utklippstavlen forblir din.** Lowspeak limer inn via utklippstavlen og legger tilbake det som lå der, et øyeblikk senere. De ti siste dikteringene blir liggende i menyen, så du kan kopiere dem igjen.
- **Native helt inn i margen.** Et ekte menylinjeobjekt med en AppKit-meny, standard dialogruter og varslinger. Ingen Electron, ingen vinduer, intet Dock-symbol.
- **Privat.** Ingen konto, ingen telemetri. Lyden går bare til leverandøren du har valgt, eller ingen steder i det hele tatt i lokal modus.
- **På ditt språk.** Diktering fungerer på dusinvis av språk, og menyen, dialogrutene og varslingene finnes på 35.

## Krav

- macOS 14 Sonoma eller nyere. Lowspeak er utviklet og testet på macOS 27.
- En Mac med Apple Silicon eller en Intel-prosessor. Lokal transkribering er laget for Apple Silicon, der Whisper kjører på GPU-en og Parakeet på Neural Engine; på Mac-maskiner med Intel kjører begge på prosessoren, mye langsommere, se [Mac-maskiner med Intel](#mac-maskiner-med-intel).
- Rundt 10 MB for appen, pluss 75 MB til 3 GB for hver lokale modell du laster ned.
- For transkribering på nett en API-nøkkel fra [OpenAI](https://platform.openai.com/api-keys) eller [Groq](https://console.groq.com/keys); begge tar betalt for det du transkriberer. Lokal transkribering krever ingen konto.

## Installasjon

**Fra diskbildet.** Last ned [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), åpne det og dra Lowspeak til Apper-mappen. Bildet er signert med en Developer ID og notarisert av Apple, så det åpnes uten advarsler.

**Oppdateringer.** Lowspeak oppdaterer seg ikke selv. Nye versjoner legges ut på [utgivelsessiden](https://github.com/sventmueller/lowspeak/releases); **Hjelp › Lowspeak på GitHub** fører dit. Installer en ny versjon over den gamle; innstillingene, nøklene og modellene dine blir værende.

**Fra kildekoden.** Se [Bygging og distribusjon](../../../docs/development.md#build--deploy) for forutsetningene, og kjør deretter:

```sh
make deploy
```

Dette bygger Lowspeak, erstatter kopien i Apper-mappen og starter den.

**Åpne ved pålogging** i menyen starter Lowspeak sammen med Macen din, via påloggingsobjektene i macOS. Bare én kopi kjører om gangen; en ny kopi avslutter seg med en gang.

## Kom i gang

### Tillatelser

Ved første oppstart ber Lowspeak om tre tillatelser:

- **Tilgjengelighet** (Systeminnstillinger › Personvern og sikkerhet › Tilgjengelighet; i macOS 27 heter listen Enhetskontroll og datatilgang). Lowspeak trenger den for å registrere fn-tasten og for å lime inn med ⌘V. macOS tilbyr å åpne listen; slå på bryteren for Lowspeak. Lowspeaks egne meldinger bruker det navnet på listen som din macOS viser.
- **Mikrofon.** Spørres om én gang; uten den tar Lowspeak ikke opp noe.
- **Varslinger.** Lowspeak bruker dem til problemer du ellers ikke ville sett, for eksempel en stille mikrofon eller en mislykket transkribering.

### Velg hvordan du vil transkribere

Sett opp minst én måte; du kan bytte når som helst i menyen.

- **På nett:** velg **Angi API-nøkkel…** og lim inn en nøkkel fra OpenAI (den begynner med `sk-`) eller Groq (`gsk_`). Lowspeak gjenkjenner leverandøren på nøkkelen. Velg deretter en modell under **OpenAI** eller **Groq**.
- **På Macen din:** velg **Administrer modeller** › **Last ned Parakeet Ultra** eller en Whisper-modell, og velg den deretter under **Lokalt**.

[Hvilken modell passer for deg](#hvilken-modell-passer-for-deg) hjelper deg med valget. Hold deretter inne fn, si noe og slipp.

### Menyen

Klikk på ringen i menylinjen:

| Valg | Hva det gjør |
|---|---|
| **Nylige dikteringer** | de ti siste tekstene; et klikk kopierer en av dem til utklippstavlen igjen |
| **Lokalt** | modellene på Macen din, gruppert i Whisper og Parakeet; de som ikke er lastet ned, er nedtonet |
| **OpenAI**, **Groq** | nettmodellene; uten nøkkel er de nedtonet, og menyen tilbyr **Angi API-nøkkel…** |
| **Språk** | språket du dikterer på, eller **Gjenkjenn automatisk**; menyen følger det |
| **Etterbehandling** | **Av**, **Rydd opp** eller **Formater**, se [Etterbehandling](#etterbehandling) |
| **Ordliste…** | navn og begreper som skal staves nøyaktig; menyen viser hvor mange du har lagt inn |
| **Mikrofon** | **Systemstandard** eller en fast mikrofon, og **Spill av lyd når klar** |
| **Administrer modeller** | last ned og slett lokale modeller, med fremdrift |
| **Angi API-nøkkel…** / **Endre API-nøkkel…** | legg til, erstatt eller fjern nøkler |
| **Åpne ved pålogging** | start Lowspeak med Macen din; haken viser at det er slått på |
| **Hjelp** | **Hurtigoversikt**, **Rapporter et problem…**, **Foreslå en funksjon…**, **Lowspeak på GitHub** |
| **Om Lowspeak** | versjon, opphavsrett, prosjektside og lisens |
| **Avslutt Lowspeak** | ⌘Q mens menyen er åpen |

Haken under **Lokalt**, **OpenAI** og **Groq** markerer modellen som er i bruk. Symbolet viser hva Lowspeak gjør:

| Symbol | Betydning |
|---|---|
| Ring med en prikk | Klar |
| Fylt sirkel | Tar opp mens fn holdes inne |
| Avrundet kvadrat | Opptaket er låst; neste trykk på fn eller Shift avslutter det |
| Blinkende ring | Transkriberer |
| ↓ 42 % | Laster ned en modell |

Lowspeak har ikke noe innstillingsvindu: alt ligger i denne menyen, og appen husker hvert valg.

### Hurtigoversikt

**Hjelp › Hurtigoversikt** viser fn-bevegelsene i et lite vindu, på ditt språk, med en knapp som åpner tastaturinnstillingene. **Rapporter et problem…** åpner et skjema på GitHub med versjonene dine av Lowspeak og macOS allerede fylt inn, **Foreslå en funksjon…** en diskusjon for ideer og **Lowspeak på GitHub** prosjektsiden.

## Diktere

### Hold inne og snakk

Hold inne fn, snakk, slipp. Opptakssymbolet vises i det øyeblikket mikrofonen leverer lyd; begynn å snakke da, for det du sier før det, blir ikke tatt opp. Når du slipper fn, avsluttes opptaket, og teksten kommer et øyeblikk senere.

Et kort trykk på fn gjør ingenting, så fn beholder de andre funksjonene sine. Et opptak som forblir tomt eller varer kortere enn 0,3 sekunder, forkastes.

### Lås med Shift eller et dobbelttrykk

For lengre tekster låser du opptaket, så det fortsetter uten at du holder inne noen tast. Det finnes tre måter:

- **Shift mens du holder inne fn.** Start med fn som vanlig, og trykk på Shift under opptaket: det låses, og du kan slippe fn. Ingenting av det du allerede har sagt, går tapt; opptaket bare fortsetter.
- **Shift, så fn.** Hold inne Shift og trykk på fn: opptaket starter låst.
- **fn to ganger.** Trykk kort på fn (kortere enn 0,3 sekunder) og trykk igjen innen 0,4 sekunder. Opptaket gjelder fra det første trykket.

Symbolet blir et avrundet kvadrat. Trykk én gang på **fn** eller **Shift** for å stoppe; teksten kommer som vanlig. Hvis 🌐-tasten er stilt inn til å vise Emoji og symboler, stopper du med Shift: et kort trykk på fn ville åpnet tegnvisningen.

Et opptak stopper av seg selv etter to minutter mens fn holdes inne, og etter fem minutter når det er låst, og transkriberes deretter som vanlig.

### Hvor teksten havner

Lowspeak limer inn teksten i appen som lå foran da du sluttet å snakke, der markøren står, via utklippstavlen og ⌘V. Appen lagrer alt som lå på utklippstavlen fra før, og legger det tilbake et halvt sekund etter at appen som mottar teksten, har lest den, eller etter ti sekunder hvis det aldri skjer, med mindre en annen app har endret utklippstavlen i mellomtiden. Teksten er merket som midlertidig, så apper for utklippshistorikk som respekterer merket, ikke registrerer den.

Du kan starte neste diktering mens den forrige fortsatt transkriberes. Tekstene limes inn i den rekkefølgen du sa dem, og aldri mens du tar opp eller holder inne Shift, så en innliming kan ikke havne midt i neste setning.

Uten tilgjengelighetstillatelsen, eller når det simulerte ⌘V ikke kan sendes, blir teksten liggende på utklippstavlen, og en varsling sier fra; lim den inn selv med ⌘V.

### Nylige dikteringer

**Nylige dikteringer** i menyen viser de ti siste tekstene, nyeste først, hver forkortet til én linje; holder du pekeren over en tekst, vises hele teksten. Et klikk kopierer den til utklippstavlen, for når en tekst har havnet på feil sted eller du trenger den to ganger. Listen finnes bare i minnet og er tom etter at Lowspeak er avsluttet.

## Transkribering

### Hvilken modell passer for deg

| Du vil ha | Velg | Hvorfor |
|---|---|---|
| **Fart uten å belaste Macen** | **Groq › Whisper Large V3 Turbo** | det raskeste nettalternativet; ingenting lastes ned, og Macen beholder minnet og diskplassen sin |
| **Det beste resultatet** | **OpenAI › GPT Transcribe** (standard) | den mest nøyaktige av modellene Lowspeak tilbyr; tar imot ordlisten din som nøkkelord |
| **Uten nett og privat** | **Lokalt › Parakeet Ultra** for europeiske språk, **Lokalt › Whisper Large v3 Turbo** for alle andre | fungerer uten nettverk, og stemmen din forlater aldri Macen; Parakeet er raskest av alle |
| **Ingen konto, ingen kostnader** | hvilken som helst modell under **Lokalt** | kjører på Macen din; ingenting faktureres |
| **Både pålitelighet og fart** | en nettmodell, pluss Parakeet Ultra eller en Whisper-modell installert | den lokale modellen trer inn når nettjenesten svikter |
| **En Mac med Intel** | **Groq** eller **OpenAI** | lokale modeller kjører der på prosessoren, se [Mac-maskiner med Intel](#mac-maskiner-med-intel) |

Lokale modeller tar opp nedlastingsstørrelsen sin på disken og, mens de er valgt, omtrent like mye minne; Lowspeak holder modellen lastet, så dikteringen starter uten forsinkelse. På en Mac med 8 GB minne bør du velge Parakeet Ultra eller en nettmodell fremfor de store Whisper-modellene.

### På nett: OpenAI og Groq

| Modell | Leverandør | Merknader |
|---|---|---|
| **GPT Transcribe** (standard) | OpenAI | mest nøyaktig; tar imot ordlisten din som nøkkelord |
| **Whisper** | OpenAI | den opprinnelige Whisper-modellen |
| **Whisper Large V3** | Groq | svært rask |
| **Whisper Large V3 Turbo** | Groq | raskest på nett |

Opptaket komprimeres til AAC før opplastingen, rundt 6 KB per sekund tale, så diktering fungerer også på trege forbindelser. Mens du holder inne fn, åpner Lowspeak allerede forbindelsen til leverandøren, så opplastingen starter i det øyeblikket du slipper.

### På Macen din: Whisper og Parakeet

| Modell | Størrelse | Kjører på | Språk |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 europeiske språk |
| **Parakeet v2** | ~600 MB | Neural Engine | engelsk |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | omtrent 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | omtrent 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | omtrent 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | omtrent 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | omtrent 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | omtrent 99 |

**Administrer modeller** laster dem ned og sletter dem; under en nedlasting viser menylinjen fremdriften, og hvis du avslutter Lowspeak, avbrytes den. Aller første gang en modell brukes, klargjøres den for Macen din: rundt 10 sekunder for Parakeet Ultra, opptil 30 sekunder for Whisper; deretter lastes den på en brøkdel av et sekund. Hvis modellen du har valgt, slettes, bytter Lowspeak til en annen installert modell.

**Parakeet** er det raskeste valget: den transkriberer 40 sekunder tale på rundt 0,3 sekunder. Lowspeak kjører den med de samme modellene og den samme dekodingen som [FluidAudio](https://github.com/FluidInference/FluidAudio) og lagrer modellene i FluidAudios mappe (`~/Library/Application Support/FluidAudio/Models`), så apper som bygger på FluidAudio, deler dem. Opptak lengre enn 15 sekunder transkriberes i overlappende vinduer som settes sammen uten at ord går tapt i skjøtene. **Whisper** forstår flere språk og leser ordlisten din; de store modellene er mest nøyaktige, de små mye svakere utenfor engelsk. Whisper-modeller lagres i `~/Library/Application Support/Lowspeak/Models`.

### Når nettjenesten svikter

Hvis en transkribering på nett mislykkes, fordi nettverket er nede, leverandøren har et problem eller nøkkelen er feil, transkriberer en installert lokal modell det samme opptaket: den som er valgt under **Lokalt**, ellers Parakeet Ultra, deretter Whisper Large v3 Turbo, Large v3 og de mindre Whisper-modellene, med Parakeet v2 sist. En varsling sier fra én gang; den kommer først igjen når en diktering på nett har lyktes i mellomtiden.

Med en lokal modell å falle tilbake på gir Lowspeak forespørselen på nett 20 sekunder; uten en slik modell prøver appen tre ganger med opptil ett minutt hver. I nettmodus klargjør Lowspeak reservemodellen Parakeet én gang ved oppstart, så den første reservetranskriberingen ikke må vente på den.

### Slik klargjøres opptaket

Lowspeak tar opp med mikrofonens egen samplingsfrekvens og konverterer til de 16 kHz som modellene forventer, når du stopper. Før transkriberingen fjerner appen lavfrekvent rumling under 80 Hz, klipper bort stillhet i begynnelsen og slutten og jevner ut volumet, så stille og høylytte talere kommer ut likt. Et opptak uten tale sendes ikke i det hele tatt. I lokal modus får Parakeet først det ubehandlede opptaket, og det klargjorte bare hvis det første ikke gir noen tekst.

### Språk per modell

Språket under **Språk** sendes til nettmodellene og til Whisper. Med **Gjenkjenn automatisk** finner modellen selv ut hvilket språk det er.

| Modell | Språk du kan diktere på |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, på Macen din) | omtrent 99, blant dem alle de 35 i menyen |
| Parakeet Ultra | bulgarsk, kroatisk, tsjekkisk, dansk, nederlandsk, engelsk, estisk, finsk, fransk, tysk, gresk, ungarsk, italiensk, latvisk, litauisk, maltesisk, polsk, portugisisk, rumensk, russisk, slovakisk, slovensk, spansk, svensk, ukrainsk |
| Parakeet v2 | engelsk |

Parakeet gjenkjenner språket selv og ignorerer valget i menyen. **Kinesisk (tradisjonelt)** får GPT Transcribe til å skrive tradisjonelle tegn; Whisper skriver kinesisk med hvilket som helst av de to skriftsystemene og portugisisk uten hensyn til varianten.

### Mac-maskiner med Intel

Lowspeak er en universell app og kjører også på Mac-maskiner med Intel og macOS 14. Alt på nett fungerer likt. De lokale modellene mangler der maskinvaren de er laget for: Whisper kjører på prosessoren i stedet for GPU-en, og Parakeet på prosessoren i stedet for Neural Engine. Begge fungerer, men bruker mange ganger lengre tid enn på Apple Silicon, de store Whisper-modellene altfor lang tid for diktering. På en Mac med Intel bør du transkribere på nett og beholde Whisper Small eller Parakeet Ultra som [reserve](#når-nettjenesten-svikter).

## Ordliste

**Ordliste…** tar imot navnene og begrepene Lowspeak skal stave nøyaktig, skilt med komma: produktnavn, kolleger, faguttrykk. Opptil 100 begreper på opptil 60 tegn hver; `<` og `>` fjernes.

- **GPT Transcribe** får dem som nøkkelord.
- **Whisper** (OpenAI, Groq og på Macen din) leser dem som et hint før opptaket, opptil 500 tegn.
- **Etterbehandlingen** får beskjed om å stave dem nøyaktig slik de er skrevet.
- **Parakeet** bruker dem ikke.

## Etterbehandling

Etter transkriberingen kan en språkmodell bearbeide teksten. Velg under **Etterbehandling**:

- **Av** (standard) – teksten slik den ble transkribert.
- **Rydd opp** – fjerner fyllord som «eh», «liksom» eller «egentlig», retter grammatikk og tegnsetting, og beholder bare rettelsen når du retter deg selv. Den legger ikke til noe og endrer ikke meningen.
- **Formater** – som Rydd opp, og tilpasser i tillegg teksten til appen du dikterer i: kodeaktige begreper i kodeeditorer, kommandolinjer i terminaler, en uformell tone i chatteapper, en profesjonell tone i e-post.

Etterbehandlingen bruker Groq (`openai/gpt-oss-120b`) hvis du har en Groq-nøkkel, ellers OpenAI (`gpt-4o-mini`), også når du transkriberer lokalt. Når et språk er valgt under **Språk**, beholder den det språket. Hvis den tar lengre tid enn åtte sekunder, mislykkes eller gir et svar som er avkuttet eller langt lengre enn det du sa, limer Lowspeak inn den ubehandlede teksten. Hvis leverandøren avviser forespørselen eller ingen nøkkel er angitt, sier en varsling fra én gang.

## Mikrofon

Under **Mikrofon** velger du en fast mikrofon eller **Systemstandard**. Lowspeak husker enheten; mens den ikke er tilkoblet, viser menyen den som **(ikke tilkoblet)**, og Lowspeak tar opp med systemstandarden.

Mikrofonen slås bare på mens du dikterer. Å gjøre den klar tar rundt 0,07 sekunder med den innebygde mikrofonen, omtrent et halvt sekund med en Studio Display og ett til to sekunder med AirPods, som først bytter til headsetmodus. Opptakssymbolet vises når lyden kommer frem; **Spill av lyd når klar** legger til en kort lyd i det øyeblikket. Det er slått av som standard, fordi selve lyden kommer et halvt sekund for sent på enkelte høyttalere.

Hvis en mikrofon bare leverer stillhet i fem sekunder, slik AirPods gjør når en annen enhet holder på mikrofonen deres, sier Lowspeak fra.

## API-nøkler

**Angi API-nøkkel…** ber om en nøkkel og gjenkjenner leverandøren på begynnelsen av den: `sk-` for OpenAI, `gsk_` for Groq. En nøkkel i et annet format avvises med en forklaring. Legg inn én nøkkel per leverandør; med nøkler for begge er alle nettmodellene tilgjengelige. **Endre API-nøkkel…** erstatter en nøkkel eller, med **Fjern alle nøkler**, sletter dem.

Nøklene lagres på påloggingsnøkkelringen din under `com.lowspeak.app`. En nøkkelfil fra en tidligere versjon (`~/Library/Application Support/Lowspeak/.api-key`) flyttes dit ved neste oppstart og slettes så snart hver nøkkel kan leses tilbake fra nøkkelringen.

## Varslinger

Lowspeak jobber i bakgrunnen, så appen melder problemer som varslinger:

| Varsling | Hva som skjedde |
|---|---|
| **Opptaket mislyktes** | ingen tilgang til mikrofonen, ingen mikrofon tilkoblet, eller mikrofonen kunne ikke starte |
| **Ingen lyd fra mikrofonen** | fem sekunder med stillhet, se [Mikrofon](#mikrofon) |
| **Tillatelse for tilgjengelighet kreves** | teksten ligger på utklippstavlen; tillat Tilgjengelighet for å lime inn automatisk |
| **Innliming mislyktes** | teksten ligger på utklippstavlen; lim den inn med ⌘V |
| **Transkriberingen mislyktes** | med årsaken: ingen nøkkel, avvist nøkkel, grensen for antall forespørsler, server- eller nettverksfeil, eller ingen lokal modell |
| **Transkribert på denne Macen** | nettjenesten sviktet, og en lokal modell tok over |
| **Etterbehandlingen mislyktes** | leverandøren avviser etterbehandlingen eller ingen nøkkel er angitt; teksten ble limt inn uten |
| **Ingen API-nøkkel** | du valgte en nettmodell uten nøkkel for leverandøren dens |
| **Nedlastingen mislyktes**, **Slettingen mislyktes**, **Kan ikke slette** | administrasjonen av en lokal modell fungerte ikke, eller en annen handling pågikk fortsatt |
| **Feil med påloggingsobjekt**, **Feil med API-nøkkel** | macOS eller nøkkelringen avviste endringen |

Varslinger inneholder aldri den dikterte teksten din. En nettjeneste som svikter, og en leverandør som avviser etterbehandling, meldes én gang, ikke etter hver diktering, til de fungerer igjen.

## Utseende og tilgjengelighet

- **Menylinjesymbolet** er et malbilde som følger en lys, mørk eller farget menylinje.
- **VoiceOver** leser symbolet som «Lowspeak», og menyen, dialogrutene og varslingene er standardkontroller i macOS.
- **Ingen tast må holdes inne.** Lås et opptak med Shift eller et dobbelttrykk på fn, så avslutter neste trykk det.
- **Hurtigoversikten** er et enkelt vindu med etiketter og en knapp, som kan nås fra tastaturet; Esc lukker det.

Kjente mangler står i [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver sier ikke fra når et opptak starter, og Lowspeak er ennå ikke kontrollert med VoiceOver eller Bryterkontroll.

## Språk

Menyen, dialogrutene og varslingene er oversatt til 35 språk:

arabisk, katalansk, kinesisk (forenklet), kinesisk (tradisjonelt), kroatisk, tsjekkisk, dansk, nederlandsk, engelsk, finsk, fransk, tysk, gresk, hebraisk, hindi, ungarsk, indonesisk, italiensk, japansk, koreansk, malayisk, norsk, polsk, portugisisk (Brasil), portugisisk (Portugal), rumensk, russisk, slovakisk, slovensk, spansk, svensk, thai, tyrkisk, ukrainsk og vietnamesisk.

Språket du velger under **Språk**, er også språket i menyen. Med **Gjenkjenn automatisk** følger menyen språkrekkefølgen i **Systeminnstillinger › Generelt › Språk og område**, og engelsk der det ikke finnes noen oversettelse. Menybegrepene følger Apples egne oversettelser, så Lowspeak leses som resten av Macen din. Bortsett fra engelsk er oversettelsene maskinlagde og ennå ikke gjennomgått av morsmålsbrukere; rettelser tas gjerne imot som [issues](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Taster og bevegelser

| Handling | Taster |
|---|---|
| Dikter mens tasten holdes inne | hold inne fn |
| Lås et pågående opptak | Shift mens du holder inne fn |
| Start et låst opptak | Shift+fn, eller fn to ganger raskt |
| Avslutt et låst opptak | fn eller Shift |
| Avslutt Lowspeak (med menyen åpen) | ⌘Q |
| Lukk hurtigoversikten | Esc |

Lowspeak hindrer at fn-trykkene appen bruker, når frem til andre apper; Shift slippes gjennom som vanlig.

## Personvern og sikkerhet

Lowspeak hører hva du sier mens fn holdes inne, kan skrive i hvilken som helst app og oppbevarer API-nøklene dine, så appen gjør så lite som mulig med alt dette.

- **Stemmen din går til ett sted.** I nettmodus går opptaket, komprimert, til leverandøren du har valgt, OpenAI eller Groq, sammen med ordlisten din. I lokal modus forlater ingenting Macen din. Når en forespørsel på nett mislykkes, transkriberer en lokal modell det samme opptaket på Macen din.
- **Etterbehandling**, hvis du slår den på, sender den transkriberte teksten og ordlisten din til Groq eller OpenAI. I modusen Formater sendes også hva slags app du dikterer i (kodeeditor, terminal, chat eller e-post); navnet på appen sendes ikke.
- **Ingen telemetri, ingen konto.** Bortsett fra transkribering og etterbehandling går Lowspeak bare på nett for å åpne forbindelsen til leverandøren din mens fn holdes inne, i nettmodus eller med etterbehandling slått på, og for å laste ned en modell når du ber om det, fra Hugging Face over HTTPS, i en fast versjon og med hver fil kontrollert mot sjekksummen sin. **Hjelp › Rapporter et problem…** åpner et skjema på GitHub i nettleseren; adressen inneholder versjonen av Lowspeak og macOS, ingenting av det du har diktert.
- **API-nøkler** blir liggende på påloggingsnøkkelringen din. De sendes bare til sin egen leverandør og fjernes fra feilmeldinger før noe skrives til loggen.
- **Diktert tekst lagres ikke.** De ti siste dikteringene ligger i minnet for **Nylige dikteringer** og forsvinner når Lowspeak avsluttes. Loggfilene registrerer lengder og tider, aldri teksten.
- **Utklippstavlen** lagres før innlimingen og gjenopprettes så snart appen som mottar teksten, har lest den, med mindre en annen app har endret utklippstavlen i mellomtiden. Dikteringen er merket som midlertidig, så apper for utklippshistorikk som respekterer merket, hopper over den.
- **Kodesignering** med Hardened Runtime; appen ber bare om tilgang til mikrofonen, Tilgjengelighet, varslinger og nettverket.

Følg [sikkerhetspolicyen](../../../SECURITY.md) for å melde fra om et sikkerhetshull.

## Hastighet

Målt på Apple Silicon:

- **Parakeet Ultra** transkriberer 40 sekunder tale på rundt 0,3 sekunder.
- **På nett:** den første forespørselen etter en pause tar rundt 0,4 sekunder hos Groq, fordi Lowspeak åpner forbindelsen mens du fortsatt snakker; senere forespørsler gjenbruker den, over HTTP/3 der leverandøren tilbyr det.
- **Opplastingsstørrelse:** rundt 6 KB per sekund tale, som AAC med 48 kbit/s.
- **Å gjøre mikrofonen klar:** rundt 0,07 sekunder for den innebygde, et halvt sekund på en Studio Display, ett til to sekunder med AirPods.

## Kjente begrensninger

- **Lokale modeller på Mac-maskiner med Intel** kjører på prosessoren og er mye langsommere; der er transkribering på nett det beste valget.
- **Det du sier før mikrofonen er klar, går tapt.** Vent på opptakssymbolet, særlig med AirPods.
- **fn-tasten kan bli tatt av andre.** Når innstillingen for 🌐-tasten i macOS, Warp eller diktering i macOS tar fn først, ser ikke Lowspeak den; se [Feilsøking](#feilsøking).
- **Parakeet** ignorerer ordlisten og språkvalget, og kan bare europeiske språk.
- **Skriftsystem og variant:** bare GPT Transcribe kan få beskjed om å skrive tradisjonell kinesisk; ingen modell kan få beskjed om å skrive europeisk i stedet for brasiliansk portugisisk.
- **Tidsgrenser:** to minutter mens fn holdes inne, fem minutter når opptaket er låst.
- **Mikrofonmodus:** Kontrollsenter tilbyr Stemmeisolasjon og Bredt spektrum for Lowspeak under **Mikrofon** og viser dem som aktive, men Lowspeak tar opp uten stemmebehandling, så valget har ingen virkning.
- **Nylige dikteringer** finnes bare i minnet og forsvinner når appen avsluttes.
- **Bevisst valgt:** ikke noe innstillingsvindu, intet Dock-symbol, ingen automatiske oppdateringer, ikke i App Store.

## Feilsøking

**Ingenting skjer når jeg holder inne fn.** Sjekk tre ting: at Lowspeak er tillatt under **Systeminnstillinger › Personvern og sikkerhet › Tilgjengelighet** (i macOS 27: **Enhetskontroll og datatilgang**); at **Systeminnstillinger › Tastatur › Trykk på 🌐 for å** er satt til **ikke gjøre noe**; og at ingen annen app tar fn. Warp gjør det som standard; slå det av med `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` og start Warp på nytt. Diktering i macOS kan også være stilt inn til å starte med fn.

**Etter en oppdatering reagerer ikke Lowspeak lenger på fn.** macOS knytter tilgjengelighetstillatelsen til appens signatur. Hvis signaturen er endret, for eksempel fra et bygg du har laget selv til den nedlastede appen, fjerner du Lowspeak fra Tilgjengelighet-listen (i macOS 27: Enhetskontroll og datatilgang) og legger den til igjen.

**De første ordene mangler.** Begynn å snakke når opptakssymbolet vises. Med AirPods trenger mikrofonen ett til to sekunder; en innebygd mikrofon eller en USB-mikrofon valgt under **Mikrofon** er klar nesten med en gang.

**«Ingen lyd fra mikrofonen.»** Mikrofonen leverte ingen lyd på fem sekunder. Med AirPods kan en annen enhet bruke mikrofonen deres; bytt til den innebygde mikrofonen eller koble dem til på nytt. Et lukket MacBook-lokk demper også den innebygde mikrofonen.

**Dikteringene blir tomme.** Inngangsvolumet kan være for lavt: apper som Teams, Zoom eller Meet endrer det under samtaler, og macOS lagrer det per enhet. Skru det opp under **Systeminnstillinger › Lyd › Lyd inn**.

**Teksten havnet på feil sted.** **Nylige dikteringer** i menyen kopierer den til utklippstavlen igjen.

**«Ingen API-nøkkel.»** Velg **Angi API-nøkkel…** i menyen, eller bytt til en lokal modell.

**Den første lokale transkriberingen tar lang tid.** Hver modell klargjøres for Macen din første gang den brukes; deretter lastes den på en brøkdel av et sekund.

**Lowspeak avslutter rett etter oppstart.** En annen kopi av Lowspeak kjører allerede; avslutt den først.

**Ingen varslinger vises.** Tillat dem under **Systeminnstillinger › Varslinger › Lowspeak**.

**Noe annet.** Loggene i `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) beskriver hva som skjedde, uten den dikterte teksten din. Legg ved de relevante linjene i en [problemrapport](https://github.com/sventmueller/lowspeak/issues/new/choose).

## For utviklere

Bygging, testing, oppbygningen av koden og reglene den følger, er beskrevet i [docs/development.md](../../../docs/development.md) (på engelsk), bidrag i [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Tredjepartskomponenter

| Komponent | Versjon | Brukt til | Lisens |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) via [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | lokal Whisper på GPU-en (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet-dekoding og sammenslåing av lange opptak, oversatt til Rust i `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) og rammeverk-cratene dens | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation og andre systemrammeverk | MIT; Zlib, Apache-2.0 eller MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | mikrofonopptak via CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | omsampling til 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) med rustls | 3.5 | nedlasting av modeller | MIT eller Apache-2.0; rustls og ring under ISC og Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | sjekksummer for nedlastede modeller | MIT eller Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | API-nøkler på nøkkelringen | MIT eller Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, feil, vokabularet til Parakeet | MIT eller Apache-2.0; Apache-2.0 |

Lisenstekstene for alt som er med i appen, følger med den som `Contents/Resources/Acknowledgements.txt`.

Modellene er ikke en del av Lowspeak; appen laster dem ned når du ber om det:

| Modell | Kilde | Lisens |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (OpenAIs Whisper i ggml-format) | MIT |
| Parakeet v2, Parakeet Ultra | CoreML-konverteringer av NVIDIAs Parakeet TDT fra [FluidInference](https://huggingface.co/FluidInference) | CC-BY-4.0 |

## Lisens

Lowspeak er lisensiert under enten

- Apache-lisensen, versjon 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- MIT-lisensen ([LICENSE-MIT](../../../LICENSE-MIT))

etter eget valg.

Med mindre du uttrykkelig sier noe annet, skal ethvert bidrag som du bevisst sender inn for å bli tatt med i Lowspeak, slik det er definert i Apache-2.0-lisensen, ha dobbel lisens som ovenfor, uten ytterligere vilkår.
