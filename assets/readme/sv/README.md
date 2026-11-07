<p><img src="../../readme-icon.png" alt="Lowspeaks appikon" width="168" height="168"></p>

# Lowspeak

**Diktering för Mac som skriver där markören står: håll ned fn, tala, släpp.**

Lowspeak finns i menyraden. Håll ned fn-tangenten (Glob) och tala; när du släpper den dyker dina ord upp i den app du skriver i, som om du hade skrivit dem själv. Lowspeak transkriberar online med OpenAI eller Groq, eller helt på din Mac med Whisper eller NVIDIA Parakeet, som gör om 40 sekunders tal till text på ungefär en tredjedels sekund. Om nätverket eller onlinetjänsten slutar fungera tar en installerad lokal modell över, så att ingen diktering går förlorad. Lär appen de namn och termer du använder, låt en språkmodell ta bort fyllnadsord om du vill, och lås en inspelning med Shift för att diktera längre texter handsfree. Lowspeak är skrivet i Rust med äkta AppKit-kontroller, finns på 35 språk och skickar din röst bara till den leverantör du har valt.

[![Ladda ner Lowspeak för macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Ladda ner Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: signerad och notariserad av Apple, för macOS 14 eller senare på Apple Silicon och Intel.

**Hittat en bugg?** [Öppna ett issue](https://github.com/sventmueller/lowspeak/issues/new/choose), eller välj **Hjälp › Rapportera ett problem…** i Lowspeaks meny: formuläret öppnas med din version redan ifylld. Om du har nytta av Lowspeak hjälper en [stjärna på GitHub](https://github.com/sventmueller/lowspeak) andra att hitta det.

[![Rust 2024 edition](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 eller senare](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universell: Apple Silicon och Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit via objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp på Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet på Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online med OpenAI eller Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 gränssnittsspråk](https://img.shields.io/badge/languages-35-4C9A2A)
![Signerad med Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licens: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licens)

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
[Português (Portugal)](../pt_PT/README.md) ·
[Română](../ro/README.md) ·
[Slovenčina](../sk/README.md) ·
[Slovenščina](../sl/README.md) ·
[Suomi](../fi/README.md) ·
**Svenska** ·
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

## Innehåll

- [Funktioner i korthet](#funktioner-i-korthet)
- [Krav](#krav)
- [Installation](#installation)
- [Komma igång](#komma-igång)
  - [Behörigheter](#behörigheter)
  - [Välj hur du vill transkribera](#välj-hur-du-vill-transkribera)
  - [Menyn](#menyn)
  - [Snabbreferens](#snabbreferens)
- [Diktera](#diktera)
  - [Håll ned och tala](#håll-ned-och-tala)
  - [Lås med Shift eller ett dubbeltryck](#lås-med-shift-eller-ett-dubbeltryck)
  - [Var texten hamnar](#var-texten-hamnar)
  - [Senaste dikteringar](#senaste-dikteringar)
- [Transkribering](#transkribering)
  - [Vilken modell passar dig](#vilken-modell-passar-dig)
  - [Online: OpenAI och Groq](#online-openai-och-groq)
  - [På din Mac: Whisper och Parakeet](#på-din-mac-whisper-och-parakeet)
  - [När onlinetjänsten inte fungerar](#när-onlinetjänsten-inte-fungerar)
  - [Hur inspelningen förbereds](#hur-inspelningen-förbereds)
  - [Språk per modell](#språk-per-modell)
  - [Mac-datorer med Intel](#mac-datorer-med-intel)
- [Ordlista](#ordlista)
- [Efterbearbetning](#efterbearbetning)
- [Mikrofon](#mikrofon)
- [API-nycklar](#api-nycklar)
- [Notiser](#notiser)
- [Utseende och tillgänglighet](#utseende-och-tillgänglighet)
- [Språk](#språk)
- [Tangenter och gester](#tangenter-och-gester)
- [Integritet och säkerhet](#integritet-och-säkerhet)
- [Hastighet](#hastighet)
- [Kända begränsningar](#kända-begränsningar)
- [Felsökning](#felsökning)
- [För utvecklare](#för-utvecklare)
  - [Komponenter från tredje part](#komponenter-från-tredje-part)
- [Licens](#licens)

## Funktioner i korthet

- **Håll ned fn, tala, släpp.** Texten skrivs in i appen längst fram, där markören står: mejl, chatt, redigerare, terminal eller webbläsare. Inget fönster att byta till, ingen knapp att klicka på.
- **Lås den för längre texter.** Tryck på Shift medan du håller ned fn, eller tryck två gånger på fn, så fortsätter inspelningen utan att du håller ned någon tangent, i upp till fem minuter. Ett tryck på fn eller Shift avslutar den.
- **Rätt modell för uppgiften.** Groq för snabbhet utan att ta din Macs minne i anspråk, OpenAIs GPT Transcribe för bästa resultat, Whisper eller NVIDIA Parakeet på din Mac för att arbeta offline och hålla din röst privat. Byt med ett klick.
- **Ingen diktering går förlorad.** Om nätverket eller onlinetjänsten slutar fungera transkriberar en installerad lokal modell samma inspelning på din Mac.
- **Dina ord, rätt stavade.** En ordlista med namn och facktermer vägleder OpenAI, Groq, Whisper och efterbearbetningen.
- **Ren text om du vill.** Valfri efterbearbetning tar bort fyllnadsord, rättar grammatiken och behåller bara rättelsen när du rättar dig själv, eller formaterar dessutom texten efter den typ av app du dikterar i.
- **Fortsätt prata.** Starta nästa diktering medan den förra fortfarande transkriberas; texterna kommer i den ordning du sa dem.
- **Ditt urklipp förblir ditt.** Lowspeak klistrar in via urklippet och lägger tillbaka det som fanns där en stund senare. De tio senaste dikteringarna ligger kvar i menyn så att du kan kopiera dem igen.
- **Native in i märgen.** Ett riktigt menyradsobjekt med en AppKit-meny, vanliga varningsrutor och notiser. Inget Electron, inget fönster, ingen symbol i Dock.
- **Privat.** Inget konto, ingen telemetri. Ljudet går bara till den leverantör du har valt, eller ingenstans alls i lokalt läge.
- **På ditt språk.** Diktering fungerar på dussintals språk, och menyn, dialogrutorna och notiserna finns på 35.

## Krav

- macOS 14 Sonoma eller senare. Lowspeak utvecklas och testas på macOS 27.
- En Mac med Apple Silicon eller en Intel-processor. Lokal transkribering är byggd för Apple Silicon, där Whisper körs på GPU:n och Parakeet på Neural Engine; på Mac-datorer med Intel körs båda på processorn, mycket långsammare, se [Mac-datorer med Intel](#mac-datorer-med-intel).
- Cirka 17 MB för appen, plus 75 MB till 3 GB för varje lokal modell du hämtar.
- För onlinetranskribering en API-nyckel från [OpenAI](https://platform.openai.com/api-keys) eller [Groq](https://console.groq.com/keys); båda tar betalt för det du transkriberar. Lokal transkribering kräver inget konto.

## Installation

**Från skivavbilden.** Ladda ner [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), öppna den och dra Lowspeak till mappen Appar. Avbilden är signerad med ett Developer ID och notariserad av Apple, så den öppnas utan varningar.

**Uppdateringar.** Lowspeak från skivavbilden håller sig uppdaterat med [Sparkle](https://sparkle-project.org). Vid andra starten frågar appen om den får leta efter uppdateringar på egen hand, en gång om dagen. **Leta efter uppdateringar…** i menyn letar direkt; **Leta efter uppdateringar automatiskt** och **Hämta och installera uppdateringar automatiskt** strax nedanför ändrar ditt val senare. När en sökning hittar en ny version medan du arbetar i en annan app visas en punkt på ringen i menyraden, och **Leta efter uppdateringar…** tar fram uppdateringen. Lowspeak installerar en uppdatering bara om den bär utvecklarens signatur; dina inställningar, nycklar och modeller finns kvar. Version 1.0.0 har ingen uppdateringsfunktion: installera en nyare version över den en gång, från [sidan med releaser](https://github.com/sventmueller/lowspeak/releases). Ett eget bygge har inte heller någon uppdateringsfunktion.

**Från källkod.** Se [Bygge och driftsättning](../../../docs/development.md#build--deploy) för förutsättningarna och kör sedan:

```sh
make deploy
```

Det bygger Lowspeak, ersätter kopian i mappen Appar och startar den.

**Öppna vid inloggning** i menyn startar Lowspeak tillsammans med din Mac, via macOS:s egna startobjekt. Bara en kopia körs åt gången; en andra avslutas direkt.

## Komma igång

### Behörigheter

Vid första starten ber Lowspeak om tre behörigheter:

- **Hjälpmedel** (Systeminställningar › Integritet och säkerhet › Hjälpmedel; i macOS 27 heter listan Enhetskontroll och dataåtkomst). Lowspeak behöver den för att märka fn-tangenten och för att klistra in med ⌘V. macOS erbjuder sig att öppna listan; slå på reglaget för Lowspeak. Lowspeaks egna meddelanden använder det namn på listan som din macOS visar.
- **Mikrofon.** Frågas en gång; utan den spelar Lowspeak inte in något.
- **Notiser.** Lowspeak använder dem för problem som du annars inte skulle se, till exempel en tyst mikrofon eller en misslyckad transkribering.

### Välj hur du vill transkribera

Ställ in minst ett sätt; du kan byta när som helst i menyn.

- **Online:** välj **Ange API-nyckel…** och klistra in en nyckel från OpenAI (den börjar med `sk-`) eller Groq (`gsk_`). Lowspeak känner igen leverantören på nyckeln. Välj sedan en modell under **OpenAI** eller **Groq**.
- **På din Mac:** välj **Hantera modeller** › **Hämta Parakeet Ultra** eller en Whisper-modell, och välj den sedan under **Lokalt**.

[Vilken modell passar dig](#vilken-modell-passar-dig) hjälper dig att välja. Håll sedan ned fn, säg något och släpp.

### Menyn

Klicka på ringen i menyraden:

| Objekt | Vad det gör |
|---|---|
| **Senaste dikteringar** | de tio senaste texterna; ett klick kopierar en av dem till urklippet igen |
| **Lokalt** | modellerna på din Mac, grupperade i Whisper och Parakeet; de som inte är hämtade är nedtonade |
| **OpenAI**, **Groq** | onlinemodellerna; utan nyckel är de nedtonade och menyn erbjuder **Ange API-nyckel…** |
| **Språk** | språket du dikterar på, eller **Identifiera automatiskt**; menyn följer det |
| **Efterbearbetning** | **Av**, **Rensa** eller **Formatera**, se [Efterbearbetning](#efterbearbetning) |
| **Ordlista…** | namn och termer som ska stavas exakt; menyn visar hur många du har angett |
| **Mikrofon** | **Systemstandard** eller en fast mikrofon, och **Spela upp ljud när redo** |
| **Hantera modeller** | hämta och radera lokala modeller, med förlopp |
| **Ange API-nyckel…** / **Ändra API-nyckel…** | lägg till, ersätt eller ta bort nycklar |
| **Öppna vid inloggning** | starta Lowspeak med din Mac; bocken visar att det är på |
| **Leta efter uppdateringar…** | leta efter en ny version nu; de två objekten nedanför avgör om Lowspeak letar en gång om dagen och installerar uppdateringar utan att fråga. Bara i appen från skivavbilden |
| **Hjälp** | **Snabbreferens**, **Rapportera ett problem…**, **Föreslå en funktion…**, **Lowspeak på GitHub** |
| **Om Lowspeak** | version, upphovsrätt, projektsida och licens |
| **Avsluta Lowspeak** | ⌘Q när menyn är öppen |

Bocken under **Lokalt**, **OpenAI** och **Groq** markerar modellen som används. Symbolen visar vad Lowspeak gör:

| Symbol | Betydelse |
|---|---|
| Ring med en punkt | Redo |
| Fylld cirkel | Spelar in medan fn hålls ned |
| Rundad kvadrat | Inspelningen är låst; nästa tryck på fn eller Shift avslutar den |
| Blinkande ring | Transkriberar |
| Ring med en punkt uppe till höger | En ny version väntar, se **Leta efter uppdateringar…** |
| ↓ 42 % | Hämtar en modell |

Lowspeak har inget inställningsfönster: allt finns i den här menyn, och appen kommer ihåg varje val.

### Snabbreferens

**Hjälp › Snabbreferens** visar fn-gesterna i ett litet fönster, på ditt språk, med en knapp som öppnar tangentbordsinställningarna. **Rapportera ett problem…** öppnar ett formulär på GitHub med dina versioner av Lowspeak och macOS ifyllda, **Föreslå en funktion…** en diskussion för idéer och **Lowspeak på GitHub** projektsidan.

## Diktera

### Håll ned och tala

Håll ned fn, tala, släpp. Inspelningssymbolen visas i samma ögonblick som mikrofonen levererar ljud; börja tala då, eftersom det du säger innan inte spelas in. När du släpper fn avslutas inspelningen, och texten följer en stund senare.

Ett kort tryck på fn gör ingenting, så fn behåller sina andra funktioner. En inspelning som förblir tom eller varar kortare än 0,3 sekunder kastas.

### Lås med Shift eller ett dubbeltryck

För längre texter låser du inspelningen så att den fortsätter utan att du håller ned någon tangent. Det finns tre sätt:

- **Shift medan du håller ned fn.** Börja med fn som vanligt och tryck på Shift under inspelningen: den låses och du kan släppa fn. Inget av det du redan har sagt går förlorat, inspelningen fortsätter bara.
- **Shift, sedan fn.** Håll ned Shift och tryck på fn: inspelningen startar låst.
- **fn två gånger.** Tryck kort på fn (kortare än 0,3 sekunder) och tryck igen inom 0,4 sekunder. Inspelningen pågår från det första trycket.

Symbolen blir en rundad kvadrat. Tryck en gång på **fn** eller **Shift** för att stoppa; texten följer som vanligt. Om 🌐-tangenten är inställd på att visa Emojier och symboler stoppar du med Shift: ett kort tryck på fn skulle öppna teckenvisaren.

En inspelning stoppas av sig själv efter två minuter medan fn hålls ned och efter fem minuter när den är låst, och transkriberas sedan som vanligt.

### Var texten hamnar

Lowspeak klistrar in texten i den app som låg längst fram när du slutade tala, vid markören, via urklippet och ⌘V. Appen sparar allt som fanns i urklippet innan och lägger tillbaka det en halv sekund efter att den mottagande appen har läst texten, eller efter tio sekunder om det aldrig sker, om inte en annan app har ändrat urklippet under tiden. Texten är markerad som tillfällig, så appar för urklippshistorik som respekterar markeringen sparar den inte.

Du kan starta nästa diktering medan den förra fortfarande transkriberas. Texterna klistras in i den ordning du sa dem, och aldrig medan du spelar in eller håller ned Shift, så en inklistring kan inte hamna mitt i din nästa mening.

Utan behörighet för Hjälpmedel, eller om det simulerade ⌘V inte kan skickas, ligger texten kvar i urklippet och en notis talar om det; klistra in den själv med ⌘V.

### Senaste dikteringar

**Senaste dikteringar** i menyn listar de tio senaste texterna, nyaste först, var och en förkortad till en rad; håller du pekaren över en text visas den i sin helhet. Ett klick kopierar den till urklippet, för när en text har hamnat på fel ställe eller du behöver den två gånger. Listan finns bara i minnet och är tom när Lowspeak har avslutats.

## Transkribering

### Vilken modell passar dig

| Du vill ha | Välj | Varför |
|---|---|---|
| **Snabbhet utan att belasta din Mac** | **Groq › Whisper Large V3 Turbo** | det snabbaste onlinealternativet; ingenting hämtas, och din Mac behåller sitt minne och diskutrymme |
| **Bästa resultat** | **OpenAI › GPT Transcribe** (standard) | den mest exakta av de modeller Lowspeak erbjuder; tar emot din ordlista som nyckelord |
| **Offline och privat** | **Lokalt › Parakeet Ultra** för europeiska språk, **Lokalt › Whisper Large v3 Turbo** för alla andra | fungerar utan nätverk, och din röst lämnar aldrig din Mac; Parakeet är snabbast av alla |
| **Inget konto, inga kostnader** | valfri modell under **Lokalt** | körs på din Mac; ingenting faktureras |
| **Både tillförlitlighet och snabbhet** | en onlinemodell, plus Parakeet Ultra eller en Whisper-modell installerad | den lokala modellen hoppar in när onlinetjänsten slutar fungera |
| **En Mac med Intel** | **Groq** eller **OpenAI** | lokala modeller körs där på processorn, se [Mac-datorer med Intel](#mac-datorer-med-intel) |

Lokala modeller tar upp sin hämtningsstorlek på disken och, så länge de är valda, ungefär lika mycket minne; Lowspeak håller modellen inläst så att dikteringen startar utan fördröjning. På en Mac med 8 GB minne är Parakeet Ultra eller en onlinemodell att föredra framför de stora Whisper-modellerna.

### Online: OpenAI och Groq

| Modell | Leverantör | Anmärkningar |
|---|---|---|
| **GPT Transcribe** (standard) | OpenAI | mest exakt; tar emot din ordlista som nyckelord |
| **Whisper** | OpenAI | den ursprungliga Whisper-modellen |
| **Whisper Large V3** | Groq | mycket snabb |
| **Whisper Large V3 Turbo** | Groq | snabbast online |

Inspelningen komprimeras till AAC före uppladdningen, cirka 6 KB per sekund tal, så dikteringen fungerar även på långsamma anslutningar. Medan du håller ned fn öppnar Lowspeak redan anslutningen till leverantören, så att uppladdningen startar i samma ögonblick som du släpper.

### På din Mac: Whisper och Parakeet

| Modell | Storlek | Körs på | Språk |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 europeiska språk |
| **Parakeet v2** | ~600 MB | Neural Engine | engelska |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | cirka 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | cirka 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | cirka 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | cirka 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | cirka 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | cirka 99 |

**Hantera modeller** hämtar och raderar dem; under en hämtning visar menyraden förloppet, och om du avslutar Lowspeak avbryts den. Allra första gången en modell används förbereds den för din Mac: cirka 10 sekunder för Parakeet Ultra, upp till 30 sekunder för Whisper; därefter läses den in på en bråkdel av en sekund. Om modellen du har valt raderas byter Lowspeak till en annan installerad modell.

**Parakeet** är det snabbaste valet: den transkriberar 40 sekunders tal på cirka 0,3 sekunder. Lowspeak kör den med samma modeller och avkodning som [FluidAudio](https://github.com/FluidInference/FluidAudio) och lagrar modellerna i FluidAudios mapp (`~/Library/Application Support/FluidAudio/Models`), så att appar som bygger på FluidAudio delar dem. Inspelningar längre än 15 sekunder transkriberas i överlappande fönster som fogas samman utan att ord går förlorade i skarvarna. **Whisper** förstår fler språk och läser din ordlista; de stora modellerna är mest exakta, de små betydligt svagare utanför engelska. Whisper-modeller lagras i `~/Library/Application Support/Lowspeak/Models`.

### När onlinetjänsten inte fungerar

Om en onlinetranskribering misslyckas, för att nätverket ligger nere, leverantören har problem eller nyckeln är fel, transkriberar en installerad lokal modell samma inspelning: den som är vald under **Lokalt**, annars Parakeet Ultra, sedan Whisper Large v3 Turbo, Large v3 och de mindre Whisper-modellerna, med Parakeet v2 sist. En notis meddelar dig en gång; den kommer igen först när en onlinediktering har lyckats däremellan.

Med en lokal modell att falla tillbaka på ger Lowspeak onlineförfrågan 20 sekunder; utan en sådan görs tre försök på upp till en minut vardera. I onlineläge förbereder Lowspeak reservmodellen Parakeet en gång vid start, så att den första reservtranskriberingen inte behöver vänta på den.

### Hur inspelningen förbereds

Lowspeak spelar in med mikrofonens egen samplingsfrekvens och konverterar till de 16 kHz som modellerna förväntar sig när du slutar. Före transkriberingen tar appen bort lågfrekvent muller under 80 Hz, klipper bort tystnad i början och slutet och jämnar ut volymen, så att tysta och högljudda talare låter lika. En inspelning utan tal skickas inte alls. I lokalt läge får Parakeet först den obearbetade inspelningen och den förberedda bara om den första inte ger någon text.

### Språk per modell

Språket under **Språk** skickas till onlinemodellerna och till Whisper. Med **Identifiera automatiskt** avgör modellen själv vilket språk det är.

| Modell | Språk du kan diktera på |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, på din Mac) | cirka 99, bland dem alla 35 i menyn |
| Parakeet Ultra | bulgariska, kroatiska, tjeckiska, danska, nederländska, engelska, estniska, finska, franska, tyska, grekiska, ungerska, italienska, lettiska, litauiska, maltesiska, polska, portugisiska, rumänska, ryska, slovakiska, slovenska, spanska, svenska, ukrainska |
| Parakeet v2 | engelska |

Parakeet identifierar språket själv och ignorerar valet i menyn. **Kinesiska (traditionell)** får GPT Transcribe att skriva traditionella tecken; Whisper skriver kinesiska med vilket av de två skriftsystemen som helst och portugisiska utan hänsyn till varianten.

### Mac-datorer med Intel

Lowspeak är en universell app och körs även på Mac-datorer med Intel och macOS 14. Allt online fungerar likadant. De lokala modellerna saknar där den hårdvara de är byggda för: Whisper körs på processorn i stället för GPU:n, och Parakeet på processorn i stället för Neural Engine. Båda fungerar, men tar många gånger längre tid än på Apple Silicon, de stora Whisper-modellerna alldeles för lång tid för diktering. På en Mac med Intel transkriberar du bäst online och behåller Whisper Small eller Parakeet Ultra som [reserv](#när-onlinetjänsten-inte-fungerar).

## Ordlista

**Ordlista…** tar emot de namn och termer som Lowspeak ska stava exakt, åtskilda med kommatecken: produktnamn, kolleger, facktermer. Upp till 100 termer med upp till 60 tecken vardera; `<` och `>` tas bort.

- **GPT Transcribe** tar emot dem som nyckelord.
- **Whisper** (OpenAI, Groq och på din Mac) läser dem som en ledtråd före inspelningen, upp till 500 tecken.
- **Efterbearbetningen** instrueras att stava dem exakt som de är skrivna.
- **Parakeet** använder dem inte.

## Efterbearbetning

Efter transkriberingen kan en språkmodell bearbeta texten. Välj under **Efterbearbetning**:

- **Av** (standard) – texten som den transkriberades.
- **Rensa** – tar bort fyllnadsord som ”öh”, ”liksom” eller ”typ”, rättar grammatik och skiljetecken, och behåller bara rättelsen när du rättar dig själv. Den lägger inte till något och ändrar inte betydelsen.
- **Formatera** – som Rensa, och anpassar dessutom texten till appen du dikterar i: kodliknande termer i kodredigerare, kommandorader i terminaler, en ledig ton i chattappar, en professionell i mejl.

Efterbearbetningen använder Groq (`openai/gpt-oss-120b`) om du har en Groq-nyckel, annars OpenAI (`gpt-4o-mini`), även när du transkriberar lokalt. Om ett språk är valt under **Språk** behåller den det språket. Om den tar längre tid än åtta sekunder, misslyckas eller ger ett svar som är avklippt eller mycket längre än det du sa, klistrar Lowspeak in den obearbetade texten. Om leverantören avvisar begäran eller ingen nyckel är angiven talar en notis om det en gång.

## Mikrofon

Under **Mikrofon** väljer du en fast mikrofon eller **Systemstandard**. Lowspeak kommer ihåg enheten; så länge den inte är ansluten visar menyn den som **(inte ansluten)** och Lowspeak spelar in med systemstandarden.

Mikrofonen slås bara på medan du dikterar. Att göra den redo tar cirka 0,07 sekunder med den inbyggda mikrofonen, ungefär en halv sekund med en Studio Display och en till två sekunder med AirPods, som först byter till sitt headsetläge. Inspelningssymbolen visas när ljudet kommer fram; **Spela upp ljud när redo** lägger till ett kort ljud i det ögonblicket. Det är avstängt som standard, eftersom själva ljudet kommer en halv sekund för sent på vissa högtalare.

Om en mikrofon bara levererar tystnad i fem sekunder, som AirPods gör när en annan enhet håller deras mikrofon, meddelar Lowspeak dig.

## API-nycklar

**Ange API-nyckel…** frågar efter en nyckel och känner igen leverantören på hur den börjar: `sk-` för OpenAI, `gsk_` för Groq. En nyckel i ett annat format avvisas med en förklaring. Ange en nyckel per leverantör; med nycklar för båda är alla onlinemodeller tillgängliga. **Ändra API-nyckel…** ersätter en nyckel eller, med **Ta bort alla nycklar**, raderar dem.

Nycklarna lagras i din inloggningsnyckelring under `com.lowspeak.app`. En nyckelfil från en tidigare version (`~/Library/Application Support/Lowspeak/.api-key`) flyttas dit vid nästa start och raderas så snart varje nyckel går att läsa tillbaka från nyckelringen.

## Notiser

Lowspeak arbetar i bakgrunden, så appen rapporterar problem som notiser:

| Notis | Vad som hände |
|---|---|
| **Inspelningen misslyckades** | ingen åtkomst till mikrofonen, ingen mikrofon ansluten, eller mikrofonen kunde inte starta |
| **Mikrofonen är tyst** | fem sekunders tystnad, se [Mikrofon](#mikrofon) |
| **Behörighet för hjälpmedel krävs** | texten finns i urklippet; tillåt Hjälpmedel för att klistra in automatiskt |
| **Inklistring misslyckades** | texten finns i urklippet; klistra in den med ⌘V |
| **Transkriberingen misslyckades** | med orsaken: ingen nyckel, avvisad nyckel, gränsen för antal förfrågningar, server- eller nätverksfel, eller ingen lokal modell |
| **Transkriberat på den här Macen** | onlinetjänsten slutade fungera och en lokal modell tog över |
| **Efterbearbetningen misslyckades** | leverantören avvisar efterbearbetningen eller ingen nyckel är angiven; texten klistrades in utan den |
| **Ingen API-nyckel** | du valde en onlinemodell utan nyckel för dess leverantör |
| **Hämtningen misslyckades**, **Raderingen misslyckades**, **Kan inte radera** | hanteringen av en lokal modell fungerade inte, eller en annan åtgärd pågick fortfarande |
| **Fel med startobjekt**, **Fel med API-nyckel** | macOS eller nyckelringen avvisade ändringen |

Notiser innehåller aldrig din dikterade text. En onlinetjänst som inte fungerar och en leverantör som avvisar efterbearbetningen rapporteras en gång, inte efter varje diktering, tills de fungerar igen.

## Utseende och tillgänglighet

- **Menyradssymbolen** är en mallbild som följer en ljus, mörk eller färgtonad menyrad.
- **VoiceOver** läser upp symbolen som ”Lowspeak”, och menyn, varningsrutorna och notiserna är vanliga macOS-kontroller.
- **Ingen tangent behöver hållas ned.** Lås en inspelning med Shift eller ett dubbeltryck på fn, så avslutar nästa tryck den.
- **Snabbreferensen** är ett enkelt fönster med etiketter och en knapp, som går att nå från tangentbordet; Esc stänger det.

Kända brister listas i [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver meddelar inte när en inspelning startar, och Lowspeak har ännu inte testats med VoiceOver eller Reglagestyrning.

## Språk

Menyn, dialogrutorna och notiserna är översatta till 35 språk:

arabiska, katalanska, kinesiska (förenklad), kinesiska (traditionell), kroatiska, tjeckiska, danska, nederländska, engelska, finska, franska, tyska, grekiska, hebreiska, hindi, ungerska, indonesiska, italienska, japanska, koreanska, malajiska, norska, polska, portugisiska (Brasilien), portugisiska (Portugal), rumänska, ryska, slovakiska, slovenska, spanska, svenska, thailändska, turkiska, ukrainska och vietnamesiska.

Det språk du väljer under **Språk** är också menyns språk. Med **Identifiera automatiskt** följer menyn språkordningen under **Systeminställningar › Allmänt › Språk och region**, och engelska används där ingen översättning finns. Menytermerna följer Apples egna översättningar, så Lowspeak låter som resten av din Mac. Utom den engelska är översättningarna maskinöversatta och har ännu inte granskats av personer med språket som modersmål; rättelser är välkomna som [issues](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Tangenter och gester

| Åtgärd | Tangenter |
|---|---|
| Diktera medan tangenten hålls ned | håll ned fn |
| Lås en pågående inspelning | Shift medan du håller ned fn |
| Starta en låst inspelning | Shift+fn, eller fn två gånger snabbt |
| Avsluta en låst inspelning | fn eller Shift |
| Avsluta Lowspeak (med menyn öppen) | ⌘Q |
| Stäng snabbreferensen | Esc |

Lowspeak hindrar de fn-tryck appen använder från att nå andra appar; Shift släpps igenom som vanligt.

## Integritet och säkerhet

Lowspeak hör vad du säger medan fn hålls ned, kan skriva i vilken app som helst och förvarar dina API-nycklar, så appen gör så lite som möjligt med allt detta.

- **Din röst går till ett enda ställe.** I onlineläge går inspelningen, komprimerad, till den leverantör du har valt, OpenAI eller Groq, tillsammans med din ordlista. I lokalt läge lämnar ingenting din Mac. Om en onlineförfrågan misslyckas transkriberar en lokal modell samma inspelning på din Mac.
- **Efterbearbetning**, om du slår på den, skickar den transkriberade texten och din ordlista till Groq eller OpenAI. I läget Formatera skickas också vilken typ av app du dikterar i (kodredigerare, terminal, chatt eller mejl); appens namn skickas inte.
- **Ingen telemetri, inget konto.** Utöver transkribering och efterbearbetning går Lowspeak bara ut på nätet för att öppna anslutningen till din leverantör medan fn hålls ned, i onlineläge eller med efterbearbetning påslagen, för att hämta en modell när du ber om det, från Hugging Face över HTTPS, i en fast version och med varje fil kontrollerad mot sin kontrollsumma, och för att leta efter en ny version på GitHub, en gång om dagen om du tillåter det. Den förfrågan innehåller Lowspeaks version och inget annat. **Hjälp › Rapportera ett problem…** öppnar ett formulär på GitHub i webbläsaren; adressen innehåller versionen av Lowspeak och macOS, inget av det du har dikterat.
- **API-nycklar** ligger kvar i din inloggningsnyckelring. De skickas bara till sin egen leverantör och tas bort ur felmeddelanden innan något skrivs till loggen.
- **Dikterad text sparas inte.** De tio senaste dikteringarna finns i minnet för **Senaste dikteringar** och försvinner när Lowspeak avslutas. Loggfilerna registrerar längder och tider, aldrig texten.
- **Urklippet** sparas före inklistringen och återställs så snart den mottagande appen har läst texten, om inte en annan app har ändrat det under tiden. Dikteringen är markerad som tillfällig, så appar för urklippshistorik som respekterar markeringen hoppar över den.
- **Kodsignering** med Hardened Runtime; appen ber bara om tillgång till mikrofonen, Hjälpmedel, notiser och nätverket. En uppdatering måste bära utvecklarens signatur, annars avvisar Sparkle den.

Om du vill rapportera en sårbarhet följer du [säkerhetspolicyn](../../../SECURITY.md).

## Hastighet

Uppmätt på Apple Silicon:

- **Parakeet Ultra** transkriberar 40 sekunders tal på cirka 0,3 sekunder.
- **Online:** den första förfrågan efter en paus tar cirka 0,4 sekunder hos Groq, eftersom Lowspeak öppnar anslutningen medan du fortfarande talar; senare förfrågningar återanvänder den, över HTTP/3 där leverantören erbjuder det.
- **Uppladdningsstorlek:** cirka 6 KB per sekund tal, som AAC med 48 kbit/s.
- **Att göra mikrofonen redo:** cirka 0,07 sekunder med den inbyggda, en halv sekund med en Studio Display, en till två sekunder med AirPods.

## Kända begränsningar

- **Lokala modeller på Mac-datorer med Intel** körs på processorn och är mycket långsammare; där är onlinetranskribering det bättre valet.
- **Det du säger innan mikrofonen är redo går förlorat.** Vänta på inspelningssymbolen, särskilt med AirPods.
- **fn-tangenten kan tas av andra.** När macOS:s inställning för 🌐-tangenten, Warp eller macOS-diktering lägger beslag på fn först ser Lowspeak den inte; se [Felsökning](#felsökning).
- **Parakeet** ignorerar ordlistan och språkvalet, och kan bara europeiska språk.
- **Skriftsystem och variant:** bara GPT Transcribe kan instrueras att skriva traditionell kinesiska; ingen modell kan instrueras att skriva europeisk snarare än brasiliansk portugisiska.
- **Tidsgränser:** två minuter medan fn hålls ned, fem minuter när inspelningen är låst.
- **Mikrofonläge:** Kontrollcenter erbjuder Röstisolering och Brett spektrum för Lowspeak och visar dem som aktiva, men Lowspeak spelar in utan röstbearbetning, så valet har ingen effekt.
- **Senaste dikteringar** finns bara i minnet och försvinner när appen avslutas.
- **Avsiktligt:** inget inställningsfönster, ingen symbol i Dock, inte i App Store.

## Felsökning

**Ingenting händer när jag håller ned fn.** Kontrollera tre saker: att Lowspeak är tillåtet under **Systeminställningar › Integritet och säkerhet › Hjälpmedel** (i macOS 27: **Enhetskontroll och dataåtkomst**); att **Systeminställningar › Tangentbord › Tryck på 🌐-tangenten för att** är inställt på **göra ingenting**; och att ingen annan app lägger beslag på fn. Warp gör det som standard; stäng av det med `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` och starta om Warp. macOS-diktering kan också vara inställd på att starta med fn.

**Efter en uppdatering reagerar Lowspeak inte längre på fn.** macOS knyter behörigheten för Hjälpmedel till appens signatur. Om signaturen har ändrats, till exempel från ett eget bygge till den hämtade appen, tar du bort Lowspeak från listan Hjälpmedel (i macOS 27: Enhetskontroll och dataåtkomst) och lägger till det igen.

**De första orden saknas.** Börja tala när inspelningssymbolen visas. Med AirPods behöver mikrofonen en till två sekunder; en inbyggd mikrofon eller USB-mikrofon som väljs under **Mikrofon** är redo nästan direkt.

**”Mikrofonen är tyst.”** Mikrofonen levererade inget ljud på fem sekunder. Med AirPods kan en annan enhet använda deras mikrofon; byt till den inbyggda mikrofonen eller anslut dem igen. Ett stängt MacBook-lock stänger också av den inbyggda mikrofonen.

**Dikteringarna blir tomma.** Ingångsvolymen kan vara för låg: appar som Teams, Zoom eller Meet ändrar den under samtal, och macOS sparar den per enhet. Höj den under **Systeminställningar › Ljud › Ljud in**.

**Texten hamnade på fel ställe.** **Senaste dikteringar** i menyn kopierar den till urklippet igen.

**”Ingen API-nyckel.”** Välj **Ange API-nyckel…** i menyn, eller byt till en lokal modell.

**Den första lokala transkriberingen tar lång tid.** Varje modell förbereds för din Mac första gången den används; därefter läses den in på en bråkdel av en sekund.

**Lowspeak avslutas direkt efter start.** En annan kopia av Lowspeak körs redan; avsluta den först.

**Inga notiser visas.** Tillåt dem under **Systeminställningar › Notiser › Lowspeak**.

**Något annat.** Loggarna i `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) beskriver vad som hände, utan din dikterade text. Bifoga de relevanta raderna till en [problemrapport](https://github.com/sventmueller/lowspeak/issues/new/choose).

## För utvecklare

Bygge, testning, kodens uppbyggnad och de regler den följer beskrivs i [docs/development.md](../../../docs/development.md) (på engelska), bidrag i [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Komponenter från tredje part

| Komponent | Version | Används för | Licens |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) via [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | lokal Whisper på GPU:n (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet-avkodning och sammanfogning av långa inspelningar, översatt till Rust i `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) och dess ramverkscrates | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation och andra systemramverk | MIT; Zlib, Apache-2.0 eller MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | mikrofoninspelning via CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | omsampling till 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) med rustls | 3.5 | modellhämtningar | MIT eller Apache-2.0; rustls och ring under ISC och Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | kontrollsummor för modellhämtningarna | MIT eller Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | API-nycklar i nyckelringen | MIT eller Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, fel, Parakeets vokabulär | MIT eller Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | uppdateringar av appen från skivavbilden | MIT, med delar under BSD- och zlib-licenser |

Licenstexterna för allt som ingår i appen levereras med den som `Contents/Resources/Acknowledgements.txt`.

Modellerna ingår inte i Lowspeak; appen hämtar dem när du ber om det:

| Modell | Källa | Licens |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (OpenAIs Whisper i ggml-format) | MIT |
| Parakeet v2, Parakeet Ultra | CoreML-konverteringar av NVIDIAs Parakeet TDT från [FluidInference](https://huggingface.co/FluidInference) | CC-BY-4.0 |

## Licens

Lowspeak är licensierat under någondera av

- Apache-licensen, version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- MIT-licensen ([LICENSE-MIT](../../../LICENSE-MIT))

efter eget val.

Om du inte uttryckligen anger något annat ska varje bidrag som du avsiktligt lämnar in för att ingå i Lowspeak, enligt definitionen i licensen Apache-2.0, dubbellicensieras som ovan, utan ytterligare villkor.
