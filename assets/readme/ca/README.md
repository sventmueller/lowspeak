<p><img src="../../readme-icon.png" alt="Icona del Lowspeak" width="168" height="168"></p>

# Lowspeak

**Dictat per al Mac que escriu allà on tens el cursor: mantén premuda la tecla fn, parla i deixa-la anar.**

El Lowspeak viu a la barra de menús. Mantén premuda la tecla fn (globus terraqüi) i parla; quan la deixes anar, les teves paraules apareixen a l’app on estiguis escrivint, com si les haguessis teclejat tu mateix. Transcriu en línia amb OpenAI o Groq, o completament al teu Mac amb Whisper o NVIDIA Parakeet, que converteix 40 segons de veu en text en aproximadament un terç de segon. Si la xarxa o el servei en línia fallen, un model local instal·lat pren el relleu, de manera que no es perd cap dictat. Ensenya-li els noms i els termes que fas servir, deixa que un model de llenguatge elimini les crosses si vols, i bloqueja un enregistrament amb Shift per dictar textos més llargs amb mans lliures. Està escrit en Rust amb controls AppKit autèntics, parla 35 idiomes i només envia la teva veu al proveïdor que has triat.

[![Descarrega el Lowspeak per a macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Descarrega Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: signat i notaritzat per Apple, per a macOS 14 o posterior, amb Apple Silicon i Intel.

**Has trobat un error?** [Obre una incidència](https://github.com/sventmueller/lowspeak/issues/new/choose) o tria **Ajuda › Informa d’un problema…** al menú del Lowspeak: el formulari s’obre amb la teva versió ja emplenada. Si el Lowspeak et resulta útil, una [estrella a GitHub](https://github.com/sventmueller/lowspeak) ajuda altres persones a trobar-lo.

[![Rust edició 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 o posterior](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon i Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit mitjançant objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp sobre Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet al Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![En línia amb OpenAI o Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 idiomes d’interfície](https://img.shields.io/badge/languages-35-4C9A2A)
![Signat amb Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Llicència: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#llicència)

[Bahasa Indonesia](../id/README.md) ·
[Bahasa Melayu](../ms/README.md) ·
**Català** ·
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

## Continguts

- [Resum de les funcions](#resum-de-les-funcions)
- [Requisits](#requisits)
- [Instal·lació](#installació)
- [Primers passos](#primers-passos)
  - [Permisos](#permisos)
  - [Tria com vols transcriure](#tria-com-vols-transcriure)
  - [El menú](#el-menú)
  - [Referència ràpida](#referència-ràpida)
- [Dictar](#dictar)
  - [Mantén premuda per parlar](#mantén-premuda-per-parlar)
  - [Bloqueja amb Shift o amb una doble pulsació](#bloqueja-amb-shift-o-amb-una-doble-pulsació)
  - [On va a parar el text](#on-va-a-parar-el-text)
  - [Dictats recents](#dictats-recents)
- [Transcripció](#transcripció)
  - [Quin model et convé](#quin-model-et-convé)
  - [En línia: OpenAI i Groq](#en-línia-openai-i-groq)
  - [Al teu Mac: Whisper i Parakeet](#al-teu-mac-whisper-i-parakeet)
  - [Quan falla el servei en línia](#quan-falla-el-servei-en-línia)
  - [Com es prepara l’enregistrament](#com-es-prepara-lenregistrament)
  - [Idiomes per model](#idiomes-per-model)
  - [Mac amb Intel](#mac-amb-intel)
- [Vocabulari](#vocabulari)
- [Postprocessament](#postprocessament)
- [Micròfon](#micròfon)
- [Claus d’API](#claus-dapi)
- [Notificacions](#notificacions)
- [Aparença i accessibilitat](#aparença-i-accessibilitat)
- [Idiomes](#idiomes)
- [Tecles i gestos](#tecles-i-gestos)
- [Privadesa i seguretat](#privadesa-i-seguretat)
- [Velocitat](#velocitat)
- [Limitacions conegudes](#limitacions-conegudes)
- [Resolució de problemes](#resolució-de-problemes)
- [Per a desenvolupadors](#per-a-desenvolupadors)
  - [Components de tercers](#components-de-tercers)
- [Llicència](#llicència)

## Resum de les funcions

- **Mantén premuda la tecla fn, parla i deixa-la anar.** El text s’escriu a l’app que tens al davant, allà on sigui el cursor: correu, xat, editor, terminal o navegador. Cap finestra a la qual canviar, cap botó on fer clic.
- **Bloqueja’l per a textos més llargs.** Prem Shift mentre mantens premuda la tecla fn, o prem fn dues vegades, i l’enregistrament continua sense haver de mantenir cap tecla premuda, fins a cinc minuts. Una pulsació de fn o Shift l’acaba.
- **El model adequat per a cada feina.** Groq per a la velocitat sense ocupar la memòria del teu Mac, GPT Transcribe d’OpenAI per al millor resultat, Whisper o NVIDIA Parakeet al teu Mac per treballar sense connexió i mantenir la teva veu en privat. Canvia d’un a l’altre amb un sol clic.
- **Cap dictat perdut.** Si la xarxa o el servei en línia fallen, un model local instal·lat transcriu el mateix enregistrament al teu Mac.
- **Les teves paraules, ben escrites.** Un vocabulari de noms i termes tècnics guia OpenAI, Groq, Whisper i el postprocessament.
- **Text net, si el vols.** El postprocessament opcional elimina les crosses, corregeix la gramàtica i, quan et corregeixes a tu mateix, només en conserva la correcció; o, a més, dona format al text segons el tipus d’app en què dictes.
- **Continua parlant.** Comença el dictat següent mentre l’anterior encara s’està transcrivint; els textos arriben en l’ordre en què els vas dir.
- **El teu porta-retalls continua sent teu.** El Lowspeak enganxa a través del porta-retalls i, un moment després, hi torna a posar el que hi havia. Els deu últims dictats es queden al menú per tornar-los a copiar.
- **Natiu fins al moll de l’os.** Un ítem real de la barra de menús amb un menú d’AppKit, alertes i notificacions estàndard. Sense Electron, sense finestra, sense icona al Dock.
- **Privat.** Sense compte ni telemetria. L’àudio només va al proveïdor que has triat, o enlloc en el mode local.
- **En el teu idioma.** El dictat funciona en desenes d’idiomes, i el menú, els diàlegs i les notificacions estan disponibles en 35.

## Requisits

- macOS 14 Sonoma o posterior. El Lowspeak es desenvolupa i es prova amb macOS 27.
- Un Mac amb Apple Silicon o amb processador Intel. La transcripció local està pensada per a Apple Silicon, on Whisper s’executa a la GPU i Parakeet al Neural Engine; als Mac amb Intel, tots dos s’executen al processador, molt més lentament; consulta [Mac amb Intel](#mac-amb-intel).
- Uns 10 MB per a l’app, més de 75 MB a 3 GB per cada model local que descarreguis.
- Per a la transcripció en línia, una clau d’API d’[OpenAI](https://platform.openai.com/api-keys) o de [Groq](https://console.groq.com/keys); tots dos cobren pel que transcrius. La transcripció local no necessita cap compte.

## Instal·lació

**Des de la imatge de disc.** Descarrega [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), obre’l i arrossega el Lowspeak a la carpeta Aplicacions. La imatge està signada amb un Developer ID i notaritzada per Apple, de manera que s’obre sense advertiments.

**Actualitzacions.** El Lowspeak no s’actualitza sol. Les versions noves apareixen a la [pàgina de versions](https://github.com/sventmueller/lowspeak/releases); **Ajuda › Lowspeak a GitHub** t’hi porta. Instal·la la versió nova damunt de l’antiga; els teus ajustos, claus i models es conserven.

**Des del codi font.** Mira [Compilació i desplegament](../../../docs/development.md#build--deploy) per als requisits previs i després:

```sh
make deploy
```

Això compila el Lowspeak, substitueix la còpia de la carpeta Aplicacions i l’obre.

**Obre en iniciar la sessió**, al menú, obre el Lowspeak juntament amb el teu Mac, mitjançant els ítems d’inici de sessió del mateix macOS. Només s’executa una còpia alhora; una segona es tanca immediatament.

## Primers passos

### Permisos

La primera vegada que l’obres, el Lowspeak demana tres permisos:

- **Accessibilitat** (Configuració del Sistema › Privacitat i seguretat › Accessibilitat; a macOS 27 la llista es diu Control de dispositius i accés a les dades). El Lowspeak el necessita per detectar la tecla fn i per enganxar amb ⌘V. macOS ofereix obrir la llista; activa l’interruptor del Lowspeak. Els missatges del Lowspeak anomenen la llista tal com la mostra el teu macOS.
- **Micròfon.** Es demana una vegada; sense aquest permís, el Lowspeak no enregistra res.
- **Notificacions.** El Lowspeak les fa servir per a problemes que, si no, no veuries, com ara un micròfon en silenci o una transcripció fallida.

### Tria com vols transcriure

Configura almenys una manera; la pots canviar en qualsevol moment des del menú.

- **En línia:** tria **Configura la clau d’API…** i enganxa una clau d’OpenAI (comença per `sk-`) o de Groq (`gsk_`). El Lowspeak reconeix el proveïdor per la clau. Després tria un model a **OpenAI** o **Groq**.
- **Al teu Mac:** tria **Gestiona els models** › **Descarrega Parakeet Ultra** o un model Whisper, i després selecciona’l a **Local**.

[Quin model et convé](#quin-model-et-convé) t’ajuda a triar. Després, mantén premuda la tecla fn, digues alguna cosa i deixa-la anar.

### El menú

Fes clic a l’anell de la barra de menús:

| Ítem | Què fa |
|---|---|
| **Dictats recents** | els deu últims textos; un clic en torna a copiar un al porta-retalls |
| **Local** | els models del teu Mac, agrupats en Whisper i Parakeet; els que no estan descarregats apareixen atenuats |
| **OpenAI**, **Groq** | els models en línia; sense clau apareixen atenuats i el menú ofereix **Configura la clau d’API…** |
| **Idioma** | l’idioma en què dictes, o **Detecció automàtica**; el menú el segueix |
| **Postprocessament** | **Desactivat**, **Neteja** o **Format**; consulta [Postprocessament](#postprocessament) |
| **Vocabulari…** | noms i termes que s’han d’escriure exactament; el menú mostra quants n’has introduït |
| **Micròfon** | **Valor per omissió del sistema** o un micròfon fix, i **Reprodueix un so quan estigui a punt** |
| **Gestiona els models** | descarregar i eliminar models locals, amb el progrés |
| **Configura la clau d’API…** / **Actualitza la clau d’API…** | afegir, substituir o eliminar claus |
| **Obre en iniciar la sessió** | obrir el Lowspeak juntament amb el teu Mac; la marca indica que està activat |
| **Ajuda** | **Referència ràpida**, **Informa d’un problema…**, **Suggereix una funció…**, **Lowspeak a GitHub** |
| **Sobre el Lowspeak** | versió, copyright, pàgina del projecte i llicència |
| **Surt del Lowspeak** | ⌘Q amb el menú obert |

La marca a **Local**, **OpenAI** i **Groq** indica el model en ús. La icona mostra què està fent el Lowspeak:

| Icona | Significat |
|---|---|
| Anell amb un punt | A punt |
| Cercle ple | Enregistrant mentre es manté premuda la tecla fn |
| Quadrat arrodonit | Enregistrament bloquejat; la pulsació següent de fn o Shift l’acaba |
| Anell parpellejant | Transcrivint |
| ↓ 42 % | Descarregant un model |

El Lowspeak no té finestra d’ajustos: tot és en aquest menú, i recorda cada elecció.

### Referència ràpida

**Ajuda › Referència ràpida** mostra els gestos de la tecla fn en una finestra petita, en el teu idioma, amb un botó que obre la configuració del teclat. **Informa d’un problema…** obre un formulari a GitHub amb les teves versions del Lowspeak i de macOS ja emplenades; **Suggereix una funció…**, una conversa per proposar idees; **Lowspeak a GitHub**, la pàgina del projecte.

## Dictar

### Mantén premuda per parlar

Mantén premuda la tecla fn, parla i deixa-la anar. La icona d’enregistrament apareix en el moment en què el micròfon lliura so; comença a parlar llavors, perquè el que diguis abans no s’enregistra. En deixar anar la tecla fn s’acaba l’enregistrament, i el text arriba un moment després.

Un toc breu a la tecla fn no fa res, de manera que fn conserva els seus altres usos. Un enregistrament que queda buit o dura menys de 0,3 segons es descarta.

### Bloqueja amb Shift o amb una doble pulsació

Per a textos més llargs, bloqueja l’enregistrament perquè continuï sense haver de mantenir cap tecla premuda. Hi ha tres maneres:

- **Shift mentre mantens premuda la tecla fn.** Comença amb fn com sempre i prem Shift durant l’enregistrament: es bloqueja, i pots deixar anar fn. No es perd res del que has dit; l’enregistrament simplement continua.
- **Shift i després fn.** Mantén premuda la tecla Shift i prem fn: l’enregistrament comença bloquejat.
- **fn dues vegades.** Prem fn breument (menys de 0,3 segons) i torna-la a prémer abans que passin 0,4 segons. L’enregistrament compta des de la primera pulsació.

La icona es converteix en un quadrat arrodonit. Prem **fn** o **Shift** una vegada per aturar-lo; el text arriba com sempre. Si la tecla 🌐 està configurada per obrir Emojis i símbols, atura’l amb Shift: una pulsació breu de fn obriria el visualitzador.

Un enregistrament s’atura sol al cap de dos minuts mentre es manté premuda la tecla fn, i al cap de cinc minuts quan està bloquejat, i després es transcriu com sempre.

### On va a parar el text

El Lowspeak enganxa el text a l’app que tenies al davant quan vas deixar de parlar, a la posició del cursor, mitjançant el porta-retalls i ⌘V. Desa tot el que hi havia abans al porta-retalls i ho restaura mig segon després que l’app hagi llegit el text, o al cap de deu segons si no el llegeix mai, tret que una altra app hagi canviat el porta-retalls mentrestant. El text es marca com a transitori, de manera que les apps d’historial del porta-retalls que respecten aquesta marca no el registren.

Pots començar el dictat següent mentre l’anterior encara s’està transcrivint. Els textos s’enganxen en l’ordre en què els vas dir, i mai mentre estàs enregistrant o mantens premuda la tecla Shift, de manera que un enganxament no pot caure al mig de la teva frase següent.

Sense el permís d’accessibilitat, o si no es pot enviar el ⌘V simulat, el text es queda al porta-retalls i una notificació t’ho indica; enganxa’l tu mateix amb ⌘V.

### Dictats recents

**Dictats recents**, al menú, mostra els deu últims textos, del més recent al més antic, cadascun escurçat a una línia; en passar-hi el punter per sobre es veu el text sencer. Un clic el copia al porta-retalls, per si un text ha anat a parar a un lloc equivocat o el necessites dues vegades. La llista només existeix a la memòria i queda buida quan surts del Lowspeak.

## Transcripció

### Quin model et convé

| Vols | Tria | Per què |
|---|---|---|
| **Velocitat sense carregar el teu Mac** | **Groq › Whisper Large V3 Turbo** | l’opció en línia més ràpida; no es descarrega res, i el teu Mac conserva la memòria i l’espai al disc |
| **El millor resultat** | **OpenAI › GPT Transcribe** (per omissió) | el més precís dels models que ofereix el Lowspeak; rep el teu vocabulari com a paraules clau |
| **Sense connexió i en privat** | **Local › Parakeet Ultra** per als idiomes europeus, **Local › Whisper Large v3 Turbo** per a tots els altres | funciona sense xarxa, i la teva veu no surt mai del teu Mac; Parakeet és el més ràpid de tots |
| **Sense compte ni costos** | qualsevol model **Local** | s’executa al teu Mac; no es factura res |
| **Fiabilitat i velocitat alhora** | un model en línia, més Parakeet Ultra o un model Whisper instal·lat | el model local entra en joc quan falla el servei en línia |
| **Un Mac amb Intel** | **Groq** o **OpenAI** | allà els models locals s’executen al processador; consulta [Mac amb Intel](#mac-amb-intel) |

Els models locals ocupen al disc la mida de la descàrrega i, mentre estan seleccionats, aproximadament el mateix a la memòria; el Lowspeak manté el model carregat perquè el dictat comenci sense demora. En un Mac amb 8 GB de memòria, és preferible Parakeet Ultra o un model en línia als models grans de Whisper.

### En línia: OpenAI i Groq

| Model | Proveïdor | Notes |
|---|---|---|
| **GPT Transcribe** (per omissió) | OpenAI | el més precís; rep el teu vocabulari com a paraules clau |
| **Whisper** | OpenAI | el model Whisper original |
| **Whisper Large V3** | Groq | molt ràpid |
| **Whisper Large V3 Turbo** | Groq | el més ràpid en línia |

L’enregistrament es comprimeix en AAC abans de pujar-lo, uns 6 KB per segon de veu, de manera que el dictat també funciona amb connexions lentes. Mentre mantens premuda la tecla fn, el Lowspeak ja obre la connexió amb el proveïdor, de manera que la pujada comença en el moment en què la deixes anar.

### Al teu Mac: Whisper i Parakeet

| Model | Mida | S’executa a | Idiomes |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 idiomes europeus |
| **Parakeet v2** | ~600 MB | Neural Engine | anglès |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | uns 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | uns 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | uns 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | uns 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | uns 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | uns 99 |

**Gestiona els models** els descarrega i els elimina; durant una descàrrega, la barra de menús en mostra el progrés, i si surts del Lowspeak es cancel·la. El primer ús d’un model el prepara per al teu Mac: uns 10 segons per a Parakeet Ultra i fins a 30 segons per a Whisper; després es carrega en una fracció de segon. Si s’elimina el model que has triat, el Lowspeak canvia a un altre dels instal·lats.

**Parakeet** és l’opció més ràpida: transcriu 40 segons de veu en uns 0,3 segons. El Lowspeak l’executa amb els mateixos models i la mateixa descodificació que [FluidAudio](https://github.com/FluidInference/FluidAudio) i els desa a la carpeta de FluidAudio (`~/Library/Application Support/FluidAudio/Models`), de manera que les apps basades en FluidAudio els comparteixen. Els enregistraments de més de 15 segons es transcriuen en finestres superposades que s’uneixen sense perdre paraules a les juntures. **Whisper** entén més idiomes i llegeix el teu vocabulari; els models grans són els més precisos, i els petits, molt més febles fora de l’anglès. Els models Whisper es desen a `~/Library/Application Support/Lowspeak/Models`.

### Quan falla el servei en línia

Si una transcripció en línia falla, perquè la xarxa no funciona, el proveïdor té un problema o la clau és incorrecta, un model local instal·lat transcriu el mateix enregistrament: el que has triat a **Local** o, si no, Parakeet Ultra, després Whisper Large v3 Turbo, Large v3 i els models Whisper més petits, i Parakeet v2 en últim lloc. Una notificació t’ho diu una vegada; només torna a aparèixer després que un dictat en línia hagi funcionat entremig.

Amb un model local de reserva, el Lowspeak dona 20 segons a la sol·licitud en línia; sense, ho intenta tres vegades durant un minut com a màxim cada vegada. En el mode en línia, el Lowspeak prepara una vegada a l’inici el model Parakeet de reserva, perquè la primera vegada que calgui no l’hagi d’esperar.

### Com es prepara l’enregistrament

El Lowspeak enregistra a la freqüència de mostreig pròpia del micròfon i, quan t’atures, el converteix als 16 kHz que esperen els models. Abans de la transcripció elimina el brunzit greu per sota de 80 Hz, retalla el silenci del principi i del final i iguala el volum, de manera que les veus fluixes i les fortes surten igual. Un enregistrament sense veu ni tan sols s’envia. En el mode local, Parakeet rep primer l’enregistrament sense tocar, i el preparat només si aquell no dona cap text.

### Idiomes per model

L’idioma triat a **Idioma** es passa als models en línia i a Whisper. Amb **Detecció automàtica**, el model determina l’idioma pel seu compte.

| Model | Idiomes en què pots dictar |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, al teu Mac) | uns 99, entre els quals tots els 35 del menú |
| Parakeet Ultra | alemany, anglès, búlgar, castellà, croat, danès, eslovac, eslovè, estonià, finès, francès, grec, hongarès, italià, letó, lituà, maltès, neerlandès, polonès, portuguès, romanès, rus, suec, txec, ucraïnès |
| Parakeet v2 | anglès |

Parakeet detecta l’idioma pel seu compte i ignora l’elecció del menú. **Xinès (tradicional)** indica a GPT Transcribe que escrigui caràcters tradicionals; Whisper escriu el xinès en qualsevol de les dues escriptures, i el portuguès sense tenir en compte la variant.

### Mac amb Intel

El Lowspeak és una app universal i també funciona als Mac amb Intel amb macOS 14. Tot el que va en línia funciona igual. Allà, els models locals no tenen el maquinari per al qual estan fets: Whisper s’executa al processador en lloc de la GPU, i Parakeet al processador en lloc del Neural Engine. Tots dos funcionen, però triguen moltes vegades més que amb Apple Silicon, i els models grans de Whisper, massa per dictar. En un Mac amb Intel, transcriu en línia i tingues Whisper Small o Parakeet Ultra com a [reserva](#quan-falla-el-servei-en-línia).

## Vocabulari

**Vocabulari…** recull els noms i els termes que el Lowspeak ha d’escriure exactament, separats per comes: noms de productes, companys de feina, termes tècnics. Fins a 100 termes de fins a 60 caràcters cadascun; `<` i `>` s’eliminen.

- **GPT Transcribe** els rep com a paraules clau.
- **Whisper** (OpenAI, Groq i al teu Mac) els llegeix com a pista abans de l’enregistrament, fins a 500 caràcters.
- **El postprocessament** rep la indicació d’escriure’ls exactament com estan escrits.
- **Parakeet** no els fa servir.

## Postprocessament

Després de la transcripció, un model de llenguatge pot reelaborar el text. Tria a **Postprocessament**:

- **Desactivat** (per omissió): el text tal com s’ha transcrit.
- **Neteja**: elimina crosses com ara «eh», «o sigui» o «bàsicament», corregeix la gramàtica i la puntuació i, quan et corregeixes a tu mateix, només en conserva la correcció. No hi afegeix res ni en canvia el sentit.
- **Format**: com Neteja, i a més adapta el text a l’app en què dictes: termes amb estil de codi als editors de codi, línies d’ordres als terminals, un to informal a les apps de xat i un de professional al correu.

El postprocessament fa servir Groq (`openai/gpt-oss-120b`) si tens una clau de Groq i, si no, OpenAI (`gpt-4o-mini`), també quan transcrius en local. Si has triat un idioma a **Idioma**, el manté. Si triga més de vuit segons, falla o retorna una resposta tallada o molt més llarga que el que has dit, el Lowspeak enganxa el text sense processar. Si el proveïdor rebutja la sol·licitud o no hi ha cap clau configurada, una notificació t’ho diu una vegada.

## Micròfon

A **Micròfon**, tria un micròfon fix o **Valor per omissió del sistema**. El Lowspeak recorda el dispositiu; mentre no està connectat, el menú el mostra amb **(no connectat)** i el Lowspeak enregistra amb el micròfon per omissió del sistema.

El micròfon només s’activa mentre dictes. Posar-lo a punt triga uns 0,07 segons amb el micròfon integrat, aproximadament mig segon amb un Studio Display i d’un a dos segons amb uns AirPods, que primer canvien al mode d’auriculars amb micròfon. La icona d’enregistrament apareix quan arriba el so; **Reprodueix un so quan estigui a punt** hi afegeix un so breu en aquell moment. Està desactivat per omissió, perquè en alguns altaveus el mateix so arriba mig segon tard.

Si un micròfon només lliura silenci durant cinc segons, com passa amb els AirPods quan un altre dispositiu en té ocupat el micròfon, el Lowspeak t’ho diu.

## Claus d’API

**Configura la clau d’API…** demana una clau i reconeix el proveïdor per com comença: `sk-` per a OpenAI, `gsk_` per a Groq. Una clau amb un altre format es rebutja amb una explicació. Introdueix una clau per proveïdor; amb claus de tots dos, tots els models en línia estan disponibles. **Actualitza la clau d’API…** substitueix una clau o, amb **Elimina totes les claus**, les esborra.

Les claus es desen al teu clauer d’inici de sessió amb el nom `com.lowspeak.app`. El fitxer de claus d’una versió anterior (`~/Library/Application Support/Lowspeak/.api-key`) s’hi trasllada la propera vegada que obres el Lowspeak i s’elimina un cop totes les claus es poden tornar a llegir del clauer.

## Notificacions

El Lowspeak treballa en segon pla, així que informa dels problemes amb notificacions:

| Notificació | Què ha passat |
|---|---|
| **Error d’enregistrament** | no hi ha accés al micròfon, no hi ha cap micròfon connectat o el micròfon no s’ha pogut iniciar |
| **Micròfon en silenci** | cinc segons de silenci; consulta [Micròfon](#micròfon) |
| **Cal el permís d’accessibilitat** | el text és al porta-retalls; concedeix el permís d’accessibilitat per enganxar automàticament |
| **Error en enganxar** | el text és al porta-retalls; enganxa’l amb ⌘V |
| **Error de transcripció** | amb el motiu: no hi ha clau, clau rebutjada, límit de sol·licituds, error del servidor o de xarxa, o cap model local |
| **Transcrit en aquest Mac** | el servei en línia ha fallat i un model local ha pres el relleu |
| **Error de postprocessament** | el proveïdor rebutja el postprocessament o no hi ha cap clau configurada; el text s’ha enganxat sense |
| **No hi ha cap clau d’API** | has triat un model en línia sense clau per al seu proveïdor |
| **Error de descàrrega**, **Error en eliminar**, **No es pot eliminar** | la gestió d’un model local no ha funcionat, o una altra operació encara estava en curs |
| **Error de l’ítem d’inici**, **Error de la clau d’API** | macOS o el clauer ha rebutjat el canvi |

Les notificacions no contenen mai el text que has dictat. Un servei en línia que falla i un proveïdor de postprocessament que rebutja les sol·licituds es notifiquen una sola vegada, no després de cada dictat, fins que tornen a funcionar.

## Aparença i accessibilitat

- **La icona de la barra de menús** és una imatge de plantilla que s’adapta a una barra de menús clara, fosca o tenyida.
- **VoiceOver** llegeix la icona com a «Lowspeak», i el menú, les alertes i les notificacions són controls estàndard de macOS.
- **No cal mantenir cap tecla premuda.** Bloqueja un enregistrament amb Shift o prement fn dues vegades, i la pulsació següent l’acaba.
- **La referència ràpida** és una finestra senzilla amb etiquetes i un botó, accessible des del teclat; Esc la tanca.

Les mancances conegudes es recullen a [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver no anuncia quan comença un enregistrament, i el Lowspeak encara no s’ha provat amb VoiceOver ni amb el control de polsador.

## Idiomes

El menú, els diàlegs i les notificacions estan traduïts a 35 idiomes:

alemany, anglès, àrab, castellà, català, coreà, croat, danès, eslovac, eslovè, finès, francès, grec, hebreu, hindi, hongarès, indonesi, italià, japonès, malai, neerlandès, noruec, polonès, portuguès (Brasil), portuguès (Portugal), romanès, rus, suec, tailandès, turc, txec, ucraïnès, vietnamita, xinès (simplificat) i xinès (tradicional).

L’idioma que triïs a **Idioma** és també l’idioma del menú. Amb **Detecció automàtica**, el menú segueix l’ordre d’idiomes de **Configuració del Sistema › General › Idioma i regió**, i fa servir l’anglès quan no hi ha traducció. Els termes dels menús segueixen les traduccions d’Apple, de manera que el Lowspeak es llegeix com la resta del teu Mac. Llevat de l’anglès, les traduccions són automàtiques i encara no les han revisat parlants nadius; les correccions són benvingudes com a [incidències](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Tecles i gestos

| Acció | Tecles |
|---|---|
| Dictar mentre es manté premuda | mantenir premuda la tecla fn |
| Bloquejar un enregistrament en curs | Shift mentre mantens premuda la tecla fn |
| Iniciar un enregistrament bloquejat | Shift+fn, o fn dues vegades ràpidament |
| Acabar un enregistrament bloquejat | fn o Shift |
| Sortir del Lowspeak (amb el menú obert) | ⌘Q |
| Tancar la referència ràpida | Esc |

El Lowspeak evita que les pulsacions de fn que fa servir arribin a altres apps; Shift passa com sempre.

## Privadesa i seguretat

El Lowspeak sent el que dius mentre mantens premuda la tecla fn, pot escriure a qualsevol app i guarda les teves claus d’API, així que en fa el mínim ús possible.

- **La teva veu va a un sol lloc.** En el mode en línia, l’enregistrament, comprimit, va al proveïdor que has triat, OpenAI o Groq, juntament amb el teu vocabulari. En el mode local, res no surt del teu Mac. Si una sol·licitud en línia falla, un model local transcriu el mateix enregistrament al teu Mac.
- **El postprocessament**, si l’actives, envia el text transcrit i el teu vocabulari a Groq o a OpenAI. En el mode Format hi afegeix el tipus d’app en què dictes (editor de codi, terminal, xat o correu); el nom de l’app no s’envia.
- **Sense telemetria ni compte.** A part de la transcripció i el postprocessament, el Lowspeak només es connecta a internet per obrir la connexió amb el teu proveïdor mentre mantens premuda la tecla fn, en el mode en línia o amb el postprocessament activat, i per descarregar un model quan ho demanes, des de Hugging Face per HTTPS, en una versió fixa i comprovant la suma de verificació de cada fitxer. **Ajuda › Informa d’un problema…** obre un formulari de GitHub al navegador; la seva adreça porta la versió del Lowspeak i la de macOS, res del que has dictat.
- **Les claus d’API** es queden al teu clauer d’inici de sessió. Només s’envien al seu propi proveïdor i s’eliminen dels missatges d’error abans d’escriure res al registre.
- **El text dictat no es desa.** Els deu últims dictats es queden a la memòria per a **Dictats recents** i desapareixen quan surts del Lowspeak. Els fitxers de registre anoten longituds i temps, mai el text.
- **El porta-retalls** es desa abans d’enganxar i es restaura un cop l’app ha llegit el text, tret que una altra app l’hagi canviat mentrestant. El dictat es marca com a transitori, de manera que les apps d’historial del porta-retalls que respecten aquesta marca l’ometen.
- **Signatura de codi** amb el Hardened Runtime; l’app només demana accés al micròfon, a l’accessibilitat, a les notificacions i a la xarxa.

Per informar d’una vulnerabilitat, segueix la [política de seguretat](../../../SECURITY.md).

## Velocitat

Mesurat amb Apple Silicon:

- **Parakeet Ultra** transcriu 40 segons de veu en uns 0,3 segons.
- **En línia:** la primera sol·licitud després d’una pausa triga uns 0,4 segons a Groq, perquè el Lowspeak obre la connexió mentre encara parles; les següents la reutilitzen, per HTTP/3 quan el proveïdor ho ofereix.
- **Mida de la pujada:** uns 6 KB per segon de veu, en AAC a 48 kbit/s.
- **Posada a punt del micròfon:** uns 0,07 segons amb l’integrat, mig segon amb un Studio Display i d’un a dos segons amb AirPods.

## Limitacions conegudes

- **Els models locals als Mac amb Intel** s’executen al processador i són molt més lents; allà és millor transcriure en línia.
- **El que dius abans que el micròfon estigui a punt es perd.** Espera la icona d’enregistrament, sobretot amb AirPods.
- **Altres poden apropiar-se de la tecla fn.** Si l’ajust de la tecla 🌐 de macOS, el Warp o el Dictat de macOS reclamen fn primer, el Lowspeak no la veu; consulta [Resolució de problemes](#resolució-de-problemes).
- **Parakeet** ignora el vocabulari i l’elecció d’idioma, i només coneix idiomes europeus.
- **Escriptura i variant:** només a GPT Transcribe se li pot indicar que escrigui en xinès tradicional; a cap model se li pot indicar que escrigui portuguès europeu en lloc de brasiler.
- **Límits de temps:** dos minuts mentre es manté premuda la tecla fn, cinc minuts quan l’enregistrament està bloquejat.
- **Mode de micròfon:** el centre de control ofereix Aïllament de veu i Ampli espectre per al Lowspeak i els mostra com a actius, però el Lowspeak enregistra sense processament de veu, de manera que l’elecció no té cap efecte.
- **Dictats recents** només existeix a la memòria i desapareix quan surts.
- **Per disseny:** sense finestra d’ajustos, sense icona al Dock, sense actualitzacions automàtiques i fora de l’App Store.

## Resolució de problemes

**No passa res quan mantinc premuda la tecla fn.** Comprova tres coses: el Lowspeak té permís a **Configuració del Sistema › Privacitat i seguretat › Accessibilitat** (a macOS 27: **Control de dispositius i accés a les dades**); **Configuració del Sistema › Teclat › En prémer la tecla 🌐** està definit com a **no facis res**; i cap altra app no reclama fn. El Warp ho fa per omissió; desactiva-ho amb `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` i reinicia el Warp. El Dictat de macOS també es pot configurar perquè comenci amb fn.

**Després d’una actualització, el Lowspeak ja no reacciona a fn.** macOS lliga el permís d’accessibilitat a la signatura de l’app. Si la signatura ha canviat, per exemple d’una compilació pròpia a l’app descarregada, treu el Lowspeak de la llista d’Accessibilitat (a macOS 27: Control de dispositius i accés a les dades) i torna’l a afegir.

**Falten les primeres paraules.** Comença a parlar quan aparegui la icona d’enregistrament. Amb AirPods, el micròfon necessita d’un a dos segons; un micròfon integrat o USB triat a **Micròfon** està a punt gairebé a l’instant.

**«Micròfon en silenci».** El micròfon no ha lliurat cap so durant cinc segons. Amb AirPods, potser un altre dispositiu està fent servir el seu micròfon; canvia al micròfon integrat o torna’ls a connectar. Amb la tapa del MacBook tancada, el micròfon integrat també queda silenciat.

**Els dictats surten buits.** Potser el volum d’entrada és massa baix: apps com el Teams, el Zoom o el Meet el canvien durant les trucades, i macOS el guarda per a cada dispositiu. Apuja’l a **Configuració del Sistema › So › Entrada**.

**El text ha anat a parar a un lloc equivocat.** **Dictats recents**, al menú, el torna a copiar al porta-retalls.

**«No hi ha cap clau d’API».** Tria **Configura la clau d’API…** al menú o canvia a un model local.

**La primera transcripció local triga molt.** Cada model es prepara per al teu Mac la primera vegada que s’usa; a partir d’aleshores es carrega en una fracció de segon.

**El Lowspeak es tanca just després d’obrir-lo.** Ja s’està executant una altra còpia del Lowspeak; tanca-la primer.

**No apareix cap notificació.** Permet-les a **Configuració del Sistema › Notificacions › Lowspeak**.

**Una altra cosa.** Els registres de `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) descriuen què ha passat, sense el text que has dictat. Adjunta les línies rellevants a un [informe de problema](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Per a desenvolupadors

La compilació, les proves, l’estructura del codi i les regles que segueix es descriuen a [docs/development.md](../../../docs/development.md); les contribucions, a [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Components de tercers

| Component | Versió | Ús | Llicència |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) mitjançant [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | Whisper local a la GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | descodificació de Parakeet i fusió d’enregistraments llargs, traduïdes a Rust a `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) i els seus crates de frameworks | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation i altres frameworks del sistema | MIT; Zlib, Apache-2.0 o MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | captura del micròfon mitjançant CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | remostreig a 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) amb rustls | 3.5 | descàrrega de models | MIT o Apache-2.0; rustls i ring sota ISC i Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | sumes de verificació de les descàrregues de models | MIT o Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | claus d’API al clauer | MIT o Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, errors, el vocabulari de Parakeet | MIT o Apache-2.0; Apache-2.0 |

Els textos de llicència de tot el que conté l’app s’hi inclouen com a `Contents/Resources/Acknowledgements.txt`.

Els models no formen part del Lowspeak; els descarrega quan li ho demanes:

| Model | Font | Llicència |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (el Whisper d’OpenAI en format ggml) | MIT |
| Parakeet v2, Parakeet Ultra | conversions a CoreML de [FluidInference](https://huggingface.co/FluidInference) del Parakeet TDT de NVIDIA | CC-BY-4.0 |

## Llicència

El Lowspeak té llicència sota qualsevol de les dues següents:

- Llicència Apache, versió 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Llicència MIT ([LICENSE-MIT](../../../LICENSE-MIT))

a la teva elecció.

Tret que indiquis explícitament el contrari, qualsevol contribució enviada intencionadament per tu per ser inclosa al Lowspeak, tal com es defineix a la llicència Apache-2.0, tindrà la doble llicència anterior, sense cap condició addicional.
