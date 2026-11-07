# Contributing

Problems go into an [issue](https://github.com/sventmueller/lowspeak/issues/new/choose), ideas and questions into
[Discussions](https://github.com/sventmueller/lowspeak/discussions). Security problems follow the
[security policy](SECURITY.md).

## Pull requests

Small fixes can go straight to a pull request. For anything larger, describe the idea in
[Discussions](https://github.com/sventmueller/lowspeak/discussions/new?category=ideas) first, so that no work goes into a
change Lowspeak will not take.

- [docs/development.md](docs/development.md) explains how to build and test Lowspeak.
- `make check` must pass. It covers formatting, clippy without warnings and all tests that need no model, key,
  network or microphone.
- New interface texts need a translation in every language, see
  [Interface language](docs/development.md#interface-language-i18n). The code does not compile without them.
- Keep to one change per pull request.

## Windows and Linux

Ports to other systems are welcome. They keep the premises of the Mac app and what the [README](README.md) promises:

- All program logic is Rust. What does not depend on the system, such as the decisions in `src/app/state.rs`, the
  transcription pipeline, the audio preparation and the texts in `src/i18n`, is shared by every platform, not copied.
- The interface is an item in the system's own tray or status area with a native menu, such as Win32 on Windows and
  StatusNotifierItem with GTK or Qt on Linux; no settings window. An interface built as a web page, in Tauri, Electron
  or a similar shell, is not taken.
- It follows the design guidelines of its platform, as the Mac app follows Apple's
  [Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/): Microsoft's
  [design guidelines for Windows apps](https://learn.microsoft.com/windows/apps/design/guidelines-overview), the
  [GNOME HIG](https://developer.gnome.org/hig/) for GTK and the [KDE HIG](https://develop.kde.org/hig/) for Qt.
- Dictation works as on the Mac: hold a key to talk, lock the recording with a second key or a double press, and the
  text goes wherever the cursor is. fn rarely reaches the system outside a Mac, so the port uses a key its platform
  delivers.
- Every interface text goes through `src/i18n`, in all 35 languages, with right-to-left layout for Arabic and Hebrew.
  The texts follow the dictation language, and in auto-detect the system's.
- Nothing goes over the network beyond what the [README](README.md#privacy-and-security) lists: transcription and
  post-processing with the provider the user chose, model downloads the user asks for, at a fixed version and checked
  against their checksums, and the update check the user allowed. Local mode sends nothing.
- API keys stay in the system's credential store, such as the Windows Credential Manager or the Secret Service on
  Linux, and never reach the log. Dictated text is not written anywhere; the clipboard comes back as it was, and the
  dictation carries the platform's marker for clipboard history apps where one exists.
- The port runs the same checks as `make check` on its own system: formatting, clippy without warnings and all tests
  that need no model, key, network or microphone. CI runs them on every pull request, and a failing step fails the run.
- Installers and updates come only from the releases of this repository and are signed where the platform signs apps,
  such as Authenticode on Windows. A port links to no other download.
- Describe the port in [Discussions](https://github.com/sventmueller/lowspeak/discussions/new?category=ideas) before
  writing code.

## License

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Lowspeak by you, as
defined in the Apache-2.0 license, shall be dual licensed under Apache-2.0 and MIT, without any additional terms or
conditions.
