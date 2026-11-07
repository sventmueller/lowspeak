# Lowspeak — Development

How Lowspeak is built, tested and laid out, and the rules its code follows.

## Overview
Native macOS menu bar app for speech-to-text, written in Rust. Hold fn (Globe) to record, release to
transcribe and paste. Rust with `objc2` bindings to AppKit and the other system frameworks, built with Cargo — no
Xcode.

## Tech Stack
- **Language:** Rust 2024 edition, macOS 14+ (Sonoma); the disk image is universal (arm64 and x86_64), every other build is for this Mac's processor
- **UI:** `NSStatusItem` + `NSMenu` through `objc2-app-kit`; no window, no egui
- **Audio capture:** `cpal` (CoreAudio) at the device rate, mixed to mono, resampled to 16 kHz with `rubato` when recording stops
- **Audio compression:** AAC/M4A through AudioToolbox `ExtAudioFile` (WAV fallback)
- **Local transcription:** Whisper via `whisper-rs` (whisper.cpp, Metal) + NVIDIA Parakeet TDT on FluidAudio's CoreML models via `objc2-core-ml` (Neural Engine)
- **Online transcription:** OpenAI + Groq via the shared `NSURLSession` (multipart/form-data, provider detected from the key prefix); `ureq` only for model downloads
- **Preprocessing:** 80 Hz high-pass (biquad) + RMS normalisation with tanh soft clip
- **Post-processing:** optional LLM clean-up (filler words, grammar, app-aware formatting)
- **Hotkey:** Quartz event tap (`objc2-core-graphics`) on its own thread with a CFRunLoop
- **Login item:** `SMAppService`
- **Build:** `cargo xtask` (bundle, sign, deploy, DMG) — see Build & Deploy

## Architecture

