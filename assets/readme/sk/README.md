<p><img src="../../readme-icon.png" alt="Ikona aplikácie Lowspeak" width="168" height="168"></p>

# Lowspeak

**Podržte fn, hovorte, pustite. Vaše slová už tam sú.**

Lowspeak sídli na lište menu a počúva, iba keď držíte kláves fn. Povedzte, čo chcete napísať, pustite ho a text sa objaví v aplikácii, v ktorej práve ste: v pošte, chate, editore kódu, termináli.

Je to skutočná aplikácia pre Mac, napísaná v Ruste na AppKite, bez jadra prehliadača pod kapotou. Spustí sa okamžite a nezavadzia. Spojenie dokonca otvára už vtedy, keď ešte hovoríte, takže text z online modelu príde asi 0,4 sekundy po tom, ako kláves pustíte.

Vyberať môžete z dvanástich modelov na prepis reči. S vlastným kľúčom Groq vás online prepis v rámci denného limitu Groq nestojí nič; náročné nahrávky zvládne GPT Transcribe od OpenAI. Lowspeak s nimi komunikuje priamo, bez účtu a bez vlastného servera ako sprostredkovateľa.

Alebo si nechajte všetko na Macu. Whisper a NVIDIA Parakeet fungujú offline, takže váš hlas nikdy neopustí počítač, a Parakeet premení 40 sekúnd reči na text asi za tretinu sekundy. Keď pracujete online a vypadne sieť, zaskočí nainštalovaný lokálny model, takže sa žiadny diktát nestratí.

Lowspeak hovorí 35 jazykmi. Naučí sa mená a výrazy, ktoré používate, dokáže odstrániť výplňové slová a učesať gramatiku. Posledných desať diktátov navyše drží v menu, keby ste niektorý znova potrebovali.

