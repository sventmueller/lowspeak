<p><img src="../../readme-icon.png" alt="Ikona aplikacije Lowspeak" width="168" height="168"></p>

# Lowspeak

**Pridržite fn, govorite, spustite. Vaše besede so že tam.**

Lowspeak živi v menijski vrstici in posluša samo, dokler držite tipko fn. Povejte, kar želite napisati, spustite jo in besedilo se pojavi v aplikaciji, v kateri ste: v pošti, klepetu, urejevalniku kode, terminalu.

To je prava aplikacija za Mac, napisana v jeziku Rust na AppKitu, brez brskalniškega pogona pod pokrovom. Zažene se takoj in vam ne hodi v napoto. Povezavo odpre celo že, ko še govorite, zato besedilo s spletnega modela prispe približno 0,4 sekunde po tem, ko spustite tipko.

Izbirate lahko med dvanajstimi modeli za prepoznavanje govora. Z lastnim ključem Groq vas spletni prepis v okviru dnevne kvote pri Groqu ne stane nič; zahtevne posnetke prevzame GPT Transcribe podjetja OpenAI. Lowspeak se z njimi poveže neposredno, brez računa in brez lastnega strežnika vmes.

Ali pa vse obdržite na svojem Macu. Whisper in NVIDIA Parakeet delujeta brez povezave, zato vaš glas nikoli ne zapusti računalnika, Parakeet pa 40 sekund govora spremeni v besedilo v približno tretjini sekunde. Če delate na spletu in omrežje odpove, na pomoč priskoči nameščeni lokalni model, zato se noben narek ne izgubi.

Lowspeak govori 35 jezikov. Nauči se imen in izrazov, ki jih uporabljate, zna odstraniti mašila in zgladiti slovnico. Zadnjih deset narekov hrani v meniju, če bi katerega spet potrebovali.