### Layout
| Path | Purpose |
|---|---|
| `src/main.rs` | `NSApplication` setup, single-instance check, Accessibility prompt, app delegate (terminate) |
| `src/app/` | `state.rs` what the app decides, without AppKit or threads (events in, `Effect`s out), built from `recording.rs` (the running recording), `delivery.rs` (dictations on their way to the paste) and `model_library.rs` (installed models, the running download or deletion); `mod.rs` carries the effects out on the main thread; `capture.rs` the recorder on its own thread; `policy.rs` further decisions without AppKit (selection fallback, notification streaks, time limits, dictation order); `gesture.rs` fn hold/double-tap; `menu.rs` status item, icons, menu; `dialogs.rs` API key and vocabulary alerts; `notify.rs` user notifications; `reference.rs` Help → Quick Reference window; `feedback.rs` GitHub links of the Help menu; `about.rs` About panel |
| `src/hotkey.rs` | fn key detection via event tap |
| `src/models.rs` | the settings enums and the model catalogue (`LocalModel::spec`); how a model runs and where it comes from is in `transcription/catalog.rs` |
| `src/audio/` | `mod.rs` sample rate and minimum length; `recorder.rs` capture, `vad.rs` silence trimming, `preprocess.rs` resampling, HPF and normalisation, `aac.rs` M4A encoding, `mp4.rs` M4A padding removal, `wav.rs` |
| `src/transcription/` | `service.rs` pipeline (`Services`), `mod.rs` `Api` (HTTP client, key store, cancellation, retry delay), `online.rs`, `postprocess.rs`, `local.rs` model thread (`LocalEngine`, `ModelLoader`), `whisper.rs`, `parakeet/` (CoreML port of FluidAudio), `catalog.rs` engine and pinned repository commit of each model, `model_store.rs` paths/download/delete (`ModelStore`, `Library`), `hf.rs` checked Hugging Face downloads (`Hub`), `error.rs`, `test_server.rs` local HTTP server for the download tests |
| `src/config/` | `settings.rs` (NSUserDefaults), `vocabulary.rs` (terms to spell exactly), `api_keys.rs` (keychain), `login_item.rs` |
| `src/i18n/` | `mod.rs` the `Texts` table and language choice; one file per language with menu, dialog, notification and error texts |
| `src/paste.rs` | clipboard backup, the dictation handed out on request, simulated ⌘V, restore after the read |
| `src/util/` | `filelog.rs` (`flog!` macro), `crash.rs`, `retry.rs`, `url_session.rs` (API requests, `HttpClient`), `http.rs` (`ureq` for downloads, key redaction), `numeric.rs` (saturating conversions), `main_thread.rs`, `paths.rs`, `version.rs` (bundle version) |
| `xtask/` | build tooling (`cargo xtask …`): `bundle.rs` Info.plist, bundle, install, universal program; `signing.rs`; `release.rs` DMG and notarisation; `assets.rs` icon; `licenses.rs` `Acknowledgements.txt` |
| `tests/` | integration tests (see Tests); `fixtures/builtin.wav` a short recording from the built-in microphone |
| `docs/` | this guide |
| `assets/readme/` | the README in the other 34 interface languages |
| `.github/` | issue, discussion and pull request templates, CI |
| root | `VERSION`, `Makefile` (entry points), `rust-toolchain.toml`, `LICENSE-MIT`, `LICENSE-APACHE`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `ACCESSIBILITY.md` |
| `assets/` | `icon.svg` (source of the app icon, splat's colours) and its render `icon-1024.png`, `AppIcon.icns`, `Lowspeak.entitlements`, `dmg-background.svg` (source of the disk image background) and its render `dmg-background.tiff` at 1× and 2× |
| `examples/transcribe.rs` | transcribe a WAV with a local model from the command line |

### Threads and state
- `State` decides everything the app does: keys, timers, finished work and menu choices go in, a list of `Effect`s comes out. It knows neither AppKit nor threads, so `app/state.rs` tests the paste order, the recording lifecycle and the model library without them. `App` carries the effects out and feeds the results back as events
- `App` lives in a `thread_local` on the main thread. `with_app(|app| …)` borrows it; `dispatch(|app| …)` queues work from other threads onto the main queue. Timers and callbacks go through `dispatch` too, so work that finds the state borrowed is re-queued, never dropped
- Never hold the `App` borrow across anything that spins the run loop (modal `NSAlert`, `NSApp.terminate`). `MenuTarget::menu_action` runs those outside the borrow
- Every menu choice is an `Action` that carries its data and goes through one exhaustive match; the menu is rebuilt in `menuNeedsUpdate:` from a `Snapshot`
- Opening a microphone takes up to two seconds with Bluetooth headsets, so the recorder runs on its own thread (`app/capture.rs`). The microphone list and the login item state are cached and refreshed on a background thread at launch and when the menu opens; a change rebuilds the open menu
- Download and delete threads catch panics and report them as failures, so the running operation always ends. A model being deleted can neither be chosen, loaded, downloaded nor used as fallback
- `LocalTranscriber` owns the loaded model on one thread and handles preload/transcribe/unload in order. CoreML objects are not `Send`, which is why they stay on that thread. A panic in a command drops the loaded model and becomes an error; the thread keeps serving. `ModelLoader` is the seam its tests use
- Each transcription runs on its own thread (`service::run`), then dispatches the result back for pasting. A panic there becomes an `Engine` error, so the queue below never stalls.
- fn starts the next dictation while earlier ones are still transcribing. `policy::Dictations` hands the results over in recording order; pasting waits until no recording runs (no ⌘V while fn or Shift is held) and until the previous paste has restored the clipboard.
- `service::run` reaches everything outside itself through `Services`: `LocalEngine` (the model thread), `ModelStore` (installed models), and `Api` with an `HttpClient` (the shared URLSession) and a `KeyStore` (the keychain). The app passes `Disk` and `Api::SYSTEM`; `tests/pipeline.rs` passes substitutes.

### Data flow
```
fn DOWN → event tap thread → dispatch → App::fn_down → FnGesture → App::start_recording
  → Recorder::start (cpal, chosen or default mic), status Starting, HEAD request opens the API connection
  → first non-zero samples (audio thread) → dispatch → status Recording, recording icon, "Tink" if enabled (off by default)
fn UP   → event tap thread → dispatch → App::fn_up → FnGesture (held: stop; tap: wait for a second press)
  → App::stop_recording_and_transcribe → Recorder::stop → worker thread: resample to 16 kHz → service::run
    → Parakeet in local mode: raw audio first; empty or error → preprocessed pipeline
    → HPF → VAD trim → normalise → too short? skip
    → Online: AAC (WAV fallback) → OpenAI/Groq with the vocabulary; failure → installed local model
    → Local: LocalTranscriber (Whisper with the vocabulary as prompt, or Parakeet)
    → optional LLM post-processing with the vocabulary
  → dispatch → Dictations queue (recording order; waits while a recording runs)
  → paste (clipboard + ⌘V, restore once the app has read it, then the next one), keep in Recent Dictations, notices → status Idle once the queue is empty
```

## Critical Implementation Details

### Event tap (`hotkey.rs`)
- Event mask **only `flagsChanged`** — adding keyDown/keyUp makes macOS stop delivering fn events
- fn: keycode 63, flag `0x800000` (`MaskSecondaryFn`); the fn events are swallowed (callback returns null)
- Shift (keycodes 56/60) is reported on press for the hands-free gestures and passed through to the app
- Tap creation retries (10 × 2 s, then 10 s pause) — Accessibility may be granted after launch
- Re-enabled on `TapDisabledByTimeout`/`ByUserInput` and by a 30 s watchdog timer
- The callback's `user_info` is the `Arc<Inner>`; the tap thread keeps it alive
- The callback never writes files: an active tap delays every modifier event of the session, so its log lines go to a logger thread. The fn/Shift edge detection is a pure function (`key_event`)

### fn gestures (`app/gesture.rs`)
- Hold fn to dictate. A press shorter than 300 ms followed by a second press within 400 ms locks the recording (hands-free, square icon); the next press stops it
- Shift+fn in either order (Shift held at fn down, or Shift pressed while fn is held) also locks it; once locked, Shift alone stops it as well. With "Press 🌐 key to: Emoji & Symbols" every short fn tap opens the viewer, so Shift is the stop for users who keep that setting
- A single tap stops once the 400 ms window has passed, so taps still end as "too short"
- Limits: 120 s while held (guards against a lost fn release), 300 s hands-free

### fn key gotchas
- **"Press 🌐 key to: Emoji & Symbols"** intercepts fn before the tap. Set it to "Do Nothing" (System Settings → Keyboard)
- **Warp** has `VoiceInputToggleKey = "Fn"` by default: `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'`, restart Warp
- **macOS Dictation** can also interfere

### Audio
- Capture runs at the device's native format; resampling happens once at stop, off the main thread
- Each recording gets a sample buffer sized for the 300 s limit plus 10 s; the audio callback writes into it without locks or allocation and readers see the published length. Zeroed memory is mapped lazily, so the capacity costs memory only as it fills; audio beyond it is dropped and logged
- The start sound comes ~0.5 s after the microphone is live on Studio Display speakers (speaker wake-up 0.53 s, 0.11 s when already playing); waking the speaker early delays the mic instead. Hence off by default
- The microphone opens only while fn is held, so speech before it delivers audio is lost. Start-up: ~0.07 s built-in mic, ~0.5 s Studio Display mic, 1–2 s AirPods (switch from A2DP to the headset profile, exact zeros meanwhile)
- Menu → Microphone pins a device by its CoreAudio UID (`microphoneID`/`microphoneName` defaults); a disconnected choice falls back to the system default and is shown as "(not connected)"
- The recording icon (and the start sound, "Play Sound When Ready", off by default) appear only with the first non-zero samples — the moment speech is actually captured. A closed MacBook lid mutes the built-in mic to zeros
- Silent-mic check after 5 s (no samples or RMS < 0.001 → notification; AirPods mic claimed elsewhere)
- Control Center's Mic Mode (Standard / Voice Isolation / Wide Spectrum) is offered for Lowspeak too, and macOS 27 reports the chosen mode as active although capture runs through CoreAudio without voice processing. `audio.log` records RMS, peak and the active/chosen mode per recording
- The macOS input volume is stored per device and any app may change it (Teams, Zoom, Meet adjust it in calls); at 50 % the Studio Display mic left speech below the VAD minimum
- Microphone permission is requested at launch via `AVCaptureDevice`
- AAC: 48 kbit/s, the highest rate the system encoder accepts for 16 kHz mono (64 kbit/s makes writes fail with `!dat`; AVAssetWriter rejects it too). `ExtAudioFile` reserves ~23 KB of `free` space in front of the audio; `mp4.rs` removes it and shifts the chunk offsets, since upload size matters on slow links

### Parakeet (`transcription/parakeet/`)
- Port of FluidAudio 0.17.7: `coreml.rs` (MLModel/MLMultiArray wrappers), `networks.rs` (prediction and joint networks behind the `Networks` trait), `decoder.rs` (greedy TDT decoding), `chunks.rs` (long-form driver), `layout.rs` (window starts and lengths), `energy.rs` (frame energy for window starts and seam repair), `merge.rs` (token merge of overlapping windows), `seams.rs` (seam-word collapse, seam-gap repair), `text.rs` (vocabulary)
- One deliberate departure from FluidAudio: the seam-word collapse and the gap repair act only where two windows meet. FluidAudio applies them to the whole transcript, which drops genuine words such as "Das Essen essen wir" and re-decodes every pause; the repair is capped at two decodes per seam
- Each threshold says whether it is FluidAudio's value or Lowspeak's own tuning
- Output is verified identical to FluidAudio 0.17.7 on the test fixtures (`tests/transcription.rs`); merge logic has FluidAudio's unit tests in `chunks.rs`
- Models: `~/Library/Application Support/FluidAudio/Models/{parakeet-tdt-0.6b-v2,parakeet-ultra}/` — FluidAudio's own cache, so models it downloaded are reused. Installed = all four `.mlmodelc` bundles + `parakeet_vocab.json` present
- Ultra (`FluidInference/parakeet-ultra-coreml`) is a post-trained v3 with v3's files and decoding (FluidAudio's `isV3Family`): `JointDecisionv3.mlmodelc`, blank 8192, no mel-context, silence-aligned + end-aligned windows. v2: `JointDecision.mlmodelc`, blank 1024, 80 ms mel-context, regular windows
- Ultra replaced v3 (more accurate in all 24 FLEURS languages at the same speed). A saved `parakeet-v3` selection maps to Ultra; an existing v3 folder stays in the shared FluidAudio cache
- Compute units: preprocessor CPU only, encoder/decoder/joint CPU+Neural Engine. The first load in a program compiles for the ANE (~10 s for Ultra); CoreML caches the result per program, later loads take ~0.1 s
- The encoder takes exactly 15 s (240 000 samples). Longer audio goes through `ChunkProcessor`; never split a recording into shorter chunks yourself — words at the edges get lost
- Minimum 0.3 s audio; `max_tokens_per_chunk` 150 as in FluidAudio, logged when a window reaches it

