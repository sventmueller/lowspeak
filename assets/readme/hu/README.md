<p><img src="../../readme-icon.png" alt="A Lowspeak appikonja" width="168" height="168"></p>

# Lowspeak

**Diktálás Macre, amely oda gépel, ahol a kurzor áll: tartsa lenyomva az fn billentyűt, beszéljen, majd engedje fel.**

A Lowspeak a menüsorban él. Tartsa lenyomva az fn (Földgömb) billentyűt, és beszéljen; amikor felengedi, a szavai abban az appban jelennek meg, amelyben éppen gépel, mintha saját maga írta volna be őket. Online az OpenAI vagy a Groq segítségével ír át, vagy teljes egészében a saját Macén a Whisperrel vagy az NVIDIA Parakeettel, amely 40 másodpercnyi beszédet nagyjából a másodperc harmada alatt alakít szöveggé. Ha a hálózat vagy az online szolgáltatás nem működik, egy telepített helyi modell veszi át a feladatot, így egyetlen diktálás sem vész el. Tanítsa meg neki a használt neveket és kifejezéseket, ha szeretné, egy nyelvi modellel távolíttassa el a töltelékszavakat, és zárolja a felvételt a Shift billentyűvel, hogy hosszabb szövegeket kéz nélkül diktálhasson. Rustban íródott, valódi AppKit-vezérlőkre épül, 35 nyelven szól, és a hangját csak az Ön által választott szolgáltatónak küldi el.

