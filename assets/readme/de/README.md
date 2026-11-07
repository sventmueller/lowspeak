<p><img src="../../readme-icon.png" alt="App-Symbol von Lowspeak" width="168" height="168"></p>

# Lowspeak

**fn halten, sprechen, loslassen. Deine Worte stehen schon da.**

Lowspeak wohnt in deiner Menüleiste und hört nur zu, solange du die fn-Taste hältst. Sag, was du schreiben willst, lass los, und der Text erscheint in der App, in der du gerade bist: Mail, Chat, Code-Editor, Terminal.

Das ist eine echte Mac-App, in Rust auf AppKit geschrieben, ohne Browser-Engine darunter. Sie startet sofort und drängt sich nie auf. Die Verbindung öffnet sie sogar schon, während du noch sprichst, und so steht der Text online etwa 0,4 Sekunden nach dem Loslassen da.

Zwölf Spracherkennungsmodelle stehen zur Wahl. Mit deinem eigenen Groq-Schlüssel kostet die Onlinetranskription nichts, solange du im Tageskontingent von Groq bleibst; GPT Transcribe von OpenAI übernimmt die schwierigen Aufnahmen. Lowspeak redet direkt mit ihnen, ohne eigenes Konto und ohne eigenen Server dazwischen.

Oder du behältst alles auf deinem Mac. Whisper und NVIDIA Parakeet laufen offline, deine Stimme verlässt das Gerät also nie, und Parakeet verwandelt 40 Sekunden Sprache in etwa einer Drittelsekunde in Text. Arbeitest du online, springt ein installiertes lokales Modell ein, wenn das Netzwerk ausfällt, und kein Diktat geht verloren.

Lowspeak spricht 35 Sprachen. Es lernt die Namen und Begriffe, die du verwendest, streicht auf Wunsch Füllwörter und glättet die Grammatik. Deine letzten zehn Diktate behält es im Menü, falls du eines noch einmal brauchst.