### Whisper (`transcription/whisper.rs`)
- ggml models from `ggerganov/whisper.cpp` in `~/Library/Application Support/Lowspeak/Models/whisper/ggml-<name>.bin`
- Metal with flash attention; a 3 s warm-up on preload or prime compiles the Metal pipelines (first launch ever: up to ~30 s). The `WhisperState` is kept between dictations; each run clears it
- **Release the model before process exit** — whisper.cpp asserts in its exit-time destructors while a model is loaded. `app::shutdown` unloads (3 s timeout, then `_exit`)

### Downloads (`hf.rs`, `model_store.rs`)
- Every repository is pinned to a commit in `transcription/catalog.rs`; a new model version means a new commit there
- Parakeet: list the repo at that commit via `api/models/<repo>/tree/<commit>?recursive=true` (follows `Link` pagination), download the required bundles + root JSON files. Whisper files carry their size and SHA-256 in the catalogue
- Every file is checked against the object id git keeps for it: SHA-256 for Git LFS files, the git blob SHA-1 for small ones. A mismatch deletes the file and fails the download. Copies installed before the pinning stay accepted
- Files stream to `<name>.partial` and are renamed when complete and checked; 15 s connect timeout and 30 s without data end a stalled transfer, so cancel (set on quit) always takes effect
- Deleting a model removes only its own files; the shared Hugging Face cache in `~/.cache/huggingface` stays untouched
- `Library::under(dir, hub)` puts the models into another directory; `tests/downloads.rs` uses a temporary one

