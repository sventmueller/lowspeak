<p><img src="../../readme-icon.png" alt="Ikon app Lowspeak" width="168" height="168"></p>

# Lowspeak

**Perencanaan untuk Mac yang menaip tepat di tempat kursor anda berada: tahan fn, bercakap, lepaskan.**

Lowspeak tinggal dalam bar menu. Tahan kekunci fn (Glob) dan bercakap; apabila anda melepaskannya, perkataan anda muncul dalam mana-mana app yang sedang anda gunakan untuk menaip, seolah-olah anda menaipnya sendiri. Ia mentranskripsi dalam talian dengan OpenAI atau Groq, atau sepenuhnya pada Mac anda dengan Whisper atau NVIDIA Parakeet, yang menukar 40 saat pertuturan kepada teks dalam kira-kira satu pertiga saat. Apabila rangkaian atau perkhidmatan dalam talian gagal, model setempat yang dipasang mengambil alih, jadi perencanaan anda tidak hilang. Ajarkan nama dan istilah yang anda gunakan, biarkan model bahasa membuang kata pengisi jika anda mahu, dan kunci rakaman dengan Shift untuk perencanaan teks yang lebih panjang secara bebas tangan. Ia ditulis dalam Rust dengan kawalan AppKit tulen, tersedia dalam 35 bahasa dan menghantar suara anda hanya kepada penyedia yang anda pilih.