[![Stiahnuť Lowspeak pre macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Stiahnuť Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: podpísaný a overený spoločnosťou Apple (notarizovaný), pre macOS 14 alebo novší na Apple Silicon aj Intel.

**Našli ste chybu?** [Založte issue](https://github.com/sventmueller/lowspeak/issues/new/choose) alebo v menu Lowspeaku zvoľte **Pomocník › Nahlásiť problém…**: formulár sa otvorí s už vyplnenou verziou. Ak je pre vás Lowspeak užitočný, [hviezdička na GitHube](https://github.com/sventmueller/lowspeak) pomôže ostatným nájsť ho.

[![Rust edícia 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 alebo novší](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Univerzálny: Apple Silicon a Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit cez objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp na Metale](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet na Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online s OpenAI alebo Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 jazykov rozhrania](https://img.shields.io/badge/languages-35-4C9A2A)
![Podpísaný s Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licencia: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licencia)

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
**Slovenčina** ·
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

## Obsah

- [Funkcie na prvý pohľad](#funkcie-na-prvý-pohľad)
- [Požiadavky](#požiadavky)
- [Inštalácia](#inštalácia)
- [Prvé kroky](#prvé-kroky)
  - [Povolenia](#povolenia)
  - [Výber spôsobu prepisu](#výber-spôsobu-prepisu)
  - [Menu](#menu)
  - [Stručný prehľad](#stručný-prehľad)
- [Diktovanie](#diktovanie)
  - [Podržte a hovorte](#podržte-a-hovorte)
  - [Uzamknutie klávesom Shift alebo dvojitým stlačením](#uzamknutie-klávesom-shift-alebo-dvojitým-stlačením)
  - [Kam sa text vloží](#kam-sa-text-vloží)
  - [Nedávne diktáty](#nedávne-diktáty)
- [Prepis](#prepis)
  - [Ktorý model je pre vás vhodný](#ktorý-model-je-pre-vás-vhodný)
  - [Online: OpenAI a Groq](#online-openai-a-groq)
  - [Na vašom Macu: Whisper a Parakeet](#na-vašom-macu-whisper-a-parakeet)
  - [Keď online služba zlyhá](#keď-online-služba-zlyhá)
  - [Ako sa nahrávka pripravuje](#ako-sa-nahrávka-pripravuje)
  - [Jazyky podľa modelu](#jazyky-podľa-modelu)
  - [Macy s procesorom Intel](#macy-s-procesorom-intel)
- [Slovník](#slovník)
- [Následné spracovanie](#následné-spracovanie)
- [Mikrofón](#mikrofón)
- [Kľúče API](#kľúče-api)
- [Hlásenia](#hlásenia)
- [Vzhľad a prístupnosť](#vzhľad-a-prístupnosť)
- [Jazyky](#jazyky)
- [Klávesy a gestá](#klávesy-a-gestá)
- [Súkromie a zabezpečenie](#súkromie-a-zabezpečenie)
- [Rýchlosť](#rýchlosť)
- [Známe obmedzenia](#známe-obmedzenia)
- [Riešenie problémov](#riešenie-problémov)
- [Pre vývojárov](#pre-vývojárov)
  - [Komponenty tretích strán](#komponenty-tretích-strán)
- [Licencia](#licencia)

## Funkcie na prvý pohľad

- **Natívny od základov.** Skutočná položka na lište menu s menu AppKit, štandardnými dialógmi a hláseniami. Žiadny Electron, žiadne okno, žiadna ikona v Docku.
- **Správny model pre každú úlohu.** Groq pre rýchlosť bez zaťaženia pamäte Macu, GPT Transcribe od OpenAI pre najlepší výsledok, Whisper alebo NVIDIA Parakeet na vašom Macu na prácu offline a súkromie vášho hlasu. Prepnete jedným kliknutím.
- **Súkromný.** Žiadny účet, žiadna telemetria. Zvuk ide iba k poskytovateľovi, ktorého ste si vybrali, a v lokálnom režime nikam.
- **Žiadny diktát sa nestratí.** Keď zlyhá sieť alebo online služba, nainštalovaný lokálny model prepíše tú istú nahrávku na vašom Macu.
- **Vo vašom jazyku.** Diktovanie funguje v desiatkach jazykov a menu, dialógy a hlásenia sú k dispozícii v 35.
- **Vaše slová, správne napísané.** Slovník mien a odborných výrazov usmerňuje OpenAI, Groq, Whisper aj následné spracovanie a opravuje, ako ich píše Parakeet.
- **Čistý text, ak ho chcete.** Voliteľné následné spracovanie odstráni výplňové slová, opraví gramatiku a keď sa opravíte, ponechá iba opravu; prípadne text aj naformátuje podľa druhu aplikácie, do ktorej diktujete.
- **Schránka zostáva vaša.** Lowspeak vkladá cez schránku a o chvíľu do nej vráti to, čo v nej bolo. Posledných desať diktátov zostáva v menu, aby ste ich mohli znova skopírovať.
- **Podržte fn, hovorte, pustite.** Text sa napíše do aplikácie v popredí, tam, kde je kurzor: do pošty, chatu, editora, terminálu alebo prehliadača. Žiadne okno, na ktoré treba prepnúť, žiadne tlačidlo, na ktoré treba kliknúť.
- **Uzamknutie pre dlhšie texty.** Stlačte Shift, kým držíte fn, alebo stlačte fn dvakrát a nahrávanie pobeží bez držania klávesu až päť minút. Jedno stlačenie fn alebo Shiftu ho ukončí.
- **Hovorte ďalej.** Začnite ďalší diktát, kým sa predchádzajúci ešte prepisuje; texty prídu v poradí, v akom ste ich vyslovili.

## Požiadavky

- macOS 14 Sonoma alebo novší. Lowspeak sa vyvíja a testuje na macOS 27.
- Mac s procesorom Apple Silicon alebo Intel. Lokálny prepis je určený pre Apple Silicon, kde Whisper beží na GPU a Parakeet na Neural Engine; na Macoch s procesorom Intel bežia oba na procesore, oveľa pomalšie, pozrite [Macy s procesorom Intel](#macy-s-procesorom-intel).
- Asi 17 MB pre aplikáciu a k tomu 75 MB až 3 GB pre každý stiahnutý lokálny model.
- Na prepis online kľúč API od [OpenAI](https://platform.openai.com/api-keys) alebo [Groq](https://console.groq.com/keys). OpenAI si účtuje za to, čo prepíšete; bezplatný plán Groq v rámci denných limitov nestojí nič a limity nájdete v konzole Groq. Lokálny prepis nepotrebuje žiadny účet.

## Inštalácia

**Z obrazu disku.** Stiahnite si [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), otvorte ho a pretiahnite Lowspeak do priečinka Aplikácie. Obraz je podpísaný certifikátom Developer ID a overený spoločnosťou Apple (notarizovaný), takže sa otvorí bez upozornení.

**Cez Homebrew.** Cask je vo vlastnom tape:

```sh
brew install --cask sventmueller/tap/lowspeak
```

`brew uninstall --zap --cask lowspeak` odstráni aj nastavenia, modely, protokoly a vyrovnávacie pamäte Lowspeaku; vaše kľúče API zostávajú v prihlasovacej kľúčenke.

**Aktualizácie.** Lowspeak z obrazu disku alebo z Homebrew sa aktualizuje sám cez [Sparkle](https://sparkle-project.org). Pri druhom spustení sa spýta, či môže raz denne sám vyhľadávať aktualizácie. **Vyhľadať aktualizácie…** v menu vyhľadá ihneď; **Automaticky vyhľadávať aktualizácie** a **Automaticky sťahovať a inštalovať aktualizácie** pod ňou vašu voľbu neskôr zmenia. Keď kontrola nájde novú verziu, kým pracujete v inej aplikácii, na krúžku na lište menu sa objaví bodka a **Vyhľadať aktualizácie…** prenesie aktualizáciu do popredia. Lowspeak nainštaluje aktualizáciu, iba ak nesie podpis vývojára; vaše nastavenia, kľúče a modely zostanú. `brew upgrade --cask lowspeak` aktualizuje aj kópiu z Homebrew. Verzia 1.0.0 nemá aktualizátor: novšiu verziu cez ňu raz nainštalujte zo [stránky vydaní](https://github.com/sventmueller/lowspeak/releases). Ani vlastné zostavenie nemá aktualizátor.

**Zo zdrojového kódu.** Predpoklady nájdete v časti [Zostavenie a nasadenie](../../../docs/development.md#build--deploy), potom:

```sh
make deploy
```

Tým sa Lowspeak zostaví, nahradí kópiu v priečinku Aplikácie a spustí sa.

**Otvoriť po prihlásení** v menu spúšťa Lowspeak spolu s Macom, prostredníctvom vlastných položiek pri prihlásení v macOS. Naraz beží iba jedna kópia; druhá sa hneď ukončí.

## Prvé kroky

### Povolenia

Pri prvom spustení Lowspeak požiada o tri povolenia:

- **Prístupnosť** (Systémové nastavenia › Súkromie a bezpečnosť › Prístupnosť; v macOS 27 sa zoznam volá Ovládanie zariadenia a prístup k dátam). Lowspeak ju potrebuje, aby zaznamenal kláves fn a mohol vkladať pomocou ⌘V. macOS ponúkne otvorenie zoznamu; zapnite prepínač pri Lowspeaku. Lowspeak vo svojich hláseniach uvádza názov zoznamu tak, ako ho zobrazuje váš macOS.
- **Mikrofón.** Žiadosť sa zobrazí raz; bez tohto povolenia Lowspeak nič nenahrá.
- **Hlásenia.** Lowspeak nimi oznamuje problémy, ktoré by ste inak nezbadali, napríklad tichý mikrofón alebo neúspešný prepis.

### Výber spôsobu prepisu

Nastavte aspoň jeden spôsob; v menu ho môžete kedykoľvek prepnúť.

- **Online:** zvoľte **Nastaviť kľúč API…** a vložte kľúč od OpenAI (začína na `sk-`) alebo Groq (`gsk_`). Lowspeak rozpozná poskytovateľa podľa kľúča. Potom vyberte model v časti **OpenAI** alebo **Groq**.
- **Na vašom Macu:** zvoľte **Spravovať modely** › **Stiahnuť Parakeet Ultra** alebo niektorý model Whisper a potom ho vyberte v časti **Lokálne**.

S výberom pomôže [Ktorý model je pre vás vhodný](#ktorý-model-je-pre-vás-vhodný). Potom podržte fn, niečo povedzte a pustite.

### Menu

Kliknite na krúžok na lište menu:

| Položka | Čo robí |
|---|---|
| **Nedávne diktáty** | posledných desať textov; kliknutím jeden z nich znova skopírujete do schránky |
| **Znova skúsiť neúspešný diktát** | znova prepíše posledný neúspešný diktát práve zvoleným modelom; zobrazí sa iba po zlyhaní |
| **Lokálne** | modely na vašom Macu, zoskupené na Whisper a Parakeet; nestiahnuté sú sivé |
| **OpenAI**, **Groq** | online modely; bez kľúča sú sivé a menu namiesto nich ponúka **Nastaviť kľúč API…** |
| **Jazyk** | jazyk, v ktorom diktujete, alebo **Rozpoznať automaticky**; menu sa ním riadi |
| **Následné spracovanie** | **Vypnuté**, **Vyčistenie** alebo **Formátovanie**, pozrite [Následné spracovanie](#následné-spracovanie) |
| **Slovník…** | mená a výrazy, ktoré sa majú písať presne; menu ukazuje, koľko ste ich zadali |
| **Mikrofón** | **Systémová predvoľba** alebo pevne zvolený mikrofón a **Prehrať zvuk po pripravení** |
| **Spravovať modely** | sťahovanie a vymazávanie lokálnych modelov, s priebehom |
| **Nastaviť kľúč API…** / **Zmeniť kľúč API…** | pridanie, nahradenie alebo odstránenie kľúčov |
| **Otvoriť po prihlásení** | spúšťanie Lowspeaku spolu s Macom; začiarknutie ukazuje, že je voľba zapnutá |
| **Vyhľadať aktualizácie…** | vyhľadanie novej verzie hneď teraz; dve položky pod ňou rozhodujú, či Lowspeak hľadá raz denne a inštaluje aktualizácie bez pýtania. Iba v aplikácii z obrazu disku |
| **Pomocník** | **Stručný prehľad**, **Nahlásiť problém…**, **Navrhnúť funkciu…**, **Lowspeak na GitHube** |
| **O aplikácii Lowspeak** | verzia, autorské práva, stránka projektu a licencia |
| **Ukončiť Lowspeak** | ⌘Q, keď je menu otvorené |

Začiarknutie v **Lokálne**, **OpenAI** a **Groq** označuje používaný model. Ikona ukazuje, čo Lowspeak práve robí:

| Ikona | Význam |
|---|---|
| Krúžok s bodkou | Pripravený |
| Vyplnený kruh | Nahrávanie, kým držíte fn |
| Zaoblený štvorec | Nahrávanie uzamknuté; ďalšie stlačenie fn alebo Shiftu ho ukončí |
| Blikajúci krúžok | Prepisovanie |
| Krúžok s bodkou vpravo hore | Čaká nová verzia, pozrite **Vyhľadať aktualizácie…** |
| ↓ 42 % | Sťahovanie modelu |

Lowspeak nemá okno s nastaveniami: všetko je v tomto menu a každú voľbu si pamätá.

### Stručný prehľad

**Pomocník › Stručný prehľad** ukáže gestá klávesu fn v malom okne, vo vašom jazyku, s tlačidlom, ktoré otvorí nastavenia klávesnice. **Nahlásiť problém…** otvorí formulár na GitHube s už vyplnenou verziou Lowspeaku a macOS, **Navrhnúť funkciu…** diskusiu na nápady a **Lowspeak na GitHube** stránku projektu.

## Diktovanie

### Podržte a hovorte

Podržte fn, hovorte, pustite. Ikona nahrávania sa objaví vo chvíli, keď mikrofón začne dodávať zvuk; až potom začnite hovoriť, pretože to, čo poviete predtým, sa nenahrá. Uvoľnením fn sa nahrávanie skončí a text nasleduje o chvíľu neskôr.

Krátke ťuknutie na fn nerobí nič, takže si fn zachová svoje ostatné využitia. Nahrávka, ktorá zostane prázdna alebo trvá menej ako 0,3 sekundy, sa zahodí.

### Uzamknutie klávesom Shift alebo dvojitým stlačením

Pri dlhších textoch nahrávanie uzamknite, aby bežalo bez držania klávesu. Sú tri spôsoby:

- **Shift počas držania fn.** Začnite ako zvyčajne s fn a počas nahrávania stlačte Shift: nahrávanie sa uzamkne a fn môžete pustiť. Nič z toho, čo ste už povedali, sa nestratí, nahrávanie jednoducho pokračuje.
- **Shift, potom fn.** Podržte Shift a stlačte fn: nahrávanie sa začne rovno uzamknuté.
- **Dvakrát fn.** Krátko ťuknite na fn (kratšie ako 0,3 sekundy) a do 0,4 sekundy ho stlačte znova. Nahrávka beží od prvého stlačenia.

Ikona sa zmení na zaoblený štvorec. Nahrávanie ukončíte jedným stlačením **fn** alebo **Shift**; text nasleduje ako zvyčajne. Ak máte kláves 🌐 nastavený na otváranie panela Emoji a symboly, ukončujte nahrávanie klávesom Shift: krátke stlačenie fn by otvorilo prehliadač znakov.

Nahrávanie sa samo zastaví po dvoch minútach, keď držíte fn, a po piatich minútach v uzamknutom režime, a potom sa prepíše ako zvyčajne.

### Kam sa text vloží

Lowspeak vloží text do aplikácie, ktorá bola v popredí, keď ste prestali hovoriť, na miesto kurzora, cez schránku a ⌘V. Uloží všetko, čo bolo predtým v schránke, a vráti to späť pol sekundy potom, ako aplikácia text prečíta, alebo po desiatich sekundách, keď ho neprečíta vôbec, ak medzitým schránku nezmenila iná aplikácia. Text je označený ako dočasný, takže ho aplikácie s históriou schránky, ktoré toto označenie rešpektujú, nezaznamenajú.

Ak sa do chvíle, keď je text hotový, dostane do popredia iná aplikácia, Lowspeak ho do nej nevloží: text zostane v schránke a hlásenie vám to oznámi. Vložte ho pomocou ⌘V tam, kam patrí.

Ďalší diktát môžete začať, kým sa predchádzajúci ešte prepisuje. Texty sa vkladajú v poradí, v akom ste ich vyslovili, a nikdy počas nahrávania alebo keď držíte Shift, takže vložený text nemôže skončiť uprostred vašej ďalšej vety.

Bez povolenia na prístupnosť alebo keď sa simulované ⌘V nedá odoslať, text zostane v schránke a hlásenie vám to oznámi; vložte ho sami pomocou ⌘V.

### Nedávne diktáty

**Nedávne diktáty** v menu zobrazujú posledných desať textov, od najnovšieho, každý skrátený na jeden riadok; keď nad položku presuniete ukazovateľ, zobrazí sa celý text. Kliknutím ho skopírujete do schránky, napríklad keď text skončil na nesprávnom mieste alebo ho potrebujete dvakrát. Zoznam existuje iba v pamäti a po ukončení Lowspeaku je prázdny.

## Prepis

### Ktorý model je pre vás vhodný

| Čo potrebujete | Zvoľte | Prečo |
|---|---|---|
| **Rýchlosť bez zaťaženia Macu** | **Groq › Whisper Large V3 Turbo** | najrýchlejšia online možnosť; nič sa nesťahuje a váš Mac si zachová pamäť aj miesto na disku |
| **Najlepší výsledok** | **OpenAI › GPT Transcribe** (predvolený) | najpresnejší z modelov, ktoré Lowspeak ponúka; váš slovník preberá ako kľúčové slová |
| **Prácu offline a súkromie** | **Lokálne › Parakeet Ultra** pre európske jazyky, **Lokálne › Whisper Large v3 Turbo** pre všetky ostatné | funguje bez siete a váš hlas nikdy neopustí Mac; Parakeet je zo všetkých najrýchlejší |
| **Žiadny účet, žiadne náklady** | ľubovoľný model v časti **Lokálne** | beží na vašom Macu; nič sa neúčtuje |
| **Online, ale zadarmo** | **Groq** s bezplatným plánom | účet Groq bez platobnej metódy; prepis v rámci denných limitov Groq nič nestojí |
| **Spoľahlivosť aj rýchlosť** | online model a k tomu nainštalovaný Parakeet Ultra alebo model Whisper | lokálny model zaskočí, keď online služba zlyhá |
| **Prepis na Macu s procesorom Intel** | **Groq** alebo **OpenAI** | lokálne modely tam bežia na procesore, pozrite [Macy s procesorom Intel](#macy-s-procesorom-intel) |

Lokálne modely zaberajú na disku toľko, koľko je ich veľkosť na stiahnutie, a keď sú zvolené, približne rovnako veľa pamäte; Lowspeak drží model načítaný, aby sa diktovanie začalo bez oneskorenia. Na Macu s 8 GB pamäte uprednostnite Parakeet Ultra alebo online model pred veľkými modelmi Whisper.

### Online: OpenAI a Groq

| Model | Poskytovateľ | Poznámky |
|---|---|---|
| **GPT Transcribe** (predvolený) | OpenAI | najpresnejší; váš slovník preberá ako kľúčové slová |
| **Whisper** | OpenAI | pôvodný model Whisper |
| **Whisper Large V3** | Groq | veľmi rýchly |
| **Whisper Large V3 Turbo** | Groq | najrýchlejší online |

Nahrávka sa pred odoslaním skomprimuje do AAC, asi 6 KB na sekundu reči, takže diktovanie funguje aj na pomalom pripojení. Kým držíte fn, Lowspeak už otvára spojenie s poskytovateľom, takže odosielanie sa začne vo chvíli, keď kláves pustíte.

### Na vašom Macu: Whisper a Parakeet

| Model | Veľkosť | Beží na | Jazyky |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 európskych jazykov |
| **Parakeet v2** | ~600 MB | Neural Engine | angličtina |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | asi 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | asi 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | asi 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | asi 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | asi 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | asi 99 |

**Spravovať modely** ich sťahuje a vymazáva; počas sťahovania ukazuje lišta menu priebeh a ukončenie Lowspeaku sťahovanie zruší. Úplne prvé použitie modelu ho pripraví pre váš Mac: asi 10 sekúnd pri Parakeet Ultra, až 30 sekúnd pri Whisperi; potom sa načíta za zlomok sekundy. Ak zvolený model vymažete, Lowspeak prepne na iný nainštalovaný.

**Parakeet** je najrýchlejšia voľba: 40 sekúnd reči prepíše asi za 0,3 sekundy. Lowspeak ho spúšťa s rovnakými modelmi a rovnakým dekódovaním ako [FluidAudio](https://github.com/FluidInference/FluidAudio) a ukladá modely do priečinka FluidAudio (`~/Library/Application Support/FluidAudio/Models`), takže ich zdieľajú aj aplikácie postavené na FluidAudio. Nahrávky dlhšie ako 15 sekúnd sa prepisujú v prekrývajúcich sa oknách, ktoré sa spoja bez straty slov na švoch. **Whisper** rozumie viacerým jazykom a číta váš slovník; veľké modely sú najpresnejšie, malé sú mimo angličtiny oveľa slabšie. Modely Whisper sa ukladajú do `~/Library/Application Support/Lowspeak/Models`.

### Keď online služba zlyhá

Ak online prepis zlyhá, pretože nefunguje sieť, poskytovateľ má problém alebo je kľúč nesprávny, prepíše tú istú nahrávku nainštalovaný lokálny model: ten zvolený v časti **Lokálne**, inak Parakeet Ultra, potom Whisper Large v3 Turbo, Large v3 a menšie modely Whisper a ako posledný Parakeet v2. Hlásenie vás na to upozorní raz; znova sa objaví až potom, ako medzitým uspeje online diktát.

S lokálnym modelom v zálohe dáva Lowspeak online požiadavke 20 sekúnd; bez neho to skúsi trikrát, zakaždým až minútu. V online režime Lowspeak pri spustení raz pripraví záložný model Parakeet, aby naň prvé zaskočenie nemuselo čakať. Preťažený poskytovateľ, ktorý požiada o prestávku, ju dostane, a to najviac na desať sekúnd; na toho, ktorý žiada dlhšiu alebo hlási, že váš účet už nemá žiadny kredit, sa Lowspeak znova neobráti.

Diktát, ktorý aj tak zlyhá, sa nestratí: Lowspeak si jeho nahrávku ponechá v pamäti a **Znova skúsiť neúspešný diktát** v menu ju prepíše znova, napríklad keď sieť opäť funguje alebo keď zvolíte iný model. Uchováva sa iba posledná neúspešná nahrávka a po ukončení Lowspeaku žiadna.

### Ako sa nahrávka pripravuje

Lowspeak nahráva vo vlastnej vzorkovacej frekvencii vášho mikrofónu a po skončení nahrávku prevedie na 16 kHz, ktoré modely očakávajú. Pred prepisom odstráni hlboké hučanie pod 80 Hz, oreže ticho na začiatku a na konci a vyrovná hlasitosť, takže tichí aj hlasní hovoriaci vyjdú rovnako. Nahrávka bez reči sa vôbec neodošle. V lokálnom režime dostane Parakeet najprv nedotknutú nahrávku a pripravenú iba vtedy, keď z tej prvej nevznikne žiadny text.

Whisper niekedy do ticha a šumu napíše text, ktorý nikto nepovedal, napríklad údaje o autoroch titulkov z videí alebo slovník, ktorý dostal. Lowspeak zahodí úseky, ktoré Whisper sám označí ako ticho, a takýto text pred vložením odstráni.

### Jazyky podľa modelu

Jazyk zvolený v časti **Jazyk** sa odovzdáva online modelom a Whisperu. Pri **Rozpoznať automaticky** model určí jazyk sám.

| Model | Jazyky, v ktorých môžete diktovať |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, na vašom Macu) | asi 99, medzi nimi všetkých 35 z menu |
| Parakeet Ultra | bulharčina, chorvátčina, čeština, dánčina, holandčina, angličtina, estónčina, fínčina, francúzština, nemčina, gréčtina, maďarčina, taliančina, lotyština, litovčina, maltčina, poľština, portugalčina, rumunčina, ruština, slovenčina, slovinčina, španielčina, švédčina, ukrajinčina |
| Parakeet v2 | angličtina |

Parakeet rozpozná jazyk sám a voľbu v menu ignoruje. **Čínština (tradičná)** povie modelu GPT Transcribe, aby písal tradičnými znakmi; Whisper píše čínštinu ktorýmkoľvek z oboch písiem a portugalčinu bez ohľadu na variant.

### Macy s procesorom Intel

Lowspeak je univerzálna aplikácia a beží aj na Macoch s procesorom Intel so systémom macOS 14. Všetko online funguje rovnako. Lokálne modely tam však nemajú hardvér, pre ktorý sú stavané: Whisper beží na procesore namiesto GPU a Parakeet na procesore namiesto Neural Engine. Oboje funguje, ale trvá to mnohonásobne dlhšie ako na Apple Silicon, pri veľkých modeloch Whisper príliš dlho na diktovanie. Na Macu s procesorom Intel prepisujte online a Whisper Small alebo Parakeet Ultra si nechajte ako [zálohu](#keď-online-služba-zlyhá).

## Slovník

**Slovník…** prijíma mená a výrazy, ktoré má Lowspeak písať presne, oddelené čiarkami: názvy produktov, mená kolegov, odborné výrazy. Až 100 výrazov, každý najviac 60 znakov; `<` a `>` sa odstránia.

- **GPT Transcribe** ich dostane ako kľúčové slová.
- **Whisper** (OpenAI, Groq aj na vašom Macu) ich číta ako nápovedu pred nahrávkou, až 500 znakov.
- **Následné spracovanie** dostane pokyn písať ich presne tak, ako sú zapísané.
- **Parakeet** ich nepočuje. Kde napísal niečo veľmi podobné, nahradí to Lowspeak v jeho texte ich zápisom: „Träfig“ alebo „Trafik“ sa zmení na Traefik, „Los Peak“ na Lowspeak. Výrazy kratšie ako päť písmen, slová v adresách a názvoch súborov a tvary s koncovkou, napríklad „Lowspeaks“, zostanú tak, ako boli napísané. Aby Lowspeak odlíšil chybný zápis od skutočného slova, pýta sa kontroly pravopisu v macOS, a to na vašom Macu.

## Následné spracovanie

Po prepise môže jazykový model text prepracovať. Vyberte v časti **Následné spracovanie**:

- **Vypnuté** (predvolené) – text tak, ako bol prepísaný.
- **Vyčistenie** – odstráni výplňové slová ako „ehm“, „vieš“ alebo „v podstate“, opraví gramatiku a interpunkciu a keď sa opravíte, ponechá iba opravu. Nič nepridáva a nemení význam.
- **Formátovanie** – ako Vyčistenie a navyše prispôsobí text aplikácii, do ktorej diktujete: výrazy v štýle kódu v editoroch kódu, príkazové riadky v termináloch, neformálny tón v chatovacích aplikáciách, profesionálny v e-maile.

Následné spracovanie používa Groq (`openai/gpt-oss-120b`), ak máte kľúč Groq, inak OpenAI (`gpt-4o-mini`), a to aj keď prepisujete lokálne. Keď je v časti **Jazyk** zvolený jazyk, zachová ho. Ak trvá dlhšie ako osem sekúnd, zlyhá alebo vráti odpoveď, ktorá je useknutá alebo oveľa dlhšia ako to, čo ste povedali, Lowspeak vloží nespracovaný text. Ak poskytovateľ požiadavku odmietne alebo nie je nastavený žiadny kľúč, hlásenie na to raz upozorní.

## Mikrofón

V časti **Mikrofón** vyberte pevne zvolený mikrofón alebo **Systémová predvoľba**. Lowspeak si zariadenie pamätá; kým nie je pripojené, menu ho ukazuje ako **(nepripojené)** a Lowspeak nahráva so systémovým predvoleným mikrofónom.

Mikrofón sa zapína iba počas diktovania. Jeho príprava trvá asi 0,07 sekundy pri vstavanom mikrofóne, asi pol sekundy pri Studio Display a jednu až dve sekundy pri AirPods, ktoré sa najprv prepínajú do režimu headsetu. Ikona nahrávania sa objaví, hneď ako príde zvuk; **Prehrať zvuk po pripravení** v tej chvíli pridá krátky zvuk. Predvolene je to vypnuté, pretože na niektorých reproduktoroch zaznie samotný zvuk o pol sekundy neskôr.

Ak mikrofón päť sekúnd dodáva iba ticho, ako AirPods, keď ich mikrofón drží iné zariadenie, Lowspeak vám to oznámi.

## Kľúče API

**Nastaviť kľúč API…** si vypýta kľúč a rozpozná poskytovateľa podľa jeho začiatku: `sk-` pre OpenAI, `gsk_` pre Groq. Kľúč v inom formáte sa s vysvetlením odmietne. Zadajte jeden kľúč pre každého poskytovateľa; s kľúčmi pre oboch sú k dispozícii všetky online modely. **Zmeniť kľúč API…** kľúč nahradí alebo kľúče pomocou **Odstrániť všetky kľúče** vymaže.

Kľúče sú uložené vo vašej prihlasovacej kľúčenke pod názvom `com.lowspeak.app`. Súbor s kľúčmi z predchádzajúcej verzie (`~/Library/Application Support/Lowspeak/.api-key`) sa tam pri najbližšom spustení presunie a vymaže sa, keď sa každý kľúč podarí z kľúčenky znova prečítať.

## Hlásenia

Lowspeak pracuje na pozadí, preto problémy oznamuje formou hlásení:

| Hlásenie | Čo sa stalo |
|---|---|
| **Nahrávanie zlyhalo** | chýba prístup k mikrofónu, nie je pripojený žiadny mikrofón alebo sa mikrofón nepodarilo spustiť |
| **Mikrofón bez signálu** | päť sekúnd ticha, pozrite [Mikrofón](#mikrofón) |
| **Vyžaduje sa povolenie na prístupnosť** | text je v schránke; povoľte Prístupnosť, aby sa vkladalo automaticky |
| **Vloženie zlyhalo** | text je v schránke; vložte ho pomocou ⌘V |
| **Prepis zlyhal** | s uvedením dôvodu: chýba kľúč, odmietnutý kľúč, vyčerpaný kredit, limit požiadaviek, chyba servera alebo siete, alebo chýba lokálny model; nahrávka zostane uchovaná pre **Znova skúsiť neúspešný diktát** |
| **Diktát nebol vložený** | kým bol text hotový, do popredia sa dostala iná aplikácia; text je v schránke |
| **Prepísané na tomto Macu** | online služba zlyhala a prácu prevzal lokálny model |
| **Následné spracovanie zlyhalo** | poskytovateľ odmieta následné spracovanie alebo nie je nastavený kľúč; text bol vložený bez neho |
| **Chýba kľúč API** | zvolili ste online model bez kľúča pre jeho poskytovateľa |
| **Sťahovanie zlyhalo**, **Vymazanie zlyhalo**, **Nedá sa vymazať** | správa lokálneho modelu nefungovala alebo ešte prebiehala iná operácia |
| **Chyba otvárania po prihlásení**, **Chyba kľúča API** | macOS alebo kľúčenka zmenu odmietla |

Hlásenia nikdy neobsahujú váš nadiktovaný text. Nefunkčná online služba a poskytovateľ následného spracovania, ktorý požiadavky odmieta, sa hlásia raz, nie po každom diktáte, kým znova nezačnú fungovať.

## Vzhľad a prístupnosť

- **Ikona na lište menu** je obrázok šablóny, ktorý sa prispôsobí svetlej, tmavej alebo tónovanej lište menu.
- **VoiceOver** číta ikonu ako „Lowspeak“ a menu, dialógy aj hlásenia sú štandardné ovládacie prvky macOS.
- **Nie je potrebné držať žiadny kláves.** Uzamknite nahrávanie klávesom Shift alebo dvojitým stlačením fn a ďalšie stlačenie ho ukončí.
- **Stručný prehľad** je jednoduché okno s popismi a tlačidlom, ovládateľné z klávesnice; zatvoríte ho klávesom Esc.

Známe nedostatky uvádza [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver neoznamuje začiatok nahrávania a Lowspeak ešte nebol vyskúšaný s VoiceOverom ani so Switch Control.

## Jazyky

Menu, dialógy a hlásenia sú preložené do 35 jazykov:

arabčina, katalánčina, čínština (zjednodušená), čínština (tradičná), chorvátčina, čeština, dánčina, holandčina, angličtina, fínčina, francúzština, nemčina, gréčtina, hebrejčina, hindčina, maďarčina, indonézština, taliančina, japončina, kórejčina, malajčina, nórčina, poľština, portugalčina (Brazília), portugalčina (Portugalsko), rumunčina, ruština, slovenčina, slovinčina, španielčina, švédčina, thajčina, turečtina, ukrajinčina a vietnamčina.

Jazyk, ktorý zvolíte v časti **Jazyk**, je zároveň jazykom menu. Pri **Rozpoznať automaticky** sa menu riadi poradím jazykov v **Systémové nastavenia › Všeobecné › Jazyk a región** a tam, kde preklad nie je, použije angličtinu. Pojmy v menu vychádzajú z vlastných prekladov Applu, takže Lowspeak pôsobí ako zvyšok vášho Macu. Okrem angličtiny sú preklady strojové a rodení hovoriaci ich zatiaľ neskontrolovali; opravy môžete poslať ako [issue](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Klávesy a gestá

| Akcia | Klávesy |
|---|---|
| Diktovať, kým je kláves podržaný | podržte fn |
| Uzamknúť prebiehajúce nahrávanie | Shift počas držania fn |
| Spustiť uzamknuté nahrávanie | Shift+fn alebo rýchlo dvakrát fn |
| Ukončiť uzamknuté nahrávanie | fn alebo Shift |
| Ukončiť Lowspeak (menu otvorené) | ⌘Q |
| Zatvoriť Stručný prehľad | Esc |

Lowspeak nepúšťa stlačenia fn, ktoré sám používa, do iných aplikácií; Shift funguje ako zvyčajne.

## Súkromie a zabezpečenie

Lowspeak počuje, čo hovoríte, kým držíte fn, môže písať do ktorejkoľvek aplikácie a uchováva vaše kľúče API, preto s tým všetkým robí čo najmenej.

- **Váš hlas ide na jedno miesto.** V online režime ide skomprimovaná nahrávka k poskytovateľovi, ktorého ste si vybrali, OpenAI alebo Groq, spolu s vaším slovníkom. V lokálnom režime nič neopustí váš Mac. Keď online požiadavka zlyhá, lokálny model prepíše tú istú nahrávku na vašom Macu.
- **Následné spracovanie**, ak ho zapnete, posiela prepísaný text a váš slovník do Groq alebo OpenAI. V režime Formátovanie pridáva druh aplikácie, do ktorej diktujete (editor kódu, terminál, chat alebo e-mail); názov aplikácie sa neposiela.
- **Žiadna telemetria, žiadny účet.** Okrem prepisu a následného spracovania sa Lowspeak pripája na internet iba preto, aby otvoril spojenie s vaším poskytovateľom, kým držíte fn, v online režime alebo so zapnutým následným spracovaním, aby stiahol model, keď oň požiadate, z Hugging Face cez HTTPS, v pevne danej verzii a s overením každého súboru podľa kontrolného súčtu, a aby na GitHube vyhľadal novú verziu, raz denne, ak to povolíte. Táto požiadavka nesie verziu Lowspeaku a nič iné. **Pomocník › Nahlásiť problém…** otvorí v prehliadači formulár na GitHube; jeho adresa nesie verziu Lowspeaku a verziu macOS, nič z toho, čo ste nadiktovali.
- **Kľúče API** zostávajú vo vašej prihlasovacej kľúčenke. Posielajú sa iba ich vlastnému poskytovateľovi a z chybových správ sa odstránia skôr, než sa čokoľvek zapíše do protokolu.
- **Nadiktovaný text sa neukladá.** Posledných desať diktátov zostáva v pamäti pre **Nedávne diktáty** a po ukončení Lowspeaku zmizne, rovnako ako nahrávka posledného neúspešného diktátu, uchovávaná pre **Znova skúsiť neúspešný diktát**. Súbory protokolu zaznamenávajú dĺžky a časy, nikdy text.
- **Schránka** sa pred vložením uloží a obnoví, keď aplikácia text prečíta, ak ju medzitým nezmenila iná aplikácia. Diktát je označený ako dočasný, takže ho aplikácie s históriou schránky, ktoré toto označenie rešpektujú, preskočia.
- **Podpísanie kódu** s Hardened Runtime; aplikácia žiada iba o mikrofón, Prístupnosť, hlásenia a sieť. Aktualizácia musí niesť podpis vývojára, inak ju Sparkle odmietne.

Ak chcete nahlásiť zraniteľnosť, postupujte podľa [bezpečnostnej politiky](../../../SECURITY.md).

## Rýchlosť

Merané na Apple Silicon:

- **Parakeet Ultra** prepíše 40 sekúnd reči asi za 0,3 sekundy.
- **Online:** prvá požiadavka po prestávke trvá pri Groq asi 0,4 sekundy, pretože Lowspeak otvára spojenie, ešte kým hovoríte; ďalšie požiadavky ho znova využijú, cez HTTP/3 tam, kde to poskytovateľ ponúka.
- **Veľkosť odosielaných dát:** asi 6 KB na sekundu reči, ako AAC s dátovým tokom 48 kbit/s.
- **Príprava mikrofónu:** asi 0,07 sekundy pri vstavanom, pol sekundy pri Studio Display, jedna až dve sekundy pri AirPods.

## Známe obmedzenia

- **Lokálne modely na Macoch s procesorom Intel** bežia na procesore a sú oveľa pomalšie; tam je lepšou voľbou prepis online.
- **Čo poviete predtým, než je mikrofón pripravený, sa stratí.** Počkajte na ikonu nahrávania, najmä s AirPods.
- **Kláves fn si môžu zabrať iní.** Keď si fn skôr zaberie nastavenie klávesu 🌐 v macOS, Warp alebo Diktovanie v macOS, Lowspeak ho nevidí; pozrite [Riešenie problémov](#riešenie-problémov).
- **Parakeet** ignoruje voľbu jazyka a pozná iba európske jazyky. Slovník jeho zápis iba dodatočne opravuje: výraz, ktorý model počul ako značne odlišné slovo, zostane nesprávny.
- **Písmo a variant:** iba modelu GPT Transcribe sa dá povedať, aby písal tradičnou čínštinou; žiadnemu modelu sa nedá povedať, aby písal európskou portugalčinou namiesto brazílskej.
- **Časové limity:** dve minúty pri držaní fn, päť minút v uzamknutom režime.
- **Režim mikrofónu:** Ovládacie centrum ponúka pre Lowspeak Izoláciu hlasu a Široké spektrum a hlási ich ako aktívne, ale Lowspeak nahráva bez spracovania hlasu, takže voľba nemá žiadny účinok.
- **Nedávne diktáty** existujú iba v pamäti a po ukončení zmiznú.
- **Zámerne:** žiadne okno s nastaveniami, žiadna ikona v Docku, nie v App Store.

## Riešenie problémov

**Keď podržím fn, nič sa nestane.** Skontrolujte tri veci: Lowspeak je povolený v **Systémové nastavenia › Súkromie a bezpečnosť › Prístupnosť** (v macOS 27: **Ovládanie zariadenia a prístup k dátam**); voľba **Systémové nastavenia › Klávesnica › Stlačenie klávesu 🌐** je nastavená na **Neurobí nič**; a žiadna iná aplikácia si nezaberá fn. Warp to predvolene robí; vypnite to pomocou `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` a Warp reštartujte. Aj Diktovanie v macOS sa dá nastaviť tak, aby sa spúšťalo klávesom fn.

**Po aktualizácii Lowspeak už nereaguje na fn.** macOS viaže povolenie na prístupnosť na podpis aplikácie. Ak sa podpis zmenil, napríklad pri prechode z vlastného zostavenia na stiahnutú aplikáciu, odstráňte Lowspeak zo zoznamu Prístupnosť (v macOS 27: Ovládanie zariadenia a prístup k dátam) a pridajte ho znova.

**Chýbajú prvé slová.** Začnite hovoriť, keď sa objaví ikona nahrávania. S AirPods potrebuje mikrofón jednu až dve sekundy; vstavaný alebo USB mikrofón zvolený v časti **Mikrofón** je pripravený takmer okamžite.

**„Mikrofón bez signálu“.** Mikrofón päť sekúnd nedodal žiadny zvuk. Pri AirPods môže ich mikrofón používať iné zariadenie; prepnite na vstavaný mikrofón alebo ich znova pripojte. Zatvorené veko MacBooku tiež stlmí vstavaný mikrofón.

**Diktáty vychádzajú prázdne.** Hlasitosť vstupu môže byť príliš nízka: aplikácie ako Teams, Zoom alebo Meet ju počas hovorov menia a macOS si ju pamätá pre každé zariadenie zvlášť. Zvýšte ju v **Systémové nastavenia › Zvuk › Vstup**.

**Text skončil na nesprávnom mieste.** **Nedávne diktáty** v menu ho znova skopírujú do schránky.

**„Chýba kľúč API“.** V menu zvoľte **Nastaviť kľúč API…** alebo prepnite na lokálny model.

**Prvý lokálny prepis trvá dlho.** Každý model sa pri prvom použití pripraví pre váš Mac; odvtedy sa načíta za zlomok sekundy.

**Lowspeak sa hneď po spustení ukončí.** Už beží iná kópia Lowspeaku; najprv ju ukončite.

**Nezobrazujú sa žiadne hlásenia.** Povoľte ich v **Systémové nastavenia › Hlásenia › Lowspeak**.

**Niečo iné.** Protokoly v `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) opisujú, čo sa stalo, bez vášho nadiktovaného textu. Priložte príslušné riadky k [hláseniu problému](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Pre vývojárov

Zostavenie, testovanie, štruktúra kódu a pravidlá, ktorými sa riadi, sú opísané v [docs/development.md](../../../docs/development.md), pravidlá pre príspevky v [CONTRIBUTING.md](../../../CONTRIBUTING.md) (po anglicky).

### Komponenty tretích strán

| Komponent | Verzia | Na čo sa používa | Licencia |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) cez [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | lokálny Whisper na GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | dekódovanie Parakeet a spájanie dlhých nahrávok, preložené do Rustu v `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) a jeho crates pre frameworky | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation a ďalšie systémové frameworky | MIT; Zlib, Apache-2.0 alebo MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | záznam z mikrofónu cez CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | prevzorkovanie na 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) s rustls | 3.5 | sťahovanie modelov | MIT alebo Apache-2.0; rustls a ring pod licenciami ISC a Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | kontrolné súčty sťahovaných modelov | MIT alebo Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | kľúče API v kľúčenke | MIT alebo Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, chyby, slovník Parakeet | MIT alebo Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | aktualizácie aplikácie z obrazu disku | MIT, s časťami pod licenciami BSD a zlib |

Licenčné texty všetkého, čo aplikácia obsahuje, sa dodávajú spolu s ňou ako `Contents/Resources/Acknowledgements.txt`.

Modely nie sú súčasťou Lowspeaku; Lowspeak ich stiahne, keď o to požiadate:

| Model | Zdroj | Licencia |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper od OpenAI vo formáte ggml) | MIT |
| Parakeet v2, Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference) – konverzie modelu Parakeet TDT od NVIDIA do CoreML | CC-BY-4.0 |

## Licencia

Lowspeak je licencovaný podľa jednej z týchto licencií

- Licencia Apache, verzia 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licencia MIT ([LICENSE-MIT](../../../LICENSE-MIT))

podľa vášho výberu.

Ak výslovne neuvediete inak, každý príspevok, ktorý ste zámerne odovzdali na zahrnutie do Lowspeaku, ako je definované v licencii Apache-2.0, bude duálne licencovaný ako vyššie, bez ďalších podmienok.