### Network
- API calls go through `NSURLSession.sharedSession`: it keeps connections alive between dictations and uses HTTP/3 (Groq/Cloudflare offer it). A fresh TLS connection per request cost ~1 s on a high-latency link
- `url_session::post` enforces a total deadline (`recv_timeout`, then `cancel`) and returns a `RequestError` (`TimedOut`, also for URLSession's own `NSURLErrorTimedOut`; `Failed` with URLSession's localised description)
- Retry waits end early when `Api::cancel` fires; `transcription::shutdown()` fires it for good when the app quits
- fn down sends a `HEAD` to the provider (at most every 20 s) so the connection is open when the upload starts: first request after a pause 0.61 s → 0.42 s (median, Groq)
- Microphone start-up (~0.5 s until the first samples on a Studio Display mic) is hardware-bound; `AVAudioEngine` is no faster

### Fallback to a local model
- A failed online transcription is redone by an installed local model: the one chosen for local mode, else the one with the lowest `fallback_rank` in the catalogue — Parakeet Ultra, then the largest Whisper, Parakeet v2 last (English only)
- With a fallback available the online request gets one attempt and 20 s; without, 3 attempts and 60 s each
- In online mode the fallback Parakeet model is loaded and released at launch (`LocalTranscriber::prime`), so the ANE compilation does not hit the first fallback
- One notification per failure streak; an online success re-arms it

### Vocabulary (`config/vocabulary.rs`)
- Terms from Menu → Vocabulary, stored as a comma-separated string in the `vocabulary` default; at most 100 terms of 60 characters, `<`/`>` removed (OpenAI rejects them)
- `gpt-transcribe`: one `keywords[]` field per term. It also takes `languages[]`, never `language` (OpenAI rejects both together)
- Whisper (OpenAI `whisper-1`, Groq, whisper.cpp): the terms as `prompt`, cut at 500 characters (Whisper reads at most 224 prompt tokens); whisper.cpp gets them as tokens because `set_initial_prompt` leaks its string in whisper-rs 0.16
- Post-processing: listed in the system prompt
- Parakeet ignores it: FluidAudio's vocabulary boosting needs a separate English CTC model (~100 MB) and its own rescoring

