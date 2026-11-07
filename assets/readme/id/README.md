<p><img src="../../readme-icon.png" alt="Ikon app Lowspeak" width="168" height="168"></p>

# Lowspeak

**Dikte untuk Mac yang mengetik tepat di posisi kursor Anda: tahan fn, bicara, lepaskan.**

Lowspeak tinggal di bar menu. Tahan tombol fn (Globe) dan bicaralah; begitu Anda melepaskannya, kata-kata Anda muncul di app mana pun tempat Anda sedang mengetik, seolah-olah Anda mengetiknya sendiri. Lowspeak mentranskripsi secara online dengan OpenAI atau Groq, atau sepenuhnya di Mac Anda dengan Whisper atau NVIDIA Parakeet, yang mengubah 40 detik ucapan menjadi teks dalam sekitar sepertiga detik. Saat jaringan atau layanan online gagal, model lokal yang terinstal mengambil alih, sehingga dikte tidak hilang. Ajarkan nama dan istilah yang Anda gunakan, biarkan model bahasa menghapus kata pengisi jika Anda mau, dan kunci rekaman dengan Shift untuk mendikte teks yang lebih panjang tanpa tangan. Lowspeak ditulis dalam Rust dengan kontrol AppKit asli, tersedia dalam 35 bahasa, dan mengirim suara Anda hanya ke penyedia yang Anda pilih.