[![Muat turun Lowspeak untuk macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Muat turun Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: ditandatangani dan dinotarisasi oleh Apple, untuk macOS 14 atau lebih baharu pada Apple Silicon dan Intel.

**Jumpa pepijat?** [Buka isu](https://github.com/sventmueller/lowspeak/issues/new/choose), atau pilih **Bantuan › Laporkan Masalah…** dalam menu Lowspeak: borang dibuka dengan versi anda sudah diisi. Jika Lowspeak berguna kepada anda, [bintang di GitHub](https://github.com/sventmueller/lowspeak) membantu orang lain menemuinya.

[![Edisi Rust 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 atau lebih baharu](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon dan Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit melalui objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp pada Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet pada Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Dalam talian dengan OpenAI atau Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 bahasa antara muka](https://img.shields.io/badge/languages-35-4C9A2A)
![Ditandatangani dengan Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Lesen: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#lesen)

[Bahasa Indonesia](../id/README.md) ·
**Bahasa Melayu** ·
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
[繁體中文](../zh-Hant/README.md)

## Kandungan

- [Ciri-ciri sepintas lalu](#ciri-ciri-sepintas-lalu)
- [Keperluan](#keperluan)
- [Pemasangan](#pemasangan)
- [Bermula](#bermula)
  - [Keizinan](#keizinan)
  - [Pilih cara transkripsi](#pilih-cara-transkripsi)
  - [Menu](#menu)
  - [Rujukan pantas](#rujukan-pantas)
- [Perencanaan](#perencanaan)
  - [Tahan untuk bercakap](#tahan-untuk-bercakap)
  - [Kunci dengan Shift atau tekan dua kali](#kunci-dengan-shift-atau-tekan-dua-kali)
  - [Ke mana teks pergi](#ke-mana-teks-pergi)
  - [Perencanaan terbaharu](#perencanaan-terbaharu)
- [Transkripsi](#transkripsi)
  - [Model mana yang sesuai untuk anda](#model-mana-yang-sesuai-untuk-anda)
  - [Dalam talian: OpenAI dan Groq](#dalam-talian-openai-dan-groq)
  - [Pada Mac anda: Whisper dan Parakeet](#pada-mac-anda-whisper-dan-parakeet)
  - [Apabila perkhidmatan dalam talian gagal](#apabila-perkhidmatan-dalam-talian-gagal)
  - [Cara rakaman disediakan](#cara-rakaman-disediakan)
  - [Bahasa mengikut model](#bahasa-mengikut-model)
  - [Mac Intel](#mac-intel)
- [Kosa kata](#kosa-kata)
- [Pascapemprosesan](#pascapemprosesan)
- [Mikrofon](#mikrofon)
- [Kunci API](#kunci-api)
- [Pemberitahuan](#pemberitahuan)
- [Penampilan dan kebolehcapaian](#penampilan-dan-kebolehcapaian)
- [Bahasa](#bahasa)
- [Kekunci dan gerak isyarat](#kekunci-dan-gerak-isyarat)
- [Privasi dan keselamatan](#privasi-dan-keselamatan)
- [Kelajuan](#kelajuan)
- [Had yang diketahui](#had-yang-diketahui)
- [Penyelesaian masalah](#penyelesaian-masalah)
- [Untuk pembangun](#untuk-pembangun)
  - [Komponen pihak ketiga](#komponen-pihak-ketiga)
- [Lesen](#lesen)

## Ciri-ciri sepintas lalu

- **Tahan fn, bercakap, lepaskan.** Teks ditaip ke dalam app di hadapan, di mana-mana sahaja kursor berada: e-mel, sembang, penyunting, terminal atau pelayar. Tiada tetingkap untuk ditukar, tiada butang untuk diklik.
- **Kunci untuk teks yang lebih panjang.** Tekan Shift semasa menahan fn, atau tekan fn dua kali, dan rakaman terus berjalan tanpa kekunci ditahan, sehingga lima minit. Satu tekanan pada fn atau Shift menamatkannya.
- **Model yang sesuai untuk setiap tugas.** Groq untuk kelajuan tanpa menggunakan memori Mac anda, GPT Transcribe daripada OpenAI untuk hasil terbaik, Whisper atau NVIDIA Parakeet pada Mac anda untuk bekerja di luar talian dan memastikan suara anda kekal peribadi. Tukar dengan satu klik.
- **Tiada perencanaan yang hilang.** Apabila rangkaian atau perkhidmatan dalam talian gagal, model setempat yang dipasang mentranskripsi rakaman yang sama pada Mac anda.
- **Perkataan anda, dieja dengan betul.** Kosa kata nama dan istilah teknikal membimbing OpenAI, Groq, Whisper dan pascapemprosesan.
- **Teks yang kemas jika anda mahu.** Pascapemprosesan pilihan membuang kata pengisi, membetulkan tatabahasa dan hanya mengekalkan pembetulan anda apabila anda membetulkan diri sendiri, atau turut memformat teks mengikut jenis app tempat anda membuat perencanaan.
- **Teruskan bercakap.** Mulakan perencanaan seterusnya semasa yang terakhir masih ditranskripsi; teks tiba mengikut susunan anda menuturkannya.
- **Papan klip anda kekal milik anda.** Lowspeak menampal melalui papan klip dan memulangkan kandungan asalnya sejurus kemudian. Sepuluh perencanaan terakhir kekal dalam menu untuk disalin semula.
- **Natif sepenuhnya.** Item bar menu sebenar dengan menu AppKit, amaran dan pemberitahuan standard. Tiada Electron, tiada tetingkap, tiada ikon Dock.
- **Peribadi.** Tiada akaun, tiada telemetri. Audio hanya dihantar kepada penyedia yang anda pilih, atau tidak ke mana-mana langsung dalam mod setempat.
- **Dalam bahasa anda.** Perencanaan berfungsi dalam berpuluh-puluh bahasa, dan menu, dialog serta pemberitahuan tersedia dalam 35 bahasa.

## Keperluan

- macOS 14 Sonoma atau lebih baharu. Lowspeak dibangunkan dan diuji pada macOS 27.
- Mac dengan Apple Silicon atau pemproses Intel. Transkripsi setempat dibina untuk Apple Silicon, tempat Whisper berjalan pada GPU dan Parakeet pada Neural Engine; pada Mac Intel kedua-duanya berjalan pada pemproses dan jauh lebih perlahan, lihat [Mac Intel](#mac-intel).
- Kira-kira 17 MB untuk app, ditambah 75 MB hingga 3 GB bagi setiap model setempat yang anda muat turun.
- Untuk transkripsi dalam talian, kunci API daripada [OpenAI](https://platform.openai.com/api-keys) atau [Groq](https://console.groq.com/keys); kedua-duanya mengenakan bayaran untuk apa yang anda transkripsi. Transkripsi setempat tidak memerlukan akaun.

## Pemasangan

**Daripada imej cakera.** Muat turun [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), buka dan seret Lowspeak ke folder Aplikasi. Imej ini ditandatangani dengan Developer ID dan dinotarisasi oleh Apple, jadi ia dibuka tanpa amaran.

**Kemas kini.** Lowspeak daripada imej cakera mengemas kini dirinya sendiri melalui [Sparkle](https://sparkle-project.org). Pada pelancaran kali kedua, ia bertanya sama ada ia boleh mencari kemas kini sendiri, sekali sehari. **Semak Kemas Kini…** dalam menu menyemak serta-merta; **Semak Kemas Kini Secara Automatik** dan **Muat Turun dan Pasang Kemas Kini Secara Automatik** di bawahnya mengubah pilihan anda kemudian. Apabila semakan menemui versi baharu semasa anda bekerja dalam app lain, satu titik muncul pada cincin dalam bar menu, dan **Semak Kemas Kini…** membawa kemas kini itu ke hadapan. Lowspeak memasang kemas kini hanya jika ia membawa tandatangan pembangun; tetapan, kunci dan model anda kekal. Versi 1.0.0 tidak mempunyai pengemas kini: pasang versi yang lebih baharu di atasnya sekali sahaja, daripada [halaman keluaran](https://github.com/sventmueller/lowspeak/releases). Binaan anda sendiri juga tidak mempunyai pengemas kini.

**Daripada kod sumber.** Lihat [Membina & memasang](../../../docs/development.md#build--deploy) untuk prasyarat, kemudian:

```sh
make deploy
```

Ini membina Lowspeak, menggantikan salinan dalam folder Aplikasi anda dan melancarkannya.

**Buka apabila Log Masuk** dalam menu memulakan Lowspeak bersama Mac anda, melalui item log masuk macOS sendiri. Hanya satu salinan berjalan pada satu masa; salinan kedua terus keluar.

## Bermula

### Keizinan

Pada pelancaran pertama, Lowspeak meminta tiga keizinan:

- **Kebolehcapaian** (Seting Sistem › Privasi & Keselamatan › Kebolehcapaian; pada macOS 27 senarai ini dinamakan Kawalan Peranti dan Akses Data). Lowspeak memerlukannya untuk mengesan kekunci fn dan untuk menampal dengan ⌘V. macOS menawarkan untuk membuka senarai itu; hidupkan suis untuk Lowspeak. Mesej Lowspeak sendiri menamakan senarai itu seperti yang ditunjukkan oleh macOS anda.
- **Mikrofon.** Diminta sekali; tanpanya Lowspeak tidak merakam apa-apa.
- **Pemberitahuan.** Lowspeak menggunakannya untuk masalah yang tidak akan anda sedari dengan cara lain, seperti mikrofon yang senyap atau transkripsi yang gagal.

### Pilih cara transkripsi

Sediakan sekurang-kurangnya satu cara; anda boleh bertukar pada bila-bila masa dalam menu.

- **Dalam talian:** pilih **Setkan Kunci API…** dan tampal kunci daripada OpenAI (bermula dengan `sk-`) atau Groq (`gsk_`). Lowspeak mengecam penyedia daripada kunci itu. Kemudian pilih model di bawah **OpenAI** atau **Groq**.
- **Pada Mac anda:** pilih **Urus Model** › **Muat Turun Parakeet Ultra** atau model Whisper, kemudian pilih model itu di bawah **Setempat**.

[Model mana yang sesuai untuk anda](#model-mana-yang-sesuai-untuk-anda) membantu anda memilih. Kemudian tahan fn, sebut sesuatu dan lepaskan.

### Menu

Klik cincin dalam bar menu:

| Item | Fungsinya |
|---|---|
| **Perencanaan Terbaharu** | sepuluh teks terakhir; satu klik menyalin satu daripadanya ke papan klip semula |
| **Setempat** | model pada Mac anda, dikumpulkan kepada Whisper dan Parakeet; yang belum dimuat turun dikelabukan |
| **OpenAI**, **Groq** | model dalam talian; tanpa kunci, model ini dikelabukan dan menu menawarkan **Setkan Kunci API…** |
| **Bahasa** | bahasa perencanaan anda, atau **Kesan Automatik**; menu turut mengikutinya |
| **Pascapemprosesan** | **Tak Aktif**, **Bersihkan** atau **Formatkan**, lihat [Pascapemprosesan](#pascapemprosesan) |
| **Kosa Kata…** | nama dan istilah yang perlu dieja dengan tepat; menu menunjukkan berapa banyak yang telah anda masukkan |
| **Mikrofon** | **Lalai Sistem** atau mikrofon tetap, dan **Mainkan Bunyi Apabila Sedia** |
| **Urus Model** | muat turun dan padam model setempat, dengan paparan kemajuan |
| **Setkan Kunci API…** / **Kemas Kini Kunci API…** | tambah, ganti atau buang kunci |
| **Buka apabila Log Masuk** | mulakan Lowspeak bersama Mac anda; tanda semak menunjukkan pilihan ini aktif |
| **Semak Kemas Kini…** | cari versi baharu sekarang; dua item di bawahnya menentukan sama ada Lowspeak menyemak sekali sehari dan memasang kemas kini tanpa bertanya. Hanya dalam app daripada imej cakera |
| **Bantuan** | **Rujukan Pantas**, **Laporkan Masalah…**, **Cadangkan Ciri…**, **Lowspeak di GitHub** |
| **Perihal Lowspeak** | versi, hak cipta, halaman projek dan lesen |
| **Keluar daripada Lowspeak** | ⌘Q semasa menu dibuka |

Tanda semak di bawah **Setempat**, **OpenAI** dan **Groq** menandakan model yang sedang digunakan. Ikon menunjukkan apa yang sedang dilakukan Lowspeak:

| Ikon | Maksud |
|---|---|
| Cincin dengan titik | Sedia |
| Bulatan penuh | Merakam semasa fn ditahan |
| Segi empat bucu bulat | Rakaman dikunci; tekanan seterusnya pada fn atau Shift menamatkannya |
| Cincin berkelip | Mentranskripsi |
| Cincin dengan titik di kanan atas | Versi baharu sedang menunggu, lihat **Semak Kemas Kini…** |
| ↓ 42% | Memuat turun model |

Lowspeak tidak mempunyai tetingkap tetapan: semuanya ada dalam menu ini, dan ia mengingati setiap pilihan.

### Rujukan pantas

**Bantuan › Rujukan Pantas** menunjukkan gerak isyarat fn dalam tetingkap kecil, dalam bahasa anda, dengan butang yang membuka seting papan kekunci. **Laporkan Masalah…** membuka borang di GitHub dengan versi Lowspeak dan macOS anda sudah diisi, **Cadangkan Ciri…** membuka perbincangan untuk idea, dan **Lowspeak di GitHub** membuka halaman projek.

## Perencanaan

### Tahan untuk bercakap

Tahan fn, bercakap, lepaskan. Ikon rakaman muncul sebaik sahaja mikrofon anda menghantar bunyi; mulakan bercakap pada saat itu, kerana apa yang anda sebut sebelumnya tidak dirakam. Melepaskan fn menamatkan rakaman, dan teks menyusul sejurus kemudian.

Ketikan pendek pada fn tidak melakukan apa-apa, jadi fn masih boleh digunakan untuk fungsi lainnya. Rakaman yang kekal kosong atau berlangsung kurang daripada 0.3 saat dibuang.

### Kunci dengan Shift atau tekan dua kali

Untuk teks yang lebih panjang, kunci rakaman supaya ia berjalan tanpa kekunci ditahan. Terdapat tiga cara:

- **Shift semasa menahan fn.** Mulakan dengan fn seperti biasa dan tekan Shift semasa rakaman: rakaman dikunci, dan anda boleh melepaskan fn. Tiada apa yang anda sebut sebelumnya hilang, rakaman hanya diteruskan.
- **Shift, kemudian fn.** Tahan Shift dan tekan fn: rakaman bermula dalam keadaan dikunci.
- **fn dua kali.** Ketik fn sekejap (kurang daripada 0.3 saat) dan tekan sekali lagi dalam masa 0.4 saat. Rakaman berjalan dari tekanan pertama.

Ikon bertukar menjadi segi empat bucu bulat. Tekan **fn** atau **Shift** sekali untuk berhenti; teks menyusul seperti biasa. Jika kekunci 🌐 anda disetkan untuk membuka Emoji & Simbol, berhenti dengan Shift: tekanan pendek pada fn akan membuka pemapar itu.

Rakaman berhenti dengan sendirinya selepas dua minit semasa fn ditahan, dan selepas lima minit apabila dikunci, kemudian ditranskripsi seperti biasa.

### Ke mana teks pergi

Lowspeak menampal teks ke dalam app yang berada di hadapan ketika anda berhenti bercakap, di kedudukan kursor, melalui papan klip dan ⌘V. Ia menyimpan semua yang ada dalam papan klip sebelum itu dan memulangkannya setengah saat selepas app membaca teks itu, atau selepas sepuluh saat jika app tidak pernah membacanya, melainkan app lain mengubah papan klip di antaranya. Teks itu ditandakan sebagai sementara, jadi app sejarah papan klip yang mematuhi penanda itu tidak merekodkannya.

Anda boleh memulakan perencanaan seterusnya semasa yang terakhir masih ditranskripsi. Teks ditampal mengikut susunan anda menuturkannya, dan tidak sekali-kali semasa anda merakam atau menahan Shift, jadi tampalan tidak boleh mendarat di tengah-tengah ayat anda yang seterusnya.

Tanpa keizinan Kebolehcapaian, atau apabila ⌘V simulasi tidak dapat dihantar, teks kekal dalam papan klip dan pemberitahuan memaklumkannya; tampal sendiri dengan ⌘V.

### Perencanaan terbaharu

**Perencanaan Terbaharu** dalam menu menyenaraikan sepuluh teks terakhir, yang terbaharu dahulu, setiap satu dipendekkan kepada satu baris; menuding padanya menunjukkan keseluruhan teks. Satu klik menyalinnya ke papan klip, untuk ketika teks mendarat di tempat yang salah atau anda memerlukannya dua kali. Senarai ini disimpan dalam memori sahaja dan kosong selepas Lowspeak keluar.

## Transkripsi

### Model mana yang sesuai untuk anda

| Anda mahukan | Pilih | Sebabnya |
|---|---|---|
| **Kelajuan tanpa membebankan Mac anda** | **Groq › Whisper Large V3 Turbo** | pilihan dalam talian terpantas; tiada apa yang dimuat turun, dan memori serta ruang cakera Mac anda kekal bebas |
| **Hasil terbaik** | **OpenAI › GPT Transcribe** (lalai) | paling tepat antara model yang ditawarkan Lowspeak; menerima kosa kata anda sebagai kata kunci |
| **Luar talian dan peribadi** | **Setempat › Parakeet Ultra** untuk bahasa Eropah, **Setempat › Whisper Large v3 Turbo** untuk semua bahasa lain | berfungsi tanpa rangkaian, dan suara anda tidak pernah meninggalkan Mac anda; Parakeet paling pantas antara semuanya |
| **Tiada akaun, tiada kos** | mana-mana model **Setempat** | berjalan pada Mac anda; tiada apa yang dikenakan bayaran |
| **Kebolehpercayaan dan kelajuan sekali gus** | model dalam talian, ditambah Parakeet Ultra atau model Whisper yang dipasang | model setempat mengambil alih apabila perkhidmatan dalam talian gagal |
| **Mac Intel** | **Groq** atau **OpenAI** | model setempat berjalan pada pemproses di sana, lihat [Mac Intel](#mac-intel) |

Model setempat menggunakan ruang cakera sebesar saiz muat turunnya dan, semasa dipilih, memori yang lebih kurang sama; Lowspeak memastikan model kekal dimuatkan supaya perencanaan bermula tanpa lengah. Pada Mac dengan memori 8 GB, utamakan Parakeet Ultra atau model dalam talian berbanding model Whisper yang besar.

### Dalam talian: OpenAI dan Groq

| Model | Penyedia | Nota |
|---|---|---|
| **GPT Transcribe** (lalai) | OpenAI | paling tepat; menerima kosa kata anda sebagai kata kunci |
| **Whisper** | OpenAI | model Whisper yang asal |
| **Whisper Large V3** | Groq | sangat pantas |
| **Whisper Large V3 Turbo** | Groq | terpantas dalam talian |

Rakaman dimampatkan kepada AAC sebelum dimuat naik, kira-kira 6 KB bagi setiap saat pertuturan, jadi perencanaan turut berfungsi pada sambungan yang perlahan. Semasa anda menahan fn, Lowspeak sudah membuka sambungan kepada penyedia, jadi muat naik bermula sebaik sahaja anda melepaskannya.

### Pada Mac anda: Whisper dan Parakeet

| Model | Saiz | Berjalan pada | Bahasa |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 bahasa Eropah |
| **Parakeet v2** | ~600 MB | Neural Engine | Inggeris |
| **Whisper Large v3 Turbo** | ~1.6 GB | GPU (Metal) | kira-kira 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | kira-kira 99 |
| **Whisper Medium** | ~1.5 GB | GPU (Metal) | kira-kira 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | kira-kira 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | kira-kira 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | kira-kira 99 |

**Urus Model** memuat turun dan memadamnya; semasa muat turun, bar menu menunjukkan kemajuannya, dan keluar daripada Lowspeak membatalkannya. Penggunaan pertama sesuatu model menyediakannya untuk Mac anda: kira-kira 10 saat untuk Parakeet Ultra, sehingga 30 saat untuk Whisper; selepas itu ia dimuatkan dalam sepersekian saat. Jika model yang anda pilih dipadam, Lowspeak bertukar kepada model lain yang dipasang.

**Parakeet** ialah pilihan terpantas: ia mentranskripsi 40 saat pertuturan dalam kira-kira 0.3 saat. Lowspeak menjalankannya dengan model dan penyahkodan yang sama seperti [FluidAudio](https://github.com/FluidInference/FluidAudio) dan menyimpan model itu dalam folder FluidAudio (`~/Library/Application Support/FluidAudio/Models`), jadi app yang dibina atas FluidAudio berkongsi model yang sama. Rakaman yang lebih panjang daripada 15 saat ditranskripsi dalam tetingkap masa yang bertindih, yang dicantumkan tanpa kehilangan perkataan pada sambungannya. **Whisper** memahami lebih banyak bahasa dan membaca kosa kata anda; model besarnya paling tepat, model kecilnya jauh lebih lemah di luar bahasa Inggeris. Model Whisper disimpan dalam `~/Library/Application Support/Lowspeak/Models`.

### Apabila perkhidmatan dalam talian gagal

Jika transkripsi dalam talian gagal, kerana rangkaian terputus, penyedia menghadapi masalah atau kunci salah, model setempat yang dipasang mentranskripsi rakaman yang sama: model yang dipilih di bawah **Setempat**, jika tiada, Parakeet Ultra, kemudian Whisper Large v3 Turbo, Large v3 dan model Whisper yang lebih kecil, dengan Parakeet v2 yang terakhir. Satu pemberitahuan memaklumkan anda sekali; ia muncul semula hanya selepas ada perencanaan dalam talian yang berjaya di antaranya.

Dengan model setempat sebagai sandaran, Lowspeak memberi permintaan dalam talian 20 saat; tanpanya, ia mencuba tiga kali, setiap satu sehingga satu minit. Dalam mod dalam talian, Lowspeak menyediakan model Parakeet sandaran sekali semasa pelancaran, jadi sandaran pertama tidak perlu menunggunya.

### Cara rakaman disediakan

Lowspeak merakam pada kadar sampel mikrofon anda sendiri dan menukarnya kepada 16 kHz yang dijangkakan oleh model sebaik sahaja anda berhenti. Sebelum transkripsi, ia membuang dengung rendah di bawah 80 Hz, memangkas kesenyapan di awal dan akhir, dan menyamakan kelantangan, jadi penutur yang perlahan dan yang lantang menghasilkan keputusan yang sama. Rakaman tanpa pertuturan tidak dihantar langsung. Dalam mod setempat, Parakeet mula-mula menerima rakaman asal yang tidak diusik, dan rakaman yang telah disediakan hanya jika yang pertama tidak menghasilkan teks.

### Bahasa mengikut model

Bahasa di bawah **Bahasa** dihantar kepada model dalam talian dan kepada Whisper. Dengan **Kesan Automatik**, model menentukan bahasa itu sendiri.

| Model | Bahasa yang boleh digunakan untuk perencanaan |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, pada Mac anda) | kira-kira 99, termasuk kesemua 35 bahasa menu |
| Parakeet Ultra | Bulgaria, Croatia, Czech, Denmark, Belanda, Inggeris, Estonia, Finland, Perancis, Jerman, Yunani, Hungary, Itali, Latvia, Lithuania, Malta, Poland, Portugis, Romania, Rusia, Slovak, Slovenia, Sepanyol, Sweden, Ukraine |
| Parakeet v2 | Inggeris |

Parakeet mengesan bahasa sendiri dan mengabaikan pilihan dalam menu. **Cina (Tradisional)** mengarahkan GPT Transcribe untuk menulis aksara tradisional; Whisper menulis bahasa Cina dalam mana-mana satu daripada dua tulisan itu dan bahasa Portugis tanpa mengambil kira variannya.

### Mac Intel

Lowspeak ialah app universal dan turut berjalan pada Mac Intel dengan macOS 14. Semua fungsi dalam talian berfungsi sama. Model setempat tidak mendapat perkakasan yang menjadi sasaran binaannya di sana: Whisper berjalan pada pemproses dan bukannya GPU, dan Parakeet pada pemproses dan bukannya Neural Engine. Kedua-duanya berfungsi, tetapi mengambil masa berkali-kali ganda lebih lama berbanding pada Apple Silicon, dan model Whisper yang besar terlalu lama untuk perencanaan. Pada Mac Intel, transkripsi dalam talian dan simpan Whisper Small atau Parakeet Ultra sebagai [sandaran](#apabila-perkhidmatan-dalam-talian-gagal).

## Kosa kata

**Kosa Kata…** menerima nama dan istilah yang perlu dieja dengan tepat oleh Lowspeak, dipisahkan dengan koma: nama produk, rakan sekerja, istilah teknikal. Sehingga 100 istilah, setiap satu sehingga 60 aksara; `<` dan `>` dibuang.

- **GPT Transcribe** menerimanya sebagai kata kunci.
- **Whisper** (OpenAI, Groq dan pada Mac anda) membacanya sebagai petunjuk sebelum rakaman, sehingga 500 aksara.
- **Pascapemprosesan** diarahkan untuk mengejanya tepat seperti yang ditulis.
- **Parakeet** tidak menggunakannya.

## Pascapemprosesan

Selepas transkripsi, model bahasa boleh mengolah semula teks. Pilih di bawah **Pascapemprosesan**:

- **Tak Aktif** (lalai) — teks sebagaimana ditranskripsi.
- **Bersihkan** — membuang kata pengisi seperti “emm”, “kan” atau “sebenarnya”, membetulkan tatabahasa dan tanda baca, dan hanya mengekalkan pembetulan apabila anda membetulkan diri sendiri. Ia tidak menambah apa-apa dan tidak mengubah maksud.
- **Formatkan** — seperti Bersihkan, dan menyesuaikan teks dengan app tempat anda membuat perencanaan: istilah gaya kod dalam penyunting kod, baris arahan dalam terminal, nada santai dalam app sembang, nada profesional dalam e-mel.

Pascapemprosesan menggunakan Groq (`openai/gpt-oss-120b`) jika anda mempunyai kunci Groq, jika tidak OpenAI (`gpt-4o-mini`), juga apabila anda mentranskripsi secara setempat. Jika bahasa dipilih di bawah **Bahasa**, ia mengekalkan bahasa itu. Jika ia mengambil masa lebih daripada lapan saat, gagal, atau memulangkan jawapan yang terpotong atau jauh lebih panjang daripada apa yang anda katakan, Lowspeak menampal teks yang belum diproses. Jika penyedia menolak permintaan atau tiada kunci disetkan, satu pemberitahuan memaklumkannya sekali.

## Mikrofon

Di bawah **Mikrofon**, pilih mikrofon tetap atau **Lalai Sistem**. Lowspeak mengingati peranti itu; selagi ia tidak disambungkan, menu menunjukkannya sebagai **(Tidak Disambung)** dan Lowspeak merakam dengan lalai sistem.

Mikrofon dihidupkan hanya semasa anda membuat perencanaan. Menyediakannya mengambil masa kira-kira 0.07 saat dengan mikrofon terbina dalam, kira-kira setengah saat dengan Studio Display, dan satu hingga dua saat dengan AirPods, yang terlebih dahulu bertukar kepada mod set kepala. Ikon rakaman muncul sebaik sahaja bunyi tiba; **Mainkan Bunyi Apabila Sedia** menambah bunyi pendek pada saat itu. Pilihan ini tidak aktif secara lalai, kerana pada sesetengah pembesar suara, bunyi itu sendiri tiba setengah saat lewat.

Jika mikrofon hanya menghantar kesenyapan selama lima saat, seperti yang berlaku pada AirPods apabila peranti lain sedang memegang mikrofonnya, Lowspeak memberitahu anda.

## Kunci API

**Setkan Kunci API…** meminta kunci dan mengecam penyedia daripada permulaannya: `sk-` untuk OpenAI, `gsk_` untuk Groq. Kunci dalam format lain ditolak dengan penjelasan. Masukkan satu kunci bagi setiap penyedia; dengan kunci untuk kedua-duanya, semua model dalam talian tersedia. **Kemas Kini Kunci API…** menggantikan kunci atau, dengan **Keluarkan Semua Kunci**, memadamnya.

Kunci disimpan dalam rantai kunci log masuk anda dengan nama `com.lowspeak.app`. Fail kunci daripada versi terdahulu (`~/Library/Application Support/Lowspeak/.api-key`) dipindahkan ke sana pada pelancaran seterusnya dan dipadam sebaik sahaja setiap kunci dapat dibaca semula daripada rantai kunci.

## Pemberitahuan

Lowspeak bekerja di latar belakang, jadi ia melaporkan masalah sebagai pemberitahuan:

| Pemberitahuan | Apa yang berlaku |
|---|---|
| **Rakaman Gagal** | tiada akses mikrofon, tiada mikrofon disambungkan, atau mikrofon tidak dapat dimulakan |
| **Mikrofon Senyap** | lima saat kesenyapan, lihat [Mikrofon](#mikrofon) |
| **Keizinan Kebolehcapaian Diperlukan** | teks berada dalam papan klip; benarkan Kebolehcapaian untuk menampal secara automatik |
| **Penampalan Gagal** | teks berada dalam papan klip; tampalkannya dengan ⌘V |
| **Transkripsi Gagal** | dengan sebabnya: tiada kunci, kunci ditolak, had kadar, ralat pelayan atau rangkaian, atau tiada model setempat |
| **Ditranskripsi pada Mac Ini** | perkhidmatan dalam talian gagal dan model setempat mengambil alih |
| **Pascapemprosesan Gagal** | penyedia menolak pascapemprosesan atau tiada kunci disetkan; teks ditampal tanpanya |
| **Tiada Kunci API** | anda memilih model dalam talian tanpa kunci untuk penyedianya |
| **Muat Turun Gagal**, **Pemadaman Gagal**, **Tidak Dapat Memadam** | pengurusan model setempat tidak berjaya, atau operasi lain masih berjalan |
| **Ralat Item Log Masuk**, **Ralat Kunci API** | macOS atau rantai kunci menolak perubahan |

Pemberitahuan tidak sekali-kali mengandungi teks perencanaan anda. Perkhidmatan dalam talian yang gagal dan penyedia pascapemprosesan yang menolak dilaporkan sekali, bukan selepas setiap perencanaan, sehingga ia berfungsi semula.

## Penampilan dan kebolehcapaian

- **Ikon bar menu** ialah imej templat yang mengikut bar menu cerah, gelap atau berwarna.
- **VoiceOver** membaca ikon sebagai “Lowspeak”, dan menu, amaran serta pemberitahuan ialah kawalan macOS standard.
- **Tiada kekunci yang perlu ditahan.** Kunci rakaman dengan Shift atau dengan menekan fn dua kali, dan tekanan seterusnya menamatkannya.
- **Rujukan Pantas** ialah tetingkap ringkas dengan label dan butang, yang boleh dicapai daripada papan kekunci; Esc menutupnya.

Kekurangan yang diketahui disenaraikan dalam [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver tidak mengumumkan apabila rakaman bermula, dan Lowspeak belum lagi diperiksa dengan VoiceOver atau Kawalan Suis.

## Bahasa

Menu, dialog dan pemberitahuan diterjemahkan ke dalam 35 bahasa:

Arab, Katalan, Cina (Ringkas), Cina (Tradisional), Croatia, Czech, Denmark, Belanda, Inggeris, Finland, Perancis, Jerman, Yunani, Ibrani, Hindi, Hungary, Indonesia, Itali, Jepun, Korea, Melayu, Norway, Poland, Portugis (Brazil), Portugis (Portugal), Romania, Rusia, Slovak, Slovenia, Sepanyol, Sweden, Thai, Turki, Ukraine dan Vietnam.

Bahasa yang anda pilih di bawah **Bahasa** juga menjadi bahasa menu. Dengan **Kesan Automatik**, menu mengikut susunan bahasa dalam **Seting Sistem › Umum › Bahasa & Rantau**, dan menggunakan bahasa Inggeris jika tiada terjemahan. Istilah menu mengikut terjemahan Apple sendiri, jadi Lowspeak terasa seperti bahagian lain Mac anda. Selain bahasa Inggeris, terjemahan dibuat oleh mesin dan belum disemak oleh penutur asli; pembetulan dialu-alukan sebagai [isu](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Kekunci dan gerak isyarat

| Tindakan | Kekunci |
|---|---|
| Perencanaan semasa ditahan | tahan fn |
| Kunci rakaman yang sedang berjalan | Shift semasa menahan fn |
| Mulakan rakaman berkunci | Shift+fn, atau fn dua kali dengan pantas |
| Tamatkan rakaman berkunci | fn atau Shift |
| Keluar daripada Lowspeak (menu dibuka) | ⌘Q |
| Tutup Rujukan Pantas | Esc |

Lowspeak menghalang tekanan fn yang digunakannya daripada sampai ke app lain; Shift diteruskan seperti biasa.

## Privasi dan keselamatan

Lowspeak mendengar apa yang anda katakan semasa fn ditahan, boleh menaip ke dalam mana-mana app dan menyimpan kunci API anda, jadi ia melakukan sesedikit mungkin dengan semua itu.

- **Suara anda pergi ke satu tempat sahaja.** Dalam mod dalam talian, rakaman dihantar dalam bentuk termampat kepada penyedia yang anda pilih, OpenAI atau Groq, bersama kosa kata anda. Dalam mod setempat, tiada apa yang meninggalkan Mac anda. Apabila permintaan dalam talian gagal, model setempat mentranskripsi rakaman yang sama pada Mac anda.
- **Pascapemprosesan**, jika anda menghidupkannya, menghantar teks yang ditranskripsi dan kosa kata anda kepada Groq atau OpenAI. Dalam mod Formatkan, ia turut menghantar jenis app tempat anda membuat perencanaan (penyunting kod, terminal, sembang atau e-mel); nama app tidak dihantar.
- **Tiada telemetri, tiada akaun.** Selain transkripsi dan pascapemprosesan, Lowspeak hanya menyambung ke internet untuk membuka sambungan kepada penyedia anda semasa fn ditahan, dalam mod dalam talian atau apabila pascapemprosesan dihidupkan, untuk memuat turun model apabila anda memintanya, daripada Hugging Face melalui HTTPS, pada versi yang tetap dan dengan setiap fail disemak berbanding hasil tambah semaknya, dan untuk mencari versi baharu di GitHub, sekali sehari jika anda membenarkannya. Permintaan itu membawa versi Lowspeak dan tiada apa-apa lagi. **Bantuan › Laporkan Masalah…** membuka borang di GitHub dalam pelayar anda; alamatnya membawa versi Lowspeak dan versi macOS, bukan apa-apa daripada perencanaan anda.
- **Kunci API** kekal dalam rantai kunci log masuk anda. Kunci hanya dihantar kepada penyedianya sendiri dan dibuang daripada mesej ralat sebelum apa-apa ditulis ke log.
- **Teks perencanaan tidak disimpan.** Sepuluh perencanaan terakhir kekal dalam memori untuk **Perencanaan Terbaharu** dan hilang apabila Lowspeak keluar. Fail log merekodkan panjang dan masa, tidak sekali-kali teksnya.
- **Papan klip** disimpan sebelum menampal dan dipulihkan sebaik sahaja app membaca teks itu, melainkan app lain mengubahnya di antaranya. Perencanaan ditandakan sebagai sementara, jadi app sejarah papan klip yang mematuhi penanda itu melangkaunya.
- **Tandatangan kod** dengan Hardened Runtime; app hanya meminta akses kepada mikrofon, Kebolehcapaian, pemberitahuan dan rangkaian. Kemas kini mesti membawa tandatangan pembangun; jika tidak, Sparkle menolaknya.

Untuk melaporkan kerentanan, ikut [dasar keselamatan](../../../SECURITY.md).

## Kelajuan

Diukur pada Apple Silicon:

- **Parakeet Ultra** mentranskripsi 40 saat pertuturan dalam kira-kira 0.3 saat.
- **Dalam talian:** permintaan pertama selepas jeda mengambil masa kira-kira 0.4 saat di Groq, kerana Lowspeak membuka sambungan semasa anda masih bercakap; permintaan seterusnya menggunakan semula sambungan itu, melalui HTTP/3 jika penyedia menawarkannya.
- **Saiz muat naik:** kira-kira 6 KB bagi setiap saat pertuturan, sebagai AAC pada 48 kbit/s.
- **Menyediakan mikrofon:** kira-kira 0.07 saat untuk mikrofon terbina dalam, setengah saat pada Studio Display, satu hingga dua saat dengan AirPods.

## Had yang diketahui

- **Model setempat pada Mac Intel** berjalan pada pemproses dan jauh lebih perlahan; transkripsi dalam talian ialah pilihan yang lebih baik di sana.
- **Apa yang anda sebut sebelum mikrofon sedia akan hilang.** Tunggu ikon rakaman, terutamanya dengan AirPods.
- **Kekunci fn boleh diambil oleh pihak lain.** Apabila seting kekunci 🌐 macOS, Warp atau Perencanaan macOS menuntut fn terlebih dahulu, Lowspeak tidak nampak kekunci itu; lihat [Penyelesaian masalah](#penyelesaian-masalah).
- **Parakeet** mengabaikan kosa kata dan pilihan bahasa, dan hanya mengenali bahasa Eropah.
- **Tulisan dan varian:** hanya GPT Transcribe boleh diarahkan untuk menulis bahasa Cina Tradisional; tiada model boleh diarahkan untuk menulis bahasa Portugis Eropah dan bukannya Portugis Brazil.
- **Had masa:** dua minit semasa fn ditahan, lima minit apabila dikunci.
- **Mod Mik:** Pusat Kawalan menawarkan Asingkan Suara dan Spektrum Lebar untuk Lowspeak dan melaporkannya sebagai aktif, tetapi Lowspeak merakam tanpa pemprosesan suara, jadi pilihan itu tiada kesan.
- **Perencanaan Terbaharu** disimpan dalam memori sahaja dan hilang selepas keluar.
- **Dengan reka bentuk:** tiada tetingkap tetapan, tiada ikon Dock, tiada dalam App Store.

## Penyelesaian masalah

**Tiada apa-apa berlaku apabila saya menahan fn.** Semak tiga perkara: Lowspeak dibenarkan di bawah **Seting Sistem › Privasi & Keselamatan › Kebolehcapaian** (pada macOS 27: **Kawalan Peranti dan Akses Data**); **Seting Sistem › Papan Kekunci › Tekan kekunci 🌐 untuk** disetkan kepada **Jangan Buat Apa-apa**; dan tiada app lain menuntut fn. Warp melakukannya secara lalai; matikannya dengan `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` dan mulakan semula Warp. Perencanaan macOS juga boleh disetkan untuk bermula dengan fn.

**Selepas kemas kini, Lowspeak tidak lagi bertindak balas kepada fn.** macOS mengikat keizinan Kebolehcapaian kepada tandatangan app. Jika tandatangan berubah, contohnya daripada binaan anda sendiri kepada app yang dimuat turun, buang Lowspeak daripada senarai Kebolehcapaian (pada macOS 27: Kawalan Peranti dan Akses Data) dan tambahkannya semula.

**Perkataan pertama hilang.** Mula bercakap sebaik sahaja ikon rakaman muncul. Dengan AirPods, mikrofon memerlukan satu hingga dua saat; mikrofon terbina dalam atau USB yang dipilih di bawah **Mikrofon** sedia hampir serta-merta.

**“Mikrofon Senyap.”** Mikrofon tidak menghantar bunyi selama lima saat. Dengan AirPods, peranti lain mungkin sedang menggunakan mikrofonnya; tukar kepada mikrofon terbina dalam atau sambungkan AirPods semula. Penutup MacBook yang tertutup juga membisukan mikrofon terbina dalam.

**Perencanaan keluar kosong.** Kelantangan input mungkin terlalu rendah: app seperti Teams, Zoom atau Meet mengubahnya semasa panggilan, dan macOS menyimpannya bagi setiap peranti. Naikkannya di bawah **Seting Sistem › Bunyi › Input**.

**Teks mendarat di tempat yang salah.** **Perencanaan Terbaharu** dalam menu menyalinnya ke papan klip semula.

**“Tiada Kunci API.”** Pilih **Setkan Kunci API…** dalam menu, atau tukar kepada model setempat.

**Transkripsi setempat pertama mengambil masa yang lama.** Setiap model disediakan untuk Mac anda pada penggunaan pertamanya; selepas itu ia dimuatkan dalam sepersekian saat.

**Lowspeak keluar sejurus selepas dilancarkan.** Satu lagi salinan Lowspeak sudah berjalan; keluar daripada salinan itu dahulu.

**Tiada pemberitahuan muncul.** Benarkannya di bawah **Seting Sistem › Pemberitahuan › Lowspeak**.

**Masalah lain.** Log dalam `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) menerangkan apa yang berlaku, tanpa teks perencanaan anda. Lampirkan baris yang berkaitan pada [laporan masalah](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Untuk pembangun

Cara membina, menguji, struktur kod dan peraturan yang dipatuhinya diterangkan dalam [docs/development.md](../../../docs/development.md) (dalam bahasa Inggeris); cara menyumbang dalam [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Komponen pihak ketiga

| Komponen | Versi | Digunakan untuk | Lesen |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) melalui [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | Whisper setempat pada GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | penyahkodan Parakeet dan pencantuman rakaman panjang, diterjemahkan ke Rust dalam `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) dan crate rangka kerjanya | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation dan rangka kerja sistem yang lain | MIT; Zlib, Apache-2.0 atau MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | tangkapan mikrofon melalui CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | pensampelan semula kepada 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) dengan rustls | 3.5 | muat turun model | MIT atau Apache-2.0; rustls dan ring di bawah ISC dan Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | hasil tambah semak muat turun model | MIT atau Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | kunci API dalam rantai kunci | MIT atau Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, ralat, kosa kata Parakeet | MIT atau Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | kemas kini app daripada imej cakera | MIT, dengan bahagian di bawah lesen BSD dan zlib |

Teks lesen bagi semua yang ada di dalam app disertakan bersamanya sebagai `Contents/Resources/Acknowledgements.txt`.

Model bukan sebahagian daripada Lowspeak; ia memuat turunnya apabila anda memintanya:

| Model | Sumber | Lesen |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper daripada OpenAI dalam format ggml) | MIT |
| Parakeet v2, Parakeet Ultra | Penukaran CoreML oleh [FluidInference](https://huggingface.co/FluidInference) bagi Parakeet TDT daripada NVIDIA | CC-BY-4.0 |

## Lesen

Lowspeak dilesenkan di bawah mana-mana satu daripada

- Lesen Apache, Versi 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Lesen MIT ([LICENSE-MIT](../../../LICENSE-MIT))

mengikut pilihan anda.

Melainkan anda menyatakan sebaliknya secara jelas, sebarang sumbangan yang sengaja dikemukakan untuk dimasukkan ke dalam Lowspeak oleh anda, seperti yang ditakrifkan dalam lesen Apache-2.0, akan dilesenkan dua kali seperti di atas, tanpa sebarang terma atau syarat tambahan.