### API keys
- Generic passwords in the login keychain: service `com.lowspeak.app`, account = provider. Every build is signed with a stable identity (see Signing), so updates read them without asking; a test binary that reads them (`tests/online.rs`) makes macOS ask once per build
- The key file of earlier versions (`~/Library/Application Support/Lowspeak/.api-key`) is moved at launch and deleted only after every key reads back; until then keys are still read from it
- `LOWSPEAK_API_KEY_FILE` names a key file to use instead of the keychain (tests)
- Provider by prefix: `sk-` = OpenAI, `gsk_` = Groq; API error bodies are redacted before logging

### Post-processing
- Groq first (faster), else OpenAI; 8 s timeout; on any failure the raw text is pasted
- The transcript goes to the model between `<transcript>` tags as data, so spoken instructions cannot pose as the prompt. An answer that is cut off (`finish_reason` other than `stop`), empty, or more than twice as long as the transcript plus 40 characters counts as failed
- Trailing line breaks are stripped from every pasted text; in a terminal they would run the line
- Refusals that persist (every 4xx except 408 and 429, and a missing key) bring one notification until post-processing succeeds again; timeouts, 408, 429 and 5xx stay in the log
- Models: OpenAI `gpt-4o-mini`, Groq `openai/gpt-oss-120b` with `reasoning_effort` "low" (`ApiProvider::post_processing_model`; Groq retires models, `GET /openai/v1/models` lists the current ones)
- Frontmost app is captured when recording stops and selects the formatting hint

### Interface language (`i18n/`)
- Menu, dialogs and notifications use the dictation language (`language` default); auto-detect follows the macOS language order, else English. Logs stay English
- One `Texts` static per language: a missing field does not compile; texts with values are closures, so each language handles its own plurals
- New UI text: add a field to `Texts` and fill it in all 35 language files; `tests/menu.rs` checks English and German titles and opens the Quick Reference in every language
- English is the source text; the other 34 languages are machine translations that no native speaker has reviewed
- The languages are splat's. Chinese (Traditional) and Portuguese (Portugal) are variants for the interface and post-processing; Whisper gets `zh` and `pt` (`Language::code`), gpt-transcribe gets `zh-tw` for Traditional Chinese (`Language::code_for`)
- Auto-detect maps the macOS language: Chinese with `Hant`, or from Taiwan, Hong Kong or Macao, is Traditional; Portuguese with a region other than Brazil is European
- The Language menu lists every language under its own name (`Language::display_name`), whatever the interface language
- macOS 27 renamed Privacy & Security › Accessibility to Device Control and Data Access. `Texts` holds both names as Apple translates them (`accessibility_list`, `device_control_list`); `Texts::accessibility_setting` picks the one of the running system

### README
- `README.md` is the English source; `assets/readme/<folder>/README.md` holds it in every other interface language, in the folders Splat uses (`pt_BR`, `pt_PT`, `zh-Hans`, `zh-Hant`, the rest by language code)
- Every README carries the same language bar, links the download of the latest release and uses the menu texts of `src/i18n` and Apple's own terms for macOS; a change to the English README goes into all 34 translations
- `cargo test -p xtask readme -- --nocapture` lists what is missing: a README per interface language, the complete language bar, the download link, relative links that exist and in-page links that match a heading

### Menu and Human Interface Guidelines
- English menu items use title-style capitalization; items that ask for more input end in `…` (German ` …`), never `...` — `i18n` tests check the ellipsis in every language
- Toggles show the item's checkmark state, not a `✓` in the title; actions start with a verb ("Download …", "Delete …")
- Model groups in a submenu are `NSMenuItem::sectionHeaderWithTitle` headers (macOS 14), not indented titles
- A provider without a key still lists its models, disabled, followed by "Set API Key…"
- The menu ends with Help (Quick Reference, Report Problem…, Suggest Feature…, Lowspeak on GitHub), About Lowspeak and Quit Lowspeak (⌘Q)
- Notifications put the event in the title; macOS shows the app's icon itself. Errors during dictation stay notifications, not alerts, since an alert would take the focus from the text field the dictation goes to
- The status item images are template images with the accessibility description "Lowspeak"
- The app icon must follow Apple's macOS template (824 px body on a 1024 px canvas); full-bleed icons get a grey frame on macOS 26

### Help menu (`feedback.rs`)
- Links go to `https://github.com/sventmueller/lowspeak`: issues with `.github/ISSUE_TEMPLATE/bug.yml` (version and macOS filled in from the query by field id), discussions in the `ideas` category

