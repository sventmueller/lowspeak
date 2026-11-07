<p><img src="../../readme-icon.png" alt="Ikona aplikace Lowspeak" width="168" height="168"></p>

# Lowspeak

**Diktování pro Mac, které píše tam, kde máte kurzor: podržte fn, mluvte, pusťte.**

Lowspeak sídlí na řádku nabídek. Podržte klávesu fn (Glóbus) a mluvte; jakmile ji pustíte, vaše slova se objeví v aplikaci, ve které právě píšete, jako byste je napsali sami. Přepisuje online přes OpenAI nebo Groq, nebo zcela na vašem Macu pomocí Whisperu nebo NVIDIA Parakeet, který promění 40 sekund řeči v text zhruba za třetinu sekundy. Když selže síť nebo online služba, převezme práci nainstalovaný místní model, takže se žádný diktát neztratí. Naučte ho jména a výrazy, které používáte, nechte jazykový model odstranit vycpávková slova, pokud chcete, a uzamkněte nahrávání klávesou Shift, abyste mohli delší texty diktovat bez rukou. Je napsaný v Rustu na skutečných ovládacích prvcích AppKit, mluví 35 jazyky a váš hlas posílá jen poskytovateli, kterého jste si vybrali.

[![Stáhnout Lowspeak pro macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Stáhnout Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: podepsáno a notarizováno Applem, pro macOS 14 nebo novější na Apple Silicon i Intelu.

**Našli jste chybu?** [Založte issue](https://github.com/sventmueller/lowspeak/issues/new/choose) nebo v nabídce Lowspeaku zvolte **Nápověda › Nahlásit problém…**: formulář se otevře s již vyplněnou verzí. Pokud je pro vás Lowspeak užitečný, [hvězdička na GitHubu](https://github.com/sventmueller/lowspeak) pomůže ostatním ho najít.

[![Rust edice 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 nebo novější](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Univerzální: Apple Silicon a Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit přes objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp na Metalu](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet na Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online s OpenAI nebo Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 jazyků rozhraní](https://img.shields.io/badge/languages-35-4C9A2A)
![Podepsáno s Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licence: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licence)

[Bahasa Indonesia](../id/README.md) ·
[Bahasa Melayu](../ms/README.md) ·
[Català](../ca/README.md) ·
**Čeština** ·
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

- [Přehled funkcí](#přehled-funkcí)
- [Požadavky](#požadavky)
- [Instalace](#instalace)
- [Začínáme](#začínáme)
  - [Oprávnění](#oprávnění)
  - [Volba způsobu přepisu](#volba-způsobu-přepisu)
  - [Nabídka](#nabídka)
  - [Stručný přehled](#stručný-přehled)
- [Diktování](#diktování)
  - [Držte a mluvte](#držte-a-mluvte)
  - [Uzamčení klávesou Shift nebo dvojím stisknutím](#uzamčení-klávesou-shift-nebo-dvojím-stisknutím)
  - [Kam se text vloží](#kam-se-text-vloží)
  - [Poslední diktáty](#poslední-diktáty)
- [Přepis](#přepis)
  - [Který model se pro vás hodí](#který-model-se-pro-vás-hodí)
  - [Online: OpenAI a Groq](#online-openai-a-groq)
  - [Na vašem Macu: Whisper a Parakeet](#na-vašem-macu-whisper-a-parakeet)
  - [Když online služba selže](#když-online-služba-selže)
  - [Jak se nahrávka připravuje](#jak-se-nahrávka-připravuje)
  - [Jazyky podle modelu](#jazyky-podle-modelu)
  - [Macy s procesorem Intel](#macy-s-procesorem-intel)
- [Slovník](#slovník)
- [Následné zpracování](#následné-zpracování)
- [Mikrofon](#mikrofon)
- [Klíče API](#klíče-api)
- [Oznámení](#oznámení)
- [Vzhled a zpřístupnění](#vzhled-a-zpřístupnění)
- [Jazyky](#jazyky)
- [Klávesy a gesta](#klávesy-a-gesta)
- [Soukromí a zabezpečení](#soukromí-a-zabezpečení)
- [Rychlost](#rychlost)
- [Známá omezení](#známá-omezení)
- [Řešení potíží](#řešení-potíží)
- [Pro vývojáře](#pro-vývojáře)
  - [Komponenty třetích stran](#komponenty-třetích-stran)
- [Licence](#licence)

## Přehled funkcí

- **Podržte fn, mluvte, pusťte.** Text se napíše do aplikace v popředí, tam, kde je kurzor: do pošty, chatu, editoru, terminálu nebo prohlížeče. Žádné okno, na které byste se museli přepínat, žádné tlačítko ke kliknutí.
- **Uzamčení pro delší texty.** Stiskněte Shift, zatímco držíte fn, nebo stiskněte fn dvakrát a nahrávání poběží bez držení klávesy až pět minut. Jedno stisknutí fn nebo Shiftu ho ukončí.
- **Správný model pro každý účel.** Groq pro rychlost bez zatížení paměti Macu, GPT Transcribe od OpenAI pro nejlepší výsledek, Whisper nebo NVIDIA Parakeet na vašem Macu pro práci offline a soukromí vašeho hlasu. Přepnete jedním kliknutím.
- **Žádný diktát se neztratí.** Když selže síť nebo online služba, nainstalovaný místní model přepíše stejnou nahrávku na vašem Macu.
- **Vaše slova, správně napsaná.** Slovník jmen a odborných výrazů vede OpenAI, Groq, Whisper i následné zpracování.
- **Čistý text, pokud o něj stojíte.** Volitelné následné zpracování odstraní vycpávková slova, opraví gramatiku, a když se opravíte, ponechá jen opravu; případně text také naformátuje podle druhu aplikace, do které diktujete.
- **Mluvte dál.** Začněte další diktát, zatímco se předchozí ještě přepisuje; texty dorazí v pořadí, v jakém jste je vyslovili.
- **Schránka zůstane vaše.** Lowspeak vkládá přes schránku a chvíli nato do ní vrátí to, co v ní bylo. Posledních deset diktátů zůstává v nabídce, abyste je mohli znovu zkopírovat.
- **Nativní do posledního detailu.** Skutečná položka na řádku nabídek s nabídkou AppKit, standardními dialogy a oznámeními. Žádný Electron, žádné okno, žádná ikona v Docku.
- **Soukromí.** Žádný účet, žádná telemetrie. Zvuk jde jen k poskytovateli, kterého jste si vybrali, a v místním režimu nikam.
- **Ve vašem jazyce.** Diktování funguje v desítkách jazyků a nabídka, dialogy a oznámení jsou k dispozici v 35.

## Požadavky

- macOS 14 Sonoma nebo novější. Lowspeak se vyvíjí a testuje na macOS 27.
- Mac s Apple Silicon nebo procesorem Intel. Místní přepis je určený pro Apple Silicon, kde Whisper běží na GPU a Parakeet na Neural Engine; na Macích s procesorem Intel běží oba na procesoru, mnohem pomaleji, viz [Macy s procesorem Intel](#macy-s-procesorem-intel).
- Asi 17 MB pro aplikaci a k tomu 75 MB až 3 GB pro každý stažený místní model.
- Pro přepis online klíč API od [OpenAI](https://platform.openai.com/api-keys) nebo [Groq](https://console.groq.com/keys); oba si účtují podle toho, co přepíšete. Místní přepis nevyžaduje žádný účet.

## Instalace

**Z obrazu disku.** Stáhněte si [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), otevřete ho a přetáhněte Lowspeak do složky Aplikace. Obraz je podepsaný certifikátem Developer ID a notarizovaný Applem, takže se otevře bez varování.

**Aktualizace.** Lowspeak z obrazu disku se aktualizuje sám pomocí [Sparkle](https://sparkle-project.org). Při druhém spuštění se zeptá, zda smí jednou denně sám vyhledávat aktualizace. **Vyhledat aktualizace…** v nabídce vyhledá hned; **Vyhledávat aktualizace automaticky** a **Stahovat a instalovat aktualizace automaticky** pod ní vaši volbu později změní. Když kontrola najde novou verzi, zatímco pracujete v jiné aplikaci, objeví se na kroužku na řádku nabídek tečka a **Vyhledat aktualizace…** přenese aktualizaci do popředí. Lowspeak nainstaluje aktualizaci, jen pokud nese podpis vývojáře; vaše nastavení, klíče i modely zůstanou zachovány. Verze 1.0.0 automatické aktualizace nemá: novější verzi přes ni jednou nainstalujte ze [stránky vydání](https://github.com/sventmueller/lowspeak/releases). Ani vlastní sestavení automatické aktualizace nemá.

**Ze zdrojového kódu.** Předpoklady najdete v části [Sestavení a nasazení](../../../docs/development.md#build--deploy), potom:

```sh
make deploy
```

Tím se Lowspeak sestaví, nahradí kopii ve složce Aplikace a spustí se.

**Otevřít při přihlášení** v nabídce spouští Lowspeak spolu s Macem, prostřednictvím vlastních položek při přihlášení v macOS. Najednou běží jen jedna kopie; druhá se hned ukončí.

## Začínáme

### Oprávnění

Při prvním spuštění Lowspeak požádá o tři oprávnění:

- **Zpřístupnění** (Nastavení systému › Soukromí a zabezpečení › Zpřístupnění; v macOS 27 se seznam jmenuje Ovládání zařízení a přístup k datům). Lowspeak ho potřebuje, aby zaznamenal klávesu fn a mohl vkládat pomocí ⌘V. macOS nabídne otevření seznamu; zapněte přepínač u Lowspeaku. Lowspeak ve svých zprávách uvádí název seznamu tak, jak ho zobrazuje váš macOS.
- **Mikrofon.** Žádost se zobrazí jednou; bez tohoto oprávnění Lowspeak nic nenahraje.
- **Oznámení.** Lowspeak jimi hlásí problémy, kterých byste si jinak nevšimli, například tichý mikrofon nebo neúspěšný přepis.

### Volba způsobu přepisu

Nastavte alespoň jeden způsob; v nabídce můžete kdykoli přepnout.

- **Online:** zvolte **Nastavit klíč API…** a vložte klíč od OpenAI (začíná na `sk-`) nebo Groq (`gsk_`). Lowspeak pozná poskytovatele podle klíče. Pak vyberte model v části **OpenAI** nebo **Groq**.
- **Na vašem Macu:** zvolte **Spravovat modely** › **Stáhnout Parakeet Ultra** nebo některý model Whisper a pak ho vyberte v části **Místní**.

S výběrem pomůže [Který model se pro vás hodí](#který-model-se-pro-vás-hodí). Potom podržte fn, něco řekněte a pusťte.

### Nabídka

Klikněte na kroužek na řádku nabídek:

| Položka | Co dělá |
|---|---|
| **Poslední diktáty** | posledních deset textů; kliknutím jeden z nich znovu zkopírujete do schránky |
| **Místní** | modely na vašem Macu, seskupené na Whisper a Parakeet; nestažené jsou zašedlé |
| **OpenAI**, **Groq** | online modely; bez klíče jsou zašedlé a nabídka místo nich ukazuje **Nastavit klíč API…** |
| **Jazyk** | jazyk, ve kterém diktujete, nebo **Rozpoznat automaticky**; nabídka se jím řídí |
| **Následné zpracování** | **Vypnuto**, **Vyčištění** nebo **Formátování**, viz [Následné zpracování](#následné-zpracování) |
| **Slovník…** | jména a výrazy, které se mají psát přesně; nabídka ukazuje, kolik jste jich zadali |
| **Mikrofon** | **Výchozí systémový** nebo pevně zvolený mikrofon a **Zvuk při připravenosti** |
| **Spravovat modely** | stahování a mazání místních modelů, s průběhem |
| **Nastavit klíč API…** / **Změnit klíč API…** | přidání, nahrazení nebo odstranění klíčů |
| **Otevřít při přihlášení** | spouštění Lowspeaku spolu s Macem; zaškrtnutí ukazuje, že je volba zapnutá |
| **Vyhledat aktualizace…** | vyhledání nové verze hned teď; dvě položky pod ní rozhodují, zda Lowspeak hledá jednou denně a instaluje aktualizace bez ptaní. Jen v aplikaci z obrazu disku |
| **Nápověda** | **Stručný přehled**, **Nahlásit problém…**, **Navrhnout funkci…**, **Lowspeak na GitHubu** |
| **O aplikaci Lowspeak** | verze, copyright, stránka projektu a licence |
| **Ukončit Lowspeak** | ⌘Q, když je nabídka otevřená |

Zaškrtnutí v **Místní**, **OpenAI** a **Groq** označuje používaný model. Ikona ukazuje, co Lowspeak právě dělá:

| Ikona | Význam |
|---|---|
| Kroužek s tečkou | Připraveno |
| Vyplněný kruh | Nahrávání, dokud držíte fn |
| Zaoblený čtverec | Nahrávání uzamčeno; další stisknutí fn nebo Shiftu ho ukončí |
| Blikající kroužek | Přepisování |
| Kroužek s tečkou vpravo nahoře | Čeká nová verze, viz **Vyhledat aktualizace…** |
| ↓ 42 % | Stahování modelu |

Lowspeak nemá okno nastavení: vše je v této nabídce a každou volbu si pamatuje.

### Stručný přehled

**Nápověda › Stručný přehled** ukáže gesta klávesy fn v malém okně, ve vašem jazyce, s tlačítkem, které otevře nastavení klávesnice. **Nahlásit problém…** otevře formulář na GitHubu s již vyplněnou verzí Lowspeaku a macOS, **Navrhnout funkci…** diskuzi pro nápady a **Lowspeak na GitHubu** stránku projektu.

## Diktování

### Držte a mluvte

Podržte fn, mluvte, pusťte. Ikona nahrávání se objeví ve chvíli, kdy mikrofon začne dodávat zvuk; teprve pak začněte mluvit, protože to, co řeknete dřív, se nenahraje. Uvolněním fn nahrávání skončí a text následuje o okamžik později.

Krátké klepnutí na fn nedělá nic, takže si fn zachová svá ostatní využití. Nahrávka, která zůstane prázdná nebo trvá méně než 0,3 sekundy, se zahodí.

### Uzamčení klávesou Shift nebo dvojím stisknutím

U delších textů nahrávání uzamkněte, aby běželo bez držení klávesy. Jsou tři způsoby:

- **Shift při držení fn.** Začněte jako obvykle s fn a během nahrávání stiskněte Shift: nahrávání se uzamkne a fn můžete pustit. Nic z toho, co jste už řekli, se neztratí, nahrávání prostě pokračuje.
- **Shift, potom fn.** Podržte Shift a stiskněte fn: nahrávání začne rovnou uzamčené.
- **Dvakrát fn.** Krátce klepněte na fn (méně než 0,3 sekundy) a do 0,4 sekundy ji stiskněte znovu. Nahrávka běží od prvního stisknutí.

Ikona se změní na zaoblený čtverec. Nahrávání ukončíte jedním stisknutím **fn** nebo **Shift**; text následuje jako obvykle. Pokud máte klávesu 🌐 nastavenou na otevírání panelu Emotikony a symboly, ukončujte nahrávání klávesou Shift: krátké stisknutí fn by otevřelo prohlížeč znaků.

Nahrávání se samo zastaví po dvou minutách, když držíte fn, a po pěti minutách v uzamčeném režimu, a pak se přepíše jako obvykle.

### Kam se text vloží

Lowspeak vloží text do aplikace, která byla v popředí, když jste přestali mluvit, na místo kurzoru, přes schránku a ⌘V. Uloží vše, co bylo předtím ve schránce, a vrátí to zpět půl sekundy poté, co aplikace text přečte, nebo po deseti sekundách, když ho nepřečte vůbec, pokud mezitím schránku nezměnila jiná aplikace. Text je označený jako dočasný, takže ho aplikace s historií schránky, které toto označení respektují, nezaznamenají.

Další diktát můžete začít, zatímco se předchozí ještě přepisuje. Texty se vkládají v pořadí, v jakém jste je vyslovili, a nikdy během nahrávání nebo když držíte Shift, takže vložený text nemůže skončit uprostřed vaší další věty.

Bez oprávnění pro zpřístupnění, nebo když nelze odeslat simulované ⌘V, zůstane text ve schránce a oznámení vám to sdělí; vložte ho sami pomocí ⌘V.

### Poslední diktáty

**Poslední diktáty** v nabídce ukazují posledních deset textů, od nejnovějšího, každý zkrácený na jeden řádek; po najetí ukazatelem se zobrazí celý text. Kliknutím ho zkopírujete do schránky, třeba když text skončil na špatném místě nebo ho potřebujete dvakrát. Seznam existuje jen v paměti a po ukončení Lowspeaku je prázdný.

## Přepis

### Který model se pro vás hodí

| Co potřebujete | Zvolte | Proč |
|---|---|---|
| **Rychlost bez zatížení Macu** | **Groq › Whisper Large V3 Turbo** | nejrychlejší online možnost; nic se nestahuje a váš Mac si zachová paměť i místo na disku |
| **Nejlepší výsledek** | **OpenAI › GPT Transcribe** (výchozí) | nejpřesnější z modelů, které Lowspeak nabízí; váš slovník přebírá jako klíčová slova |
| **Práci offline a soukromí** | **Místní › Parakeet Ultra** pro evropské jazyky, **Místní › Whisper Large v3 Turbo** pro všechny ostatní | funguje bez sítě a váš hlas nikdy neopustí Mac; Parakeet je ze všech nejrychlejší |
| **Žádný účet, žádné náklady** | libovolný model v části **Místní** | běží na vašem Macu; nic se neúčtuje |
| **Spolehlivost i rychlost** | online model a k tomu nainstalovaný Parakeet Ultra nebo model Whisper | místní model zaskočí, když online služba selže |
| **Přepis na Macu s procesorem Intel** | **Groq** nebo **OpenAI** | místní modely tam běží na procesoru, viz [Macy s procesorem Intel](#macy-s-procesorem-intel) |

Místní modely zabírají na disku tolik, kolik činí jejich velikost ke stažení, a když jsou zvolené, přibližně stejně paměti; Lowspeak drží model načtený, aby diktování začalo bez prodlení. Na Macu s 8 GB paměti dejte přednost Parakeet Ultra nebo online modelu před velkými modely Whisper.

### Online: OpenAI a Groq

| Model | Poskytovatel | Poznámky |
|---|---|---|
| **GPT Transcribe** (výchozí) | OpenAI | nejpřesnější; váš slovník přebírá jako klíčová slova |
| **Whisper** | OpenAI | původní model Whisper |
| **Whisper Large V3** | Groq | velmi rychlý |
| **Whisper Large V3 Turbo** | Groq | nejrychlejší online |

Nahrávka se před odesláním zkomprimuje do AAC, asi 6 KB na sekundu řeči, takže diktování funguje i na pomalém připojení. Zatímco držíte fn, Lowspeak už otevírá spojení s poskytovatelem, takže odesílání začne ve chvíli, kdy klávesu pustíte.

### Na vašem Macu: Whisper a Parakeet

| Model | Velikost | Běží na | Jazyky |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 evropských jazyků |
| **Parakeet v2** | ~600 MB | Neural Engine | angličtina |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | asi 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | asi 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | asi 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | asi 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | asi 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | asi 99 |

**Spravovat modely** je stahuje a maže; během stahování ukazuje řádek nabídek průběh a ukončení Lowspeaku stahování zruší. Úplně první použití modelu ho připraví pro váš Mac: asi 10 sekund u Parakeet Ultra, až 30 sekund u Whisperu; potom se načte ve zlomku sekundy. Pokud zvolený model smažete, Lowspeak přepne na jiný nainstalovaný.

**Parakeet** je nejrychlejší volba: 40 sekund řeči přepíše asi za 0,3 sekundy. Lowspeak ho spouští se stejnými modely a stejným dekódováním jako [FluidAudio](https://github.com/FluidInference/FluidAudio) a ukládá modely do složky FluidAudio (`~/Library/Application Support/FluidAudio/Models`), takže je sdílejí i aplikace postavené na FluidAudio. Nahrávky delší než 15 sekund se přepisují v překrývajících se oknech, která se spojí bez ztráty slov na švech. **Whisper** rozumí více jazykům a čte váš slovník; velké modely jsou nejpřesnější, malé jsou mimo angličtinu mnohem slabší. Modely Whisper se ukládají do `~/Library/Application Support/Lowspeak/Models`.

### Když online služba selže

Pokud online přepis selže, protože nefunguje síť, poskytovatel má problém nebo je klíč chybný, přepíše stejnou nahrávku nainstalovaný místní model: ten zvolený v části **Místní**, jinak Parakeet Ultra, potom Whisper Large v3 Turbo, Large v3 a menší modely Whisper a jako poslední Parakeet v2. Oznámení vás na to upozorní jednou; znovu se objeví až poté, co mezitím uspěje online diktát.

S místním modelem v záloze dává Lowspeak online požadavku 20 sekund; bez něj to zkusí třikrát, pokaždé až minutu. V online režimu Lowspeak při spuštění jednou připraví záložní model Parakeet, aby na něj první záskok nemusel čekat.

### Jak se nahrávka připravuje

Lowspeak nahrává ve vlastní vzorkovací frekvenci vašeho mikrofonu a po skončení převede nahrávku na 16 kHz, které modely očekávají. Před přepisem odstraní hluboké hučení pod 80 Hz, ořízne ticho na začátku a na konci a vyrovná hlasitost, takže tiší i hlasití mluvčí vyjdou stejně. Nahrávka bez řeči se vůbec neodešle. V místním režimu dostane Parakeet nejprve nedotčenou nahrávku a připravenou jen tehdy, když z té první nevznikne žádný text.

### Jazyky podle modelu

Jazyk zvolený v části **Jazyk** se předává online modelům a Whisperu. Při **Rozpoznat automaticky** model určí jazyk sám.

| Model | Jazyky, ve kterých můžete diktovat |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, na vašem Macu) | asi 99, mezi nimi všech 35 z nabídky |
| Parakeet Ultra | bulharština, chorvatština, čeština, dánština, nizozemština, angličtina, estonština, finština, francouzština, němčina, řečtina, maďarština, italština, lotyština, litevština, maltština, polština, portugalština, rumunština, ruština, slovenština, slovinština, španělština, švédština, ukrajinština |
| Parakeet v2 | angličtina |

Parakeet rozpozná jazyk sám a volbu v nabídce ignoruje. **Čínština (tradiční)** řekne GPT Transcribe, aby psal tradičními znaky; Whisper píše čínštinu kterýmkoli z obou písem a portugalštinu bez ohledu na variantu.

### Macy s procesorem Intel

Lowspeak je univerzální aplikace a běží i na Macích s procesorem Intel se systémem macOS 14. Vše online funguje stejně. Místní modely tam ale nemají hardware, pro který jsou stavěné: Whisper běží na procesoru místo na GPU a Parakeet na procesoru místo na Neural Engine. Obojí funguje, ale trvá to mnohonásobně déle než na Apple Silicon, u velkých modelů Whisper příliš dlouho na diktování. Na Macu s procesorem Intel přepisujte online a Whisper Small nebo Parakeet Ultra si nechte jako [zálohu](#když-online-služba-selže).

## Slovník

**Slovník…** přijímá jména a výrazy, které má Lowspeak psát přesně, oddělené čárkami: názvy produktů, jména kolegů, odborné výrazy. Až 100 výrazů, každý nejvýše 60 znaků; `<` a `>` se odstraní.

- **GPT Transcribe** je dostane jako klíčová slova.
- **Whisper** (OpenAI, Groq i na vašem Macu) je čte jako nápovědu před nahrávkou, až 500 znaků.
- **Následné zpracování** dostane pokyn psát je přesně tak, jak jsou zapsané.
- **Parakeet** je nepoužívá.

## Následné zpracování

Po přepisu může jazykový model text přepracovat. Vyberte v části **Následné zpracování**:

- **Vypnuto** (výchozí) – text tak, jak byl přepsán.
- **Vyčištění** – odstraní vycpávková slova jako „ehm“, „víte“ nebo „v podstatě“, opraví gramatiku a interpunkci, a když se opravíte, ponechá jen opravu. Nic nepřidává a nemění význam.
- **Formátování** – jako Vyčištění a navíc přizpůsobí text aplikaci, do které diktujete: výrazy ve stylu kódu v editorech kódu, příkazové řádky v terminálech, neformální tón v chatovacích aplikacích, profesionální v e-mailu.

Následné zpracování používá Groq (`openai/gpt-oss-120b`), pokud máte klíč Groq, jinak OpenAI (`gpt-4o-mini`), a to i když přepisujete místně. Když je v části **Jazyk** zvolený jazyk, zachová ho. Pokud trvá déle než osm sekund, selže nebo vrátí odpověď, která je useknutá nebo mnohem delší než to, co jste řekli, Lowspeak vloží nezpracovaný text. Pokud poskytovatel požadavek odmítne nebo není nastavený žádný klíč, oznámení na to jednou upozorní.

## Mikrofon

V části **Mikrofon** vyberte pevně daný mikrofon nebo **Výchozí systémový**. Lowspeak si zařízení pamatuje; dokud není připojené, nabídka ho ukazuje jako **(nepřipojeno)** a Lowspeak nahrává s výchozím systémovým mikrofonem.

Mikrofon se zapíná jen během diktování. Jeho příprava trvá asi 0,07 sekundy u vestavěného mikrofonu, asi půl sekundy u Studio Display a jednu až dvě sekundy u AirPods, které se nejprve přepínají do režimu náhlavní soupravy. Ikona nahrávání se objeví, jakmile dorazí zvuk; **Zvuk při připravenosti** v tu chvíli přidá krátký zvuk. Ve výchozím stavu je vypnutý, protože na některých reproduktorech zazní samotný zvuk o půl sekundy později.

Pokud mikrofon pět sekund dodává jen ticho, jako AirPods, když jejich mikrofon drží jiné zařízení, Lowspeak vám to oznámí.

## Klíče API

**Nastavit klíč API…** si vyžádá klíč a pozná poskytovatele podle jeho začátku: `sk-` pro OpenAI, `gsk_` pro Groq. Klíč v jiném formátu se s vysvětlením odmítne. Zadejte jeden klíč pro každého poskytovatele; s klíči pro oba jsou k dispozici všechny online modely. **Změnit klíč API…** klíč nahradí, nebo klíče pomocí **Odstranit všechny klíče** smaže.

Klíče jsou uložené ve vašem svazku klíčů „přihlášení“ pod názvem `com.lowspeak.app`. Soubor s klíči z dřívější verze (`~/Library/Application Support/Lowspeak/.api-key`) se tam při příštím spuštění přesune a smaže se, jakmile se každý klíč podaří ze svazku klíčů zpět přečíst.

## Oznámení

Lowspeak pracuje na pozadí, takže problémy hlásí formou oznámení:

| Oznámení | Co se stalo |
|---|---|
| **Nahrávání se nezdařilo** | chybí přístup k mikrofonu, není připojen žádný mikrofon nebo se mikrofon nepodařilo spustit |
| **Mikrofon bez signálu** | pět sekund ticha, viz [Mikrofon](#mikrofon) |
| **Je vyžadováno oprávnění pro zpřístupnění** | text je ve schránce; povolte Zpřístupnění, aby se vkládalo automaticky |
| **Vložení se nezdařilo** | text je ve schránce; vložte ho pomocí ⌘V |
| **Přepis se nezdařil** | s uvedením důvodu: chybí klíč, odmítnutý klíč, limit požadavků, chyba serveru nebo sítě, nebo chybí místní model |
| **Přepsáno na tomto Macu** | online služba selhala a práci převzal místní model |
| **Následné zpracování se nezdařilo** | poskytovatel odmítá následné zpracování nebo není nastavený klíč; text byl vložen bez něj |
| **Chybí klíč API** | zvolili jste online model bez klíče pro jeho poskytovatele |
| **Stahování se nezdařilo**, **Smazání se nezdařilo**, **Nelze smazat** | správa místního modelu nefungovala, nebo ještě probíhala jiná operace |
| **Chyba spouštění při přihlášení**, **Chyba klíče API** | macOS nebo svazek klíčů změnu odmítl |

Oznámení nikdy neobsahují váš nadiktovaný text. Nefunkční online služba a poskytovatel následného zpracování, který požadavky odmítá, se hlásí jednou, ne po každém diktátu, dokud zase nezačnou fungovat.

## Vzhled a zpřístupnění

- **Ikona na řádku nabídek** je šablonový obrázek, který se přizpůsobí světlému, tmavému nebo tónovanému řádku nabídek.
- **VoiceOver** čte ikonu jako „Lowspeak“ a nabídka, dialogy i oznámení jsou standardní ovládací prvky macOS.
- **Není třeba držet žádnou klávesu.** Uzamkněte nahrávání klávesou Shift nebo dvojím stisknutím fn a další stisknutí ho ukončí.
- **Stručný přehled** je jednoduché okno s popisky a tlačítkem, ovladatelné z klávesnice; zavřete ho klávesou Esc.

Známé nedostatky uvádí [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver neoznamuje začátek nahrávání a Lowspeak zatím nebyl vyzkoušen s VoiceOverem ani se Switch Control.

## Jazyky

Nabídka, dialogy a oznámení jsou přeloženy do 35 jazyků:

arabština, katalánština, čínština (zjednodušená), čínština (tradiční), chorvatština, čeština, dánština, nizozemština, angličtina, finština, francouzština, němčina, řečtina, hebrejština, hindština, maďarština, indonéština, italština, japonština, korejština, malajština, norština, polština, portugalština (Brazílie), portugalština (Portugalsko), rumunština, ruština, slovenština, slovinština, španělština, švédština, thajština, turečtina, ukrajinština a vietnamština.

Jazyk, který zvolíte v části **Jazyk**, je zároveň jazykem nabídky. Při **Rozpoznat automaticky** se nabídka řídí pořadím jazyků v **Nastavení systému › Obecné › Jazyk a oblast**, a kde překlad není, použije angličtinu. Termíny v nabídce vycházejí z vlastních překladů Applu, takže Lowspeak působí jako zbytek vašeho Macu. Kromě angličtiny jsou překlady strojové a rodilí mluvčí je zatím nezkontrolovali; opravy můžete poslat jako [issue](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Klávesy a gesta

| Akce | Klávesy |
|---|---|
| Diktovat, dokud je klávesa držena | podržte fn |
| Uzamknout probíhající nahrávání | Shift při držení fn |
| Spustit uzamčené nahrávání | Shift+fn nebo rychle dvakrát fn |
| Ukončit uzamčené nahrávání | fn nebo Shift |
| Ukončit Lowspeak (nabídka otevřená) | ⌘Q |
| Zavřít Stručný přehled | Esc |

Lowspeak nepropouští stisknutí fn, která sám používá, do jiných aplikací; Shift funguje jako obvykle.

## Soukromí a zabezpečení

Lowspeak slyší, co říkáte, dokud držíte fn, může psát do kterékoli aplikace a uchovává vaše klíče API, proto s tím vším dělá co nejméně.

- **Váš hlas jde na jedno místo.** V online režimu jde zkomprimovaná nahrávka k poskytovateli, kterého jste si vybrali, OpenAI nebo Groq, spolu s vaším slovníkem. V místním režimu nic neopustí váš Mac. Když online požadavek selže, místní model přepíše stejnou nahrávku na vašem Macu.
- **Následné zpracování**, pokud ho zapnete, posílá přepsaný text a váš slovník do Groq nebo OpenAI. V režimu Formátování přidává druh aplikace, do které diktujete (editor kódu, terminál, chat nebo e-mail); název aplikace se neodesílá.
- **Žádná telemetrie, žádný účet.** Kromě přepisu a následného zpracování se Lowspeak připojuje k internetu jen proto, aby otevřel spojení s vaším poskytovatelem, zatímco držíte fn, v online režimu nebo se zapnutým následným zpracováním, aby stáhl model, když o to požádáte, z Hugging Face přes HTTPS, v pevně dané verzi a s ověřením každého souboru podle kontrolního součtu, a aby na GitHubu vyhledal novou verzi, jednou denně, pokud to povolíte. Tento požadavek nese verzi Lowspeaku a nic jiného. **Nápověda › Nahlásit problém…** otevře v prohlížeči formulář na GitHubu; jeho adresa nese verzi Lowspeaku a verzi macOS, nic z toho, co jste nadiktovali.
- **Klíče API** zůstávají ve vašem svazku klíčů „přihlášení“. Posílají se jen jejich vlastnímu poskytovateli a z chybových zpráv se odstraní dřív, než se cokoli zapíše do protokolu.
- **Nadiktovaný text se neukládá.** Posledních deset diktátů zůstává v paměti pro **Poslední diktáty** a po ukončení Lowspeaku zmizí. Soubory protokolu zaznamenávají délky a časy, nikdy text.
- **Schránka** se před vložením uloží a obnoví, jakmile aplikace text přečte, pokud ji mezitím nezměnila jiná aplikace. Diktát je označený jako dočasný, takže ho aplikace s historií schránky, které toto označení respektují, přeskočí.
- **Podpis kódu** s Hardened Runtime; aplikace žádá jen o mikrofon, Zpřístupnění, oznámení a síť. Aktualizace musí nést podpis vývojáře, jinak ji Sparkle odmítne.

Chcete-li nahlásit zranitelnost, postupujte podle [bezpečnostních zásad](../../../SECURITY.md).

## Rychlost

Měřeno na Apple Silicon:

- **Parakeet Ultra** přepíše 40 sekund řeči asi za 0,3 sekundy.
- **Online:** první požadavek po pauze trvá u Groq asi 0,4 sekundy, protože Lowspeak otevírá spojení, ještě když mluvíte; další požadavky ho znovu využijí, přes HTTP/3 tam, kde to poskytovatel nabízí.
- **Velikost odesílaných dat:** asi 6 KB na sekundu řeči, jako AAC s datovým tokem 48 kbit/s.
- **Příprava mikrofonu:** asi 0,07 sekundy u vestavěného, půl sekundy u Studio Display, jedna až dvě sekundy u AirPods.

## Známá omezení

- **Místní modely na Macích s procesorem Intel** běží na procesoru a jsou mnohem pomalejší; tam je lepší volbou přepis online.
- **Co řeknete, než je mikrofon připravený, se ztratí.** Počkejte na ikonu nahrávání, zvlášť s AirPods.
- **Klávesu fn si mohou zabrat jiní.** Když si fn dřív zabere nastavení klávesy 🌐 v macOS, Warp nebo Diktování v macOS, Lowspeak ji nevidí; viz [Řešení potíží](#řešení-potíží).
- **Parakeet** ignoruje slovník i volbu jazyka a zná jen evropské jazyky.
- **Písmo a varianta:** jen GPT Transcribe lze říct, aby psal tradiční čínštinou; žádnému modelu nelze říct, aby psal evropskou portugalštinou místo brazilské.
- **Časové limity:** dvě minuty při držení fn, pět minut v uzamčeném režimu.
- **Režim mikrofonu:** Ovládací centrum nabízí pro Lowspeak v části Mikrofon Izolaci hlasu a Široké spektrum a hlásí je jako aktivní, ale Lowspeak nahrává bez zpracování hlasu, takže volba nemá žádný účinek.
- **Poslední diktáty** existují jen v paměti a po ukončení zmizí.
- **Záměrně:** žádné okno nastavení, žádná ikona v Docku, ne v App Store.

## Řešení potíží

**Když podržím fn, nic se nestane.** Zkontrolujte tři věci: Lowspeak je povolený v **Nastavení systému › Soukromí a zabezpečení › Zpřístupnění** (v macOS 27: **Ovládání zařízení a přístup k datům**); volba **Nastavení systému › Klávesnice › Stisknutím klávesy 🌐** je nastavená na **Nedělat nic**; a žádná jiná aplikace si nezabírá fn. Warp to ve výchozím stavu dělá; vypněte to pomocí `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` a Warp restartujte. Také Diktování v macOS lze nastavit tak, aby se spouštělo klávesou fn.

**Po aktualizaci Lowspeak na fn nereaguje.** macOS váže oprávnění pro zpřístupnění na podpis aplikace. Pokud se podpis změnil, například při přechodu z vlastního sestavení na staženou aplikaci, odstraňte Lowspeak ze seznamu Zpřístupnění (v macOS 27: Ovládání zařízení a přístup k datům) a přidejte ho znovu.

**Chybí první slova.** Začněte mluvit, až se objeví ikona nahrávání. S AirPods potřebuje mikrofon jednu až dvě sekundy; vestavěný nebo USB mikrofon zvolený v části **Mikrofon** je připravený téměř okamžitě.

**„Mikrofon bez signálu“.** Mikrofon pět sekund nedodal žádný zvuk. U AirPods může jejich mikrofon používat jiné zařízení; přepněte na vestavěný mikrofon nebo je znovu připojte. Zavřené víko MacBooku také ztlumí vestavěný mikrofon.

**Diktáty vycházejí prázdné.** Hlasitost vstupu může být příliš nízká: aplikace jako Teams, Zoom nebo Meet ji během hovorů mění a macOS si ji pamatuje pro každé zařízení zvlášť. Zvyšte ji v **Nastavení systému › Zvuk › Vstup**.

**Text skončil na špatném místě.** **Poslední diktáty** v nabídce ho znovu zkopírují do schránky.

**„Chybí klíč API“.** V nabídce zvolte **Nastavit klíč API…**, nebo přepněte na místní model.

**První místní přepis trvá dlouho.** Každý model se při prvním použití připraví pro váš Mac; od té doby se načte ve zlomku sekundy.

**Lowspeak se hned po spuštění ukončí.** Už běží jiná kopie Lowspeaku; nejdřív ji ukončete.

**Nezobrazují se žádná oznámení.** Povolte je v **Nastavení systému › Oznámení › Lowspeak**.

**Něco jiného.** Protokoly v `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) popisují, co se stalo, bez vašeho nadiktovaného textu. Připojte příslušné řádky k [hlášení problému](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Pro vývojáře

Sestavení, testování, struktura kódu a pravidla, kterými se řídí, jsou popsány v [docs/development.md](../../../docs/development.md), pravidla pro příspěvky v [CONTRIBUTING.md](../../../CONTRIBUTING.md) (anglicky).

### Komponenty třetích stran

| Komponenta | Verze | Použití | Licence |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) přes [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | místní Whisper na GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | dekódování Parakeet a spojování dlouhých nahrávek, přeloženo do Rustu v `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) a jeho crates pro frameworky | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation a další systémové frameworky | MIT; Zlib, Apache-2.0 nebo MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | záznam z mikrofonu přes CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | převzorkování na 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) s rustls | 3.5 | stahování modelů | MIT nebo Apache-2.0; rustls a ring pod licencemi ISC a Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | kontrolní součty stahovaných modelů | MIT nebo Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | klíče API ve svazku klíčů | MIT nebo Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, chyby, slovník Parakeet | MIT nebo Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | aktualizace aplikace z obrazu disku | MIT, s částmi pod licencemi BSD a zlib |

Licenční texty všeho, co aplikace obsahuje, jsou k ní přiloženy jako `Contents/Resources/Acknowledgements.txt`.

Modely nejsou součástí Lowspeaku; Lowspeak je stáhne, když o to požádáte:

| Model | Zdroj | Licence |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper od OpenAI ve formátu ggml) | MIT |
| Parakeet v2, Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference) – konverze modelu Parakeet TDT od NVIDIA do CoreML | CC-BY-4.0 |

## Licence

Lowspeak je licencován podle jedné z těchto licencí:

- Licence Apache, verze 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licence MIT ([LICENSE-MIT](../../../LICENSE-MIT))

dle vaší volby.

Pokud výslovně neuvedete jinak, každý příspěvek, který jste záměrně předložili k zahrnutí do Lowspeaku, jak je definováno v licenci Apache-2.0, bude licencován duálně jako výše, bez dalších podmínek.