[![Prenesite Lowspeak za macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Prenesite Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: podpisan in overjen pri Applu, za macOS 14 ali novejši na Applovih čipih in procesorjih Intel.

**Ste našli napako?** [Odprite issue](https://github.com/sventmueller/lowspeak/issues/new/choose) ali v meniju aplikacije Lowspeak izberite **Pomoč › Prijavi težavo…**: obrazec se odpre z že vpisano vašo različico. Če vam je Lowspeak koristen, [zvezdica na GitHubu](https://github.com/sventmueller/lowspeak) pomaga drugim, da ga najdejo.

[![Izdaja Rust 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 ali novejši](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Univerzalna aplikacija: Applovi čipi in Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit prek objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp na Metalu](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet na enoti Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Na spletu prek OpenAI ali Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 jezikov vmesnika](https://img.shields.io/badge/languages-35-4C9A2A)
![Podpisan s Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
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
**Slovenščina** ·
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

## Vsebina

- [Zmožnosti na prvi pogled](#zmožnosti-na-prvi-pogled)
- [Zahteve](#zahteve)
- [Namestitev](#namestitev)
- [Prvi koraki](#prvi-koraki)
  - [Dovoljenja](#dovoljenja)
  - [Izberite način prepisovanja](#izberite-način-prepisovanja)
  - [Meni](#meni)
  - [Hitri pregled](#hitri-pregled)
- [Narekovanje](#narekovanje)
  - [Držite in govorite](#držite-in-govorite)
  - [Zaklepanje s tipko Shift ali dvojnim pritiskom](#zaklepanje-s-tipko-shift-ali-dvojnim-pritiskom)
  - [Kam gre besedilo](#kam-gre-besedilo)
  - [Nedavni nareki](#nedavni-nareki)
- [Prepisovanje](#prepisovanje)
  - [Kateri model je pravi za vas](#kateri-model-je-pravi-za-vas)
  - [Na spletu: OpenAI in Groq](#na-spletu-openai-in-groq)
  - [Na vašem Macu: Whisper in Parakeet](#na-vašem-macu-whisper-in-parakeet)
  - [Ko spletna storitev odpove](#ko-spletna-storitev-odpove)
  - [Kako se posnetek pripravi](#kako-se-posnetek-pripravi)
  - [Jeziki po modelih](#jeziki-po-modelih)
  - [Maci s procesorjem Intel](#maci-s-procesorjem-intel)
- [Besedišče](#besedišče)
- [Naknadna obdelava](#naknadna-obdelava)
- [Mikrofon](#mikrofon)
- [Ključi API](#ključi-api)
- [Obvestila](#obvestila)
- [Videz in dostopnost](#videz-in-dostopnost)
- [Jeziki](#jeziki)
- [Tipke in kretnje](#tipke-in-kretnje)
- [Zasebnost in varnost](#zasebnost-in-varnost)
- [Hitrost](#hitrost)
- [Znane omejitve](#znane-omejitve)
- [Odpravljanje težav](#odpravljanje-težav)
- [Za razvijalce](#za-razvijalce)
  - [Komponente tretjih oseb](#komponente-tretjih-oseb)
- [Licenca](#licenca)

## Zmožnosti na prvi pogled

- **Domač na Macu skozi in skozi.** Pravi element menijske vrstice z menijem AppKit, standardnimi opozorili in obvestili. Brez Electrona, brez okna, brez ikone v Docku.
- **Pravi model za vsako nalogo.** Groq za hitrost brez porabe pomnilnika vašega Maca, GPT Transcribe podjetja OpenAI za najboljši rezultat, Whisper ali NVIDIA Parakeet na vašem Macu za delo brez povezave in glas, ki ostane zaseben. Preklop z enim klikom.
- **Zasebno.** Brez računa, brez telemetrije. Zvok gre samo k ponudniku, ki ste ga izbrali, v lokalnem načinu pa nikamor.
- **Noben narek se ne izgubi.** Ko omrežje ali spletna storitev odpove, nameščeni lokalni model isti posnetek prepiše na vašem Macu.
- **V vašem jeziku.** Narekovanje deluje v desetinah jezikov, meni, pogovorna okna in obvestila pa so na voljo v 35 jezikih.
- **Vaše besede, pravilno zapisane.** Besedišče imen in strokovnih izrazov usmerja OpenAI, Groq, Whisper in naknadno obdelavo ter popravlja, kako jih zapiše Parakeet.
- **Čisto besedilo, če ga želite.** Izbirna naknadna obdelava odstrani mašila, popravi slovnico in, ko se popravite, obdrži samo popravek ali pa besedilo dodatno oblikuje glede na vrsto aplikacije, v katero narekujete.
- **Vaše odložišče ostane vaše.** Lowspeak lepi prek odložišča in trenutek pozneje vrne, kar je bilo v njem. Zadnjih deset narekov ostane v meniju, da jih lahko znova kopirate.
- **Pridržite fn, govorite, spustite.** Besedilo se natipka v aplikacijo v ospredju, kjer koli je kazalka: pošta, klepet, urejevalnik, terminal ali brskalnik. Nobenega okna, na katero bi morali preklopiti, nobenega gumba, ki bi ga morali klikniti.
- **Zaklenite ga za daljša besedila.** Pritisnite Shift, medtem ko držite fn, ali dvakrat pritisnite fn in snemanje teče naprej brez pridržane tipke, do pet minut. En pritisk na fn ali Shift ga konča.
- **Kar govorite naprej.** Naslednji narek začnite, medtem ko se prejšnji še prepisuje; besedila prispejo v vrstnem redu, v katerem ste jih izgovorili.

## Zahteve

- macOS 14 Sonoma ali novejši. Lowspeak je razvit in preizkušen v macOS 27.
- Mac z Applovim čipom ali procesorjem Intel. Lokalni prepis je zasnovan za Applove čipe, kjer Whisper teče na grafičnem procesorju, Parakeet pa na enoti Neural Engine; na Macih s procesorjem Intel oba tečeta na procesorju, precej počasneje, glejte [Maci s procesorjem Intel](#maci-s-procesorjem-intel).
- Približno 17 MB za aplikacijo ter od 75 MB do 3 GB za vsak lokalni model, ki ga prenesete.
- Za spletni prepis ključ API pri [OpenAI](https://platform.openai.com/api-keys) ali [Groq](https://console.groq.com/keys). OpenAI zaračunava, kar prepišete; brezplačni paket Groq v okviru dnevnih omejitev ne stane nič, omejitve pa vidite v konzoli Groq. Lokalni prepis ne potrebuje računa.

## Namestitev

**Z diskovne slike.** Prenesite [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), jo odprite in povlecite Lowspeak v mapo Aplikacije. Slika je podpisana z ID-jem razvijalca in overjena pri Applu, zato se odpre brez opozoril.

**S Homebrewom.** Cask je v lastnem tapu:

```sh
brew install --cask sventmueller/tap/lowspeak
```

`brew uninstall --zap --cask lowspeak` odstrani tudi nastavitve, modele, dnevnike in predpomnilnike aplikacije Lowspeak; vaši ključi API ostanejo v verigi ključev za prijavo.

**Posodobitve.** Lowspeak z diskovne slike ali iz Homebrewa se posodablja sam s pomočjo [Sparkla](https://sparkle-project.org). Ob drugem zagonu vpraša, ali sme enkrat na dan sam preveriti, ali so na voljo posodobitve. **Preveri, ali so na voljo posodobitve…** v meniju preveri takoj; **Samodejno preverjaj, ali so na voljo posodobitve** in **Samodejno prenesi in namesti posodobitve** pod njim pozneje spremenita vašo izbiro. Ko preverjanje najde novo različico, medtem ko delate v drugi aplikaciji, se na obroču v menijski vrstici pokaže pika, **Preveri, ali so na voljo posodobitve…** pa posodobitev prikaže v ospredju. Lowspeak namesti posodobitev le, če nosi podpis razvijalca; vaše nastavitve, ključi in modeli ostanejo. `brew upgrade --cask lowspeak` posodobi tudi kopijo iz Homebrewa. Različica 1.0.0 nima posodabljalnika: novejšo različico enkrat namestite čeznjo s [strani izdaj](https://github.com/sventmueller/lowspeak/releases). Tudi vaša lastna zgradba nima posodabljalnika.

**Iz izvorne kode.** Predpogoje najdete v razdelku [Gradnja in namestitev](../../../docs/development.md#build--deploy), nato:

```sh
make deploy
```

Ukaz zgradi Lowspeak, zamenja kopijo v vaši mapi Aplikacije in jo zažene.

**Odpiranje ob prijavi** v meniju zažene Lowspeak skupaj z vašim Macom, prek elementov prijave samega sistema macOS. Hkrati teče le ena kopija; druga se takoj zapre.

## Prvi koraki

### Dovoljenja

Ob prvem zagonu Lowspeak zaprosi za tri dovoljenja:

- **Dostopnost** (Nastavitve sistema › Zasebnost in varnost › Dostopnost; v macOS 27 se seznam imenuje Upravljanje naprave in dostop do podatkov). Lowspeak ga potrebuje, da zazna tipko fn in lepi s ⌘V. macOS ponudi, da odpre seznam; vklopite stikalo za Lowspeak. Lowspeakova sporočila seznam imenujejo tako, kot ga prikazuje vaš macOS.
- **Mikrofon.** Vpraša le enkrat; brez tega dovoljenja Lowspeak ne posname ničesar.
- **Obvestila.** Lowspeak jih uporablja za težave, ki jih sicer ne bi opazili, na primer mikrofon brez signala ali neuspel prepis.

### Izberite način prepisovanja

Nastavite vsaj en način; v meniju lahko kadar koli preklopite.

- **Na spletu:** izberite **Nastavi ključ API…** in prilepite ključ OpenAI (začne se s `sk-`) ali Groq (`gsk_`). Lowspeak ponudnika prepozna po ključu. Nato izberite model pod **OpenAI** ali **Groq**.
- **Na vašem Macu:** izberite **Upravljaj modele** › **Prenesi Parakeet Ultra** ali model Whisper, nato ga izberite pod **Lokalno**.

Pri izbiri pomaga razdelek [Kateri model je pravi za vas](#kateri-model-je-pravi-za-vas). Nato pridržite fn, nekaj povejte in spustite.

### Meni

Kliknite obroč v menijski vrstici:

| Ukaz | Kaj naredi |
|---|---|
| **Nedavni nareki** | zadnjih deset besedil; klik eno od njih znova kopira v odložišče |
| **Znova poskusi neuspeli narek** | znova prepiše zadnji neuspeli narek s trenutno izbranim modelom; prikaže se le po napaki |
| **Lokalno** | modeli na vašem Macu, razvrščeni v Whisper in Parakeet; tisti, ki niso preneseni, so zatemnjeni |
| **OpenAI**, **Groq** | spletni modeli; brez ključa so zatemnjeni, meni pa ponudi **Nastavi ključ API…** |
| **Jezik** | jezik, v katerem narekujete, ali **Samodejno zaznaj**; meni mu sledi |
| **Naknadna obdelava** | **Izklopljeno**, **Čiščenje** ali **Oblikovanje**, glejte [Naknadna obdelava](#naknadna-obdelava) |
| **Besedišče…** | imena in izrazi, ki naj bodo zapisani natanko; meni pokaže, koliko ste jih vnesli |
| **Mikrofon** | **Privzeto sistemsko** ali izbran mikrofon ter **Predvajaj zvok ob pripravljenosti** |
| **Upravljaj modele** | prenos in brisanje lokalnih modelov, s prikazom napredka |
| **Nastavi ključ API…** / **Posodobi ključ API…** | dodajanje, zamenjava ali odstranitev ključev |
| **Odpiranje ob prijavi** | zagon Lowspeaka skupaj z vašim Macom; kljukica pokaže, da je vklopljeno |
| **Preveri, ali so na voljo posodobitve…** | takojšnje iskanje nove različice; dva ukaza pod njim določata, ali Lowspeak preverja enkrat na dan in namešča posodobitve brez vprašanja. Samo v aplikaciji z diskovne slike |
| **Pomoč** | **Hitri pregled**, **Prijavi težavo…**, **Predlagaj funkcijo…**, **Lowspeak na GitHubu** |
| **O aplikaciji Lowspeak** | različica, avtorske pravice, stran projekta in licenca |
| **Zapri aplikacijo Lowspeak** | ⌘Q, ko je meni odprt |

Kljukica pod **Lokalno**, **OpenAI** in **Groq** označuje model v uporabi. Ikona pokaže, kaj Lowspeak počne:

| Ikona | Pomen |
|---|---|
| Obroč s piko | Pripravljen |
| Poln krog | Snemanje, dokler je fn pridržana |
| Zaobljen kvadrat | Snemanje zaklenjeno; naslednji pritisk na fn ali Shift ga konča |
| Utripajoč obroč | Prepisovanje |
| Obroč s piko zgoraj desno | Čaka nova različica, glejte **Preveri, ali so na voljo posodobitve…** |
| ↓ 42 % | Prenos modela |

Lowspeak nima okna z nastavitvami: vse je v tem meniju, ki si zapomni vsako vašo izbiro.

### Hitri pregled

**Pomoč › Hitri pregled** v majhnem oknu in v vašem jeziku pokaže kretnje s tipko fn, z gumbom, ki odpre nastavitve tipkovnice. **Prijavi težavo…** odpre obrazec na GitHubu z že vpisanima različicama Lowspeaka in macOS, **Predlagaj funkcijo…** razpravo za zamisli, **Lowspeak na GitHubu** pa stran projekta.

## Narekovanje

### Držite in govorite

Pridržite fn, govorite, spustite. Ikona snemanja se pojavi v trenutku, ko mikrofon začne dajati zvok; takrat začnite govoriti, saj to, kar rečete prej, ni posneto. Ko spustite fn, se snemanje konča, besedilo pa sledi trenutek pozneje.

Kratek dotik tipke fn ne naredi ničesar, zato fn ohrani svoje druge namene. Posnetek, ki ostane prazen ali traja manj kot 0,3 sekunde, se zavrže.

### Zaklepanje s tipko Shift ali dvojnim pritiskom

Za daljša besedila snemanje zaklenite, da teče brez pridržane tipke. Na voljo so trije načini:

- **Shift med držanjem tipke fn.** Začnite s fn kot običajno in med snemanjem pritisnite Shift: snemanje se zaklene in fn lahko spustite. Nič, kar ste rekli prej, se ne izgubi, snemanje preprosto teče naprej.
- **Shift, nato fn.** Pridržite Shift in pritisnite fn: snemanje se začne zaklenjeno.
- **Dvakrat fn.** Na kratko tapnite fn (manj kot 0,3 sekunde) in jo znova pritisnite v 0,4 sekunde. Snemanje teče od prvega pritiska.

Ikona se spremeni v zaobljen kvadrat. Za ustavitev enkrat pritisnite **fn** ali **Shift**; besedilo sledi kot običajno. Če je vaša tipka 🌐 nastavljena tako, da odpre okno »Emojiji in simboli«, ustavljajte s tipko Shift: kratek pritisk na fn bi odprl to okno.

Snemanje se samo ustavi po dveh minutah, ko je fn pridržana, in po petih minutah, ko je zaklenjeno, nato pa se prepiše kot običajno.

### Kam gre besedilo

Lowspeak besedilo prilepi v aplikacijo, ki je bila v ospredju, ko ste nehali govoriti, na mesto kazalke, prek odložišča in ⌘V. Shrani vse, kar je bilo prej v odložišču, in to vrne pol sekunde po tem, ko aplikacija prebere besedilo, ali po desetih sekundah, če ga sploh ne prebere, razen če je odložišče medtem spremenila druga aplikacija. Besedilo je označeno kot začasno, zato ga aplikacije z zgodovino odložišča, ki to oznako upoštevajo, ne zabeležijo.

Če je do takrat, ko je besedilo pripravljeno, v ospredje prišla druga aplikacija, ga Lowspeak tja ne prilepi: besedilo ostane v odložišču, obvestilo pa to sporoči. Prilepite ga s ⌘V tja, kamor sodi.

Naslednji narek lahko začnete, medtem ko se prejšnji še prepisuje. Besedila se prilepijo v vrstnem redu, v katerem ste jih izgovorili, in nikoli med snemanjem ali medtem ko držite Shift, zato lepljenje ne more pristati sredi vašega naslednjega stavka.

Brez dovoljenja za dostopnost ali ko simuliranega ⌘V ni mogoče poslati, besedilo ostane v odložišču, obvestilo pa to sporoči; prilepite ga sami s ⌘V.

### Nedavni nareki

**Nedavni nareki** v meniju navede zadnjih deset besedil, najnovejše na vrhu, vsako skrajšano na eno vrstico; ko s kazalcem zadržite nad njim, se pokaže celotno besedilo. Klik ga kopira v odložišče, za primere, ko je besedilo pristalo na napačnem mestu ali ga potrebujete dvakrat. Seznam obstaja samo v pomnilniku in je po zaprtju Lowspeaka prazen.

## Prepisovanje

### Kateri model je pravi za vas

| Želite | Izberite | Zakaj |
|---|---|---|
| **Hitrost brez obremenitve Maca** | **Groq › Whisper Large V3 Turbo** | najhitrejša spletna možnost; nič se ne prenaša, vaš Mac pa obdrži pomnilnik in prostor na disku |
| **Najboljši rezultat** | **OpenAI › GPT Transcribe** (privzeto) | najnatančnejši od modelov, ki jih ponuja Lowspeak; vaše besedišče sprejme kot ključne besede |
| **Brez povezave in zasebno** | **Lokalno › Parakeet Ultra** za evropske jezike, **Lokalno › Whisper Large v3 Turbo** za vse druge | deluje brez omrežja, vaš glas pa nikoli ne zapusti vašega Maca; Parakeet je najhitrejši od vseh |
| **Brez računa, brez stroškov** | kateri koli model pod **Lokalno** | teče na vašem Macu; nič se ne zaračuna |
| **Na spletu, a brezplačno** | **Groq** z brezplačnim paketom | račun Groq brez plačilnega sredstva; prepis v okviru dnevnih omejitev Groq ne stane nič |
| **Zanesljivost in hitrost hkrati** | spletni model, poleg tega pa nameščen Parakeet Ultra ali model Whisper | lokalni model priskoči na pomoč, ko spletna storitev odpove |
| **Mac s procesorjem Intel** | **Groq** ali **OpenAI** | lokalni modeli tam tečejo na procesorju, glejte [Maci s procesorjem Intel](#maci-s-procesorjem-intel) |

Lokalni modeli na disku zasedejo toliko, kolikor je velik njihov prenos, in, dokler so izbrani, približno toliko pomnilnika; Lowspeak ima model ves čas naložen, da se narekovanje začne brez zamika. Na Macu z 8 GB pomnilnika raje izberite Parakeet Ultra ali spletni model kot velike modele Whisper.

### Na spletu: OpenAI in Groq

| Model | Ponudnik | Opombe |
|---|---|---|
| **GPT Transcribe** (privzeto) | OpenAI | najnatančnejši; vaše besedišče sprejme kot ključne besede |
| **Whisper** | OpenAI | izvirni model Whisper |
| **Whisper Large V3** | Groq | zelo hiter |
| **Whisper Large V3 Turbo** | Groq | najhitrejši na spletu |

Posnetek se pred nalaganjem stisne v AAC, približno 6 KB na sekundo govora, zato narekovanje deluje tudi na počasnih povezavah. Medtem ko držite fn, Lowspeak že odpre povezavo s ponudnikom, zato se nalaganje začne v trenutku, ko spustite tipko.

### Na vašem Macu: Whisper in Parakeet

| Model | Velikost | Teče na | Jeziki |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 evropskih jezikov |
| **Parakeet v2** | ~600 MB | Neural Engine | angleščina |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | približno 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | približno 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | približno 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | približno 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | približno 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | približno 99 |

**Upravljaj modele** jih prenaša in briše; med prenosom menijska vrstica kaže napredek, zaprtje Lowspeaka pa prenos prekliče. Prva uporaba modela ga pripravi za vaš Mac: približno 10 sekund za Parakeet Ultra, do 30 sekund za Whisper; potem se naloži v delčku sekunde. Če izbrani model izbrišete, Lowspeak preklopi na drug nameščen model.

**Parakeet** je najhitrejša izbira: 40 sekund govora prepiše v približno 0,3 sekunde. Lowspeak ga poganja z enakimi modeli in dekodiranjem kot [FluidAudio](https://github.com/FluidInference/FluidAudio) in jih shranjuje v mapo FluidAudio (`~/Library/Application Support/FluidAudio/Models`), zato si jih aplikacije, zgrajene na FluidAudio, delijo. Posnetki, daljši od 15 sekund, se prepisujejo v prekrivajočih se oknih, ki se združijo brez izgube besed na stikih. **Whisper** razume več jezikov in bere vaše besedišče; veliki modeli so najnatančnejši, majhni pa zunaj angleščine precej šibkejši. Modeli Whisper so shranjeni v `~/Library/Application Support/Lowspeak/Models`.

### Ko spletna storitev odpove

Če spletni prepis ne uspe, ker omrežje ne deluje, ima ponudnik težavo ali je ključ napačen, isti posnetek prepiše nameščeni lokalni model: tisti, ki je izbran pod **Lokalno**, sicer Parakeet Ultra, nato Whisper Large v3 Turbo, Large v3 in manjši modeli Whisper, Parakeet v2 pa nazadnje. O tem vas enkrat opozori obvestilo; znova se pojavi šele, ko je medtem uspel kakšen spletni narek.

Če ima lokalni model kot rezervo, Lowspeak spletni zahtevi da 20 sekund; brez njega poskusi trikrat, vsakič do ene minute. V spletnem načinu Lowspeak rezervni model Parakeet pripravi enkrat ob zagonu, zato prvi preklop na rezervo ne čaka nanj. Preobremenjen ponudnik, ki prosi za premor, ga dobi, največ deset sekund; tistemu, ki prosi za daljši premor ali sporoči, da na vašem računu ni več dobroimetja, Lowspeak ne pošlje nove zahteve.

Narek, ki kljub temu ne uspe, ni izgubljen: Lowspeak njegov posnetek obdrži v pomnilniku, **Znova poskusi neuspeli narek** v meniju pa ga znova prepiše, na primer ko omrežje spet deluje ali ko izberete drug model. Obdrži se le zadnji neuspeli posnetek, po zaprtju Lowspeaka pa nobeden.

### Kako se posnetek pripravi

Lowspeak snema z lastno frekvenco vzorčenja vašega mikrofona in posnetek, ko končate, pretvori v 16 kHz, ki jih pričakujejo modeli. Pred prepisom odstrani nizko bobnenje pod 80 Hz, obreže tišino na začetku in koncu ter izenači glasnost, tako da tihi in glasni govorci zvenijo enako. Posnetek brez govora sploh ni poslan. V lokalnem načinu Parakeet najprej dobi neobdelan posnetek, pripravljenega pa le, če iz prvega ne dobi besedila.

Whisper v tišino in šum včasih zapiše besedilo, ki ga ni nihče izgovoril, na primer navedbe avtorjev podnapisov iz videov ali besedišče, ki ga je dobil. Lowspeak zavrže odseke, ki jih Whisper sam označi kot tišino, in takšno besedilo odstrani pred lepljenjem.

### Jeziki po modelih

Jezik, izbran pod **Jezik**, se posreduje spletnim modelom in Whisperju. S **Samodejno zaznaj** model jezik ugotovi sam.

| Model | Jeziki, v katerih lahko narekujete |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, na vašem Macu) | približno 99, med njimi vseh 35 jezikov menija |
| Parakeet Ultra | bolgarščina, hrvaščina, češčina, danščina, nizozemščina, angleščina, estonščina, finščina, francoščina, nemščina, grščina, madžarščina, italijanščina, latvijščina, litovščina, malteščina, poljščina, portugalščina, romunščina, ruščina, slovaščina, slovenščina, španščina, švedščina, ukrajinščina |
| Parakeet v2 | angleščina |

Parakeet jezik zazna sam in prezre izbiro v meniju. **Kitajščina (tradicionalna)** modelu GPT Transcribe naroči, naj piše s tradicionalnimi pismenkami; Whisper kitajščino piše v kateri koli od obeh pisav, portugalščino pa ne glede na različico.

### Maci s procesorjem Intel

Lowspeak je univerzalna aplikacija in deluje tudi na Macih s procesorjem Intel in sistemom macOS 14. Vse spletne funkcije delujejo enako. Lokalni modeli tam nimajo strojne opreme, za katero so zasnovani: Whisper teče na procesorju namesto na grafičnem procesorju, Parakeet pa na procesorju namesto na enoti Neural Engine. Oba delujeta, vendar potrebujeta večkrat več časa kot na Applovih čipih, veliki modeli Whisper pa veliko predolgo za narekovanje. Na Macu s procesorjem Intel prepisujte na spletu, kot [rezervo](#ko-spletna-storitev-odpove) pa obdržite Whisper Small ali Parakeet Ultra.

## Besedišče

**Besedišče…** sprejme imena in izraze, ki naj jih Lowspeak zapiše natanko, ločene z vejicami: imena izdelkov, sodelavcev, strokovne izraze. Do 100 izrazov z največ 60 znaki; `<` in `>` se odstranita.

- **GPT Transcribe** jih prejme kot ključne besede.
- **Whisper** (OpenAI, Groq in na vašem Macu) jih prebere kot namig pred posnetkom, do 500 znakov.
- **Naknadni obdelavi** je naročeno, naj jih zapiše natanko tako, kot so napisani.
- **Parakeet** jih ne sliši. Kjer je zapisal nekaj zelo podobnega, Lowspeak to v njegovem besedilu nadomesti z njihovim zapisom: »Träfig« ali »Trafik« postane Traefik, »Los Peak« postane Lowspeak. Izrazi, krajši od petih črk, besede v naslovih in imenih datotek ter oblike s končnico, kot je »Lowspeaks«, ostanejo, kot so zapisani. Da bi napačen zapis ločil od prave besede, Lowspeak povpraša črkovalnik v macOS, in sicer na vašem Macu.

## Naknadna obdelava

Po prepisu lahko jezikovni model besedilo predela. Izberite pod **Naknadna obdelava**:

- **Izklopljeno** (privzeto) – besedilo, kot je bilo prepisano.
- **Čiščenje** – odstrani mašila, kot so »ee«, »veš« ali »pravzaprav«, popravi slovnico in ločila ter, ko se popravite, obdrži samo popravek. Ničesar ne doda in ne spremeni pomena.
- **Oblikovanje** – kot Čiščenje, poleg tega pa besedilo prilagodi aplikaciji, v katero narekujete: izrazi v slogu kode v urejevalnikih kode, ukazne vrstice v terminalih, sproščen ton v aplikacijah za klepet, poslovni v e-pošti.

Naknadna obdelava uporablja Groq (`openai/gpt-oss-120b`), če imate ključ Groq, sicer OpenAI (`gpt-4o-mini`), tudi ko prepisujete lokalno. Če je pod **Jezik** izbran jezik, ta jezik ohrani. Če traja dlje kot osem sekund, ne uspe ali vrne odgovor, ki je odrezan ali precej daljši od tega, kar ste povedali, Lowspeak prilepi neobdelano besedilo. Če ponudnik zahtevo zavrne ali ključ ni nastavljen, to enkrat sporoči obvestilo.

## Mikrofon

Pod **Mikrofon** izberite določen mikrofon ali **Privzeto sistemsko**. Lowspeak si napravo zapomni; dokler ni povezana, jo meni prikaže kot **(ni povezan)**, Lowspeak pa snema s sistemskim privzetim mikrofonom.

Mikrofon je vklopljen samo med narekovanjem. Njegova priprava traja približno 0,07 sekunde z vgrajenim mikrofonom, približno pol sekunde z zaslonom Studio Display ter eno do dve sekundi s slušalkami AirPods, ki najprej preklopijo v način slušalk z mikrofonom. Ikona snemanja se pojavi, ko prispe zvok; **Predvajaj zvok ob pripravljenosti** v tistem trenutku doda kratek zvok. Privzeto je izklopljeno, ker na nekaterih zvočnikih sam zvok zamuja pol sekunde.

Če mikrofon pet sekund daje samo tišino, kot slušalke AirPods, ko njihov mikrofon zaseda druga naprava, vas Lowspeak na to opozori.

## Ključi API

**Nastavi ključ API…** vpraša za ključ in prepozna ponudnika po začetku ključa: `sk-` za OpenAI, `gsk_` za Groq. Ključ v drugi obliki je zavrnjen z razlago. Vnesite po en ključ za vsakega ponudnika; s ključi za oba so na voljo vsi spletni modeli. **Posodobi ključ API…** zamenja ključ ali jih z **Odstrani vse ključe** izbriše.

Ključi so shranjeni v vaši verigi ključev za prijavo pod imenom `com.lowspeak.app`. Datoteka s ključi iz starejše različice (`~/Library/Application Support/Lowspeak/.api-key`) se ob naslednjem zagonu premakne tja in izbriše, ko je vsak ključ mogoče znova prebrati iz verige ključev.

## Obvestila

Lowspeak deluje v ozadju, zato težave sporoča z obvestili:

| Obvestilo | Kaj se je zgodilo |
|---|---|
| **Snemanje ni uspelo** | ni dostopa do mikrofona, noben mikrofon ni priključen ali mikrofona ni bilo mogoče zagnati |
| **Mikrofon brez signala** | pet sekund tišine, glejte [Mikrofon](#mikrofon) |
| **Potrebno je dovoljenje za dostopnost** | besedilo je v odložišču; za samodejno lepljenje dovolite dostopnost |
| **Lepljenje ni uspelo** | besedilo je v odložišču; prilepite ga s ⌘V |
| **Prepis ni uspel** | z razlogom: ni ključa, ključ zavrnjen, ni več dobroimetja, omejitev zahtev presežena, napaka strežnika ali omrežja ali ni lokalnega modela; posnetek se obdrži za **Znova poskusi neuspeli narek** |
| **Narek ni bil prilepljen** | preden je bilo besedilo pripravljeno, je v ospredje prišla druga aplikacija; besedilo je v odložišču |
| **Prepisano na tem Macu** | spletna storitev je odpovedala in prevzel je lokalni model |
| **Naknadna obdelava ni uspela** | ponudnik zavrača naknadno obdelavo ali ključ ni nastavljen; besedilo je bilo prilepljeno brez nje |
| **Ni ključa API** | izbrali ste spletni model brez ključa za njegovega ponudnika |
| **Prenos ni uspel**, **Brisanje ni uspelo**, **Brisanje ni mogoče** | upravljanje lokalnega modela ni uspelo ali pa je še potekalo drugo opravilo |
| **Napaka elementa za prijavo**, **Napaka ključa API** | macOS ali veriga ključev je zavrnila spremembo |

Obvestila nikoli ne vsebujejo vašega narekovanega besedila. Spletna storitev, ki odpoveduje, in ponudnik naknadne obdelave, ki zavrača zahteve, sta sporočena enkrat in ne po vsakem nareku, dokler spet ne delujeta.

## Videz in dostopnost

- **Ikona v menijski vrstici** je predloga slike, ki sledi svetli, temni ali obarvani menijski vrstici.
- **VoiceOver** ikono prebere kot »Lowspeak«, meni, opozorila in obvestila pa so standardni kontrolniki macOS.
- **Nobene tipke ni treba držati.** Snemanje zaklenite s tipko Shift ali z dvojnim pritiskom na fn, naslednji pritisk pa ga konča.
- **Hitri pregled** je preprosto okno z oznakami in gumbom, dosegljivo s tipkovnice; Esc ga zapre.

Znane pomanjkljivosti so navedene v [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver ne napove začetka snemanja, Lowspeak pa še ni bil preizkušen z VoiceOverjem ali Stikalnim upravljanjem.

## Jeziki

Meni, pogovorna okna in obvestila so prevedeni v 35 jezikov:

arabščino, katalonščino, kitajščino (poenostavljeno), kitajščino (tradicionalno), hrvaščino, češčino, danščino, nizozemščino, angleščino, finščino, francoščino, nemščino, grščino, hebrejščino, hindijščino, madžarščino, indonezijščino, italijanščino, japonščino, korejščino, malajščino, norveščino, poljščino, portugalščino (Brazilija), portugalščino (Portugalska), romunščino, ruščino, slovaščino, slovenščino, španščino, švedščino, tajščino, turščino, ukrajinščino in vietnamščino.

Jezik, ki ga izberete pod **Jezik**, je tudi jezik menija. S **Samodejno zaznaj** meni sledi vrstnemu redu jezikov v **Nastavitve sistema › Splošno › Jezik in regija**, kjer prevoda ni, pa angleščini. Izrazi v meniju sledijo Applovim lastnim prevodom, zato se Lowspeak bere kot preostanek vašega Maca. Razen angleškega so prevodi strojni in jih rojeni govorci še niso pregledali; popravki so dobrodošli kot [issue na GitHubu](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Tipke in kretnje

| Dejanje | Tipke |
|---|---|
| Narekovanje med držanjem | pridržite fn |
| Zaklepanje tekočega snemanja | Shift med držanjem tipke fn |
| Začetek zaklenjenega snemanja | Shift+fn ali dvakrat hitro fn |
| Konec zaklenjenega snemanja | fn ali Shift |
| Zapiranje Lowspeaka (meni odprt) | ⌘Q |
| Zapiranje hitrega pregleda | Esc |

Lowspeak pritiskov tipke fn, ki jih uporablja, ne posreduje drugim aplikacijam; Shift gre skozi kot običajno.

## Zasebnost in varnost

Lowspeak sliši, kaj govorite, dokler je fn pridržana, lahko piše v katero koli aplikacijo in hrani vaše ključe API, zato pri tem počne čim manj.

- **Vaš glas gre na eno mesto.** V spletnem načinu gre stisnjen posnetek k ponudniku, ki ste ga izbrali, OpenAI ali Groq, skupaj z vašim besediščem. V lokalnem načinu nič ne zapusti vašega Maca. Ko spletna zahteva ne uspe, isti posnetek na vašem Macu prepiše lokalni model.
- **Naknadna obdelava**, če jo vklopite, pošlje prepisano besedilo in vaše besedišče storitvi Groq ali OpenAI. V načinu Oblikovanje doda vrsto aplikacije, v katero narekujete (urejevalnik kode, terminal, klepet ali e-pošta); ime aplikacije ni poslano.
- **Brez telemetrije, brez računa.** Razen za prepis in naknadno obdelavo gre Lowspeak na splet samo zato, da odpre povezavo s ponudnikom, medtem ko je fn pridržana, v spletnem načinu ali z vklopljeno naknadno obdelavo, da prenese model, ko to zahtevate, s Hugging Face prek HTTPS, v fiksni različici in s preverjanjem kontrolne vsote vsake datoteke, in da na GitHubu poišče novo različico, enkrat na dan, če to dovolite. Ta zahteva vsebuje različico Lowspeaka in nič drugega. **Pomoč › Prijavi težavo…** v brskalniku odpre obrazec na GitHubu; njegov naslov vsebuje različico Lowspeaka in različico macOS, nič od tega, kar ste narekovali.
- **Ključi API** ostanejo v vaši verigi ključev za prijavo. Pošljejo se samo njihovemu ponudniku in se odstranijo iz sporočil o napakah, preden se karkoli zapiše v dnevnik.
- **Narekovano besedilo se ne shranjuje.** Zadnjih deset narekov ostane v pomnilniku za **Nedavni nareki** in izgine, ko se Lowspeak zapre, prav tako posnetek zadnjega neuspelega nareka, ki se hrani za **Znova poskusi neuspeli narek**. Dnevniške datoteke beležijo dolžine in čase, nikoli besedila.
- **Odložišče** se pred lepljenjem shrani in obnovi, ko aplikacija prebere besedilo, razen če ga je medtem spremenila druga aplikacija. Narek je označen kot začasen, zato ga aplikacije z zgodovino odložišča, ki to oznako upoštevajo, preskočijo.
- **Podpisovanje kode** s Hardened Runtime; aplikacija zahteva le mikrofon, dostopnost, obvestila in omrežje. Posodobitev mora nositi podpis razvijalca, sicer jo Sparkle zavrne.

Če želite prijaviti ranljivost, upoštevajte [varnostni pravilnik](../../../SECURITY.md).

## Hitrost

Izmerjeno na Applovih čipih:

- **Parakeet Ultra** prepiše 40 sekund govora v približno 0,3 sekunde.
- **Na spletu:** prva zahteva po premoru traja pri Groq približno 0,4 sekunde, ker Lowspeak odpre povezavo, medtem ko še govorite; poznejše zahteve jo uporabijo znova, prek HTTP/3, kjer ga ponudnik podpira.
- **Velikost nalaganja:** približno 6 KB na sekundo govora, kot AAC s 48 kbit/s.
- **Priprava mikrofona:** približno 0,07 sekunde pri vgrajenem, pol sekunde pri zaslonu Studio Display, ena do dve sekundi s slušalkami AirPods.

## Znane omejitve

- **Lokalni modeli na Macih s procesorjem Intel** tečejo na procesorju in so precej počasnejši; tam je spletni prepis boljša izbira.
- **Kar rečete, preden je mikrofon pripravljen, se izgubi.** Počakajte na ikono snemanja, zlasti s slušalkami AirPods.
- **Tipko fn si lahko prilastijo drugi.** Ko nastavitev tipke 🌐 v macOS, Warp ali Narekovanje v macOS prvi zasedejo fn, je Lowspeak ne vidi; glejte [Odpravljanje težav](#odpravljanje-težav).
- **Parakeet** prezre izbiro jezika in pozna samo evropske jezike. Besedišče njegov zapis le naknadno popravi: izraz, ki ga je model slišal kot precej drugačno besedo, ostane napačen.
- **Pisava in različica:** samo modelu GPT Transcribe lahko naročite, naj piše tradicionalno kitajščino; nobenemu modelu ne morete naročiti, naj piše evropsko namesto brazilske portugalščine.
- **Časovne omejitve:** dve minuti, ko je fn pridržana, pet minut, ko je snemanje zaklenjeno.
- **Način mikrofona:** Nadzorno središče za Lowspeak ponuja Osamitev zvoka in Široki spekter ter ju prikaže kot aktivna, vendar Lowspeak snema brez obdelave glasu, zato izbira nima učinka.
- **Nedavni nareki** obstajajo samo v pomnilniku in po zaprtju izginejo.
- **Po zasnovi:** brez okna z nastavitvami, brez ikone v Docku, ne v App Storu.

## Odpravljanje težav

**Ko pridržim fn, se nič ne zgodi.** Preverite tri stvari: Lowspeak je dovoljen pod **Nastavitve sistema › Zasebnost in varnost › Dostopnost** (v macOS 27: **Upravljanje naprave in dostop do podatkov**); **Nastavitve sistema › Tipkovnica › Ob pritisku tipke 🌐** je nastavljeno na **Ne naredi ničesar**; in nobena druga aplikacija ne zaseda tipke fn. Warp jo zaseda privzeto; to izklopite z `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` in znova zaženite Warp. Tudi Narekovanje v macOS je lahko nastavljeno tako, da se zažene s tipko fn.

**Po posodobitvi se Lowspeak ne odziva več na fn.** macOS dovoljenje za dostopnost veže na podpis aplikacije. Če se je podpis spremenil, na primer z vaše lastne zgradbe na preneseno aplikacijo, odstranite Lowspeak s seznama Dostopnost (v macOS 27: Upravljanje naprave in dostop do podatkov) in ga znova dodajte.

**Prve besede manjkajo.** Začnite govoriti, ko se pojavi ikona snemanja. S slušalkami AirPods mikrofon potrebuje eno do dve sekundi; vgrajeni ali USB-mikrofon, izbran pod **Mikrofon**, je pripravljen skoraj takoj.

**»Mikrofon brez signala.«** Mikrofon pet sekund ni dal zvoka. Pri slušalkah AirPods morda njihov mikrofon uporablja druga naprava; preklopite na vgrajeni mikrofon ali jih znova povežite. Tudi zaprt pokrov MacBooka utiša vgrajeni mikrofon.

**Nareki so prazni.** Vhodna glasnost je morda prenizka: aplikacije, kot so Teams, Zoom ali Meet, jo med klici spreminjajo, macOS pa jo hrani za vsako napravo posebej. Povečajte jo pod **Nastavitve sistema › Zvok › Vhod**.

**Besedilo je pristalo na napačnem mestu.** **Nedavni nareki** v meniju ga znova kopirajo v odložišče.

**»Ni ključa API.«** V meniju izberite **Nastavi ključ API…** ali preklopite na lokalni model.

**Prvi lokalni prepis traja dolgo.** Vsak model se ob prvi uporabi pripravi za vaš Mac; od takrat se naloži v delčku sekunde.

**Lowspeak se zapre takoj po zagonu.** Druga kopija Lowspeaka že teče; najprej zaprite njo.

**Obvestila se ne prikazujejo.** Dovolite jih pod **Nastavitve sistema › Obvestila › Lowspeak**.

**Nekaj drugega.** Dnevniki v `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) opisujejo, kaj se je zgodilo, brez vašega narekovanega besedila. Ustrezne vrstice priložite [prijavi težave](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Za razvijalce

Gradnja, testiranje, zgradba kode in pravila, ki jim sledi, so opisani v [docs/development.md](../../../docs/development.md), prispevki pa v [CONTRIBUTING.md](../../../CONTRIBUTING.md) (oboje v angleščini).

### Komponente tretjih oseb

| Komponenta | Različica | Za kaj se uporablja | Licenca |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) prek [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | lokalni Whisper na grafičnem procesorju (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | dekodiranje Parakeeta in združevanje dolgih posnetkov, prevedeno v Rust v `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) in pripadajoči paketi za ogrodja | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation in druga sistemska ogrodja | MIT; Zlib, Apache-2.0 ali MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | zajem zvoka iz mikrofona prek CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | prevzorčenje na 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) z rustls | 3.5 | prenosi modelov | MIT ali Apache-2.0; rustls in ring pod ISC in Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | kontrolne vsote prenosov modelov | MIT ali Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | ključi API v verigi ključev | MIT ali Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, napake, slovar Parakeeta | MIT ali Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | posodobitve aplikacije z diskovne slike | MIT, z deli pod licencama BSD in zlib |

Besedila licenc za vse, kar je v aplikaciji, so priložena aplikaciji kot `Contents/Resources/Acknowledgements.txt`.

Modeli niso del Lowspeaka; prenese jih, ko to zahtevate:

| Model | Vir | Licenca |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper podjetja OpenAI v formatu ggml) | MIT |
| Parakeet v2, Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference), pretvorbe CoreML modela Parakeet TDT podjetja NVIDIA | CC-BY-4.0 |

## Licenca

Lowspeak je licenciran po eni od naslednjih licenc

- Licenca Apache, različica 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licenca MIT ([LICENSE-MIT](../../../LICENSE-MIT))

po vaši izbiri.

Če izrecno ne navedete drugače, je vsak prispevek, ki ga za vključitev v Lowspeak namenoma predložite, kot je opredeljeno v licenci Apache-2.0, dvojno licenciran kot zgoraj, brez dodatnih pogojev.
