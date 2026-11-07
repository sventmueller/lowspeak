<p><img src="../../readme-icon.png" alt="Lowspeak 앱 아이콘" width="168" height="168"></p>

# Lowspeak

**커서가 있는 곳에 바로 입력하는 Mac용 받아쓰기: fn을 누른 채 말하고, 놓으면 끝.**

Lowspeak는 메뉴 막대에 있습니다. fn(지구본) 키를 누른 채 말하고 손을 떼면, 입력 중인 앱에 마치 직접 타이핑한 것처럼 말한 내용이 나타납니다. OpenAI나 Groq에서 온라인으로 텍스트로 변환하거나, Whisper 또는 NVIDIA Parakeet으로 완전히 Mac에서 변환할 수 있습니다. Parakeet은 40초 분량의 말을 약 3분의 1초 만에 텍스트로 바꿉니다. 네트워크나 온라인 서비스에 문제가 생기면 설치된 로컬 모델이 대신 처리하므로 받아쓰기를 잃지 않습니다. 자주 쓰는 이름과 용어를 알려 줄 수 있고, 원하면 언어 모델이 불필요한 말을 없애게 할 수 있으며, Shift로 녹음을 잠가 긴 글을 핸즈프리로 받아쓸 수 있습니다. Rust로 진짜 AppKit 컨트롤 위에 만들어졌고, 35개 언어를 지원하며, 음성은 사용자가 선택한 제공업체에만 보냅니다.