[![Unduh Lowspeak untuk macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Unduh Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: ditandatangani dan dinotarisasi oleh Apple, untuk macOS 14 atau yang lebih baru di Apple Silicon dan Intel.

**Menemukan bug?** [Buka issue](https://github.com/sventmueller/lowspeak/issues/new/choose), atau pilih **Bantuan › Laporkan Masalah…** di menu Lowspeak: formulirnya terbuka dengan versi Anda sudah terisi. Jika Lowspeak berguna bagi Anda, [bintang di GitHub](https://github.com/sventmueller/lowspeak) membantu orang lain menemukannya.

[![Rust edisi 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 atau yang lebih baru](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon dan Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit melalui objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp di Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet di Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online dengan OpenAI atau Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 bahasa antarmuka](https://img.shields.io/badge/languages-35-4C9A2A)
![Ditandatangani dengan Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Lisensi: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#lisensi)

**Bahasa Indonesia** ·
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
[繁體中文](../zh-Hant/README.md)

## Daftar isi

- [Sekilas fitur](#sekilas-fitur)
- [Persyaratan](#persyaratan)
- [Instalasi](#instalasi)
- [Memulai](#memulai)
  - [Izin](#izin)
  - [Pilih cara mentranskripsi](#pilih-cara-mentranskripsi)
  - [Menu](#menu)
  - [Referensi cepat](#referensi-cepat)
- [Mendikte](#mendikte)
  - [Tahan untuk bicara](#tahan-untuk-bicara)
  - [Kunci dengan Shift atau tekan dua kali](#kunci-dengan-shift-atau-tekan-dua-kali)
  - [Ke mana teks masuk](#ke-mana-teks-masuk)
  - [Dikte terbaru](#dikte-terbaru)
- [Transkripsi](#transkripsi)
  - [Model mana yang tepat untuk Anda](#model-mana-yang-tepat-untuk-anda)
  - [Online: OpenAI dan Groq](#online-openai-dan-groq)
  - [Di Mac Anda: Whisper dan Parakeet](#di-mac-anda-whisper-dan-parakeet)
  - [Saat layanan online gagal](#saat-layanan-online-gagal)
  - [Cara rekaman disiapkan](#cara-rekaman-disiapkan)
  - [Bahasa per model](#bahasa-per-model)
  - [Mac Intel](#mac-intel)
- [Kosakata](#kosakata)
- [Pascapemrosesan](#pascapemrosesan)
- [Mikrofon](#mikrofon)
- [Kunci API](#kunci-api)
- [Notifikasi](#notifikasi)
- [Tampilan dan aksesibilitas](#tampilan-dan-aksesibilitas)
- [Bahasa](#bahasa)
- [Tombol dan gerakan](#tombol-dan-gerakan)
- [Privasi dan keamanan](#privasi-dan-keamanan)
- [Kecepatan](#kecepatan)
- [Keterbatasan yang diketahui](#keterbatasan-yang-diketahui)
- [Pemecahan masalah](#pemecahan-masalah)
- [Untuk pengembang](#untuk-pengembang)
  - [Komponen pihak ketiga](#komponen-pihak-ketiga)
- [Lisensi](#lisensi)

## Sekilas fitur

- **Tahan fn, bicara, lepaskan.** Teks diketik ke app yang sedang berada di depan, di mana pun kursornya: email, chat, editor, terminal, atau browser. Tanpa jendela yang perlu dibuka, tanpa tombol yang perlu diklik.
- **Kunci untuk teks yang lebih panjang.** Tekan Shift sambil menahan fn, atau tekan fn dua kali, dan rekaman terus berjalan tanpa tombol yang ditahan, hingga lima menit. Satu tekanan pada fn atau Shift mengakhirinya.
- **Model yang tepat untuk setiap keperluan.** Groq untuk kecepatan tanpa memakai memori Mac Anda, GPT Transcribe dari OpenAI untuk hasil terbaik, Whisper atau NVIDIA Parakeet di Mac Anda untuk bekerja offline dan menjaga suara Anda tetap privat. Beralih dengan satu klik.
- **Tidak ada dikte yang hilang.** Saat jaringan atau layanan online gagal, model lokal yang terinstal mentranskripsi rekaman yang sama di Mac Anda.
- **Kata-kata Anda, dieja dengan benar.** Kosakata berisi nama dan istilah teknis memandu OpenAI, Groq, Whisper, dan pascapemrosesan.
- **Teks yang rapi jika Anda mau.** Pascapemrosesan opsional menghapus kata pengisi, memperbaiki tata bahasa, dan hanya menyimpan koreksi Anda saat Anda meralat diri sendiri, atau sekaligus memformat teks sesuai jenis app tempat Anda mendikte.
- **Terus saja bicara.** Mulai dikte berikutnya selagi dikte sebelumnya masih ditranskripsi; teks-teksnya tiba sesuai urutan Anda mengucapkannya.
- **Papan klip Anda tetap milik Anda.** Lowspeak menempel melalui papan klip dan mengembalikan isi sebelumnya sesaat kemudian. Sepuluh dikte terakhir tetap ada di menu untuk disalin lagi.
- **Asli sepenuhnya.** Item bar menu sungguhan dengan menu AppKit, peringatan dan notifikasi standar. Tanpa Electron, tanpa jendela, tanpa ikon Dock.
- **Privat.** Tanpa akun, tanpa telemetri. Audio hanya dikirim ke penyedia yang Anda pilih, atau tidak ke mana pun dalam mode lokal.
- **Dalam bahasa Anda.** Dikte berfungsi dalam puluhan bahasa, dan menu, dialog, serta notifikasi tersedia dalam 35 bahasa.

## Persyaratan

- macOS 14 Sonoma atau yang lebih baru. Lowspeak dikembangkan dan diuji di macOS 27.
- Mac dengan Apple Silicon atau prosesor Intel. Transkripsi lokal dibuat untuk Apple Silicon, tempat Whisper berjalan di GPU dan Parakeet di Neural Engine; di Mac Intel keduanya berjalan di prosesor dan jauh lebih lambat, lihat [Mac Intel](#mac-intel).
- Sekitar 17 MB untuk app, ditambah 75 MB hingga 3 GB untuk setiap model lokal yang Anda unduh.
- Untuk transkripsi online, kunci API dari [OpenAI](https://platform.openai.com/api-keys) atau [Groq](https://console.groq.com/keys); keduanya mengenakan biaya untuk apa yang Anda transkripsi. Transkripsi lokal tidak memerlukan akun.

## Instalasi

**Dari image disk.** Unduh [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), buka, lalu seret Lowspeak ke folder Aplikasi. Image ini ditandatangani dengan Developer ID dan dinotarisasi oleh Apple, sehingga terbuka tanpa peringatan.

**Dengan Homebrew.** Cask-nya ada di tap tersendiri:

```sh
brew install --cask sventmueller/tap/lowspeak
```

`brew uninstall --zap --cask lowspeak` juga menghapus pengaturan, model, log, dan cache Lowspeak; kunci API tetap berada di rantai kunci masuk Anda.

**Pembaruan.** Lowspeak dari image disk atau Homebrew memperbarui dirinya sendiri melalui [Sparkle](https://sparkle-project.org). Pada peluncuran kedua, Lowspeak bertanya apakah boleh mencari pembaruan sendiri, sekali sehari. **Periksa Pembaruan…** di menu langsung memeriksa; **Periksa Pembaruan Secara Otomatis** dan **Unduh dan Instal Pembaruan Secara Otomatis** di bawahnya mengubah pilihan Anda nanti. Jika pemeriksaan menemukan versi baru saat Anda bekerja di app lain, sebuah titik muncul pada cincin di bar menu, dan **Periksa Pembaruan…** membawa pembaruan itu ke depan. Lowspeak hanya menginstal pembaruan yang membawa tanda tangan pengembang; pengaturan, kunci, dan model Anda tetap ada. `brew upgrade --cask lowspeak` juga memperbarui salinan dari Homebrew. Versi 1.0.0 tidak punya pemutakhir: instal versi yang lebih baru di atasnya sekali saja, dari [halaman rilis](https://github.com/sventmueller/lowspeak/releases). Build buatan sendiri juga tidak punya pemutakhir.

**Dari sumber.** Lihat [Membangun & memasang](../../../docs/development.md#build--deploy) untuk prasyaratnya, lalu:

```sh
make deploy
```

Perintah ini membangun Lowspeak, mengganti salinan di folder Aplikasi Anda, lalu meluncurkannya.

**Buka saat Masuk** di menu memulai Lowspeak bersama Mac Anda, melalui item masuk milik macOS sendiri. Hanya satu salinan yang berjalan pada satu waktu; salinan kedua langsung keluar.

## Memulai

### Izin

Pada peluncuran pertama, Lowspeak meminta tiga izin:

- **Aksesibilitas** (Pengaturan Sistem › Privasi & Keamanan › Aksesibilitas; di macOS 27 daftar ini bernama Kontrol Perangkat dan Akses Data). Lowspeak memerlukannya untuk mengenali tombol fn dan untuk menempel dengan ⌘V. macOS menawarkan untuk membuka daftarnya; nyalakan sakelar untuk Lowspeak. Pesan dari Lowspeak sendiri menyebut daftar ini sesuai namanya di macOS Anda.
- **Mikrofon.** Diminta sekali; tanpanya Lowspeak tidak merekam apa pun.
- **Notifikasi.** Lowspeak menggunakannya untuk masalah yang tidak akan Anda sadari dengan cara lain, seperti mikrofon yang senyap atau transkripsi yang gagal.

### Pilih cara mentranskripsi

Siapkan setidaknya satu cara; Anda dapat beralih kapan saja di menu.

- **Online:** pilih **Atur Kunci API…** dan tempel kunci dari OpenAI (diawali `sk-`) atau Groq (`gsk_`). Lowspeak mengenali penyedianya dari kunci tersebut. Lalu pilih model di bawah **OpenAI** atau **Groq**.
- **Di Mac Anda:** pilih **Kelola Model** › **Unduh Parakeet Ultra** atau model Whisper, lalu pilih model itu di bawah **Lokal**.

[Model mana yang tepat untuk Anda](#model-mana-yang-tepat-untuk-anda) membantu Anda memilih. Lalu tahan fn, ucapkan sesuatu, dan lepaskan.

### Menu

Klik cincin di bar menu:

| Item | Fungsinya |
|---|---|
| **Dikte Terbaru** | sepuluh teks terakhir; satu klik menyalin teks ke papan klip lagi |
| **Lokal** | model di Mac Anda, dikelompokkan menjadi Whisper dan Parakeet; yang belum diunduh tampil redup |
| **OpenAI**, **Groq** | model online; tanpa kunci, model-model ini tampil redup dan menu menawarkan **Atur Kunci API…** |
| **Bahasa** | bahasa yang Anda pakai untuk mendikte, atau **Deteksi Otomatis**; menu ikut menyesuaikan |
| **Pascapemrosesan** | **Mati**, **Rapikan**, atau **Format**, lihat [Pascapemrosesan](#pascapemrosesan) |
| **Kosakata…** | nama dan istilah yang harus dieja persis; menu menunjukkan berapa banyak yang telah Anda masukkan |
| **Mikrofon** | **Default Sistem** atau mikrofon tetap, dan **Putar Bunyi Saat Siap** |
| **Kelola Model** | mengunduh dan menghapus model lokal, dengan tampilan kemajuan |
| **Atur Kunci API…** / **Perbarui Kunci API…** | menambah, mengganti, atau menghapus kunci |
| **Buka saat Masuk** | memulai Lowspeak bersama Mac Anda; tanda centang menunjukkan bahwa opsi ini aktif |
| **Periksa Pembaruan…** | mencari versi baru sekarang; dua item di bawahnya menentukan apakah Lowspeak memeriksa sekali sehari dan menginstal pembaruan tanpa bertanya. Hanya di app dari image disk |
| **Bantuan** | **Referensi Cepat**, **Laporkan Masalah…**, **Usulkan Fitur…**, **Lowspeak di GitHub** |
| **Mengenai Lowspeak** | versi, hak cipta, halaman proyek, dan lisensi |
| **Tutup Lowspeak** | ⌘Q saat menu terbuka |

Tanda centang di bawah **Lokal**, **OpenAI**, dan **Groq** menandai model yang sedang dipakai. Ikon menunjukkan apa yang sedang dilakukan Lowspeak:

| Ikon | Arti |
|---|---|
| Cincin dengan titik | Siap |
| Lingkaran penuh | Merekam selagi fn ditahan |
| Persegi bersudut bulat | Rekaman terkunci; tekanan berikutnya pada fn atau Shift mengakhirinya |
| Cincin berkedip | Mentranskripsi |
| Cincin dengan titik di kanan atas | Ada versi baru yang menunggu, lihat **Periksa Pembaruan…** |
| ↓ 42% | Mengunduh model |

Lowspeak tidak punya jendela pengaturan: semuanya ada di menu ini, dan Lowspeak mengingat setiap pilihan.

### Referensi cepat

**Bantuan › Referensi Cepat** menampilkan gerakan fn di jendela kecil, dalam bahasa Anda, dengan tombol yang membuka pengaturan papan ketik. **Laporkan Masalah…** membuka formulir di GitHub dengan versi Lowspeak dan macOS Anda sudah terisi, **Usulkan Fitur…** membuka diskusi untuk ide, dan **Lowspeak di GitHub** membuka halaman proyek.

## Mendikte

### Tahan untuk bicara

Tahan fn, bicara, lepaskan. Ikon rekaman muncul begitu mikrofon Anda mengirimkan suara; mulailah bicara saat itu, karena apa yang Anda ucapkan sebelumnya tidak terekam. Melepaskan fn mengakhiri rekaman, dan teksnya menyusul sesaat kemudian.

Ketukan singkat pada fn tidak melakukan apa pun, sehingga fn tetap dapat dipakai untuk fungsi lainnya. Rekaman yang tetap kosong atau berlangsung kurang dari 0,3 detik dibuang.

### Kunci dengan Shift atau tekan dua kali

Untuk teks yang lebih panjang, kunci rekaman agar berjalan tanpa tombol yang ditahan. Ada tiga cara:

- **Shift sambil menahan fn.** Mulai dengan fn seperti biasa dan tekan Shift selama rekaman: rekaman terkunci, dan Anda dapat melepaskan fn. Tidak ada yang Anda ucapkan sebelumnya yang hilang, rekaman cukup berlanjut.
- **Shift, lalu fn.** Tahan Shift dan tekan fn: rekaman dimulai dalam keadaan terkunci.
- **fn dua kali.** Ketuk fn sebentar (kurang dari 0,3 detik) dan tekan lagi dalam 0,4 detik. Rekaman berjalan sejak tekanan pertama.

Ikon berubah menjadi persegi bersudut bulat. Tekan **fn** atau **Shift** sekali untuk berhenti; teksnya menyusul seperti biasa. Jika tombol 🌐 Anda diatur untuk membuka Emoji & Simbol, berhentilah dengan Shift: tekanan singkat pada fn akan membuka penampilnya.

Rekaman berhenti dengan sendirinya setelah dua menit selagi fn ditahan, dan setelah lima menit saat terkunci, lalu ditranskripsi seperti biasa.

### Ke mana teks masuk

Lowspeak menempel teks ke app yang berada di depan saat Anda berhenti bicara, di posisi kursor, melalui papan klip dan ⌘V. Lowspeak menyimpan semua yang ada di papan klip sebelumnya dan mengembalikannya setengah detik setelah app membaca teks, atau setelah sepuluh detik jika app tidak pernah membacanya, kecuali app lain mengubah papan klip di antaranya. Teks ditandai sebagai sementara, sehingga app riwayat papan klip yang menghormati penanda tersebut tidak mencatatnya.

Anda dapat memulai dikte berikutnya selagi dikte sebelumnya masih ditranskripsi. Teks ditempel sesuai urutan Anda mengucapkannya, dan tidak pernah selagi Anda merekam atau menahan Shift, sehingga tempelan tidak bisa mendarat di tengah kalimat Anda berikutnya.

Tanpa izin Aksesibilitas, atau saat ⌘V simulasi tidak dapat dikirim, teks tetap berada di papan klip dan notifikasi memberitahukannya; tempel sendiri dengan ⌘V.

### Dikte terbaru

**Dikte Terbaru** di menu mencantumkan sepuluh teks terakhir, yang terbaru di atas, masing-masing dipersingkat menjadi satu baris; mengarahkan penunjuk ke atasnya menampilkan teks lengkapnya. Satu klik menyalinnya ke papan klip, untuk saat teks mendarat di tempat yang salah atau Anda memerlukannya dua kali. Daftar ini hanya ada di memori dan kosong setelah Lowspeak ditutup.

## Transkripsi

### Model mana yang tepat untuk Anda

| Anda menginginkan | Pilih | Alasannya |
|---|---|---|
| **Kecepatan tanpa membebani Mac Anda** | **Groq › Whisper Large V3 Turbo** | opsi online tercepat; tidak ada yang diunduh, dan memori serta ruang disk Mac Anda tetap utuh |
| **Hasil terbaik** | **OpenAI › GPT Transcribe** (default) | yang paling akurat di antara model yang ditawarkan Lowspeak; menerima kosakata Anda sebagai kata kunci |
| **Offline dan privat** | **Lokal › Parakeet Ultra** untuk bahasa Eropa, **Lokal › Whisper Large v3 Turbo** untuk semua bahasa lainnya | berfungsi tanpa jaringan, dan suara Anda tidak pernah meninggalkan Mac Anda; Parakeet adalah yang tercepat dari semuanya |
| **Tanpa akun, tanpa biaya** | model **Lokal** mana pun | berjalan di Mac Anda; tidak ada yang ditagih |
| **Keandalan sekaligus kecepatan** | model online, ditambah Parakeet Ultra atau model Whisper yang terinstal | model lokal mengambil alih saat layanan online gagal |
| **Mac Intel** | **Groq** atau **OpenAI** | model lokal berjalan di prosesor di sana, lihat [Mac Intel](#mac-intel) |

Model lokal menempati ruang disk sebesar ukuran unduhannya dan, selama dipilih, memori yang kurang lebih sama besarnya; Lowspeak menjaga model tetap dimuat agar dikte dimulai tanpa jeda. Di Mac dengan memori 8 GB, utamakan Parakeet Ultra atau model online daripada model Whisper yang besar.

### Online: OpenAI dan Groq

| Model | Penyedia | Catatan |
|---|---|---|
| **GPT Transcribe** (default) | OpenAI | paling akurat; menerima kosakata Anda sebagai kata kunci |
| **Whisper** | OpenAI | model Whisper yang asli |
| **Whisper Large V3** | Groq | sangat cepat |
| **Whisper Large V3 Turbo** | Groq | tercepat secara online |

Rekaman dikompresi ke AAC sebelum diunggah, sekitar 6 KB per detik ucapan, sehingga dikte juga berfungsi di koneksi yang lambat. Selagi Anda menahan fn, Lowspeak sudah membuka koneksi ke penyedia, sehingga unggahan dimulai begitu Anda melepaskannya.

### Di Mac Anda: Whisper dan Parakeet

| Model | Ukuran | Berjalan di | Bahasa |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 bahasa Eropa |
| **Parakeet v2** | ~600 MB | Neural Engine | Inggris |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | sekitar 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | sekitar 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | sekitar 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | sekitar 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | sekitar 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | sekitar 99 |

**Kelola Model** mengunduh dan menghapusnya; selama pengunduhan, bar menu menunjukkan kemajuannya, dan menutup Lowspeak membatalkannya. Penggunaan pertama sebuah model menyiapkannya untuk Mac Anda: sekitar 10 detik untuk Parakeet Ultra, hingga 30 detik untuk Whisper; setelah itu model dimuat dalam sepersekian detik. Jika model yang Anda pilih dihapus, Lowspeak beralih ke model lain yang terinstal.

**Parakeet** adalah pilihan tercepat: model ini mentranskripsi 40 detik ucapan dalam sekitar 0,3 detik. Lowspeak menjalankannya dengan model dan dekode yang sama seperti [FluidAudio](https://github.com/FluidInference/FluidAudio) dan menyimpan modelnya di folder FluidAudio (`~/Library/Application Support/FluidAudio/Models`), sehingga app yang dibangun di atas FluidAudio dapat memakainya bersama. Rekaman yang lebih panjang dari 15 detik ditranskripsi dalam jendela-jendela yang saling tumpang tindih, lalu digabungkan tanpa kehilangan kata di sambungannya. **Whisper** memahami lebih banyak bahasa dan membaca kosakata Anda; model besarnya adalah yang paling akurat, model kecilnya jauh lebih lemah di luar bahasa Inggris. Model Whisper disimpan di `~/Library/Application Support/Lowspeak/Models`.

### Saat layanan online gagal

Jika transkripsi online gagal, karena jaringan terputus, penyedia sedang bermasalah, atau kuncinya salah, model lokal yang terinstal mentranskripsi rekaman yang sama: model yang dipilih di bawah **Lokal**, jika tidak ada, Parakeet Ultra, lalu Whisper Large v3 Turbo, Large v3, dan model Whisper yang lebih kecil, dengan Parakeet v2 sebagai yang terakhir. Sebuah notifikasi memberi tahu Anda sekali; notifikasi itu baru muncul lagi setelah ada dikte online yang berhasil di antaranya.

Jika ada model lokal sebagai cadangan, Lowspeak memberi permintaan online waktu 20 detik; tanpa cadangan, Lowspeak mencoba tiga kali, masing-masing hingga satu menit. Dalam mode online, Lowspeak menyiapkan model Parakeet cadangan sekali saat peluncuran, sehingga penggantian pertama tidak perlu menunggunya.

### Cara rekaman disiapkan

Lowspeak merekam pada laju sampel mikrofon Anda sendiri dan mengonversinya ke 16 kHz yang diharapkan model begitu Anda berhenti. Sebelum transkripsi, Lowspeak menghapus dengung rendah di bawah 80 Hz, memangkas keheningan di awal dan akhir, dan menyeimbangkan volume, sehingga pembicara yang pelan dan yang keras menghasilkan hasil yang sama. Rekaman tanpa ucapan sama sekali tidak dikirim. Dalam mode lokal, Parakeet pertama-tama menerima rekaman yang belum diolah, dan rekaman yang telah disiapkan hanya jika yang pertama tidak menghasilkan teks.

### Bahasa per model

Bahasa di bawah **Bahasa** diteruskan ke model online dan ke Whisper. Dengan **Deteksi Otomatis**, model menentukan bahasanya sendiri.

| Model | Bahasa yang dapat Anda dikte |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, di Mac Anda) | sekitar 99, termasuk semua 35 bahasa menu |
| Parakeet Ultra | Bulgaria, Kroasia, Ceko, Denmark, Belanda, Inggris, Estonia, Finlandia, Prancis, Jerman, Yunani, Hungaria, Italia, Latvia, Lituania, Malta, Polandia, Portugis, Rumania, Rusia, Slovakia, Slovenia, Spanyol, Swedia, Ukraina |
| Parakeet v2 | Inggris |

Parakeet mendeteksi bahasa sendiri dan mengabaikan pilihan di menu. **Tionghoa (Tradisional)** membuat GPT Transcribe menulis aksara tradisional; Whisper menulis bahasa Tionghoa dalam salah satu dari kedua aksara dan bahasa Portugis tanpa memperhatikan variannya.

### Mac Intel

Lowspeak adalah app universal dan juga berjalan di Mac Intel dengan macOS 14. Semua fungsi online bekerja sama persis. Model lokal di sana tidak mendapatkan perangkat keras yang menjadi tujuan pembuatannya: Whisper berjalan di prosesor alih-alih GPU, dan Parakeet di prosesor alih-alih Neural Engine. Keduanya berfungsi, tetapi memerlukan waktu berkali-kali lipat lebih lama daripada di Apple Silicon, dan model Whisper yang besar terlalu lama untuk dikte. Di Mac Intel, transkripsikan secara online dan simpan Whisper Small atau Parakeet Ultra sebagai [cadangan](#saat-layanan-online-gagal).

## Kosakata

**Kosakata…** menerima nama dan istilah yang harus dieja persis oleh Lowspeak, dipisahkan dengan koma: nama produk, rekan kerja, istilah teknis. Hingga 100 istilah, masing-masing hingga 60 karakter; `<` dan `>` dihapus.

- **GPT Transcribe** menerimanya sebagai kata kunci.
- **Whisper** (OpenAI, Groq, dan di Mac Anda) membacanya sebagai petunjuk sebelum rekaman, hingga 500 karakter.
- **Pascapemrosesan** diminta mengejanya persis seperti yang tertulis.
- **Parakeet** tidak menggunakannya.

## Pascapemrosesan

Setelah transkripsi, model bahasa dapat mengolah ulang teks. Pilih di bawah **Pascapemrosesan**:

- **Mati** (default) — teks apa adanya seperti hasil transkripsi.
- **Rapikan** — menghapus kata pengisi seperti "eh", "anu", atau "intinya", memperbaiki tata bahasa dan tanda baca, dan hanya menyimpan koreksinya saat Anda meralat diri sendiri. Opsi ini tidak menambahkan apa pun dan tidak mengubah makna.
- **Format** — seperti Rapikan, dan menyesuaikan teks dengan app tempat Anda mendikte: istilah bergaya kode di editor kode, baris perintah di terminal, nada santai di app chat, nada profesional di email.

Pascapemrosesan menggunakan Groq (`openai/gpt-oss-120b`) jika Anda memiliki kunci Groq, jika tidak OpenAI (`gpt-4o-mini`), juga saat Anda mentranskripsi secara lokal. Jika sebuah bahasa dipilih di bawah **Bahasa**, pascapemrosesan mempertahankan bahasa tersebut. Jika prosesnya memakan waktu lebih dari delapan detik, gagal, atau menghasilkan jawaban yang terpotong atau jauh lebih panjang daripada yang Anda ucapkan, Lowspeak menempel teks yang belum diolah. Jika penyedia menolak permintaan atau belum ada kunci yang diatur, sebuah notifikasi memberitahukannya sekali.

## Mikrofon

Di bawah **Mikrofon**, pilih mikrofon tetap atau **Default Sistem**. Lowspeak mengingat perangkat tersebut; selama perangkat itu tidak terhubung, menu menampilkannya sebagai **(Tidak Terhubung)** dan Lowspeak merekam dengan default sistem.

Mikrofon hanya dinyalakan selama Anda mendikte. Menyiapkannya memerlukan sekitar 0,07 detik dengan mikrofon internal, sekitar setengah detik dengan Studio Display, dan satu hingga dua detik dengan AirPods, yang terlebih dahulu beralih ke mode headset-nya. Ikon rekaman muncul begitu suara masuk; **Putar Bunyi Saat Siap** menambahkan bunyi singkat pada saat itu. Opsi ini mati secara default, karena pada beberapa speaker bunyinya sendiri datang setengah detik terlambat.

Jika mikrofon hanya mengirimkan keheningan selama lima detik, seperti yang terjadi pada AirPods saat perangkat lain sedang memakai mikrofonnya, Lowspeak memberi tahu Anda.

## Kunci API

**Atur Kunci API…** meminta kunci dan mengenali penyedianya dari awalannya: `sk-` untuk OpenAI, `gsk_` untuk Groq. Kunci dengan format lain ditolak disertai penjelasan. Masukkan satu kunci per penyedia; dengan kunci untuk keduanya, semua model online tersedia. **Perbarui Kunci API…** mengganti kunci atau, dengan **Hapus Semua Kunci**, menghapusnya.

Kunci disimpan di rantai kunci masuk Anda dengan nama `com.lowspeak.app`. File kunci dari versi sebelumnya (`~/Library/Application Support/Lowspeak/.api-key`) dipindahkan ke sana pada peluncuran berikutnya dan dihapus setelah setiap kunci dapat dibaca kembali dari rantai kunci.

## Notifikasi

Lowspeak bekerja di latar belakang, sehingga melaporkan masalah sebagai notifikasi:

| Notifikasi | Apa yang terjadi |
|---|---|
| **Perekaman Gagal** | tidak ada akses mikrofon, tidak ada mikrofon yang terhubung, atau mikrofon tidak dapat dimulai |
| **Mikrofon Senyap** | lima detik keheningan, lihat [Mikrofon](#mikrofon) |
| **Izin Aksesibilitas Diperlukan** | teks ada di papan klip; izinkan Aksesibilitas agar dapat menempel secara otomatis |
| **Penempelan Gagal** | teks ada di papan klip; tempel dengan ⌘V |
| **Transkripsi Gagal** | beserta alasannya: tidak ada kunci, kunci ditolak, batas permintaan, kesalahan server atau jaringan, atau tidak ada model lokal |
| **Ditranskripsi di Mac Ini** | layanan online gagal dan model lokal mengambil alih |
| **Pascapemrosesan Gagal** | penyedia menolak pascapemrosesan atau belum ada kunci yang diatur; teks ditempel tanpa pascapemrosesan |
| **Tidak Ada Kunci API** | Anda memilih model online tanpa kunci untuk penyedianya |
| **Pengunduhan Gagal**, **Penghapusan Gagal**, **Tidak Dapat Menghapus** | pengelolaan model lokal tidak berhasil, atau operasi lain masih berjalan |
| **Kesalahan Item Masuk**, **Kesalahan Kunci API** | macOS atau rantai kunci menolak perubahan |

Notifikasi tidak pernah memuat teks yang Anda dikte. Layanan online yang gagal dan penyedia pascapemrosesan yang menolak dilaporkan sekali, bukan setelah setiap dikte, hingga keduanya berfungsi lagi.

## Tampilan dan aksesibilitas

- **Ikon bar menu** adalah gambar template yang menyesuaikan diri dengan bar menu terang, gelap, atau berwarna.
- **VoiceOver** membacakan ikon sebagai "Lowspeak", dan menu, peringatan, serta notifikasi adalah kontrol macOS standar.
- **Tidak ada tombol yang harus ditahan.** Kunci rekaman dengan Shift atau dengan menekan fn dua kali, dan tekanan berikutnya mengakhirinya.
- **Referensi Cepat** adalah jendela sederhana dengan label dan tombol, yang dapat dijangkau dari papan ketik; Esc menutupnya.

Kekurangan yang diketahui tercantum di [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver tidak mengumumkan saat rekaman dimulai, dan Lowspeak belum diperiksa dengan VoiceOver atau Kontrol Pengalihan.

## Bahasa

Menu, dialog, dan notifikasi diterjemahkan ke 35 bahasa:

Arab, Katalan, Tionghoa (Sederhana), Tionghoa (Tradisional), Kroasia, Ceko, Denmark, Belanda, Inggris, Finlandia, Prancis, Jerman, Yunani, Ibrani, Hindi, Hungaria, Indonesia, Italia, Jepang, Korea, Melayu, Norwegia, Polandia, Portugis (Brasil), Portugis (Portugal), Rumania, Rusia, Slovakia, Slovenia, Spanyol, Swedia, Thai, Turki, Ukraina, dan Vietnam.

Bahasa yang Anda pilih di bawah **Bahasa** juga menjadi bahasa menu. Dengan **Deteksi Otomatis**, menu mengikuti urutan bahasa di **Pengaturan Sistem › Umum › Bahasa & Wilayah**, dan memakai bahasa Inggris jika terjemahannya tidak tersedia. Istilah menu mengikuti terjemahan resmi Apple, sehingga Lowspeak terbaca seperti bagian lain di Mac Anda. Selain bahasa Inggris, terjemahannya dibuat oleh mesin dan belum ditinjau oleh penutur asli; koreksi dipersilakan sebagai [issue](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Tombol dan gerakan

| Tindakan | Tombol |
|---|---|
| Mendikte selama ditahan | tahan fn |
| Mengunci rekaman yang sedang berjalan | Shift sambil menahan fn |
| Memulai rekaman terkunci | Shift+fn, atau fn dua kali dengan cepat |
| Mengakhiri rekaman terkunci | fn atau Shift |
| Menutup Lowspeak (menu terbuka) | ⌘Q |
| Menutup Referensi Cepat | Esc |

Lowspeak mencegah tekanan fn yang dipakainya sampai ke app lain; Shift diteruskan seperti biasa.

## Privasi dan keamanan

Lowspeak mendengar apa yang Anda ucapkan selagi fn ditahan, dapat mengetik ke app mana pun, dan menyimpan kunci API Anda, jadi Lowspeak melakukan sesedikit mungkin dengan semua itu.

- **Suara Anda pergi ke satu tempat saja.** Dalam mode online, rekaman dikirim dalam bentuk terkompresi ke penyedia yang Anda pilih, OpenAI atau Groq, bersama kosakata Anda. Dalam mode lokal, tidak ada yang meninggalkan Mac Anda. Saat permintaan online gagal, model lokal mentranskripsi rekaman yang sama di Mac Anda.
- **Pascapemrosesan**, jika Anda menyalakannya, mengirim teks hasil transkripsi dan kosakata Anda ke Groq atau OpenAI. Dalam mode Format, ikut dikirim jenis app tempat Anda mendikte (editor kode, terminal, chat, atau email); nama app-nya tidak dikirim.
- **Tanpa telemetri, tanpa akun.** Selain untuk transkripsi dan pascapemrosesan, Lowspeak hanya terhubung ke internet untuk membuka koneksi ke penyedia Anda selagi fn ditahan, dalam mode online atau saat pascapemrosesan aktif, untuk mengunduh model saat Anda memintanya, dari Hugging Face melalui HTTPS, pada versi yang tetap dan dengan setiap file dicocokkan dengan ceksumnya, dan untuk mencari versi baru di GitHub, sekali sehari jika Anda mengizinkannya. Permintaan itu memuat versi Lowspeak dan tidak ada yang lain. **Bantuan › Laporkan Masalah…** membuka formulir di GitHub pada browser Anda; alamatnya memuat versi Lowspeak dan versi macOS, tanpa apa pun yang Anda dikte.
- **Kunci API** tetap berada di rantai kunci masuk Anda. Kunci hanya dikirim ke penyedianya masing-masing dan dihapus dari pesan kesalahan sebelum apa pun ditulis ke log.
- **Teks yang didikte tidak disimpan.** Sepuluh dikte terakhir tetap ada di memori untuk **Dikte Terbaru** dan hilang saat Lowspeak ditutup. File log mencatat panjang dan waktu, tidak pernah teksnya.
- **Papan klip** disimpan sebelum menempel dan dipulihkan setelah app membaca teks, kecuali app lain mengubahnya di antaranya. Dikte ditandai sebagai sementara, sehingga app riwayat papan klip yang menghormati penanda tersebut melewatinya.
- **Penandatanganan kode** dengan Hardened Runtime; app hanya meminta akses ke mikrofon, Aksesibilitas, notifikasi, dan jaringan. Pembaruan harus membawa tanda tangan pengembang; jika tidak, Sparkle menolaknya.

Untuk melaporkan kerentanan, ikuti [kebijakan keamanan](../../../SECURITY.md).

## Kecepatan

Diukur di Apple Silicon:

- **Parakeet Ultra** mentranskripsi 40 detik ucapan dalam sekitar 0,3 detik.
- **Online:** permintaan pertama setelah jeda memerlukan sekitar 0,4 detik di Groq, karena Lowspeak membuka koneksi selagi Anda masih berbicara; permintaan berikutnya memakai ulang koneksi itu, melalui HTTP/3 jika penyedia menawarkannya.
- **Ukuran unggahan:** sekitar 6 KB per detik ucapan, sebagai AAC pada 48 kbit/s.
- **Menyiapkan mikrofon:** sekitar 0,07 detik untuk mikrofon internal, setengah detik di Studio Display, satu hingga dua detik dengan AirPods.

## Keterbatasan yang diketahui

- **Model lokal di Mac Intel** berjalan di prosesor dan jauh lebih lambat; di sana transkripsi online adalah pilihan yang lebih baik.
- **Apa yang Anda ucapkan sebelum mikrofon siap akan hilang.** Tunggu ikon rekaman, terutama dengan AirPods.
- **Tombol fn dapat direbut pihak lain.** Saat pengaturan tombol 🌐 di macOS, Warp, atau Dikte macOS mengklaim fn lebih dulu, Lowspeak tidak melihatnya; lihat [Pemecahan masalah](#pemecahan-masalah).
- **Parakeet** mengabaikan kosakata dan pilihan bahasa, dan hanya mengenal bahasa Eropa.
- **Aksara dan varian:** hanya GPT Transcribe yang dapat diminta menulis bahasa Tionghoa Tradisional; tidak ada model yang dapat diminta menulis bahasa Portugis Eropa alih-alih Portugis Brasil.
- **Batas waktu:** dua menit selagi fn ditahan, lima menit saat terkunci.
- **Mode Mik:** Pusat Kontrol menawarkan Isolasi Suara dan Spektrum Lebar untuk Lowspeak dan melaporkannya sebagai aktif, tetapi Lowspeak merekam tanpa pemrosesan suara, sehingga pilihan itu tidak berpengaruh.
- **Dikte Terbaru** hanya ada di memori dan hilang setelah Lowspeak ditutup.
- **Menurut rancangan:** tanpa jendela pengaturan, tanpa ikon Dock, tidak ada di App Store.

## Pemecahan masalah

**Tidak terjadi apa-apa saat saya menahan fn.** Periksa tiga hal: Lowspeak diizinkan di **Pengaturan Sistem › Privasi & Keamanan › Aksesibilitas** (di macOS 27: **Kontrol Perangkat dan Akses Data**); **Pengaturan Sistem › Papan Ketik › Tekan tombol 🌐 untuk** diatur ke **Jangan Lakukan Apa Pun**; dan tidak ada app lain yang mengklaim fn. Warp melakukannya secara default; matikan dengan `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` lalu mulai ulang Warp. Dikte macOS juga dapat diatur agar dimulai dengan fn.

**Setelah pembaruan, Lowspeak tidak lagi bereaksi terhadap fn.** macOS mengaitkan izin Aksesibilitas dengan tanda tangan app. Jika tanda tangannya berubah, misalnya dari build buatan sendiri ke app yang diunduh, hapus Lowspeak dari daftar Aksesibilitas (di macOS 27: Kontrol Perangkat dan Akses Data) lalu tambahkan lagi.

**Kata-kata pertama hilang.** Mulailah bicara begitu ikon rekaman muncul. Dengan AirPods, mikrofon memerlukan satu hingga dua detik; mikrofon internal atau USB yang dipilih di bawah **Mikrofon** siap hampir seketika.

**"Mikrofon Senyap."** Mikrofon tidak mengirimkan suara selama lima detik. Dengan AirPods, perangkat lain mungkin sedang memakai mikrofonnya; beralihlah ke mikrofon internal atau sambungkan ulang AirPods. MacBook yang ditutup juga membisukan mikrofon internal.

**Hasil dikte kosong.** Volume input mungkin terlalu rendah: app seperti Teams, Zoom, atau Meet mengubahnya selama panggilan, dan macOS menyimpannya per perangkat. Naikkan di **Pengaturan Sistem › Bunyi › Input**.

**Teks mendarat di tempat yang salah.** **Dikte Terbaru** di menu menyalinnya ke papan klip lagi.

**"Tidak Ada Kunci API."** Pilih **Atur Kunci API…** di menu, atau beralih ke model lokal.

**Transkripsi lokal pertama memakan waktu lama.** Setiap model disiapkan untuk Mac Anda saat pertama kali digunakan; sejak itu model dimuat dalam sepersekian detik.

**Lowspeak keluar tepat setelah diluncurkan.** Salinan Lowspeak lain sudah berjalan; tutup salinan itu terlebih dahulu.

**Tidak ada notifikasi yang muncul.** Izinkan di **Pengaturan Sistem › Notifikasi › Lowspeak**.

**Masalah lain.** Log di `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) menjelaskan apa yang terjadi, tanpa teks yang Anda dikte. Lampirkan baris yang relevan ke [laporan masalah](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Untuk pengembang

Cara membangun, menguji, struktur kode, dan aturan yang diikutinya dijelaskan di [docs/development.md](../../../docs/development.md) (dalam bahasa Inggris); cara berkontribusi di [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Komponen pihak ketiga

| Komponen | Versi | Dipakai untuk | Lisensi |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) melalui [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | Whisper lokal di GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | dekode Parakeet dan penggabungan rekaman panjang, diterjemahkan ke Rust di `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) dan crate framework-nya | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation, dan framework sistem lainnya | MIT; Zlib, Apache-2.0 atau MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | perekaman mikrofon melalui CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | resampling ke 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) dengan rustls | 3.5 | pengunduhan model | MIT atau Apache-2.0; rustls dan ring di bawah ISC dan Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | ceksum unduhan model | MIT atau Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | kunci API di rantai kunci | MIT atau Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, penanganan kesalahan, kosakata Parakeet | MIT atau Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | pembaruan app dari image disk | MIT, dengan bagian di bawah lisensi BSD dan zlib |

Teks lisensi dari semua yang ada di dalam app disertakan bersamanya sebagai `Contents/Resources/Acknowledgements.txt`.

Model-model ini bukan bagian dari Lowspeak; Lowspeak mengunduhnya saat Anda memintanya:

| Model | Sumber | Lisensi |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper dari OpenAI dalam format ggml) | MIT |
| Parakeet v2, Parakeet Ultra | Konversi CoreML oleh [FluidInference](https://huggingface.co/FluidInference) dari Parakeet TDT milik NVIDIA | CC-BY-4.0 |

## Lisensi

Lowspeak dilisensikan di bawah salah satu dari

- Lisensi Apache, Versi 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Lisensi MIT ([LICENSE-MIT](../../../LICENSE-MIT))

sesuai pilihan Anda.

Kecuali Anda menyatakan lain secara eksplisit, setiap kontribusi yang dengan sengaja diajukan untuk dimasukkan ke dalam Lowspeak oleh Anda, sebagaimana didefinisikan dalam lisensi Apache-2.0, dilisensikan ganda seperti di atas, tanpa syarat atau ketentuan tambahan apa pun.
