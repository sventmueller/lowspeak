<p><img src="../../readme-icon.png" alt="Lowspeaks appikon" width="168" height="168"></p>

# Lowspeak

**Diktering til Mac, der skriver, hvor din markør står: hold fn nede, tal, slip.**

Lowspeak bor i menulinjen. Hold fn-tasten (globus) nede, og tal; når du slipper, dukker dine ord op i den app, du skriver i, som om du selv havde skrevet dem. Programmet transskriberer online med OpenAI eller Groq eller helt på din Mac med Whisper eller NVIDIA Parakeet, som gør 40 sekunders tale til tekst på omkring en tredjedel af et sekund. Hvis netværket eller onlinetjenesten svigter, tager en installeret lokal model over, så en diktering ikke går tabt. Lær det de navne og termer, du bruger, lad en sprogmodel fjerne fyldord, hvis du vil, og lås en optagelse med Shift for at diktere længere tekster håndfrit. Programmet er skrevet i Rust med ægte AppKit-kontroller, findes på 35 sprog og sender kun din stemme til den udbyder, du har valgt.

[![Hent Lowspeak til macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Hent Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: signeret og notariseret af Apple, til macOS 14 eller nyere på Apple Silicon og Intel.

**Har du fundet en fejl?** [Opret et issue](https://github.com/sventmueller/lowspeak/issues/new/choose), eller vælg **Hjælp › Rapporter et problem…** i Lowspeaks menu: Formularen åbner med din version udfyldt. Hvis du har glæde af Lowspeak, hjælper en [stjerne på GitHub](https://github.com/sventmueller/lowspeak) andre med at finde det.

[![Rust 2024 edition](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 eller nyere](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon og Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit via objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp på Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet på Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online med OpenAI eller Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 grænsefladesprog](https://img.shields.io/badge/languages-35-4C9A2A)
![Signeret med Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licens: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licens)

[Bahasa Indonesia](../id/README.md) ·
[Bahasa Melayu](../ms/README.md) ·
[Català](../ca/README.md) ·
[Čeština](../cs/README.md) ·
**Dansk** ·
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

## Indhold

- [Overblik](#overblik)
- [Krav](#krav)
- [Installation](#installation)
- [Kom i gang](#kom-i-gang)
  - [Tilladelser](#tilladelser)
  - [Vælg, hvordan der transskriberes](#vælg-hvordan-der-transskriberes)
  - [Menuen](#menuen)
  - [Hurtig oversigt](#hurtig-oversigt)
- [Diktering](#diktering)
  - [Hold nede for at tale](#hold-nede-for-at-tale)
  - [Lås med Shift eller et dobbelttryk](#lås-med-shift-eller-et-dobbelttryk)
  - [Hvor teksten havner](#hvor-teksten-havner)
  - [Seneste dikteringer](#seneste-dikteringer)
- [Transskription](#transskription)
  - [Hvilken model passer til dig](#hvilken-model-passer-til-dig)
  - [Online: OpenAI og Groq](#online-openai-og-groq)
  - [På din Mac: Whisper og Parakeet](#på-din-mac-whisper-og-parakeet)
  - [Når onlinetjenesten svigter](#når-onlinetjenesten-svigter)
  - [Sådan forberedes optagelsen](#sådan-forberedes-optagelsen)
  - [Sprog pr. model](#sprog-pr-model)
  - [Mac-computere med Intel](#mac-computere-med-intel)
- [Ordliste](#ordliste)
- [Efterbehandling](#efterbehandling)
- [Mikrofon](#mikrofon)
- [API-nøgler](#api-nøgler)
- [Notifikationer](#notifikationer)
- [Udseende og tilgængelighed](#udseende-og-tilgængelighed)
- [Sprog](#sprog)
- [Taster og bevægelser](#taster-og-bevægelser)
- [Fortrolighed og sikkerhed](#fortrolighed-og-sikkerhed)
- [Hastighed](#hastighed)
- [Kendte begrænsninger](#kendte-begrænsninger)
- [Fejlfinding](#fejlfinding)
- [Til udviklere](#til-udviklere)
  - [Tredjepartskomponenter](#tredjepartskomponenter)
- [Licens](#licens)

## Overblik

- **Hold fn nede, tal, slip.** Teksten skrives ind i appen forrest, uanset hvor markøren står: mail, chat, editor, terminal eller browser. Intet vindue at skifte til, ingen knap at klikke på.
- **Lås den til længere tekster.** Tryk på Shift, mens du holder fn nede, eller tryk to gange på fn, så kører optagelsen videre, uden at du holder en tast nede, i op til fem minutter. Ét tryk på fn eller Shift afslutter den.
- **Den rigtige model til opgaven.** Groq for hastighed uden at bruge din Macs hukommelse, OpenAIs GPT Transcribe for det bedste resultat, Whisper eller NVIDIA Parakeet på din Mac for at arbejde offline og holde din stemme privat. Skift med ét klik.
- **Ingen diktering går tabt.** Hvis netværket eller onlinetjenesten svigter, transskriberer en installeret lokal model den samme optagelse på din Mac.
- **Dine ord, stavet rigtigt.** En ordliste med navne og fagudtryk guider OpenAI, Groq, Whisper og efterbehandlingen.
- **Ren tekst, hvis du vil.** Den valgfri efterbehandling fjerner fyldord, retter grammatikken og beholder kun din rettelse, når du retter dig selv, eller formaterer desuden teksten efter den slags app, du dikterer i.
- **Bliv bare ved med at tale.** Start den næste diktering, mens den forrige stadig transskriberes; teksterne kommer i den rækkefølge, du sagde dem.
- **Din udklipsholder forbliver din.** Lowspeak indsætter via udklipsholderen og lægger et øjeblik senere det tilbage, der lå på den. De sidste ti dikteringer bliver i menuen, så du kan kopiere dem igen.
- **Native helt ind til benet.** Et ægte menulinjeemne med en AppKit-menu, standardadvarsler og -notifikationer. Ingen Electron, intet vindue, intet Dock-symbol.
- **Privat.** Ingen konto, ingen telemetri. Lyd sendes kun til den udbyder, du har valgt, eller slet ingen steder hen i lokal tilstand.
- **På dit sprog.** Diktering virker på snesevis af sprog, og menuen, dialogerne og notifikationerne findes på 35.

## Krav

- macOS 14 Sonoma eller nyere. Lowspeak udvikles og testes på macOS 27.
- En Mac med Apple Silicon eller en Intel-processor. Lokal transskription er bygget til Apple Silicon, hvor Whisper kører på GPU'en og Parakeet på Neural Engine; på Mac-computere med Intel kører begge på processoren, meget langsommere, se [Mac-computere med Intel](#mac-computere-med-intel).
- Cirka 10 MB til appen plus 75 MB til 3 GB for hver lokal model, du henter.
- Til onlinetransskription en API-nøgle fra [OpenAI](https://platform.openai.com/api-keys) eller [Groq](https://console.groq.com/keys); begge tager betaling for det, du transskriberer. Lokal transskription kræver ingen konto.

## Installation

**Fra diskaftrykket.** Hent [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), åbn det, og træk Lowspeak til mappen Programmer. Aftrykket er signeret med et Developer ID og notariseret af Apple, så det åbner uden advarsler.

**Opdateringer.** Lowspeak opdaterer ikke sig selv. Nye versioner dukker op på [udgivelsessiden](https://github.com/sventmueller/lowspeak/releases); **Hjælp › Lowspeak på GitHub** fører dertil. Installer en ny version oven på den gamle; dine indstillinger, nøgler og modeller bevares.

**Fra kildekode.** Se [Bygning og installation](../../../docs/development.md#build--deploy) for forudsætningerne, og kør derefter:

```sh
make deploy
```

Det bygger Lowspeak, erstatter kopien i din mappe Programmer og starter den.

**Åbn ved login** i menuen starter Lowspeak sammen med din Mac via macOS' egne loginemner. Der kører kun én kopi ad gangen; en anden afslutter med det samme.

## Kom i gang

### Tilladelser

Ved første opstart beder Lowspeak om tre tilladelser:

- **Tilgængelighed** (Systemindstillinger › Anonymitet & sikkerhed › Tilgængelighed; på macOS 27 hedder listen Styring af enhed og adgang til data). Lowspeak skal bruge den for at registrere fn-tasten og indsætte med ⌘V. macOS tilbyder at åbne listen; slå kontakten til for Lowspeak. Lowspeaks egne beskeder bruger det navn på listen, som din version af macOS viser.
- **Mikrofon.** Spørges én gang; uden den optager Lowspeak intet.
- **Notifikationer.** Lowspeak bruger dem til problemer, du ellers ikke ville opdage, f.eks. en mikrofon uden lyd eller en mislykket transskription.

### Vælg, hvordan der transskriberes

Konfigurer mindst én måde; du kan skifte når som helst i menuen.

- **Online:** Vælg **Indstil API-nøgle…**, og indsæt en nøgle fra OpenAI (den starter med `sk-`) eller Groq (`gsk_`). Lowspeak genkender udbyderen på nøglen. Vælg derefter en model under **OpenAI** eller **Groq**.
- **På din Mac:** Vælg **Administrer modeller** › **Hent Parakeet Ultra** eller en Whisper-model, og vælg den derefter under **Lokalt**.

[Hvilken model passer til dig](#hvilken-model-passer-til-dig) hjælper med valget. Hold derefter fn nede, sig noget, og slip.

### Menuen

Klik på ringen i menulinjen:

| Punkt | Hvad det gør |
|---|---|
| **Seneste dikteringer** | de sidste ti tekster; et klik kopierer en af dem til udklipsholderen igen |
| **Lokalt** | modellerne på din Mac, grupperet i Whisper og Parakeet; dem, der ikke er hentet, er nedtonede |
| **OpenAI**, **Groq** | onlinemodellerne; uden nøgle er de nedtonede, og menuen tilbyder **Indstil API-nøgle…** |
| **Sprog** | det sprog, du dikterer på, eller **Registrer automatisk**; menuen følger det |
| **Efterbehandling** | **Fra**, **Ryd op** eller **Formater**, se [Efterbehandling](#efterbehandling) |
| **Ordliste…** | navne og termer, der skal staves præcist; menuen viser, hvor mange du har angivet |
| **Mikrofon** | **Systemstandard** eller en fast mikrofon samt **Afspil lyd, når klar** |
| **Administrer modeller** | hent og slet lokale modeller, med status for hentningen |
| **Indstil API-nøgle…** / **Skift API-nøgle…** | tilføj, erstat eller fjern nøgler |
| **Åbn ved login** | start Lowspeak sammen med din Mac; fluebenet viser, at det er slået til |
| **Hjælp** | **Hurtig oversigt**, **Rapporter et problem…**, **Foreslå en funktion…**, **Lowspeak på GitHub** |
| **Om Lowspeak** | version, copyright, projektside og licens |
| **Slut Lowspeak** | ⌘Q, mens menuen er åben |

Fluebenet under **Lokalt**, **OpenAI** og **Groq** markerer den model, der er i brug. Symbolet viser, hvad Lowspeak er i gang med:

| Symbol | Betydning |
|---|---|
| Ring med en prik | Klar |
| Udfyldt cirkel | Optager, mens fn holdes nede |
| Afrundet firkant | Optagelsen er låst; næste tryk på fn eller Shift afslutter den |
| Blinkende ring | Transskriberer |
| ↓ 42 % | Henter en model |

Lowspeak har intet indstillingsvindue: Alt findes i denne menu, og den husker hvert valg.

### Hurtig oversigt

**Hjælp › Hurtig oversigt** viser fn-bevægelserne i et lille vindue på dit sprog med en knap, der åbner tastaturindstillingerne. **Rapporter et problem…** åbner en formular på GitHub, hvor dine versioner af Lowspeak og macOS allerede er udfyldt, **Foreslå en funktion…** en diskussion til idéer og **Lowspeak på GitHub** projektsiden.

## Diktering

### Hold nede for at tale

Hold fn nede, tal, slip. Optagelsessymbolet vises i det øjeblik, din mikrofon leverer lyd; begynd først at tale da, for det, du siger før, bliver ikke optaget. Når du slipper fn, slutter optagelsen, og teksten følger et øjeblik senere.

Et kort tryk på fn gør ingenting, så fn beholder sine andre funktioner. En optagelse, der forbliver tom eller varer under 0,3 sekunder, kasseres.

### Lås med Shift eller et dobbelttryk

Til længere tekster kan du låse optagelsen, så den kører, uden at du holder en tast nede. Der er tre måder:

- **Shift, mens du holder fn nede.** Start med fn som sædvanligt, og tryk på Shift under optagelsen: Den låses, og du kan slippe fn. Intet af det, du allerede har sagt, går tabt; optagelsen fortsætter bare.
- **Shift, derefter fn.** Hold Shift nede, og tryk på fn: Optagelsen starter låst.
- **fn to gange.** Tryk kort på fn (kortere end 0,3 sekunder), og tryk igen inden for 0,4 sekunder. Optagelsen kører fra det første tryk.

Symbolet bliver til en afrundet firkant. Tryk én gang på **fn** eller **Shift** for at stoppe; teksten følger som sædvanligt. Hvis din 🌐-tast er indstillet til at åbne Emojis og symboler, så stop med Shift: Et kort tryk på fn ville åbne oversigten.

En optagelse stopper af sig selv efter to minutter, mens fn holdes nede, og efter fem minutter, når den er låst, og transskriberes derefter som sædvanligt.

### Hvor teksten havner

Lowspeak indsætter teksten i den app, der var forrest, da du holdt op med at tale, ved markøren, via udklipsholderen og ⌘V. Programmet gemmer alt, hvad der lå på udklipsholderen før, og lægger det tilbage et halvt sekund efter, at appen har læst teksten, eller efter ti sekunder, hvis den aldrig gør det, medmindre en anden app har ændret udklipsholderen i mellemtiden. Teksten er markeret som midlertidig, så apps til udklipsholderhistorik, der respekterer markeringen, ikke registrerer den.

Du kan starte den næste diktering, mens den forrige stadig transskriberes. Teksterne indsættes i den rækkefølge, du sagde dem, og aldrig mens du optager eller holder Shift nede, så en indsætning ikke kan lande midt i din næste sætning.

Uden tilladelsen til Tilgængelighed, eller når det simulerede ⌘V ikke kan sendes, bliver teksten på udklipsholderen, og en notifikation fortæller det; indsæt den så selv med ⌘V.

### Seneste dikteringer

**Seneste dikteringer** i menuen viser de sidste ti tekster, nyeste først, hver forkortet til én linje; holder du markøren over en, vises hele teksten. Et klik kopierer den til udklipsholderen, til når en tekst er havnet det forkerte sted, eller du skal bruge den to gange. Listen findes kun i hukommelsen og er tom, når Lowspeak er afsluttet.

## Transskription

### Hvilken model passer til dig

| Du ønsker | Vælg | Hvorfor |
|---|---|---|
| **Hastighed uden at belaste din Mac** | **Groq › Whisper Large V3 Turbo** | den hurtigste onlinemulighed; intet hentes, og din Mac beholder sin hukommelse og diskplads |
| **Det bedste resultat** | **OpenAI › GPT Transcribe** (standard) | den mest præcise af de modeller, Lowspeak tilbyder; tager din ordliste som nøgleord |
| **Offline og privat** | **Lokalt › Parakeet Ultra** til europæiske sprog, **Lokalt › Whisper Large v3 Turbo** til alle andre | virker uden netværk, og din stemme forlader aldrig din Mac; Parakeet er den hurtigste af alle |
| **Ingen konto, ingen omkostninger** | en hvilken som helst model under **Lokalt** | kører på din Mac; intet faktureres |
| **Både pålidelighed og hastighed** | en onlinemodel plus Parakeet Ultra eller en Whisper-model installeret | den lokale model træder til, når onlinetjenesten svigter |
| **At diktere på en Mac med Intel** | **Groq** eller **OpenAI** | lokale modeller kører på processoren dér, se [Mac-computere med Intel](#mac-computere-med-intel) |

Lokale modeller optager deres downloadstørrelse på disken og, mens de er valgt, omtrent lige så meget hukommelse; Lowspeak holder modellen indlæst, så dikteringen starter uden forsinkelse. På en Mac med 8 GB hukommelse bør du foretrække Parakeet Ultra eller en onlinemodel frem for de store Whisper-modeller.

### Online: OpenAI og Groq

| Model | Udbyder | Bemærkninger |
|---|---|---|
| **GPT Transcribe** (standard) | OpenAI | mest præcis; tager din ordliste som nøgleord |
| **Whisper** | OpenAI | den oprindelige Whisper-model |
| **Whisper Large V3** | Groq | meget hurtig |
| **Whisper Large V3 Turbo** | Groq | hurtigst online |

Optagelsen komprimeres til AAC før overførslen, omkring 6 KB pr. sekund tale, så diktering også virker på langsomme forbindelser. Mens du holder fn nede, åbner Lowspeak allerede forbindelsen til udbyderen, så overførslen starter i det øjeblik, du slipper.

### På din Mac: Whisper og Parakeet

| Model | Størrelse | Kører på | Sprog |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 europæiske sprog |
| **Parakeet v2** | ~600 MB | Neural Engine | engelsk |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | omkring 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | omkring 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | omkring 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | omkring 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | omkring 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | omkring 99 |

**Administrer modeller** henter og sletter dem; under en hentning viser menulinjen, hvor langt den er nået, og når du afslutter Lowspeak, annulleres den. Første gang en model bruges, forberedes den til din Mac: omkring 10 sekunder for Parakeet Ultra, op til 30 sekunder for Whisper; derefter indlæses den på en brøkdel af et sekund. Hvis den model, du har valgt, slettes, skifter Lowspeak til en anden installeret model.

**Parakeet** er det hurtigste valg: Den transskriberer 40 sekunders tale på omkring 0,3 sekunder. Lowspeak kører den med de samme modeller og den samme afkodning som [FluidAudio](https://github.com/FluidInference/FluidAudio) og gemmer modellerne i FluidAudios mappe (`~/Library/Application Support/FluidAudio/Models`), så apps, der er bygget på FluidAudio, deler dem. Optagelser på mere end 15 sekunder transskriberes i overlappende vinduer, der sættes sammen uden at miste ord i overgangene. **Whisper** forstår flere sprog og læser din ordliste; de store modeller er de mest præcise, de små er meget svagere uden for engelsk. Whisper-modeller gemmes i `~/Library/Application Support/Lowspeak/Models`.

### Når onlinetjenesten svigter

Hvis en onlinetransskription mislykkes, fordi netværket er nede, udbyderen har et problem, eller nøglen er forkert, transskriberer en installeret lokal model den samme optagelse: den, der er valgt under **Lokalt**, ellers Parakeet Ultra, derefter Whisper Large v3 Turbo, Large v3 og de mindre Whisper-modeller og til sidst Parakeet v2. En notifikation fortæller dig det én gang; den kommer først igen, når en onlinediktering er lykkedes i mellemtiden.

Med en lokal model at falde tilbage på giver Lowspeak onlineanmodningen 20 sekunder; uden prøver programmet tre gange med op til et minut hver gang. I onlinetilstand forbereder Lowspeak Parakeet-reservemodellen én gang ved opstart, så den første reservetransskription ikke skal vente på den.

### Sådan forberedes optagelsen

Lowspeak optager med din mikrofons egen samplingfrekvens og konverterer, når du stopper, til de 16 kHz, som modellerne forventer. Før transskriptionen fjerner programmet dyb rumlen under 80 Hz, klipper stilhed i starten og slutningen væk og udjævner lydstyrken, så stille og høje talere kommer ens ud. En optagelse uden tale sendes slet ikke. I lokal tilstand får Parakeet først den urørte optagelse og kun den forberedte, hvis det ikke giver nogen tekst.

### Sprog pr. model

Sproget under **Sprog** sendes videre til onlinemodellerne og til Whisper. Med **Registrer automatisk** finder modellen selv ud af sproget.

| Model | Sprog, du kan diktere på |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, på din Mac) | omkring 99, heriblandt alle 35 fra menuen |
| Parakeet Ultra | bulgarsk, kroatisk, tjekkisk, dansk, hollandsk, engelsk, estisk, finsk, fransk, tysk, græsk, ungarsk, italiensk, lettisk, litauisk, maltesisk, polsk, portugisisk, rumænsk, russisk, slovakisk, slovensk, spansk, svensk, ukrainsk |
| Parakeet v2 | engelsk |

Parakeet registrerer selv sproget og ignorerer valget i menuen. **Kinesisk (traditionelt)** får GPT Transcribe til at skrive traditionelle tegn; Whisper skriver kinesisk med begge skriftsystemer og portugisisk uden hensyn til varianten.

### Mac-computere med Intel

Lowspeak er en universel app og kører også på Mac-computere med Intel og macOS 14. Alt, der foregår online, virker på samme måde. De lokale modeller har ikke den hardware, de er bygget til, dér: Whisper kører på processoren i stedet for GPU'en, og Parakeet på processoren i stedet for Neural Engine. Begge virker, men tager mange gange længere tid end på Apple Silicon, de store Whisper-modeller alt for lang tid til diktering. På en Mac med Intel bør du transskribere online og beholde Whisper Small eller Parakeet Ultra som [reserve](#når-onlinetjenesten-svigter).

## Ordliste

**Ordliste…** tager imod de navne og termer, Lowspeak skal stave præcist, adskilt med komma: produktnavne, kolleger, fagudtryk. Op til 100 termer på højst 60 tegn hver; `<` og `>` fjernes.

- **GPT Transcribe** modtager dem som nøgleord.
- **Whisper** (OpenAI, Groq og på din Mac) læser dem som et hint før optagelsen, op til 500 tegn.
- **Efterbehandlingen** får besked på at stave dem præcis som skrevet.
- **Parakeet** bruger dem ikke.

## Efterbehandling

Efter transskriptionen kan en sprogmodel bearbejde teksten. Vælg under **Efterbehandling**:

- **Fra** (standard) – teksten, som den blev transskriberet.
- **Ryd op** – fjerner fyldord som “øh”, “ikke” eller “altså”, retter grammatik og tegnsætning og beholder kun rettelsen, når du retter dig selv. Den tilføjer intet og ændrer ikke betydningen.
- **Formater** – som Ryd op og tilpasser desuden teksten til den app, du dikterer i: termer i kodestil i kodeeditorer, kommandolinjer i terminaler, en uformel tone i chatapps, en professionel i e-mail.

Efterbehandlingen bruger Groq (`openai/gpt-oss-120b`), hvis du har en Groq-nøgle, ellers OpenAI (`gpt-4o-mini`), også når du transskriberer lokalt. Med et sprog valgt under **Sprog** holder den sig til det sprog. Tager den mere end otte sekunder, mislykkes den, eller returnerer den et svar, der er afskåret eller langt længere end det, du sagde, indsætter Lowspeak den ubehandlede tekst. Afviser udbyderen anmodningen, eller er der ikke indstillet nogen nøgle, fortæller en notifikation det én gang.

## Mikrofon

Vælg en fast mikrofon eller **Systemstandard** under **Mikrofon**. Lowspeak husker enheden; mens den ikke er tilsluttet, viser menuen den som **(ikke tilsluttet)**, og Lowspeak optager med systemstandarden.

Mikrofonen slås kun til, mens du dikterer. Det tager omkring 0,07 sekunder at gøre den klar med den indbyggede mikrofon, omkring et halvt sekund med et Studio Display og et til to sekunder med AirPods, der først skifter til deres headsettilstand. Optagelsessymbolet vises, så snart der kommer lyd; **Afspil lyd, når klar** tilføjer en kort lyd i det øjeblik. Den er slået fra som standard, fordi lyden selv kommer et halvt sekund for sent på nogle højttalere.

Hvis en mikrofon kun leverer stilhed i fem sekunder, som AirPods gør, når en anden enhed holder på deres mikrofon, giver Lowspeak dig besked.

## API-nøgler

**Indstil API-nøgle…** beder om en nøgle og genkender udbyderen på dens begyndelse: `sk-` for OpenAI, `gsk_` for Groq. En nøgle i et andet format afvises med en forklaring. Angiv én nøgle pr. udbyder; med nøgler til begge er alle onlinemodeller tilgængelige. **Skift API-nøgle…** erstatter en nøgle eller sletter dem med **Fjern alle nøgler**.

Nøglerne gemmes i din loginnøglering under `com.lowspeak.app`. En nøglefil fra en tidligere version (`~/Library/Application Support/Lowspeak/.api-key`) flyttes dertil ved næste opstart og slettes, så snart alle nøgler kan læses tilbage fra nøgleringen.

## Notifikationer

Lowspeak arbejder i baggrunden, så det rapporterer problemer som notifikationer:

| Notifikation | Hvad der skete |
|---|---|
| **Optagelse mislykkedes** | ingen adgang til mikrofonen, ingen mikrofon tilsluttet, eller mikrofonen kunne ikke starte |
| **Ingen lyd fra mikrofonen** | fem sekunders stilhed, se [Mikrofon](#mikrofon) |
| **Tilladelse til tilgængelighed kræves** | teksten ligger på udklipsholderen; giv tilladelse til Tilgængelighed for at indsætte automatisk |
| **Indsætning mislykkedes** | teksten ligger på udklipsholderen; indsæt den med ⌘V |
| **Transskription mislykkedes** | med årsagen: ingen nøgle, afvist nøgle, grænse for forespørgsler, server- eller netværksfejl eller ingen lokal model |
| **Transskriberet på denne Mac** | onlinetjenesten svigtede, og en lokal model tog over |
| **Efterbehandling mislykkedes** | udbyderen afviser efterbehandlingen, eller der er ikke indstillet nogen nøgle; teksten blev indsat uden efterbehandling |
| **Ingen API-nøgle** | du har valgt en onlinemodel uden en nøgle til dens udbyder |
| **Hentning mislykkedes**, **Sletning mislykkedes**, **Kan ikke slette** | administrationen af en lokal model lykkedes ikke, eller en anden handling var stadig i gang |
| **Fejl ved loginemne**, **Fejl ved API-nøgle** | macOS eller nøgleringen afviste ændringen |

Notifikationer indeholder aldrig din dikterede tekst. En onlinetjeneste, der svigter, og en udbyder, der afviser efterbehandlingen, rapporteres én gang, ikke efter hver diktering, indtil de virker igen.

## Udseende og tilgængelighed

- **Menulinjesymbolet** er et skabelonbillede, der følger en lys, mørk eller farvet menulinje.
- **VoiceOver** læser symbolet som “Lowspeak”, og menuen, advarslerne og notifikationerne er standardkontroller i macOS.
- **Ingen tast skal holdes nede.** Lås en optagelse med Shift eller et dobbelttryk på fn, og det næste tryk afslutter den.
- **Hurtig oversigt** er et enkelt vindue med etiketter og en knap, som kan nås fra tastaturet; Esc lukker det.

Kendte mangler står i [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver annoncerer ikke, når en optagelse starter, og Lowspeak er endnu ikke afprøvet med VoiceOver eller Knapbetjening.

## Sprog

Menuen, dialogerne og notifikationerne er oversat til 35 sprog:

arabisk, catalansk, kinesisk (forenklet), kinesisk (traditionelt), kroatisk, tjekkisk, dansk, hollandsk, engelsk, finsk, fransk, tysk, græsk, hebraisk, hindi, ungarsk, indonesisk, italiensk, japansk, koreansk, malajisk, norsk, polsk, portugisisk (Brasilien), portugisisk (Portugal), rumænsk, russisk, slovakisk, slovensk, spansk, svensk, thai, tyrkisk, ukrainsk og vietnamesisk.

Det sprog, du vælger under **Sprog**, er også menuens sprog. Med **Registrer automatisk** følger menuen sprogrækkefølgen i **Systemindstillinger › Generelt › Sprog & område** og bruger engelsk, hvor der ikke findes en oversættelse. Menutermerne følger Apples egne oversættelser, så Lowspeak læses som resten af din Mac. Bortset fra engelsk er oversættelserne maskinlavede og endnu ikke gennemgået af personer med sproget som modersmål; rettelser er velkomne som [issues](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Taster og bevægelser

| Handling | Taster |
|---|---|
| Diktér, mens der holdes nede | hold fn nede |
| Lås en igangværende optagelse | Shift, mens du holder fn nede |
| Start en låst optagelse | Shift+fn eller fn to gange hurtigt |
| Afslut en låst optagelse | fn eller Shift |
| Slut Lowspeak (menuen åben) | ⌘Q |
| Luk Hurtig oversigt | Esc |

Lowspeak forhindrer de fn-tryk, det bruger, i at nå andre apps; Shift sendes videre som normalt.

## Fortrolighed og sikkerhed

Lowspeak hører, hvad du siger, mens fn holdes nede, kan skrive i enhver app og opbevarer dine API-nøgler, så det gør så lidt som muligt med dem.

- **Din stemme går ét sted hen.** I onlinetilstand sendes optagelsen komprimeret til den udbyder, du har valgt, OpenAI eller Groq, sammen med din ordliste. I lokal tilstand forlader intet din Mac. Når en onlineanmodning mislykkes, transskriberer en lokal model den samme optagelse på din Mac.
- **Efterbehandling** sender, hvis du slår den til, den transskriberede tekst og din ordliste til Groq eller OpenAI. I tilstanden Formater tilføjer den, hvilken slags app du dikterer i (kodeeditor, terminal, chat eller e-mail); appens navn sendes ikke.
- **Ingen telemetri, ingen konto.** Ud over transskription og efterbehandling går Lowspeak kun online for at åbne forbindelsen til din udbyder, mens fn holdes nede, i onlinetilstand eller med efterbehandling slået til, og for at hente en model, når du beder om det, fra Hugging Face over HTTPS, i en fast version og med kontrol af hver fils kontrolsum. **Hjælp › Rapporter et problem…** åbner en formular på GitHub i din browser; dens adresse indeholder Lowspeaks version og macOS-versionen, intet af det, du har dikteret.
- **API-nøgler** bliver i din loginnøglering. De sendes kun til deres egen udbyder og fjernes fra fejlmeddelelser, før noget skrives til loggen.
- **Dikteret tekst gemmes ikke.** De sidste ti dikteringer bliver i hukommelsen til **Seneste dikteringer** og forsvinder, når Lowspeak afsluttes. Logfilerne registrerer længder og tider, aldrig teksten.
- **Udklipsholderen** gemmes før indsætning og gendannes, så snart appen har læst teksten, medmindre en anden app har ændret den i mellemtiden. Dikteringen er markeret som midlertidig, så apps til udklipsholderhistorik, der respekterer markeringen, springer den over.
- **Kodesignering** med Hardened Runtime; appen beder kun om adgang til mikrofonen, Tilgængelighed, notifikationer og netværket.

Følg [sikkerhedspolitikken](../../../SECURITY.md) for at rapportere en sårbarhed.

## Hastighed

Målt på Apple Silicon:

- **Parakeet Ultra** transskriberer 40 sekunders tale på omkring 0,3 sekunder.
- **Online:** Den første anmodning efter en pause tager omkring 0,4 sekunder hos Groq, fordi Lowspeak åbner forbindelsen, mens du stadig taler; senere anmodninger genbruger den, over HTTP/3, hvor udbyderen tilbyder det.
- **Overførselsstørrelse:** omkring 6 KB pr. sekund tale, som AAC ved 48 kbit/s.
- **At gøre mikrofonen klar:** omkring 0,07 sekunder for den indbyggede, et halvt sekund på et Studio Display, et til to sekunder med AirPods.

## Kendte begrænsninger

- **Lokale modeller på Mac-computere med Intel** kører på processoren og er meget langsommere; onlinetransskription er det bedre valg dér.
- **Det, du siger, før mikrofonen er klar, går tabt.** Vent på optagelsessymbolet, især med AirPods.
- **fn-tasten kan blive optaget af andre.** Når macOS' indstilling for 🌐-tasten, Warp eller macOS' diktering gør krav på fn først, ser Lowspeak den ikke; se [Fejlfinding](#fejlfinding).
- **Parakeet** ignorerer ordlisten og sprogvalget og kender kun europæiske sprog.
- **Skriftsystem og variant:** Kun GPT Transcribe kan få besked på at skrive traditionelt kinesisk; ingen model kan få besked på at skrive europæisk frem for brasiliansk portugisisk.
- **Tidsgrænser:** to minutter, mens fn holdes nede, fem minutter, når optagelsen er låst.
- **Mikrofontilstand:** Kontrolcenter tilbyder Stemmeisolering og Bredt spektrum til Lowspeak og viser dem som aktive, men Lowspeak optager uden stemmebehandling, så valget har ingen virkning.
- **Seneste dikteringer** findes kun i hukommelsen og er væk efter afslutning.
- **Af design:** intet indstillingsvindue, intet Dock-symbol, ingen automatiske opdateringer, ikke i App Store.

## Fejlfinding

**Der sker intet, når jeg holder fn nede.** Kontroller tre ting: Lowspeak er tilladt under **Systemindstillinger › Anonymitet & sikkerhed › Tilgængelighed** (på macOS 27: **Styring af enhed og adgang til data**); **Systemindstillinger › Tastatur › Tryk på 🌐-tasten for at** er indstillet til **Gøre ingenting**; og ingen anden app gør krav på fn. Det gør Warp som standard; slå det fra med `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'`, og genstart Warp. macOS' diktering kan også være indstillet til at starte med fn.

**Efter en opdatering reagerer Lowspeak ikke længere på fn.** macOS knytter tilladelsen til Tilgængelighed til appens signatur. Hvis signaturen er ændret, f.eks. fra et build, du selv har lavet, til den hentede app, så fjern Lowspeak fra listen under Tilgængelighed (på macOS 27: Styring af enhed og adgang til data), og tilføj det igen.

**De første ord mangler.** Begynd at tale, så snart optagelsessymbolet vises. Med AirPods skal mikrofonen bruge et til to sekunder; en indbygget mikrofon eller en USB-mikrofon under **Mikrofon** er klar næsten med det samme.

**“Ingen lyd fra mikrofonen.”** Mikrofonen leverede ingen lyd i fem sekunder. Med AirPods bruger en anden enhed måske deres mikrofon; skift til den indbyggede mikrofon, eller forbind dem igen. Et lukket MacBook-låg slår også den indbyggede mikrofon fra.

**Dikteringer bliver tomme.** Styrken på lyd ind er måske for lav: Apps som Teams, Zoom eller Meet ændrer den under opkald, og macOS gemmer den for hver enhed. Skru op for den under **Systemindstillinger › Lyd › Lyd ind**.

**Teksten havnede det forkerte sted.** **Seneste dikteringer** i menuen kopierer den til udklipsholderen igen.

**“Ingen API-nøgle.”** Vælg **Indstil API-nøgle…** i menuen, eller skift til en lokal model.

**Den første lokale transskription tager lang tid.** Hver model forberedes til din Mac, første gang den bruges; derefter indlæses den på en brøkdel af et sekund.

**Lowspeak afslutter lige efter start.** En anden kopi af Lowspeak kører allerede; afslut den først.

**Der vises ingen notifikationer.** Tillad dem under **Systemindstillinger › Notifikationer › Lowspeak**.

**Noget andet.** Logfilerne i `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) beskriver, hvad der skete, uden din dikterede tekst. Vedhæft de relevante linjer til en [problemrapport](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Til udviklere

Bygning, test, kodens opbygning og de regler, den følger, er beskrevet i [docs/development.md](../../../docs/development.md); hvordan du bidrager, står i [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Tredjepartskomponenter

| Komponent | Version | Bruges til | Licens |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) via [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | lokal Whisper på GPU'en (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet-afkodning og sammenfletning af lange optagelser, oversat til Rust i `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) og dens framework-crates | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation og andre systemframeworks | MIT; Zlib, Apache-2.0 eller MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | mikrofonoptagelse via CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | omsampling til 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) med rustls | 3.5 | hentning af modeller | MIT eller Apache-2.0; rustls og ring under ISC og Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | kontrolsummer for hentede modeller | MIT eller Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | API-nøgler i nøgleringen | MIT eller Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, fejl, Parakeets ordforråd | MIT eller Apache-2.0; Apache-2.0 |

Licensteksterne for alt i appen følger med den som `Contents/Resources/Acknowledgements.txt`.

Modellerne er ikke en del af Lowspeak; programmet henter dem, når du beder om det:

| Model | Kilde | Licens |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (OpenAIs Whisper i ggml-format) | MIT |
| Parakeet v2, Parakeet Ultra | CoreML-konverteringer af NVIDIAs Parakeet TDT fra [FluidInference](https://huggingface.co/FluidInference) | CC-BY-4.0 |

## Licens

Lowspeak er licenseret under en af følgende

- Apache-licensen, version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- MIT-licensen ([LICENSE-MIT](../../../LICENSE-MIT))

efter eget valg.

Medmindre du udtrykkeligt angiver andet, skal ethvert bidrag, som du bevidst indsender til optagelse i Lowspeak, som defineret i Apache-2.0-licensen, have dobbeltlicens som ovenfor, uden yderligere vilkår eller betingelser.