### Model catalogue (`models.rs`)
- `LocalModel::spec` holds each model's name, download size, language note for the menu and fallback rank; `catalog::engine` its engine (Whisper file with size and SHA-256, or Parakeet version) and pinned repository. A new model is one variant plus an entry in each
- `stored_enum!` generates `ALL`, `raw_value` and `from_raw` for the settings enums, so `ALL` cannot miss a variant; earlier spellings follow `|` (`"parakeet-ultra" | "parakeet-v3"`)

### Errors
- One error type per layer: `RequestError` (URLSession), `TranscriptionError` (pipeline), `PostProcessError`, `recorder::StartError`; `anyhow` only inside the engines and for downloads, converted explicitly at the boundary
- HTTP statuses are classified once (`error::StatusClass`) for transcription and post-processing alike
- Notifications show translated texts (`user_facing_description`, and fixed texts for download, deletion, key and login item failures), never raw API bodies, error chains or paths; the details go to the log
- A failed resampling (`TranscriptionError::Audio`) and a failed ⌘V (`Pasted::Failed`, text left on the clipboard) reach the user as notifications

### Settings
- `NSUserDefaults` domain `com.lowspeak.app` (keys `mode`, `localModel`, `onlineModel`, `language`, `postProcessingMode`, `preprocessingEnabled`, `startSound`, `microphoneID`, `microphoneName`, `vocabulary`)
- `policy::reconciled` falls back to an available provider/installed model when the saved one is gone

### Clipboard
- All pasteboard items and types are saved before pasting. The dictation is put on the pasteboard through an `NSPasteboardItemDataProvider`, so Lowspeak learns when an app reads it; the old content comes back 500 ms after the first read, or after 10 s if nobody reads, unless another app changed the clipboard meanwhile (change count). The next queued dictation is pasted only after that, otherwise it would save the previous dictation as the content to restore
- The dictation carries `org.nspasteboard.TransientType`, so clipboard history apps that follow nspasteboard.org skip it. One that ignores the marker and reads first starts the 500 ms early
- Both ⌘V events are created before either is posted, so a failure cannot leave a pressed key behind
- Without Accessibility permission, or when the ⌘V keystroke cannot be created, the text stays in the clipboard and a notification says so
- Menu → Recent Dictations keeps the last 10 texts in memory only (gone on quit); a click copies one to the clipboard

## Build & Deploy

```bash
make                      # lists the entry points below
make check                # formatting, clippy without warnings, all tests that need no model, key, network or microphone
make build                # cargo xtask bundle
make deploy               # cargo xtask deploy
make release              # make check, then cargo xtask dmg: the notarized release in dist/
make universal            # the program for Apple Silicon and Intel, as the disk image carries it
make test-intel           # the tests as Intel code under Rosetta
```

```bash
cargo xtask bundle        # target/bundle/Lowspeak.app, signed for this Mac
cargo xtask deploy        # bundle, quit the running app, replace the installed copy, launch it, then the DMG
cargo xtask dmg           # dist/Lowspeak-<version>.dmg, signed with the Developer ID and notarized; see Distribution
cargo xtask icon          # regenerate assets/AppIcon.icns from assets/icon-1024.png (resvg render of assets/icon.svg)
cargo xtask signing-identity  # once per Mac: local certificate so updates keep the permissions
# add --debug for a debug build
```
- `deploy` installs to the existing copy under `/Applications` (found via Spotlight), else `/Applications/Lowspeak.app`; the previous copy goes to the Trash (`trash`: Homebrew or macOS 15+)
- `deploy` then packs the same bundle as `target/bundle/Lowspeak-<version>.dmg`, not notarized: for testing on another Mac, which asks before opening it
- `MACOSX_DEPLOYMENT_TARGET=14.0` is set in `.cargo/config.toml`
- `target/` may be a symlink to another volume to keep the build output off the system volume; Cargo follows it
- `rust-toolchain.toml` pins the Rust version for rustup and CI; Homebrew's Rust ignores it

### Licenses
- Lowspeak is `MIT OR Apache-2.0` (`LICENSE-MIT`, `LICENSE-APACHE`, `[workspace.package]` in `Cargo.toml`)
- `bundle` writes `Contents/Resources/Acknowledgements.txt`: Lowspeak's licenses and the license text of every crate `cargo tree` finds in the program, each text once under the components it covers, plus whisper.cpp (built from the sources inside `whisper-rs-sys`) and FluidAudio (`src/transcription/parakeet/LICENSE-FluidAudio`)
- Of alternatives a crate offers, MIT is taken where it can be; a crate that names MIT without shipping its text gets Lowspeak's MIT terms with the authors from its manifest
- A dependency whose license text xtask does not recognise stops the build with its name; `cargo test -p xtask` finds it first
- The Parakeet module is a translation of FluidAudio's Swift code; Apache-2.0 asks that the license goes along and that the module says so, which its doc comment does

