<p><img src="assets/readme-icon.png" alt="Lowspeak app icon" width="168" height="168"></p>

# Lowspeak

**Hold fn, talk, let go. Your words are already there.**

Lowspeak lives in your menu bar and listens only while you hold the fn key. Say what you want to write, let go, and the
text appears in whatever app you are in: mail, chat, code editor, terminal.

It is a real Mac app, written in Rust on AppKit, with no browser engine underneath. It starts at once and stays out of
your way. It even opens the connection while you are still speaking, so online text arrives about 0.4 seconds after you
let go.

Choose from twelve speech models. With your own Groq key, online transcription costs nothing within Groq's daily
allowance; OpenAI's GPT Transcribe takes on the difficult recordings. Lowspeak talks to them directly, with no account
and no server of its own in between.

Or keep everything on your Mac. Whisper and NVIDIA Parakeet work offline, so your voice never leaves the machine, and
Parakeet turns 40 seconds of speech into text in about a third of a second. If you work online, an installed local model
steps in when the network drops, so no dictation is lost.

Lowspeak speaks 35 languages. It learns the names and terms you use, can strip out filler words and tidy the grammar,
and keeps your last ten dictations in the menu in case you need one again.

[![Download Lowspeak for macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Download Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: signed and notarized by Apple, for macOS 14 or later on Apple Silicon and Intel.

**Found a bug?** [Open an issue](https://github.com/sventmueller/lowspeak/issues/new/choose), or choose **Help › Report
Problem…** in Lowspeak's menu: the form opens with your version filled in. If Lowspeak is useful to you, a
[star on GitHub](https://github.com/sventmueller/lowspeak) helps others find it.

[![Rust 2024 edition](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 or later](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon and Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit via objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp on Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet on the Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online with OpenAI or Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 interface languages](https://img.shields.io/badge/languages-35-4C9A2A)
![Signed with Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#license)

[Bahasa Indonesia](assets/readme/id/README.md) ·
[Bahasa Melayu](assets/readme/ms/README.md) ·
[Català](assets/readme/ca/README.md) ·
[Čeština](assets/readme/cs/README.md) ·
[Dansk](assets/readme/da/README.md) ·
[Deutsch](assets/readme/de/README.md) ·
**English** ·
[Español](assets/readme/es/README.md) ·
[Français](assets/readme/fr/README.md) ·
[Hrvatski](assets/readme/hr/README.md) ·
[Italiano](assets/readme/it/README.md) ·
[Magyar](assets/readme/hu/README.md) ·
[Nederlands](assets/readme/nl/README.md) ·
[Norsk](assets/readme/no/README.md) ·
[Polski](assets/readme/pl/README.md) ·
[Português (Brasil)](assets/readme/pt_BR/README.md) ·
[Português (Portugal)](assets/readme/pt_PT/README.md) ·
[Română](assets/readme/ro/README.md) ·
[Slovenčina](assets/readme/sk/README.md) ·
[Slovenščina](assets/readme/sl/README.md) ·
[Suomi](assets/readme/fi/README.md) ·
[Svenska](assets/readme/sv/README.md) ·
[Tiếng Việt](assets/readme/vi/README.md) ·
[Türkçe](assets/readme/tr/README.md) ·
[Ελληνικά](assets/readme/el/README.md) ·
[Русский](assets/readme/ru/README.md) ·
[Українська](assets/readme/uk/README.md) ·
[עברית](assets/readme/he/README.md)‎ ·
[العربية](assets/readme/ar/README.md)‎ ·
[हिन्दी](assets/readme/hi/README.md) ·
[ไทย](assets/readme/th/README.md) ·
[한국어](assets/readme/ko/README.md) ·
[日本語](assets/readme/ja/README.md) ·
[简体中文](assets/readme/zh-Hans/README.md) ·
[繁體中文](assets/readme/zh-Hant/README.md)

## Contents

- [Features at a glance](#features-at-a-glance)
- [Requirements](#requirements)
- [Installation](#installation)
- [Getting started](#getting-started)
  - [Permissions](#permissions)
  - [Choose how to transcribe](#choose-how-to-transcribe)
  - [The menu](#the-menu)
  - [Quick reference](#quick-reference)
- [Dictating](#dictating)
  - [Hold to talk](#hold-to-talk)
  - [Lock with Shift or a double press](#lock-with-shift-or-a-double-press)
  - [Where the text goes](#where-the-text-goes)
  - [Recent dictations](#recent-dictations)
- [Transcription](#transcription)
  - [Which model is right for you](#which-model-is-right-for-you)
  - [Online: OpenAI and Groq](#online-openai-and-groq)
  - [On your Mac: Whisper and Parakeet](#on-your-mac-whisper-and-parakeet)
  - [When the online service fails](#when-the-online-service-fails)
  - [How the recording is prepared](#how-the-recording-is-prepared)
  - [Languages per model](#languages-per-model)
  - [Intel Macs](#intel-macs)
- [Vocabulary](#vocabulary)
- [Post-processing](#post-processing)
- [Microphone](#microphone)
- [API keys](#api-keys)
- [Notifications](#notifications)
- [Appearance and accessibility](#appearance-and-accessibility)
- [Languages](#languages)
- [Keys and gestures](#keys-and-gestures)
- [Privacy and security](#privacy-and-security)
- [Speed](#speed)
- [Known limitations](#known-limitations)
- [Troubleshooting](#troubleshooting)
- [For developers](#for-developers)
  - [Third-party components](#third-party-components)
- [License](#license)

## Features at a glance

- **Native to the bone.** A real menu bar item with an AppKit menu, standard alerts and notifications. No Electron, no
  window, no Dock icon.
- **The right model for the job.** Groq for speed without using your Mac's memory, OpenAI's GPT Transcribe for the best
  result, Whisper or NVIDIA Parakeet on your Mac for working offline and keeping your voice private. Switch with one
  click.
- **Private.** No account, no telemetry. Audio goes only to the provider you chose, or nowhere at all in local mode.
- **No dictation lost.** When the network or the online service fails, an installed local model transcribes the same
  recording on your Mac.
- **In your language.** Dictation works in dozens of languages, and the menu, dialogs and notifications come in 35.
- **Your words, spelled right.** A vocabulary of names and technical terms guides OpenAI, Groq, Whisper and
  post-processing.
- **Clean text if you want it.** Optional post-processing removes filler words, fixes grammar and keeps only your
  correction when you correct yourself, or also formats the text for the kind of app you dictate into.
- **Your clipboard stays yours.** Lowspeak pastes through the clipboard and puts back what was on it a moment later. The
  last ten dictations stay in the menu to copy again.
- **Hold fn, speak, release.** The text is typed into the app in front, wherever the cursor is: mail, chat, editor,
  terminal or browser. No window to switch to, no button to click.
- **Lock it for longer texts.** Press Shift while you hold fn, or press fn twice, and the recording keeps running
  without a key held, for up to five minutes. One press on fn or Shift ends it.
- **Keep talking.** Start the next dictation while the last one is still being transcribed; the texts arrive in the order
  you spoke them.

## Requirements

- macOS 14 Sonoma or later. Lowspeak is developed and tested on macOS 27.
- A Mac with Apple Silicon or an Intel processor. Local transcription is built for Apple Silicon, where Whisper runs
  on the GPU and Parakeet on the Neural Engine; on Intel Macs both run on the processor, much more slowly, see
  [Intel Macs](#intel-macs).
- About 17 MB for the app, plus 75 MB to 3 GB for each local model you download.
- For online transcription, an API key from [OpenAI](https://platform.openai.com/api-keys) or
  [Groq](https://console.groq.com/keys). OpenAI charges for what you transcribe; Groq's free plan costs nothing within
  its daily limits, which the Groq console shows. Local transcription needs no account.

## Installation

**From the disk image.** Download [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg),
open it and drag Lowspeak onto the Applications folder. The image is signed with a Developer ID and notarized by
Apple, so it opens without warnings.

**With Homebrew.** The cask lives in a tap of its own:

```sh
brew install --cask sventmueller/tap/lowspeak
```

`brew uninstall --zap --cask lowspeak` also removes Lowspeak's settings, models, logs and caches; your API keys stay in
the login keychain.

**Updates.** Lowspeak from the disk image or Homebrew keeps itself up to date with
[Sparkle](https://sparkle-project.org). On the second launch it asks whether it may look for updates on its own, once a
day. **Check for Updates…** in the menu looks right away; **Check for Updates Automatically** and **Automatically
Download and Install Updates** below it change your choice later. When a check finds a new version while you work in
another app, a dot appears on the ring in the menu bar, and **Check for Updates…** brings the update to the front.
Lowspeak installs an update only if it carries the developer's signature; your settings, keys and models stay.
`brew upgrade --cask lowspeak` updates a copy from Homebrew as well. Version 1.0.0 has no updater: install a newer
version over it once, from the [releases page](https://github.com/sventmueller/lowspeak/releases). A build of your own
has no updater either.

**From source.** See [Build & Deploy](docs/development.md#build--deploy) for the prerequisites, then:

```sh
make deploy
```

This builds Lowspeak, replaces the copy in your Applications folder and launches it.

**Open at Login** in the menu starts Lowspeak with your Mac, through macOS's own login items. Only one copy runs at a
time; a second one quits right away.

## Getting started

### Permissions

On the first launch, Lowspeak asks for three permissions:

- **Accessibility** (System Settings › Privacy & Security › Accessibility; on macOS 27 the list is called Device Control
  and Data Access). Lowspeak needs it to notice the fn key and to paste with ⌘V. macOS offers to open the list; turn on
  the switch for Lowspeak. Lowspeak's own messages name the list as your macOS shows it.
- **Microphone.** Asked once; without it Lowspeak records nothing.
- **Notifications.** Lowspeak uses them for problems you would otherwise not see, such as a silent microphone or a
  failed transcription.

### Choose how to transcribe

Set up at least one way; you can switch at any time in the menu.

- **Online:** choose **Set API Key…** and paste a key from OpenAI (it starts with `sk-`) or Groq (`gsk_`). Lowspeak
  recognizes the provider by the key. Then pick a model under **OpenAI** or **Groq**.
- **On your Mac:** choose **Manage Models** › **Download Parakeet Ultra** or a Whisper model, then pick it under
  **Local**.

[Which model is right for you](#which-model-is-right-for-you) helps with the choice. Then hold fn, say something, and
let go.

### The menu

Click the ring in the menu bar:

| Item | What it does |
|---|---|
| **Recent Dictations** | the last ten texts; a click copies one to the clipboard again |
| **Local** | the models on your Mac, grouped into Whisper and Parakeet; those not downloaded are greyed out |
| **OpenAI**, **Groq** | the online models; without a key they are greyed out and the menu offers **Set API Key…** |
| **Language** | the language you dictate in, or **Auto-Detect**; the menu follows it |
| **Post-Processing** | **Off**, **Clean Up** or **Format**, see [Post-processing](#post-processing) |
| **Vocabulary…** | names and terms to spell exactly; the menu shows how many you entered |
| **Microphone** | **System Default** or a fixed microphone, and **Play Sound When Ready** |
| **Manage Models** | download and delete local models, with progress |
| **Set API Key…** / **Update API Key…** | add, replace or remove keys |
| **Open at Login** | start Lowspeak with your Mac; the checkmark shows it is on |
| **Check for Updates…** | look for a new version now; the two items below it decide whether Lowspeak looks once a day and installs updates without asking. Only in the app from the disk image |
| **Help** | **Quick Reference**, **Report Problem…**, **Suggest Feature…**, **Lowspeak on GitHub** |
| **About Lowspeak** | version, copyright, project page and license |
| **Quit Lowspeak** | ⌘Q while the menu is open |

The checkmark under **Local**, **OpenAI** and **Groq** marks the model in use. The icon shows what Lowspeak is doing:

| Icon | Meaning |
|---|---|
| Ring with a dot | Ready |
| Filled circle | Recording while fn is held |
| Rounded square | Recording locked; the next press on fn or Shift ends it |
| Blinking ring | Transcribing |
| Ring with a dot at the top right | A new version is waiting, see **Check for Updates…** |
| ↓ 42 % | Downloading a model |

Lowspeak has no settings window: everything is in this menu, and it remembers every choice.

### Quick reference

**Help › Quick Reference** shows the fn gestures in a small window, in your language, with a button that opens the
keyboard settings. **Report Problem…** opens a form on GitHub with your Lowspeak and macOS versions filled in,
**Suggest Feature…** a discussion for ideas, **Lowspeak on GitHub** the project page.

## Dictating

### Hold to talk

Hold fn, speak, release. The recording icon appears the moment your microphone delivers sound; start speaking then,
since what you say before is not recorded. Releasing fn ends the recording, and the text follows a moment later.

A short tap on fn does nothing, so fn keeps its other uses. A recording that stays empty or lasts under 0.3 seconds is
dropped.

### Lock with Shift or a double press

For longer texts, lock the recording so it runs without a key held. There are three ways:

- **Shift while holding fn.** Start with fn as usual and press Shift during the recording: it locks, and you can let go
  of fn. Nothing you said before is lost, the recording simply continues.
- **Shift, then fn.** Hold Shift and press fn: the recording starts locked.
- **fn twice.** Tap fn briefly (shorter than 0.3 seconds) and press it again within 0.4 seconds. The recording runs
  from the first press.

The icon turns into a rounded square. Press **fn** or **Shift** once to stop; the text follows as usual. If your 🌐 key
is set to open Emoji & Symbols, stop with Shift: a short press on fn would open the viewer.

A recording stops by itself after two minutes while fn is held, and after five minutes when locked, and is then
transcribed as usual.

### Where the text goes

Lowspeak pastes the text into the app that was in front when you stopped speaking, at the cursor, through the clipboard
and ⌘V. It saves everything that was on the clipboard before and puts it back half a second after the app has read the
text, or after ten seconds if it never does, unless another app changed the clipboard in between. The text is marked as
transient, so clipboard history apps that respect the marker do not record it.

You can start the next dictation while the last one is still being transcribed. The texts are pasted in the order you
spoke them, and never while you are recording or holding Shift, so a paste cannot land in the middle of your next
sentence.

Without the Accessibility permission, or when the simulated ⌘V cannot be sent, the text stays on the clipboard and a
notification says so; paste it yourself with ⌘V.

### Recent dictations

**Recent Dictations** in the menu lists the last ten texts, newest first, each shortened to one line; hovering shows the
whole text. A click copies it to the clipboard, for when a text landed in the wrong place or you need it twice. The
list lives in memory only and is empty after Lowspeak quits.

## Transcription

### Which model is right for you

| You want | Choose | Why |
|---|---|---|
| **Speed without using your Mac** | **Groq › Whisper Large V3 Turbo** | the fastest online option; nothing is downloaded, and your Mac keeps its memory and disk space |
| **The best result** | **OpenAI › GPT Transcribe** (default) | the most accurate of the models Lowspeak offers; takes your vocabulary as keywords |
| **Offline and private** | **Local › Parakeet Ultra** for European languages, **Local › Whisper Large v3 Turbo** for all others | works without a network, and your voice never leaves your Mac; Parakeet is the fastest of all |
| **No account, no costs** | any **Local** model | runs on your Mac; nothing is billed |
| **Online, but free** | **Groq**, on its free plan | a Groq account without a payment method; transcription costs nothing within Groq's daily limits |
| **Both reliability and speed** | an online model, plus Parakeet Ultra or a Whisper model installed | the local model steps in when the online service fails |
| **An Intel Mac** | **Groq** or **OpenAI** | local models run on the processor there, see [Intel Macs](#intel-macs) |

Local models occupy their download size on disk and, while chosen, about as much memory; Lowspeak keeps the model loaded
so dictation starts without delay. On a Mac with 8 GB of memory, prefer Parakeet Ultra or an online model over the large
Whisper models.

### Online: OpenAI and Groq

| Model | Provider | Notes |
|---|---|---|
| **GPT Transcribe** (default) | OpenAI | most accurate; takes your vocabulary as keywords |
| **Whisper** | OpenAI | the original Whisper model |
| **Whisper Large V3** | Groq | very fast |
| **Whisper Large V3 Turbo** | Groq | fastest online |

The recording is compressed to AAC before the upload, about 6 KB per second of speech, so dictation works on slow
connections too. While you hold fn, Lowspeak already opens the connection to the provider, so the upload starts the
moment you let go.

### On your Mac: Whisper and Parakeet

| Model | Size | Runs on | Languages |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 European languages |
| **Parakeet v2** | ~600 MB | Neural Engine | English |
| **Whisper Large v3 Turbo** | ~1.6 GB | GPU (Metal) | about 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | about 99 |
| **Whisper Medium** | ~1.5 GB | GPU (Metal) | about 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | about 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | about 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | about 99 |

**Manage Models** downloads and deletes them; during a download the menu bar shows its progress, and quitting Lowspeak
cancels it. The very first use of a model prepares it for your Mac: about 10 seconds for Parakeet Ultra, up to 30
seconds for Whisper; after that it loads in a fraction of a second. If the model you chose is deleted, Lowspeak switches
to another installed one.

**Parakeet** is the fastest choice: it transcribes 40 seconds of speech in about 0.3 seconds. Lowspeak runs it with the
same models and decoding as [FluidAudio](https://github.com/FluidInference/FluidAudio) and stores them in FluidAudio's
folder (`~/Library/Application Support/FluidAudio/Models`), so apps built on FluidAudio share them. Recordings longer
than 15 seconds are transcribed in overlapping windows that are joined without losing words at the seams.
**Whisper** understands more languages and reads your vocabulary; the large models are the most accurate, the small ones
much weaker outside English. Whisper models are stored in `~/Library/Application Support/Lowspeak/Models`.

### When the online service fails

If an online transcription fails, because the network is down, the provider has a problem or the key is wrong, an
installed local model transcribes the same recording: the one chosen under **Local**, otherwise Parakeet Ultra, then
Whisper Large v3 Turbo, Large v3 and the smaller Whisper models, with Parakeet v2 last. A notification tells you once;
it comes again only after an online dictation has succeeded in between.

With a local model to fall back on, Lowspeak gives the online request 20 seconds; without one, it tries three times for
up to a minute each. In online mode, Lowspeak prepares the fallback Parakeet model once at launch, so the first fallback
does not wait for it.

### How the recording is prepared

Lowspeak records at your microphone's own rate and converts to the 16 kHz the models expect once you stop. Before the
transcription it removes low rumble below 80 Hz, trims silence at the start and end, and evens out the volume, so quiet
and loud speakers come out alike. A recording without speech is not sent at all. In local mode, Parakeet first gets the
untouched recording and the prepared one only if that yields no text.

### Languages per model

The language under **Language** is passed to the online models and to Whisper. With **Auto-Detect** the model works out
the language itself.

| Model | Languages you can dictate |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, on your Mac) | about 99, among them all 35 of the menu |
| Parakeet Ultra | Bulgarian, Croatian, Czech, Danish, Dutch, English, Estonian, Finnish, French, German, Greek, Hungarian, Italian, Latvian, Lithuanian, Maltese, Polish, Portuguese, Romanian, Russian, Slovak, Slovenian, Spanish, Swedish, Ukrainian |
| Parakeet v2 | English |

Parakeet detects the language itself and ignores the choice in the menu. **Chinese (Traditional)** tells GPT Transcribe
to write traditional characters; Whisper writes Chinese in either script and Portuguese without regard to the variant.

### Intel Macs

Lowspeak is a universal app and runs on Intel Macs with macOS 14 as well. Everything online works the same. The local
models do not have the hardware they are built for there: Whisper runs on the processor instead of the GPU, and
Parakeet on the processor instead of the Neural Engine. Both work, but take many times longer than on Apple Silicon,
the large Whisper models far too long for dictation. On an Intel Mac, transcribe online and keep Whisper Small or
Parakeet Ultra as the [fallback](#when-the-online-service-fails).

## Vocabulary

**Vocabulary…** takes the names and terms Lowspeak should spell exactly, separated by commas: product names, colleagues,
technical terms. Up to 100 terms of up to 60 characters each; `<` and `>` are removed.

- **GPT Transcribe** receives them as keywords.
- **Whisper** (OpenAI, Groq and on your Mac) reads them as a hint before the recording, up to 500 characters.
- **Post-processing** is told to spell them exactly as written.
- **Parakeet** does not use them.

## Post-processing

After the transcription, a language model can rework the text. Choose under **Post-Processing**:

- **Off** (default) — the text as transcribed.
- **Clean Up** — removes filler words such as "um", "you know" or "basically", fixes grammar and punctuation, and keeps
  only the correction when you correct yourself. It adds nothing and does not change the meaning.
- **Format** — like Clean Up, and adapts the text to the app you dictate into: code-style terms in code editors,
  command lines in terminals, a casual tone in chat apps, a professional one in email.

Post-processing uses Groq (`openai/gpt-oss-120b`) if you have a Groq key, otherwise OpenAI (`gpt-4o-mini`), also when
you transcribe locally. With a language chosen under **Language**, it keeps that language. If it takes longer than
eight seconds, fails, or returns an answer that is cut off or far longer than what you said, Lowspeak pastes the
unprocessed text. If the provider refuses the request or no key is set, a notification says so once.

## Microphone

Under **Microphone**, pick a fixed microphone or **System Default**. Lowspeak remembers the device; while it is not
connected, the menu shows it as **(Not Connected)** and Lowspeak records with the system default.

The microphone is switched on only while you dictate. Getting it ready takes about 0.07 seconds with the built-in
microphone, about half a second with a Studio Display, and one to two seconds with AirPods, which first switch to their
headset mode. The recording icon appears once sound arrives; **Play Sound When Ready** adds a short sound at that moment.
It is off by default, because on some speakers the sound itself comes half a second late.

If a microphone delivers only silence for five seconds, as AirPods do when another device holds their microphone,
Lowspeak tells you.

## API keys

**Set API Key…** asks for a key and recognizes the provider from its start: `sk-` for OpenAI, `gsk_` for Groq. A key in
another format is refused with an explanation. Enter one key per provider; with keys for both, all online models are
available. **Update API Key…** replaces a key or, with **Remove All Keys**, deletes them.

The keys are stored in your login keychain under `com.lowspeak.app`. A key file of an earlier version
(`~/Library/Application Support/Lowspeak/.api-key`) is moved there at the next launch and deleted once every key reads
back from the keychain.

## Notifications

Lowspeak works in the background, so it reports problems as notifications:

| Notification | What happened |
|---|---|
| **Recording Failed** | no microphone access, no microphone connected, or the microphone could not start |
| **Microphone Silent** | five seconds of silence, see [Microphone](#microphone) |
| **Accessibility Permission Required** | the text is on the clipboard; allow Accessibility to paste automatically |
| **Paste Failed** | the text is on the clipboard; paste it with ⌘V |
| **Transcription Failed** | with the reason: no key, rejected key, rate limit, server or network error, or no local model |
| **Transcribed on This Mac** | the online service failed and a local model took over |
| **Post-Processing Failed** | the provider refuses post-processing or no key is set; the text was pasted without it |
| **No API Key** | you chose an online model without a key for its provider |
| **Download Failed**, **Delete Failed**, **Cannot Delete** | managing a local model did not work, or another operation was still running |
| **Login Item Error**, **API Key Error** | macOS or the keychain refused the change |

Notifications never contain your dictated text. A failing online service and a refusing post-processing provider are
reported once, not after every dictation, until they work again.

## Appearance and accessibility

- **The menu bar icon** is a template image that follows a light, dark or tinted menu bar.
- **VoiceOver** reads the icon as "Lowspeak", and the menu, alerts and notifications are standard macOS controls.
- **No key needs to be held.** Lock a recording with Shift or a double press of fn, and the next press ends it.
- **The Quick Reference** is a plain window with labels and a button, reachable from the keyboard; Esc closes it.

Known gaps are listed in [ACCESSIBILITY.md](ACCESSIBILITY.md): VoiceOver does not announce when a recording starts, and
Lowspeak has not yet been checked with VoiceOver or Switch Control.

## Languages

The menu, dialogs and notifications are translated into 35 languages:

Arabic, Catalan, Chinese (Simplified), Chinese (Traditional), Croatian, Czech, Danish, Dutch, English, Finnish, French,
German, Greek, Hebrew, Hindi, Hungarian, Indonesian, Italian, Japanese, Korean, Malay, Norwegian, Polish, Portuguese
(Brazil), Portuguese (Portugal), Romanian, Russian, Slovak, Slovenian, Spanish, Swedish, Thai, Turkish, Ukrainian and
Vietnamese.

The language you choose under **Language** is also the language of the menu. With **Auto-Detect**, the menu follows the
language order in **System Settings › General › Language & Region**, and English where no translation exists. Menu
terms follow Apple's own translations, so Lowspeak reads like the rest of your Mac. Apart from English, the translations
are machine-made and have not yet been reviewed by native speakers; corrections are welcome as
[issues](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Keys and gestures

| Action | Keys |
|---|---|
| Dictate while held | hold fn |
| Lock a running recording | Shift while holding fn |
| Start a locked recording | Shift+fn, or fn twice quickly |
| End a locked recording | fn or Shift |
| Quit Lowspeak (menu open) | ⌘Q |
| Close the Quick Reference | Esc |

Lowspeak keeps the fn presses it uses from reaching other apps; Shift passes through as usual.

## Privacy and security

Lowspeak hears what you say while fn is held, can type into any app and keeps your API keys, so it does as little as it
can with them.

- **Your voice goes to one place.** In online mode the recording, compressed, goes to the provider you chose, OpenAI or
  Groq, together with your vocabulary. In local mode nothing leaves your Mac. When an online request fails, a local model
  transcribes the same recording on your Mac.
- **Post-processing**, if you turn it on, sends the transcribed text and your vocabulary to Groq or OpenAI. In Format
  mode it adds the kind of app you dictate into (code editor, terminal, chat or email); the app's name is not sent.
- **No telemetry, no account.** Apart from transcription and post-processing, Lowspeak goes online only to open the
  connection to your provider while fn is held, in online mode or with post-processing on, to download a model when
  you ask for one, from Hugging Face over HTTPS, at a fixed version and with every file checked against its checksum,
  and to look for a new version on GitHub, once a day if you allow it. That request carries Lowspeak's version and
  nothing else.
  **Help › Report Problem…** opens a form on GitHub in your browser; its address carries Lowspeak's version and the macOS
  version, nothing you dictated.
- **API keys** stay in your login keychain. They are sent only to their own provider and removed from error messages
  before anything is written to the log.
- **Dictated text is not stored.** The last ten dictations stay in memory for **Recent Dictations** and are gone when
  Lowspeak quits. The log files record lengths and timings, never the text.
- **The clipboard** is saved before pasting and restored once the app has read the text, unless another app changed it
  in between. The dictation is marked as transient, so clipboard history apps that respect the marker skip it.
- **Code signing** with the Hardened Runtime; the app asks only for the microphone, Accessibility, notifications and the
  network. An update must carry the developer's signature, or Sparkle refuses it.

To report a vulnerability, follow the [security policy](SECURITY.md).

## Speed

Measured on Apple Silicon:

- **Parakeet Ultra** transcribes 40 seconds of speech in about 0.3 seconds.
- **Online:** the first request after a pause takes about 0.4 seconds at Groq, because Lowspeak opens the connection while
  you are still speaking; later requests reuse it, over HTTP/3 where the provider offers it.
- **Upload size:** about 6 KB per second of speech, as AAC at 48 kbit/s.
- **Getting the microphone ready:** about 0.07 seconds built in, half a second on a Studio Display, one to two seconds
  with AirPods.

## Known limitations

- **Local models on Intel Macs** run on the processor and are much slower; online transcription is the better choice
  there.
- **What you say before the microphone is ready is lost.** Wait for the recording icon, especially with AirPods.
- **The fn key can be taken by others.** When macOS's 🌐 key setting, Warp or macOS Dictation claim fn first, Lowspeak
  does not see it; see [Troubleshooting](#troubleshooting).
- **Parakeet** ignores the vocabulary and the language choice, and knows only European languages.
- **Script and variant:** only GPT Transcribe can be told to write Traditional Chinese; no model can be told to write
  European rather than Brazilian Portuguese.
- **Time limits:** two minutes while fn is held, five minutes when locked.
- **Mic Mode:** Control Center offers Voice Isolation and Wide Spectrum for Lowspeak and reports them as active, but
  Lowspeak records without voice processing, so the choice has no effect.
- **Recent Dictations** live in memory only and are gone after quitting.
- **By design:** no settings window, no Dock icon, not in the App Store.

## Troubleshooting

**Nothing happens when I hold fn.** Check three things: Lowspeak is allowed under **System Settings › Privacy &
Security › Accessibility** (on macOS 27: **Device Control and Data Access**); **System Settings › Keyboard › Press 🌐 key to** is set to **Do Nothing**; and no other app
claims fn. Warp does by default; turn it off with `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` and
restart Warp. macOS Dictation can also be set to start with fn.

**After an update, Lowspeak no longer reacts to fn.** macOS ties the Accessibility permission to the app's signature.
If the signature changed, for example from a build of your own to the downloaded app, remove Lowspeak from the
Accessibility list (on macOS 27: Device Control and Data Access) and add it again.

**The first words are missing.** Start speaking once the recording icon appears. With AirPods, the microphone needs one
to two seconds; a built-in or USB microphone under **Microphone** is ready almost at once.

**"Microphone Silent."** The microphone delivered no sound for five seconds. With AirPods, another device may be using
their microphone; switch to the built-in microphone or reconnect them. A closed MacBook lid also mutes the built-in
microphone.

**Dictations come out empty.** The input volume may be too low: apps such as Teams, Zoom or Meet change it during calls,
and macOS keeps it per device. Raise it under **System Settings › Sound › Input**.

**The text landed in the wrong place.** **Recent Dictations** in the menu copies it to the clipboard again.

**"No API Key."** Choose **Set API Key…** in the menu, or switch to a local model.

**The first local transcription takes long.** Each model is prepared for your Mac on its first use; from then on it
loads in a fraction of a second.

**Lowspeak quits right after launch.** Another copy of Lowspeak is already running; quit that one first.

**No notifications appear.** Allow them under **System Settings › Notifications › Lowspeak**.

**Something else.** The logs in `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`,
`hotkey.log`, `crash.log`) describe what happened, without your dictated text. Attach the relevant lines to a
[problem report](https://github.com/sventmueller/lowspeak/issues/new/choose).

## For developers

Building, testing, the layout of the code and the rules it follows are described in
[docs/development.md](docs/development.md); contributions in [CONTRIBUTING.md](CONTRIBUTING.md).

### Third-party components

| Component | Version | Used for | License |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) via [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | local Whisper on the GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet decoding and long-form merge, translated to Rust in `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) and its framework crates | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation and other system frameworks | MIT; Zlib, Apache-2.0 or MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | microphone capture through CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | resampling to 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) with rustls | 3.5 | model downloads | MIT or Apache-2.0; rustls and ring under ISC and Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | checksums of the model downloads | MIT or Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | API keys in the keychain | MIT or Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, errors, the vocabulary of Parakeet | MIT or Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | updates of the app from the disk image | MIT, with parts under BSD and zlib licenses |

The license texts of everything inside the app ship with it as `Contents/Resources/Acknowledgements.txt`.

The models are not part of Lowspeak; it downloads them when you ask:

| Model | Source | License |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (OpenAI's Whisper in ggml format) | MIT |
| Parakeet v2, Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference) CoreML conversions of NVIDIA's Parakeet TDT | CC-BY-4.0 |

## License

Lowspeak is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Lowspeak by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
