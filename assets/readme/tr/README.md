<p><img src="../../readme-icon.png" alt="Lowspeak uygulama simgesi" width="168" height="168"></p>

# Lowspeak

**İmlecin olduğu yere yazan Mac dikte uygulaması: fn tuşunu basılı tutun, konuşun, bırakın.**

Lowspeak menü çubuğunda yaşar. fn (Dünya) tuşunu basılı tutun ve konuşun; tuşu bıraktığınızda sözleriniz, o anda hangi uygulamada yazıyorsanız orada, sanki kendiniz yazmışsınız gibi belirir. Metne dönüştürmeyi çevrimiçi olarak OpenAI ya da Groq ile veya tamamen Mac’inizde Whisper ya da NVIDIA Parakeet ile yapar; Parakeet 40 saniyelik konuşmayı yaklaşık üçte bir saniyede metne çevirir. Ağ ya da çevrimiçi hizmet başarısız olduğunda yüklü bir yerel model devreye girer, böylece hiçbir dikte kaybolmaz. Kullandığınız adları ve terimleri ona öğretin, isterseniz dolgu sözcüklerini bir dil modeline temizletin ve daha uzun metinleri eller serbest dikte etmek için kaydı Shift ile kilitleyin. Rust ile gerçek AppKit denetimleri üzerine yazılmıştır, 35 dilde kullanılabilir ve sesinizi yalnızca seçtiğiniz sağlayıcıya gönderir.