[![macOS용 Lowspeak 다운로드](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Lowspeak.dmg 다운로드](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: Apple의 서명과 공증을 거쳤으며, Apple Silicon과 Intel을 탑재한 macOS 14 이상에서 사용할 수 있습니다.

**버그를 발견하셨나요?** [이슈를 열거나](https://github.com/sventmueller/lowspeak/issues/new/choose) Lowspeak 메뉴에서 **도움말 › 문제 보고…** 항목을 선택해 주세요. 사용 중인 버전이 채워진 상태로 양식이 열립니다. Lowspeak가 유용하다면 [GitHub 스타](https://github.com/sventmueller/lowspeak)가 다른 사람들이 Lowspeak를 찾는 데 도움이 됩니다.

[![Rust 2024 에디션](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 이상](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![유니버설: Apple Silicon 및 Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![objc2를 통한 AppKit](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![Metal에서 실행되는 whisper.cpp](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Neural Engine에서 실행되는 Parakeet](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![온라인: OpenAI 또는 Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35개 인터페이스 언어](https://img.shields.io/badge/languages-35-4C9A2A)
![Hardened Runtime으로 서명됨](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![라이선스: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#라이선스)

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
**한국어** ·
[日本語](../ja/README.md) ·
[简体中文](../zh-Hans/README.md) ·
[繁體中文](../zh-Hant/README.md)

## 목차

- [주요 기능](#주요-기능)
- [요구 사항](#요구-사항)
- [설치](#설치)
- [시작하기](#시작하기)
  - [권한](#권한)
  - [변환 방식 선택하기](#변환-방식-선택하기)
  - [메뉴](#메뉴)
  - [빠른 참조](#빠른-참조)
- [받아쓰기](#받아쓰기)
  - [누르고 말하기](#누르고-말하기)
  - [Shift 또는 두 번 눌러 잠그기](#shift-또는-두-번-눌러-잠그기)
  - [텍스트가 입력되는 위치](#텍스트가-입력되는-위치)
  - [최근 받아쓰기](#최근-받아쓰기)
- [텍스트 변환](#텍스트-변환)
  - [나에게 맞는 모델](#나에게-맞는-모델)
  - [온라인: OpenAI와 Groq](#온라인-openai와-groq)
  - [Mac에서: Whisper와 Parakeet](#mac에서-whisper와-parakeet)
  - [온라인 서비스가 실패할 때](#온라인-서비스가-실패할-때)
  - [녹음을 준비하는 방법](#녹음을-준비하는-방법)
  - [모델별 언어](#모델별-언어)
  - [Intel Mac](#intel-mac)
- [어휘](#어휘)
- [후처리](#후처리)
- [마이크](#마이크)
- [API 키](#api-키)
- [알림](#알림)
- [화면 모양과 손쉬운 사용](#화면-모양과-손쉬운-사용)
- [언어](#언어)
- [키와 제스처](#키와-제스처)
- [개인 정보 보호와 보안](#개인-정보-보호와-보안)
- [속도](#속도)
- [알려진 제한 사항](#알려진-제한-사항)
- [문제 해결](#문제-해결)
- [개발자용](#개발자용)
  - [서드파티 구성 요소](#서드파티-구성-요소)
- [라이선스](#라이선스)

## 주요 기능

- **fn을 누른 채 말하고, 놓으면 끝.** 메일, 채팅, 편집기, 터미널, 브라우저 등 앞에 있는 앱의 커서 위치에 텍스트가 입력됩니다. 전환할 윈도우도, 클릭할 버튼도 없습니다.
- **긴 글은 잠가서.** fn을 누른 채 Shift를 누르거나 fn을 두 번 누르면, 키를 누르고 있지 않아도 최대 5분 동안 녹음이 계속됩니다. fn이나 Shift를 한 번 누르면 끝납니다.
- **작업에 맞는 모델.** Mac의 메모리를 쓰지 않고 빠르게 하려면 Groq, 최고의 결과를 원하면 OpenAI의 GPT Transcribe, 오프라인으로 작업하며 음성을 외부에 보내지 않으려면 Mac에서 실행되는 Whisper나 NVIDIA Parakeet. 클릭 한 번으로 전환할 수 있습니다.
- **받아쓰기를 잃지 않습니다.** 네트워크나 온라인 서비스에 문제가 생기면 설치된 로컬 모델이 같은 녹음을 Mac에서 텍스트로 변환합니다.
- **정확한 표기.** 이름과 전문 용어로 된 어휘가 OpenAI, Groq, Whisper와 후처리를 안내합니다.
- **원하면 깔끔한 텍스트로.** 선택 사항인 후처리는 불필요한 말을 없애고, 문법을 고치며, 말하다가 고쳐 말하면 고친 내용만 남깁니다. 받아쓰는 앱의 종류에 맞게 텍스트 서식을 지정할 수도 있습니다.
- **계속 말하십시오.** 이전 받아쓰기가 아직 변환 중이어도 다음 받아쓰기를 시작할 수 있으며, 텍스트는 말한 순서대로 입력됩니다.
- **클립보드는 그대로.** Lowspeak는 클립보드를 통해 붙여넣고, 잠시 후 원래 내용을 되돌려 놓습니다. 최근 받아쓰기 10개는 메뉴에 남아 다시 복사할 수 있습니다.
- **뼛속까지 네이티브.** AppKit 메뉴를 갖춘 진짜 메뉴 막대 항목, 표준 경고와 알림. Electron도, 윈도우도, Dock 아이콘도 없습니다.
- **개인 정보 보호.** 계정도, 원격 측정도 없습니다. 오디오는 사용자가 선택한 제공업체에만 전송되며, 로컬 모드에서는 어디에도 전송되지 않습니다.
- **사용자의 언어로.** 받아쓰기는 수십 개 언어로 할 수 있으며, 메뉴, 대화상자, 알림은 35개 언어로 제공됩니다.

## 요구 사항

- macOS 14 Sonoma 이상. Lowspeak는 macOS 27에서 개발 및 테스트됩니다.
- Apple Silicon 또는 Intel 프로세서를 탑재한 Mac. 로컬 변환은 Apple Silicon용으로 만들어져 Whisper는 GPU에서, Parakeet은 Neural Engine에서 실행됩니다. Intel Mac에서는 둘 다 프로세서에서 실행되어 훨씬 느립니다. [Intel Mac](#intel-mac)을 참고하십시오.
- 앱에 약 17MB, 그리고 다운로드하는 로컬 모델마다 75MB~3GB.
- 온라인 변환에는 [OpenAI](https://platform.openai.com/api-keys) 또는 [Groq](https://console.groq.com/keys)의 API 키가 필요하며, 둘 다 변환한 양만큼 요금을 청구합니다. 로컬 변환에는 계정이 필요하지 않습니다.

## 설치

**디스크 이미지로 설치.** [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)를 내려받아 열고, Lowspeak를 응용 프로그램 폴더로 드래그하십시오. 이 이미지는 Developer ID로 서명되고 Apple의 공증을 거쳤으므로 경고 없이 열립니다.

**업데이트.** 디스크 이미지로 설치한 Lowspeak는 [Sparkle](https://sparkle-project.org)로 스스로 최신 상태를 유지합니다. 두 번째로 실행하면 하루에 한 번 직접 업데이트를 확인해도 되는지 묻습니다. 메뉴의 **업데이트 확인…** 항목은 바로 확인하며, 그 아래의 **자동으로 업데이트 확인**과 **자동으로 업데이트 다운로드 및 설치**로 나중에 선택을 바꿀 수 있습니다. 다른 앱에서 작업하는 동안 확인에서 새 버전이 발견되면 메뉴 막대의 링에 점이 나타나고, **업데이트 확인…** 항목이 업데이트를 앞으로 가져옵니다. Lowspeak는 개발자의 서명이 있는 업데이트만 설치하며, 설정, 키, 모델은 그대로 유지됩니다. 버전 1.0.0에는 업데이트 기능이 없습니다. 한 번만 [릴리스 페이지](https://github.com/sventmueller/lowspeak/releases)에서 더 새로운 버전을 받아 그 위에 설치하십시오. 직접 빌드한 버전에도 업데이트 기능이 없습니다.

**소스에서 빌드.** 필요한 사항은 [Build & Deploy](../../../docs/development.md#build--deploy)를 참고한 다음 아래를 실행하십시오.

```sh
make deploy
```

Lowspeak를 빌드하고, 응용 프로그램 폴더에 있는 사본을 교체한 뒤 실행합니다.

메뉴의 **로그인 시 열기**를 켜면 macOS 자체의 로그인 항목을 통해 Mac과 함께 Lowspeak가 시작됩니다. 한 번에 하나의 사본만 실행되며, 두 번째 사본은 바로 종료됩니다.

## 시작하기

### 권한

처음 실행하면 Lowspeak는 세 가지 권한을 요청합니다.

- **손쉬운 사용**(시스템 설정 › 개인정보 보호 및 보안 › 손쉬운 사용. macOS 27에서는 이 목록의 이름이 “기기 제어 및 데이터 접근”입니다). Lowspeak가 fn 키를 감지하고 ⌘V로 붙여넣는 데 필요합니다. macOS가 목록을 열어 주겠다고 제안하면 Lowspeak의 스위치를 켜십시오. Lowspeak 자체 메시지에는 사용 중인 macOS에 표시되는 목록 이름이 그대로 나옵니다.
- **마이크.** 한 번만 요청합니다. 허용하지 않으면 Lowspeak는 아무것도 녹음하지 않습니다.
- **알림.** 마이크에 신호가 없거나 변환에 실패하는 등 달리 알아차리기 어려운 문제를 알리는 데 사용합니다.

### 변환 방식 선택하기

적어도 한 가지 방식을 설정하십시오. 메뉴에서 언제든지 전환할 수 있습니다.

- **온라인:** **API 키 설정…** 항목을 선택하고 OpenAI 키(`sk-`로 시작) 또는 Groq 키(`gsk_`)를 붙여넣습니다. Lowspeak는 키를 보고 제공업체를 알아냅니다. 그런 다음 **OpenAI** 또는 **Groq** 아래에서 모델을 고릅니다.
- **Mac에서:** **모델 관리** › **Parakeet Ultra 다운로드** 또는 Whisper 모델을 선택한 다음 **로컬** 아래에서 해당 모델을 고릅니다.

[나에게 맞는 모델](#나에게-맞는-모델)이 선택에 도움이 됩니다. 그런 다음 fn을 누른 채 무언가 말하고 손을 떼십시오.

### 메뉴

메뉴 막대의 링을 클릭합니다.

| 항목 | 기능 |
|---|---|
| **최근 받아쓰기** | 최근 텍스트 10개. 클릭하면 다시 클립보드에 복사됩니다 |
| **로컬** | Mac에 있는 모델로, Whisper와 Parakeet으로 묶여 있습니다. 다운로드하지 않은 모델은 흐리게 표시됩니다 |
| **OpenAI**, **Groq** | 온라인 모델. 키가 없으면 흐리게 표시되고 메뉴에 **API 키 설정…** 항목이 나타납니다 |
| **언어** | 받아쓰는 언어 또는 **자동 감지**. 메뉴도 이 언어를 따릅니다 |
| **후처리** | **끔**, **정리** 또는 **서식 지정**. [후처리](#후처리)를 참고하십시오 |
| **어휘…** | 정확하게 표기할 이름과 용어. 입력한 개수가 메뉴에 표시됩니다 |
| **마이크** | **시스템 기본값** 또는 고정 마이크, 그리고 **준비되면 사운드 재생** |
| **모델 관리** | 로컬 모델 다운로드 및 삭제, 진행률 표시 |
| **API 키 설정…** / **API 키 업데이트…** | 키 추가, 교체 또는 제거 |
| **로그인 시 열기** | Mac과 함께 Lowspeak 시작. 켜져 있으면 체크 표시가 나타납니다 |
| **업데이트 확인…** | 지금 새 버전 확인. 그 아래의 두 항목으로 Lowspeak가 하루에 한 번 확인할지, 묻지 않고 업데이트를 설치할지 정합니다. 디스크 이미지로 설치한 앱에만 있습니다 |
| **도움말** | **빠른 참조**, **문제 보고…**, **기능 제안…**, **GitHub의 Lowspeak** |
| **Lowspeak에 관하여** | 버전, 저작권, 프로젝트 페이지, 라이선스 |
| **Lowspeak 종료** | 메뉴가 열려 있을 때 ⌘Q |

**로컬**, **OpenAI**, **Groq** 아래의 체크 표시는 사용 중인 모델을 나타냅니다. 아이콘은 Lowspeak가 무엇을 하고 있는지 보여 줍니다.

| 아이콘 | 의미 |
|---|---|
| 점이 있는 링 | 준비됨 |
| 채워진 원 | fn을 누르고 있는 동안 녹음 중 |
| 둥근 사각형 | 녹음 잠김. 다음에 fn이나 Shift를 누르면 끝납니다 |
| 깜박이는 링 | 텍스트로 변환 중 |
| 오른쪽 위에 점이 있는 링 | 새 버전 대기 중. **업데이트 확인…** 항목을 참고하십시오 |
| ↓ 42% | 모델 다운로드 중 |

Lowspeak에는 설정 윈도우가 없습니다. 모든 것이 이 메뉴에 있으며, Lowspeak는 모든 선택을 기억합니다.

### 빠른 참조

**도움말 › 빠른 참조**는 fn 제스처를 사용자의 언어로 작은 윈도우에 보여 주며, 키보드 설정을 여는 버튼도 있습니다. **문제 보고…** 항목은 Lowspeak와 macOS 버전이 채워진 GitHub 양식을 열고, **기능 제안…** 항목은 아이디어를 위한 토론을, **GitHub의 Lowspeak** 항목은 프로젝트 페이지를 엽니다.

## 받아쓰기

### 누르고 말하기

fn을 누른 채 말하고 손을 뗍니다. 마이크가 소리를 전달하기 시작하는 순간 녹음 아이콘이 나타납니다. 그때 말하기 시작하십시오. 그 전에 한 말은 녹음되지 않습니다. fn을 놓으면 녹음이 끝나고, 잠시 후 텍스트가 입력됩니다.

fn을 짧게 탭하면 아무 일도 일어나지 않으므로 fn의 다른 기능은 그대로 쓸 수 있습니다. 비어 있거나 0.3초 미만인 녹음은 버려집니다.

### Shift 또는 두 번 눌러 잠그기

긴 글을 받아쓸 때는 녹음을 잠가 키를 누르고 있지 않아도 계속되게 합니다. 방법은 세 가지입니다.

- **fn을 누른 채 Shift.** 평소처럼 fn으로 시작하고 녹음 중에 Shift를 누르면 녹음이 잠기고, fn에서 손을 뗄 수 있습니다. 그 전에 한 말도 잃지 않으며, 녹음은 그대로 이어집니다.
- **Shift 다음 fn.** Shift를 누른 채 fn을 누르면 잠긴 상태로 녹음이 시작됩니다.
- **fn 두 번.** fn을 짧게(0.3초 미만) 탭하고 0.4초 안에 다시 누릅니다. 녹음은 처음 누른 시점부터 진행됩니다.

아이콘이 둥근 사각형으로 바뀝니다. 멈추려면 **fn** 또는 **Shift**를 한 번 누르십시오. 텍스트는 평소처럼 입력됩니다. 🌐 키가 이모지 및 기호를 열도록 설정되어 있다면 Shift로 멈추십시오. fn을 짧게 누르면 보기 창이 열립니다.

녹음은 fn을 누르고 있는 동안에는 2분, 잠긴 상태에서는 5분이 지나면 저절로 멈추고, 평소처럼 텍스트로 변환됩니다.

### 텍스트가 입력되는 위치

Lowspeak는 말을 마쳤을 때 앞에 있던 앱의 커서 위치에 클립보드와 ⌘V를 통해 텍스트를 붙여넣습니다. 그 전에 클립보드에 있던 모든 내용을 저장해 두었다가, 앱이 텍스트를 읽고 나서 0.5초 후에, 앱이 끝내 읽지 않으면 10초 후에 되돌려 놓습니다. 단, 그 사이에 다른 앱이 클립보드를 바꿨다면 되돌리지 않습니다. 텍스트에는 임시 데이터라는 표시가 붙으므로, 이 표시를 따르는 클립보드 기록 앱은 텍스트를 기록하지 않습니다.

이전 받아쓰기가 아직 변환 중일 때도 다음 받아쓰기를 시작할 수 있습니다. 텍스트는 말한 순서대로 붙여넣어지며, 녹음 중이거나 Shift를 누르고 있는 동안에는 붙여넣지 않으므로 다음 문장 중간에 붙여넣기가 끼어들 수 없습니다.

손쉬운 사용 권한이 없거나 가상의 ⌘V를 보낼 수 없으면 텍스트는 클립보드에 남고 알림으로 알려 줍니다. ⌘V로 직접 붙여넣으십시오.

### 최근 받아쓰기

메뉴의 **최근 받아쓰기**에는 최근 텍스트 10개가 최신순으로 각각 한 줄로 줄여 표시되며, 포인터를 올리면 전체 텍스트가 보입니다. 클릭하면 클립보드에 복사되므로, 텍스트가 엉뚱한 곳에 입력되었거나 두 번 필요할 때 쓸 수 있습니다. 목록은 메모리에만 있으며 Lowspeak를 종료하면 비워집니다.

## 텍스트 변환

### 나에게 맞는 모델

| 원하는 것 | 선택 | 이유 |
|---|---|---|
| **Mac을 쓰지 않고 빠르게** | **Groq › Whisper Large V3 Turbo** | 가장 빠른 온라인 옵션. 아무것도 다운로드하지 않으며, Mac의 메모리와 디스크 공간을 그대로 둡니다 |
| **최고의 결과** | **OpenAI › GPT Transcribe**(기본값) | Lowspeak가 제공하는 모델 중 가장 정확하며, 어휘를 키워드로 받습니다 |
| **오프라인과 개인 정보 보호** | 유럽 언어는 **로컬 › Parakeet Ultra**, 그 밖의 언어는 **로컬 › Whisper Large v3 Turbo** | 네트워크 없이 작동하며 음성이 Mac 밖으로 나가지 않습니다. Parakeet은 모든 모델 중 가장 빠릅니다 |
| **계정도 비용도 없이** | 아무 **로컬** 모델 | Mac에서 실행되며 요금이 청구되지 않습니다 |
| **안정성과 속도 모두** | 온라인 모델에 더해 Parakeet Ultra 또는 Whisper 모델 설치 | 온라인 서비스가 실패하면 로컬 모델이 대신합니다 |
| **Intel Mac** | **Groq** 또는 **OpenAI** | Intel Mac에서는 로컬 모델이 프로세서에서 실행됩니다. [Intel Mac](#intel-mac)을 참고하십시오 |

로컬 모델은 다운로드 크기만큼 디스크 공간을 차지하고, 선택되어 있는 동안 비슷한 양의 메모리를 사용합니다. 받아쓰기가 지연 없이 시작되도록 Lowspeak는 모델을 로드된 상태로 유지합니다. 메모리가 8GB인 Mac에서는 큰 Whisper 모델보다 Parakeet Ultra나 온라인 모델을 선택하십시오.

### 온라인: OpenAI와 Groq

| 모델 | 제공업체 | 참고 |
|---|---|---|
| **GPT Transcribe**(기본값) | OpenAI | 가장 정확하며, 어휘를 키워드로 받습니다 |
| **Whisper** | OpenAI | 원래의 Whisper 모델 |
| **Whisper Large V3** | Groq | 매우 빠름 |
| **Whisper Large V3 Turbo** | Groq | 가장 빠른 온라인 모델 |

녹음은 업로드 전에 AAC로 압축되어 말 1초당 약 6KB가 되므로, 느린 연결에서도 받아쓰기를 할 수 있습니다. fn을 누르고 있는 동안 Lowspeak가 미리 제공업체와의 연결을 열어 두므로, 손을 떼는 순간 업로드가 시작됩니다.

### Mac에서: Whisper와 Parakeet

| 모델 | 크기 | 실행 위치 | 언어 |
|---|---|---|---|
| **Parakeet Ultra** | 약 630MB | Neural Engine | 유럽 언어 25개 |
| **Parakeet v2** | 약 600MB | Neural Engine | 영어 |
| **Whisper Large v3 Turbo** | 약 1.6GB | GPU(Metal) | 약 99개 |
| **Whisper Large v3** | 약 3GB | GPU(Metal) | 약 99개 |
| **Whisper Medium** | 약 1.5GB | GPU(Metal) | 약 99개 |
| **Whisper Small** | 약 470MB | GPU(Metal) | 약 99개 |
| **Whisper Base** | 약 150MB | GPU(Metal) | 약 99개 |
| **Whisper Tiny** | 약 75MB | GPU(Metal) | 약 99개 |

**모델 관리**에서 모델을 다운로드하고 삭제합니다. 다운로드하는 동안 메뉴 막대에 진행률이 표시되며, Lowspeak를 종료하면 다운로드가 취소됩니다. 모델을 처음 사용할 때는 사용자의 Mac에 맞게 준비하는 과정을 거칩니다. Parakeet Ultra는 약 10초, Whisper는 최대 30초가 걸리며, 그 후에는 1초도 안 되어 로드됩니다. 선택한 모델이 삭제되면 Lowspeak는 설치된 다른 모델로 전환합니다.

**Parakeet**은 가장 빠른 선택으로, 40초 분량의 말을 약 0.3초 만에 변환합니다. Lowspeak는 [FluidAudio](https://github.com/FluidInference/FluidAudio)와 같은 모델과 디코딩 방식으로 Parakeet을 실행하고 모델을 FluidAudio의 폴더(`~/Library/Application Support/FluidAudio/Models`)에 저장하므로, FluidAudio 기반의 앱과 모델을 공유합니다. 15초보다 긴 녹음은 서로 겹치는 구간으로 나누어 변환한 뒤, 이음매에서 단어를 잃지 않도록 합칩니다. **Whisper**는 더 많은 언어를 이해하고 어휘를 읽습니다. 큰 모델이 가장 정확하며, 작은 모델은 영어 외의 언어에서 훨씬 약합니다. Whisper 모델은 `~/Library/Application Support/Lowspeak/Models`에 저장됩니다.

### 온라인 서비스가 실패할 때

네트워크가 끊겼거나, 제공업체에 문제가 있거나, 키가 잘못되어 온라인 변환이 실패하면 설치된 로컬 모델이 같은 녹음을 변환합니다. **로컬**에서 선택한 모델을 쓰고, 없으면 Parakeet Ultra, 그다음 Whisper Large v3 Turbo, Large v3, 더 작은 Whisper 모델 순이며, Parakeet v2가 마지막입니다. 알림은 한 번만 표시되며, 그 사이에 온라인 받아쓰기가 한 번 성공한 뒤에야 다시 표시됩니다.

대체할 로컬 모델이 있으면 Lowspeak는 온라인 요청에 20초를 줍니다. 없으면 각각 최대 1분씩 세 번 시도합니다. 온라인 모드에서는 Lowspeak가 실행할 때 대체용 Parakeet 모델을 한 번 준비해 두므로, 첫 번째 대체 변환이 그 준비를 기다리지 않습니다.

### 녹음을 준비하는 방법

Lowspeak는 마이크 고유의 샘플 레이트로 녹음하고, 녹음을 멈추면 모델이 요구하는 16kHz로 변환합니다. 변환 전에 80Hz 미만의 낮은 웅웅거림을 없애고, 시작과 끝의 무음을 잘라 내고, 음량을 고르게 맞추므로 작게 말하는 사람과 크게 말하는 사람이 비슷하게 처리됩니다. 말소리가 없는 녹음은 아예 전송하지 않습니다. 로컬 모드에서는 Parakeet이 먼저 손대지 않은 녹음을 받고, 거기서 텍스트가 나오지 않을 때만 전처리한 녹음을 받습니다.

### 모델별 언어

**언어**에서 선택한 언어는 온라인 모델과 Whisper에 전달됩니다. **자동 감지**를 선택하면 모델이 스스로 언어를 판단합니다.

| 모델 | 받아쓸 수 있는 언어 |
|---|---|
| GPT Transcribe, Whisper(OpenAI, Groq, Mac에서) | 약 99개. 메뉴의 35개 언어 모두 포함 |
| Parakeet Ultra | 불가리아어, 크로아티아어, 체코어, 덴마크어, 네덜란드어, 영어, 에스토니아어, 핀란드어, 프랑스어, 독일어, 그리스어, 헝가리어, 이탈리아어, 라트비아어, 리투아니아어, 몰타어, 폴란드어, 포르투갈어, 루마니아어, 러시아어, 슬로바키아어, 슬로베니아어, 스페인어, 스웨덴어, 우크라이나어 |
| Parakeet v2 | 영어 |

Parakeet은 언어를 스스로 감지하며 메뉴의 선택을 무시합니다. **繁體中文**을 선택하면 GPT Transcribe가 번체자로 씁니다. Whisper는 중국어를 간체자와 번체자 중 어느 쪽으로든 쓰고, 포르투갈어는 지역 변형을 구분하지 않고 씁니다.

### Intel Mac

Lowspeak는 유니버설 앱이며 macOS 14가 설치된 Intel Mac에서도 실행됩니다. 온라인 기능은 모두 똑같이 작동합니다. 하지만 로컬 모델은 그곳에서 본래 염두에 둔 하드웨어를 쓸 수 없습니다. Whisper는 GPU 대신 프로세서에서, Parakeet은 Neural Engine 대신 프로세서에서 실행됩니다. 둘 다 작동하지만 Apple Silicon보다 몇 배나 오래 걸리며, 큰 Whisper 모델은 받아쓰기에 쓰기에는 너무 느립니다. Intel Mac에서는 온라인으로 변환하고, Whisper Small이나 Parakeet Ultra를 [대체 모델](#온라인-서비스가-실패할-때)로 설치해 두십시오.

## 어휘

**어휘…** 항목에는 Lowspeak가 정확하게 표기해야 할 이름과 용어를 쉼표로 구분해 입력합니다. 제품 이름, 동료의 이름, 전문 용어 등입니다. 최대 100개, 각각 최대 60자이며, `<`와 `>`는 제거됩니다.

- **GPT Transcribe**는 키워드로 받습니다.
- **Whisper**(OpenAI, Groq, Mac에서)는 녹음 앞에 놓이는 힌트로 최대 500자까지 읽습니다.
- **후처리**에는 적힌 그대로 정확하게 표기하라는 지시가 전달됩니다.
- **Parakeet**은 사용하지 않습니다.

## 후처리

변환 후에 언어 모델이 텍스트를 다듬을 수 있습니다. **후처리**에서 선택하십시오.

- **끔**(기본값) — 변환된 그대로의 텍스트.
- **정리** — “음”, “그러니까”, “기본적으로” 같은 불필요한 말을 없애고, 문법과 문장 부호를 고치며, 말하다가 고쳐 말하면 고친 내용만 남깁니다. 아무것도 덧붙이지 않으며 의미를 바꾸지 않습니다.
- **서식 지정** — 정리와 같은 작업에 더해, 받아쓰는 앱에 맞게 텍스트를 조정합니다. 코드 편집기에서는 코드 스타일의 용어로, 터미널에서는 명령어로, 채팅 앱에서는 편한 말투로, 이메일에서는 격식 있는 말투로 바꿉니다.

Groq 키가 있으면 후처리에 Groq(`openai/gpt-oss-120b`)를, 없으면 OpenAI(`gpt-4o-mini`)를 사용하며, 로컬로 변환할 때도 마찬가지입니다. **언어**에서 언어를 선택했다면 그 언어를 유지합니다. 8초보다 오래 걸리거나, 실패하거나, 돌아온 답변이 중간에 잘렸거나 말한 내용보다 훨씬 길면 Lowspeak는 처리하지 않은 텍스트를 붙여넣습니다. 제공업체가 요청을 거부하거나 키가 설정되어 있지 않으면 알림으로 한 번 알려 줍니다.

## 마이크

**마이크**에서 고정 마이크 또는 **시스템 기본값**을 선택합니다. Lowspeak는 장치를 기억하며, 연결되어 있지 않은 동안에는 메뉴에 그 마이크가 **(연결되지 않음)** 상태로 표시되고 시스템 기본값으로 녹음합니다.

마이크는 받아쓰는 동안에만 켜집니다. 준비되는 데 내장 마이크는 약 0.07초, Studio Display는 약 0.5초, AirPods는 먼저 헤드셋 모드로 전환하므로 1~2초가 걸립니다. 소리가 들어오면 녹음 아이콘이 나타나며, **준비되면 사운드 재생**을 켜면 그 순간 짧은 소리가 납니다. 일부 스피커에서는 소리 자체가 0.5초 늦게 나오기 때문에 기본적으로 꺼져 있습니다.

마이크가 5초 동안 무음만 전달하면 Lowspeak가 알려 줍니다. 다른 기기가 AirPods의 마이크를 쓰고 있을 때 이런 일이 생깁니다.

## API 키

**API 키 설정…** 항목은 키를 입력받고 그 앞부분으로 제공업체를 알아냅니다. `sk-`는 OpenAI, `gsk_`는 Groq입니다. 다른 형식의 키는 설명과 함께 거부됩니다. 제공업체마다 키를 하나씩 입력하십시오. 두 제공업체의 키가 모두 있으면 모든 온라인 모델을 사용할 수 있습니다. **API 키 업데이트…** 항목에서 키를 교체하거나, **모든 키 제거**로 키를 삭제합니다.

키는 로그인 키체인에 `com.lowspeak.app`이라는 이름으로 저장됩니다. 이전 버전의 키 파일(`~/Library/Application Support/Lowspeak/.api-key`)은 다음 실행 시 키체인으로 옮겨지고, 모든 키를 키체인에서 다시 읽을 수 있는지 확인한 후 삭제됩니다.

## 알림

Lowspeak는 백그라운드에서 작동하므로 문제를 알림으로 알려 줍니다.

| 알림 | 무슨 일이 있었는지 |
|---|---|
| **녹음 실패** | 마이크 접근 권한이 없거나, 연결된 마이크가 없거나, 마이크를 시작할 수 없음 |
| **마이크 신호 없음** | 5초 동안 무음. [마이크](#마이크)를 참고하십시오 |
| **손쉬운 사용 권한 필요** | 텍스트는 클립보드에 있습니다. 자동으로 붙여넣으려면 손쉬운 사용을 허용하십시오 |
| **붙여넣기 실패** | 텍스트는 클립보드에 있습니다. ⌘V로 붙여넣으십시오 |
| **텍스트 변환 실패** | 원인과 함께 표시: 키 없음, 거부된 키, 요청 한도, 서버 또는 네트워크 오류, 로컬 모델 없음 |
| **이 Mac에서 텍스트로 변환함** | 온라인 서비스가 실패해 로컬 모델이 대신 변환함 |
| **후처리 실패** | 제공업체가 후처리를 거부하거나 키가 설정되지 않음. 텍스트는 후처리 없이 붙여넣었습니다 |
| **API 키 없음** | 제공업체의 키 없이 온라인 모델을 선택함 |
| **다운로드 실패**, **삭제 실패**, **삭제할 수 없음** | 로컬 모델 관리가 실패했거나 다른 작업이 아직 진행 중이었음 |
| **로그인 항목 오류**, **API 키 오류** | macOS 또는 키체인이 변경을 거부함 |

알림에는 받아쓴 텍스트가 절대 들어가지 않습니다. 실패하는 온라인 서비스와 거부하는 후처리 제공업체는 받아쓰기 때마다가 아니라, 다시 작동할 때까지 한 번만 알립니다.

## 화면 모양과 손쉬운 사용

- **메뉴 막대 아이콘**은 템플릿 이미지로, 밝은 메뉴 막대, 어두운 메뉴 막대, 색조가 입혀진 메뉴 막대에 맞춰집니다.
- **VoiceOver**는 아이콘을 “Lowspeak”로 읽으며, 메뉴, 경고, 알림은 표준 macOS 컨트롤입니다.
- **키를 계속 누르고 있을 필요가 없습니다.** Shift나 fn 두 번 누르기로 녹음을 잠그면, 다음에 누를 때 끝납니다.
- **빠른 참조**는 레이블과 버튼만 있는 단순한 윈도우로, 키보드로 접근할 수 있으며 Esc로 닫힙니다.

알려진 부족한 점은 [ACCESSIBILITY.md](../../../ACCESSIBILITY.md)에 정리되어 있습니다. VoiceOver는 녹음이 시작될 때 알리지 않으며, Lowspeak는 아직 VoiceOver나 스위치 제어로 확인되지 않았습니다.

## 언어

메뉴, 대화상자, 알림이 35개 언어로 번역되어 있습니다.

아랍어, 카탈루냐어, 중국어(간체), 중국어(번체), 크로아티아어, 체코어, 덴마크어, 네덜란드어, 영어, 핀란드어, 프랑스어, 독일어, 그리스어, 히브리어, 힌디어, 헝가리어, 인도네시아어, 이탈리아어, 일본어, 한국어, 말레이어, 노르웨이어, 폴란드어, 포르투갈어(브라질), 포르투갈어(포르투갈), 루마니아어, 러시아어, 슬로바키아어, 슬로베니아어, 스페인어, 스웨덴어, 태국어, 튀르키예어, 우크라이나어, 베트남어.

**언어**에서 선택한 언어가 메뉴의 언어도 됩니다. **자동 감지**를 선택하면 메뉴는 **시스템 설정 › 일반 › 언어 및 지역**의 언어 순서를 따르며, 번역이 없으면 영어로 표시됩니다. 메뉴 용어는 Apple의 공식 번역을 따르므로 Lowspeak는 Mac의 다른 부분과 같은 말로 표시됩니다. 영어 외의 번역은 기계 번역이며 아직 원어민의 검토를 받지 않았습니다. 수정 사항은 [이슈](https://github.com/sventmueller/lowspeak/issues/new/choose)로 보내 주시면 환영합니다.

## 키와 제스처

| 동작 | 키 |
|---|---|
| 누르고 있는 동안 받아쓰기 | fn 길게 누르기 |
| 진행 중인 녹음 잠그기 | fn을 누른 채 Shift |
| 잠긴 녹음 시작 | Shift+fn 또는 fn 빠르게 두 번 |
| 잠긴 녹음 끝내기 | fn 또는 Shift |
| Lowspeak 종료(메뉴가 열려 있을 때) | ⌘Q |
| 빠른 참조 닫기 | Esc |

Lowspeak는 자신이 사용하는 fn 누름이 다른 앱에 전달되지 않도록 막습니다. Shift는 평소처럼 그대로 전달됩니다.

## 개인 정보 보호와 보안

Lowspeak는 fn을 누르고 있는 동안 사용자가 하는 말을 듣고, 어떤 앱에든 입력할 수 있으며, API 키를 보관합니다. 그래서 이것들로 하는 일을 최소한으로 줄입니다.

- **음성은 한 곳으로만 갑니다.** 온라인 모드에서는 압축된 녹음이 어휘와 함께 사용자가 선택한 제공업체(OpenAI 또는 Groq)에 전송됩니다. 로컬 모드에서는 아무것도 Mac 밖으로 나가지 않습니다. 온라인 요청이 실패하면 로컬 모델이 같은 녹음을 Mac에서 변환합니다.
- **후처리**를 켜면 변환된 텍스트와 어휘가 Groq 또는 OpenAI에 전송됩니다. 서식 지정 모드에서는 받아쓰는 앱의 종류(코드 편집기, 터미널, 채팅 또는 이메일)도 함께 보내며, 앱의 이름은 보내지 않습니다.
- **원격 측정도, 계정도 없습니다.** 변환과 후처리 외에 Lowspeak가 온라인에 접속하는 경우는, 온라인 모드이거나 후처리가 켜져 있을 때 fn을 누르고 있는 동안 제공업체와의 연결을 열 때, 사용자가 요청한 모델을 Hugging Face에서 HTTPS로 다운로드할 때, 그리고 허용한 경우 하루에 한 번 GitHub에서 새 버전을 확인할 때뿐입니다. 모델은 고정된 버전으로 받으며, 모든 파일을 체크섬과 대조해 확인합니다. 버전 확인 요청에는 Lowspeak 버전 외에는 아무것도 담기지 않습니다. **도움말 › 문제 보고…** 항목은 브라우저에서 GitHub 양식을 엽니다. 그 주소에는 Lowspeak 버전과 macOS 버전만 담기며, 받아쓴 내용은 담기지 않습니다.
- **API 키**는 로그인 키체인에 저장됩니다. 키는 해당 제공업체에만 전송되며, 로그에 무언가를 기록하기 전에 오류 메시지에서 제거됩니다.
- **받아쓴 텍스트는 저장되지 않습니다.** 최근 받아쓰기 10개는 **최근 받아쓰기**를 위해 메모리에 남아 있다가 Lowspeak를 종료하면 사라집니다. 로그 파일에는 길이와 소요 시간만 기록되며, 텍스트는 절대 기록되지 않습니다.
- **클립보드**는 붙여넣기 전에 저장되고, 앱이 텍스트를 읽은 후에 복원됩니다. 단, 그 사이에 다른 앱이 클립보드를 바꿨다면 복원하지 않습니다. 받아쓴 텍스트에는 임시 데이터라는 표시가 붙으므로, 이 표시를 따르는 클립보드 기록 앱은 이를 건너뜁니다.
- **코드 서명**에 Hardened Runtime을 사용하며, 앱은 마이크, 손쉬운 사용, 알림, 네트워크만 요청합니다. 업데이트에는 개발자의 서명이 있어야 하며, 그렇지 않으면 Sparkle이 거부합니다.

보안 취약점을 보고하려면 [보안 정책](../../../SECURITY.md)을 따르십시오.

## 속도

Apple Silicon에서 측정했습니다.

- **Parakeet Ultra**는 40초 분량의 말을 약 0.3초 만에 변환합니다.
- **온라인:** 잠시 쉬었다가 보내는 첫 요청은 Groq에서 약 0.4초가 걸립니다. 아직 말하는 동안 Lowspeak가 연결을 열어 두기 때문입니다. 이후 요청은 그 연결을 재사용하며, 제공업체가 지원하면 HTTP/3을 사용합니다.
- **업로드 크기:** 48kbit/s AAC로 말 1초당 약 6KB.
- **마이크 준비:** 내장 마이크는 약 0.07초, Studio Display는 0.5초, AirPods는 1~2초.

## 알려진 제한 사항

- **Intel Mac의 로컬 모델**은 프로세서에서 실행되므로 훨씬 느립니다. Intel Mac에서는 온라인 변환이 더 나은 선택입니다.
- **마이크가 준비되기 전에 한 말은 사라집니다.** 특히 AirPods 사용 시에는 녹음 아이콘이 나타날 때까지 기다리십시오.
- **fn 키를 다른 프로그램이 가져갈 수 있습니다.** macOS의 🌐 키 설정, Warp 또는 macOS 받아쓰기가 먼저 fn을 가져가면 Lowspeak는 fn을 감지하지 못합니다. [문제 해결](#문제-해결)을 참고하십시오.
- **Parakeet**은 어휘와 언어 선택을 무시하며, 유럽 언어만 압니다.
- **문자와 변형:** 번체 중국어로 쓰도록 지정할 수 있는 모델은 GPT Transcribe뿐이며, 브라질 포르투갈어가 아닌 유럽 포르투갈어로 쓰도록 지정할 수 있는 모델은 없습니다.
- **시간 제한:** fn을 누르고 있는 동안 2분, 잠긴 상태에서 5분.
- **마이크 모드:** 제어 센터는 Lowspeak에 대해서도 음성 분리와 와이드 스펙트럼을 제공하며 활성 상태로 표시하지만, Lowspeak는 음성 처리 없이 녹음하므로 이 선택은 효과가 없습니다.
- **최근 받아쓰기**는 메모리에만 있으며 종료하면 사라집니다.
- **의도된 설계:** 설정 윈도우 없음, Dock 아이콘 없음, App Store에서 제공하지 않음.

## 문제 해결

**fn을 누르고 있어도 아무 일도 일어나지 않습니다.** 세 가지를 확인하십시오. **시스템 설정 › 개인정보 보호 및 보안 › 손쉬운 사용**(macOS 27에서는 **기기 제어 및 데이터 접근**)에서 Lowspeak가 허용되어 있는지, **시스템 설정 › 키보드 › 🌐 키를 누를 때 실행할 동작**이 **안 함**으로 설정되어 있는지, 그리고 fn을 가져가는 다른 앱이 없는지 확인하십시오. Warp는 기본적으로 fn을 가져갑니다. `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'`로 끄고 Warp를 다시 시작하십시오. macOS 받아쓰기도 fn으로 시작하도록 설정되어 있을 수 있습니다.

**업데이트 후 Lowspeak가 더 이상 fn에 반응하지 않습니다.** macOS는 손쉬운 사용 권한을 앱의 서명에 연결합니다. 직접 빌드한 앱에서 다운로드한 앱으로 바꾼 경우처럼 서명이 달라졌다면, 손쉬운 사용 목록(macOS 27에서는 기기 제어 및 데이터 접근)에서 Lowspeak를 제거한 다음 다시 추가하십시오.

**첫 단어가 빠집니다.** 녹음 아이콘이 나타난 뒤에 말하기 시작하십시오. AirPods는 마이크가 준비되는 데 1~2초가 걸립니다. **마이크**에서 내장 마이크나 USB 마이크를 선택하면 거의 즉시 준비됩니다.

**“마이크 신호 없음.”** 마이크가 5초 동안 소리를 전달하지 않았습니다. AirPods의 경우 다른 기기가 그 마이크를 쓰고 있을 수 있습니다. 내장 마이크로 전환하거나 AirPods 연결을 끊었다가 다시 연결하십시오. MacBook 덮개를 닫아도 내장 마이크가 음소거됩니다.

**받아쓴 결과가 비어 있습니다.** 입력 음량이 너무 낮을 수 있습니다. Teams, Zoom, Meet 같은 앱은 통화 중에 입력 음량을 바꾸며, macOS는 이 음량을 장치별로 유지합니다. **시스템 설정 › 사운드 › 입력**에서 높이십시오.

**텍스트가 엉뚱한 곳에 입력되었습니다.** 메뉴의 **최근 받아쓰기**에서 다시 클립보드에 복사할 수 있습니다.

**“API 키 없음.”** 메뉴에서 **API 키 설정…** 항목을 선택하거나 로컬 모델로 전환하십시오.

**첫 로컬 변환이 오래 걸립니다.** 각 모델은 처음 사용할 때 사용자의 Mac에 맞게 준비됩니다. 그 후에는 1초도 안 되어 로드됩니다.

**Lowspeak가 실행 직후 종료됩니다.** 다른 Lowspeak가 이미 실행 중입니다. 먼저 그것을 종료하십시오.

**알림이 나타나지 않습니다.** **시스템 설정 › 알림 › Lowspeak**에서 허용하십시오.

**그 밖의 문제.** `~/Library/Application Support/Lowspeak/`에 있는 로그(`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`)에 무슨 일이 있었는지 기록되며, 받아쓴 텍스트는 포함되지 않습니다. 관련된 줄을 [문제 보고](https://github.com/sventmueller/lowspeak/issues/new/choose)에 첨부하십시오.

## 개발자용

빌드, 테스트, 코드 구조와 코드가 따르는 규칙은 [docs/development.md](../../../docs/development.md)에, 기여 방법은 [CONTRIBUTING.md](../../../CONTRIBUTING.md)에 설명되어 있습니다(모두 영어).

### 서드파티 구성 요소

| 구성 요소 | 버전 | 용도 | 라이선스 |
|---|---|---|---|
| [whisper-rs](https://codeberg.org/tazz4843/whisper-rs)를 통한 [whisper.cpp](https://github.com/ggml-org/whisper.cpp) | 0.16 | GPU(Metal)에서 실행되는 로컬 Whisper | MIT; whisper-rs는 Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet 디코딩과 긴 녹음 병합, `src/transcription/parakeet`에서 Rust로 옮김 | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) 및 프레임워크 크레이트 | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation 및 기타 시스템 프레임워크 | MIT; Zlib, Apache-2.0 or MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | CoreAudio를 통한 마이크 녹음 | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | 16kHz로 리샘플링 | MIT |
| rustls를 사용하는 [ureq](https://github.com/algesten/ureq) | 3.5 | 모델 다운로드 | MIT or Apache-2.0; rustls와 ring은 ISC 및 Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | 다운로드한 모델의 체크섬 | MIT or Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | 키체인의 API 키 | MIT or Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, 오류, Parakeet의 어휘 | MIT or Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | 디스크 이미지로 설치한 앱의 업데이트 | MIT, 일부는 BSD 및 zlib 라이선스 |

앱에 포함된 모든 구성 요소의 라이선스 텍스트는 `Contents/Resources/Acknowledgements.txt`로 앱과 함께 제공됩니다.

모델은 Lowspeak에 포함되어 있지 않으며, 사용자가 요청하면 Lowspeak가 다운로드합니다.

| 모델 | 출처 | 라이선스 |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp)(ggml 형식의 OpenAI Whisper) | MIT |
| Parakeet v2, Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference)가 CoreML로 변환한 NVIDIA의 Parakeet TDT | CC-BY-4.0 |

## 라이선스

Lowspeak는 다음 둘 중 하나의 라이선스로 제공됩니다.

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- MIT License ([LICENSE-MIT](../../../LICENSE-MIT))

사용자의 선택에 따릅니다.

별도로 명시하지 않는 한, Apache-2.0 라이선스에서 정의하는 바와 같이 사용자가 Lowspeak에 포함하도록 의도적으로 제출한 기여물은 추가 약관이나 조건 없이 위와 같이 이중 라이선스로 제공됩니다.