### CI
- `.github/workflows/ci.yml` runs `make check` on a macOS runner for every push to `main` and every pull request. Signing, notarisation and the `#[ignore]` tests need the developer's keychain, keys, models or microphone and run only locally

### Version
- `VERSION` holds `major.minor.build`, the version the next build gets; `xtask` writes it into `CFBundleShortVersionString` and `CFBundleVersion`. The version in `Cargo.toml` stays `0.0.0`
- Every release build that succeeds (`bundle`, `deploy`, `dmg`) raises the build number by one; debug builds leave it alone. `dmg` raises it only after the notarized image is in `dist/`. Major and minor are raised by hand: set `VERSION` to `1.1.0` and the next build gets exactly that
- At runtime `util::version::app_version` reads the bundle's version ("dev" outside a bundle) for the About panel, the problem report, the download User-Agent and the launch line in `appstate.log`

### Distribution
`cargo xtask dmg` (or `make release`, which runs `make check` first) builds a release, signs it with the Developer ID,
notarizes the app with Apple and staples the ticket, packs it into a compressed, read-only disk image with a link to
Applications, signs, notarizes and staples the image as well and checks the result with Gatekeeper (`spctl`). The app
carries its own ticket so that a copy taken out of the image opens offline on first launch too; the two rounds with the
notary service take a few minutes each. Only an image that passed every step becomes `dist/Lowspeak-<version>.dmg`,
next to `dist/Lowspeak-<version>.md` with the notes; older ones go to the Trash and the build number goes up. A failed
run leaves `dist/` and `VERSION` as they were. It needs:

- a "Developer ID Application" certificate in the keychain
- a saved login for Apple's notary service with an app-specific password, created once per Mac with
  `xcrun notarytool store-credentials notary --apple-id <Apple ID> --team-id <team ID>`; splat uses the same profile
- `RELEASE_NOTES.md` with what this release brings; `dmg` stops if it is empty or still holds the notes in `dist/`

If one of them is missing, `dmg` stops before building. On the first run, macOS asks whether Terminal may control the
Finder, which lays out the disk image window. `dmg` then reads the icon positions Finder stored in the image and stops
if they are not the intended ones, as when Finder shows hidden files (⇧⌘. hides them) or lacks the permission;
`deploy` packs its image with Finder's default window instead.

The images are not committed (`.gitignore`); the notes are. The download link in the README points at `Lowspeak.dmg`
in the latest GitHub release, so each release carries a copy under that name:
`cp dist/Lowspeak-<version>.dmg target/Lowspeak.dmg`, then
`gh release create v<version> dist/Lowspeak-<version>.dmg target/Lowspeak.dmg --notes-file dist/Lowspeak-<version>.md`.

The disk image is a universal binary for Apple Silicon and Intel; other builds are for the host processor alone.
Homebrew's Rust ships the standard library for the host processor only, so `dmg` builds through rustup, which installs
the toolchain and targets of `rust-toolchain.toml` on first use, joins the two programs with `lipo` and stops if either
half is missing. `make universal` builds only the program, into `target/universal/`.

- The universal build has a target directory of its own (`target/universal/`): whisper-rs-sys hands every `GGML_*`
  environment variable to whisper.cpp's CMake but does not rebuild when one changes, so it must not share output with
  the builds for this Mac
- `GGML_NATIVE=OFF` for both halves. Otherwise whisper.cpp is compiled with `-mcpu=native` for the build Mac's chip,
  and a program built on an M3 can stop with an illegal instruction on an M1. The x86_64 half enables SSE4.2, AVX,
  AVX2, BMI2, FMA and F16C, which every Intel Mac with macOS 14 has; cross-compiling would otherwise switch them off
- With macOS 14 as deployment target, whisper.cpp's Metal code calls `__isPlatformVersionAtLeast` for its
  `@available` checks. That lives in clang's runtime library, which rustc does not link, so `build.rs` adds
  `libclang_rt.osx.a` from `xcrun clang --print-resource-dir` to every build
- On Intel, Whisper runs on the CPU (`use_gpu` only on aarch64): ggml's fast Metal kernels need an Apple GPU. Parakeet's
  CoreML models fall back from the Neural Engine to the CPU by themselves
- `make test-intel` (`cargo xtask intel-tests`) runs the tests of `make check` as x86_64 code, which Rosetta runs on
  Apple Silicon, built with the same settings as the universal program. CI runs them on an Intel runner as well

