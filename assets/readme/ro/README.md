<p><img src="../../readme-icon.png" alt="Pictograma aplicației Lowspeak" width="168" height="168"></p>

# Lowspeak

**Dictare pentru Mac care scrie acolo unde se află cursorul: țineți apăsat fn, vorbiți, eliberați.**

Lowspeak se află în bara de meniu. Țineți apăsată tasta fn (Glob) și vorbiți; când o eliberați, cuvintele dumneavoastră apar în aplicația în care scrieți, ca și cum le-ați fi tastat chiar dumneavoastră. Transcrie online cu OpenAI sau Groq ori complet pe Mac-ul dumneavoastră cu Whisper sau NVIDIA Parakeet, care transformă 40 de secunde de vorbire în text în aproximativ o treime de secundă. Când rețeaua sau serviciul online nu funcționează, preia un model local instalat, astfel că dictarea nu se pierde. Învățați-l numele și termenii pe care îi folosiți, lăsați, dacă doriți, un model lingvistic să elimine cuvintele de umplutură și blocați o înregistrare cu Shift pentru a dicta texte mai lungi fără mâini. Este scris în Rust, pe controale AppKit autentice, vorbește 35 de limbi și vă trimite vocea doar furnizorului pe care l-ați ales.

[![Descărcați Lowspeak pentru macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Descărcați Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: semnat și notarizat de Apple, pentru macOS 14 sau mai nou, pe Apple Silicon și Intel.

**Ați găsit o eroare?** [Deschideți un issue](https://github.com/sventmueller/lowspeak/issues/new/choose) sau alegeți **Ajutor › Raportați o problemă…** din meniul Lowspeak: formularul se deschide cu versiunea dumneavoastră deja completată. Dacă Lowspeak vă este util, o [stea pe GitHub](https://github.com/sventmueller/lowspeak) îi ajută pe alții să îl găsească.

[![Rust, ediția 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 sau mai nou](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon și Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit prin objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp pe Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet pe Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online cu OpenAI sau Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 de limbi de interfață](https://img.shields.io/badge/languages-35-4C9A2A)
![Semnat cu Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licență: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licență)

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
**Română** ·
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

## Cuprins

- [Pe scurt](#pe-scurt)
- [Cerințe](#cerințe)
- [Instalare](#instalare)
- [Primii pași](#primii-pași)
  - [Permisiuni](#permisiuni)
  - [Alegeți modul de transcriere](#alegeți-modul-de-transcriere)
  - [Meniul](#meniul)
  - [Referință rapidă](#referință-rapidă)
- [Dictarea](#dictarea)
  - [Țineți apăsat și vorbiți](#țineți-apăsat-și-vorbiți)
  - [Blocarea cu Shift sau cu o apăsare dublă](#blocarea-cu-shift-sau-cu-o-apăsare-dublă)
  - [Unde ajunge textul](#unde-ajunge-textul)
  - [Dictări recente](#dictări-recente)
- [Transcrierea](#transcrierea)
  - [Ce model vi se potrivește](#ce-model-vi-se-potrivește)
  - [Online: OpenAI și Groq](#online-openai-și-groq)
  - [Pe Mac-ul dumneavoastră: Whisper și Parakeet](#pe-mac-ul-dumneavoastră-whisper-și-parakeet)
  - [Când serviciul online nu funcționează](#când-serviciul-online-nu-funcționează)
  - [Cum este pregătită înregistrarea](#cum-este-pregătită-înregistrarea)
  - [Limbi pentru fiecare model](#limbi-pentru-fiecare-model)
  - [Mac-uri cu Intel](#mac-uri-cu-intel)
- [Vocabular](#vocabular)
- [Postprocesare](#postprocesare)
- [Microfon](#microfon)
- [Chei API](#chei-api)
- [Notificări](#notificări)
- [Aspect și accesibilitate](#aspect-și-accesibilitate)
- [Limbi](#limbi)
- [Taste și gesturi](#taste-și-gesturi)
- [Confidențialitate și securitate](#confidențialitate-și-securitate)
- [Viteză](#viteză)
- [Limitări cunoscute](#limitări-cunoscute)
- [Depanare](#depanare)
- [Pentru dezvoltatori](#pentru-dezvoltatori)
  - [Componente terțe](#componente-terțe)
- [Licență](#licență)

## Pe scurt

- **Țineți apăsat fn, vorbiți, eliberați.** Textul este scris în aplicația din prim-plan, oriunde se află cursorul: e-mail, chat, editor, terminal sau browser. Nicio fereastră la care să comutați, niciun buton pe care să faceți clic.
- **Blocați înregistrarea pentru texte mai lungi.** Apăsați Shift în timp ce țineți fn sau apăsați fn de două ori, iar înregistrarea continuă fără să țineți vreo tastă, până la cinci minute. O apăsare pe fn sau Shift o încheie.
- **Modelul potrivit pentru fiecare sarcină.** Groq pentru viteză fără a folosi memoria Mac-ului, GPT Transcribe de la OpenAI pentru cel mai bun rezultat, Whisper sau NVIDIA Parakeet pe Mac-ul dumneavoastră pentru lucru offline și o voce care rămâne privată. Comutați cu un singur clic.
- **Nicio dictare pierdută.** Când rețeaua sau serviciul online nu funcționează, un model local instalat transcrie aceeași înregistrare pe Mac-ul dumneavoastră.
- **Cuvintele dumneavoastră, scrise corect.** Un vocabular de nume și termeni tehnici ghidează OpenAI, Groq, Whisper și postprocesarea.
- **Text curat, dacă doriți.** Postprocesarea opțională elimină cuvintele de umplutură, corectează gramatica și, când vă corectați, păstrează doar corectura sau formatează în plus textul pentru tipul de aplicație în care dictați.
- **Continuați să vorbiți.** Începeți următoarea dictare în timp ce precedenta este încă transcrisă; textele sosesc în ordinea în care le-ați rostit.
- **Clipboardul rămâne al dumneavoastră.** Lowspeak lipește prin clipboard și pune la loc ce era în el o clipă mai târziu. Ultimele zece dictări rămân în meniu, pentru a fi copiate din nou.
- **Nativ până în măduva oaselor.** Un element real în bara de meniu, cu un meniu AppKit, alerte și notificări standard. Fără Electron, fără fereastră, fără pictogramă în Dock.
- **Privat.** Fără cont, fără telemetrie. Sunetul ajunge doar la furnizorul ales de dumneavoastră sau, în modul local, nicăieri.
- **În limba dumneavoastră.** Dictarea funcționează în zeci de limbi, iar meniul, dialogurile și notificările sunt disponibile în 35 de limbi.

## Cerințe

- macOS 14 Sonoma sau mai nou. Lowspeak este dezvoltat și testat pe macOS 27.
- Un Mac cu Apple Silicon sau cu procesor Intel. Transcrierea locală este construită pentru Apple Silicon, unde Whisper rulează pe GPU, iar Parakeet pe Neural Engine; pe Mac-urile cu Intel, ambele rulează pe procesor, mult mai lent, vedeți [Mac-uri cu Intel](#mac-uri-cu-intel).
- Circa 10 MB pentru aplicație, plus între 75 MB și 3 GB pentru fiecare model local pe care îl descărcați.
- Pentru transcrierea online, o cheie API de la [OpenAI](https://platform.openai.com/api-keys) sau [Groq](https://console.groq.com/keys); ambele taxează ce transcrieți. Transcrierea locală nu necesită niciun cont.

## Instalare

**Din imaginea de disc.** Descărcați [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), deschideți-o și trageți Lowspeak în dosarul Aplicații. Imaginea este semnată cu un Developer ID și notarizată de Apple, deci se deschide fără avertismente.

**Actualizări.** Lowspeak nu se actualizează singur. Versiunile noi apar pe [pagina de lansări](https://github.com/sventmueller/lowspeak/releases); **Ajutor › Lowspeak pe GitHub** duce acolo. Instalați versiunea nouă peste cea veche; setările, cheile și modelele rămân.

**Din sursă.** Vedeți [Compilare și instalare](../../../docs/development.md#build--deploy) pentru cerințe, apoi:

```sh
make deploy
```

Comanda compilează Lowspeak, înlocuiește copia din dosarul Aplicații și o lansează.

**Deschideți la login** din meniu pornește Lowspeak odată cu Mac-ul, prin funcția Lansare la login a sistemului macOS. Rulează o singură copie odată; a doua se închide imediat.

## Primii pași

### Permisiuni

La prima lansare, Lowspeak cere trei permisiuni:

- **Accesibilitate** (Configurări sistem › Intimitate și securitate › Accesibilitate; în macOS 27 lista se numește Control al dispozitivului și acces la date). Lowspeak are nevoie de ea pentru a observa tasta fn și pentru a lipi cu ⌘V. macOS se oferă să deschidă lista; activați comutatorul pentru Lowspeak. Mesajele Lowspeak denumesc lista așa cum o afișează macOS-ul dumneavoastră.
- **Microfon.** Cerută o singură dată; fără ea, Lowspeak nu înregistrează nimic.
- **Notificări.** Lowspeak le folosește pentru probleme pe care altfel nu le-ați observa, cum ar fi un microfon fără semnal sau o transcriere eșuată.

### Alegeți modul de transcriere

Configurați cel puțin un mod; îl puteți schimba oricând din meniu.

- **Online:** alegeți **Configurați cheia API…** și lipiți o cheie de la OpenAI (începe cu `sk-`) sau de la Groq (`gsk_`). Lowspeak recunoaște furnizorul după cheie. Apoi alegeți un model din **OpenAI** sau **Groq**.
- **Pe Mac-ul dumneavoastră:** alegeți **Gestionați modelele** › **Descărcați Parakeet Ultra** sau un model Whisper, apoi selectați-l din **Local**.

[Ce model vi se potrivește](#ce-model-vi-se-potrivește) vă ajută să alegeți. Apoi țineți apăsat fn, spuneți ceva și eliberați.

### Meniul

Faceți clic pe inelul din bara de meniu:

| Element | Ce face |
|---|---|
| **Dictări recente** | ultimele zece texte; un clic copiază din nou unul dintre ele în clipboard |
| **Local** | modelele de pe Mac-ul dumneavoastră, grupate în Whisper și Parakeet; cele nedescărcate apar estompate |
| **OpenAI**, **Groq** | modelele online; fără cheie apar estompate, iar meniul oferă **Configurați cheia API…** |
| **Limbă** | limba în care dictați sau **Detectare automată**; meniul o urmează |
| **Postprocesare** | **Dezactivată**, **Curățare** sau **Formatare**, vedeți [Postprocesare](#postprocesare) |
| **Vocabular…** | nume și termeni de scris exact; meniul arată câți ați introdus |
| **Microfon** | **Implicit pentru sistem** sau un microfon fix și **Semnal sonor când este gata** |
| **Gestionați modelele** | descărcarea și ștergerea modelelor locale, cu indicarea progresului |
| **Configurați cheia API…** / **Actualizați cheia API…** | adăugarea, înlocuirea sau eliminarea cheilor |
| **Deschideți la login** | pornește Lowspeak odată cu Mac-ul; bifa arată că opțiunea este activată |
| **Ajutor** | **Referință rapidă**, **Raportați o problemă…**, **Sugerați o funcție…**, **Lowspeak pe GitHub** |
| **Despre Lowspeak** | versiunea, drepturile de autor, pagina proiectului și licența |
| **Închideți aplicația Lowspeak** | ⌘Q cât timp meniul este deschis |

Bifa din **Local**, **OpenAI** și **Groq** marchează modelul folosit. Pictograma arată ce face Lowspeak:

| Pictogramă | Semnificație |
|---|---|
| Inel cu un punct | Pregătit |
| Cerc plin | Înregistrează cât timp fn este ținut apăsat |
| Pătrat rotunjit | Înregistrare blocată; următoarea apăsare pe fn sau Shift o încheie |
| Inel care clipește | Transcrie |
| ↓ 42 % | Descarcă un model |

Lowspeak nu are fereastră de setări: totul se află în acest meniu, care își amintește fiecare alegere.

### Referință rapidă

**Ajutor › Referință rapidă** afișează gesturile cu fn într-o fereastră mică, în limba dumneavoastră, cu un buton care deschide configurările tastaturii. **Raportați o problemă…** deschide un formular pe GitHub cu versiunile Lowspeak și macOS deja completate, **Sugerați o funcție…** o discuție pentru idei, iar **Lowspeak pe GitHub** pagina proiectului.

## Dictarea

### Țineți apăsat și vorbiți

Țineți apăsat fn, vorbiți, eliberați. Pictograma de înregistrare apare în momentul în care microfonul livrează sunet; începeți să vorbiți atunci, deoarece ce spuneți înainte nu este înregistrat. Eliberarea tastei fn încheie înregistrarea, iar textul urmează o clipă mai târziu.

O atingere scurtă pe fn nu face nimic, așa că fn își păstrează celelalte utilizări. O înregistrare care rămâne goală sau durează sub 0,3 secunde este abandonată.

### Blocarea cu Shift sau cu o apăsare dublă

Pentru texte mai lungi, blocați înregistrarea, astfel încât să ruleze fără să țineți vreo tastă. Există trei moduri:

- **Shift în timp ce țineți fn.** Începeți cu fn ca de obicei și apăsați Shift în timpul înregistrării: aceasta se blochează și puteți elibera fn. Nu se pierde nimic din ce ați spus înainte, înregistrarea pur și simplu continuă.
- **Shift, apoi fn.** Țineți apăsat Shift și apăsați fn: înregistrarea pornește blocată.
- **fn de două ori.** Atingeți scurt fn (mai puțin de 0,3 secunde) și apăsați-o din nou în cel mult 0,4 secunde. Înregistrarea rulează de la prima apăsare.

Pictograma devine un pătrat rotunjit. Apăsați o dată **fn** sau **Shift** pentru a opri; textul urmează ca de obicei. Dacă tasta 🌐 este setată să deschidă Emoji și simboluri, opriți cu Shift: o apăsare scurtă pe fn ar deschide fereastra respectivă.

O înregistrare se oprește singură după două minute cât timp fn este ținut apăsat și după cinci minute când este blocată, apoi este transcrisă ca de obicei.

### Unde ajunge textul

Lowspeak lipește textul în aplicația care era în prim-plan când ați terminat de vorbit, la cursor, prin clipboard și ⌘V. Salvează tot ce era înainte în clipboard și pune la loc conținutul la o jumătate de secundă după ce aplicația a citit textul sau după zece secunde dacă aceasta nu îl citește deloc, cu excepția cazului în care altă aplicație a modificat între timp clipboardul. Textul este marcat ca temporar, așa că aplicațiile de istoric al clipboardului care respectă acest marcaj nu îl înregistrează.

Puteți începe următoarea dictare în timp ce precedenta este încă transcrisă. Textele sunt lipite în ordinea în care le-ați rostit și niciodată în timp ce înregistrați sau țineți Shift, astfel că o lipire nu poate ajunge în mijlocul următoarei propoziții.

Fără permisiunea de accesibilitate sau când ⌘V simulat nu poate fi trimis, textul rămâne în clipboard și o notificare vă anunță acest lucru; lipiți-l singur cu ⌘V.

### Dictări recente

**Dictări recente** din meniu listează ultimele zece texte, cel mai nou primul, fiecare scurtat la un rând; când țineți cursorul deasupra, se afișează textul întreg. Un clic îl copiază în clipboard, pentru cazul în care un text a ajuns în locul greșit sau aveți nevoie de el de două ori. Lista există doar în memorie și este goală după închiderea Lowspeak.

## Transcrierea

### Ce model vi se potrivește

| Doriți | Alegeți | De ce |
|---|---|---|
| **Viteză fără a solicita Mac-ul** | **Groq › Whisper Large V3 Turbo** | cea mai rapidă opțiune online; nu se descarcă nimic, iar Mac-ul își păstrează memoria și spațiul pe disc |
| **Cel mai bun rezultat** | **OpenAI › GPT Transcribe** (implicit) | cel mai precis dintre modelele oferite de Lowspeak; primește vocabularul ca cuvinte-cheie |
| **Offline și privat** | **Local › Parakeet Ultra** pentru limbile europene, **Local › Whisper Large v3 Turbo** pentru toate celelalte | funcționează fără rețea, iar vocea nu părăsește niciodată Mac-ul; Parakeet este cel mai rapid dintre toate |
| **Fără cont, fără costuri** | orice model din **Local** | rulează pe Mac-ul dumneavoastră; nu se facturează nimic |
| **Fiabilitate și viteză deopotrivă** | un model online, plus Parakeet Ultra sau un model Whisper instalat | modelul local intervine când serviciul online nu funcționează |
| **Un Mac cu Intel** | **Groq** sau **OpenAI** | modelele locale rulează acolo pe procesor, vedeți [Mac-uri cu Intel](#mac-uri-cu-intel) |

Modelele locale ocupă pe disc cât dimensiunea descărcării și, cât timp sunt selectate, cam tot atâta memorie; Lowspeak păstrează modelul încărcat, astfel încât dictarea să înceapă fără întârziere. Pe un Mac cu 8 GB de memorie, preferați Parakeet Ultra sau un model online în locul modelelor Whisper mari.

### Online: OpenAI și Groq

| Model | Furnizor | Note |
|---|---|---|
| **GPT Transcribe** (implicit) | OpenAI | cel mai precis; primește vocabularul ca cuvinte-cheie |
| **Whisper** | OpenAI | modelul Whisper original |
| **Whisper Large V3** | Groq | foarte rapid |
| **Whisper Large V3 Turbo** | Groq | cel mai rapid online |

Înregistrarea este comprimată în AAC înainte de încărcare, circa 6 KB pe secundă de vorbire, astfel că dictarea funcționează și pe conexiuni lente. Cât timp țineți fn, Lowspeak deschide deja conexiunea cu furnizorul, astfel că încărcarea începe în momentul în care eliberați tasta.

### Pe Mac-ul dumneavoastră: Whisper și Parakeet

| Model | Dimensiune | Rulează pe | Limbi |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 de limbi europene |
| **Parakeet v2** | ~600 MB | Neural Engine | engleză |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | circa 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | circa 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | circa 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | circa 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | circa 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | circa 99 |

**Gestionați modelele** le descarcă și le șterge; în timpul unei descărcări, bara de meniu afișează progresul, iar închiderea Lowspeak o anulează. Prima utilizare a unui model îl pregătește pentru Mac-ul dumneavoastră: circa 10 secunde pentru Parakeet Ultra, până la 30 de secunde pentru Whisper; după aceea se încarcă într-o fracțiune de secundă. Dacă modelul ales este șters, Lowspeak trece la alt model instalat.

**Parakeet** este alegerea cea mai rapidă: transcrie 40 de secunde de vorbire în circa 0,3 secunde. Lowspeak îl rulează cu aceleași modele și aceeași decodare ca [FluidAudio](https://github.com/FluidInference/FluidAudio) și le stochează în dosarul FluidAudio (`~/Library/Application Support/FluidAudio/Models`), astfel că aplicațiile construite pe FluidAudio le folosesc în comun. Înregistrările mai lungi de 15 secunde sunt transcrise în ferestre care se suprapun și care sunt îmbinate fără a pierde cuvinte la îmbinări. **Whisper** înțelege mai multe limbi și citește vocabularul; modelele mari sunt cele mai precise, iar cele mici sunt mult mai slabe în afara limbii engleze. Modelele Whisper sunt stocate în `~/Library/Application Support/Lowspeak/Models`.

### Când serviciul online nu funcționează

Dacă o transcriere online eșuează, pentru că rețeaua este căzută, furnizorul are o problemă sau cheia este greșită, un model local instalat transcrie aceeași înregistrare: cel ales la **Local**, altfel Parakeet Ultra, apoi Whisper Large v3 Turbo, Large v3 și modelele Whisper mai mici, iar Parakeet v2 la final. O notificare vă anunță o singură dată; reapare doar după ce între timp a reușit o dictare online.

Când există un model local de rezervă, Lowspeak acordă cererii online 20 de secunde; fără unul, încearcă de trei ori, de fiecare dată până la un minut. În modul online, Lowspeak pregătește modelul Parakeet de rezervă o dată, la lansare, astfel că prima trecere pe rezervă nu trebuie să îl aștepte.

### Cum este pregătită înregistrarea

Lowspeak înregistrează la rata proprie a microfonului și, după ce vă opriți, convertește la cei 16 kHz pe care îi așteaptă modelele. Înainte de transcriere elimină huruitul grav de sub 80 Hz, taie liniștea de la început și de la sfârșit și egalizează volumul, astfel încât vorbitorii cu voce slabă și cei cu voce puternică se aud la fel. O înregistrare fără vorbire nu este trimisă deloc. În modul local, Parakeet primește întâi înregistrarea neprelucrată și pe cea pregătită doar dacă prima nu produce text.

### Limbi pentru fiecare model

Limba aleasă la **Limbă** este transmisă modelelor online și lui Whisper. Cu **Detectare automată**, modelul deduce singur limba.

| Model | Limbile în care puteți dicta |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, pe Mac-ul dumneavoastră) | circa 99, printre care toate cele 35 ale meniului |
| Parakeet Ultra | bulgară, croată, cehă, daneză, olandeză, engleză, estonă, finlandeză, franceză, germană, greacă, maghiară, italiană, letonă, lituaniană, malteză, poloneză, portugheză, română, rusă, slovacă, slovenă, spaniolă, suedeză, ucraineană |
| Parakeet v2 | engleză |

Parakeet detectează singur limba și ignoră alegerea din meniu. **Chineză (tradițională)** îi cere lui GPT Transcribe să scrie cu caractere tradiționale; Whisper scrie chineza în oricare dintre cele două scrieri, iar portugheza fără a ține cont de variantă.

### Mac-uri cu Intel

Lowspeak este o aplicație universală și rulează și pe Mac-urile cu Intel cu macOS 14. Tot ce ține de online funcționează la fel. Modelele locale nu au acolo hardware-ul pentru care sunt construite: Whisper rulează pe procesor în loc de GPU, iar Parakeet pe procesor în loc de Neural Engine. Ambele funcționează, dar durează de multe ori mai mult decât pe Apple Silicon, iar modelele Whisper mari mult prea mult pentru dictare. Pe un Mac cu Intel, transcrieți online și păstrați Whisper Small sau Parakeet Ultra ca [rezervă](#când-serviciul-online-nu-funcționează).

## Vocabular

**Vocabular…** preia numele și termenii pe care Lowspeak trebuie să îi scrie exact, separați prin virgule: nume de produse, de colegi, termeni tehnici. Până la 100 de termeni, de cel mult 60 de caractere fiecare; `<` și `>` sunt eliminate.

- **GPT Transcribe** îi primește ca cuvinte-cheie.
- **Whisper** (OpenAI, Groq și pe Mac-ul dumneavoastră) îi citește ca indiciu înaintea înregistrării, până la 500 de caractere.
- **Postprocesării** i se cere să îi scrie exact așa cum sunt scriși.
- **Parakeet** nu îi folosește.

## Postprocesare

După transcriere, un model lingvistic poate prelucra textul. Alegeți din **Postprocesare**:

- **Dezactivată** (implicit) – textul așa cum a fost transcris.
- **Curățare** – elimină cuvintele de umplutură precum „ăă”, „știi” sau „practic”, corectează gramatica și punctuația și, când vă corectați, păstrează doar corectura. Nu adaugă nimic și nu schimbă sensul.
- **Formatare** – ca Curățare, și în plus adaptează textul la aplicația în care dictați: termeni în stil de cod în editoarele de cod, linii de comandă în terminale, un ton relaxat în aplicațiile de chat, unul profesionist în e-mail.

Postprocesarea folosește Groq (`openai/gpt-oss-120b`) dacă aveți o cheie Groq, altfel OpenAI (`gpt-4o-mini`), și atunci când transcrieți local. Când este aleasă o limbă la **Limbă**, păstrează limba respectivă. Dacă durează mai mult de opt secunde, eșuează sau returnează un răspuns trunchiat ori mult mai lung decât ce ați spus, Lowspeak lipește textul neprelucrat. Dacă furnizorul refuză cererea sau nu este setată nicio cheie, o notificare vă spune acest lucru o singură dată.

## Microfon

La **Microfon**, alegeți un microfon fix sau **Implicit pentru sistem**. Lowspeak își amintește dispozitivul; cât timp nu este conectat, meniul îl afișează ca **(neconectat)**, iar Lowspeak înregistrează cu microfonul implicit al sistemului.

Microfonul este pornit doar cât timp dictați. Pregătirea lui durează circa 0,07 secunde cu microfonul încorporat, circa o jumătate de secundă cu un Studio Display și una până la două secunde cu AirPods, care trec mai întâi în modul cască cu microfon. Pictograma de înregistrare apare imediat ce sosește sunetul; **Semnal sonor când este gata** adaugă un sunet scurt în acel moment. Opțiunea este dezactivată implicit, deoarece pe unele difuzoare sunetul însuși vine cu o jumătate de secundă întârziere.

Dacă un microfon livrează doar liniște timp de cinci secunde, cum fac AirPods când alt dispozitiv le ocupă microfonul, Lowspeak vă anunță.

## Chei API

**Configurați cheia API…** cere o cheie și recunoaște furnizorul după începutul ei: `sk-` pentru OpenAI, `gsk_` pentru Groq. O cheie în alt format este refuzată, cu o explicație. Introduceți câte o cheie pentru fiecare furnizor; cu chei pentru amândoi, toate modelele online sunt disponibile. **Actualizați cheia API…** înlocuiește o cheie sau, cu **Eliminați toate cheile**, le șterge.

Cheile sunt stocate în portcheiul dumneavoastră de login, sub numele `com.lowspeak.app`. Fișierul de chei al unei versiuni anterioare (`~/Library/Application Support/Lowspeak/.api-key`) este mutat acolo la următoarea lansare și șters după ce fiecare cheie poate fi citită înapoi din portchei.

## Notificări

Lowspeak lucrează în fundal, așa că raportează problemele prin notificări:

| Notificare | Ce s-a întâmplat |
|---|---|
| **Înregistrarea a eșuat** | fără acces la microfon, niciun microfon conectat sau microfonul nu a putut porni |
| **Microfon fără semnal** | cinci secunde de liniște, vedeți [Microfon](#microfon) |
| **Este necesară permisiunea de accesibilitate** | textul se află în clipboard; permiteți Accesibilitate pentru lipirea automată |
| **Lipirea a eșuat** | textul se află în clipboard; lipiți-l cu ⌘V |
| **Transcrierea a eșuat** | cu motivul: nicio cheie, cheie respinsă, limită de cereri, eroare de server sau de rețea ori niciun model local |
| **Transcris pe acest Mac** | serviciul online nu a funcționat și a preluat un model local |
| **Postprocesarea a eșuat** | furnizorul refuză postprocesarea sau nu este setată nicio cheie; textul a fost lipit fără ea |
| **Nicio cheie API** | ați ales un model online fără o cheie pentru furnizorul său |
| **Descărcarea a eșuat**, **Ștergerea a eșuat**, **Nu se poate șterge** | gestionarea unui model local nu a funcționat sau o altă operațiune era încă în curs |
| **Eroare de lansare la login**, **Eroare la cheia API** | macOS sau portcheiul a refuzat modificarea |

Notificările nu conțin niciodată textul dictat. Un serviciu online care nu funcționează și un furnizor de postprocesare care refuză cererile sunt raportate o singură dată, nu după fiecare dictare, până când funcționează din nou.

## Aspect și accesibilitate

- **Pictograma din bara de meniu** este o imagine-șablon care urmează o bară de meniu deschisă, întunecată sau colorată.
- **VoiceOver** citește pictograma ca „Lowspeak”, iar meniul, alertele și notificările sunt controale macOS standard.
- **Nu trebuie ținută apăsată nicio tastă.** Blocați o înregistrare cu Shift sau cu o apăsare dublă pe fn, iar următoarea apăsare o încheie.
- **Referința rapidă** este o fereastră simplă cu etichete și un buton, accesibilă de la tastatură; Esc o închide.

Lacunele cunoscute sunt enumerate în [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver nu anunță când începe o înregistrare, iar Lowspeak nu a fost încă verificat cu VoiceOver sau cu Control comutator.

## Limbi

Meniul, dialogurile și notificările sunt traduse în 35 de limbi:

arabă, catalană, chineză (simplificată), chineză (tradițională), croată, cehă, daneză, olandeză, engleză, finlandeză, franceză, germană, greacă, ebraică, hindi, maghiară, indoneziană, italiană, japoneză, coreeană, malaeză, norvegiană, poloneză, portugheză (Brazilia), portugheză (Portugalia), română, rusă, slovacă, slovenă, spaniolă, suedeză, thailandeză, turcă, ucraineană și vietnameză.

Limba pe care o alegeți la **Limbă** este și limba meniului. Cu **Detectare automată**, meniul urmează ordinea limbilor din **Configurări sistem › General › Limbă și regiune**, iar acolo unde nu există traducere, engleza. Termenii din meniu urmează traducerile Apple, astfel că Lowspeak se citește ca restul Mac-ului dumneavoastră. În afară de engleză, traducerile sunt automate și nu au fost încă verificate de vorbitori nativi; corecturile sunt binevenite sub formă de [issues pe GitHub](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Taste și gesturi

| Acțiune | Taste |
|---|---|
| Dictare cât timp țineți apăsat | țineți apăsat fn |
| Blocarea unei înregistrări în curs | Shift în timp ce țineți fn |
| Pornirea unei înregistrări blocate | Shift+fn sau fn de două ori rapid |
| Încheierea unei înregistrări blocate | fn sau Shift |
| Închiderea Lowspeak (cu meniul deschis) | ⌘Q |
| Închiderea referinței rapide | Esc |

Lowspeak împiedică apăsările pe fn pe care le folosește să ajungă la alte aplicații; Shift trece mai departe ca de obicei.

## Confidențialitate și securitate

Lowspeak aude ce spuneți cât timp fn este ținut apăsat, poate scrie în orice aplicație și vă păstrează cheile API, așa că face cât mai puțin cu putință cu toate acestea.

- **Vocea dumneavoastră merge într-un singur loc.** În modul online, înregistrarea, comprimată, merge la furnizorul ales, OpenAI sau Groq, împreună cu vocabularul. În modul local, nimic nu părăsește Mac-ul. Când o cerere online eșuează, un model local transcrie aceeași înregistrare pe Mac-ul dumneavoastră.
- **Postprocesarea**, dacă o activați, trimite textul transcris și vocabularul la Groq sau OpenAI. În modul Formatare adaugă tipul de aplicație în care dictați (editor de cod, terminal, chat sau e-mail); numele aplicației nu este trimis.
- **Fără telemetrie, fără cont.** În afară de transcriere și postprocesare, Lowspeak se conectează la internet doar pentru a deschide conexiunea cu furnizorul cât timp fn este ținut apăsat, în modul online sau cu postprocesarea activată, și pentru a descărca un model când cereți unul, de pe Hugging Face prin HTTPS, la o versiune fixă și cu fiecare fișier verificat după suma lui de control. **Ajutor › Raportați o problemă…** deschide un formular pe GitHub în browser; adresa lui conține versiunea Lowspeak și versiunea macOS, nimic din ce ați dictat.
- **Cheile API** rămân în portcheiul dumneavoastră de login. Sunt trimise doar propriului furnizor și sunt eliminate din mesajele de eroare înainte ca ceva să fie scris în jurnal.
- **Textul dictat nu este stocat.** Ultimele zece dictări rămân în memorie pentru **Dictări recente** și dispar când Lowspeak se închide. Fișierele jurnal înregistrează lungimi și durate, niciodată textul.
- **Clipboardul** este salvat înainte de lipire și restaurat după ce aplicația a citit textul, cu excepția cazului în care altă aplicație l-a modificat între timp. Dictarea este marcată ca temporară, așa că aplicațiile de istoric al clipboardului care respectă acest marcaj o omit.
- **Semnarea codului** cu Hardened Runtime; aplicația cere doar microfonul, Accesibilitate, notificările și rețeaua.

Pentru a raporta o vulnerabilitate, urmați [politica de securitate](../../../SECURITY.md).

## Viteză

Măsurat pe Apple Silicon:

- **Parakeet Ultra** transcrie 40 de secunde de vorbire în circa 0,3 secunde.
- **Online:** prima cerere după o pauză durează circa 0,4 secunde la Groq, deoarece Lowspeak deschide conexiunea cât timp încă vorbiți; cererile ulterioare o refolosesc, prin HTTP/3 acolo unde furnizorul îl oferă.
- **Dimensiunea încărcării:** circa 6 KB pe secundă de vorbire, ca AAC la 48 kbit/s.
- **Pregătirea microfonului:** circa 0,07 secunde pentru cel încorporat, o jumătate de secundă pe un Studio Display, una până la două secunde cu AirPods.

## Limitări cunoscute

- **Modelele locale pe Mac-urile cu Intel** rulează pe procesor și sunt mult mai lente; acolo, transcrierea online este alegerea mai bună.
- **Ce spuneți înainte ca microfonul să fie pregătit se pierde.** Așteptați pictograma de înregistrare, mai ales cu AirPods.
- **Tasta fn poate fi preluată de alții.** Când setarea tastei 🌐 din macOS, Warp sau Dictare din macOS revendică primele tasta fn, Lowspeak nu o vede; vedeți [Depanare](#depanare).
- **Parakeet** ignoră vocabularul și alegerea limbii și cunoaște doar limbi europene.
- **Scriere și variantă:** doar lui GPT Transcribe i se poate cere să scrie în chineza tradițională; niciunui model nu i se poate cere să scrie portugheza europeană în locul celei braziliene.
- **Limite de timp:** două minute cât timp fn este ținut apăsat, cinci minute când înregistrarea este blocată.
- **Mod microfon:** Centrul de control oferă pentru Lowspeak Izolare voce și Spectru larg și le raportează ca active, dar Lowspeak înregistrează fără procesarea vocii, așa că alegerea nu are niciun efect.
- **Dictări recente** există doar în memorie și dispar după închidere.
- **Prin concepție:** fără fereastră de setări, fără pictogramă în Dock, fără actualizări automate, fără App Store.

## Depanare

**Nu se întâmplă nimic când țin apăsat fn.** Verificați trei lucruri: Lowspeak este permis la **Configurări sistem › Intimitate și securitate › Accesibilitate** (în macOS 27: **Control al dispozitivului și acces la date**); **Configurări sistem › Tastatură › Apăsarea tastei 🌐** este setată la **Nicio acțiune**; și nicio altă aplicație nu revendică tasta fn. Warp o face implicit; dezactivați acest lucru cu `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` și reporniți Warp. Și Dictare din macOS poate fi setată să pornească cu fn.

**După o actualizare, Lowspeak nu mai reacționează la fn.** macOS leagă permisiunea de accesibilitate de semnătura aplicației. Dacă semnătura s-a schimbat, de exemplu de la o versiune compilată de dumneavoastră la aplicația descărcată, eliminați Lowspeak din lista Accesibilitate (în macOS 27: Control al dispozitivului și acces la date) și adăugați-l din nou.

**Lipsesc primele cuvinte.** Începeți să vorbiți imediat ce apare pictograma de înregistrare. Cu AirPods, microfonul are nevoie de una până la două secunde; un microfon încorporat sau USB ales la **Microfon** este gata aproape imediat.

**„Microfon fără semnal.”** Microfonul nu a livrat niciun sunet timp de cinci secunde. Cu AirPods, este posibil ca alt dispozitiv să le folosească microfonul; treceți la microfonul încorporat sau reconectați-le. Și un capac de MacBook închis dezactivează microfonul încorporat.

**Dictările ies goale.** Volumul de intrare poate fi prea mic: aplicații precum Teams, Zoom sau Meet îl modifică în timpul apelurilor, iar macOS îl păstrează separat pentru fiecare dispozitiv. Măriți-l la **Configurări sistem › Sunet › Intrare**.

**Textul a ajuns în locul greșit.** **Dictări recente** din meniu îl copiază din nou în clipboard.

**„Nicio cheie API.”** Alegeți **Configurați cheia API…** din meniu sau treceți la un model local.

**Prima transcriere locală durează mult.** Fiecare model este pregătit pentru Mac-ul dumneavoastră la prima utilizare; de atunci se încarcă într-o fracțiune de secundă.

**Lowspeak se închide imediat după lansare.** O altă copie a Lowspeak rulează deja; închideți-o mai întâi pe aceea.

**Nu apar notificări.** Permiteți-le la **Configurări sistem › Notificări › Lowspeak**.

**Altceva.** Jurnalele din `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) descriu ce s-a întâmplat, fără textul dictat. Atașați rândurile relevante la un [raport de problemă](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Pentru dezvoltatori

Compilarea, testarea, structura codului și regulile pe care le urmează sunt descrise în [docs/development.md](../../../docs/development.md), iar contribuțiile în [CONTRIBUTING.md](../../../CONTRIBUTING.md) (ambele în engleză).

### Componente terțe

| Componentă | Versiune | Folosită pentru | Licență |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) prin [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | Whisper local pe GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | decodarea Parakeet și îmbinarea înregistrărilor lungi, traduse în Rust în `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) și crate-urile sale pentru framework-uri | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation și alte framework-uri de sistem | MIT; Zlib, Apache-2.0 sau MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | captarea microfonului prin CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | reeșantionare la 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) cu rustls | 3.5 | descărcarea modelelor | MIT sau Apache-2.0; rustls și ring sub ISC și Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | sumele de control ale modelelor descărcate | MIT sau Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | cheile API în portchei | MIT sau Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, erori, vocabularul Parakeet | MIT sau Apache-2.0; Apache-2.0 |

Textele licențelor pentru tot ce se află în aplicație sunt livrate împreună cu ea, ca `Contents/Resources/Acknowledgements.txt`.

Modelele nu fac parte din Lowspeak; aplicația le descarcă atunci când cereți:

| Model | Sursă | Licență |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper de la OpenAI în format ggml) | MIT |
| Parakeet v2, Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference), conversii CoreML ale modelului Parakeet TDT de la NVIDIA | CC-BY-4.0 |

## Licență

Lowspeak este licențiat sub oricare dintre

- Licența Apache, versiunea 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licența MIT ([LICENSE-MIT](../../../LICENSE-MIT))

la alegerea dumneavoastră.

Dacă nu declarați explicit altfel, orice contribuție trimisă în mod intenționat pentru includere în Lowspeak de către dumneavoastră, așa cum este definită în licența Apache-2.0, va fi licențiată dublu ca mai sus, fără termeni sau condiții suplimentare.