[![A Lowspeak letöltése macOS-re](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[A Lowspeak.dmg letöltése](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: az Apple aláírta és hitelesítette, macOS 14 vagy újabb rendszerhez, Apple Silicon és Intel gépekre.

**Hibát talált?** [Nyisson egy issue-t](https://github.com/sventmueller/lowspeak/issues/new/choose), vagy válassza a Lowspeak menüjében a **Súgó › Probléma jelentése…** pontot: az űrlap az Ön verziójával kitöltve nyílik meg. Ha hasznosnak találja a Lowspeaket, egy [csillag a GitHubon](https://github.com/sventmueller/lowspeak) segít másoknak megtalálni.

[![Rust 2024-es kiadás](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 vagy újabb](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Univerzális: Apple Silicon és Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit az objc2-n keresztül](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp Metalon](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet a Neural Engine-en](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online az OpenAI-jal vagy a Groqkal](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 felületi nyelv](https://img.shields.io/badge/languages-35-4C9A2A)
![Hardened Runtime-mal aláírva](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licenc: MIT vagy Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licenc)

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
**Magyar** ·
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

## Tartalom

- [A funkciók áttekintése](#a-funkciók-áttekintése)
- [Rendszerkövetelmények](#rendszerkövetelmények)
- [Telepítés](#telepítés)
- [Első lépések](#első-lépések)
  - [Engedélyek](#engedélyek)
  - [Az átírás módjának kiválasztása](#az-átírás-módjának-kiválasztása)
  - [A menü](#a-menü)
  - [Gyors áttekintés](#gyors-áttekintés)
- [Diktálás](#diktálás)
  - [Tartsa lenyomva, és beszéljen](#tartsa-lenyomva-és-beszéljen)
  - [Zárolás Shifttel vagy dupla lenyomással](#zárolás-shifttel-vagy-dupla-lenyomással)
  - [Hová kerül a szöveg](#hová-kerül-a-szöveg)
  - [Legutóbbi diktálások](#legutóbbi-diktálások)
- [Átírás](#átírás)
  - [Melyik modell való Önnek](#melyik-modell-való-önnek)
  - [Online: OpenAI és Groq](#online-openai-és-groq)
  - [A saját Macén: Whisper és Parakeet](#a-saját-macén-whisper-és-parakeet)
  - [Ha az online szolgáltatás nem működik](#ha-az-online-szolgáltatás-nem-működik)
  - [Hogyan készül elő a felvétel](#hogyan-készül-elő-a-felvétel)
  - [Nyelvek modellenként](#nyelvek-modellenként)
  - [Intel-alapú Macek](#intel-alapú-macek)
- [Szókincs](#szókincs)
- [Utófeldolgozás](#utófeldolgozás)
- [Mikrofon](#mikrofon)
- [API-kulcsok](#api-kulcsok)
- [Értesítések](#értesítések)
- [Megjelenés és kisegítő lehetőségek](#megjelenés-és-kisegítő-lehetőségek)
- [Nyelvek](#nyelvek)
- [Billentyűk és mozdulatok](#billentyűk-és-mozdulatok)
- [Adatvédelem és biztonság](#adatvédelem-és-biztonság)
- [Sebesség](#sebesség)
- [Ismert korlátozások](#ismert-korlátozások)
- [Hibaelhárítás](#hibaelhárítás)
- [Fejlesztőknek](#fejlesztőknek)
  - [Harmadik féltől származó összetevők](#harmadik-féltől-származó-összetevők)
- [Licenc](#licenc)

## A funkciók áttekintése

- **Tartsa lenyomva az fn billentyűt, beszéljen, engedje fel.** A szöveg az előtérben lévő appba kerül, oda, ahol a kurzor áll: levélbe, csevegésbe, szerkesztőbe, terminálba vagy böngészőbe. Nincs ablak, amelyre át kellene váltani, és nincs gomb, amelyre kattintani kellene.
- **Hosszabb szövegekhez zárolja.** Nyomja meg a Shiftet, miközben az fn billentyűt lenyomva tartja, vagy nyomja meg kétszer az fn billentyűt, és a felvétel billentyű lenyomva tartása nélkül folytatódik, legfeljebb öt percig. Az fn vagy a Shift egyetlen lenyomása befejezi.
- **A feladathoz illő modell.** A Groq a sebességért, a Mac memóriájának igénybevétele nélkül, az OpenAI GPT Transcribe modellje a legjobb eredményért, a Whisper vagy az NVIDIA Parakeet a saját Macén az offline munkához és ahhoz, hogy a hangja az Öné maradjon. Egy kattintással válthat.
- **Egyetlen diktálás sem vész el.** Ha a hálózat vagy az online szolgáltatás nem működik, egy telepített helyi modell ugyanazt a felvételt írja át a Macen.
- **Az Ön szavai, helyesen leírva.** A nevekből és szakkifejezésekből álló szókincs az OpenAI-t, a Groqot, a Whispert és az utófeldolgozást is irányítja.
- **Tiszta szöveg, ha szeretné.** Az opcionális utófeldolgozás eltávolítja a töltelékszavakat, kijavítja a nyelvtant, és ha menet közben helyesbít, csak a helyesbítést tartja meg, vagy a szöveget annak az appnak a típusához is igazítja, amelybe diktál.
- **Beszéljen tovább.** Indítsa el a következő diktálást, miközben az előző átírása még tart; a szövegek abban a sorrendben érkeznek, ahogy elmondta őket.
- **A vágólap az Öné marad.** A Lowspeak a vágólapon keresztül illeszt be, és egy pillanattal később visszateszi, ami előtte rajta volt. Az utolsó tíz diktálás a menüben marad, hogy újra kimásolhassa.
- **Tősgyökeresen natív.** Valódi menüsorelem AppKit-menüvel, szabványos figyelmeztetésekkel és értesítésekkel. Nincs Electron, nincs ablak, nincs Dock-ikon.
- **Diszkrét.** Nincs fiók, nincs telemetria. A hang csak az Ön által választott szolgáltatóhoz jut el, helyi módban pedig sehová.
- **Az Ön nyelvén.** A diktálás több tucat nyelven működik, a menü, a párbeszédablakok és az értesítések pedig 35 nyelven érhetők el.

## Rendszerkövetelmények

- macOS 14 Sonoma vagy újabb. A Lowspeaket macOS 27 alatt fejlesztik és tesztelik.
- Apple Silicon vagy Intel processzoros Mac. A helyi átírás Apple Siliconra készült, ahol a Whisper a GPU-n, a Parakeet pedig a Neural Engine-en fut; Intel-alapú Maceken mindkettő a processzoron fut, jóval lassabban, lásd: [Intel-alapú Macek](#intel-alapú-macek).
- Körülbelül 10 MB az appnak, továbbá 75 MB és 3 GB között minden letöltött helyi modellnek.
- Online átíráshoz API-kulcs az [OpenAI](https://platform.openai.com/api-keys) vagy a [Groq](https://console.groq.com/keys) szolgáltatótól; mindkettő díjat számít fel azért, amit átír. A helyi átíráshoz nem kell fiók.

## Telepítés

**Lemezképből.** Töltse le a [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg) fájlt, nyissa meg, és húzza a Lowspeaket az Alkalmazások mappára. A lemezképet Developer ID-val írták alá, és az Apple hitelesítette, így figyelmeztetés nélkül megnyílik.

**Frissítések.** A Lowspeak nem frissíti magát. Az új verziók a [kiadások oldalán](https://github.com/sventmueller/lowspeak/releases) jelennek meg; a **Súgó › A Lowspeak a GitHubon** oda vezet. Az új verziót a régire telepítse; a beállítások, a kulcsok és a modellek megmaradnak.

**Forráskódból.** Az előfeltételeket lásd a [Fordítás és telepítés](../../../docs/development.md#build--deploy) szakaszban, majd:

```sh
make deploy
```

Ez lefordítja a Lowspeaket, lecseréli az Alkalmazások mappában lévő példányt, és elindítja.

A menü **Megnyitás bejelentkezéskor** pontja a Mackel együtt indítja a Lowspeaket, a macOS saját indítóelemein keresztül. Egyszerre csak egy példány fut; a második azonnal kilép.

## Első lépések

### Engedélyek

Az első indításkor a Lowspeak három engedélyt kér:

- **Kisegítő lehetőségek** (Rendszerbeállítások › Adatvédelem és biztonság › Kisegítő lehetőségek; macOS 27 alatt a lista neve Eszközvezérlés és adathozzáférés). A Lowspeaknek erre ahhoz van szüksége, hogy észlelje az fn billentyűt, és a ⌘V-vel beillesszen. A macOS felajánlja a lista megnyitását; kapcsolja be a Lowspeak kapcsolóját. A Lowspeak saját üzenetei a listát azon a néven említik, ahogy az Ön macOS-e mutatja.
- **Mikrofon.** Egyszer kéri; nélküle a Lowspeak semmit sem rögzít.
- **Értesítések.** A Lowspeak olyan problémák jelzésére használja őket, amelyeket különben nem venne észre, például néma mikrofon vagy sikertelen átírás.

### Az átírás módjának kiválasztása

Állítson be legalább egy módot; a menüben bármikor válthat.

- **Online:** válassza az **API-kulcs megadása…** pontot, és illesszen be egy kulcsot az OpenAI-tól (`sk-` kezdetű) vagy a Groqtól (`gsk_`). A Lowspeak a kulcsból ismeri fel a szolgáltatót. Ezután válasszon modellt az **OpenAI** vagy a **Groq** alatt.
- **A saját Macén:** válassza a **Modellek kezelése** › **Parakeet Ultra letöltése** pontot vagy egy Whisper-modellt, majd válassza ki a **Helyi** alatt.

A választásban a [Melyik modell való Önnek](#melyik-modell-való-önnek) szakasz segít. Ezután tartsa lenyomva az fn billentyűt, mondjon valamit, és engedje fel.

### A menü

Kattintson a menüsorban lévő gyűrűre:

| Elem | Mire szolgál |
|---|---|
| **Legutóbbi diktálások** | az utolsó tíz szöveg; egy kattintás újra a vágólapra másolja valamelyiket |
| **Helyi** | a Macen lévő modellek Whisper és Parakeet szerint csoportosítva; a le nem töltöttek szürkén jelennek meg |
| **OpenAI**, **Groq** | az online modellek; kulcs nélkül szürkék, és a menü az **API-kulcs megadása…** pontot kínálja |
| **Nyelv** | a diktálás nyelve vagy az **Automatikus felismerés**; a menü is ezt követi |
| **Utófeldolgozás** | **Ki**, **Tisztítás** vagy **Formázás**, lásd: [Utófeldolgozás](#utófeldolgozás) |
| **Szókincs…** | pontosan leírandó nevek és kifejezések; a menü mutatja, hányat adott meg |
| **Mikrofon** | **Rendszer alapértelmezése** vagy egy kiválasztott mikrofon, valamint a **Hangjelzés, ha kész** |
| **Modellek kezelése** | helyi modellek letöltése és törlése, folyamatjelzővel |
| **API-kulcs megadása…** / **API-kulcs módosítása…** | kulcsok hozzáadása, cseréje vagy eltávolítása |
| **Megnyitás bejelentkezéskor** | a Lowspeak indítása a Mackel együtt; a pipa jelzi, hogy be van kapcsolva |
| **Súgó** | **Gyors áttekintés**, **Probléma jelentése…**, **Funkció javaslása…**, **A Lowspeak a GitHubon** |
| **A Lowspeak névjegye** | verzió, szerzői jog, projektoldal és licenc |
| **Lowspeak bezárása** | ⌘Q, amíg a menü nyitva van |

A pipa a **Helyi**, az **OpenAI** és a **Groq** alatt a használt modellt jelöli. Az ikon mutatja, mit csinál éppen a Lowspeak:

| Ikon | Jelentés |
|---|---|
| Gyűrű ponttal | Kész |
| Kitöltött kör | Felvétel, amíg az fn le van nyomva |
| Lekerekített négyzet | Zárolt felvétel; az fn vagy a Shift következő lenyomása befejezi |
| Villogó gyűrű | Átírás |
| ↓ 42% | Modell letöltése |

A Lowspeaknek nincs beállítási ablaka: minden ebben a menüben van, és minden választását megjegyzi.

### Gyors áttekintés

A **Súgó › Gyors áttekintés** egy kis ablakban, az Ön nyelvén mutatja az fn-mozdulatokat, egy gombbal, amely megnyitja a billentyűzet-beállításokat. A **Probléma jelentése…** egy GitHub-űrlapot nyit meg, amelyben a Lowspeak és a macOS verziója már ki van töltve, a **Funkció javaslása…** egy beszélgetést az ötletekhez, **A Lowspeak a GitHubon** pedig a projektoldalt.

## Diktálás

### Tartsa lenyomva, és beszéljen

Tartsa lenyomva az fn billentyűt, beszéljen, engedje fel. A felvétel ikonja abban a pillanatban jelenik meg, amikor a mikrofon hangot ad; ekkor kezdjen beszélni, mert ami előtte elhangzik, az nem kerül a felvételre. Az fn felengedése befejezi a felvételt, a szöveg pedig egy pillanattal később megérkezik.

Az fn rövid megérintése nem csinál semmit, így az fn a többi funkcióját is megtartja. Az üresen maradó vagy 0,3 másodpercnél rövidebb felvételt a Lowspeak elveti.

### Zárolás Shifttel vagy dupla lenyomással

Hosszabb szövegekhez zárolja a felvételt, hogy billentyű lenyomva tartása nélkül fusson. Háromféleképpen teheti meg:

- **Shift az fn lenyomva tartása közben.** Kezdje a szokásos módon az fn billentyűvel, és a felvétel közben nyomja meg a Shiftet: a felvétel zárolódik, és felengedheti az fn billentyűt. Semmi sem vész el abból, amit addig mondott, a felvétel egyszerűen folytatódik.
- **Shift, majd fn.** Tartsa lenyomva a Shiftet, és nyomja meg az fn billentyűt: a felvétel zárolva indul.
- **Kétszer fn.** Érintse meg röviden az fn billentyűt (0,3 másodpercnél rövidebben), és 0,4 másodpercen belül nyomja meg újra. A felvétel már az első lenyomástól kezdve fut.

Az ikon lekerekített négyzetté változik. A leállításhoz nyomja meg egyszer az **fn** vagy a **Shift** billentyűt; a szöveg a szokásos módon megérkezik. Ha a 🌐 billentyű az Emojik és szimbólumok megnyitására van beállítva, a Shifttel állítsa le: az fn rövid lenyomása megnyitná a nézegetőt.

A felvétel lenyomott fn mellett két perc, zárolva öt perc után magától leáll, majd a szokásos módon átíródik.

### Hová kerül a szöveg

A Lowspeak abba az appba illeszti be a szöveget, amely a beszéd befejezésekor az előtérben volt, a kurzor helyére, a vágólapon és a ⌘V-n keresztül. Előtte elmenti mindazt, ami a vágólapon volt, és fél másodperccel azután teszi vissza, hogy az app kiolvasta a szöveget, vagy tíz másodperc múlva, ha az app ezt sosem teszi meg, hacsak egy másik app közben meg nem változtatta a vágólapot. A szöveg ideiglenesként van megjelölve, így a vágólap-előzményeket kezelő appok, amelyek tiszteletben tartják ezt a jelölést, nem rögzítik.

A következő diktálást akkor is elindíthatja, ha az előző átírása még tart. A szövegek abban a sorrendben kerülnek beillesztésre, ahogy elmondta őket, és soha nem felvétel közben vagy a Shift lenyomva tartása alatt, így a beillesztés nem kerülhet a következő mondata közepébe.

A Kisegítő lehetőségek engedélye nélkül, vagy ha a szimulált ⌘V nem küldhető el, a szöveg a vágólapon marad, és egy értesítés jelzi ezt; illessze be saját maga a ⌘V-vel.

### Legutóbbi diktálások

A menü **Legutóbbi diktálások** pontja az utolsó tíz szöveget sorolja fel, a legújabbal kezdve, mindegyiket egy sorra rövidítve; ha fölé viszi a mutatót, a teljes szöveg látszik. Egy kattintás a vágólapra másolja, arra az esetre, ha egy szöveg rossz helyre került, vagy kétszer van rá szüksége. A lista csak a memóriában él, és a Lowspeak bezárása után üres.

## Átírás

### Melyik modell való Önnek

| Amit szeretne | Ezt válassza | Miért |
|---|---|---|
| **Sebesség a Mac igénybevétele nélkül** | **Groq › Whisper Large V3 Turbo** | a leggyorsabb online lehetőség; semmit sem kell letölteni, a Mac memóriája és tárhelye érintetlen marad |
| **A legjobb eredmény** | **OpenAI › GPT Transcribe** (alapértelmezett) | a Lowspeak által kínált modellek közül a legpontosabb; kulcsszavakként kapja meg a szókincset |
| **Offline és diszkréten** | **Helyi › Parakeet Ultra** európai nyelvekhez, **Helyi › Whisper Large v3 Turbo** minden más nyelvhez | hálózat nélkül működik, és a hangja sosem hagyja el a Macet; a Parakeet a leggyorsabb mind közül |
| **Fiók és költség nélkül** | bármelyik **Helyi** modell | a Macen fut; semmit sem számláznak |
| **Megbízhatóság és sebesség egyszerre** | egy online modell, mellé telepítve a Parakeet Ultra vagy egy Whisper-modell | a helyi modell beugrik, ha az online szolgáltatás nem működik |
| **Intel-alapú Mac** | **Groq** vagy **OpenAI** | a helyi modellek ott a processzoron futnak, lásd: [Intel-alapú Macek](#intel-alapú-macek) |

A helyi modellek a letöltési méretüknek megfelelő helyet foglalnak a lemezen, és amíg ki vannak választva, nagyjából ugyanannyi memóriát is; a Lowspeak betöltve tartja a modellt, hogy a diktálás késedelem nélkül induljon. 8 GB memóriájú Macen a nagy Whisper-modellek helyett inkább a Parakeet Ultrát vagy egy online modellt válassza.

### Online: OpenAI és Groq

| Modell | Szolgáltató | Megjegyzés |
|---|---|---|
| **GPT Transcribe** (alapértelmezett) | OpenAI | a legpontosabb; kulcsszavakként kapja meg a szókincset |
| **Whisper** | OpenAI | az eredeti Whisper-modell |
| **Whisper Large V3** | Groq | nagyon gyors |
| **Whisper Large V3 Turbo** | Groq | a leggyorsabb online |

A felvétel feltöltés előtt AAC formátumba tömörül, beszédmásodpercenként körülbelül 6 KB-ra, így a diktálás lassú kapcsolaton is működik. Amíg az fn billentyűt lenyomva tartja, a Lowspeak már megnyitja a kapcsolatot a szolgáltatóval, így a feltöltés abban a pillanatban elindul, amikor felengedi.

### A saját Macén: Whisper és Parakeet

| Modell | Méret | Ezen fut | Nyelvek |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 európai nyelv |
| **Parakeet v2** | ~600 MB | Neural Engine | angol |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | kb. 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | kb. 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | kb. 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | kb. 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | kb. 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | kb. 99 |

A **Modellek kezelése** tölti le és törli őket; letöltés közben a menüsor mutatja a folyamatot, a Lowspeak bezárása pedig megszakítja. Egy modell legelső használata felkészíti a modellt az Ön Macére: a Parakeet Ultránál ez körülbelül 10 másodperc, a Whispernél legfeljebb 30 másodperc; ezután a modell a másodperc töredéke alatt betöltődik. Ha a kiválasztott modellt törli, a Lowspeak egy másik telepítettre vált.

A **Parakeet** a leggyorsabb választás: 40 másodpercnyi beszédet nagyjából 0,3 másodperc alatt ír át. A Lowspeak ugyanazokkal a modellekkel és dekódolással futtatja, mint a [FluidAudio](https://github.com/FluidInference/FluidAudio), és a FluidAudio mappájában tárolja őket (`~/Library/Application Support/FluidAudio/Models`), így a FluidAudióra épülő appok közösen használják őket. A 15 másodpercnél hosszabb felvételeket egymást átfedő ablakokban írja át, és ezeket úgy illeszti össze, hogy az illesztéseknél egyetlen szó se vesszen el. A **Whisper** több nyelvet ért, és figyelembe veszi a szókincset; a nagy modellek a legpontosabbak, a kicsik angolon kívül jóval gyengébbek. A Whisper-modellek helye: `~/Library/Application Support/Lowspeak/Models`.

### Ha az online szolgáltatás nem működik

Ha egy online átírás nem sikerül, mert nincs hálózat, a szolgáltatónál hiba van, vagy a kulcs hibás, egy telepített helyi modell írja át ugyanazt a felvételt: a **Helyi** alatt kiválasztott, ennek hiányában a Parakeet Ultra, majd a Whisper Large v3 Turbo, a Large v3 és a kisebb Whisper-modellek, végül a Parakeet v2. Erről egyszer értesítés tájékoztatja; újra csak akkor érkezik, ha közben egy online diktálás sikerült.

Ha van helyi modell tartaléknak, a Lowspeak 20 másodpercet ad az online kérésnek; ha nincs, háromszor próbálkozik, alkalmanként legfeljebb egy percig. Online módban a Lowspeak indításkor egyszer előkészíti a tartalékként szolgáló Parakeet-modellt, így az első tartalékátírásnak nem kell rá várnia.

### Hogyan készül elő a felvétel

A Lowspeak a mikrofon saját mintavételi frekvenciáján rögzít, és a leállítás után alakítja át a modellek által várt 16 kHz-re. Az átírás előtt eltávolítja a 80 Hz alatti mély dübörgést, levágja a csendet az elejéről és a végéről, és kiegyenlíti a hangerőt, így a halkan és a hangosan beszélők egyformán szólnak. A beszéd nélküli felvételt egyáltalán nem küldi el. Helyi módban a Parakeet először az érintetlen felvételt kapja meg, az előkészítettet pedig csak akkor, ha abból nem lesz szöveg.

### Nyelvek modellenként

A **Nyelv** alatt kiválasztott nyelvet a Lowspeak átadja az online modelleknek és a Whispernek. **Automatikus felismerés** esetén a modell maga állapítja meg a nyelvet.

| Modell | Diktálható nyelvek |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, a saját Macén) | kb. 99, köztük a menü mind a 35 nyelve |
| Parakeet Ultra | angol, bolgár, cseh, dán, észt, finn, francia, görög, holland, horvát, lengyel, lett, litván, magyar, máltai, német, olasz, orosz, portugál, román, spanyol, svéd, szlovák, szlovén, ukrán |
| Parakeet v2 | angol |

A Parakeet maga ismeri fel a nyelvet, és figyelmen kívül hagyja a menüben választottat. A **kínai (hagyományos)** arra utasítja a GPT Transcribe-ot, hogy hagyományos írásjegyekkel írjon; a Whisper a kínait bármelyik írásmóddal, a portugált pedig a változattól függetlenül írja.

### Intel-alapú Macek

A Lowspeak univerzális app, és macOS 14 alatt Intel-alapú Maceken is fut. Minden online funkció ugyanúgy működik. A helyi modellek viszont nem találják ott azt a hardvert, amelyre készültek: a Whisper a GPU helyett a processzoron fut, a Parakeet pedig a Neural Engine helyett szintén a processzoron. Mindkettő működik, de sokszor tovább tart, mint Apple Siliconon, a nagy Whisper-modellek pedig diktáláshoz túlságosan is lassúak. Intel-alapú Macen online írjon át, és [tartaléknak](#ha-az-online-szolgáltatás-nem-működik) tartsa meg a Whisper Smallt vagy a Parakeet Ultrát.

## Szókincs

A **Szókincs…** azokat a neveket és kifejezéseket fogadja, amelyeket a Lowspeaknek pontosan kell leírnia, vesszővel elválasztva: termékneveket, kollégák nevét, szakkifejezéseket. Legfeljebb 100 kifejezés, egyenként legfeljebb 60 karakter; a `<` és a `>` jelet eltávolítja.

- A **GPT Transcribe** kulcsszavakként kapja meg őket.
- A **Whisper** (OpenAI, Groq és a saját Macén) a felvétel előtti útmutatásként olvassa őket, legfeljebb 500 karakterig.
- Az **utófeldolgozás** utasítást kap, hogy pontosan úgy írja őket, ahogy meg vannak adva.
- A **Parakeet** nem használja őket.

## Utófeldolgozás

Az átírás után egy nyelvi modell átdolgozhatja a szöveget. Az **Utófeldolgozás** alatt választhat:

- **Ki** (alapértelmezett) — a szöveg úgy, ahogy átíródott.
- **Tisztítás** — eltávolítja az olyan töltelékszavakat, mint az „öö”, az „ugye” vagy az „alapvetően”, javítja a nyelvtant és az írásjeleket, és ha menet közben helyesbít, csak a helyesbítést tartja meg. Semmit sem tesz hozzá, és nem változtat a jelentésen.
- **Formázás** — mint a Tisztítás, és ezen felül ahhoz az apphoz igazítja a szöveget, amelybe diktál: kódszerű kifejezések kódszerkesztőkben, parancssorok terminálokban, laza hangnem csevegőappokban, hivatalos az e-mailekben.

Az utófeldolgozás a Groqot (`openai/gpt-oss-120b`) használja, ha van Groq-kulcsa, egyébként az OpenAI-t (`gpt-4o-mini`), akkor is, ha helyben ír át. Ha a **Nyelv** alatt kiválasztott egy nyelvet, megtartja azt a nyelvet. Ha nyolc másodpercnél tovább tart, nem sikerül, vagy olyan választ ad, amely csonka vagy jóval hosszabb annál, amit mondott, a Lowspeak a feldolgozatlan szöveget illeszti be. Ha a szolgáltató elutasítja a kérést, vagy nincs megadva kulcs, erről egyszer értesítés szól.

## Mikrofon

A **Mikrofon** alatt kiválaszthat egy adott mikrofont vagy a **Rendszer alapértelmezése** lehetőséget. A Lowspeak megjegyzi az eszközt; amíg nincs csatlakoztatva, a menü **(nincs csatlakoztatva)** jelzéssel mutatja, és a Lowspeak a rendszer alapértelmezett mikrofonjával rögzít.

A mikrofon csak diktálás közben van bekapcsolva. Az előkészítése a beépített mikrofonnal körülbelül 0,07 másodperc, Studio Display esetén nagyjából fél másodperc, AirPods esetén pedig egy-két másodperc, mivel az AirPods előbb headset módba vált. A felvétel ikonja akkor jelenik meg, amikor megérkezik a hang; a **Hangjelzés, ha kész** ebben a pillanatban egy rövid hangot is lejátszik. Alapértelmezés szerint ki van kapcsolva, mert egyes hangszórókon maga a hang fél másodperccel később szólal meg.

Ha egy mikrofon öt másodpercig csak csendet ad, ahogy az AirPods teszi, amikor egy másik eszköz foglalja a mikrofonját, a Lowspeak szól.

## API-kulcsok

Az **API-kulcs megadása…** bekér egy kulcsot, és az eleje alapján felismeri a szolgáltatót: `sk-` az OpenAI, `gsk_` a Groq esetén. Más formátumú kulcsot magyarázattal elutasít. Szolgáltatónként egy kulcsot adjon meg; ha mindkettőhöz van kulcsa, minden online modell elérhető. Az **API-kulcs módosítása…** lecserél egy kulcsot, vagy az **Összes kulcs eltávolítása** paranccsal törli őket.

A kulcsokat a Lowspeak az Ön bejelentkezési kulcskarikáján tárolja, `com.lowspeak.app` néven. Egy korábbi verzió kulcsfájlját (`~/Library/Application Support/Lowspeak/.api-key`) a következő indításkor oda helyezi át, és törli, amint minden kulcs visszaolvasható a kulcskarikáról.

## Értesítések

A Lowspeak a háttérben dolgozik, ezért a problémákat értesítésekben jelzi:

| Értesítés | Mi történt |
|---|---|
| **A felvétel nem sikerült** | nincs mikrofon-hozzáférés, nincs csatlakoztatott mikrofon, vagy a mikrofont nem sikerült elindítani |
| **A mikrofon néma** | öt másodperc csend, lásd: [Mikrofon](#mikrofon) |
| **Engedély szükséges a kisegítő lehetőségekhez** | a szöveg a vágólapon van; engedélyezze a Kisegítő lehetőségeket az automatikus beillesztéshez |
| **A beillesztés nem sikerült** | a szöveg a vágólapon van; illessze be a ⌘V-vel |
| **Az átírás nem sikerült** | az okkal együtt: nincs kulcs, elutasított kulcs, túl sok kérés, szerver- vagy hálózati hiba, vagy nincs helyi modell |
| **Átírva ezen a Macen** | az online szolgáltatás nem működött, és egy helyi modell vette át a feladatot |
| **Az utófeldolgozás nem sikerült** | a szolgáltató elutasítja az utófeldolgozást, vagy nincs megadva kulcs; a szöveg nélküle lett beillesztve |
| **Nincs API-kulcs** | olyan online modellt választott, amelynek szolgáltatójához nincs kulcs |
| **A letöltés nem sikerült**, **A törlés nem sikerült**, **Nem törölhető** | egy helyi modell kezelése nem sikerült, vagy még egy másik művelet futott |
| **Indítóelem-hiba**, **API-kulcshiba** | a macOS vagy a kulcskarika elutasította a módosítást |

Az értesítések soha nem tartalmazzák a diktált szöveget. A nem működő online szolgáltatást és az utófeldolgozást elutasító szolgáltatót a Lowspeak egyszer jelzi, nem minden diktálás után, egészen addig, amíg újra nem működnek.

## Megjelenés és kisegítő lehetőségek

- **A menüsor ikonja** sablonkép, amely alkalmazkodik a világos, a sötét vagy a színezett menüsorhoz.
- A **VoiceOver** „Lowspeak”-ként olvassa fel az ikont, a menü, a figyelmeztetések és az értesítések pedig szabványos macOS-vezérlők.
- **Egyetlen billentyűt sem kell lenyomva tartani.** Zárolja a felvételt a Shifttel vagy az fn dupla lenyomásával, és a következő lenyomás befejezi.
- **A Gyors áttekintés** egyszerű ablak feliratokkal és egy gombbal, billentyűzetről is elérhető; az Esc bezárja.

Az ismert hiányosságok az [ACCESSIBILITY.md](../../../ACCESSIBILITY.md) fájlban vannak felsorolva: a VoiceOver nem jelzi, amikor egy felvétel elindul, és a Lowspeaket még nem ellenőrizték a VoiceOverrel és a Kapcsolóvezérléssel.

## Nyelvek

A menü, a párbeszédablakok és az értesítések 35 nyelvre vannak lefordítva:

angol, arab, cseh, dán, finn, francia, görög, héber, hindi, holland, horvát, indonéz, japán, katalán, kínai (egyszerűsített), kínai (hagyományos), koreai, lengyel, magyar, maláj, német, norvég, olasz, orosz, portugál (Brazília), portugál (Portugália), román, spanyol, svéd, szlovák, szlovén, thai, török, ukrán és vietnami.

A **Nyelv** alatt választott nyelv egyben a menü nyelve is. **Automatikus felismerés** esetén a menü a **Rendszerbeállítások › Általános › Nyelv és régió** nyelvi sorrendjét követi, és angolt használ, ahol nincs fordítás. A menük kifejezései az Apple saját fordításait követik, így a Lowspeak úgy szól, mint a Mac többi része. Az angol kivételével a fordítások gépi fordítások, és anyanyelvi beszélők még nem nézték át őket; a javításokat szívesen fogadjuk [issue-ként](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Billentyűk és mozdulatok

| Művelet | Billentyűk |
|---|---|
| Diktálás, amíg lenyomva tartja | az fn lenyomva tartása |
| Futó felvétel zárolása | Shift az fn lenyomva tartása közben |
| Zárolt felvétel indítása | Shift+fn vagy kétszer gyorsan fn |
| Zárolt felvétel befejezése | fn vagy Shift |
| Kilépés a Lowspeakből (nyitott menünél) | ⌘Q |
| A Gyors áttekintés bezárása | Esc |

A Lowspeak nem engedi tovább más appokhoz az általa használt fn-lenyomásokat; a Shift a szokásos módon továbbjut.

## Adatvédelem és biztonság

A Lowspeak hallja, amit mond, amíg az fn le van nyomva, bármelyik appba tud gépelni, és őrzi az API-kulcsait, ezért a lehető legkevesebbet teszi ezekkel.

- **A hangja egyetlen helyre jut.** Online módban a tömörített felvétel a választott szolgáltatóhoz, az OpenAI-hoz vagy a Groqhoz kerül, a szókincsével együtt. Helyi módban semmi sem hagyja el a Macet. Ha egy online kérés nem sikerül, egy helyi modell írja át ugyanazt a felvételt a Macen.
- Az **utófeldolgozás**, ha bekapcsolja, az átírt szöveget és a szókincsét elküldi a Groqnak vagy az OpenAI-nak. Formázás módban hozzáadja annak az appnak a típusát, amelybe diktál (kódszerkesztő, terminál, csevegés vagy e-mail); az app nevét nem küldi el.
- **Nincs telemetria, nincs fiók.** Az átíráson és az utófeldolgozáson kívül a Lowspeak csak azért kapcsolódik a hálózathoz, hogy online módban vagy bekapcsolt utófeldolgozásnál az fn lenyomva tartása alatt megnyissa a kapcsolatot a szolgáltatóval, és hogy kérésre letöltsön egy modellt a Hugging Face-ről, HTTPS-en keresztül, rögzített verzióban, minden fájlt az ellenőrzőösszegével összevetve. A **Súgó › Probléma jelentése…** egy GitHub-űrlapot nyit meg a böngészőben; a címe a Lowspeak és a macOS verzióját tartalmazza, semmit abból, amit diktált.
- Az **API-kulcsok** az Ön bejelentkezési kulcskarikáján maradnak. Csak a saját szolgáltatójukhoz jutnak el, és a Lowspeak eltávolítja őket a hibaüzenetekből, mielőtt bármi a naplóba kerülne.
- **A diktált szöveg nem tárolódik.** Az utolsó tíz diktálás a **Legutóbbi diktálások** számára a memóriában marad, és a Lowspeak bezárásakor eltűnik. A naplófájlok hosszakat és időzítéseket rögzítenek, a szöveget soha.
- **A vágólap** tartalmát a Lowspeak beillesztés előtt elmenti, és visszaállítja, amint az app kiolvasta a szöveget, hacsak egy másik app közben meg nem változtatta. A diktált szöveg ideiglenesként van megjelölve, így a vágólap-előzményeket kezelő appok, amelyek tiszteletben tartják ezt a jelölést, kihagyják.
- **Kódaláírás** Hardened Runtime-mal; az app csak a mikrofonhoz, a Kisegítő lehetőségekhez, az értesítésekhez és a hálózathoz kér engedélyt.

A sebezhetőségek bejelentéséhez kövesse a [biztonsági szabályzatot](../../../SECURITY.md).

## Sebesség

Apple Siliconon mérve:

- A **Parakeet Ultra** 40 másodpercnyi beszédet körülbelül 0,3 másodperc alatt ír át.
- **Online:** szünet után az első kérés a Groqnál körülbelül 0,4 másodpercig tart, mert a Lowspeak már beszéd közben megnyitja a kapcsolatot; a későbbi kérések újra felhasználják, HTTP/3-on keresztül, ahol a szolgáltató kínálja.
- **Feltöltési méret:** beszédmásodpercenként körülbelül 6 KB, 48 kbit/s-os AAC-ként.
- **A mikrofon előkészítése:** a beépítettnél körülbelül 0,07 másodperc, Studio Display esetén fél másodperc, AirPods esetén egy-két másodperc.

## Ismert korlátozások

- **Intel-alapú Maceken a helyi modellek** a processzoron futnak, és jóval lassabbak; ott az online átírás a jobb választás.
- **Ami a mikrofon elkészülte előtt hangzik el, elvész.** Várja meg a felvétel ikonját, különösen AirPods esetén.
- **Az fn billentyűt mások is lefoglalhatják.** Ha a macOS 🌐 billentyűre vonatkozó beállítása, a Warp vagy a macOS Diktálás funkciója előbb lefoglalja az fn billentyűt, a Lowspeak nem észleli; lásd: [Hibaelhárítás](#hibaelhárítás).
- A **Parakeet** figyelmen kívül hagyja a szókincset és a nyelvválasztást, és csak európai nyelveket ismer.
- **Írásmód és változat:** csak a GPT Transcribe utasítható hagyományos kínai írásra; egyetlen modell sem utasítható arra, hogy brazíliai helyett európai portugál nyelven írjon.
- **Időkorlátok:** két perc, amíg az fn le van nyomva, öt perc zárolva.
- **Mikrofonmód:** a Vezérlőközpont felkínálja a Lowspeaknek a Beszédhang izolálása és a Széles spektrum módot, és aktívként jelzi őket, a Lowspeak azonban hangfeldolgozás nélkül rögzít, így a választásnak nincs hatása.
- A **Legutóbbi diktálások** csak a memóriában élnek, és kilépés után eltűnnek.
- **Szándékosan:** nincs beállítási ablak, nincs Dock-ikon, nincsenek automatikus frissítések, és nincs az App Store-ban.

## Hibaelhárítás

**Semmi sem történik, amikor lenyomva tartom az fn billentyűt.** Ellenőrizzen három dolgot: engedélyezve van-e a Lowspeak a **Rendszerbeállítások › Adatvédelem és biztonság › Kisegítő lehetőségek** alatt (macOS 27 alatt: **Eszközvezérlés és adathozzáférés**); a **Rendszerbeállítások › Billentyűzet › Nyomja meg a 🌐 billentyűt a következőhöz:** beállítás értéke **Nincs művelet**-e; és nem foglalja-e le egy másik app az fn billentyűt. A Warp alapértelmezés szerint lefoglalja; kapcsolja ki a `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` paranccsal, és indítsa újra a Warpot. A macOS Diktálás funkciója is beállítható úgy, hogy az fn billentyűvel induljon.

**Frissítés után a Lowspeak már nem reagál az fn billentyűre.** A macOS a Kisegítő lehetőségek engedélyét az app aláírásához köti. Ha az aláírás megváltozott, például egy saját fordításról a letöltött appra váltott, távolítsa el a Lowspeaket a Kisegítő lehetőségek listájából (macOS 27 alatt: Eszközvezérlés és adathozzáférés), majd adja hozzá újra.

**Hiányoznak az első szavak.** Akkor kezdjen beszélni, amikor megjelenik a felvétel ikonja. AirPods esetén a mikrofonnak egy-két másodperc kell; a **Mikrofon** alatt kiválasztott beépített vagy USB-mikrofon szinte azonnal kész.

**„A mikrofon néma.”** A mikrofon öt másodpercig nem adott hangot. AirPods esetén lehet, hogy egy másik eszköz használja a mikrofonját; váltson a beépített mikrofonra, vagy csatlakoztassa újra az AirPodsot. A lecsukott MacBook-fedél is elnémítja a beépített mikrofont.

**A diktálások üresek.** Lehet, hogy túl alacsony a bemeneti hangerő: az olyan appok, mint a Teams, a Zoom vagy a Meet, hívás közben módosítják, és a macOS eszközönként tárolja. Növelje a **Rendszerbeállítások › Hang › Bemenet** alatt.

**A szöveg rossz helyre került.** A menü **Legutóbbi diktálások** pontja újra a vágólapra másolja.

**„Nincs API-kulcs.”** Válassza a menüben az **API-kulcs megadása…** pontot, vagy váltson helyi modellre.

**Az első helyi átírás sokáig tart.** Minden modell az első használatakor készül fel az Ön Macére; onnantól a másodperc töredéke alatt betöltődik.

**A Lowspeak közvetlenül az indítás után kilép.** Már fut a Lowspeak egy másik példánya; előbb abból lépjen ki.

**Nem jelennek meg értesítések.** Engedélyezze őket a **Rendszerbeállítások › Értesítések › Lowspeak** alatt.

**Valami más.** A `~/Library/Application Support/Lowspeak/` mappában lévő naplók (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) leírják, mi történt, a diktált szöveg nélkül. Csatolja a vonatkozó sorokat egy [hibajelentéshez](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Fejlesztőknek

A fordítás, a tesztelés, a kód felépítése és az általa követett szabályok leírása a [docs/development.md](../../../docs/development.md) fájlban, a közreműködésé a [CONTRIBUTING.md](../../../CONTRIBUTING.md) fájlban található (angolul).

### Harmadik féltől származó összetevők

| Összetevő | Verzió | Mire szolgál | Licenc |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) a [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) segítségével | 0.16 | helyi Whisper a GPU-n (Metal) | MIT; whisper-rs: Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet-dekódolás és hosszú felvételek összefűzése, Rustra átültetve a `src/transcription/parakeet` mappában | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) és a keretrendszer-crate-jei | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation és más rendszer-keretrendszerek | MIT; Zlib, Apache-2.0 vagy MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | mikrofonfelvétel a CoreAudión keresztül | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | újramintavételezés 16 kHz-re | MIT |
| [ureq](https://github.com/algesten/ureq) rustls-szel | 3.5 | modellek letöltése | MIT vagy Apache-2.0; a rustls és a ring ISC és Apache-2.0 licenc alatt |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | a letöltött modellek ellenőrzőösszegei | MIT vagy Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | API-kulcsok a kulcskarikán | MIT vagy Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, hibák, a Parakeet szókészlete | MIT vagy Apache-2.0; Apache-2.0 |

Az appban található összes összetevő licencszövege `Contents/Resources/Acknowledgements.txt` néven az appal együtt érkezik.

A modellek nem részei a Lowspeaknek; kérésre tölti le őket:

| Modell | Forrás | Licenc |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (az OpenAI Whisper modellje ggml formátumban) | MIT |
| Parakeet v2, Parakeet Ultra | az NVIDIA Parakeet TDT modelljének CoreML-átalakításai a [FluidInference](https://huggingface.co/FluidInference) oldaláról | CC-BY-4.0 |

## Licenc

A Lowspeak licence az alábbiak bármelyike

- Apache-licenc, 2.0-s verzió ([LICENSE-APACHE](../../../LICENSE-APACHE))
- MIT-licenc ([LICENSE-MIT](../../../LICENSE-MIT))

az Ön választása szerint.

Hacsak Ön kifejezetten másként nem nyilatkozik, a Lowspeakbe való felvételre szándékosan benyújtott minden hozzájárulás, az Apache-2.0 licenc meghatározása szerint, a fentiek szerint kettős licenc alatt áll, további feltételek nélkül.