### Signing
- `bundle` and `deploy` sign with the first available identity: "Developer ID Application", "Apple Development", the local "Lowspeak Local Signing" identity, else ad hoc. `dmg` requires the Developer ID
- `cargo xtask signing-identity` creates the local identity once: a self-signed certificate in its own keychain `~/Library/Application Support/Lowspeak/Signing/signing.keychain-db` (password in `keychain-password` next to it, mode 0600). The login keychain, its search list and trust settings stay untouched; `codesign --keychain` finds the identity
- Hardened runtime + `assets/Lowspeak.entitlements` (audio input, network client, Apple Events)
- **Accessibility and microphone permissions are tied to the designated requirement.** With any certificate it is `identifier "com.lowspeak.app" and certificate leaf = H"…"` and survives rebuilds; ad-hoc builds pin the cdhash, so every install needs new permissions
- `security find-identity -v -p codesigning` lists Apple identities

## Tests
```bash
make check                                             # what CI runs
cargo test --release                                   # unit and property tests, pipeline with substitutes, menu
cargo clippy --release --all-targets --workspace       # lint rules in Cargo.toml; must pass without findings
cargo test --release --test transcription -- --ignored # installed models; filter with `parakeet` or `whisper`
cargo test --release --test recording -- --ignored     # the default microphone; the terminal needs microphone access
cargo test --release --test downloads -- --ignored     # Hugging Face listing, Whisper tiny (75 MB) and Parakeet Ultra (630 MB) downloads
cargo test --release --test online -- --ignored        # OpenAI and Groq requests with the stored keys, incl. vocabulary
cargo test --release --test fallback -- --ignored      # invalid key → local Parakeet Ultra takes over
cargo run --release --example transcribe -- tests/fixtures/builtin.wav parakeet-ultra
```
- `tests/menu.rs` runs without the test harness, because AppKit needs the main thread
- Tests that need models, keys, the network or the microphone are `#[ignore]` with the reason and fail when run without what they need
- Property tests (`proptest`) cover the MP4 and WAV parsers and the chunk merge; a failure leaves its seed in `proptest-regressions/`
- Seams for tests without hardware or network: `State` (app decisions), `ModelLoader` (model thread), `Networks` and `WindowDecoder` (Parakeet decoding), `Library` and `test_server` (downloads), `Api::retry_delay` (retries without waiting)
- Release profile: whisper.cpp and the resampler are far too slow unoptimised (dependencies use opt-level 3 in dev too)

## Debugging

### Log files (`~/Library/Application Support/Lowspeak/`)
Everything run through Cargo (tests, examples, `cargo run`) logs to `target/logs/` instead: `.cargo/config.toml` sets `LOWSPEAK_LOG_DIR`. `open` passes its environment on to the app, so `deploy` removes the variable before launching it.
- `hotkey.log` — event tap, fn events
- `appstate.log` — recording/transcription flow, models, notifications
- `audio.log` — devices, capture, compression
- `crash.log` — fatal signals, panics, uncaught Objective-C exceptions; every launch writes "Crash reporter installed", and `appstate.log` notes crashes of the session before
- macOS crash reports: `~/Library/Logs/DiagnosticReports/Lowspeak-*.ips`

### Common issues
| Issue | Cause | Fix |
|---|---|---|
| fn key not detected | Emoji picker intercepts fn | "Press 🌐 key to: Do Nothing" |
| fn key not detected | Warp steals fn | `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` + restart Warp |
| AXIsProcessTrusted: false | Signature identity changed (e.g. first build with a new certificate, or ad hoc) | Remove and re-add Lowspeak in Privacy & Security → Accessibility |
| App quits right after start | Another instance with the same bundle ID runs | Quit the other instance (`appstate.log`: "Another instance already running") |
| No notifications from `cargo run` | Unbundled binary has no bundle ID | Use `cargo xtask bundle`/`deploy` |

## Conventions
- Code, identifiers, commits in English
- Comments only where knowledge is missing that the code cannot show: a library's behaviour the call does not reveal, a legal or domain requirement, why the obvious solution fails. Usually one sentence; no paraphrase of the code, no history
- Log through `flog!(filelog::APP, …)` — stdout is invisible when launched via `open`. Log files stay open; rotation follows the size written, and every 64 lines the file is checked for having been deleted
- Every `unsafe` block has a `// SAFETY:` comment, and integer conversions that can lose values go through `try_from` or `util::numeric`; clippy enforces both
- A new dependency needs a license that `xtask/src/licenses.rs` recognises (see Licenses)
- Bundle ID: `com.lowspeak.app`
- No runtime dependencies besides system frameworks
