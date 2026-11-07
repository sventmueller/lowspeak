<p><img src="../../readme-icon.png" alt="Appicoon van Lowspeak" width="168" height="168"></p>

# Lowspeak

**Dicteren op de Mac, precies waar je cursor staat: houd fn ingedrukt, spreek, laat los.**

Lowspeak woont in de menubalk. Houd de fn-toets (wereldbol) ingedrukt en spreek; zodra je loslaat, verschijnen je woorden in de app waarin je aan het typen bent, alsof je ze zelf had getypt. Het transcribeert online met OpenAI of Groq, of volledig op je Mac met Whisper of NVIDIA Parakeet, dat 40 seconden spraak in ongeveer een derde seconde omzet in tekst. Valt het netwerk of de onlinedienst uit, dan neemt een geïnstalleerd lokaal model het over, zodat er geen dictaat verloren gaat. Leer het de namen en termen die je gebruikt, laat desgewenst een taalmodel stopwoorden verwijderen en zet een opname vast met Shift om langere teksten handsfree te dicteren. Het is geschreven in Rust op echte AppKit-besturingselementen, spreekt 35 talen en stuurt je stem alleen naar de aanbieder die je hebt gekozen.

[![Download Lowspeak voor macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Download Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: ondertekend en door Apple genotariseerd, voor macOS 14 of nieuwer op Apple Silicon en Intel.

**Een bug gevonden?** [Open een issue](https://github.com/sventmueller/lowspeak/issues/new/choose) of kies **Help › Meld een probleem…** in het menu van Lowspeak: het formulier opent met je versie al ingevuld. Heb je iets aan Lowspeak, dan helpt een [ster op GitHub](https://github.com/sventmueller/lowspeak) anderen het te vinden.

[![Rust-editie 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 of nieuwer](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universeel: Apple Silicon en Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit via objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp op Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet op de Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online met OpenAI of Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 interfacetalen](https://img.shields.io/badge/languages-35-4C9A2A)
![Ondertekend met Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licentie: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licentie)

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
**Nederlands** ·
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

## Inhoud

- [Functies in het kort](#functies-in-het-kort)
- [Systeemvereisten](#systeemvereisten)
- [Installatie](#installatie)
- [Aan de slag](#aan-de-slag)
  - [Toestemmingen](#toestemmingen)
  - [Kies hoe je transcribeert](#kies-hoe-je-transcribeert)
  - [Het menu](#het-menu)
  - [Beknopt overzicht](#beknopt-overzicht)
- [Dicteren](#dicteren)
  - [Ingedrukt houden om te praten](#ingedrukt-houden-om-te-praten)
  - [Vastzetten met Shift of dubbel drukken](#vastzetten-met-shift-of-dubbel-drukken)
  - [Waar de tekst terechtkomt](#waar-de-tekst-terechtkomt)
  - [Recente dictaten](#recente-dictaten)
- [Transcriptie](#transcriptie)
  - [Welk model bij je past](#welk-model-bij-je-past)
  - [Online: OpenAI en Groq](#online-openai-en-groq)
  - [Op je Mac: Whisper en Parakeet](#op-je-mac-whisper-en-parakeet)
  - [Als de onlinedienst uitvalt](#als-de-onlinedienst-uitvalt)
  - [Hoe de opname wordt voorbereid](#hoe-de-opname-wordt-voorbereid)
  - [Talen per model](#talen-per-model)
  - [Intel-Macs](#intel-macs)
- [Woordenlijst](#woordenlijst)
- [Nabewerking](#nabewerking)
- [Microfoon](#microfoon)
- [API-sleutels](#api-sleutels)
- [Meldingen](#meldingen)
- [Uiterlijk en toegankelijkheid](#uiterlijk-en-toegankelijkheid)
- [Talen](#talen)
- [Toetsen en gebaren](#toetsen-en-gebaren)
- [Privacy en beveiliging](#privacy-en-beveiliging)
- [Snelheid](#snelheid)
- [Bekende beperkingen](#bekende-beperkingen)
- [Problemen oplossen](#problemen-oplossen)
- [Voor ontwikkelaars](#voor-ontwikkelaars)
  - [Componenten van derden](#componenten-van-derden)
- [Licentie](#licentie)

## Functies in het kort

- **Houd fn ingedrukt, spreek, laat los.** De tekst wordt getypt in de app op de voorgrond, waar de cursor ook staat: mail, chat, editor, terminal of browser. Geen venster om naar over te schakelen, geen knop om op te klikken.
- **Zet hem vast voor langere teksten.** Druk op Shift terwijl je fn ingedrukt houdt, of druk twee keer op fn, en de opname loopt door zonder dat je een toets vasthoudt, tot vijf minuten lang. Eén druk op fn of Shift beëindigt hem.
- **Het juiste model voor elke klus.** Groq voor snelheid zonder het geheugen van je Mac te gebruiken, GPT Transcribe van OpenAI voor het beste resultaat, Whisper of NVIDIA Parakeet op je Mac om offline te werken en je stem privé te houden. Wissel met één klik.
- **Geen dictaat verloren.** Valt het netwerk of de onlinedienst uit, dan transcribeert een geïnstalleerd lokaal model dezelfde opname op je Mac.
- **Je woorden, goed gespeld.** Een woordenlijst met namen en vaktermen stuurt OpenAI, Groq, Whisper en de nabewerking.
- **Schone tekst als je dat wilt.** De optionele nabewerking verwijdert stopwoorden, verbetert de grammatica en houdt alleen je correctie over als je jezelf verbetert, of maakt de tekst bovendien op voor het soort app waarin je dicteert.
- **Blijf gewoon praten.** Begin aan het volgende dictaat terwijl het vorige nog wordt getranscribeerd; de teksten komen aan in de volgorde waarin je ze uitsprak.
- **Je klembord blijft van jou.** Lowspeak plakt via het klembord en zet een moment later terug wat erop stond. De laatste tien dictaten blijven in het menu staan, zodat je ze opnieuw kunt kopiëren.
- **Door en door native.** Een echt menubalkonderdeel met een AppKit-menu, standaardwaarschuwingen en -meldingen. Geen Electron, geen venster, geen Dock-symbool.
- **Privé.** Geen account, geen telemetrie. Audio gaat alleen naar de aanbieder die je hebt gekozen, of in de lokale modus nergens heen.
- **In jouw taal.** Dicteren werkt in tientallen talen, en het menu, de dialoogvensters en de meldingen zijn er in 35.

## Systeemvereisten

- macOS 14 Sonoma of nieuwer. Lowspeak wordt ontwikkeld en getest op macOS 27.
- Een Mac met Apple Silicon of een Intel-processor. Lokale transcriptie is gemaakt voor Apple Silicon, waar Whisper op de GPU draait en Parakeet op de Neural Engine; op Intel-Macs draaien beide op de processor, veel langzamer, zie [Intel-Macs](#intel-macs).
- Ongeveer 17 MB voor de app, plus 75 MB tot 3 GB voor elk lokaal model dat je downloadt.
- Voor online transcriptie een API-sleutel van [OpenAI](https://platform.openai.com/api-keys) of [Groq](https://console.groq.com/keys); beide rekenen af voor wat je transcribeert. Lokale transcriptie heeft geen account nodig.

## Installatie

**Vanaf de schijfkopie.** Download [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), open haar en sleep Lowspeak naar de map Programma's. De schijfkopie is ondertekend met een Developer ID en door Apple genotariseerd, dus ze opent zonder waarschuwingen.

**Updates.** Lowspeak vanaf de schijfkopie houdt zichzelf up-to-date met [Sparkle](https://sparkle-project.org). Bij de tweede start vraagt het of het zelf mag zoeken naar updates, eenmaal per dag. **Zoek naar updates…** in het menu zoekt meteen; met **Zoek automatisch naar updates** en **Download en installeer updates automatisch** eronder wijzig je je keuze later. Vindt een zoekactie een nieuwe versie terwijl je in een andere app werkt, dan verschijnt er een stip op de ring in de menubalk, en **Zoek naar updates…** haalt de update naar de voorgrond. Lowspeak installeert een update alleen als die de handtekening van de ontwikkelaar draagt; je instellingen, sleutels en modellen blijven behouden. Versie 1.0.0 heeft geen updater: installeer er één keer een nieuwere versie overheen, van de [releasepagina](https://github.com/sventmueller/lowspeak/releases). Een eigen build heeft ook geen updater.

**Vanaf de broncode.** Zie [Bouwen en installeren](../../../docs/development.md#build--deploy) voor de vereisten en voer dan uit:

```sh
make deploy
```

Dit bouwt Lowspeak, vervangt de kopie in je map Programma's en start het.

**Open bij inloggen** in het menu start Lowspeak samen met je Mac, via de eigen inlogonderdelen van macOS. Er draait altijd maar één exemplaar; een tweede stopt meteen.

## Aan de slag

### Toestemmingen

Bij de eerste start vraagt Lowspeak om drie toestemmingen:

- **Toegankelijkheid** (Systeeminstellingen › Privacy en beveiliging › Toegankelijkheid; op macOS 27 heet de lijst Apparaatbeheer en gegevenstoegang). Lowspeak heeft deze nodig om de fn-toets op te merken en met ⌘V te plakken. macOS biedt aan de lijst te openen; zet daar de schakelaar voor Lowspeak aan. De meldingen van Lowspeak noemen de lijst zoals je macOS hem toont.
- **Microfoon.** Wordt één keer gevraagd; zonder deze toestemming neemt Lowspeak niets op.
- **Meldingen.** Lowspeak gebruikt ze voor problemen die je anders niet zou zien, zoals een stille microfoon of een mislukte transcriptie.

### Kies hoe je transcribeert

Stel minstens één manier in; je kunt op elk moment wisselen in het menu.

- **Online:** kies **Stel API-sleutel in…** en plak een sleutel van OpenAI (die begint met `sk-`) of Groq (`gsk_`). Lowspeak herkent de aanbieder aan de sleutel. Kies daarna een model onder **OpenAI** of **Groq**.
- **Op je Mac:** kies **Beheer modellen** › **Download Parakeet Ultra** of een Whisper-model en kies het daarna onder **Lokaal**.

[Welk model bij je past](#welk-model-bij-je-past) helpt bij de keuze. Houd dan fn ingedrukt, zeg iets en laat los.

### Het menu

Klik op de ring in de menubalk:

| Onderdeel | Wat het doet |
|---|---|
| **Recente dictaten** | de laatste tien teksten; een klik kopieert er een opnieuw naar het klembord |
| **Lokaal** | de modellen op je Mac, gegroepeerd in Whisper en Parakeet; modellen die niet zijn gedownload, zijn grijs weergegeven |
| **OpenAI**, **Groq** | de onlinemodellen; zonder sleutel zijn ze grijs weergegeven en biedt het menu **Stel API-sleutel in…** aan |
| **Taal** | de taal waarin je dicteert, of **Automatisch herkennen**; het menu volgt die taal |
| **Nabewerking** | **Uit**, **Opschonen** of **Opmaken**, zie [Nabewerking](#nabewerking) |
| **Woordenlijst…** | namen en termen die precies zo gespeld moeten worden; het menu toont hoeveel je er hebt ingevoerd |
| **Microfoon** | **Systeemstandaard** of een vaste microfoon, en **Speel geluid af zodra gereed** |
| **Beheer modellen** | lokale modellen downloaden en verwijderen, met voortgang |
| **Stel API-sleutel in…** / **Wijzig API-sleutel…** | sleutels toevoegen, vervangen of verwijderen |
| **Open bij inloggen** | Lowspeak met je Mac starten; het vinkje geeft aan dat het aan staat |
| **Zoek naar updates…** | nu naar een nieuwe versie zoeken; de twee onderdelen eronder bepalen of Lowspeak eenmaal per dag zoekt en updates zonder te vragen installeert. Alleen in de app van de schijfkopie |
| **Help** | **Beknopt overzicht**, **Meld een probleem…**, **Stel een functie voor…**, **Lowspeak op GitHub** |
| **Over Lowspeak** | versie, copyright, projectpagina en licentie |
| **Stop Lowspeak** | ⌘Q terwijl het menu open is |

Het vinkje onder **Lokaal**, **OpenAI** en **Groq** markeert het model dat in gebruik is. Het symbool laat zien wat Lowspeak aan het doen is:

| Symbool | Betekenis |
|---|---|
| Ring met een stip | Gereed |
| Gevulde cirkel | Opname terwijl fn ingedrukt is |
| Afgerond vierkant | Opname vastgezet; de volgende druk op fn of Shift beëindigt hem |
| Knipperende ring | Bezig met transcriberen |
| Ring met een stip rechtsboven | Er wacht een nieuwe versie, zie **Zoek naar updates…** |
| ↓ 42% | Een model wordt gedownload |

Lowspeak heeft geen instellingenvenster: alles staat in dit menu, en het onthoudt elke keuze.

### Beknopt overzicht

**Help › Beknopt overzicht** toont de fn-gebaren in een klein venster, in jouw taal, met een knop die de toetsenbordinstellingen opent. **Meld een probleem…** opent een formulier op GitHub waarin je versies van Lowspeak en macOS al zijn ingevuld, **Stel een functie voor…** een discussie voor ideeën, **Lowspeak op GitHub** de projectpagina.

## Dicteren

### Ingedrukt houden om te praten

Houd fn ingedrukt, spreek, laat los. Het opnamesymbool verschijnt op het moment dat je microfoon geluid levert; begin dan te spreken, want wat je daarvoor zegt, wordt niet opgenomen. Loslaten van fn beëindigt de opname, en de tekst volgt even later.

Een korte tik op fn doet niets, zodat fn zijn andere functies houdt. Een opname die leeg blijft of korter dan 0,3 seconden duurt, wordt weggegooid.

### Vastzetten met Shift of dubbel drukken

Zet de opname voor langere teksten vast, zodat hij doorloopt zonder dat je een toets vasthoudt. Dat kan op drie manieren:

- **Shift terwijl je fn ingedrukt houdt.** Begin zoals gewoonlijk met fn en druk tijdens de opname op Shift: de opname wordt vastgezet en je kunt fn loslaten. Niets van wat je al zei gaat verloren, de opname loopt gewoon door.
- **Shift, dan fn.** Houd Shift ingedrukt en druk op fn: de opname begint vastgezet.
- **Twee keer fn.** Tik kort op fn (korter dan 0,3 seconden) en druk binnen 0,4 seconden opnieuw. De opname loopt vanaf de eerste druk.

Het symbool verandert in een afgerond vierkant. Druk één keer op **fn** of **Shift** om te stoppen; de tekst volgt zoals gewoonlijk. Is je 🌐-toets ingesteld om Emoji's en symbolen te openen, stop dan met Shift: een korte druk op fn zou dat venster openen.

Een opname stopt vanzelf na twee minuten als je fn ingedrukt houdt en na vijf minuten als hij vastgezet is, en wordt daarna zoals gewoonlijk getranscribeerd.

### Waar de tekst terechtkomt

Lowspeak plakt de tekst in de app die op de voorgrond stond toen je stopte met spreken, bij de cursor, via het klembord en ⌘V. Het bewaart alles wat eerder op het klembord stond en zet het terug een halve seconde nadat de app de tekst heeft gelezen, of na tien seconden als de app dat nooit doet, tenzij een andere app het klembord in de tussentijd heeft gewijzigd. De tekst is gemarkeerd als tijdelijk, zodat apps voor klembordgeschiedenis die deze markering respecteren hem niet vastleggen.

Je kunt het volgende dictaat beginnen terwijl het vorige nog wordt getranscribeerd. De teksten worden geplakt in de volgorde waarin je ze uitsprak, en nooit terwijl je opneemt of Shift ingedrukt houdt, zodat er niets midden in je volgende zin geplakt kan worden.

Zonder de toestemming voor toegankelijkheid, of als de gesimuleerde ⌘V niet kan worden verstuurd, blijft de tekst op het klembord staan en laat een melding je dat weten; plak hem dan zelf met ⌘V.

### Recente dictaten

**Recente dictaten** in het menu toont de laatste tien teksten, de nieuwste eerst, elk ingekort tot één regel; houd je de aanwijzer erboven, dan zie je de hele tekst. Een klik kopieert hem naar het klembord, voor als een tekst op de verkeerde plek terechtkwam of je hem twee keer nodig hebt. De lijst staat alleen in het geheugen en is leeg nadat Lowspeak is gestopt.

## Transcriptie

### Welk model bij je past

| Je wilt | Kies | Waarom |
|---|---|---|
| **Snelheid zonder je Mac te belasten** | **Groq › Whisper Large V3 Turbo** | de snelste onlineoptie; er wordt niets gedownload en je Mac houdt zijn geheugen en schijfruimte |
| **Het beste resultaat** | **OpenAI › GPT Transcribe** (standaard) | het nauwkeurigst van de modellen die Lowspeak biedt; neemt je woordenlijst als trefwoorden |
| **Offline en privé** | **Lokaal › Parakeet Ultra** voor Europese talen, **Lokaal › Whisper Large v3 Turbo** voor alle andere | werkt zonder netwerk en je stem verlaat je Mac nooit; Parakeet is het snelst van allemaal |
| **Geen account, geen kosten** | elk model onder **Lokaal** | draait op je Mac; er wordt niets in rekening gebracht |
| **Betrouwbaarheid én snelheid** | een onlinemodel, plus Parakeet Ultra of een Whisper-model geïnstalleerd | het lokale model springt in als de onlinedienst uitvalt |
| **Dicteren op een Intel-Mac** | **Groq** of **OpenAI** | lokale modellen draaien daar op de processor, zie [Intel-Macs](#intel-macs) |

Lokale modellen nemen hun downloadgrootte in beslag op de schijf en, zolang ze gekozen zijn, ongeveer evenveel geheugen; Lowspeak houdt het model geladen, zodat dicteren zonder vertraging begint. Kies op een Mac met 8 GB geheugen liever Parakeet Ultra of een onlinemodel dan de grote Whisper-modellen.

### Online: OpenAI en Groq

| Model | Aanbieder | Opmerkingen |
|---|---|---|
| **GPT Transcribe** (standaard) | OpenAI | het nauwkeurigst; neemt je woordenlijst als trefwoorden |
| **Whisper** | OpenAI | het oorspronkelijke Whisper-model |
| **Whisper Large V3** | Groq | zeer snel |
| **Whisper Large V3 Turbo** | Groq | snelst online |

De opname wordt vóór het uploaden gecomprimeerd naar AAC, ongeveer 6 KB per seconde spraak, zodat dicteren ook over trage verbindingen werkt. Terwijl je fn ingedrukt houdt, opent Lowspeak al de verbinding met de aanbieder, zodat het uploaden begint op het moment dat je loslaat.

### Op je Mac: Whisper en Parakeet

| Model | Grootte | Draait op | Talen |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 Europese talen |
| **Parakeet v2** | ~600 MB | Neural Engine | Engels |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | ongeveer 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | ongeveer 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | ongeveer 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | ongeveer 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | ongeveer 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | ongeveer 99 |

**Beheer modellen** downloadt en verwijdert ze; tijdens een download toont de menubalk de voortgang, en als je Lowspeak stopt, wordt de download afgebroken. Het allereerste gebruik van een model bereidt het voor op je Mac: ongeveer 10 seconden voor Parakeet Ultra, tot 30 seconden voor Whisper; daarna laadt het in een fractie van een seconde. Wordt het gekozen model verwijderd, dan schakelt Lowspeak over naar een ander geïnstalleerd model.

**Parakeet** is de snelste keuze: het transcribeert 40 seconden spraak in ongeveer 0,3 seconden. Lowspeak draait het met dezelfde modellen en decodering als [FluidAudio](https://github.com/FluidInference/FluidAudio) en bewaart de modellen in de map van FluidAudio (`~/Library/Application Support/FluidAudio/Models`), zodat apps die op FluidAudio zijn gebouwd ze delen. Opnamen langer dan 15 seconden worden getranscribeerd in overlappende vensters die worden samengevoegd zonder woorden te verliezen op de naden. **Whisper** begrijpt meer talen en leest je woordenlijst; de grote modellen zijn het nauwkeurigst, de kleine zijn buiten het Engels veel zwakker. Whisper-modellen worden bewaard in `~/Library/Application Support/Lowspeak/Models`.

### Als de onlinedienst uitvalt

Mislukt een online transcriptie omdat het netwerk plat ligt, de aanbieder een probleem heeft of de sleutel onjuist is, dan transcribeert een geïnstalleerd lokaal model dezelfde opname: het model dat onder **Lokaal** is gekozen, anders Parakeet Ultra, dan Whisper Large v3 Turbo, Large v3 en de kleinere Whisper-modellen, met Parakeet v2 als laatste. Een melding laat het je één keer weten; die komt pas terug nadat er tussendoor een online dictaat is gelukt.

Met een lokaal model als terugvaloptie geeft Lowspeak het onlineverzoek 20 seconden; zonder probeert het drie keer, telkens tot een minuut. In de onlinemodus bereidt Lowspeak het Parakeet-model voor de terugval één keer voor bij het starten, zodat de eerste terugval er niet op hoeft te wachten.

### Hoe de opname wordt voorbereid

Lowspeak neemt op met de eigen samplefrequentie van je microfoon en zet na het stoppen om naar de 16 kHz die de modellen verwachten. Vóór de transcriptie verwijdert het laag gerommel onder 80 Hz, knipt het stilte aan het begin en einde weg en egaliseert het het volume, zodat zachte en luide sprekers er gelijk uitkomen. Een opname zonder spraak wordt helemaal niet verstuurd. In de lokale modus krijgt Parakeet eerst de onbewerkte opname, en de voorbereide alleen als dat geen tekst oplevert.

### Talen per model

De taal onder **Taal** wordt doorgegeven aan de onlinemodellen en aan Whisper. Met **Automatisch herkennen** bepaalt het model de taal zelf.

| Model | Talen die je kunt dicteren |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, op je Mac) | ongeveer 99, waaronder alle 35 uit het menu |
| Parakeet Ultra | Bulgaars, Kroatisch, Tsjechisch, Deens, Nederlands, Engels, Ests, Fins, Frans, Duits, Grieks, Hongaars, Italiaans, Lets, Litouws, Maltees, Pools, Portugees, Roemeens, Russisch, Slowaaks, Sloveens, Spaans, Zweeds, Oekraïens |
| Parakeet v2 | Engels |

Parakeet herkent de taal zelf en negeert de keuze in het menu. **Chinees (traditioneel)** laat GPT Transcribe traditionele karakters schrijven; Whisper schrijft Chinees in beide schriften en Portugees zonder rekening te houden met de variant.

### Intel-Macs

Lowspeak is een universele app en draait ook op Intel-Macs met macOS 14. Alles wat online gebeurt, werkt hetzelfde. De lokale modellen hebben daar niet de hardware waarvoor ze zijn gebouwd: Whisper draait op de processor in plaats van op de GPU, en Parakeet op de processor in plaats van op de Neural Engine. Beide werken, maar doen er vele malen langer over dan op Apple Silicon, de grote Whisper-modellen veel te lang om mee te dicteren. Transcribeer op een Intel-Mac online en houd Whisper Small of Parakeet Ultra achter de hand als [terugvaloptie](#als-de-onlinedienst-uitvalt).

## Woordenlijst

**Woordenlijst…** neemt de namen en termen op die Lowspeak precies zo moet spellen, gescheiden door komma's: productnamen, collega's, vaktermen. Tot 100 termen van elk maximaal 60 tekens; `<` en `>` worden verwijderd.

- **GPT Transcribe** krijgt ze als trefwoorden.
- **Whisper** (OpenAI, Groq en op je Mac) leest ze als hint vóór de opname, tot 500 tekens.
- **Nabewerking** krijgt de opdracht ze precies zo te spellen als ze geschreven zijn.
- **Parakeet** gebruikt ze niet.

## Nabewerking

Na de transcriptie kan een taalmodel de tekst herwerken. Kies onder **Nabewerking**:

- **Uit** (standaard) – de tekst zoals hij is getranscribeerd.
- **Opschonen** – verwijdert stopwoorden zoals "eh", "weet je" of "eigenlijk", verbetert grammatica en interpunctie, en houdt alleen de correctie over als je jezelf verbetert. Het voegt niets toe en verandert de betekenis niet.
- **Opmaken** – zoals Opschonen, en past de tekst bovendien aan de app aan waarin je dicteert: termen in codestijl in code-editors, opdrachtregels in terminals, een informele toon in chatapps, een zakelijke in e-mail.

De nabewerking gebruikt Groq (`openai/gpt-oss-120b`) als je een Groq-sleutel hebt, anders OpenAI (`gpt-4o-mini`), ook als je lokaal transcribeert. Is er onder **Taal** een taal gekozen, dan houdt ze die taal aan. Duurt ze langer dan acht seconden, mislukt ze of geeft ze een antwoord dat is afgekapt of veel langer is dan wat je zei, dan plakt Lowspeak de onbewerkte tekst. Weigert de aanbieder het verzoek of is er geen sleutel ingesteld, dan laat een melding dat één keer weten.

## Microfoon

Kies onder **Microfoon** een vaste microfoon of **Systeemstandaard**. Lowspeak onthoudt het apparaat; zolang het niet is aangesloten, toont het menu het als **(niet verbonden)** en neemt Lowspeak op met de systeemstandaard.

De microfoon wordt alleen ingeschakeld terwijl je dicteert. Hem gereedmaken duurt ongeveer 0,07 seconden met de ingebouwde microfoon, ongeveer een halve seconde met een Studio Display en een tot twee seconden met AirPods, die eerst naar hun headsetmodus overschakelen. Het opnamesymbool verschijnt zodra er geluid binnenkomt; **Speel geluid af zodra gereed** speelt op dat moment ook een kort geluid af. Het staat standaard uit, omdat het geluid op sommige luidsprekers zelf een halve seconde te laat komt.

Levert een microfoon vijf seconden lang alleen stilte, zoals AirPods doen wanneer een ander apparaat hun microfoon bezet houdt, dan laat Lowspeak het je weten.

## API-sleutels

**Stel API-sleutel in…** vraagt om een sleutel en herkent de aanbieder aan het begin ervan: `sk-` voor OpenAI, `gsk_` voor Groq. Een sleutel in een andere indeling wordt met een uitleg geweigerd. Voer één sleutel per aanbieder in; met sleutels voor beide zijn alle onlinemodellen beschikbaar. **Wijzig API-sleutel…** vervangt een sleutel of verwijdert ze met **Verwijder alle sleutels**.

De sleutels worden bewaard in je inlogsleutelhanger onder `com.lowspeak.app`. Een sleutelbestand van een eerdere versie (`~/Library/Application Support/Lowspeak/.api-key`) wordt bij de volgende start daarheen verplaatst en verwijderd zodra elke sleutel weer uit de sleutelhanger kan worden gelezen.

## Meldingen

Lowspeak werkt op de achtergrond, dus meldt het problemen als meldingen:

| Melding | Wat er gebeurde |
|---|---|
| **Opname mislukt** | geen toegang tot de microfoon, geen microfoon aangesloten, of de microfoon kon niet starten |
| **Geen geluid van microfoon** | vijf seconden stilte, zie [Microfoon](#microfoon) |
| **Toestemming voor toegankelijkheid vereist** | de tekst staat op het klembord; geef toestemming voor toegankelijkheid om automatisch te plakken |
| **Plakken mislukt** | de tekst staat op het klembord; plak hem met ⌘V |
| **Transcriptie mislukt** | met de reden: geen sleutel, geweigerde sleutel, limiet voor verzoeken, server- of netwerkfout, of geen lokaal model |
| **Getranscribeerd op deze Mac** | de onlinedienst viel uit en een lokaal model nam het over |
| **Nabewerking mislukt** | de aanbieder weigert de nabewerking of er is geen sleutel ingesteld; de tekst is zonder nabewerking geplakt |
| **Geen API-sleutel** | je hebt een onlinemodel gekozen zonder sleutel voor de aanbieder ervan |
| **Download mislukt**, **Verwijderen mislukt**, **Kan niet verwijderen** | het beheren van een lokaal model lukte niet, of er liep nog een andere bewerking |
| **Fout bij inlogonderdeel**, **Fout bij API-sleutel** | macOS of de sleutelhanger weigerde de wijziging |

Meldingen bevatten nooit je gedicteerde tekst. Een falende onlinedienst en een aanbieder die de nabewerking weigert, worden één keer gemeld, niet na elk dictaat, totdat ze weer werken.

## Uiterlijk en toegankelijkheid

- **Het menubalksymbool** is een sjabloonafbeelding die zich aanpast aan een lichte, donkere of getinte menubalk.
- **VoiceOver** leest het symbool voor als "Lowspeak", en het menu, de waarschuwingen en de meldingen zijn standaardelementen van macOS.
- **Er hoeft geen toets ingedrukt te blijven.** Zet een opname vast met Shift of door twee keer op fn te drukken, en de volgende druk beëindigt hem.
- **Het beknopte overzicht** is een eenvoudig venster met labels en een knop, bereikbaar met het toetsenbord; Esc sluit het.

Bekende tekortkomingen staan in [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver kondigt niet aan wanneer een opname begint, en Lowspeak is nog niet getest met VoiceOver of Schakelbediening.

## Talen

Het menu, de dialoogvensters en de meldingen zijn vertaald in 35 talen:

Arabisch, Catalaans, Chinees (vereenvoudigd), Chinees (traditioneel), Kroatisch, Tsjechisch, Deens, Nederlands, Engels, Fins, Frans, Duits, Grieks, Hebreeuws, Hindi, Hongaars, Indonesisch, Italiaans, Japans, Koreaans, Maleis, Noors, Pools, Portugees (Brazilië), Portugees (Portugal), Roemeens, Russisch, Slowaaks, Sloveens, Spaans, Zweeds, Thai, Turks, Oekraïens en Vietnamees.

De taal die je onder **Taal** kiest, is ook de taal van het menu. Met **Automatisch herkennen** volgt het menu de taalvolgorde in **Systeeminstellingen › Algemeen › Taal en regio**, en Engels waar geen vertaling bestaat. Menutermen volgen de eigen vertalingen van Apple, zodat Lowspeak leest als de rest van je Mac. Behalve het Engels zijn de vertalingen machinaal gemaakt en nog niet nagekeken door moedertaalsprekers; correcties zijn welkom als [issues](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Toetsen en gebaren

| Actie | Toetsen |
|---|---|
| Dicteren zolang ingedrukt | houd fn ingedrukt |
| Een lopende opname vastzetten | Shift terwijl je fn ingedrukt houdt |
| Een vastgezette opname starten | Shift+fn, of snel twee keer fn |
| Een vastgezette opname beëindigen | fn of Shift |
| Lowspeak stoppen (menu open) | ⌘Q |
| Het beknopte overzicht sluiten | Esc |

Lowspeak houdt de fn-drukken die het gebruikt weg bij andere apps; Shift gaat gewoon door.

## Privacy en beveiliging

Lowspeak hoort wat je zegt terwijl fn ingedrukt is, kan in elke app typen en bewaart je API-sleutels, dus doet het er zo weinig mogelijk mee.

- **Je stem gaat naar één plek.** In de onlinemodus gaat de opname, gecomprimeerd, naar de aanbieder die je hebt gekozen, OpenAI of Groq, samen met je woordenlijst. In de lokale modus verlaat niets je Mac. Mislukt een onlineverzoek, dan transcribeert een lokaal model dezelfde opname op je Mac.
- **Nabewerking** stuurt, als je die inschakelt, de getranscribeerde tekst en je woordenlijst naar Groq of OpenAI. In de modus Opmaken voegt ze het soort app toe waarin je dicteert (code-editor, terminal, chat of e-mail); de naam van de app wordt niet verstuurd.
- **Geen telemetrie, geen account.** Naast transcriptie en nabewerking gaat Lowspeak alleen online om de verbinding met je aanbieder te openen terwijl fn ingedrukt is, in de onlinemodus of met nabewerking aan, om een model te downloaden als je daarom vraagt, van Hugging Face via HTTPS, in een vaste versie, waarbij elk bestand wordt gecontroleerd aan de hand van zijn checksum, en om op GitHub naar een nieuwe versie te zoeken, eenmaal per dag als je dat toestaat. Dat verzoek bevat de versie van Lowspeak en verder niets. **Help › Meld een probleem…** opent een formulier op GitHub in je browser; het adres bevat de versie van Lowspeak en de macOS-versie, niets van wat je hebt gedicteerd.
- **API-sleutels** blijven in je inlogsleutelhanger. Ze worden alleen naar hun eigen aanbieder gestuurd en uit foutmeldingen verwijderd voordat er iets naar het logbestand wordt geschreven.
- **Gedicteerde tekst wordt niet opgeslagen.** De laatste tien dictaten blijven in het geheugen voor **Recente dictaten** en zijn weg als Lowspeak stopt. De logbestanden leggen lengtes en tijden vast, nooit de tekst.
- **Het klembord** wordt vóór het plakken bewaard en hersteld zodra de app de tekst heeft gelezen, tenzij een andere app het in de tussentijd heeft gewijzigd. Het dictaat is gemarkeerd als tijdelijk, zodat apps voor klembordgeschiedenis die deze markering respecteren het overslaan.
- **Codeondertekening** met de Hardened Runtime; de app vraagt alleen om de microfoon, toegankelijkheid, meldingen en het netwerk. Een update moet de handtekening van de ontwikkelaar dragen, anders weigert Sparkle hem.

Volg het [beveiligingsbeleid](../../../SECURITY.md) om een kwetsbaarheid te melden.

## Snelheid

Gemeten op Apple Silicon:

- **Parakeet Ultra** transcribeert 40 seconden spraak in ongeveer 0,3 seconden.
- **Online:** het eerste verzoek na een pauze duurt bij Groq ongeveer 0,4 seconden, omdat Lowspeak de verbinding opent terwijl je nog spreekt; latere verzoeken hergebruiken die, via HTTP/3 waar de aanbieder dat biedt.
- **Uploadgrootte:** ongeveer 6 KB per seconde spraak, als AAC met 48 kbit/s.
- **De microfoon gereedmaken:** ongeveer 0,07 seconden ingebouwd, een halve seconde op een Studio Display, een tot twee seconden met AirPods.

## Bekende beperkingen

- **Lokale modellen op Intel-Macs** draaien op de processor en zijn veel langzamer; online transcriptie is daar de betere keuze.
- **Wat je zegt voordat de microfoon gereed is, gaat verloren.** Wacht op het opnamesymbool, vooral met AirPods.
- **De fn-toets kan door anderen worden ingepikt.** Als de 🌐-toetsinstelling van macOS, Warp of de dicteerfunctie van macOS fn als eerste claimt, ziet Lowspeak hem niet; zie [Problemen oplossen](#problemen-oplossen).
- **Parakeet** negeert de woordenlijst en de taalkeuze, en kent alleen Europese talen.
- **Schrift en variant:** alleen GPT Transcribe kan worden opgedragen traditioneel Chinees te schrijven; geen enkel model kan worden opgedragen Europees in plaats van Braziliaans Portugees te schrijven.
- **Tijdslimieten:** twee minuten als je fn ingedrukt houdt, vijf minuten als de opname vastgezet is.
- **Microfoonmodus:** Bedieningspaneel biedt Stemisolatie en Breed spectrum aan voor Lowspeak en meldt ze als actief, maar Lowspeak neemt op zonder stemverwerking, dus de keuze heeft geen effect.
- **Recente dictaten** staan alleen in het geheugen en zijn weg na het stoppen.
- **Bewust zo:** geen instellingenvenster, geen Dock-symbool, niet in de App Store.

## Problemen oplossen

**Er gebeurt niets als ik fn ingedrukt houd.** Controleer drie dingen: Lowspeak is toegestaan onder **Systeeminstellingen › Privacy en beveiliging › Toegankelijkheid** (op macOS 27: **Apparaatbeheer en gegevenstoegang**); **Systeeminstellingen › Toetsenbord › Druk op 🌐-toets:** staat op **Doe niets**; en geen andere app claimt fn. Warp doet dat standaard; zet het uit met `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` en start Warp opnieuw. Ook de dicteerfunctie van macOS kan zo zijn ingesteld dat ze met fn start.

**Na een update reageert Lowspeak niet meer op fn.** macOS koppelt de toestemming voor toegankelijkheid aan de handtekening van de app. Is de handtekening veranderd, bijvoorbeeld van een eigen build naar de gedownloade app, verwijder Lowspeak dan uit de lijst bij Toegankelijkheid (op macOS 27: Apparaatbeheer en gegevenstoegang) en voeg het opnieuw toe.

**De eerste woorden ontbreken.** Begin te spreken zodra het opnamesymbool verschijnt. Met AirPods heeft de microfoon een tot twee seconden nodig; een ingebouwde of USB-microfoon onder **Microfoon** is vrijwel meteen gereed.

**"Geen geluid van microfoon."** De microfoon heeft vijf seconden lang geen geluid geleverd. Bij AirPods gebruikt misschien een ander apparaat hun microfoon; schakel over naar de ingebouwde microfoon of verbind ze opnieuw. Een gesloten MacBook-deksel dempt ook de ingebouwde microfoon.

**Dictaten komen leeg terug.** Het invoervolume is misschien te laag: apps zoals Teams, Zoom of Meet wijzigen het tijdens gesprekken, en macOS bewaart het per apparaat. Verhoog het onder **Systeeminstellingen › Geluid › Invoer**.

**De tekst kwam op de verkeerde plek terecht.** **Recente dictaten** in het menu kopieert hem opnieuw naar het klembord.

**"Geen API-sleutel."** Kies **Stel API-sleutel in…** in het menu, of schakel over naar een lokaal model.

**De eerste lokale transcriptie duurt lang.** Elk model wordt bij het eerste gebruik voorbereid op je Mac; daarna laadt het in een fractie van een seconde.

**Lowspeak stopt meteen na het starten.** Er draait al een ander exemplaar van Lowspeak; stop dat eerst.

**Er verschijnen geen meldingen.** Sta ze toe onder **Systeeminstellingen › Meldingen › Lowspeak**.

**Iets anders.** De logbestanden in `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) beschrijven wat er gebeurde, zonder je gedicteerde tekst. Voeg de relevante regels toe aan een [probleemmelding](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Voor ontwikkelaars

Bouwen, testen, de opbouw van de code en de regels die hij volgt staan in [docs/development.md](../../../docs/development.md); hoe je kunt bijdragen staat in [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Componenten van derden

| Component | Versie | Gebruikt voor | Licentie |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) via [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | lokale Whisper op de GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet-decodering en het samenvoegen van lange opnamen, vertaald naar Rust in `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) en zijn framework-crates | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation en andere systeemframeworks | MIT; Zlib, Apache-2.0 of MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | microfoonopname via CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | omzetten naar 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) met rustls | 3.5 | modeldownloads | MIT of Apache-2.0; rustls en ring onder ISC en Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | checksums van de modeldownloads | MIT of Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | API-sleutels in de sleutelhanger | MIT of Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, fouten, het vocabulaire van Parakeet | MIT of Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | updates van de app van de schijfkopie | MIT, met onderdelen onder BSD- en zlib-licenties |

De licentieteksten van alles in de app worden meegeleverd als `Contents/Resources/Acknowledgements.txt`.

De modellen maken geen deel uit van Lowspeak; het downloadt ze als je daarom vraagt:

| Model | Bron | Licentie |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper van OpenAI in ggml-indeling) | MIT |
| Parakeet v2, Parakeet Ultra | CoreML-conversies van NVIDIA's Parakeet TDT door [FluidInference](https://huggingface.co/FluidInference) | CC-BY-4.0 |

## Licentie

Lowspeak is gelicenseerd onder een van de volgende twee licenties

- Apache-licentie, versie 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- MIT-licentie ([LICENSE-MIT](../../../LICENSE-MIT))

naar jouw keuze.

Tenzij je uitdrukkelijk anders aangeeft, wordt elke bijdrage die je opzettelijk indient voor opname in Lowspeak, zoals gedefinieerd in de Apache-2.0-licentie, onder dubbele licentie geplaatst zoals hierboven, zonder aanvullende voorwaarden.
