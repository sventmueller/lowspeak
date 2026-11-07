<p><img src="../../readme-icon.png" alt="Lowspeak App 圖像" width="168" height="168"></p>

# Lowspeak

**在游標所在處直接輸入的 Mac 聽寫工具：按住 fn、說話、放開。**

Lowspeak 常駐在選單列。按住 fn（地球）鍵說話，放開後，你說的話就會出現在你正在輸入的 App 中，就像你親手打字一樣。它可以透過 OpenAI 或 Groq 線上轉錄，也可以用 Whisper 或 NVIDIA Parakeet 完全在你的 Mac 上轉錄；Parakeet 能在約三分之一秒內把 40 秒的語音轉成文字。當網路或線上服務出問題時，已安裝的本機模型會接手，所以聽寫不會遺失。你可以教它認識你常用的名稱和術語，也可以視需要讓語言模型刪除贅詞，還能用 Shift 鎖定錄音，以免持方式聽寫較長的文字。它以 Rust 寫成，建立在原生 AppKit 控制項之上，支援 35 種語言，只會把你的聲音傳送給你選擇的服務提供者。

[![下載 Mac 版 Lowspeak](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[下載 Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**：已由 Apple 簽署並公證，適用於 macOS 14 或以上版本，支援 Apple 晶片與 Intel。

**發現 Bug 了嗎？** 請[提交 Issue](https://github.com/sventmueller/lowspeak/issues/new/choose)，或在 Lowspeak 的選單中選擇 **輔助說明 › 回報問題…**：表單開啟時會自動填好你的版本。如果 Lowspeak 對你有用，在 [GitHub 上按個 Star](https://github.com/sventmueller/lowspeak) 能幫助更多人找到它。

[![Rust 2024 版次](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 或以上版本](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![通用：Apple 晶片與 Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![透過 objc2 使用 AppKit](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![在 Metal 上執行的 whisper.cpp](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![在神經網路引擎上執行的 Parakeet](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![透過 OpenAI 或 Groq 線上轉錄](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 種介面語言](https://img.shields.io/badge/languages-35-4C9A2A)
![以 Hardened Runtime 簽署](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![授權條款：MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#授權條款)

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
[한국어](../ko/README.md) ·
[日本語](../ja/README.md) ·
[简体中文](../zh-Hans/README.md) ·
**繁體中文**

## 目錄

- [功能一覽](#功能一覽)
- [系統需求](#系統需求)
- [安裝](#安裝)
- [開始使用](#開始使用)
  - [權限](#權限)
  - [選擇轉錄方式](#選擇轉錄方式)
  - [選單](#選單)
  - [快速參考](#快速參考)
- [聽寫](#聽寫)
  - [按住說話](#按住說話)
  - [用 Shift 或按兩下鎖定](#用-shift-或按兩下鎖定)
  - [文字會貼到哪裡](#文字會貼到哪裡)
  - [最近的聽寫](#最近的聽寫)
- [轉錄](#轉錄)
  - [哪個模型適合你](#哪個模型適合你)
  - [線上：OpenAI 與 Groq](#線上openai-與-groq)
  - [在 Mac 上：Whisper 與 Parakeet](#在-mac-上whisper-與-parakeet)
  - [線上服務失敗時](#線上服務失敗時)
  - [錄音的前置處理](#錄音的前置處理)
  - [各模型支援的語言](#各模型支援的語言)
  - [Intel Mac](#intel-mac)
- [詞彙表](#詞彙表)
- [後處理](#後處理)
- [麥克風](#麥克風)
- [API 密鑰](#api-密鑰)
- [通知](#通知)
- [外觀與輔助使用](#外觀與輔助使用)
- [語言](#語言)
- [按鍵與手勢](#按鍵與手勢)
- [隱私與安全性](#隱私與安全性)
- [速度](#速度)
- [已知限制](#已知限制)
- [疑難排解](#疑難排解)
- [給開發者](#給開發者)
  - [第三方元件](#第三方元件)
- [授權條款](#授權條款)

## 功能一覽

- **按住 fn、說話、放開。** 文字會輸入到最前方 App 的游標位置：郵件、聊天、編輯器、終端機或瀏覽器都行。不必切換視窗，也不必按任何按鈕。
- **長篇文字可鎖定錄音。** 按住 fn 時按下 Shift，或按兩下 fn，錄音就會在不按住任何鍵的情況下持續進行，最長五分鐘。按一下 fn 或 Shift 即可結束。
- **依工作選對模型。** 想要速度又不佔用 Mac 的記憶體，選 Groq；想要最好的結果，選 OpenAI 的 GPT Transcribe；想離線工作並讓聲音留在本機，選在 Mac 上執行的 Whisper 或 NVIDIA Parakeet。按一下即可切換。
- **聽寫不會遺失。** 當網路或線上服務出問題時，已安裝的本機模型會在你的 Mac 上轉錄同一段錄音。
- **你的用詞，正確拼寫。** 由名稱和專業術語組成的詞彙表會引導 OpenAI、Groq、Whisper 和後處理。
- **需要時就給你整潔的文字。** 選用的後處理會刪除贅詞、修正文法，並在你改口時只保留更正後的內容；也可以依你聽寫時所用 App 的類型來調整文字格式。
- **繼續說下去。** 上一段聽寫還在轉錄時，就能開始下一段；文字會依你說話的順序送達。
- **剪貼板還是你的。** Lowspeak 透過剪貼板貼上，片刻後再放回原本的內容。最近十筆聽寫會保留在選單中，可再次拷貝。
- **徹頭徹尾的原生。** 真正的選單列項目，搭配 AppKit 選單、標準的提示框和通知。沒有 Electron，沒有視窗，也沒有 Dock 圖像。
- **注重隱私。** 不需帳號，沒有遙測。音訊只會傳送給你選擇的服務提供者，本機模式下則完全不會傳送。
- **使用你的語言。** 聽寫支援數十種語言，選單、對話框和通知則提供 35 種語言。

## 系統需求

- macOS 14 Sonoma 或以上版本。Lowspeak 在 macOS 27 上開發與測試。
- 搭載 Apple 晶片或 Intel 處理器的 Mac。本機轉錄是為 Apple 晶片打造：Whisper 在 GPU 上執行，Parakeet 在神經網路引擎上執行；在 Intel Mac 上兩者都在處理器上執行，速度慢上許多，請參閱 [Intel Mac](#intel-mac)。
- App 本身約 17 MB，每下載一個本機模型另需 75 MB 到 3 GB。
- 線上轉錄需要 [OpenAI](https://platform.openai.com/api-keys) 或 [Groq](https://console.groq.com/keys) 的 API 密鑰；兩者都會依你轉錄的量收費。本機轉錄不需要帳號。

## 安裝

**從磁碟映像檔安裝。** 下載 [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)，打開後將 Lowspeak 拖到「應用程式」檔案夾。映像檔已用 Developer ID 簽署並由 Apple 公證，所以打開時不會出現警告。

**更新。** 從磁碟映像檔安裝的 Lowspeak 會透過 [Sparkle](https://sparkle-project.org) 自行保持最新。第二次啟動時，它會詢問是否可以每天自動檢查一次更新。選單中的 **檢查更新項目…** 會立即檢查；之後若想更改選擇，可使用其下方的 **自動檢查更新項目** 和 **自動下載並安裝更新項目**。如果在你使用其他 App 時檢查到新版本，選單列中的圓環上會出現一個圓點，**檢查更新項目…** 會將更新帶到最前面。Lowspeak 只會安裝帶有開發者簽章的更新；你的設定、密鑰和模型都會保留。1.0.0 版本沒有更新功能：請從[發行頁面](https://github.com/sventmueller/lowspeak/releases)下載較新的版本，直接安裝在它上面，只需這一次。自行建置的版本同樣沒有更新功能。

**從原始碼安裝。** 先在「[建置與部署](../../../docs/development.md#build--deploy)」中查看前置條件，然後：

```sh
make deploy
```

這會建置 Lowspeak、取代「應用程式」檔案夾中的副本，然後啟動它。

選單中的 **在登入時打開** 會透過 macOS 本身的登入項目，讓 Lowspeak 隨 Mac 一起啟動。同一時間只會執行一個 Lowspeak；第二個會立即結束。

## 開始使用

### 權限

第一次啟動時，Lowspeak 會要求三項權限：

- **輔助使用**（系統設定 › 隱私權與安全性 › 輔助使用；在 macOS 27 中，此列表名為「裝置控制和資料取用」）。Lowspeak 需要它來偵測 fn 鍵，並用 ⌘V 貼上。macOS 會提議為你打開列表；請開啟 Lowspeak 的開關。Lowspeak 本身的訊息會依你的 macOS 所顯示的名稱來稱呼這個列表。
- **麥克風。** 只會詢問一次；沒有此權限，Lowspeak 什麼都錄不到。
- **通知。** Lowspeak 用它來告訴你原本不會注意到的問題，例如麥克風沒有聲音或轉錄失敗。

### 選擇轉錄方式

至少設定一種方式；你可以隨時在選單中切換。

- **線上：** 選擇 **設定 API 密鑰…**，貼上 OpenAI 的密鑰（以 `sk-` 開頭）或 Groq 的密鑰（`gsk_`）。Lowspeak 會依密鑰辨識服務提供者。接著在 **OpenAI** 或 **Groq** 下選擇模型。
- **在你的 Mac 上：** 選擇 **管理模型** › **下載 Parakeet Ultra** 或某個 Whisper 模型，然後在 **本機** 下選擇它。

「[哪個模型適合你](#哪個模型適合你)」可協助你做選擇。接著按住 fn，說點什麼，然後放開。

### 選單

按一下選單列中的圓環：

| 項目 | 功能 |
|---|---|
| **最近的聽寫** | 最近十筆文字；按一下其中一筆，就會再次拷貝到剪貼板 |
| **本機** | 你 Mac 上的模型，分為 Whisper 和 Parakeet 兩組；尚未下載的會以灰色顯示 |
| **OpenAI**、**Groq** | 線上模型；沒有密鑰時以灰色顯示，選單會提供 **設定 API 密鑰…** |
| **語言** | 你聽寫所用的語言，或 **自動偵測**；選單語言也會隨之改變 |
| **後處理** | **關閉**、**清理** 或 **格式化**，請參閱[後處理](#後處理) |
| **詞彙表…** | 需要精確拼寫的名稱和術語；選單會顯示你輸入了幾個 |
| **麥克風** | **系統預設值** 或指定的麥克風，以及 **就緒時播放聲音** |
| **管理模型** | 下載和刪除本機模型，並顯示進度 |
| **設定 API 密鑰…** / **更新 API 密鑰…** | 加入、取代或移除密鑰 |
| **在登入時打開** | 讓 Lowspeak 隨 Mac 一起啟動；勾選代表已開啟 |
| **檢查更新項目…** | 立即尋找新版本；其下方的兩個項目決定 Lowspeak 是否每天檢查一次、是否不經詢問就安裝更新。僅限從磁碟映像檔安裝的 App |
| **輔助說明** | **快速參考**、**回報問題…**、**建議功能…**、**GitHub 上的 Lowspeak** |
| **關於 Lowspeak** | 版本、版權、專案頁面和授權條款 |
| **結束 Lowspeak** | 選單開啟時按 ⌘Q |

**本機**、**OpenAI** 和 **Groq** 下的勾號代表正在使用的模型。圖像會顯示 Lowspeak 目前在做什麼：

| 圖像 | 意義 |
|---|---|
| 帶圓點的圓環 | 就緒 |
| 實心圓 | 按住 fn 時正在錄音 |
| 圓角方塊 | 錄音已鎖定；再按一下 fn 或 Shift 即結束 |
| 閃爍的圓環 | 正在轉錄 |
| 右上角帶圓點的圓環 | 有新版本等待安裝，請參閱 **檢查更新項目…** |
| ↓ 42% | 正在下載模型 |

Lowspeak 沒有設定視窗：一切都在這個選單中，而且它會記住你的每個選擇。

### 快速參考

**輔助說明 › 快速參考** 會在一個小視窗中用你的語言顯示 fn 手勢，視窗中還有一個可打開鍵盤設定的按鈕。**回報問題…** 會打開 GitHub 上的表單，並已填好你的 Lowspeak 和 macOS 版本；**建議功能…** 會打開用來交流點子的討論區；**GitHub 上的 Lowspeak** 會打開專案頁面。

## 聽寫

### 按住說話

按住 fn、說話、放開。麥克風一開始傳入聲音，錄音圖像就會出現；請在那時開始說話，因為在那之前說的話不會被錄下。放開 fn 就會結束錄音，片刻之後文字便會出現。

短按一下 fn 不會有任何作用，所以 fn 的其他用途不受影響。始終沒有內容或短於 0.3 秒的錄音會被捨棄。

### 用 Shift 或按兩下鎖定

較長的文字可以鎖定錄音，讓它在不按住按鍵的情況下繼續。共有三種方式：

- **按住 fn 時按 Shift。** 像平常一樣用 fn 開始，在錄音時按下 Shift：錄音會鎖定，你就可以放開 fn。先前說的話不會遺失，錄音只是繼續進行。
- **先按 Shift，再按 fn。** 按住 Shift 再按 fn：錄音一開始就是鎖定狀態。
- **按兩下 fn。** 短按 fn（短於 0.3 秒），並在 0.4 秒內再按一次。錄音從第一次按下時開始。

圖像會變成圓角方塊。按一下 **fn** 或 **Shift** 即可停止；文字會照常出現。若你的 🌐 鍵設為打開「表情符號與符號」，請用 Shift 停止：短按 fn 會打開該檢視器。

按住 fn 時，錄音會在兩分鐘後自動停止；鎖定時則在五分鐘後自動停止，然後照常轉錄。

### 文字會貼到哪裡

Lowspeak 會透過剪貼板和 ⌘V，把文字貼到你停止說話時位於最前方的 App 的游標位置。它會先儲存剪貼板上原有的所有內容，並在該 App 讀取文字半秒後放回（如果該 App 始終沒有讀取，則在十秒後放回），除非這段期間有其他 App 更動了剪貼板。文字會被標示為暫時性內容，因此遵循這項標示的剪貼板歷史記錄 App 不會記錄它。

上一段聽寫還在轉錄時，你就可以開始下一段。文字會依你說話的順序貼上，而且絕不會在你錄音或按住 Shift 時貼上，所以貼上的內容不會插進你下一句話的中間。

如果沒有輔助使用權限，或無法送出模擬的 ⌘V，文字會留在剪貼板上，並有通知告訴你；請自行按 ⌘V 貼上。

### 最近的聽寫

選單中的 **最近的聽寫** 會列出最近十筆文字，最新的在最前面，每筆都縮短為一行；將指標停在上面即可看到全文。按一下就會拷貝到剪貼板，適合文字貼錯地方或需要用兩次的時候。這份列表只存在記憶體中，Lowspeak 結束後就會清空。

## 轉錄

### 哪個模型適合你

| 你想要 | 選擇 | 原因 |
|---|---|---|
| **速度快又不佔用 Mac** | **Groq › Whisper Large V3 Turbo** | 最快的線上選項；不必下載任何東西，Mac 的記憶體和磁碟空間不受影響 |
| **最好的結果** | **OpenAI › GPT Transcribe**（預設） | Lowspeak 提供的模型中最準確的；會將你的詞彙表當作關鍵字 |
| **離線且保有隱私** | 歐洲語言選 **本機 › Parakeet Ultra**，其他語言選 **本機 › Whisper Large v3 Turbo** | 不需網路就能運作，你的聲音絕不會離開你的 Mac；Parakeet 是所有模型中最快的 |
| **不需帳號、不花錢** | 任一 **本機** 模型 | 在你的 Mac 上執行；不會產生任何費用 |
| **兼顧可靠與速度** | 一個線上模型，再加上安裝 Parakeet Ultra 或某個 Whisper 模型 | 線上服務失敗時由本機模型接手 |
| **Intel Mac** | **Groq** 或 **OpenAI** | 在這類 Mac 上，本機模型會在處理器上執行，請參閱 [Intel Mac](#intel-mac) |

本機模型在磁碟上佔用的空間與下載大小相同，被選用時也會佔用差不多的記憶體；Lowspeak 會讓模型保持載入，讓聽寫能立即開始。在記憶體為 8 GB 的 Mac 上，建議選用 Parakeet Ultra 或線上模型，而非大型 Whisper 模型。

### 線上：OpenAI 與 Groq

| 模型 | 服務提供者 | 說明 |
|---|---|---|
| **GPT Transcribe**（預設） | OpenAI | 最準確；會將你的詞彙表當作關鍵字 |
| **Whisper** | OpenAI | 原版 Whisper 模型 |
| **Whisper Large V3** | Groq | 非常快 |
| **Whisper Large V3 Turbo** | Groq | 線上最快 |

錄音在上傳前會壓縮為 AAC，每秒語音約 6 KB，所以在慢速連線下也能聽寫。你按住 fn 時，Lowspeak 就已經開始建立與服務提供者的連線，所以你一放開，上傳就會開始。

### 在 Mac 上：Whisper 與 Parakeet

| 模型 | 大小 | 執行於 | 語言 |
|---|---|---|---|
| **Parakeet Ultra** | 約 630 MB | 神經網路引擎 | 25 種歐洲語言 |
| **Parakeet v2** | 約 600 MB | 神經網路引擎 | 英文 |
| **Whisper Large v3 Turbo** | 約 1.6 GB | GPU（Metal） | 約 99 種 |
| **Whisper Large v3** | 約 3 GB | GPU（Metal） | 約 99 種 |
| **Whisper Medium** | 約 1.5 GB | GPU（Metal） | 約 99 種 |
| **Whisper Small** | 約 470 MB | GPU（Metal） | 約 99 種 |
| **Whisper Base** | 約 150 MB | GPU（Metal） | 約 99 種 |
| **Whisper Tiny** | 約 75 MB | GPU（Metal） | 約 99 種 |

**管理模型** 可下載和刪除模型；下載期間選單列會顯示進度，結束 Lowspeak 會取消下載。第一次使用某個模型時，它會先針對你的 Mac 做好準備：Parakeet Ultra 約需 10 秒，Whisper 最多 30 秒；之後只需不到一秒即可載入。如果你選擇的模型被刪除，Lowspeak 會切換到另一個已安裝的模型。

**Parakeet** 是最快的選擇：它能在約 0.3 秒內轉錄 40 秒的語音。Lowspeak 以與 [FluidAudio](https://github.com/FluidInference/FluidAudio) 相同的模型和解碼方式執行它，並將模型存放在 FluidAudio 的檔案夾（`~/Library/Application Support/FluidAudio/Models`）中，所以以 FluidAudio 打造的 App 可以共用這些模型。超過 15 秒的錄音會分成彼此重疊的片段轉錄，再接合起來，接縫處不會漏字。**Whisper** 聽得懂更多語言，也會讀取你的詞彙表；大型模型最準確，小型模型在英文以外的語言則弱上許多。Whisper 模型存放在 `~/Library/Application Support/Lowspeak/Models` 中。

### 線上服務失敗時

如果線上轉錄失敗，不論是網路中斷、服務提供者出問題，還是密鑰錯誤，已安裝的本機模型都會轉錄同一段錄音：優先使用在 **本機** 下選擇的模型，否則依序為 Parakeet Ultra、Whisper Large v3 Turbo、Large v3 和較小的 Whisper 模型，最後才是 Parakeet v2。通知只會提醒你一次；必須在這之間有一次線上聽寫成功之後，才會再次出現。

如果有本機模型可以接手，Lowspeak 會給線上要求 20 秒；如果沒有，就會嘗試三次，每次最多一分鐘。在線上模式下，Lowspeak 會在啟動時預先準備一次用來接手的 Parakeet 模型，所以第一次接手時不必等待。

### 錄音的前置處理

Lowspeak 以麥克風本身的取樣率錄音，並在你停止後轉換成模型所需的 16 kHz。轉錄之前，它會去除 80 Hz 以下的低頻隆隆聲，修剪開頭和結尾的靜音，並平衡音量，讓說話小聲和大聲的人都能得到相同的效果。不含語音的錄音完全不會送出。在本機模式下，Parakeet 會先拿到未經處理的錄音，只有在得不到文字時才改用處理過的錄音。

### 各模型支援的語言

在 **語言** 下選擇的語言會傳給線上模型和 Whisper。選擇 **自動偵測** 時，模型會自行判斷語言。

| 模型 | 可聽寫的語言 |
|---|---|
| GPT Transcribe、Whisper（OpenAI、Groq、在你的 Mac 上） | 約 99 種，包括選單中的全部 35 種 |
| Parakeet Ultra | 保加利亞文、克羅埃西亞文、捷克文、丹麥文、荷蘭文、英文、愛沙尼亞文、芬蘭文、法文、德文、希臘文、匈牙利文、義大利文、拉脫維亞文、立陶宛文、馬爾他文、波蘭文、葡萄牙文、羅馬尼亞文、俄文、斯洛伐克文、斯洛維尼亞文、西班牙文、瑞典文、烏克蘭文 |
| Parakeet v2 | 英文 |

Parakeet 會自行偵測語言，並忽略選單中的選擇。選擇 **繁體中文** 會讓 GPT Transcribe 以繁體字書寫；Whisper 寫中文時繁簡皆有可能，寫葡萄牙文時也不區分變體。

### Intel Mac

Lowspeak 是通用 App，在裝有 macOS 14 的 Intel Mac 上也能執行。所有線上功能都一樣可用。但本機模型在這裡沒有它們所針對的硬體：Whisper 在處理器而非 GPU 上執行，Parakeet 在處理器而非神經網路引擎上執行。兩者都能運作，但耗時是 Apple 晶片的好幾倍，大型 Whisper 模型更是慢到無法用於聽寫。在 Intel Mac 上，請使用線上轉錄，並安裝 Whisper Small 或 Parakeet Ultra 作為[備用模型](#線上服務失敗時)。

## 詞彙表

**詞彙表…** 用來輸入 Lowspeak 應精確拼寫的名稱和術語，以逗號分隔：產品名稱、同事、專業術語。最多 100 個詞語，每個最多 60 個字元；`<` 和 `>` 會被移除。

- **GPT Transcribe** 會將它們當作關鍵字。
- **Whisper**（OpenAI、Groq 和在你的 Mac 上）會將它們當作錄音前的提示來讀取，最多 500 個字元。
- **後處理** 會被要求完全依照所寫的方式拼寫它們。
- **Parakeet** 不會使用它們。

## 後處理

轉錄之後，語言模型可以再修飾文字。在 **後處理** 下選擇：

- **關閉**（預設）— 保持轉錄後的原樣。
- **清理** — 刪除「嗯」、「那個」、「基本上」之類的贅詞，修正文法和標點，並在你改口時只保留更正後的內容。它不會加入任何東西，也不會改變意思。
- **格式化** — 在「清理」的基礎上，再依你聽寫時所用的 App 調整文字：在程式碼編輯器中使用程式碼風格的用語，在終端機中使用命令列，在聊天 App 中使用輕鬆的語氣，在電子郵件中使用正式的語氣。

如果你有 Groq 密鑰，後處理會使用 Groq（`openai/gpt-oss-120b`），否則使用 OpenAI（`gpt-4o-mini`），本機轉錄時也是如此。若在 **語言** 下選擇了某種語言，後處理會保留該語言。如果耗時超過八秒、失敗，或傳回的回答被截斷或遠比你說的內容長，Lowspeak 會貼上未經處理的文字。如果服務提供者拒絕要求或尚未設定密鑰，會有一則通知告訴你一次。

## 麥克風

在 **麥克風** 下選擇指定的麥克風或 **系統預設值**。Lowspeak 會記住該裝置；裝置未連接時，選單會將它顯示為 **（未連接）**，而 Lowspeak 會使用系統預設值錄音。

麥克風只在你聽寫時才會開啟。準備就緒所需時間：內建麥克風約 0.07 秒，Studio Display 約半秒，AirPods 則要一到兩秒，因為它們得先切換到耳機模式。聲音一傳入，錄音圖像就會出現；**就緒時播放聲音** 會在那一刻播放一聲短促的聲音。此選項預設為關閉，因為在某些揚聲器上，聲音本身就會晚半秒才響起。

如果某個麥克風連續五秒只傳來靜音，Lowspeak 會告訴你；當另一部裝置佔用 AirPods 的麥克風時，AirPods 就會這樣。

## API 密鑰

**設定 API 密鑰…** 會要求你輸入密鑰，並依開頭辨識服務提供者：`sk-` 代表 OpenAI，`gsk_` 代表 Groq。其他格式的密鑰會被拒絕，並附上說明。每個服務提供者輸入一個密鑰；兩者的密鑰都有時，就能使用所有線上模型。**更新 API 密鑰…** 可取代密鑰，或透過 **移除所有密鑰** 將它們刪除。

密鑰以 `com.lowspeak.app` 為名稱儲存在你的登入鑰匙圈中。舊版本的密鑰檔案（`~/Library/Application Support/Lowspeak/.api-key`）會在下次啟動時移入鑰匙圈，並在每個密鑰都能從鑰匙圈讀回後刪除。

## 通知

Lowspeak 在背景運作，所以會以通知回報問題：

| 通知 | 發生了什麼事 |
|---|---|
| **錄音失敗** | 沒有麥克風取用權限、沒有連接麥克風，或無法啟動麥克風 |
| **麥克風沒有聲音** | 五秒鐘沒有聲音，請參閱[麥克風](#麥克風) |
| **需要輔助使用權限** | 文字在剪貼板上；允許輔助使用即可自動貼上 |
| **貼上失敗** | 文字在剪貼板上；請按 ⌘V 貼上 |
| **轉錄失敗** | 附上原因：沒有密鑰、密鑰被拒絕、超過要求次數限制、伺服器或網路錯誤，或沒有本機模型 |
| **已在這台 Mac 上轉錄** | 線上服務失敗，由本機模型接手 |
| **後處理失敗** | 服務提供者拒絕後處理，或尚未設定密鑰；文字已貼上，但未經後處理 |
| **沒有 API 密鑰** | 你選擇了線上模型，但沒有其服務提供者的密鑰 |
| **下載失敗**、**刪除失敗**、**無法刪除** | 管理本機模型時出錯，或另一項操作仍在進行 |
| **登入項目錯誤**、**API 密鑰錯誤** | macOS 或鑰匙圈拒絕了更動 |

通知絕不會包含你聽寫的文字。線上服務失敗和服務提供者拒絕後處理時只會回報一次，而不是每次聽寫都回報，直到它們恢復正常為止。

## 外觀與輔助使用

- **選單列圖像** 是範本影像，會配合淺色、深色或有色調的選單列。
- **旁白** 會將圖像讀成「Lowspeak」，選單、提示框和通知都是標準的 macOS 控制項。
- **不必一直按住按鍵。** 用 Shift 或按兩下 fn 鎖定錄音，下一次按鍵就會結束。
- **快速參考** 是只有標籤和按鈕的簡單視窗，可用鍵盤操作；按 Esc 即可關閉。

已知的不足列在 [ACCESSIBILITY.md](../../../ACCESSIBILITY.md) 中：旁白不會在錄音開始時播報，而且 Lowspeak 尚未經過旁白或切換控制的檢查。

## 語言

選單、對話框和通知已翻譯成 35 種語言：

阿拉伯文、加泰隆尼亞文、簡體中文、繁體中文、克羅埃西亞文、捷克文、丹麥文、荷蘭文、英文、芬蘭文、法文、德文、希臘文、希伯來文、印地文、匈牙利文、印尼文、義大利文、日文、韓文、馬來文、挪威文、波蘭文、葡萄牙文（巴西）、葡萄牙文（葡萄牙）、羅馬尼亞文、俄文、斯洛伐克文、斯洛維尼亞文、西班牙文、瑞典文、泰文、土耳其文、烏克蘭文和越南文。

在 **語言** 下選擇的語言，也就是選單的語言。選擇 **自動偵測** 時，選單會依照 **系統設定 › 一般 › 語言與地區** 中的語言順序；沒有對應翻譯時則使用英文。選單用語依循 Apple 自己的翻譯，所以 Lowspeak 讀起來就像 Mac 的其他部分。除了英文以外，各語言的翻譯都是機器翻譯，尚未經過母語人士審閱；歡迎以 [Issue](https://github.com/sventmueller/lowspeak/issues/new/choose) 提出更正。

## 按鍵與手勢

| 動作 | 按鍵 |
|---|---|
| 按住時聽寫 | 按住 fn |
| 鎖定進行中的錄音 | 按住 fn 時按 Shift |
| 開始鎖定的錄音 | Shift+fn，或快速按兩下 fn |
| 結束鎖定的錄音 | fn 或 Shift |
| 結束 Lowspeak（選單開啟時） | ⌘Q |
| 關閉快速參考 | Esc |

Lowspeak 會攔下它所使用的 fn 按鍵，不讓其他 App 收到；Shift 則照常傳遞。

## 隱私與安全性

Lowspeak 會在你按住 fn 時聽到你說的話，能在任何 App 中輸入，還保管著你的 API 密鑰，所以它對這些東西的處理盡可能少。

- **你的聲音只會去一個地方。** 在線上模式下，壓縮過的錄音會連同你的詞彙表，傳送給你選擇的服務提供者 OpenAI 或 Groq。在本機模式下，沒有任何東西會離開你的 Mac。線上要求失敗時，本機模型會在你的 Mac 上轉錄同一段錄音。
- **後處理** 若開啟，會把轉錄的文字和你的詞彙表傳送給 Groq 或 OpenAI。在「格式化」模式下，還會附上你聽寫時所用 App 的類型（程式碼編輯器、終端機、聊天或電子郵件）；不會傳送 App 的名稱。
- **沒有遙測，不需帳號。** 除了轉錄和後處理之外，Lowspeak 只會在以下情況連上網路：在線上模式或後處理開啟時，於你按住 fn 期間建立與服務提供者的連線；在你要求時，透過 HTTPS 從 Hugging Face 下載固定版本的模型，並依據核對碼檢查每個檔案；以及在你允許的情況下，每天一次在 GitHub 上尋找新版本，該請求只帶有 Lowspeak 的版本，別無其他。**輔助說明 › 回報問題…** 會在瀏覽器中打開 GitHub 上的表單；其網址只帶有 Lowspeak 的版本和 macOS 的版本，不含你聽寫的任何內容。
- **API 密鑰** 存放在你的登入鑰匙圈中。它們只會傳送給各自的服務提供者，並會在任何內容寫入記錄之前從錯誤訊息中移除。
- **聽寫的文字不會儲存。** 最近十筆聽寫會留在記憶體中供 **最近的聽寫** 使用，Lowspeak 結束後就會消失。記錄檔只記錄長度和耗時，絕不記錄文字。
- **剪貼板** 會在貼上前儲存，並在目標 App 讀取文字後還原，除非這段期間有其他 App 更動了它。聽寫內容會被標示為暫時性內容，因此遵循這項標示的剪貼板歷史記錄 App 會略過它。
- **程式碼簽章** 使用 Hardened Runtime；App 只要求麥克風、輔助使用、通知和網路權限。更新必須帶有開發者簽章，否則 Sparkle 會拒絕它。

若要回報安全漏洞，請依照[安全性政策](../../../SECURITY.md)。

## 速度

在 Apple 晶片上測得：

- **Parakeet Ultra** 能在約 0.3 秒內轉錄 40 秒的語音。
- **線上：** 停頓一段時間後的第一個要求，在 Groq 上約需 0.4 秒，因為 Lowspeak 在你還在說話時就建立了連線；之後的要求會沿用該連線，並在服務提供者支援時使用 HTTP/3。
- **上傳大小：** 以 48 kbit/s 的 AAC 編碼，每秒語音約 6 KB。
- **麥克風準備時間：** 內建麥克風約 0.07 秒，Studio Display 約半秒，AirPods 一到兩秒。

## 已知限制

- **Intel Mac 上的本機模型** 會在處理器上執行，速度慢上許多；在這類 Mac 上，線上轉錄是較好的選擇。
- **麥克風就緒前說的話會遺失。** 請等錄音圖像出現，使用 AirPods 時尤其如此。
- **fn 鍵可能會被其他程式佔用。** 當 macOS 的 🌐 鍵設定、Warp 或 macOS 聽寫搶先佔用 fn，Lowspeak 就偵測不到它；請參閱[疑難排解](#疑難排解)。
- **Parakeet** 會忽略詞彙表和語言選擇，而且只認得歐洲語言。
- **字體與變體：** 只有 GPT Transcribe 能被指定以繁體中文書寫；沒有任何模型能被指定以歐洲葡萄牙文而非巴西葡萄牙文書寫。
- **時間限制：** 按住 fn 時兩分鐘，鎖定時五分鐘。
- **麥克風模式：** 控制中心也會為 Lowspeak 提供「語音隔離」和「寬廣頻譜」，並顯示為已啟用，但 Lowspeak 錄音時不做語音處理，所以這項選擇沒有作用。
- **最近的聽寫** 只存在記憶體中，結束後就會消失。
- **刻意的設計：** 沒有設定視窗、沒有 Dock 圖像，也不在 App Store 上架。

## 疑難排解

**按住 fn 時沒有任何反應。** 檢查三件事：在 **系統設定 › 隱私權與安全性 › 輔助使用**（macOS 27 中為 **裝置控制和資料取用**）中已允許 Lowspeak；**系統設定 › 鍵盤 › 按下🌐鍵來** 設為 **不執行任何動作**；而且沒有其他 App 佔用 fn。Warp 預設會佔用；請用 `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` 關閉它，然後重新啟動 Warp。macOS 聽寫也可能被設定為用 fn 啟動。

**更新後，Lowspeak 不再回應 fn。** macOS 會把輔助使用權限與 App 的簽章綁在一起。如果簽章變了，例如從你自行建置的版本換成下載的 App，請從輔助使用列表（macOS 27 中為「裝置控制和資料取用」）中移除 Lowspeak，然後再重新加入。

**開頭的幾個字不見了。** 請在錄音圖像出現後再開始說話。使用 AirPods 時，麥克風需要一到兩秒；在 **麥克風** 下選擇內建或 USB 麥克風，幾乎可以立即就緒。

**「麥克風沒有聲音」。** 麥克風五秒鐘沒有傳來聲音。使用 AirPods 時，可能是另一部裝置正在使用它們的麥克風；請切換到內建麥克風，或重新連接 AirPods。闔上 MacBook 的螢幕也會讓內建麥克風靜音。

**聽寫結果是空的。** 輸入音量可能太低：Teams、Zoom 或 Meet 等 App 會在通話時更改它，而 macOS 會分別保存每個裝置的音量。請在 **系統設定 › 聲音 › 輸入** 中調高。

**文字貼錯地方了。** 選單中的 **最近的聽寫** 可以把它再次拷貝到剪貼板。

**「沒有 API 密鑰」。** 在選單中選擇 **設定 API 密鑰…**，或切換到本機模型。

**第一次本機轉錄花了很久。** 每個模型在第一次使用時，都要針對你的 Mac 做準備；之後只需不到一秒即可載入。

**Lowspeak 啟動後立刻結束。** 另一個 Lowspeak 已經在執行；請先結束那一個。

**沒有出現通知。** 請在 **系統設定 › 通知 › Lowspeak** 中允許通知。

**其他問題。** `~/Library/Application Support/Lowspeak/` 中的記錄檔（`appstate.log`、`audio.log`、`hotkey.log`、`crash.log`）會描述發生了什麼事，但不含你聽寫的文字。請將相關的行附在[問題回報](https://github.com/sventmueller/lowspeak/issues/new/choose)中。

## 給開發者

建置、測試、程式碼結構及其遵循的規則請見 [docs/development.md](../../../docs/development.md)，貢獻方式請見 [CONTRIBUTING.md](../../../CONTRIBUTING.md)（皆為英文）。

### 第三方元件

| 元件 | 版本 | 用途 | 授權條款 |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp)，透過 [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | 在 GPU（Metal）上執行的本機 Whisper | MIT；whisper-rs 為 Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet 解碼與長錄音合併，已在 `src/transcription/parakeet` 中翻譯為 Rust | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) 及其框架 crate | 0.6 / 0.3 | AppKit、CoreML、CoreAudio、AVFoundation 和其他系統框架 | MIT；Zlib、Apache-2.0 或 MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | 透過 CoreAudio 擷取麥克風音訊 | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | 重新取樣為 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq)（使用 rustls） | 3.5 | 下載模型 | MIT 或 Apache-2.0；rustls 和 ring 採用 ISC 和 Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | 所下載模型的核對碼 | MIT 或 Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | 鑰匙圈中的 API 密鑰 | MIT 或 Apache-2.0 |
| [serde](https://serde.rs)、serde_json、anyhow、unicode-general-category | 1 | JSON、錯誤處理、Parakeet 的詞彙 | MIT 或 Apache-2.0；Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | 從磁碟映像檔安裝的 App 的更新 | MIT，部分採用 BSD 和 zlib 授權 |

App 內所有元件的授權文字都隨 App 一起提供，位於 `Contents/Resources/Acknowledgements.txt`。

模型不包含在 Lowspeak 中；它會在你要求時下載：

| 模型 | 來源 | 授權條款 |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp)（ggml 格式的 OpenAI Whisper） | MIT |
| Parakeet v2、Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference) 將 NVIDIA Parakeet TDT 轉換成的 CoreML 版本 | CC-BY-4.0 |

## 授權條款

Lowspeak 採用以下任一授權條款：

- Apache License, Version 2.0（[LICENSE-APACHE](../../../LICENSE-APACHE)）
- MIT License（[LICENSE-MIT](../../../LICENSE-MIT)）

由你選擇。

除非你另有明確聲明，否則依 Apache-2.0 授權條款所定義，你有意提交以納入 Lowspeak 的任何貢獻，都將採用如上的雙重授權，不附加任何其他條款或條件。