[![Lowspeak für macOS laden](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Lowspeak.dmg laden](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: von Apple signiert und notarisiert, für macOS 14 oder neuer auf Apple Silicon und Intel.

**Fehler gefunden?** [Eröffne ein Issue](https://github.com/sventmueller/lowspeak/issues/new/choose) oder wähle im Menü von Lowspeak **Hilfe › Problem melden …**: Das Formular öffnet sich mit bereits eingetragener Version. Wenn dir Lowspeak nützt, hilft ein [Stern auf GitHub](https://github.com/sventmueller/lowspeak) anderen, es zu finden.

[![Rust 2024 edition](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 oder neuer](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon und Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit via objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp auf Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet auf der Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online mit OpenAI oder Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 Sprachen der Oberfläche](https://img.shields.io/badge/languages-35-4C9A2A)
![Mit Hardened Runtime signiert](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Lizenz: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#lizenz)

[Bahasa Indonesia](../id/README.md) ·
[Bahasa Melayu](../ms/README.md) ·
[Català](../ca/README.md) ·
[Čeština](../cs/README.md) ·
[Dansk](../da/README.md) ·
**Deutsch** ·
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

## Inhalt

- [Funktionen im Überblick](#funktionen-im-überblick)
- [Voraussetzungen](#voraussetzungen)
- [Installation](#installation)
- [Erste Schritte](#erste-schritte)
  - [Berechtigungen](#berechtigungen)
  - [Art der Transkription wählen](#art-der-transkription-wählen)
  - [Das Menü](#das-menü)
  - [Kurzreferenz](#kurzreferenz)
- [Diktieren](#diktieren)
  - [Halten zum Sprechen](#halten-zum-sprechen)
  - [Mit Shift oder Doppeldruck feststellen](#mit-shift-oder-doppeldruck-feststellen)
  - [Wohin der Text geht](#wohin-der-text-geht)
  - [Letzte Diktate](#letzte-diktate)
- [Transkription](#transkription)
  - [Welches Modell zu dir passt](#welches-modell-zu-dir-passt)
  - [Online: OpenAI und Groq](#online-openai-und-groq)
  - [Auf deinem Mac: Whisper und Parakeet](#auf-deinem-mac-whisper-und-parakeet)
  - [Wenn der Onlinedienst ausfällt](#wenn-der-onlinedienst-ausfällt)
  - [Wie die Aufnahme vorbereitet wird](#wie-die-aufnahme-vorbereitet-wird)
  - [Sprachen je Modell](#sprachen-je-modell)
  - [Intel-Macs](#intel-macs)
- [Vokabular](#vokabular)
- [Nachbearbeitung](#nachbearbeitung)
- [Mikrofon](#mikrofon)
- [API-Schlüssel](#api-schlüssel)
- [Mitteilungen](#mitteilungen)
- [Darstellung und Bedienungshilfen](#darstellung-und-bedienungshilfen)
- [Sprachen](#sprachen)
- [Tasten und Gesten](#tasten-und-gesten)
- [Datenschutz und Sicherheit](#datenschutz-und-sicherheit)
- [Geschwindigkeit](#geschwindigkeit)
- [Bekannte Einschränkungen](#bekannte-einschränkungen)
- [Fehlerbehebung](#fehlerbehebung)
- [Für Entwickler](#für-entwickler)
  - [Komponenten von Drittanbietern](#komponenten-von-drittanbietern)
- [Lizenz](#lizenz)

## Funktionen im Überblick

- **Durch und durch nativ.** Ein echtes Objekt in der Menüleiste mit einem AppKit-Menü, Standarddialogen und -mitteilungen. Kein Electron, kein Fenster, kein Dock-Symbol.
- **Das richtige Modell für jeden Zweck.** Groq für Tempo, ohne den Arbeitsspeicher deines Mac zu belegen, GPT Transcribe von OpenAI für das beste Ergebnis, Whisper oder NVIDIA Parakeet auf deinem Mac, um offline zu arbeiten und deine Stimme für dich zu behalten. Gewechselt wird mit einem Klick.
- **Privat.** Kein Konto, keine Telemetrie. Audio geht nur an den Anbieter, den du gewählt hast, und im lokalen Modus nirgendwohin.
- **Kein Diktat geht verloren.** Fallen das Netzwerk oder der Onlinedienst aus, transkribiert ein installiertes lokales Modell dieselbe Aufnahme auf deinem Mac.
- **In deiner Sprache.** Diktieren funktioniert in Dutzenden Sprachen, und Menü, Dialoge und Mitteilungen gibt es in 35.
- **Deine Wörter, richtig geschrieben.** Ein Vokabular aus Namen und Fachbegriffen leitet OpenAI, Groq, Whisper und die Nachbearbeitung.
- **Sauberer Text, wenn du willst.** Die optionale Nachbearbeitung entfernt Füllwörter, korrigiert die Grammatik und behält nur deine Korrektur, wenn du dich selbst verbesserst, oder formatiert den Text zusätzlich passend zur Art der App, in die du diktierst.
- **Deine Zwischenablage bleibt deine.** Lowspeak fügt über die Zwischenablage ein und legt einen Moment später zurück, was vorher darin lag. Die letzten zehn Diktate bleiben im Menü, damit du sie erneut kopieren kannst.
- **fn halten, sprechen, loslassen.** Der Text wird in die App im Vordergrund getippt, dort, wo der Cursor steht: Mail, Chat, Editor, Terminal oder Browser. Kein Fenster, zu dem du wechseln musst, keine Schaltfläche, die du anklicken musst.
- **Feststellen für längere Texte.** Drück Shift, während du fn hältst, oder drück zweimal fn, und die Aufnahme läuft ohne gehaltene Taste weiter, bis zu fünf Minuten lang. Ein Druck auf fn oder Shift beendet sie.
- **Sprich einfach weiter.** Starte das nächste Diktat, während das letzte noch transkribiert wird; die Texte kommen in der Reihenfolge an, in der du sie gesprochen hast.

## Voraussetzungen

- macOS 14 Sonoma oder neuer. Lowspeak wird auf macOS 27 entwickelt und getestet.
- Ein Mac mit Apple Silicon oder einem Intel-Prozessor. Die lokale Transkription ist für Apple Silicon gebaut, wo Whisper auf der GPU und Parakeet auf der Neural Engine läuft; auf Intel-Macs laufen beide auf dem Prozessor, viel langsamer, siehe [Intel-Macs](#intel-macs).
- Etwa 17 MB für die App, dazu 75 MB bis 3 GB für jedes lokale Modell, das du lädst.
- Für die Onlinetranskription einen API-Schlüssel von [OpenAI](https://platform.openai.com/api-keys) oder [Groq](https://console.groq.com/keys). OpenAI berechnet, was du transkribierst; der kostenlose Plan von Groq kostet nichts, solange du unter seinen Tageslimits bleibst, und die Groq-Konsole zeigt sie dir an. Die lokale Transkription braucht kein Konto.

## Installation

**Vom Disk-Image.** Lade [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), öffne es und ziehe Lowspeak auf den Ordner „Programme“. Das Image ist mit einer Developer ID signiert und von Apple notarisiert, sodass es ohne Warnungen öffnet.

**Mit Homebrew.** Das Cask liegt in einem eigenen Tap:

```sh
brew install --cask sventmueller/tap/lowspeak
```

`brew uninstall --zap --cask lowspeak` entfernt zusätzlich die Einstellungen, Modelle, Logs und Caches von Lowspeak; deine API-Schlüssel bleiben im Schlüsselbund „Anmeldung“.

**Updates.** Lowspeak aus dem Disk-Image oder von Homebrew hält sich mit [Sparkle](https://sparkle-project.org) selbst aktuell. Beim zweiten Start fragt es, ob es einmal täglich selbstständig nach Updates suchen darf. **Nach Updates suchen …** im Menü sucht sofort; **Automatisch nach Updates suchen** und **Updates automatisch laden und installieren** direkt darunter ändern deine Wahl später. Findet eine Suche eine neue Version, während du in einer anderen App arbeitest, erscheint ein Punkt am Ring in der Menüleiste, und **Nach Updates suchen …** bringt das Update in den Vordergrund. Lowspeak installiert ein Update nur, wenn es die Signatur des Entwicklers trägt; deine Einstellungen, Schlüssel und Modelle bleiben erhalten. `brew upgrade --cask lowspeak` aktualisiert eine Kopie von Homebrew ebenfalls. Version 1.0.0 hat keinen Updater: Installiere einmalig eine neuere Version von der [Release-Seite](https://github.com/sventmueller/lowspeak/releases) darüber. Ein selbst gebautes Lowspeak hat ebenfalls keinen Updater.

**Aus dem Quellcode.** Die Voraussetzungen stehen unter [Bauen und Installieren](../../../docs/development.md#build--deploy), dann:

```sh
make deploy
```

Das baut Lowspeak, ersetzt die Kopie in deinem Ordner „Programme“ und startet sie.

**Bei der Anmeldung öffnen** im Menü startet Lowspeak mit deinem Mac, über die Anmeldeobjekte von macOS selbst. Es läuft immer nur eine Kopie; eine zweite beendet sich sofort.

## Erste Schritte

### Berechtigungen

Beim ersten Start fragt Lowspeak nach drei Berechtigungen:

- **Bedienungshilfen** (Systemeinstellungen › Datenschutz & Sicherheit › Bedienungshilfen; unter macOS 27 heißt die Liste Gerätesteuerung und Datenzugriff). Lowspeak braucht sie, um die fn-Taste zu bemerken und mit ⌘V einzufügen. macOS bietet an, die Liste zu öffnen; schalte dort den Schalter für Lowspeak ein. Die Meldungen von Lowspeak nennen die Liste so, wie dein macOS sie anzeigt.
- **Mikrofon.** Wird einmal abgefragt; ohne diese Berechtigung nimmt Lowspeak nichts auf.
- **Mitteilungen.** Lowspeak nutzt sie für Probleme, die du sonst nicht bemerken würdest, etwa ein stummes Mikrofon oder eine fehlgeschlagene Transkription.

### Art der Transkription wählen

Richte mindestens einen Weg ein; im Menü kannst du jederzeit wechseln.

- **Online:** Wähle **API-Schlüssel festlegen …** und füge einen Schlüssel von OpenAI (er beginnt mit `sk-`) oder Groq (`gsk_`) ein. Lowspeak erkennt den Anbieter am Schlüssel. Wähle dann ein Modell unter **OpenAI** oder **Groq**.
- **Auf deinem Mac:** Wähle **Modelle verwalten** › **Parakeet Ultra laden** oder ein Whisper-Modell und wähle es dann unter **Lokal** aus.

[Welches Modell zu dir passt](#welches-modell-zu-dir-passt) hilft bei der Wahl. Dann halte fn gedrückt, sag etwas und lass los.

### Das Menü

Klicke auf den Ring in der Menüleiste:

| Eintrag | Was er tut |
|---|---|
| **Letzte Diktate** | die letzten zehn Texte; ein Klick kopiert einen davon erneut in die Zwischenablage |
| **Lokal** | die Modelle auf deinem Mac, gruppiert nach Whisper und Parakeet; nicht geladene sind ausgegraut |
| **OpenAI**, **Groq** | die Onlinemodelle; ohne Schlüssel sind sie ausgegraut, und das Menü bietet **API-Schlüssel festlegen …** an |
| **Sprache** | die Sprache, in der du diktierst, oder **Automatisch erkennen**; das Menü folgt ihr |
| **Nachbearbeitung** | **Aus**, **Bereinigen** oder **Formatieren**, siehe [Nachbearbeitung](#nachbearbeitung) |
| **Vokabular …** | Namen und Begriffe, die genau so geschrieben werden sollen; das Menü zeigt, wie viele du eingetragen hast |
| **Mikrofon** | **Systemstandard** oder ein festes Mikrofon, dazu **Signalton, sobald bereit** |
| **Modelle verwalten** | lokale Modelle laden und löschen, mit Fortschrittsanzeige |
| **API-Schlüssel festlegen …** / **API-Schlüssel ändern …** | Schlüssel hinzufügen, ersetzen oder entfernen |
| **Bei der Anmeldung öffnen** | Lowspeak mit deinem Mac starten; das Häkchen zeigt, dass es eingeschaltet ist |
| **Nach Updates suchen …** | jetzt nach einer neuen Version suchen; die beiden Einträge darunter legen fest, ob Lowspeak einmal täglich sucht und Updates ohne Nachfrage installiert. Nur in der App aus dem Disk-Image |
| **Hilfe** | **Kurzreferenz**, **Problem melden …**, **Funktion vorschlagen …**, **Lowspeak auf GitHub** |
| **Über Lowspeak** | Version, Copyright, Projektseite und Lizenz |
| **Lowspeak beenden** | ⌘Q, solange das Menü offen ist |

Das Häkchen unter **Lokal**, **OpenAI** und **Groq** markiert das Modell, das in Gebrauch ist. Das Symbol zeigt, was Lowspeak gerade tut:

| Symbol | Bedeutung |
|---|---|
| Ring mit Punkt | Bereit |
| Gefüllter Kreis | Aufnahme, solange fn gehalten wird |
| Abgerundetes Quadrat | Aufnahme festgestellt; der nächste Druck auf fn oder Shift beendet sie |
| Blinkender Ring | Transkription läuft |
| Ring mit Punkt oben rechts | Eine neue Version wartet, siehe **Nach Updates suchen …** |
| ↓ 42 % | Ein Modell wird geladen |

Lowspeak hat kein Einstellungsfenster: Alles steht in diesem Menü, und es merkt sich jede Wahl.

### Kurzreferenz

**Hilfe › Kurzreferenz** zeigt die fn-Gesten in einem kleinen Fenster, in deiner Sprache, mit einer Schaltfläche, die die Tastatureinstellungen öffnet. **Problem melden …** öffnet ein Formular auf GitHub, in dem deine Versionen von Lowspeak und macOS schon eingetragen sind, **Funktion vorschlagen …** eine Diskussion für Ideen, **Lowspeak auf GitHub** die Projektseite.

## Diktieren

### Halten zum Sprechen

fn halten, sprechen, loslassen. Das Aufnahmesymbol erscheint in dem Moment, in dem dein Mikrofon Ton liefert; fang dann an zu sprechen, denn was du vorher sagst, wird nicht aufgenommen. Das Loslassen von fn beendet die Aufnahme, und der Text folgt einen Moment später.

Ein kurzes Tippen auf fn bewirkt nichts, sodass fn seine anderen Funktionen behält. Eine Aufnahme, die leer bleibt oder kürzer als 0,3 Sekunden ist, wird verworfen.

### Mit Shift oder Doppeldruck feststellen

Für längere Texte stellst du die Aufnahme fest, sodass sie ohne gehaltene Taste läuft. Dafür gibt es drei Wege:

- **Shift, während du fn hältst.** Beginne wie gewohnt mit fn und drücke während der Aufnahme Shift: Sie wird festgestellt, und du kannst fn loslassen. Nichts, was du vorher gesagt hast, geht verloren, die Aufnahme läuft einfach weiter.
- **Shift, dann fn.** Halte Shift gedrückt und drücke fn: Die Aufnahme startet festgestellt.
- **Zweimal fn.** Tippe kurz auf fn (kürzer als 0,3 Sekunden) und drücke innerhalb von 0,4 Sekunden erneut. Die Aufnahme läuft ab dem ersten Druck.

Das Symbol wird zu einem abgerundeten Quadrat. Drücke einmal **fn** oder **Shift**, um zu stoppen; der Text folgt wie gewohnt. Ist deine 🌐-Taste so eingestellt, dass sie „Emoji & Symbole“ öffnet, beende mit Shift: Ein kurzer Druck auf fn würde die Übersicht öffnen.

Eine Aufnahme stoppt von selbst nach zwei Minuten, solange fn gehalten wird, und nach fünf Minuten, wenn sie festgestellt ist, und wird dann wie gewohnt transkribiert.

### Wohin der Text geht

Lowspeak fügt den Text in die App ein, die im Vordergrund war, als du aufgehört hast zu sprechen, an der Cursorposition, über die Zwischenablage und ⌘V. Es sichert alles, was vorher in der Zwischenablage lag, und legt es zurück, eine halbe Sekunde nachdem die App den Text gelesen hat, oder nach zehn Sekunden, falls sie ihn nie liest, sofern nicht eine andere App die Zwischenablage in der Zwischenzeit geändert hat. Der Text ist als flüchtig gekennzeichnet, sodass Apps für den Zwischenablageverlauf, die diese Kennzeichnung beachten, ihn nicht aufzeichnen.

Du kannst das nächste Diktat starten, während das letzte noch transkribiert wird. Die Texte werden in der Reihenfolge eingefügt, in der du sie gesprochen hast, und nie, während du aufnimmst oder Shift hältst, sodass ein Einfügen nicht mitten in deinem nächsten Satz landen kann.

Ohne die Berechtigung für Bedienungshilfen, oder wenn das simulierte ⌘V nicht gesendet werden kann, bleibt der Text in der Zwischenablage, und eine Mitteilung sagt Bescheid; füge ihn dann selbst mit ⌘V ein.

### Letzte Diktate

**Letzte Diktate** im Menü listet die letzten zehn Texte auf, den neuesten zuerst, jeden auf eine Zeile gekürzt; hältst du den Zeiger darüber, siehst du den ganzen Text. Ein Klick kopiert ihn in die Zwischenablage, für den Fall, dass ein Text an der falschen Stelle gelandet ist oder du ihn zweimal brauchst. Die Liste liegt nur im Arbeitsspeicher und ist leer, nachdem Lowspeak beendet wurde.

## Transkription

### Welches Modell zu dir passt

| Du willst | Wähle | Warum |
|---|---|---|
| **Tempo, ohne deinen Mac zu belasten** | **Groq › Whisper Large V3 Turbo** | die schnellste Onlineoption; nichts wird geladen, und dein Mac behält seinen Arbeitsspeicher und Speicherplatz |
| **Das beste Ergebnis** | **OpenAI › GPT Transcribe** (Standard) | das genaueste der Modelle, die Lowspeak anbietet; nimmt dein Vokabular als Schlüsselwörter |
| **Offline und privat** | **Lokal › Parakeet Ultra** für europäische Sprachen, **Lokal › Whisper Large v3 Turbo** für alle anderen | funktioniert ohne Netzwerk, und deine Stimme verlässt nie deinen Mac; Parakeet ist das schnellste von allen |
| **Kein Konto, keine Kosten** | ein beliebiges Modell unter **Lokal** | läuft auf deinem Mac; nichts wird berechnet |
| **Online, aber kostenlos** | **Groq** im kostenlosen Plan | ein Groq-Konto ohne Zahlungsmethode; die Transkription kostet innerhalb der Tageslimits von Groq nichts |
| **Zuverlässigkeit und Tempo zugleich** | ein Onlinemodell, dazu Parakeet Ultra oder ein Whisper-Modell installiert | das lokale Modell springt ein, wenn der Onlinedienst ausfällt |
| **Auf einem Intel-Mac diktieren** | **Groq** oder **OpenAI** | lokale Modelle laufen dort auf dem Prozessor, siehe [Intel-Macs](#intel-macs) |

Lokale Modelle belegen ihre Downloadgröße auf dem Datenträger und, solange sie ausgewählt sind, etwa ebenso viel Arbeitsspeicher; Lowspeak hält das Modell geladen, damit das Diktieren ohne Verzögerung beginnt. Auf einem Mac mit 8 GB Arbeitsspeicher sind Parakeet Ultra oder ein Onlinemodell den großen Whisper-Modellen vorzuziehen.

### Online: OpenAI und Groq

| Modell | Anbieter | Hinweise |
|---|---|---|
| **GPT Transcribe** (Standard) | OpenAI | am genauesten; nimmt dein Vokabular als Schlüsselwörter |
| **Whisper** | OpenAI | das ursprüngliche Whisper-Modell |
| **Whisper Large V3** | Groq | sehr schnell |
| **Whisper Large V3 Turbo** | Groq | online am schnellsten |

Die Aufnahme wird vor dem Hochladen nach AAC komprimiert, etwa 6 KB pro Sekunde Sprache, sodass das Diktieren auch über langsame Verbindungen funktioniert. Während du fn hältst, öffnet Lowspeak bereits die Verbindung zum Anbieter, sodass das Hochladen in dem Moment beginnt, in dem du loslässt.

### Auf deinem Mac: Whisper und Parakeet

| Modell | Größe | Läuft auf | Sprachen |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 europäische Sprachen |
| **Parakeet v2** | ~600 MB | Neural Engine | Englisch |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | etwa 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | etwa 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | etwa 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | etwa 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | etwa 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | etwa 99 |

**Modelle verwalten** lädt und löscht sie; während eines Downloads zeigt die Menüleiste den Fortschritt, und das Beenden von Lowspeak bricht ihn ab. Die allererste Verwendung eines Modells bereitet es für deinen Mac vor: etwa 10 Sekunden bei Parakeet Ultra, bis zu 30 Sekunden bei Whisper; danach lädt es in Sekundenbruchteilen. Wird das gewählte Modell gelöscht, wechselt Lowspeak zu einem anderen installierten.

**Parakeet** ist die schnellste Wahl: Es transkribiert 40 Sekunden Sprache in etwa 0,3 Sekunden. Lowspeak betreibt es mit denselben Modellen und derselben Dekodierung wie [FluidAudio](https://github.com/FluidInference/FluidAudio) und legt die Modelle im Ordner von FluidAudio ab (`~/Library/Application Support/FluidAudio/Models`), sodass Apps, die auf FluidAudio aufbauen, sie mitnutzen. Aufnahmen über 15 Sekunden werden in überlappenden Fenstern transkribiert, die zusammengefügt werden, ohne an den Nahtstellen Wörter zu verlieren. **Whisper** versteht mehr Sprachen und liest dein Vokabular; die großen Modelle sind am genauesten, die kleinen außerhalb des Englischen deutlich schwächer. Whisper-Modelle liegen in `~/Library/Application Support/Lowspeak/Models`.

### Wenn der Onlinedienst ausfällt

Schlägt eine Onlinetranskription fehl, weil das Netzwerk ausgefallen ist, der Anbieter ein Problem hat oder der Schlüssel falsch ist, transkribiert ein installiertes lokales Modell dieselbe Aufnahme: das unter **Lokal** gewählte, sonst Parakeet Ultra, dann Whisper Large v3 Turbo, Large v3 und die kleineren Whisper-Modelle, zuletzt Parakeet v2. Eine Mitteilung informiert dich einmal darüber; sie kommt erst wieder, nachdem zwischendurch ein Onlinediktat gelungen ist.

Mit einem lokalen Modell als Rückfallebene gibt Lowspeak der Onlineanfrage 20 Sekunden; ohne versucht es dreimal, jeweils bis zu einer Minute lang. Im Onlinemodus bereitet Lowspeak das Parakeet-Modell für den Ausfall einmal beim Start vor, sodass die erste Ausweichtranskription nicht darauf warten muss.

### Wie die Aufnahme vorbereitet wird

Lowspeak nimmt mit der eigenen Abtastrate deines Mikrofons auf und wandelt nach dem Stoppen in die 16 kHz um, die die Modelle erwarten. Vor der Transkription entfernt es tiefes Rumpeln unter 80 Hz, schneidet Stille am Anfang und Ende ab und gleicht die Lautstärke aus, sodass leise und laute Sprecher gleich ankommen. Eine Aufnahme ohne Sprache wird gar nicht erst gesendet. Im lokalen Modus bekommt Parakeet zuerst die unbearbeitete Aufnahme und die vorbereitete nur, wenn dabei kein Text herauskommt.

### Sprachen je Modell

Die Sprache unter **Sprache** wird an die Onlinemodelle und an Whisper übergeben. Mit **Automatisch erkennen** ermittelt das Modell die Sprache selbst.

| Modell | Sprachen, die du diktieren kannst |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, auf deinem Mac) | etwa 99, darunter alle 35 des Menüs |
| Parakeet Ultra | Bulgarisch, Kroatisch, Tschechisch, Dänisch, Niederländisch, Englisch, Estnisch, Finnisch, Französisch, Deutsch, Griechisch, Ungarisch, Italienisch, Lettisch, Litauisch, Maltesisch, Polnisch, Portugiesisch, Rumänisch, Russisch, Slowakisch, Slowenisch, Spanisch, Schwedisch, Ukrainisch |
| Parakeet v2 | Englisch |

Parakeet erkennt die Sprache selbst und ignoriert die Auswahl im Menü. **Chinesisch (traditionell)** weist GPT Transcribe an, traditionelle Schriftzeichen zu schreiben; Whisper schreibt Chinesisch in beiden Schriften und Portugiesisch ohne Rücksicht auf die Variante.

### Intel-Macs

Lowspeak ist eine Universal-App und läuft auch auf Intel-Macs mit macOS 14. Alles, was online läuft, funktioniert genauso. Den lokalen Modellen fehlt dort die Hardware, für die sie gebaut sind: Whisper läuft auf dem Prozessor statt auf der GPU und Parakeet auf dem Prozessor statt auf der Neural Engine. Beide funktionieren, brauchen aber ein Vielfaches der Zeit wie auf Apple Silicon, die großen Whisper-Modelle viel zu lange zum Diktieren. Transkribiere auf einem Intel-Mac online und behalte Whisper Small oder Parakeet Ultra als [Rückfallebene](#wenn-der-onlinedienst-ausfällt).

## Vokabular

**Vokabular …** nimmt die Namen und Begriffe auf, die Lowspeak genau so schreiben soll, durch Kommas getrennt: Produktnamen, Kollegen, Fachbegriffe. Bis zu 100 Begriffe mit je bis zu 60 Zeichen; `<` und `>` werden entfernt.

- **GPT Transcribe** erhält sie als Schlüsselwörter.
- **Whisper** (OpenAI, Groq und auf deinem Mac) liest sie als Hinweis vor der Aufnahme, bis zu 500 Zeichen.
- **Die Nachbearbeitung** wird angewiesen, sie genau so zu schreiben, wie sie dastehen.
- **Parakeet** verwendet sie nicht.

## Nachbearbeitung

Nach der Transkription kann ein Sprachmodell den Text überarbeiten. Wähle unter **Nachbearbeitung**:

- **Aus** (Standard) – der Text, wie er transkribiert wurde.
- **Bereinigen** – entfernt Füllwörter wie „äh“, „sozusagen“ oder „halt“, korrigiert Grammatik und Zeichensetzung und behält nur die Korrektur, wenn du dich selbst verbesserst. Es fügt nichts hinzu und ändert nicht den Sinn.
- **Formatieren** – wie Bereinigen und passt den Text zusätzlich an die App an, in die du diktierst: Begriffe im Codestil in Code-Editoren, Befehlszeilen in Terminals, ein lockerer Ton in Chat-Apps, ein professioneller in E-Mails.

Die Nachbearbeitung nutzt Groq (`openai/gpt-oss-120b`), wenn du einen Groq-Schlüssel hast, sonst OpenAI (`gpt-4o-mini`), auch wenn du lokal transkribierst. Ist unter **Sprache** eine Sprache gewählt, bleibt sie bei dieser Sprache. Dauert sie länger als acht Sekunden, schlägt sie fehl oder liefert sie eine Antwort, die abgeschnitten oder weit länger ist als das, was du gesagt hast, fügt Lowspeak den unbearbeiteten Text ein. Lehnt der Anbieter die Anfrage ab oder ist kein Schlüssel festgelegt, sagt eine Mitteilung das einmal.

## Mikrofon

Wähle unter **Mikrofon** ein festes Mikrofon oder **Systemstandard**. Lowspeak merkt sich das Gerät; solange es nicht angeschlossen ist, zeigt das Menü es als **(nicht verbunden)** an, und Lowspeak nimmt mit dem Systemstandard auf.

Das Mikrofon wird nur eingeschaltet, während du diktierst. Bis es bereit ist, dauert es etwa 0,07 Sekunden mit dem eingebauten Mikrofon, etwa eine halbe Sekunde mit einem Studio Display und ein bis zwei Sekunden mit AirPods, die zuerst in ihren Headset-Modus wechseln. Das Aufnahmesymbol erscheint, sobald Ton ankommt; **Signalton, sobald bereit** spielt in diesem Moment zusätzlich einen kurzen Ton. Er ist standardmäßig aus, weil der Ton auf manchen Lautsprechern selbst eine halbe Sekunde zu spät kommt.

Liefert ein Mikrofon fünf Sekunden lang nur Stille, wie AirPods, wenn ein anderes Gerät ihr Mikrofon belegt, sagt Lowspeak dir Bescheid.

## API-Schlüssel

**API-Schlüssel festlegen …** fragt nach einem Schlüssel und erkennt den Anbieter an dessen Anfang: `sk-` für OpenAI, `gsk_` für Groq. Ein Schlüssel in einem anderen Format wird mit einer Erklärung abgelehnt. Gib einen Schlüssel pro Anbieter ein; mit Schlüsseln für beide stehen alle Onlinemodelle zur Verfügung. **API-Schlüssel ändern …** ersetzt einen Schlüssel oder löscht sie mit **Alle Schlüssel entfernen**.

Die Schlüssel liegen in deinem Schlüsselbund „Anmeldung“ unter `com.lowspeak.app`. Eine Schlüsseldatei einer früheren Version (`~/Library/Application Support/Lowspeak/.api-key`) wird beim nächsten Start dorthin verschoben und gelöscht, sobald sich jeder Schlüssel wieder aus dem Schlüsselbund lesen lässt.

## Mitteilungen

Lowspeak arbeitet im Hintergrund und meldet Probleme deshalb als Mitteilungen:

| Mitteilung | Was passiert ist |
|---|---|
| **Aufnahme fehlgeschlagen** | kein Zugriff auf das Mikrofon, kein Mikrofon angeschlossen, oder das Mikrofon konnte nicht starten |
| **Mikrofon stumm** | fünf Sekunden Stille, siehe [Mikrofon](#mikrofon) |
| **Zugriff auf Bedienungshilfen erforderlich** | der Text liegt in der Zwischenablage; erlaube den Zugriff auf Bedienungshilfen, damit automatisch eingefügt wird |
| **Einfügen fehlgeschlagen** | der Text liegt in der Zwischenablage; füge ihn mit ⌘V ein |
| **Transkription fehlgeschlagen** | mit dem Grund: kein Schlüssel, abgelehnter Schlüssel, Anfragelimit, Server- oder Netzwerkfehler oder kein lokales Modell |
| **Auf diesem Mac transkribiert** | der Onlinedienst ist ausgefallen, und ein lokales Modell hat übernommen |
| **Nachbearbeitung fehlgeschlagen** | der Anbieter lehnt die Nachbearbeitung ab, oder es ist kein Schlüssel festgelegt; der Text wurde ohne sie eingefügt |
| **Kein API-Schlüssel** | du hast ein Onlinemodell gewählt, ohne einen Schlüssel für seinen Anbieter zu haben |
| **Download fehlgeschlagen**, **Löschen fehlgeschlagen**, **Löschen nicht möglich** | das Verwalten eines lokalen Modells hat nicht funktioniert, oder ein anderer Vorgang lief noch |
| **Anmeldeobjekt nicht geändert**, **Fehler beim API-Schlüssel** | macOS oder der Schlüsselbund hat die Änderung abgelehnt |

Mitteilungen enthalten nie deinen diktierten Text. Ein ausfallender Onlinedienst und ein Anbieter, der die Nachbearbeitung ablehnt, werden einmal gemeldet, nicht nach jedem Diktat, bis sie wieder funktionieren.

## Darstellung und Bedienungshilfen

- **Das Symbol in der Menüleiste** ist ein Vorlagenbild, das sich einer hellen, dunklen oder getönten Menüleiste anpasst.
- **VoiceOver** liest das Symbol als „Lowspeak“ vor, und Menü, Hinweise und Mitteilungen sind Standardelemente von macOS.
- **Keine Taste muss gehalten werden.** Stell eine Aufnahme mit Shift oder einem Doppeldruck auf fn fest, und der nächste Druck beendet sie.
- **Die Kurzreferenz** ist ein schlichtes Fenster mit Beschriftungen und einer Schaltfläche, per Tastatur erreichbar; Esc schließt es.

Bekannte Lücken stehen in [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver kündigt nicht an, wenn eine Aufnahme beginnt, und Lowspeak wurde noch nicht mit VoiceOver oder der Schaltersteuerung geprüft.

## Sprachen

Menü, Dialoge und Mitteilungen sind in 35 Sprachen übersetzt:

Arabisch, Katalanisch, Chinesisch (vereinfacht), Chinesisch (traditionell), Kroatisch, Tschechisch, Dänisch, Niederländisch, Englisch, Finnisch, Französisch, Deutsch, Griechisch, Hebräisch, Hindi, Ungarisch, Indonesisch, Italienisch, Japanisch, Koreanisch, Malaiisch, Norwegisch, Polnisch, Portugiesisch (Brasilien), Portugiesisch (Portugal), Rumänisch, Russisch, Slowakisch, Slowenisch, Spanisch, Schwedisch, Thai, Türkisch, Ukrainisch und Vietnamesisch.

Die Sprache, die du unter **Sprache** wählst, ist auch die Sprache des Menüs. Mit **Automatisch erkennen** folgt das Menü der Sprachreihenfolge unter **Systemeinstellungen › Allgemein › Sprache & Region** und nimmt Englisch, wo es keine Übersetzung gibt. Die Menübegriffe folgen den Übersetzungen von Apple, sodass Lowspeak sich liest wie der Rest deines Mac. Abgesehen vom Englischen sind die Übersetzungen maschinell erstellt und noch nicht von Muttersprachlern geprüft; Korrekturen sind als [Issues](https://github.com/sventmueller/lowspeak/issues/new/choose) willkommen.

## Tasten und Gesten

| Aktion | Tasten |
|---|---|
| Diktieren, solange gehalten | fn halten |
| Eine laufende Aufnahme feststellen | Shift, während fn gehalten wird |
| Eine festgestellte Aufnahme starten | Shift+fn oder zweimal schnell fn |
| Eine festgestellte Aufnahme beenden | fn oder Shift |
| Lowspeak beenden (Menü offen) | ⌘Q |
| Die Kurzreferenz schließen | Esc |

Lowspeak hält die fn-Drücke, die es nutzt, von anderen Apps fern; Shift wird wie gewohnt durchgereicht.

## Datenschutz und Sicherheit

Lowspeak hört, was du sagst, während fn gehalten wird, kann in jede App tippen und verwahrt deine API-Schlüssel, deshalb tut es damit so wenig wie möglich.

- **Deine Stimme geht an genau einen Ort.** Im Onlinemodus geht die Aufnahme komprimiert an den Anbieter, den du gewählt hast, OpenAI oder Groq, zusammen mit deinem Vokabular. Im lokalen Modus verlässt nichts deinen Mac. Schlägt eine Onlineanfrage fehl, transkribiert ein lokales Modell dieselbe Aufnahme auf deinem Mac.
- **Die Nachbearbeitung** sendet, wenn du sie einschaltest, den transkribierten Text und dein Vokabular an Groq oder OpenAI. Im Modus „Formatieren“ kommt die Art der App hinzu, in die du diktierst (Code-Editor, Terminal, Chat oder E-Mail); der Name der App wird nicht gesendet.
- **Keine Telemetrie, kein Konto.** Abgesehen von Transkription und Nachbearbeitung geht Lowspeak nur online, um die Verbindung zu deinem Anbieter zu öffnen, während fn gehalten wird, im Onlinemodus oder mit eingeschalteter Nachbearbeitung, um ein Modell zu laden, wenn du darum bittest, von Hugging Face über HTTPS, in einer festen Version, wobei jede Datei mit ihrer Prüfsumme abgeglichen wird, und um auf GitHub nach einer neuen Version zu suchen, einmal täglich, wenn du es erlaubst. Diese Anfrage enthält die Version von Lowspeak und sonst nichts. **Hilfe › Problem melden …** öffnet in deinem Browser ein Formular auf GitHub; seine Adresse enthält die Version von Lowspeak und die von macOS, nichts von dem, was du diktiert hast.
- **API-Schlüssel** bleiben in deinem Schlüsselbund „Anmeldung“. Sie werden nur an ihren eigenen Anbieter gesendet und aus Fehlermeldungen entfernt, bevor etwas ins Log geschrieben wird.
- **Diktierter Text wird nicht gespeichert.** Die letzten zehn Diktate bleiben für **Letzte Diktate** im Arbeitsspeicher und sind weg, wenn Lowspeak beendet wird. Die Logdateien halten Längen und Zeiten fest, nie den Text.
- **Die Zwischenablage** wird vor dem Einfügen gesichert und wiederhergestellt, sobald die App den Text gelesen hat, sofern nicht eine andere App sie zwischendurch geändert hat. Das Diktat ist als flüchtig gekennzeichnet, sodass Apps für den Zwischenablageverlauf, die diese Kennzeichnung beachten, es überspringen.
- **Codesignatur** mit der Hardened Runtime; die App fragt nur nach Mikrofon, Bedienungshilfen, Mitteilungen und dem Netzwerk. Ein Update muss die Signatur des Entwicklers tragen, sonst lehnt Sparkle es ab.

Um eine Sicherheitslücke zu melden, folge der [Sicherheitsrichtlinie](../../../SECURITY.md).

## Geschwindigkeit

Gemessen auf Apple Silicon:

- **Parakeet Ultra** transkribiert 40 Sekunden Sprache in etwa 0,3 Sekunden.
- **Online:** Die erste Anfrage nach einer Pause dauert bei Groq etwa 0,4 Sekunden, weil Lowspeak die Verbindung öffnet, während du noch sprichst; spätere Anfragen nutzen sie wieder, über HTTP/3, wo der Anbieter es anbietet.
- **Upload-Größe:** etwa 6 KB pro Sekunde Sprache, als AAC mit 48 kbit/s.
- **Bis das Mikrofon bereit ist:** etwa 0,07 Sekunden eingebaut, eine halbe Sekunde an einem Studio Display, ein bis zwei Sekunden mit AirPods.

## Bekannte Einschränkungen

- **Lokale Modelle auf Intel-Macs** laufen auf dem Prozessor und sind viel langsamer; dort ist die Onlinetranskription die bessere Wahl.
- **Was du sagst, bevor das Mikrofon bereit ist, geht verloren.** Warte auf das Aufnahmesymbol, besonders mit AirPods.
- **Die fn-Taste kann von anderen belegt werden.** Wenn die Einstellung der 🌐-Taste in macOS, Warp oder die Diktierfunktion von macOS fn zuerst beanspruchen, sieht Lowspeak sie nicht; siehe [Fehlerbehebung](#fehlerbehebung).
- **Parakeet** ignoriert das Vokabular und die Sprachwahl und kennt nur europäische Sprachen.
- **Schrift und Variante:** Nur GPT Transcribe lässt sich anweisen, traditionelles Chinesisch zu schreiben; kein Modell lässt sich anweisen, europäisches statt brasilianisches Portugiesisch zu schreiben.
- **Zeitlimits:** zwei Minuten, solange fn gehalten wird, fünf Minuten, wenn die Aufnahme festgestellt ist.
- **Mikrofonmodus:** Das Kontrollzentrum bietet für Lowspeak Stimmisolation und Breites Spektrum an und meldet sie als aktiv, aber Lowspeak nimmt ohne Sprachverarbeitung auf, sodass die Wahl keine Wirkung hat.
- **Letzte Diktate** liegen nur im Arbeitsspeicher und sind nach dem Beenden weg.
- **Gewollt:** kein Einstellungsfenster, kein Dock-Symbol, nicht im App Store.

## Fehlerbehebung

**Nichts passiert, wenn ich fn halte.** Prüfe drei Dinge: Lowspeak ist unter **Systemeinstellungen › Datenschutz & Sicherheit › Bedienungshilfen** (unter macOS 27: **Gerätesteuerung und Datenzugriff**) erlaubt; **Systemeinstellungen › Tastatur › Beim Drücken der 🌐-Taste** steht auf **Keine Aktion**; und keine andere App beansprucht fn. Warp tut das standardmäßig; schalte es mit `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` ab und starte Warp neu. Auch die Diktierfunktion von macOS kann so eingestellt sein, dass sie mit fn startet.

**Nach einem Update reagiert Lowspeak nicht mehr auf fn.** macOS bindet die Berechtigung für Bedienungshilfen an die Signatur der App. Hat sich die Signatur geändert, zum Beispiel von einem eigenen Build zur geladenen App, entferne Lowspeak aus der Liste der Bedienungshilfen (unter macOS 27: Gerätesteuerung und Datenzugriff) und füge es erneut hinzu.

**Die ersten Wörter fehlen.** Fang an zu sprechen, sobald das Aufnahmesymbol erscheint. Mit AirPods braucht das Mikrofon ein bis zwei Sekunden; ein eingebautes oder USB-Mikrofon unter **Mikrofon** ist fast sofort bereit.

**„Mikrofon stumm“.** Das Mikrofon hat fünf Sekunden lang keinen Ton geliefert. Bei AirPods nutzt möglicherweise ein anderes Gerät ihr Mikrofon; wechsle zum eingebauten Mikrofon oder verbinde sie neu. Ein geschlossener MacBook-Deckel schaltet das eingebaute Mikrofon ebenfalls stumm.

**Diktate kommen leer an.** Die Eingangslautstärke ist vielleicht zu niedrig: Apps wie Teams, Zoom oder Meet ändern sie während Anrufen, und macOS speichert sie pro Gerät. Erhöhe sie unter **Systemeinstellungen › Ton › Eingabe**.

**Der Text ist an der falschen Stelle gelandet.** **Letzte Diktate** im Menü kopiert ihn erneut in die Zwischenablage.

**„Kein API-Schlüssel“.** Wähle im Menü **API-Schlüssel festlegen …** oder wechsle zu einem lokalen Modell.

**Die erste lokale Transkription dauert lange.** Jedes Modell wird bei seiner ersten Verwendung für deinen Mac vorbereitet; danach lädt es in Sekundenbruchteilen.

**Lowspeak beendet sich direkt nach dem Start.** Eine andere Kopie von Lowspeak läuft bereits; beende zuerst diese.

**Es erscheinen keine Mitteilungen.** Erlaube sie unter **Systemeinstellungen › Mitteilungen › Lowspeak**.

**Etwas anderes.** Die Logs in `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) beschreiben, was passiert ist, ohne deinen diktierten Text. Hänge die relevanten Zeilen an einen [Problembericht](https://github.com/sventmueller/lowspeak/issues/new/choose) an.

## Für Entwickler

Bauen, Testen, der Aufbau des Codes und die Regeln, denen er folgt, stehen in [docs/development.md](../../../docs/development.md); wie du beitragen kannst, steht in [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Komponenten von Drittanbietern

| Komponente | Version | Verwendet für | Lizenz |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) via [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | lokales Whisper auf der GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet-Dekodierung und Zusammenführen langer Aufnahmen, nach Rust übertragen in `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) und seine Framework-Crates | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation und andere Systemframeworks | MIT; Zlib, Apache-2.0 oder MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | Mikrofonaufnahme über CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | Umrechnung auf 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) mit rustls | 3.5 | Modell-Downloads | MIT oder Apache-2.0; rustls und ring unter ISC und Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | Prüfsummen der Modell-Downloads | MIT oder Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | API-Schlüssel im Schlüsselbund | MIT oder Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, Fehler, das Vokabular von Parakeet | MIT oder Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | Updates der App aus dem Disk-Image | MIT, mit Teilen unter BSD- und zlib-Lizenzen |

Die Lizenztexte von allem, was in der App steckt, liegen ihr als `Contents/Resources/Acknowledgements.txt` bei.

Die Modelle sind nicht Teil von Lowspeak; es lädt sie, wenn du darum bittest:

| Modell | Quelle | Lizenz |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper von OpenAI im ggml-Format) | MIT |
| Parakeet v2, Parakeet Ultra | CoreML-Konvertierungen von NVIDIAs Parakeet TDT durch [FluidInference](https://huggingface.co/FluidInference) | CC-BY-4.0 |

## Lizenz

Lowspeak ist unter einer der folgenden Lizenzen lizenziert

- Apache-Lizenz, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- MIT-Lizenz ([LICENSE-MIT](../../../LICENSE-MIT))

nach deiner Wahl.

Sofern du nicht ausdrücklich etwas anderes erklärst, wird jeder Beitrag, den du absichtlich zur Aufnahme in Lowspeak einreichst, wie in der Apache-2.0-Lizenz definiert, unter den obigen Lizenzen doppelt lizenziert, ohne zusätzliche Bedingungen.
