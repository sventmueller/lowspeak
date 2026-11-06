<p><img src="../../readme-icon.png" alt="Lowspeak 应用图标" width="168" height="168"></p>

# Lowspeak

**在光标处直接输入的 Mac 听写工具：按住 fn，说话，松开。**

Lowspeak 常驻菜单栏。按住 fn（地球仪）键说话，松开后，你说的话就会出现在你正在输入的 App 中，就像你亲手键入的一样。它可以通过 OpenAI 或 Groq 在线转写，也可以借助 Whisper 或 NVIDIA Parakeet 完全在你的 Mac 上转写；Parakeet 能在约三分之一秒内把 40 秒的语音变成文字。网络或在线服务出现故障时，已安装的本地模型会接手，因此听写不会丢失。你可以教它认识你常用的名称和术语，可以按需让语言模型去除语气词，还可以用 Shift 锁定录音，免提听写较长的文本。它用 Rust 编写，基于原生 AppKit 控件，支持 35 种语言，只会把你的声音发送给你选择的服务提供商。

[![下载适用于 macOS 的 Lowspeak](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[下载 Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**：已由 Apple 签名并公证，适用于 macOS 14 或更高版本，支持 Apple 芯片和 Intel 处理器。

**发现 Bug 了？** 请[提交 Issue](https://github.com/sventmueller/lowspeak/issues/new/choose)，或在 Lowspeak 的菜单中选取 **帮助 › 报告问题…**：表单打开时会自动填好你的版本。如果 Lowspeak 对你有用，在 [GitHub 上点个 Star](https://github.com/sventmueller/lowspeak) 能帮助更多人找到它。

[![Rust 2024 版](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 或更高版本](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![通用：Apple 芯片和 Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![通过 objc2 使用 AppKit](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![在 Metal 上运行的 whisper.cpp](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![在神经网络引擎上运行的 Parakeet](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![通过 OpenAI 或 Groq 在线转写](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 种界面语言](https://img.shields.io/badge/languages-35-4C9A2A)
![使用 Hardened Runtime 签名](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![许可证：MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#许可证)

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
**简体中文** ·
[繁體中文](../zh-Hant/README.md)

## 目录

- [功能一览](#功能一览)
- [系统要求](#系统要求)
- [安装](#安装)
- [开始使用](#开始使用)
  - [权限](#权限)
  - [选择转写方式](#选择转写方式)
  - [菜单](#菜单)
  - [快速参考](#快速参考)
- [听写](#听写)
  - [按住说话](#按住说话)
  - [用 Shift 或连按两次锁定](#用-shift-或连按两次锁定)
  - [文本的去向](#文本的去向)
  - [最近的听写](#最近的听写)
- [转写](#转写)
  - [哪个模型适合你](#哪个模型适合你)
  - [在线：OpenAI 和 Groq](#在线openai-和-groq)
  - [在 Mac 上：Whisper 和 Parakeet](#在-mac-上whisper-和-parakeet)
  - [在线服务失败时](#在线服务失败时)
  - [录音的预处理](#录音的预处理)
  - [各模型支持的语言](#各模型支持的语言)
  - [Intel Mac](#intel-mac)
- [词汇表](#词汇表)
- [后处理](#后处理)
- [麦克风](#麦克风)
- [API 密钥](#api-密钥)
- [通知](#通知)
- [外观和辅助功能](#外观和辅助功能)
- [语言](#语言)
- [按键和手势](#按键和手势)
- [隐私和安全](#隐私和安全)
- [速度](#速度)
- [已知限制](#已知限制)
- [疑难解答](#疑难解答)
- [开发者须知](#开发者须知)
  - [第三方组件](#第三方组件)
- [许可证](#许可证)

## 功能一览

- **按住 fn，说话，松开。** 文本会输入到前台 App 中光标所在的位置：邮件、聊天、编辑器、终端或浏览器都可以。无需切换窗口，也无需点按按钮。
- **长文本可锁定录音。** 按住 fn 时按下 Shift，或连按两次 fn，录音就会在不按任何键的情况下持续进行，最长五分钟。按一下 fn 或 Shift 即可结束。
- **为任务选对模型。** 想要速度又不占用 Mac 的内存，选 Groq；想要最佳效果，选 OpenAI 的 GPT Transcribe；想离线工作并让声音留在本机，选在 Mac 上运行的 Whisper 或 NVIDIA Parakeet。点按一下即可切换。
- **听写不会丢失。** 网络或在线服务出现故障时，已安装的本地模型会在你的 Mac 上转写同一段录音。
- **你的用词，拼写无误。** 由名称和专业术语组成的词汇表会引导 OpenAI、Groq、Whisper 和后处理。
- **按需获得整洁的文本。** 可选的后处理会去除语气词、修正语法，并在你改口时只保留更正后的内容；也可以根据你听写所用 App 的类型来编排文本格式。
- **一直说下去。** 上一段听写还在转写时，就可以开始下一段；文本会按你说话的顺序送达。
- **剪贴板还是你的。** Lowspeak 通过剪贴板粘贴，片刻之后再放回原来的内容。最近的十条听写会保留在菜单中，可以再次拷贝。
- **彻头彻尾的原生。** 真正的菜单栏项目，配有 AppKit 菜单、标准的提醒和通知。没有 Electron，没有窗口，也没有程序坞图标。
- **注重隐私。** 无需账户，没有遥测。音频只发送给你选择的服务提供商，在本地模式下则不发送到任何地方。
- **使用你的语言。** 听写支持数十种语言，菜单、对话框和通知提供 35 种语言版本。

## 系统要求

- macOS 14 Sonoma 或更高版本。Lowspeak 在 macOS 27 上开发和测试。
- 搭载 Apple 芯片或 Intel 处理器的 Mac。本地转写是为 Apple 芯片打造的：Whisper 在 GPU 上运行，Parakeet 在神经网络引擎上运行；在 Intel Mac 上，两者都在处理器上运行，速度慢得多，参见 [Intel Mac](#intel-mac)。
- App 本身约 10 MB，每下载一个本地模型另需 75 MB 至 3 GB。
- 在线转写需要 [OpenAI](https://platform.openai.com/api-keys) 或 [Groq](https://console.groq.com/keys) 的 API 密钥；两者都会按转写量收费。本地转写无需账户。

## 安装

**通过磁盘映像。** 下载 [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)，打开后将 Lowspeak 拖到“应用程序”文件夹。该映像已使用 Developer ID 签名并经 Apple 公证，因此打开时不会出现警告。

**更新。** Lowspeak 不会自行更新。新版本会发布在[发布页面](https://github.com/sventmueller/lowspeak/releases)上，**帮助 › GitHub 上的 Lowspeak** 可以带你前往。将新版本直接安装在旧版本之上即可，你的设置、密钥和模型都会保留。

**从源代码构建。** 先查看[构建与部署](../../../docs/development.md#build--deploy)了解所需的前提条件，然后执行：

```sh
make deploy
```

这会构建 Lowspeak，替换“应用程序”文件夹中的副本，然后启动它。

菜单中的 **登录时打开** 会通过 macOS 自带的登录项，让 Lowspeak 随 Mac 一起启动。同一时间只能运行一个 Lowspeak，第二个会立即退出。

## 开始使用

### 权限

首次启动时，Lowspeak 会请求三项权限：

- **无障碍**（系统设置 › 隐私与安全 › 无障碍；在 macOS 27 中，此列表名为“设备控制和数据访问”）。Lowspeak 需要它来识别 fn 键并用 ⌘V 粘贴。macOS 会提出为你打开列表；请打开 Lowspeak 的开关。Lowspeak 自身的提示会按你的 macOS 所显示的名称来称呼这个列表。
- **麦克风。** 只询问一次；没有此权限，Lowspeak 什么也录不到。
- **通知。** Lowspeak 用它来告诉你那些否则不会注意到的问题，比如麦克风无声或转写失败。

### 选择转写方式

至少设置一种方式；你可以随时在菜单中切换。

- **在线：** 选取 **设置 API 密钥…**，粘贴 OpenAI 的密钥（以 `sk-` 开头）或 Groq 的密钥（`gsk_`）。Lowspeak 会根据密钥识别服务提供商。然后在 **OpenAI** 或 **Groq** 下选取一个模型。
- **在你的 Mac 上：** 选取 **管理模型** › **下载 Parakeet Ultra** 或某个 Whisper 模型，然后在 **本地** 下选取它。

[哪个模型适合你](#哪个模型适合你)可以帮助你选择。然后按住 fn，说点什么，再松开。

### 菜单

点按菜单栏中的圆环：

| 项目 | 作用 |
|---|---|
| **最近的听写** | 最近的十条文本；点按其中一条可再次将其拷贝到剪贴板 |
| **本地** | 你 Mac 上的模型，分为 Whisper 和 Parakeet 两组；未下载的显示为灰色 |
| **OpenAI**、**Groq** | 在线模型；没有密钥时显示为灰色，菜单会提供 **设置 API 密钥…** |
| **语言** | 你听写所用的语言，或 **自动检测**；菜单语言也随之改变 |
| **后处理** | **关闭**、**清理** 或 **格式化**，参见[后处理](#后处理) |
| **词汇表…** | 需要准确拼写的名称和术语；菜单会显示你输入了多少个 |
| **麦克风** | **系统默认** 或某个固定的麦克风，以及 **就绪时播放提示音** |
| **管理模型** | 下载和删除本地模型，并显示进度 |
| **设置 API 密钥…** / **更新 API 密钥…** | 添加、替换或移除密钥 |
| **登录时打开** | 让 Lowspeak 随 Mac 一起启动；打勾表示已开启 |
| **帮助** | **快速参考**、**报告问题…**、**建议功能…**、**GitHub 上的 Lowspeak** |
| **关于 Lowspeak** | 版本、版权、项目页面和许可证 |
| **退出 Lowspeak** | 菜单打开时按 ⌘Q |

**本地**、**OpenAI** 和 **Groq** 下的勾号标示正在使用的模型。图标显示 Lowspeak 当前在做什么：

| 图标 | 含义 |
|---|---|
| 带圆点的圆环 | 就绪 |
| 实心圆 | 按住 fn 时正在录音 |
| 圆角方块 | 录音已锁定；再按一下 fn 或 Shift 即结束 |
| 闪烁的圆环 | 正在转写 |
| ↓ 42% | 正在下载模型 |

Lowspeak 没有设置窗口：一切都在这个菜单里，而且它会记住你的每一个选择。

### 快速参考

**帮助 › 快速参考** 会在一个小窗口中用你的语言显示 fn 手势，窗口中还有一个打开键盘设置的按钮。**报告问题…** 会打开 GitHub 上的表单，并已填好你的 Lowspeak 和 macOS 版本；**建议功能…** 会打开用于交流想法的讨论区；**GitHub 上的 Lowspeak** 会打开项目页面。

## 听写

### 按住说话

按住 fn，说话，松开。麦克风一开始传入声音，录音图标就会出现；请在那时开始说话，因为在此之前说的话不会被录下。松开 fn 即结束录音，片刻之后文本就会出现。

轻按一下 fn 不会触发任何操作，因此 fn 的其他用途不受影响。始终为空的录音或短于 0.3 秒的录音会被丢弃。

### 用 Shift 或连按两次锁定

对于较长的文本，可以锁定录音，让它在不按键的情况下继续。有三种方法：

- **按住 fn 时按 Shift。** 像平常一样用 fn 开始，在录音过程中按下 Shift：录音即被锁定，你可以松开 fn。之前说的话不会丢失，录音只是继续进行。
- **先 Shift，再 fn。** 按住 Shift 再按 fn：录音一开始就处于锁定状态。
- **连按两次 fn。** 轻按一下 fn（短于 0.3 秒），并在 0.4 秒内再按一次。录音从第一次按下时开始。

图标会变成圆角方块。按一下 **fn** 或 **Shift** 即可停止；文本会照常出现。如果你的 🌐 键设置为打开表情与符号，请用 Shift 停止：短按 fn 会打开该检视器。

按住 fn 时，录音会在两分钟后自动停止；锁定时，会在五分钟后自动停止，然后照常转写。

### 文本的去向

Lowspeak 会通过剪贴板和 ⌘V，把文本粘贴到你停止说话时位于前台的 App 中光标所在的位置。它会先保存剪贴板上原有的全部内容，并在该 App 读取文本半秒后放回（如果该 App 始终没有读取，则在十秒后放回），除非在此期间有其他 App 更改了剪贴板。文本会被标记为临时内容，因此遵循这一标记的剪贴板历史 App 不会记录它。

上一段听写还在转写时，你就可以开始下一段。文本会按你说话的顺序粘贴，而且绝不会在你录音或按住 Shift 时粘贴，因此粘贴的内容不会插到你下一句话的中间。

如果没有无障碍权限，或无法发送模拟的 ⌘V，文本会留在剪贴板上，并有通知告知你；请自己按 ⌘V 粘贴。

### 最近的听写

菜单中的 **最近的听写** 列出最近的十条文本，最新的在前，每条都缩短为一行；将指针悬停在上面即可看到全文。点按一条会将其拷贝到剪贴板，适合文本出现在错误位置或你需要用两次的情况。该列表只保存在内存中，Lowspeak 退出后即清空。

## 转写

### 哪个模型适合你

| 你想要 | 选择 | 原因 |
|---|---|---|
| **速度快且不占用 Mac** | **Groq › Whisper Large V3 Turbo** | 最快的在线选项；无需下载任何东西，Mac 的内存和磁盘空间不受影响 |
| **最佳效果** | **OpenAI › GPT Transcribe**（默认） | Lowspeak 提供的模型中最准确的；会将你的词汇表作为关键词使用 |
| **离线且私密** | 欧洲语言选 **本地 › Parakeet Ultra**，其他语言选 **本地 › Whisper Large v3 Turbo** | 无需网络即可工作，你的声音永远不会离开你的 Mac；Parakeet 是所有模型中最快的 |
| **无需账户，没有费用** | 任意 **本地** 模型 | 在你的 Mac 上运行；不产生任何费用 |
| **兼顾可靠与速度** | 一个在线模型，再安装 Parakeet Ultra 或某个 Whisper 模型 | 在线服务失败时由本地模型接手 |
| **Intel Mac** | **Groq** 或 **OpenAI** | 在这类 Mac 上，本地模型在处理器上运行，参见 [Intel Mac](#intel-mac) |

本地模型占用的磁盘空间与其下载大小相同，被选中时还会占用大致等量的内存；Lowspeak 会让模型保持载入状态，以便听写可以立即开始。在内存为 8 GB 的 Mac 上，建议选择 Parakeet Ultra 或在线模型，而非大型 Whisper 模型。

### 在线：OpenAI 和 Groq

| 模型 | 服务提供商 | 说明 |
|---|---|---|
| **GPT Transcribe**（默认） | OpenAI | 最准确；会将你的词汇表作为关键词使用 |
| **Whisper** | OpenAI | 原版 Whisper 模型 |
| **Whisper Large V3** | Groq | 非常快 |
| **Whisper Large V3 Turbo** | Groq | 在线最快 |

录音在上传前会被压缩为 AAC，每秒语音约 6 KB，因此在慢速网络连接下也能听写。在你按住 fn 时，Lowspeak 就已经开始建立与服务提供商的连接，所以你一松开，上传就会开始。

### 在 Mac 上：Whisper 和 Parakeet

| 模型 | 大小 | 运行于 | 语言 |
|---|---|---|---|
| **Parakeet Ultra** | 约 630 MB | 神经网络引擎 | 25 种欧洲语言 |
| **Parakeet v2** | 约 600 MB | 神经网络引擎 | 英语 |
| **Whisper Large v3 Turbo** | 约 1.6 GB | GPU（Metal） | 约 99 种 |
| **Whisper Large v3** | 约 3 GB | GPU（Metal） | 约 99 种 |
| **Whisper Medium** | 约 1.5 GB | GPU（Metal） | 约 99 种 |
| **Whisper Small** | 约 470 MB | GPU（Metal） | 约 99 种 |
| **Whisper Base** | 约 150 MB | GPU（Metal） | 约 99 种 |
| **Whisper Tiny** | 约 75 MB | GPU（Metal） | 约 99 种 |

**管理模型** 用于下载和删除模型；下载期间菜单栏会显示进度，退出 Lowspeak 会取消下载。首次使用某个模型时，它会先针对你的 Mac 做好准备：Parakeet Ultra 约需 10 秒，Whisper 最多需 30 秒；之后载入只需不到一秒。如果你选择的模型被删除，Lowspeak 会切换到另一个已安装的模型。

**Parakeet** 是最快的选择：它能在约 0.3 秒内转写 40 秒的语音。Lowspeak 使用与 [FluidAudio](https://github.com/FluidInference/FluidAudio) 相同的模型和解码方式运行它，并将模型存放在 FluidAudio 的文件夹（`~/Library/Application Support/FluidAudio/Models`）中，因此基于 FluidAudio 的 App 可以共用这些模型。超过 15 秒的录音会分成相互重叠的片段分别转写，再拼接起来，接缝处不会丢字。**Whisper** 能听懂更多语言，也会读取你的词汇表；大型模型最准确，小型模型在英语以外的语言上则弱得多。Whisper 模型存放在 `~/Library/Application Support/Lowspeak/Models` 中。

### 在线服务失败时

如果在线转写失败，无论是因为网络中断、服务提供商出现问题还是密钥有误，已安装的本地模型都会转写同一段录音：优先使用在 **本地** 下选择的模型，否则依次为 Parakeet Ultra、Whisper Large v3 Turbo、Large v3 和较小的 Whisper 模型，Parakeet v2 排在最后。通知只会提醒你一次；只有在此期间有一次在线听写成功之后，它才会再次出现。

如果有本地模型可以接手，Lowspeak 会给在线请求 20 秒时间；如果没有，它会尝试三次，每次最多一分钟。在在线模式下，Lowspeak 会在启动时预先准备一次用于接手的 Parakeet 模型，这样第一次接手时就不必等待。

### 录音的预处理

Lowspeak 以麦克风自身的采样率录音，并在你停止后转换为模型所需的 16 kHz。转写之前，它会去除 80 Hz 以下的低频隆隆声，裁掉开头和结尾的静音，并均衡音量，使说话轻柔的人和说话响亮的人效果一致。不含语音的录音根本不会发送。在本地模式下，Parakeet 会先拿到未经处理的录音，只有在没有得到文本时才使用预处理后的录音。

### 各模型支持的语言

在 **语言** 下选择的语言会传给在线模型和 Whisper。选择 **自动检测** 时，模型会自行判断语言。

| 模型 | 可听写的语言 |
|---|---|
| GPT Transcribe、Whisper（OpenAI、Groq、在你的 Mac 上） | 约 99 种，涵盖菜单中的全部 35 种 |
| Parakeet Ultra | 保加利亚语、克罗地亚语、捷克语、丹麦语、荷兰语、英语、爱沙尼亚语、芬兰语、法语、德语、希腊语、匈牙利语、意大利语、拉脱维亚语、立陶宛语、马耳他语、波兰语、葡萄牙语、罗马尼亚语、俄语、斯洛伐克语、斯洛文尼亚语、西班牙语、瑞典语、乌克兰语 |
| Parakeet v2 | 英语 |

Parakeet 会自行检测语言，并忽略菜单中的选择。选择 **繁體中文** 会让 GPT Transcribe 用繁体字书写；Whisper 写中文时简繁皆有可能，写葡萄牙语时也不区分变体。

### Intel Mac

Lowspeak 是通用 App，也能在装有 macOS 14 的 Intel Mac 上运行。所有在线功能都一样可用。但本地模型在这里没有它们所针对的硬件：Whisper 在处理器而不是 GPU 上运行，Parakeet 在处理器而不是神经网络引擎上运行。两者都能用，但耗时是 Apple 芯片上的许多倍，大型 Whisper 模型更是慢得无法用于听写。在 Intel Mac 上，请使用在线转写，并安装 Whisper Small 或 Parakeet Ultra 作为[备用模型](#在线服务失败时)。

## 词汇表

**词汇表…** 用于输入 Lowspeak 应准确拼写的名称和术语，以逗号分隔：产品名称、同事的名字、专业术语。最多 100 个词条，每个最长 60 个字符；`<` 和 `>` 会被移除。

- **GPT Transcribe** 将它们作为关键词接收。
- **Whisper**（OpenAI、Groq 和在你的 Mac 上）将它们作为录音之前的提示读取，最多 500 个字符。
- **后处理** 会被要求严格按照所写的方式拼写它们。
- **Parakeet** 不使用它们。

## 后处理

转写之后，语言模型可以对文本进行再加工。在 **后处理** 下选择：

- **关闭**（默认）— 保持转写原样的文本。
- **清理** — 去除“嗯”“那个”“就是说”之类的语气词，修正语法和标点，在你改口时只保留更正后的内容。它不会添加任何内容，也不会改变意思。
- **格式化** — 在“清理”的基础上，再根据你听写所用的 App 调整文本：在代码编辑器中使用代码风格的术语，在终端中使用命令行，在聊天 App 中使用随意的语气，在邮件中使用正式的语气。

如果你有 Groq 密钥，后处理使用 Groq（`openai/gpt-oss-120b`），否则使用 OpenAI（`gpt-4o-mini`），在本地转写时也是如此。如果在 **语言** 下选择了某种语言，后处理会保持该语言。如果耗时超过八秒、失败，或返回的回答被截断或远长于你所说的内容，Lowspeak 会粘贴未经处理的文本。如果服务提供商拒绝请求或未设置密钥，会有一条通知告知你一次。

## 麦克风

在 **麦克风** 下，选择一个固定的麦克风或 **系统默认**。Lowspeak 会记住该设备；在它未连接期间，菜单中会将其显示为 **(未连接)**，Lowspeak 则使用系统默认麦克风录音。

麦克风只在你听写时才会开启。准备就绪所需的时间：内建麦克风约 0.07 秒，Studio Display 约半秒，AirPods 需要一到两秒，因为它们要先切换到耳机模式。声音一到达，录音图标就会出现；**就绪时播放提示音** 会在那一刻播放一声短促的提示音。此选项默认关闭，因为在某些扬声器上，提示音本身就会晚半秒才响起。

如果某个麦克风连续五秒只传来静音，Lowspeak 会告诉你；当另一台设备占用了 AirPods 的麦克风时，AirPods 就会这样。

## API 密钥

**设置 API 密钥…** 会要求你输入密钥，并根据开头识别服务提供商：`sk-` 对应 OpenAI，`gsk_` 对应 Groq。其他格式的密钥会被拒绝，并附有说明。每个服务提供商输入一个密钥；两者的密钥都有时，所有在线模型均可使用。**更新 API 密钥…** 可替换密钥，或者通过 **移除所有密钥** 将其删除。

密钥以 `com.lowspeak.app` 为名称存储在你的登录钥匙串中。旧版本的密钥文件（`~/Library/Application Support/Lowspeak/.api-key`）会在下次启动时移入钥匙串，并在每个密钥都能从钥匙串中读回后删除。

## 通知

Lowspeak 在后台工作，因此会以通知的形式报告问题：

| 通知 | 发生了什么 |
|---|---|
| **录音失败** | 没有麦克风访问权限、没有连接麦克风，或麦克风无法启动 |
| **麦克风无声** | 五秒钟没有声音，参见[麦克风](#麦克风) |
| **需要无障碍权限** | 文本在剪贴板上；允许无障碍权限即可自动粘贴 |
| **粘贴失败** | 文本在剪贴板上；请按 ⌘V 粘贴 |
| **转写失败** | 附有原因：没有密钥、密钥被拒绝、超出请求次数限制、服务器或网络错误，或没有本地模型 |
| **已在这台 Mac 上转写** | 在线服务失败，由本地模型接手 |
| **后处理失败** | 服务提供商拒绝了后处理，或未设置密钥；文本已粘贴，未经后处理 |
| **无 API 密钥** | 你选择了一个在线模型，但没有其服务提供商的密钥 |
| **下载失败**、**删除失败**、**无法删除** | 管理本地模型时出错，或另一项操作仍在进行 |
| **登录项错误**、**API 密钥错误** | macOS 或钥匙串拒绝了更改 |

通知中绝不会包含你听写的文本。在线服务出现故障或后处理服务提供商拒绝请求时，只会报告一次，而不是每次听写都报告，直到它们恢复正常为止。

## 外观和辅助功能

- **菜单栏图标** 是一个模板图像，会随浅色、深色或着色的菜单栏而变化。
- **旁白** 会将图标读作“Lowspeak”，菜单、提醒和通知都是标准的 macOS 控件。
- **无需一直按住按键。** 用 Shift 或连按两次 fn 锁定录音，下一次按键即可结束。
- **快速参考** 是一个只含标签和按钮的简单窗口，可以通过键盘操作；按 Esc 即可关闭。

已知的不足列在 [ACCESSIBILITY.md](../../../ACCESSIBILITY.md) 中：旁白不会在录音开始时播报，并且 Lowspeak 尚未用旁白或切换控制进行过检查。

## 语言

菜单、对话框和通知已翻译为 35 种语言：

阿拉伯语、加泰罗尼亚语、简体中文、繁体中文、克罗地亚语、捷克语、丹麦语、荷兰语、英语、芬兰语、法语、德语、希腊语、希伯来语、印地语、匈牙利语、印度尼西亚语、意大利语、日语、韩语、马来语、挪威语、波兰语、葡萄牙语（巴西）、葡萄牙语（葡萄牙）、罗马尼亚语、俄语、斯洛伐克语、斯洛文尼亚语、西班牙语、瑞典语、泰语、土耳其语、乌克兰语和越南语。

在 **语言** 下选择的语言同时也是菜单的语言。选择 **自动检测** 时，菜单会遵循 **系统设置 › 通用 › 语言与地区** 中的语言顺序；没有相应翻译时使用英语。菜单术语沿用 Apple 自己的翻译，因此 Lowspeak 读起来与 Mac 上的其他部分一致。除英语外，各语言的翻译均为机器翻译，尚未经过母语人士审阅；欢迎以 [Issue](https://github.com/sventmueller/lowspeak/issues/new/choose) 的形式提交更正。

## 按键和手势

| 操作 | 按键 |
|---|---|
| 按住时听写 | 按住 fn |
| 锁定正在进行的录音 | 按住 fn 时按 Shift |
| 开始一段锁定的录音 | Shift+fn，或快速连按两次 fn |
| 结束锁定的录音 | fn 或 Shift |
| 退出 Lowspeak（菜单打开时） | ⌘Q |
| 关闭快速参考 | Esc |

Lowspeak 会拦截它所使用的 fn 按键，不让其传到其他 App；Shift 则照常传递。

## 隐私和安全

Lowspeak 会在你按住 fn 时听到你说的话，能向任何 App 输入内容，还保管着你的 API 密钥，因此它对这些东西的处理尽可能少。

- **你的声音只去一个地方。** 在在线模式下，压缩后的录音会连同你的词汇表一起，发送给你选择的服务提供商 OpenAI 或 Groq。在本地模式下，任何内容都不会离开你的 Mac。在线请求失败时，本地模型会在你的 Mac 上转写同一段录音。
- **后处理** 如果开启，会把转写的文本和你的词汇表发送给 Groq 或 OpenAI。在“格式化”模式下，还会附上你听写所用 App 的类型（代码编辑器、终端、聊天或邮件）；不会发送 App 的名称。
- **没有遥测，无需账户。** 除了转写和后处理之外，Lowspeak 只在以下情况下联网：在在线模式或开启后处理时，于你按住 fn 期间建立与服务提供商的连接；以及在你要求时，通过 HTTPS 从 Hugging Face 下载固定版本的模型，并根据校验和核对每个文件。**帮助 › 报告问题…** 会在浏览器中打开 GitHub 上的表单；其地址只带有 Lowspeak 的版本和 macOS 的版本，不含你听写的任何内容。
- **API 密钥** 保存在你的登录钥匙串中。它们只会发送给各自的服务提供商，并且在任何内容写入日志之前就会从错误信息中移除。
- **听写的文本不会被存储。** 最近的十条听写保留在内存中，供 **最近的听写** 使用，Lowspeak 退出后即消失。日志文件只记录长度和耗时，从不记录文本。
- **剪贴板** 会在粘贴前保存，并在目标 App 读取文本后恢复，除非在此期间有其他 App 更改了它。听写内容会被标记为临时内容，因此遵循这一标记的剪贴板历史 App 会跳过它。
- **代码签名** 使用 Hardened Runtime；App 只请求麦克风、无障碍、通知和网络权限。

如需报告漏洞，请遵循[安全策略](../../../SECURITY.md)。

## 速度

在 Apple 芯片上测得：

- **Parakeet Ultra** 能在约 0.3 秒内转写 40 秒的语音。
- **在线：** 停顿一段时间后的第一个请求在 Groq 上约需 0.4 秒，因为 Lowspeak 在你还在说话时就建立了连接；之后的请求会复用该连接，并在服务提供商支持时使用 HTTP/3。
- **上传大小：** 以 48 kbit/s 的 AAC 编码，每秒语音约 6 KB。
- **麦克风准备时间：** 内建麦克风约 0.07 秒，Studio Display 约半秒，AirPods 一到两秒。

## 已知限制

- **Intel Mac 上的本地模型** 在处理器上运行，速度慢得多；在这类 Mac 上，在线转写是更好的选择。
- **麦克风就绪之前说的话会丢失。** 请等待录音图标出现，使用 AirPods 时尤其如此。
- **fn 键可能被其他程序占用。** 当 macOS 的 🌐 键设置、Warp 或 macOS 听写先占用了 fn，Lowspeak 就检测不到它；参见[疑难解答](#疑难解答)。
- **Parakeet** 会忽略词汇表和语言选择，并且只认识欧洲语言。
- **字形和变体：** 只有 GPT Transcribe 可以被指定用繁体中文书写；没有任何模型可以被指定用欧洲葡萄牙语而不是巴西葡萄牙语书写。
- **时间限制：** 按住 fn 时两分钟，锁定时五分钟。
- **麦克风模式：** 控制中心也为 Lowspeak 提供“语音突显”和“宽谱”，并显示为已启用，但 Lowspeak 录音时不做语音处理，因此该选择不起作用。
- **最近的听写** 只保存在内存中，退出后即消失。
- **设计如此：** 没有设置窗口，没有程序坞图标，没有自动更新，不在 App Store 上架。

## 疑难解答

**按住 fn 时没有任何反应。** 检查三件事：在 **系统设置 › 隐私与安全 › 无障碍**（macOS 27 中为 **设备控制和数据访问**）中已允许 Lowspeak；**系统设置 › 键盘 › 按下🌐键时** 设为 **不执行任何操作**；并且没有其他 App 占用 fn。Warp 默认会占用；请用 `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` 关闭它，然后重新启动 Warp。macOS 听写也可能被设置为用 fn 启动。

**更新后，Lowspeak 不再响应 fn。** macOS 将无障碍权限与 App 的签名绑定。如果签名变了，例如从你自己构建的版本换成下载的 App，请将 Lowspeak 从无障碍列表（macOS 27 中为“设备控制和数据访问”）中移除，然后重新添加。

**开头几个字丢了。** 请在录音图标出现后再开始说话。使用 AirPods 时，麦克风需要一到两秒；在 **麦克风** 下选择内建或 USB 麦克风，几乎可以立即就绪。

**“麦克风无声”。** 麦克风五秒钟没有传来声音。使用 AirPods 时，可能是另一台设备正在使用它们的麦克风；请切换到内建麦克风，或重新连接 AirPods。合上 MacBook 的盖子也会使内建麦克风静音。

**听写结果是空的。** 输入音量可能太低：Teams、Zoom 或 Meet 等 App 会在通话时更改它，而 macOS 会按设备分别保存音量。请在 **系统设置 › 声效 › 输入** 中调高。

**文本出现在了错误的位置。** 菜单中的 **最近的听写** 可以将它再次拷贝到剪贴板。

**“无 API 密钥”。** 在菜单中选取 **设置 API 密钥…**，或切换到本地模型。

**第一次本地转写耗时很长。** 每个模型在首次使用时都要针对你的 Mac 做准备；此后载入只需不到一秒。

**Lowspeak 启动后立即退出。** 另一个 Lowspeak 已在运行；请先退出那一个。

**没有出现通知。** 请在 **系统设置 › 通知 › Lowspeak** 中允许通知。

**其他问题。** `~/Library/Application Support/Lowspeak/` 中的日志（`appstate.log`、`audio.log`、`hotkey.log`、`crash.log`）会记录发生了什么，但不含你听写的文本。请将相关的行附在[问题报告](https://github.com/sventmueller/lowspeak/issues/new/choose)中。

## 开发者须知

构建、测试、代码结构及其遵循的规则见 [docs/development.md](../../../docs/development.md)，贡献方式见 [CONTRIBUTING.md](../../../CONTRIBUTING.md)（均为英文）。

### 第三方组件

| 组件 | 版本 | 用途 | 许可证 |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp)，通过 [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | 在 GPU（Metal）上运行的本地 Whisper | MIT；whisper-rs 为 Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet 解码和长录音合并，已在 `src/transcription/parakeet` 中翻译为 Rust | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) 及其框架 crate | 0.6 / 0.3 | AppKit、CoreML、CoreAudio、AVFoundation 和其他系统框架 | MIT；Zlib、Apache-2.0 或 MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | 通过 CoreAudio 采集麦克风音频 | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | 重采样为 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq)（使用 rustls） | 3.5 | 下载模型 | MIT 或 Apache-2.0；rustls 和 ring 采用 ISC 和 Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | 所下载模型的校验和 | MIT 或 Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | 钥匙串中的 API 密钥 | MIT 或 Apache-2.0 |
| [serde](https://serde.rs)、serde_json、anyhow、unicode-general-category | 1 | JSON、错误处理、Parakeet 的词表 | MIT 或 Apache-2.0；Apache-2.0 |

App 内所有组件的许可证文本都随 App 一起提供，位于 `Contents/Resources/Acknowledgements.txt`。

模型不包含在 Lowspeak 中；它会在你要求时下载：

| 模型 | 来源 | 许可证 |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp)（ggml 格式的 OpenAI Whisper） | MIT |
| Parakeet v2、Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference) 对 NVIDIA Parakeet TDT 的 CoreML 转换 | CC-BY-4.0 |

## 许可证

Lowspeak 依据以下任一许可证授权

- Apache License, Version 2.0（[LICENSE-APACHE](../../../LICENSE-APACHE)）
- MIT License（[LICENSE-MIT](../../../LICENSE-MIT)）

由你选择。

除非你明确另行声明，否则你有意提交以纳入 Lowspeak 的任何贡献，按 Apache-2.0 许可证中的定义，均应按上述方式双重授权，不附加任何其他条款或条件。