[![Lowspeak’i macOS için indirin](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Lowspeak.dmg dosyasını indirin](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: Apple tarafından imzalanmış ve notarize edilmiştir; Apple Silicon ve Intel üzerinde macOS 14 veya üstü için.

**Bir hata mı buldunuz?** [Bir issue açın](https://github.com/sventmueller/lowspeak/issues/new/choose) ya da Lowspeak’in menüsünde **Yardım › Sorun Bildir…** seçeneğini seçin: form, sürümünüz doldurulmuş olarak açılır. Lowspeak işinize yarıyorsa [GitHub’da bir yıldız](https://github.com/sventmueller/lowspeak) başkalarının da onu bulmasına yardımcı olur.

[![Rust 2024 sürümü](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 veya üstü](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon ve Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![objc2 üzerinden AppKit](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![Metal üzerinde whisper.cpp](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Neural Engine üzerinde Parakeet](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![OpenAI veya Groq ile çevrimiçi](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 arayüz dili](https://img.shields.io/badge/languages-35-4C9A2A)
![Hardened Runtime ile imzalı](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Lisans: MIT veya Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#lisans)

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
**Türkçe** ·
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

## İçindekiler

- [Bir bakışta özellikler](#bir-bakışta-özellikler)
- [Gereksinimler](#gereksinimler)
- [Kurulum](#kurulum)
- [Başlarken](#başlarken)
  - [Gerekli izinler](#gerekli-izinler)
  - [Metne dönüştürme yolunu seçin](#metne-dönüştürme-yolunu-seçin)
  - [Menü](#menü)
  - [Hızlı başvuru](#hızlı-başvuru)
- [Dikte etme](#dikte-etme)
  - [Basılı tutun ve konuşun](#basılı-tutun-ve-konuşun)
  - [Shift ya da çift basışla kilitleme](#shift-ya-da-çift-basışla-kilitleme)
  - [Metin nereye gider](#metin-nereye-gider)
  - [Son dikteler](#son-dikteler)
- [Metne dönüştürme](#metne-dönüştürme)
  - [Size uygun model hangisi](#size-uygun-model-hangisi)
  - [Çevrimiçi: OpenAI ve Groq](#çevrimiçi-openai-ve-groq)
  - [Mac’inizde: Whisper ve Parakeet](#macinizde-whisper-ve-parakeet)
  - [Çevrimiçi hizmet başarısız olduğunda](#çevrimiçi-hizmet-başarısız-olduğunda)
  - [Kayıt nasıl hazırlanır](#kayıt-nasıl-hazırlanır)
  - [Modele göre diller](#modele-göre-diller)
  - [Intel Mac’ler](#intel-macler)
- [Sözlük](#sözlük)
- [Son işleme](#son-işleme)
- [Mikrofon](#mikrofon)
- [API anahtarları](#api-anahtarları)
- [Bildirimler](#bildirimler)
- [Görünüm ve erişilebilirlik](#görünüm-ve-erişilebilirlik)
- [Diller](#diller)
- [Tuşlar ve hareketler](#tuşlar-ve-hareketler)
- [Gizlilik ve güvenlik](#gizlilik-ve-güvenlik)
- [Hız](#hız)
- [Bilinen sınırlamalar](#bilinen-sınırlamalar)
- [Sorun giderme](#sorun-giderme)
- [Geliştiriciler için](#geliştiriciler-için)
  - [Üçüncü taraf bileşenler](#üçüncü-taraf-bileşenler)
- [Lisans](#lisans)

## Bir bakışta özellikler

- **fn tuşunu basılı tutun, konuşun, bırakın.** Metin, öndeki uygulamaya, imleç neredeyse oraya yazılır: e-posta, sohbet, düzenleyici, terminal ya da tarayıcı. Geçiş yapılacak bir pencere, tıklanacak bir düğme yok.
- **Uzun metinler için kilitleyin.** fn tuşunu basılı tutarken Shift tuşuna basın ya da fn tuşuna iki kez basın; kayıt, hiçbir tuş basılı tutulmadan beş dakikaya kadar sürer. fn ya da Shift tuşuna bir kez basmak kaydı bitirir.
- **İşe uygun model.** Mac’inizin belleğini kullanmadan hız için Groq, en iyi sonuç için OpenAI’ın GPT Transcribe modeli, çevrimdışı çalışmak ve sesinizi kendinize saklamak için Mac’inizde Whisper ya da NVIDIA Parakeet. Tek tıkla geçiş yapın.
- **Hiçbir dikte kaybolmaz.** Ağ ya da çevrimiçi hizmet başarısız olduğunda yüklü bir yerel model aynı kaydı Mac’inizde metne dönüştürür.
- **Sözcükleriniz, doğru yazılmış.** Adlardan ve teknik terimlerden oluşan bir sözlük OpenAI, Groq, Whisper ve son işleme için yol gösterir.
- **İsterseniz temiz metin.** İsteğe bağlı son işleme dolgu sözcüklerini kaldırır, dil bilgisini düzeltir ve kendinizi düzelttiğinizde yalnızca düzeltmenizi tutar ya da metni ayrıca dikte ettiğiniz uygulamanın türüne göre biçimlendirir.
- **Konuşmaya devam edin.** Önceki dikte hâlâ metne dönüştürülürken bir sonrakini başlatın; metinler söylediğiniz sırayla gelir.
- **Panonuz sizin kalır.** Lowspeak pano üzerinden yapıştırır ve panoda önceden ne varsa bir an sonra geri koyar. Son on dikte yeniden kopyalamak için menüde kalır.
- **Tepeden tırnağa yerel.** AppKit menüsüyle gerçek bir menü çubuğu öğesi, standart uyarılar ve bildirimler. Electron yok, pencere yok, Dock simgesi yok.
- **Gizliliğe saygılı.** Hesap yok, telemetri yok. Ses yalnızca seçtiğiniz sağlayıcıya gider, yerel modda ise hiçbir yere gitmez.
- **Kendi dilinizde.** Dikte onlarca dilde çalışır; menü, iletişim kutuları ve bildirimler 35 dilde gelir.

## Gereksinimler

- macOS 14 Sonoma veya üstü. Lowspeak macOS 27’de geliştirilir ve test edilir.
- Apple Silicon veya Intel işlemcili bir Mac. Yerel metne dönüştürme Apple Silicon için tasarlanmıştır; orada Whisper GPU’da, Parakeet ise Neural Engine’de çalışır. Intel Mac’lerde ikisi de işlemcide ve çok daha yavaş çalışır; bkz. [Intel Mac’ler](#intel-macler).
- Uygulama için yaklaşık 10 MB, ayrıca indirdiğiniz her yerel model için 75 MB ile 3 GB arası.
- Çevrimiçi metne dönüştürme için [OpenAI](https://platform.openai.com/api-keys) ya da [Groq](https://console.groq.com/keys) API anahtarı; ikisi de metne dönüştürdüğünüz kadar ücret alır. Yerel metne dönüştürme için hesap gerekmez.

## Kurulum

**Disk görüntüsünden.** [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg) dosyasını indirin, açın ve Lowspeak’i Uygulamalar klasörüne sürükleyin. Görüntü bir Developer ID ile imzalanmış ve Apple tarafından notarize edilmiştir; bu yüzden uyarısız açılır.

**Güncellemeler.** Lowspeak kendini güncellemez. Yeni sürümler [sürümler sayfasında](https://github.com/sventmueller/lowspeak/releases) yayımlanır; **Yardım › GitHub’da Lowspeak** oraya götürür. Yeni sürümü eskisinin üzerine kurun; ayarlarınız, anahtarlarınız ve modelleriniz korunur.

**Kaynaktan.** Önkoşullar için [Derleme ve dağıtım](../../../docs/development.md#build--deploy) bölümüne bakın, ardından:

```sh
make deploy
```

Bu komut Lowspeak’i derler, Uygulamalar klasörünüzdeki kopyayı değiştirir ve başlatır.

Menüdeki **Oturum Açıldığında Aç**, Lowspeak’i macOS’in kendi oturum açma öğeleri aracılığıyla Mac’inizle birlikte başlatır. Aynı anda yalnızca bir kopya çalışır; ikincisi hemen kapanır.

## Başlarken

### Gerekli izinler

Lowspeak ilk açılışta üç izin ister:

- **Erişilebilirlik** (Sistem Ayarları › Gizlilik ve Güvenlik › Erişilebilirlik; macOS 27’de bu liste Aygıt Denetimi ve Veri Erişimi olarak adlandırılır). Lowspeak’in fn tuşunu fark etmesi ve ⌘V ile yapıştırması için gereklidir. macOS listeyi açmayı önerir; Lowspeak’in anahtarını açın. Lowspeak’in kendi iletileri listeyi macOS’inizin gösterdiği adla anar.
- **Mikrofon.** Bir kez sorulur; bu izin olmadan Lowspeak hiçbir şey kaydetmez.
- **Bildirimler.** Lowspeak bunları, aksi hâlde fark etmeyeceğiniz sorunlar için kullanır; örneğin sessiz bir mikrofon ya da başarısız bir metne dönüştürme.

### Metne dönüştürme yolunu seçin

En az bir yol ayarlayın; menüden istediğiniz zaman geçiş yapabilirsiniz.

- **Çevrimiçi:** **API anahtarını ayarla…** seçeneğini seçin ve OpenAI’dan (`sk-` ile başlar) ya da Groq’tan (`gsk_`) aldığınız bir anahtarı yapıştırın. Lowspeak sağlayıcıyı anahtardan tanır. Ardından **OpenAI** ya da **Groq** altından bir model seçin.
- **Mac’inizde:** **Modelleri yönet** › **Parakeet Ultra modelini indir** ya da bir Whisper modeli seçin, ardından onu **Yerel** altından seçin.

Seçimde [Size uygun model hangisi](#size-uygun-model-hangisi) bölümü yardımcı olur. Ardından fn tuşunu basılı tutun, bir şey söyleyin ve bırakın.

### Menü

Menü çubuğundaki halkaya tıklayın:

| Öğe | Ne yapar |
|---|---|
| **Son dikteler** | son on metin; bir tıklama birini yeniden panoya kopyalar |
| **Yerel** | Mac’inizdeki modeller, Whisper ve Parakeet olarak gruplanmış; indirilmemiş olanlar soluk görünür |
| **OpenAI**, **Groq** | çevrimiçi modeller; anahtar yoksa soluk görünür ve menü **API anahtarını ayarla…** seçeneğini sunar |
| **Dil** | dikte ettiğiniz dil ya da **Otomatik algıla**; menü de bunu izler |
| **Son işleme** | **Kapalı**, **Temizleme** ya da **Biçimlendirme**; bkz. [Son işleme](#son-işleme) |
| **Sözlük…** | tam olarak yazılması gereken adlar ve terimler; menü kaç tane girdiğinizi gösterir |
| **Mikrofon** | **Sistem varsayılanı** ya da sabit bir mikrofon ve **Hazır olduğunda ses çal** |
| **Modelleri yönet** | yerel modelleri ilerleme durumuyla birlikte indirme ve silme |
| **API anahtarını ayarla…** / **API anahtarını güncelle…** | anahtar ekleme, değiştirme ya da kaldırma |
| **Oturum Açıldığında Aç** | Lowspeak’i Mac’inizle birlikte başlatır; onay işareti açık olduğunu gösterir |
| **Yardım** | **Hızlı Başvuru**, **Sorun Bildir…**, **Özellik Öner…**, **GitHub’da Lowspeak** |
| **Lowspeak Hakkında** | sürüm, telif hakkı, proje sayfası ve lisans |
| **Lowspeak’ten Çık** | menü açıkken ⌘Q |

**Yerel**, **OpenAI** ve **Groq** altındaki onay işareti kullanılan modeli gösterir. Simge, Lowspeak’in ne yaptığını gösterir:

| Simge | Anlamı |
|---|---|
| Noktalı halka | Hazır |
| Dolu daire | fn basılıyken kayıt |
| Yuvarlatılmış kare | Kayıt kilitli; fn ya da Shift tuşuna bir sonraki basış kaydı bitirir |
| Yanıp sönen halka | Metne dönüştürülüyor |
| ↓ 42% | Model indiriliyor |

Lowspeak’in ayarlar penceresi yoktur: her şey bu menüdedir ve Lowspeak her seçiminizi hatırlar.

### Hızlı başvuru

**Yardım › Hızlı Başvuru**, fn hareketlerini küçük bir pencerede, sizin dilinizde, klavye ayarlarını açan bir düğmeyle birlikte gösterir. **Sorun Bildir…** GitHub’da Lowspeak ve macOS sürümleriniz doldurulmuş bir form açar, **Özellik Öner…** fikirler için bir tartışma, **GitHub’da Lowspeak** ise proje sayfasını açar.

## Dikte etme

### Basılı tutun ve konuşun

fn tuşunu basılı tutun, konuşun, bırakın. Kayıt simgesi, mikrofonunuz ses verdiği anda belirir; konuşmaya o zaman başlayın, çünkü öncesinde söyledikleriniz kaydedilmez. fn tuşunu bırakmak kaydı bitirir ve metin bir an sonra gelir.

fn tuşuna kısa bir dokunuş hiçbir şey yapmaz, böylece fn diğer işlevlerini korur. Boş kalan ya da 0,3 saniyeden kısa süren bir kayıt atılır.

### Shift ya da çift basışla kilitleme

Daha uzun metinler için kaydı kilitleyin; böylece hiçbir tuş basılı tutulmadan sürer. Bunun üç yolu vardır:

- **fn basılıyken Shift.** Her zamanki gibi fn ile başlayın ve kayıt sırasında Shift tuşuna basın: kayıt kilitlenir ve fn tuşunu bırakabilirsiniz. Önceden söylediğiniz hiçbir şey kaybolmaz, kayıt yalnızca devam eder.
- **Önce Shift, sonra fn.** Shift tuşunu basılı tutun ve fn tuşuna basın: kayıt kilitli olarak başlar.
- **İki kez fn.** fn tuşuna kısaca dokunun (0,3 saniyeden kısa) ve 0,4 saniye içinde yeniden basın. Kayıt ilk basıştan itibaren sürer.

Simge yuvarlatılmış bir kareye dönüşür. Durdurmak için **fn** ya da **Shift** tuşuna bir kez basın; metin her zamanki gibi gelir. 🌐 tuşunuz Emoji ve Semboller’i açacak şekilde ayarlıysa Shift ile durdurun: fn tuşuna kısa bir basış görüntüleyiciyi açar.

Kayıt, fn basılı tutulurken iki dakika, kilitliyken beş dakika sonra kendiliğinden durur ve ardından her zamanki gibi metne dönüştürülür.

### Metin nereye gider

Lowspeak metni, konuşmayı bitirdiğiniz anda önde olan uygulamaya, imlecin olduğu yere, pano ve ⌘V aracılığıyla yapıştırır. Panoda önceden ne varsa kaydeder ve uygulama metni okuduktan yarım saniye sonra, hiç okumazsa on saniye sonra geri koyar; bu arada başka bir uygulama panoyu değiştirdiyse geri koymaz. Metin geçici olarak işaretlenir; bu işarete uyan pano geçmişi uygulamaları onu kaydetmez.

Önceki dikte hâlâ metne dönüştürülürken bir sonrakini başlatabilirsiniz. Metinler söylediğiniz sırayla yapıştırılır ve kayıt yaparken ya da Shift tuşunu basılı tutarken asla yapıştırılmaz; böylece bir yapıştırma bir sonraki cümlenizin ortasına düşemez.

Erişilebilirlik izni olmadan ya da benzetilmiş ⌘V gönderilemediğinde metin panoda kalır ve bir bildirim bunu söyler; metni ⌘V ile kendiniz yapıştırın.

### Son dikteler

Menüdeki **Son dikteler**, son on metni en yenisi başta olmak üzere, her birini tek satıra kısaltarak listeler; imleçle üzerine gelindiğinde metnin tamamı görünür. Bir tıklama metni panoya kopyalar; bir metin yanlış yere düştüğünde ya da ona iki kez ihtiyaç duyduğunuzda işe yarar. Liste yalnızca bellekte tutulur ve Lowspeak kapandıktan sonra boştur.

## Metne dönüştürme

### Size uygun model hangisi

| İstediğiniz | Seçin | Neden |
|---|---|---|
| **Mac’inizi kullanmadan hız** | **Groq › Whisper Large V3 Turbo** | en hızlı çevrimiçi seçenek; hiçbir şey indirilmez, Mac’inizin belleği ve disk alanı size kalır |
| **En iyi sonuç** | **OpenAI › GPT Transcribe** (varsayılan) | Lowspeak’in sunduğu modellerin en isabetlisi; sözlüğünüzü anahtar sözcük olarak alır |
| **Çevrimdışı ve gizli** | Avrupa dilleri için **Yerel › Parakeet Ultra**, diğer tüm diller için **Yerel › Whisper Large v3 Turbo** | ağ olmadan çalışır ve sesiniz Mac’inizden asla çıkmaz; Parakeet hepsinin en hızlısıdır |
| **Hesap yok, maliyet yok** | herhangi bir **Yerel** model | Mac’inizde çalışır; hiçbir şey faturalandırılmaz |
| **Hem güvenilirlik hem hız** | bir çevrimiçi model, yanında yüklü Parakeet Ultra ya da bir Whisper modeli | çevrimiçi hizmet başarısız olduğunda yerel model devreye girer |
| **Bir Intel Mac** | **Groq** ya da **OpenAI** | yerel modeller orada işlemcide çalışır; bkz. [Intel Mac’ler](#intel-macler) |

Yerel modeller diskte indirme boyutları kadar yer kaplar ve seçili oldukları sürece yaklaşık o kadar bellek kullanır; Lowspeak modeli yüklü tutar, böylece dikte gecikmeden başlar. 8 GB belleği olan bir Mac’te büyük Whisper modelleri yerine Parakeet Ultra’yı ya da bir çevrimiçi modeli tercih edin.

### Çevrimiçi: OpenAI ve Groq

| Model | Sağlayıcı | Notlar |
|---|---|---|
| **GPT Transcribe** (varsayılan) | OpenAI | en isabetlisi; sözlüğünüzü anahtar sözcük olarak alır |
| **Whisper** | OpenAI | özgün Whisper modeli |
| **Whisper Large V3** | Groq | çok hızlı |
| **Whisper Large V3 Turbo** | Groq | en hızlı çevrimiçi model |

Kayıt, yüklemeden önce AAC biçiminde sıkıştırılır, konuşmanın saniyesi başına yaklaşık 6 KB; böylece dikte yavaş bağlantılarda da çalışır. Siz fn tuşunu basılı tutarken Lowspeak sağlayıcıyla bağlantıyı şimdiden açar, böylece yükleme tuşu bıraktığınız anda başlar.

### Mac’inizde: Whisper ve Parakeet

| Model | Boyut | Çalıştığı yer | Diller |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 Avrupa dili |
| **Parakeet v2** | ~600 MB | Neural Engine | İngilizce |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | yaklaşık 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | yaklaşık 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | yaklaşık 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | yaklaşık 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | yaklaşık 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | yaklaşık 99 |

**Modelleri yönet** modelleri indirir ve siler; indirme sırasında menü çubuğu ilerlemeyi gösterir, Lowspeak’ten çıkmak ise indirmeyi iptal eder. Bir modelin ilk kullanımı onu Mac’iniz için hazırlar: Parakeet Ultra için yaklaşık 10 saniye, Whisper için 30 saniyeye kadar; sonrasında model saniyenin çok küçük bir bölümünde yüklenir. Seçtiğiniz model silinirse Lowspeak yüklü başka bir modele geçer.

**Parakeet** en hızlı seçenektir: 40 saniyelik konuşmayı yaklaşık 0,3 saniyede metne dönüştürür. Lowspeak onu [FluidAudio](https://github.com/FluidInference/FluidAudio) ile aynı modeller ve aynı kod çözme yöntemiyle çalıştırır ve modelleri FluidAudio’nun klasöründe (`~/Library/Application Support/FluidAudio/Models`) saklar; böylece FluidAudio tabanlı uygulamalar onları paylaşır. 15 saniyeden uzun kayıtlar birbiriyle örtüşen pencerelerde metne dönüştürülür ve bu pencereler, birleşim yerlerinde hiçbir sözcük kaybolmadan birleştirilir. **Whisper** daha çok dil anlar ve sözlüğünüzü okur; büyük modeller en isabetli olanlardır, küçükler İngilizce dışında çok daha zayıftır. Whisper modelleri `~/Library/Application Support/Lowspeak/Models` içinde saklanır.

### Çevrimiçi hizmet başarısız olduğunda

Bir çevrimiçi metne dönüştürme ağ kesik olduğu, sağlayıcıda bir sorun olduğu ya da anahtar yanlış olduğu için başarısız olursa yüklü bir yerel model aynı kaydı metne dönüştürür: **Yerel** altında seçili olan, o yoksa Parakeet Ultra, ardından Whisper Large v3 Turbo, Large v3 ve daha küçük Whisper modelleri, en son da Parakeet v2. Bir bildirim bunu size bir kez söyler; bildirim ancak arada bir çevrimiçi dikte başarılı olduktan sonra yeniden gelir.

Yedek olarak bir yerel model varsa Lowspeak çevrimiçi isteğe 20 saniye tanır; yoksa her biri en fazla bir dakika süren üç deneme yapar. Çevrimiçi modda Lowspeak yedek Parakeet modelini açılışta bir kez hazırlar, böylece ilk yedek dönüştürme onu beklemez.

### Kayıt nasıl hazırlanır

Lowspeak, mikrofonunuzun kendi örnekleme hızında kaydeder ve siz durdurduğunuzda kaydı modellerin beklediği 16 kHz’e dönüştürür. Metne dönüştürmeden önce 80 Hz’in altındaki düşük uğultuyu temizler, baştaki ve sondaki sessizliği kırpar ve ses düzeyini dengeler; böylece alçak ve yüksek sesle konuşanlar aynı şekilde çıkar. Konuşma içermeyen bir kayıt hiç gönderilmez. Yerel modda Parakeet önce işlenmemiş kaydı alır, hazırlanmış olanı ise yalnızca ilki metin vermezse.

### Modele göre diller

**Dil** altında seçilen dil çevrimiçi modellere ve Whisper’a iletilir. **Otomatik algıla** ile model dili kendisi belirler.

| Model | Dikte edebileceğiniz diller |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, Mac’inizde) | yaklaşık 99; menünün 35 dilinin tümü dahil |
| Parakeet Ultra | Almanca, Bulgarca, Çekçe, Danca, Estonca, Felemenkçe, Fince, Fransızca, Hırvatça, İngilizce, İspanyolca, İsveççe, İtalyanca, Lehçe, Letonca, Litvanca, Macarca, Maltaca, Portekizce, Romence, Rusça, Slovakça, Slovence, Ukraynaca, Yunanca |
| Parakeet v2 | İngilizce |

Parakeet dili kendisi algılar ve menüdeki seçimi yok sayar. **Çince (Geleneksel)**, GPT Transcribe’a geleneksel karakterlerle yazmasını söyler; Whisper Çinceyi her iki yazı sisteminde, Portekizceyi de değişkesine bakmadan yazar.

### Intel Mac’ler

Lowspeak evrensel bir uygulamadır ve macOS 14 yüklü Intel Mac’lerde de çalışır. Çevrimiçi her şey aynı şekilde çalışır. Ancak yerel modeller orada kendileri için tasarlandıkları donanımı bulamaz: Whisper GPU yerine işlemcide, Parakeet de Neural Engine yerine işlemcide çalışır. İkisi de çalışır ama Apple Silicon’dakinden kat kat uzun sürer; büyük Whisper modelleri dikte için fazlasıyla yavaştır. Bir Intel Mac’te çevrimiçi olarak metne dönüştürün ve [yedek](#çevrimiçi-hizmet-başarısız-olduğunda) olarak Whisper Small ya da Parakeet Ultra’yı tutun.

## Sözlük

**Sözlük…**, Lowspeak’in tam olarak yazması gereken adları ve terimleri virgülle ayrılmış olarak alır: ürün adları, iş arkadaşları, teknik terimler. En fazla 100 terim, her biri en fazla 60 karakter; `<` ve `>` kaldırılır.

- **GPT Transcribe** bunları anahtar sözcük olarak alır.
- **Whisper** (OpenAI, Groq ve Mac’inizde) bunları kayıttan önce bir ipucu olarak, 500 karaktere kadar okur.
- **Son işlemeye** bunları tam yazıldıkları gibi yazması söylenir.
- **Parakeet** bunları kullanmaz.

## Son işleme

Metne dönüştürmeden sonra bir dil modeli metni yeniden işleyebilir. **Son işleme** altından seçin:

- **Kapalı** (varsayılan) — metin, dönüştürüldüğü gibi.
- **Temizleme** — “ııı”, “yani” ya da “aslında” gibi dolgu sözcüklerini kaldırır, dil bilgisini ve noktalamayı düzeltir ve kendinizi düzelttiğinizde yalnızca düzeltmeyi tutar. Hiçbir şey eklemez ve anlamı değiştirmez.
- **Biçimlendirme** — Temizleme gibi; ayrıca metni dikte ettiğiniz uygulamaya uyarlar: kod düzenleyicilerinde koda uygun terimler, terminallerde komut satırları, sohbet uygulamalarında samimi bir üslup, e-postada profesyonel bir üslup.

Son işleme, Groq anahtarınız varsa Groq’u (`openai/gpt-oss-120b`), yoksa OpenAI’ı (`gpt-4o-mini`) kullanır; yerel olarak metne dönüştürdüğünüzde de. **Dil** altında bir dil seçtiyseniz o dili korur. Sekiz saniyeden uzun sürerse, başarısız olursa ya da kesik veya söylediğinizden çok daha uzun bir yanıt döndürürse Lowspeak işlenmemiş metni yapıştırır. Sağlayıcı isteği reddederse ya da anahtar ayarlanmamışsa bir bildirim bunu bir kez söyler.

## Mikrofon

**Mikrofon** altından sabit bir mikrofon ya da **Sistem varsayılanı** seçin. Lowspeak aygıtı hatırlar; aygıt bağlı değilken menü onu **(bağlı değil)** olarak gösterir ve Lowspeak sistem varsayılanıyla kaydeder.

Mikrofon yalnızca dikte ederken açılır. Hazır hâle gelmesi dahili mikrofonda yaklaşık 0,07 saniye, Studio Display’de yaklaşık yarım saniye, önce kulaklık moduna geçen AirPods’ta ise bir ila iki saniye sürer. Kayıt simgesi ses geldiği anda belirir; **Hazır olduğunda ses çal** o anda kısa bir ses ekler. Bu seçenek varsayılan olarak kapalıdır, çünkü bazı hoparlörlerde sesin kendisi yarım saniye geç gelir.

Bir mikrofon beş saniye boyunca yalnızca sessizlik verirse (başka bir aygıt AirPods’un mikrofonunu tuttuğunda olduğu gibi) Lowspeak sizi uyarır.

## API anahtarları

**API anahtarını ayarla…** bir anahtar ister ve sağlayıcıyı anahtarın başından tanır: OpenAI için `sk-`, Groq için `gsk_`. Başka biçimdeki bir anahtar bir açıklamayla reddedilir. Her sağlayıcı için bir anahtar girin; ikisinin de anahtarı varsa tüm çevrimiçi modeller kullanılabilir. **API anahtarını güncelle…** bir anahtarı değiştirir ya da **Tüm anahtarları kaldır** ile anahtarları siler.

Anahtarlar, oturum açma anahtar zincirinizde `com.lowspeak.app` adıyla saklanır. Önceki bir sürümün anahtar dosyası (`~/Library/Application Support/Lowspeak/.api-key`) bir sonraki açılışta oraya taşınır ve anahtarların tümü anahtar zincirinden geri okunabildiğinde silinir.

## Bildirimler

Lowspeak arka planda çalıştığı için sorunları bildirim olarak iletir:

| Bildirim | Ne oldu |
|---|---|
| **Kayıt başarısız oldu** | mikrofon erişimi yok, bağlı mikrofon yok ya da mikrofon başlatılamadı |
| **Mikrofon sessiz** | beş saniyelik sessizlik; bkz. [Mikrofon](#mikrofon) |
| **Erişilebilirlik izni gerekli** | metin panoda; otomatik yapıştırma için Erişilebilirlik izni verin |
| **Yapıştırma başarısız** | metin panoda; ⌘V ile yapıştırın |
| **Metne dönüştürme başarısız oldu** | nedeniyle birlikte: anahtar yok, anahtar reddedildi, istek sınırı, sunucu ya da ağ hatası veya yerel model yok |
| **Bu Mac'te metne dönüştürüldü** | çevrimiçi hizmet başarısız oldu ve bir yerel model devraldı |
| **Son işleme başarısız oldu** | sağlayıcı son işlemeyi reddediyor ya da anahtar ayarlanmamış; metin son işleme olmadan yapıştırıldı |
| **API anahtarı yok** | sağlayıcısı için anahtar olmayan bir çevrimiçi model seçtiniz |
| **İndirme başarısız oldu**, **Silme başarısız oldu**, **Silinemiyor** | bir yerel modelin yönetimi başarısız oldu ya da başka bir işlem hâlâ sürüyordu |
| **Oturum açma öğesi hatası**, **API anahtarı hatası** | macOS ya da anahtar zinciri değişikliği reddetti |

Bildirimler dikte ettiğiniz metni asla içermez. Başarısız olan bir çevrimiçi hizmet ve son işlemeyi reddeden bir sağlayıcı, her dikteden sonra değil, yeniden çalışana kadar yalnızca bir kez bildirilir.

## Görünüm ve erişilebilirlik

- **Menü çubuğu simgesi**, açık, koyu ya da renklendirilmiş menü çubuğuna uyum sağlayan bir şablon görüntüdür.
- **VoiceOver** simgeyi “Lowspeak” olarak okur; menü, uyarılar ve bildirimler standart macOS denetimleridir.
- **Hiçbir tuşu basılı tutmak gerekmez.** Bir kaydı Shift ile ya da fn tuşuna iki kez basarak kilitleyin; bir sonraki basış kaydı bitirir.
- **Hızlı Başvuru**, etiketler ve bir düğme içeren, klavyeden erişilebilen sade bir penceredir; Esc onu kapatır.

Bilinen eksikler [ACCESSIBILITY.md](../../../ACCESSIBILITY.md) dosyasında listelenmiştir: VoiceOver bir kaydın başladığını duyurmaz ve Lowspeak henüz VoiceOver ya da Anahtarla Denetim ile denetlenmemiştir.

## Diller

Menü, iletişim kutuları ve bildirimler 35 dile çevrilmiştir:

Almanca, Arapça, Çekçe, Çince (Basitleştirilmiş), Çince (Geleneksel), Danca, Endonezce, Felemenkçe, Fince, Fransızca, Hırvatça, Hintçe, İbranice, İngilizce, İspanyolca, İsveççe, İtalyanca, Japonca, Katalanca, Korece, Lehçe, Macarca, Malayca, Norveççe, Portekizce (Brezilya), Portekizce (Portekiz), Romence, Rusça, Slovakça, Slovence, Tayca, Türkçe, Ukraynaca, Vietnamca ve Yunanca.

**Dil** altında seçtiğiniz dil aynı zamanda menünün dilidir. **Otomatik algıla** ile menü, **Sistem Ayarları › Genel › Dil ve Bölge** içindeki dil sırasını izler; çeviri olmayan yerde İngilizceyi kullanır. Menü terimleri Apple’ın kendi çevirilerini izler, böylece Lowspeak Mac’inizin geri kalanı gibi okunur. İngilizce dışındaki çeviriler makine çevirisidir ve henüz ana dili konuşan kişilerce gözden geçirilmemiştir; düzeltmeler [issue](https://github.com/sventmueller/lowspeak/issues/new/choose) olarak memnuniyetle karşılanır.

## Tuşlar ve hareketler

| Eylem | Tuşlar |
|---|---|
| Basılı tutarken dikte et | fn tuşunu basılı tutun |
| Süren bir kaydı kilitle | fn basılıyken Shift |
| Kilitli bir kayıt başlat | Shift+fn ya da hızlıca iki kez fn |
| Kilitli bir kaydı bitir | fn ya da Shift |
| Lowspeak’ten çık (menü açıkken) | ⌘Q |
| Hızlı Başvuru’yu kapat | Esc |

Lowspeak, kullandığı fn basışlarının diğer uygulamalara ulaşmasını engeller; Shift her zamanki gibi geçer.

## Gizlilik ve güvenlik

Lowspeak, fn basılıyken söylediklerinizi duyar, herhangi bir uygulamaya yazabilir ve API anahtarlarınızı saklar; bu yüzden bunlarla olabildiğince az şey yapar.

- **Sesiniz tek bir yere gider.** Çevrimiçi modda kayıt sıkıştırılmış olarak, sözlüğünüzle birlikte seçtiğiniz sağlayıcıya, OpenAI ya da Groq’a gider. Yerel modda hiçbir şey Mac’inizden çıkmaz. Bir çevrimiçi istek başarısız olduğunda aynı kaydı Mac’inizde bir yerel model metne dönüştürür.
- **Son işleme**, açarsanız, metne dönüştürülmüş metni ve sözlüğünüzü Groq’a ya da OpenAI’a gönderir. Biçimlendirme modunda dikte ettiğiniz uygulamanın türünü de ekler (kod düzenleyici, terminal, sohbet ya da e-posta); uygulamanın adı gönderilmez.
- **Telemetri yok, hesap yok.** Lowspeak, metne dönüştürme ve son işleme dışında yalnızca çevrimiçi modda ya da son işleme açıkken fn basılıyken sağlayıcınızla bağlantıyı açmak için ve siz istediğinizde Hugging Face’ten HTTPS üzerinden, sabit bir sürümde ve her dosyayı sağlama toplamıyla denetleyerek bir model indirmek için ağa bağlanır. **Yardım › Sorun Bildir…** tarayıcınızda GitHub’da bir form açar; formun adresi Lowspeak’in sürümünü ve macOS sürümünü taşır, dikte ettiğiniz hiçbir şeyi taşımaz.
- **API anahtarları** oturum açma anahtar zincirinizde kalır. Yalnızca kendi sağlayıcılarına gönderilir ve günlüğe herhangi bir şey yazılmadan önce hata iletilerinden çıkarılır.
- **Dikte edilen metin saklanmaz.** Son on dikte **Son dikteler** için bellekte kalır ve Lowspeak kapandığında silinir. Günlük dosyaları uzunlukları ve süreleri kaydeder, metni asla.
- **Pano** yapıştırmadan önce kaydedilir ve uygulama metni okuduğunda geri yüklenir; bu arada başka bir uygulama panoyu değiştirdiyse geri yüklenmez. Dikte edilen metin geçici olarak işaretlenir; bu işarete uyan pano geçmişi uygulamaları onu atlar.
- **Kod imzalama**, Hardened Runtime ile; uygulama yalnızca mikrofon, Erişilebilirlik, bildirimler ve ağ için izin ister.

Bir güvenlik açığını bildirmek için [güvenlik politikasını](../../../SECURITY.md) izleyin.

## Hız

Apple Silicon’da ölçülmüştür:

- **Parakeet Ultra**, 40 saniyelik konuşmayı yaklaşık 0,3 saniyede metne dönüştürür.
- **Çevrimiçi:** Bir aradan sonraki ilk istek Groq’ta yaklaşık 0,4 saniye sürer, çünkü Lowspeak siz daha konuşurken bağlantıyı açar; sonraki istekler bu bağlantıyı yeniden kullanır, sağlayıcı sunuyorsa HTTP/3 üzerinden.
- **Yükleme boyutu:** Konuşmanın saniyesi başına yaklaşık 6 KB, 48 kbit/s AAC olarak.
- **Mikrofonun hazır hâle gelmesi:** Dahili mikrofonda yaklaşık 0,07 saniye, Studio Display’de yarım saniye, AirPods ile bir ila iki saniye.

## Bilinen sınırlamalar

- **Intel Mac’lerde yerel modeller** işlemcide çalışır ve çok daha yavaştır; orada çevrimiçi metne dönüştürme daha iyi bir seçimdir.
- **Mikrofon hazır olmadan söylediğiniz kaybolur.** Özellikle AirPods ile kayıt simgesini bekleyin.
- **fn tuşunu başkaları kapabilir.** macOS’in 🌐 tuşu ayarı, Warp ya da macOS Dikte fn tuşunu önce sahiplenirse Lowspeak onu görmez; bkz. [Sorun giderme](#sorun-giderme).
- **Parakeet** sözlüğü ve dil seçimini yok sayar ve yalnızca Avrupa dillerini bilir.
- **Yazı sistemi ve değişke:** Yalnızca GPT Transcribe’a Geleneksel Çince yazması söylenebilir; hiçbir modele Brezilya Portekizcesi yerine Avrupa Portekizcesi yazması söylenemez.
- **Süre sınırları:** fn basılıyken iki dakika, kilitliyken beş dakika.
- **Mikrofon Modu:** Denetim Merkezi Lowspeak için Ses İzolasyonu ve Geniş Spektrum seçeneklerini sunar ve bunları etkin olarak gösterir; ancak Lowspeak ses işleme olmadan kaydeder, bu yüzden seçimin hiçbir etkisi olmaz.
- **Son dikteler** yalnızca bellekte tutulur ve çıktıktan sonra silinir.
- **Tasarım gereği:** Ayarlar penceresi yok, Dock simgesi yok, otomatik güncelleme yok, App Store’da değil.

## Sorun giderme

**fn tuşunu basılı tuttuğumda hiçbir şey olmuyor.** Üç şeyi denetleyin: Lowspeak’e **Sistem Ayarları › Gizlilik ve Güvenlik › Erişilebilirlik** altında (macOS 27’de: **Aygıt Denetimi ve Veri Erişimi**) izin verilmiş olmalı; **Sistem Ayarları › Klavye › 🌐 tuşuna basıldığında** ayarı **Hiçbir Şey Yapma** olmalı; ve fn tuşunu başka hiçbir uygulama sahiplenmemeli. Warp bunu varsayılan olarak yapar; `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` ile kapatın ve Warp’ı yeniden başlatın. macOS Dikte de fn ile başlayacak şekilde ayarlanmış olabilir.

**Bir güncellemeden sonra Lowspeak artık fn tuşuna tepki vermiyor.** macOS, Erişilebilirlik iznini uygulamanın imzasına bağlar. İmza değiştiyse, örneğin kendi derlemenizden indirilen uygulamaya geçtiyseniz, Lowspeak’i Erişilebilirlik listesinden (macOS 27’de: Aygıt Denetimi ve Veri Erişimi) kaldırın ve yeniden ekleyin.

**İlk sözcükler eksik.** Kayıt simgesi belirdiğinde konuşmaya başlayın. AirPods ile mikrofonun bir ila iki saniyeye ihtiyacı vardır; **Mikrofon** altında seçilen dahili ya da USB mikrofon neredeyse anında hazırdır.

**“Mikrofon sessiz.”** Mikrofon beş saniye boyunca hiç ses vermedi. AirPods ile başka bir aygıt onların mikrofonunu kullanıyor olabilir; dahili mikrofona geçin ya da AirPods’u yeniden bağlayın. Kapalı bir MacBook kapağı da dahili mikrofonu susturur.

**Dikteler boş çıkıyor.** Giriş ses düzeyi çok düşük olabilir: Teams, Zoom ya da Meet gibi uygulamalar görüşmeler sırasında onu değiştirir ve macOS onu her aygıt için ayrı tutar. **Sistem Ayarları › Ses › Giriş** altından yükseltin.

**Metin yanlış yere düştü.** Menüdeki **Son dikteler** onu yeniden panoya kopyalar.

**“API anahtarı yok.”** Menüden **API anahtarını ayarla…** seçeneğini seçin ya da bir yerel modele geçin.

**İlk yerel metne dönüştürme uzun sürüyor.** Her model ilk kullanımında Mac’iniz için hazırlanır; o andan sonra saniyenin çok küçük bir bölümünde yüklenir.

**Lowspeak açılır açılmaz kapanıyor.** Lowspeak’in başka bir kopyası zaten çalışıyor; önce ondan çıkın.

**Hiç bildirim görünmüyor.** **Sistem Ayarları › Bildirimler › Lowspeak** altından izin verin.

**Başka bir şey.** `~/Library/Application Support/Lowspeak/` içindeki günlükler (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`), dikte ettiğiniz metin olmadan ne olduğunu anlatır. İlgili satırları bir [sorun bildirimine](https://github.com/sventmueller/lowspeak/issues/new/choose) ekleyin.

## Geliştiriciler için

Derleme, test, kodun yapısı ve uyduğu kurallar [docs/development.md](../../../docs/development.md) dosyasında, katkıda bulunma ise [CONTRIBUTING.md](../../../CONTRIBUTING.md) dosyasında anlatılıyor (İngilizce).

### Üçüncü taraf bileşenler

| Bileşen | Sürüm | Kullanım amacı | Lisans |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp), [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) aracılığıyla | 0.16 | GPU’da (Metal) yerel Whisper | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeet kod çözme ve uzun kayıtları birleştirme, `src/transcription/parakeet` içinde Rust’a çevrilmiş | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) ve çerçeve crate’leri | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation ve diğer sistem çerçeveleri | MIT; Zlib, Apache-2.0 veya MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | CoreAudio üzerinden mikrofon kaydı | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | 16 kHz’e yeniden örnekleme | MIT |
| [ureq](https://github.com/algesten/ureq), rustls ile | 3.5 | model indirmeleri | MIT veya Apache-2.0; rustls ve ring ISC ve Apache-2.0 altında |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | model indirmelerinin sağlama toplamları | MIT veya Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | anahtar zincirindeki API anahtarları | MIT veya Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, hatalar, Parakeet’in sözcük dağarcığı | MIT veya Apache-2.0; Apache-2.0 |

Uygulamanın içindeki her şeyin lisans metinleri, uygulamayla birlikte `Contents/Resources/Acknowledgements.txt` olarak gelir.

Modeller Lowspeak’in parçası değildir; Lowspeak onları siz istediğinizde indirir:

| Model | Kaynak | Lisans |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (OpenAI’ın Whisper modeli, ggml biçiminde) | MIT |
| Parakeet v2, Parakeet Ultra | NVIDIA’nın Parakeet TDT modelinin [FluidInference](https://huggingface.co/FluidInference) tarafından yapılan CoreML dönüştürmeleri | CC-BY-4.0 |

## Lisans

Lowspeak, aşağıdakilerden herhangi birinin altında lisanslanmıştır

- Apache Lisansı, Sürüm 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- MIT Lisansı ([LICENSE-MIT](../../../LICENSE-MIT))

tercihinize bağlı olarak.

Siz açıkça başka türlü belirtmedikçe, Apache-2.0 lisansında tanımlandığı üzere Lowspeak’e dahil edilmek üzere sizin tarafınızdan kasıtlı olarak sunulan her katkı, ek hüküm ya da koşul olmaksızın yukarıdaki gibi çift lisanslı olacaktır.
