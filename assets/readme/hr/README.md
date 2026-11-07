<p><img src="../../readme-icon.png" alt="Ikona aplikacije Lowspeak" width="168" height="168"></p>

# Lowspeak

**Diktiranje za Mac koje piše ondje gdje je vaš kursor: držite fn, govorite, otpustite.**

Lowspeak živi u traci s izbornicima. Držite tipku fn (globus) i govorite; kad je otpustite, vaše se riječi pojavljuju u aplikaciji u kojoj upravo pišete, kao da ste ih sami utipkali. Transkribira online putem OpenAI-ja ili Groqa ili u potpunosti na vašem Macu s Whisperom ili NVIDIA Parakeetom, koji 40 sekundi govora pretvara u tekst za otprilike trećinu sekunde. Kad mreža ili online usluga zakaže, preuzima instalirani lokalni model, pa se diktat ne gubi. Naučite ga imenima i pojmovima koje koristite, ako želite, prepustite jezičnom modelu da ukloni poštapalice, a snimanje zaključajte tipkom Shift kako biste dulje tekstove diktirali bez ruku. Napisan je u Rustu na pravim AppKit kontrolama, govori 35 jezika i vaš glas šalje samo pružatelju usluge kojeg ste odabrali.

[![Preuzmite Lowspeak za macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Preuzmite Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: potpisan i ovjeren (notariziran) od Applea, za macOS 14 ili noviji na Apple Siliconu i Intelu.

**Pronašli ste grešku?** [Otvorite issue](https://github.com/sventmueller/lowspeak/issues/new/choose) ili u izborniku Lowspeaka odaberite **Pomoć › Prijavi problem…**: obrazac se otvara s već upisanom vašom verzijom. Ako vam je Lowspeak koristan, [zvjezdica na GitHubu](https://github.com/sventmueller/lowspeak) pomaže drugima da ga pronađu.

[![Rust, izdanje 2024.](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 ili noviji](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Univerzalno: Apple Silicon i Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit preko objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp na Metalu](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet na Neural Engineu](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online putem OpenAI-ja ili Groqa](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 jezika sučelja](https://img.shields.io/badge/languages-35-4C9A2A)
![Potpisano s Hardened Runtimeom](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licenca: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licenca)

[Bahasa Indonesia](../id/README.md) ·
[Bahasa Melayu](../ms/README.md) ·
[Català](../ca/README.md) ·
[Čeština](../cs/README.md) ·
[Dansk](../da/README.md) ·
[Deutsch](../de/README.md) ·
[English](../../../README.md) ·
[Español](../es/README.md) ·
[Français](../fr/README.md) ·
**Hrvatski** ·
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

## Sadržaj

- [Značajke ukratko](#značajke-ukratko)
- [Zahtjevi](#zahtjevi)
- [Instalacija](#instalacija)
- [Početak rada](#početak-rada)
  - [Dozvole](#dozvole)
  - [Odaberite način transkripcije](#odaberite-način-transkripcije)
  - [Izbornik](#izbornik)
  - [Kratki pregled](#kratki-pregled)
- [Diktiranje](#diktiranje)
  - [Držite i govorite](#držite-i-govorite)
  - [Zaključavanje tipkom Shift ili dvostrukim pritiskom](#zaključavanje-tipkom-shift-ili-dvostrukim-pritiskom)
  - [Kamo ide tekst](#kamo-ide-tekst)
  - [Nedavni diktati](#nedavni-diktati)
- [Transkripcija](#transkripcija)
  - [Koji je model pravi za vas](#koji-je-model-pravi-za-vas)
  - [Online: OpenAI i Groq](#online-openai-i-groq)
  - [Na vašem Macu: Whisper i Parakeet](#na-vašem-macu-whisper-i-parakeet)
  - [Kad online usluga zakaže](#kad-online-usluga-zakaže)
  - [Kako se snimka priprema](#kako-se-snimka-priprema)
  - [Jezici po modelu](#jezici-po-modelu)
  - [Macovi s Intelom](#macovi-s-intelom)
- [Rječnik](#rječnik)
- [Naknadna obrada](#naknadna-obrada)
- [Mikrofon](#mikrofon)
- [API ključevi](#api-ključevi)
- [Obavijesti](#obavijesti)
- [Izgled i pristupačnost](#izgled-i-pristupačnost)
- [Jezici](#jezici)
- [Tipke i geste](#tipke-i-geste)
- [Privatnost i sigurnost](#privatnost-i-sigurnost)
- [Brzina](#brzina)
- [Poznata ograničenja](#poznata-ograničenja)
- [Rješavanje problema](#rješavanje-problema)
- [Za programere](#za-programere)
  - [Komponente trećih strana](#komponente-trećih-strana)
- [Licenca](#licenca)

## Značajke ukratko

- **Držite fn, govorite, otpustite.** Tekst se upisuje u aplikaciju u prvom planu, ondje gdje je kursor: e-pošta, chat, uređivač, terminal ili preglednik. Nema prozora na koji treba prijeći ni gumba koji treba kliknuti.
- **Zaključajte ga za dulje tekstove.** Pritisnite Shift dok držite fn ili dvaput pritisnite fn i snimanje se nastavlja bez držanja tipke, do pet minuta. Jedan pritisak na fn ili Shift završava ga.
- **Pravi model za svaki posao.** Groq za brzinu bez trošenja memorije vašeg Maca, OpenAI-jev GPT Transcribe za najbolji rezultat, Whisper ili NVIDIA Parakeet na vašem Macu za rad izvan mreže i glas koji ostaje privatan. Prebacivanje jednim klikom.
- **Nijedan diktat se ne gubi.** Kad mreža ili online usluga zakaže, instalirani lokalni model transkribira istu snimku na vašem Macu.
- **Vaše riječi, ispravno napisane.** Rječnik imena i stručnih pojmova usmjerava OpenAI, Groq, Whisper i naknadnu obradu.
- **Čist tekst ako ga želite.** Neobavezna naknadna obrada uklanja poštapalice, ispravlja gramatiku i, kad se ispravite, zadržava samo ispravak ili tekst dodatno oblikuje prema vrsti aplikacije u koju diktirate.
- **Samo nastavite govoriti.** Započnite sljedeći diktat dok se prethodni još transkribira; tekstovi stižu redoslijedom kojim ste ih izgovorili.
- **Vaš međuspremnik ostaje vaš.** Lowspeak lijepi putem međuspremnika i trenutak kasnije vraća ono što je bilo u njemu. Posljednjih deset diktata ostaje u izborniku da ih ponovno kopirate.
- **Izvorno do srži.** Prava stavka u traci s izbornicima s AppKit izbornikom, standardnim upozorenjima i obavijestima. Bez Electrona, bez prozora, bez ikone u Docku.
- **Privatno.** Bez računa, bez telemetrije. Zvuk ide samo pružatelju usluge kojeg ste odabrali, a u lokalnom načinu rada nikamo.
- **Na vašem jeziku.** Diktiranje radi na desecima jezika, a izbornik, dijalozi i obavijesti dostupni su na 35 jezika.

## Zahtjevi

- macOS 14 Sonoma ili noviji. Lowspeak se razvija i testira na macOSu 27.
- Mac s Apple Silicon ili Intelovim procesorom. Lokalna transkripcija izrađena je za Apple Silicon, gdje Whisper radi na GPU-u, a Parakeet na Neural Engineu; na Macovima s Intelom oba rade na procesoru, znatno sporije, vidi [Macovi s Intelom](#macovi-s-intelom).
- Oko 17 MB za aplikaciju, plus od 75 MB do 3 GB za svaki lokalni model koji preuzmete.
- Za online transkripciju API ključ od [OpenAI-ja](https://platform.openai.com/api-keys) ili [Groqa](https://console.groq.com/keys); oba naplaćuju ono što transkribirate. Lokalnoj transkripciji ne treba račun.

## Instalacija

**Iz slike diska.** Preuzmite [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), otvorite je i povucite Lowspeak na mapu Aplikacije. Slika je potpisana certifikatom Developer ID i ovjerena (notarizirana) od Applea, pa se otvara bez upozorenja.

**S Homebrewom.** Cask se nalazi u vlastitom tapu:

```sh
brew install --cask sventmueller/tap/lowspeak
```

`brew uninstall --zap --cask lowspeak` uklanja i Lowspeakove postavke, modele, zapisnike i predmemoriju; vaši API ključevi ostaju u privjesku ključeva za prijavu.

**Ažuriranja.** Lowspeak iz slike diska ili iz Homebrewa sam se održava ažurnim putem [Sparkla](https://sparkle-project.org). Pri drugom pokretanju pita smije li jednom dnevno sam potražiti ažuriranja. **Potraži ažuriranja…** u izborniku traži odmah; **Automatski traži ažuriranja** i **Automatski preuzmi i instaliraj ažuriranja** ispod nje kasnije mijenjaju vaš odabir. Kad provjera pronađe novu verziju dok radite u drugoj aplikaciji, na prstenu u traci s izbornicima pojavljuje se točka, a **Potraži ažuriranja…** donosi ažuriranje u prvi plan. Lowspeak instalira ažuriranje samo ako nosi potpis razvojnog programera; vaše postavke, ključevi i modeli ostaju. `brew upgrade --cask lowspeak` ažurira i kopiju iz Homebrewa. Verzija 1.0.0 nema automatsko ažuriranje: jednom preko nje instalirajte noviju verziju sa [stranice izdanja](https://github.com/sventmueller/lowspeak/releases). Ni vlastita izgradnja nema automatsko ažuriranje.

**Iz izvornog koda.** Preduvjete potražite u odjeljku [Izgradnja i postavljanje](../../../docs/development.md#build--deploy), a zatim:

```sh
make deploy
```

Naredba izgrađuje Lowspeak, zamjenjuje kopiju u vašoj mapi Aplikacije i pokreće ga.

**Otvori kod prijave** u izborniku pokreće Lowspeak zajedno s vašim Macom, putem stavki za prijavu samog macOS-a. Istodobno radi samo jedna kopija; druga se odmah zatvara.

## Početak rada

### Dozvole

Pri prvom pokretanju Lowspeak traži tri dozvole:

- **Pristupačnost** (Postavke sustava › Privatnost i sigurnost › Pristupačnost; u macOS-u 27 popis se zove Upravljanje uređajem i pristup podacima). Lowspeaku je potrebna kako bi primijetio tipku fn i lijepio pomoću ⌘V. macOS nudi da otvori popis; uključite prekidač za Lowspeak. Lowspeakove poruke navode popis pod imenom koje prikazuje vaš macOS.
- **Mikrofon.** Traži se jednom; bez te dozvole Lowspeak ništa ne snima.
- **Obavijesti.** Lowspeak ih koristi za probleme koje inače ne biste vidjeli, poput mikrofona bez signala ili neuspjele transkripcije.

### Odaberite način transkripcije

Postavite barem jedan način; u izborniku ga možete promijeniti u bilo kojem trenutku.

- **Online:** odaberite **Postavi API ključ…** i zalijepite ključ od OpenAI-ja (počinje sa `sk-`) ili Groqa (`gsk_`). Lowspeak prepoznaje pružatelja usluge po ključu. Zatim odaberite model pod **OpenAI** ili **Groq**.
- **Na vašem Macu:** odaberite **Upravljaj modelima** › **Preuzmi Parakeet Ultra** ili neki Whisper model, a zatim ga odaberite pod **Lokalno**.

[Koji je model pravi za vas](#koji-je-model-pravi-za-vas) pomaže pri odabiru. Zatim držite fn, recite nešto i otpustite.

### Izbornik

Kliknite prsten u traci s izbornicima:

| Stavka | Što radi |
|---|---|
| **Nedavni diktati** | posljednjih deset tekstova; klik ponovno kopira jedan od njih u međuspremnik |
| **Lokalno** | modeli na vašem Macu, grupirani u Whisper i Parakeet; oni koji nisu preuzeti zasivljeni su |
| **OpenAI**, **Groq** | online modeli; bez ključa su zasivljeni, a izbornik nudi **Postavi API ključ…** |
| **Jezik** | jezik na kojem diktirate ili **Automatsko prepoznavanje**; izbornik ga slijedi |
| **Naknadna obrada** | **Isključeno**, **Čišćenje** ili **Oblikovanje**, vidi [Naknadna obrada](#naknadna-obrada) |
| **Rječnik…** | imena i pojmovi koje treba napisati točno; izbornik pokazuje koliko ste ih unijeli |
| **Mikrofon** | **Standardno podešenje sustava** ili određeni mikrofon te **Reproduciraj zvuk kad je spremno** |
| **Upravljaj modelima** | preuzimanje i brisanje lokalnih modela, s prikazom napretka |
| **Postavi API ključ…** / **Promijeni API ključ…** | dodavanje, zamjena ili uklanjanje ključeva |
| **Otvori kod prijave** | pokretanje Lowspeaka zajedno s vašim Macom; kvačica pokazuje da je uključeno |
| **Potraži ažuriranja…** | traženje nove verzije odmah; dvije stavke ispod nje određuju traži li Lowspeak jednom dnevno i instalira li ažuriranja bez pitanja. Samo u aplikaciji iz slike diska |
| **Pomoć** | **Kratki pregled**, **Prijavi problem…**, **Predloži značajku…**, **Lowspeak na GitHubu** |
| **Više o aplikaciji Lowspeak** | verzija, autorska prava, stranica projekta i licenca |
| **Zatvori Lowspeak** | ⌘Q dok je izbornik otvoren |

Kvačica pod **Lokalno**, **OpenAI** i **Groq** označava model koji se koristi. Ikona pokazuje što Lowspeak radi:

| Ikona | Značenje |
|---|---|
| Prsten s točkom | Spreman |
| Ispunjeni krug | Snimanje dok se drži fn |
| Zaobljeni kvadrat | Snimanje zaključano; sljedeći pritisak na fn ili Shift završava ga |
| Prsten koji treperi | Transkribiranje |
| Prsten s točkom gore desno | Čeka nova verzija, vidi **Potraži ažuriranja…** |
| ↓ 42 % | Preuzimanje modela |

Lowspeak nema prozor s postavkama: sve je u ovom izborniku, a on pamti svaki vaš odabir.

### Kratki pregled

**Pomoć › Kratki pregled** prikazuje geste tipke fn u malom prozoru, na vašem jeziku, s gumbom koji otvara postavke tipkovnice. **Prijavi problem…** otvara obrazac na GitHubu s već upisanim verzijama Lowspeaka i macOS-a, **Predloži značajku…** raspravu za ideje, a **Lowspeak na GitHubu** stranicu projekta.

## Diktiranje

### Držite i govorite

Držite fn, govorite, otpustite. Ikona snimanja pojavljuje se u trenutku kad mikrofon počne isporučivati zvuk; tada počnite govoriti jer se ono što kažete prije toga ne snima. Otpuštanje tipke fn završava snimanje, a tekst slijedi trenutak kasnije.

Kratak dodir tipke fn ne radi ništa, pa fn zadržava svoje druge namjene. Snimka koja ostane prazna ili traje kraće od 0,3 sekunde odbacuje se.

### Zaključavanje tipkom Shift ili dvostrukim pritiskom

Za dulje tekstove zaključajte snimanje kako bi radilo bez držanja tipke. Postoje tri načina:

- **Shift dok držite fn.** Počnite s fn kao i obično i tijekom snimanja pritisnite Shift: snimanje se zaključava i možete otpustiti fn. Ništa što ste rekli prije toga ne gubi se, snimanje se jednostavno nastavlja.
- **Shift, zatim fn.** Držite Shift i pritisnite fn: snimanje počinje zaključano.
- **Dvaput fn.** Kratko dodirnite fn (kraće od 0,3 sekunde) i ponovno ga pritisnite unutar 0,4 sekunde. Snimanje teče od prvog pritiska.

Ikona se pretvara u zaobljeni kvadrat. Za zaustavljanje jednom pritisnite **fn** ili **Shift**; tekst slijedi kao i obično. Ako je vaša tipka 🌐 postavljena da otvara Emoji i simboli, zaustavljajte tipkom Shift: kratak pritisak na fn otvorio bi taj prozor.

Snimanje se samo zaustavlja nakon dvije minute dok se drži fn, odnosno nakon pet minuta kad je zaključano, a zatim se transkribira kao i obično.

### Kamo ide tekst

Lowspeak lijepi tekst u aplikaciju koja je bila u prvom planu kad ste prestali govoriti, na mjesto kursora, putem međuspremnika i ⌘V. Sprema sve što je prije bilo u međuspremniku i vraća to pola sekunde nakon što aplikacija pročita tekst ili nakon deset sekundi ako ga nikad ne pročita, osim ako je neka druga aplikacija u međuvremenu promijenila međuspremnik. Tekst je označen kao privremen, pa ga aplikacije za povijest međuspremnika koje poštuju tu oznaku ne bilježe.

Sljedeći diktat možete započeti dok se prethodni još transkribira. Tekstovi se lijepe redoslijedom kojim ste ih izgovorili, i nikad dok snimate ili držite Shift, pa lijepljenje ne može završiti usred vaše sljedeće rečenice.

Bez dozvole za pristupačnost ili kad se simulirani ⌘V ne može poslati, tekst ostaje u međuspremniku, a obavijest to javlja; zalijepite ga sami pomoću ⌘V.

### Nedavni diktati

**Nedavni diktati** u izborniku navodi posljednjih deset tekstova, najnoviji prvi, svaki skraćen na jedan redak; kad zadržite pokazivač iznad stavke, prikazuje se cijeli tekst. Klik ga kopira u međuspremnik, za slučaj kad je tekst završio na pogrešnom mjestu ili vam treba dvaput. Popis postoji samo u memoriji i prazan je nakon zatvaranja Lowspeaka.

## Transkripcija

### Koji je model pravi za vas

| Želite | Odaberite | Zašto |
|---|---|---|
| **Brzinu bez opterećenja Maca** | **Groq › Whisper Large V3 Turbo** | najbrža online opcija; ništa se ne preuzima, a vaš Mac zadržava svoju memoriju i prostor na disku |
| **Najbolji rezultat** | **OpenAI › GPT Transcribe** (zadano) | najtočniji od modela koje Lowspeak nudi; vaš rječnik prima kao ključne riječi |
| **Izvan mreže i privatno** | **Lokalno › Parakeet Ultra** za europske jezike, **Lokalno › Whisper Large v3 Turbo** za sve ostale | radi bez mreže, a vaš glas nikad ne napušta vaš Mac; Parakeet je najbrži od svih |
| **Bez računa, bez troškova** | bilo koji model pod **Lokalno** | radi na vašem Macu; ništa se ne naplaćuje |
| **I pouzdanost i brzinu** | online model, uz instaliran Parakeet Ultra ili neki Whisper model | lokalni model uskače kad online usluga zakaže |
| **Mac s Intelom** | **Groq** ili **OpenAI** | lokalni modeli ondje rade na procesoru, vidi [Macovi s Intelom](#macovi-s-intelom) |

Lokalni modeli zauzimaju na disku onoliko koliko iznosi njihovo preuzimanje i, dok su odabrani, otprilike jednako toliko memorije; Lowspeak drži model učitanim kako bi diktiranje počelo bez odgode. Na Macu s 8 GB memorije radije odaberite Parakeet Ultra ili online model nego velike Whisper modele.

### Online: OpenAI i Groq

| Model | Pružatelj usluge | Napomene |
|---|---|---|
| **GPT Transcribe** (zadano) | OpenAI | najtočniji; vaš rječnik prima kao ključne riječi |
| **Whisper** | OpenAI | izvorni model Whisper |
| **Whisper Large V3** | Groq | vrlo brz |
| **Whisper Large V3 Turbo** | Groq | najbrži online |

Snimka se prije slanja komprimira u AAC, otprilike 6 KB po sekundi govora, pa diktiranje radi i na sporim vezama. Dok držite fn, Lowspeak već otvara vezu s pružateljem usluge, pa slanje počinje u trenutku kad otpustite tipku.

### Na vašem Macu: Whisper i Parakeet

| Model | Veličina | Radi na | Jezici |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 europskih jezika |
| **Parakeet v2** | ~600 MB | Neural Engine | engleski |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | oko 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | oko 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | oko 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | oko 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | oko 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | oko 99 |

**Upravljaj modelima** ih preuzima i briše; tijekom preuzimanja traka s izbornicima prikazuje napredak, a zatvaranje Lowspeaka prekida preuzimanje. Prva upotreba modela priprema ga za vaš Mac: oko 10 sekundi za Parakeet Ultra, do 30 sekundi za Whisper; nakon toga učitava se u djeliću sekunde. Ako se odabrani model obriše, Lowspeak prelazi na neki drugi instalirani model.

**Parakeet** je najbrži izbor: 40 sekundi govora transkribira za oko 0,3 sekunde. Lowspeak ga pokreće s istim modelima i dekodiranjem kao [FluidAudio](https://github.com/FluidInference/FluidAudio) i sprema ih u FluidAudiovu mapu (`~/Library/Application Support/FluidAudio/Models`), pa ih aplikacije izgrađene na FluidAudiju dijele. Snimke dulje od 15 sekundi transkribiraju se u prozorima koji se preklapaju i spajaju bez gubitka riječi na spojevima. **Whisper** razumije više jezika i čita vaš rječnik; veliki modeli su najtočniji, a mali su izvan engleskog znatno slabiji. Whisper modeli spremaju se u `~/Library/Application Support/Lowspeak/Models`.

### Kad online usluga zakaže

Ako online transkripcija ne uspije, jer mreža ne radi, pružatelj usluge ima problem ili je ključ pogrešan, instalirani lokalni model transkribira istu snimku: onaj odabran pod **Lokalno**, inače Parakeet Ultra, zatim Whisper Large v3 Turbo, Large v3 i manji Whisper modeli, a Parakeet v2 posljednji. Obavijest vam to javlja jednom; ponovno se pojavljuje tek nakon što u međuvremenu uspije neki online diktat.

Uz lokalni model kao rezervu, Lowspeak online zahtjevu daje 20 sekundi; bez njega pokušava triput, svaki put do jedne minute. U online načinu rada Lowspeak rezervni model Parakeet priprema jednom pri pokretanju, pa prvo prebacivanje na rezervu ne mora čekati na njega.

### Kako se snimka priprema

Lowspeak snima u izvornoj frekvenciji uzorkovanja vašeg mikrofona i, čim prestanete, snimku pretvara u 16 kHz koje modeli očekuju. Prije transkripcije uklanja duboko brujanje ispod 80 Hz, odrezuje tišinu na početku i na kraju te ujednačuje glasnoću, pa tihi i glasni govornici zvuče jednako. Snimka bez govora uopće se ne šalje. U lokalnom načinu rada Parakeet najprije dobiva neobrađenu snimku, a pripremljenu samo ako ona ne da tekst.

### Jezici po modelu

Jezik odabran pod **Jezik** prosljeđuje se online modelima i Whisperu. S **Automatsko prepoznavanje** model sam određuje jezik.

| Model | Jezici na kojima možete diktirati |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, na vašem Macu) | oko 99, među njima svih 35 jezika izbornika |
| Parakeet Ultra | bugarski, hrvatski, češki, danski, nizozemski, engleski, estonski, finski, francuski, njemački, grčki, mađarski, talijanski, latvijski, litavski, malteški, poljski, portugalski, rumunjski, ruski, slovački, slovenski, španjolski, švedski, ukrajinski |
| Parakeet v2 | engleski |

Parakeet sam prepoznaje jezik i zanemaruje odabir u izborniku. **Kineski (tradicionalni)** nalaže GPT Transcribeu da piše tradicionalnim znakovima; Whisper kineski piše bilo kojim od dvaju pisama, a portugalski bez obzira na varijantu.

### Macovi s Intelom

Lowspeak je univerzalna aplikacija i radi i na Macovima s Intelom s macOS-om 14. Sve online funkcije rade jednako. Lokalni modeli ondje nemaju hardver za koji su izrađeni: Whisper radi na procesoru umjesto na GPU-u, a Parakeet na procesoru umjesto na Neural Engineu. Oba rade, ali traju višestruko dulje nego na Apple Siliconu, a veliki Whisper modeli predugo za diktiranje. Na Macu s Intelom transkribirajte online, a Whisper Small ili Parakeet Ultra zadržite kao [rezervu](#kad-online-usluga-zakaže).

## Rječnik

**Rječnik…** prima imena i pojmove koje Lowspeak treba napisati točno, odvojene zarezima: nazive proizvoda, imena kolega, stručne pojmove. Do 100 pojmova od najviše 60 znakova; `<` i `>` uklanjaju se.

- **GPT Transcribe** prima ih kao ključne riječi.
- **Whisper** (OpenAI, Groq i na vašem Macu) čita ih kao natuknicu prije snimke, do 500 znakova.
- **Naknadnoj obradi** nalaže se da ih napiše točno onako kako su zapisani.
- **Parakeet** ih ne koristi.

## Naknadna obrada

Nakon transkripcije jezični model može preraditi tekst. Odaberite pod **Naknadna obrada**:

- **Isključeno** (zadano) – tekst onakav kakav je transkribiran.
- **Čišćenje** – uklanja poštapalice kao što su „ovaj”, „znaš” ili „zapravo”, ispravlja gramatiku i interpunkciju i, kad se ispravite, zadržava samo ispravak. Ništa ne dodaje i ne mijenja značenje.
- **Oblikovanje** – kao Čišćenje, a uz to prilagođava tekst aplikaciji u koju diktirate: pojmovi u stilu koda u uređivačima koda, naredbeni retci u terminalima, opušten ton u aplikacijama za chat, profesionalan u e-pošti.

Naknadna obrada koristi Groq (`openai/gpt-oss-120b`) ako imate ključ za Groq, inače OpenAI (`gpt-4o-mini`), i kad transkribirate lokalno. Ako je pod **Jezik** odabran jezik, zadržava taj jezik. Ako traje dulje od osam sekundi, ne uspije ili vrati odgovor koji je odsječen ili mnogo dulji od onoga što ste rekli, Lowspeak lijepi neobrađeni tekst. Ako pružatelj usluge odbije zahtjev ili ključ nije postavljen, to jednom javlja obavijest.

## Mikrofon

Pod **Mikrofon** odaberite određeni mikrofon ili **Standardno podešenje sustava**. Lowspeak pamti uređaj; dok nije povezan, izbornik ga prikazuje kao **(nije povezan)**, a Lowspeak snima sa standardnim mikrofonom sustava.

Mikrofon se uključuje samo dok diktirate. Njegova priprema traje oko 0,07 sekundi s ugrađenim mikrofonom, oko pola sekunde sa Studio Displayjem te jednu do dvije sekunde s AirPods slušalicama, koje se najprije prebacuju u način rada slušalica s mikrofonom. Ikona snimanja pojavljuje se čim zvuk stigne; **Reproduciraj zvuk kad je spremno** u tom trenutku dodaje kratak zvuk. Standardno je isključeno jer na nekim zvučnicima sam zvuk kasni pola sekunde.

Ako mikrofon pet sekundi isporučuje samo tišinu, kao AirPods slušalice kad njihov mikrofon drži neki drugi uređaj, Lowspeak vam to javlja.

## API ključevi

**Postavi API ključ…** traži ključ i prepoznaje pružatelja usluge po njegovu početku: `sk-` za OpenAI, `gsk_` za Groq. Ključ u drugom formatu odbija se uz objašnjenje. Unesite po jedan ključ za svakog pružatelja usluge; s ključevima za oba dostupni su svi online modeli. **Promijeni API ključ…** zamjenjuje ključ ili ih, pomoću **Ukloni sve ključeve**, briše.

Ključevi se spremaju u vaš privjesak ključeva za prijavu pod nazivom `com.lowspeak.app`. Datoteka s ključevima iz ranije verzije (`~/Library/Application Support/Lowspeak/.api-key`) premješta se onamo pri sljedećem pokretanju i briše čim se svaki ključ može ponovno pročitati iz privjeska ključeva.

## Obavijesti

Lowspeak radi u pozadini, pa probleme javlja kao obavijesti:

| Obavijest | Što se dogodilo |
|---|---|
| **Snimanje nije uspjelo** | nema pristupa mikrofonu, nijedan mikrofon nije priključen ili se mikrofon nije mogao pokrenuti |
| **Mikrofon bez signala** | pet sekundi tišine, vidi [Mikrofon](#mikrofon) |
| **Potrebna je dozvola za pristupačnost** | tekst je u međuspremniku; dopustite Pristupačnost kako bi se lijepio automatski |
| **Lijepljenje nije uspjelo** | tekst je u međuspremniku; zalijepite ga pomoću ⌘V |
| **Transkripcija nije uspjela** | s razlogom: nema ključa, ključ je odbijen, prekoračeno ograničenje zahtjeva, pogreška poslužitelja ili mreže ili nema lokalnog modela |
| **Transkribirano na ovom Macu** | online usluga je zakazala i preuzeo je lokalni model |
| **Naknadna obrada nije uspjela** | pružatelj usluge odbija naknadnu obradu ili ključ nije postavljen; tekst je zalijepljen bez nje |
| **Nema API ključa** | odabrali ste online model bez ključa za njegova pružatelja usluge |
| **Preuzimanje nije uspjelo**, **Brisanje nije uspjelo**, **Brisanje nije moguće** | upravljanje lokalnim modelom nije uspjelo ili je još bila u tijeku druga radnja |
| **Pogreška stavke za prijavu**, **Pogreška API ključa** | macOS ili privjesak ključeva odbili su promjenu |

Obavijesti nikad ne sadrže vaš diktirani tekst. Online usluga koja zakazuje i pružatelj naknadne obrade koji odbija zahtjeve javljaju se jednom, a ne nakon svakog diktata, sve dok ponovno ne prorade.

## Izgled i pristupačnost

- **Ikona u traci s izbornicima** predložak je slike koji prati svijetlu, tamnu ili obojenu traku s izbornicima.
- **VoiceOver** čita ikonu kao „Lowspeak”, a izbornik, upozorenja i obavijesti standardne su macOS kontrole.
- **Nijednu tipku ne treba držati.** Zaključajte snimanje tipkom Shift ili dvostrukim pritiskom na fn, a sljedeći pritisak ga završava.
- **Kratki pregled** je jednostavan prozor s oznakama i gumbom, dostupan s tipkovnice; Esc ga zatvara.

Poznati nedostaci navedeni su u [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver ne najavljuje početak snimanja, a Lowspeak još nije provjeren s VoiceOverom ni s Kontrolom prekidača.

## Jezici

Izbornik, dijalozi i obavijesti prevedeni su na 35 jezika:

arapski, katalonski, kineski (pojednostavljeni), kineski (tradicionalni), hrvatski, češki, danski, nizozemski, engleski, finski, francuski, njemački, grčki, hebrejski, hindski, mađarski, indonezijski, talijanski, japanski, korejski, malajski, norveški, poljski, portugalski (Brazil), portugalski (Portugal), rumunjski, ruski, slovački, slovenski, španjolski, švedski, tajlandski, turski, ukrajinski i vijetnamski.

Jezik koji odaberete pod **Jezik** ujedno je i jezik izbornika. S **Automatsko prepoznavanje** izbornik slijedi redoslijed jezika u **Postavke sustava › Općenito › Jezik i regija**, a engleski ondje gdje prijevod ne postoji. Izrazi u izborniku slijede Appleove vlastite prijevode, pa se Lowspeak čita kao ostatak vašeg Maca. Osim engleskog, prijevodi su strojni i izvorni govornici ih još nisu pregledali; ispravci su dobrodošli kao [issue na GitHubu](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Tipke i geste

| Radnja | Tipke |
|---|---|
| Diktiranje dok se drži | držite fn |
| Zaključavanje snimanja u tijeku | Shift dok držite fn |
| Pokretanje zaključanog snimanja | Shift+fn ili dvaput brzo fn |
| Završetak zaključanog snimanja | fn ili Shift |
| Zatvaranje Lowspeaka (izbornik otvoren) | ⌘Q |
| Zatvaranje kratkog pregleda | Esc |

Lowspeak sprječava da pritisci tipke fn koje koristi stignu do drugih aplikacija; Shift prolazi kao i obično.

## Privatnost i sigurnost

Lowspeak čuje što govorite dok se drži fn, može pisati u bilo koju aplikaciju i čuva vaše API ključeve, pa s time radi što je manje moguće.

- **Vaš glas ide na jedno mjesto.** U online načinu rada snimka, komprimirana, ide pružatelju usluge kojeg ste odabrali, OpenAI-ju ili Groqu, zajedno s vašim rječnikom. U lokalnom načinu rada ništa ne napušta vaš Mac. Kad online zahtjev ne uspije, lokalni model transkribira istu snimku na vašem Macu.
- **Naknadna obrada**, ako je uključite, šalje transkribirani tekst i vaš rječnik Groqu ili OpenAI-ju. U načinu Oblikovanje dodaje vrstu aplikacije u koju diktirate (uređivač koda, terminal, chat ili e-pošta); naziv aplikacije ne šalje se.
- **Bez telemetrije, bez računa.** Osim za transkripciju i naknadnu obradu, Lowspeak se spaja na mrežu samo kako bi otvorio vezu s vašim pružateljem usluge dok se drži fn, u online načinu rada ili s uključenom naknadnom obradom, kako bi preuzeo model kad to zatražite, s Hugging Facea preko HTTPS-a, u fiksnoj verziji i uz provjeru kontrolnog zbroja svake datoteke, te kako bi na GitHubu potražio novu verziju, jednom dnevno ako to dopustite. Taj zahtjev sadrži verziju Lowspeaka i ništa drugo. **Pomoć › Prijavi problem…** otvara obrazac na GitHubu u vašem pregledniku; njegova adresa sadrži verziju Lowspeaka i verziju macOS-a, ništa od onoga što ste diktirali.
- **API ključevi** ostaju u vašem privjesku ključeva za prijavu. Šalju se samo vlastitom pružatelju usluge i uklanjaju se iz poruka o pogreškama prije nego što se išta zapiše u zapisnik.
- **Diktirani tekst ne sprema se.** Posljednjih deset diktata ostaje u memoriji za **Nedavni diktati** i nestaje kad se Lowspeak zatvori. Zapisnici bilježe duljine i vremena, nikad tekst.
- **Međuspremnik** se sprema prije lijepljenja i vraća čim aplikacija pročita tekst, osim ako ga je u međuvremenu promijenila neka druga aplikacija. Diktat je označen kao privremen, pa ga aplikacije za povijest međuspremnika koje poštuju tu oznaku preskaču.
- **Potpisivanje koda** s Hardened Runtimeom; aplikacija traži samo mikrofon, Pristupačnost, obavijesti i mrežu. Ažuriranje mora nositi potpis razvojnog programera, inače ga Sparkle odbija.

Za prijavu ranjivosti slijedite [sigurnosna pravila](../../../SECURITY.md).

## Brzina

Izmjereno na Apple Siliconu:

- **Parakeet Ultra** transkribira 40 sekundi govora za oko 0,3 sekunde.
- **Online:** prvi zahtjev nakon stanke traje oko 0,4 sekunde kod Groqa jer Lowspeak otvara vezu dok još govorite; kasniji zahtjevi ponovno je koriste, preko HTTP/3 ondje gdje ga pružatelj usluge nudi.
- **Veličina slanja:** oko 6 KB po sekundi govora, kao AAC pri 48 kbit/s.
- **Priprema mikrofona:** oko 0,07 sekundi za ugrađeni, pola sekunde na Studio Displayju, jedna do dvije sekunde s AirPods slušalicama.

## Poznata ograničenja

- **Lokalni modeli na Macovima s Intelom** rade na procesoru i znatno su sporiji; ondje je online transkripcija bolji izbor.
- **Ono što kažete prije nego što je mikrofon spreman gubi se.** Pričekajte ikonu snimanja, osobito s AirPods slušalicama.
- **Tipku fn mogu preuzeti drugi.** Kad postavka tipke 🌐 u macOS-u, Warp ili Diktat u macOS-u prvi zauzmu fn, Lowspeak je ne vidi; vidi [Rješavanje problema](#rješavanje-problema).
- **Parakeet** zanemaruje rječnik i odabir jezika te poznaje samo europske jezike.
- **Pismo i varijanta:** samo GPT Transcribeu može se zadati da piše tradicionalni kineski; nijednom modelu ne može se zadati da piše europski umjesto brazilskog portugalskog.
- **Vremenska ograničenja:** dvije minute dok se drži fn, pet minuta kad je snimanje zaključano.
- **Način rada mikrofona:** Kontrolni centar za Lowspeak nudi Izoliranje glasa i Široki spektar i prikazuje ih kao aktivne, ali Lowspeak snima bez obrade glasa, pa odabir nema učinka.
- **Nedavni diktati** postoje samo u memoriji i nestaju nakon zatvaranja.
- **Namjerno:** bez prozora s postavkama, bez ikone u Docku, nije u App Storeu.

## Rješavanje problema

**Ništa se ne događa kad držim fn.** Provjerite tri stvari: Lowspeak je dopušten pod **Postavke sustava › Privatnost i sigurnost › Pristupačnost** (u macOS-u 27: **Upravljanje uređajem i pristup podacima**); **Postavke sustava › Tipkovnica › Pritisnite tipku 🌐 za** postavljeno je na **Ništa**; i nijedna druga aplikacija ne zauzima fn. Warp to radi prema zadanim postavkama; isključite to naredbom `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` i ponovno pokrenite Warp. I Diktat u macOS-u može se postaviti da se pokreće tipkom fn.

**Nakon ažuriranja Lowspeak više ne reagira na fn.** macOS veže dozvolu za pristupačnost uz potpis aplikacije. Ako se potpis promijenio, primjerice s vlastite izgradnje na preuzetu aplikaciju, uklonite Lowspeak s popisa Pristupačnost (u macOS-u 27: Upravljanje uređajem i pristup podacima) i ponovno ga dodajte.

**Nedostaju prve riječi.** Počnite govoriti kad se pojavi ikona snimanja. S AirPods slušalicama mikrofonu treba jedna do dvije sekunde; ugrađeni ili USB mikrofon odabran pod **Mikrofon** spreman je gotovo odmah.

**„Mikrofon bez signala.”** Mikrofon pet sekundi nije isporučio zvuk. Kod AirPods slušalica njihov mikrofon možda koristi neki drugi uređaj; prijeđite na ugrađeni mikrofon ili ih ponovno povežite. Zatvoren poklopac MacBooka također utišava ugrađeni mikrofon.

**Diktati su prazni.** Glasnoća ulaza možda je preniska: aplikacije kao što su Teams, Zoom ili Meet mijenjaju je tijekom poziva, a macOS je pamti zasebno za svaki uređaj. Pojačajte je pod **Postavke sustava › Zvuk › Unos**.

**Tekst je završio na pogrešnom mjestu.** **Nedavni diktati** u izborniku ponovno ga kopiraju u međuspremnik.

**„Nema API ključa.”** U izborniku odaberite **Postavi API ključ…** ili prijeđite na lokalni model.

**Prva lokalna transkripcija traje dugo.** Svaki se model pri prvoj upotrebi priprema za vaš Mac; od tada se učitava u djeliću sekunde.

**Lowspeak se zatvara odmah nakon pokretanja.** Već radi druga kopija Lowspeaka; najprije zatvorite nju.

**Ne pojavljuju se obavijesti.** Dopustite ih pod **Postavke sustava › Obavijesti › Lowspeak**.

**Nešto drugo.** Zapisnici u `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) opisuju što se dogodilo, bez vašeg diktiranog teksta. Priložite relevantne retke [prijavi problema](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Za programere

Izgradnja, testiranje, struktura koda i pravila koja slijedi opisani su u [docs/development.md](../../../docs/development.md), a doprinosi u [CONTRIBUTING.md](../../../CONTRIBUTING.md) (oba na engleskom).

### Komponente trećih strana

| Komponenta | Verzija | Namjena | Licenca |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) preko [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | lokalni Whisper na GPU-u (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | dekodiranje Parakeeta i spajanje dugih snimki, prevedeno u Rust u `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) i njegovi crateovi za frameworke | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation i drugi sistemski frameworki | MIT; Zlib, Apache-2.0 ili MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | snimanje mikrofona putem CoreAudija | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | preuzorkovanje na 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) s rustls | 3.5 | preuzimanje modela | MIT ili Apache-2.0; rustls i ring pod ISC i Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | kontrolni zbrojevi preuzetih modela | MIT ili Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | API ključevi u privjesku ključeva | MIT ili Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, pogreške, vokabular Parakeeta | MIT ili Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | ažuriranja aplikacije iz slike diska | MIT, s dijelovima pod licencama BSD i zlib |

Licencni tekstovi svega što se nalazi u aplikaciji isporučuju se s njom kao `Contents/Resources/Acknowledgements.txt`.

Modeli nisu dio Lowspeaka; preuzima ih kad to zatražite:

| Model | Izvor | Licenca |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (OpenAI-jev Whisper u formatu ggml) | MIT |
| Parakeet v2, Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference), CoreML pretvorbe NVIDIA-ina modela Parakeet TDT | CC-BY-4.0 |

## Licenca

Lowspeak je licenciran pod jednom od sljedećih licenci

- Licenca Apache, verzija 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licenca MIT ([LICENSE-MIT](../../../LICENSE-MIT))

po vašem izboru.

Osim ako izričito ne navedete drukčije, svaki doprinos koji ste namjerno poslali za uključivanje u Lowspeak, kako je definiran u licenci Apache-2.0, dvostruko je licenciran kako je gore navedeno, bez dodatnih uvjeta.
