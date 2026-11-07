<p><img src="../../readme-icon.png" alt="Lowspeakin sovelluskuvake" width="168" height="168"></p>

# Lowspeak

**Sanelu Macille, joka kirjoittaa sinne, missä kohdistin on: pidä fn painettuna, puhu, päästä irti.**

Lowspeak asuu valikkorivillä. Pidä fn-näppäintä (Maapallo) painettuna ja puhu; kun päästät irti, sanasi ilmestyvät siihen appiin, jossa kirjoitat, ikään kuin olisit kirjoittanut ne itse. Se litteroi verkossa OpenAI:n tai Groqin avulla tai kokonaan Macillasi Whisperillä tai NVIDIA Parakeetilla, joka muuttaa 40 sekuntia puhetta tekstiksi noin kolmasosasekunnissa. Jos verkko tai verkkopalvelu pettää, asennettu paikallinen malli ottaa ohjat, joten sanelu ei katoa. Opeta sille käyttämäsi nimet ja termit, anna halutessasi kielimallin poistaa täytesanat ja lukitse äänitys Shiftillä, kun haluat sanella pidempiä tekstejä handsfree-tilassa. Se on kirjoitettu Rustilla aitojen AppKit-ohjainten päälle, puhuu 35 kieltä ja lähettää äänesi vain valitsemallesi palveluntarjoajalle.

[![Lataa Lowspeak macOS:lle](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Lataa Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: Applen allekirjoittama ja notarisoima, macOS 14:lle tai uudemmalle, Apple Silicon- ja Intel-Maceille.

**Löysitkö virheen?** [Avaa issue](https://github.com/sventmueller/lowspeak/issues/new/choose) tai valitse Lowspeakin valikosta **Ohje › Ilmoita ongelmasta…**: lomake avautuu versiosi valmiiksi täytettynä. Jos Lowspeak on sinulle hyödyllinen, [tähti GitHubissa](https://github.com/sventmueller/lowspeak) auttaa muita löytämään sen.

[![Rust 2024 -versio](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 tai uudempi](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon ja Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit objc2:n kautta](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp Metalilla](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet Neural Enginellä](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Verkossa OpenAI tai Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 käyttöliittymäkieltä](https://img.shields.io/badge/languages-35-4C9A2A)
![Allekirjoitettu Hardened Runtimella](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Lisenssi: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#lisenssi)

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
**Suomi** ·
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

## Sisältö

- [Pähkinänkuoressa](#pähkinänkuoressa)
- [Vaatimukset](#vaatimukset)
- [Asennus](#asennus)
- [Aloittaminen](#aloittaminen)
  - [Käyttöoikeudet](#käyttöoikeudet)
  - [Litterointitavan valinta](#litterointitavan-valinta)
  - [Valikko](#valikko)
  - [Pikaopas](#pikaopas)
- [Saneleminen](#saneleminen)
  - [Pidä painettuna ja puhu](#pidä-painettuna-ja-puhu)
  - [Lukitse Shiftillä tai kaksoispainalluksella](#lukitse-shiftillä-tai-kaksoispainalluksella)
  - [Minne teksti menee](#minne-teksti-menee)
  - [Viimeaikaiset sanelut](#viimeaikaiset-sanelut)
- [Litterointi](#litterointi)
  - [Mikä malli sopii sinulle](#mikä-malli-sopii-sinulle)
  - [Verkossa: OpenAI ja Groq](#verkossa-openai-ja-groq)
  - [Macillasi: Whisper ja Parakeet](#macillasi-whisper-ja-parakeet)
  - [Kun verkkopalvelu pettää](#kun-verkkopalvelu-pettää)
  - [Miten äänitys valmistellaan](#miten-äänitys-valmistellaan)
  - [Kielet malleittain](#kielet-malleittain)
  - [Intel-Macit](#intel-macit)
- [Sanasto](#sanasto)
- [Jälkikäsittely](#jälkikäsittely)
- [Mikrofoni](#mikrofoni)
- [API-avaimet](#api-avaimet)
- [Ilmoitukset](#ilmoitukset)
- [Ulkoasu ja saavutettavuus](#ulkoasu-ja-saavutettavuus)
- [Kielet](#kielet)
- [Näppäimet ja eleet](#näppäimet-ja-eleet)
- [Tietosuoja ja turvallisuus](#tietosuoja-ja-turvallisuus)
- [Nopeus](#nopeus)
- [Tunnetut rajoitukset](#tunnetut-rajoitukset)
- [Vianmääritys](#vianmääritys)
- [Kehittäjille](#kehittäjille)
  - [Kolmannen osapuolen komponentit](#kolmannen-osapuolen-komponentit)
- [Lisenssi](#lisenssi)

## Pähkinänkuoressa

- **Pidä fn painettuna, puhu, päästä irti.** Teksti kirjoitetaan edessä olevaan appiin, kohdistimen kohdalle: sähköpostiin, chattiin, editoriin, päätteeseen tai selaimeen. Ei ikkunaa, johon vaihtaa, ei painiketta, jota napsauttaa.
- **Lukitse pidempiä tekstejä varten.** Paina Shiftiä pitäessäsi fn:ää painettuna tai paina fn:ää kahdesti, niin äänitys jatkuu ilman, että pidät mitään näppäintä painettuna, enintään viisi minuuttia. Yksi fn- tai Shift-painallus lopettaa sen.
- **Oikea malli oikeaan tehtävään.** Groq nopeuteen kuluttamatta Macisi muistia, OpenAI:n GPT Transcribe parhaaseen tulokseen, Whisper tai NVIDIA Parakeet Macillasi offline-työskentelyyn ja äänesi pitämiseen yksityisenä. Vaihda yhdellä napsautuksella.
- **Yksikään sanelu ei katoa.** Jos verkko tai verkkopalvelu pettää, asennettu paikallinen malli litteroi saman äänityksen Macillasi.
- **Sanasi oikein kirjoitettuina.** Nimien ja erikoistermien sanasto ohjaa OpenAI:ta, Groqia, Whisperiä ja jälkikäsittelyä.
- **Siisti teksti, jos haluat.** Valinnainen jälkikäsittely poistaa täytesanat, korjaa kieliopin ja säilyttää vain korjauksen, kun korjaat itseäsi, tai muotoilee tekstin lisäksi sen mukaan, millaiseen appiin sanelet.
- **Jatka puhumista.** Aloita seuraava sanelu, vaikka edellistä vielä litteroidaan; tekstit saapuvat siinä järjestyksessä, jossa sanoit ne.
- **Leikepöytäsi pysyy omanasi.** Lowspeak sijoittaa tekstin leikepöydän kautta ja palauttaa hetkeä myöhemmin sen, mitä leikepöydällä oli. Kymmenen viimeisintä sanelua jää valikkoon uudelleen kopioitaviksi.
- **Natiivi läpikotaisin.** Aito valikkorivin kohde AppKit-valikolla, vakiomuotoiset valintaikkunat ja ilmoitukset. Ei Electronia, ei ikkunaa, ei Dock-kuvaketta.
- **Yksityinen.** Ei tiliä, ei telemetriaa. Ääni menee vain valitsemallesi palveluntarjoajalle, tai paikallisessa tilassa ei minnekään.
- **Omalla kielelläsi.** Sanelu toimii kymmenillä kielillä, ja valikko, valintaikkunat ja ilmoitukset ovat saatavilla 35 kielellä.

## Vaatimukset

- macOS 14 Sonoma tai uudempi. Lowspeakia kehitetään ja testataan macOS 27:llä.
- Mac, jossa on Apple Silicon tai Intel-suoritin. Paikallinen litterointi on tehty Apple Siliconille, jossa Whisper toimii grafiikkasuorittimella ja Parakeet Neural Enginellä; Intel-Maceissa molemmat toimivat suorittimella ja paljon hitaammin, katso [Intel-Macit](#intel-macit).
- Noin 17 Mt appia varten sekä 75 Mt:stä 3 Gt:hen jokaista lataamaasi paikallista mallia kohden.
- Verkkolitterointiin API-avain palvelusta [OpenAI](https://platform.openai.com/api-keys) tai [Groq](https://console.groq.com/keys); molemmat laskuttavat litteroimastasi määrästä. Paikallinen litterointi ei vaadi tiliä.

## Asennus

**Levykuvasta.** Lataa [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), avaa se ja vedä Lowspeak Apit-kansioon. Kuva on allekirjoitettu Developer ID:llä ja Apple on notarisoinut sen, joten se aukeaa ilman varoituksia.

**Päivitykset.** Levykuvasta asennettu Lowspeak päivittää itsensä [Sparklen](https://sparkle-project.org) avulla. Toisella käynnistyskerralla se kysyy, saako se etsiä päivityksiä itse kerran päivässä. Valikon **Etsi päivityksiä…** etsii heti; sen alla olevat **Etsi päivityksiä automaattisesti** ja **Lataa ja asenna päivitykset automaattisesti** muuttavat valintaasi myöhemmin. Jos haku löytää uuden version, kun työskentelet toisessa apissa, valikkorivin renkaaseen ilmestyy piste ja **Etsi päivityksiä…** tuo päivityksen etualalle. Lowspeak asentaa päivityksen vain, jos siinä on kehittäjän allekirjoitus; asetuksesi, avaimesi ja mallisi säilyvät. Versiossa 1.0.0 ei ole päivitysohjelmaa: asenna uudempi versio sen päälle kerran [julkaisusivulta](https://github.com/sventmueller/lowspeak/releases). Myöskään itse rakennetussa versiossa ei ole päivitysohjelmaa.

**Lähdekoodista.** Katso esivaatimukset kohdasta [Rakentaminen ja käyttöönotto](../../../docs/development.md#build--deploy) ja suorita sitten:

```sh
make deploy
```

Tämä rakentaa Lowspeakin, korvaa Apit-kansiossa olevan kopion ja käynnistää sen.

Valikon **Avaa sisäänkirjauduttaessa** käynnistää Lowspeakin yhdessä Macisi kanssa macOS:n omien kirjautumiskohteiden kautta. Vain yksi kopio voi olla käynnissä kerrallaan; toinen sulkeutuu heti.

## Aloittaminen

### Käyttöoikeudet

Ensimmäisellä käynnistyskerralla Lowspeak pyytää kolmea käyttöoikeutta:

- **Käyttöapu** (Järjestelmäasetukset › Tietosuoja ja suojaus › Käyttöapu; macOS 27:ssä luettelon nimi on Laitehallinta ja tietojen käyttö). Lowspeak tarvitsee sen huomatakseen fn-näppäimen ja sijoittaakseen tekstin ⌘V:llä. macOS tarjoutuu avaamaan luettelon; laita Lowspeakin kytkin päälle. Lowspeakin omat viestit käyttävät luettelosta samaa nimeä kuin macOS-versiosi.
- **Mikrofoni.** Kysytään kerran; ilman sitä Lowspeak ei äänitä mitään.
- **Ilmoitukset.** Lowspeak käyttää niitä ongelmiin, joita et muuten näkisi, kuten hiljaiseen mikrofoniin tai epäonnistuneeseen litterointiin.

### Litterointitavan valinta

Ota käyttöön vähintään yksi tapa; voit vaihtaa milloin tahansa valikosta.

- **Verkossa:** valitse **Aseta API-avain…** ja sijoita avain OpenAI:lta (se alkaa merkeillä `sk-`) tai Groqilta (`gsk_`). Lowspeak tunnistaa palveluntarjoajan avaimesta. Valitse sitten malli kohdasta **OpenAI** tai **Groq**.
- **Macillasi:** valitse **Hallitse malleja** › **Lataa Parakeet Ultra** tai jokin Whisper-malli ja valitse se sitten kohdasta **Paikallinen**.

[Mikä malli sopii sinulle](#mikä-malli-sopii-sinulle) auttaa valinnassa. Pidä sitten fn painettuna, sano jotain ja päästä irti.

### Valikko

Napsauta valikkorivillä olevaa rengasta:

| Kohde | Mitä se tekee |
|---|---|
| **Viimeaikaiset sanelut** | kymmenen viimeisintä tekstiä; napsautus kopioi yhden niistä uudelleen leikepöydälle |
| **Paikallinen** | Macisi mallit Whisper- ja Parakeet-ryhmiin jaettuina; lataamattomat näkyvät himmennettyinä |
| **OpenAI**, **Groq** | verkkomallit; ilman avainta ne näkyvät himmennettyinä ja valikko tarjoaa kohdan **Aseta API-avain…** |
| **Kieli** | kieli, jolla sanelet, tai **Tunnista automaattisesti**; valikko seuraa sitä |
| **Jälkikäsittely** | **Pois**, **Siisti** tai **Muotoile**, katso [Jälkikäsittely](#jälkikäsittely) |
| **Sanasto…** | nimet ja termit, jotka kirjoitetaan täsmälleen oikein; valikko näyttää, montako olet syöttänyt |
| **Mikrofoni** | **Järjestelmän oletus** tai kiinteä mikrofoni sekä **Toista ääni, kun valmis** |
| **Hallitse malleja** | lataa ja poista paikallisia malleja edistymisen näyttäen |
| **Aseta API-avain…** / **Vaihda API-avain…** | lisää, korvaa tai poista avaimia |
| **Avaa sisäänkirjauduttaessa** | käynnistä Lowspeak yhdessä Macisi kanssa; valintamerkki näyttää, että se on päällä |
| **Etsi päivityksiä…** | etsi uutta versiota heti; sen alla olevat kaksi kohdetta ratkaisevat, etsiikö Lowspeak kerran päivässä ja asentaako se päivitykset kysymättä. Vain levykuvasta asennetussa apissa |
| **Ohje** | **Pikaopas**, **Ilmoita ongelmasta…**, **Ehdota ominaisuutta…**, **Lowspeak GitHubissa** |
| **Tietoja: Lowspeak** | versio, tekijänoikeus, projektisivu ja lisenssi |
| **Lopeta Lowspeak** | ⌘Q, kun valikko on auki |

Valintamerkki kohdissa **Paikallinen**, **OpenAI** ja **Groq** osoittaa käytössä olevan mallin. Kuvake näyttää, mitä Lowspeak tekee:

| Kuvake | Merkitys |
|---|---|
| Rengas, jossa on piste | Valmis |
| Täytetty ympyrä | Äänittää, kun fn on painettuna |
| Pyöristetty neliö | Äänitys lukittu; seuraava fn- tai Shift-painallus lopettaa sen |
| Vilkkuva rengas | Litteroi |
| Rengas, jossa on piste oikeassa yläkulmassa | Uusi versio odottaa, katso **Etsi päivityksiä…** |
| ↓ 42 % | Lataa mallia |

Lowspeakissa ei ole asetusikkunaa: kaikki on tässä valikossa, ja se muistaa jokaisen valinnan.

### Pikaopas

**Ohje › Pikaopas** näyttää fn-eleet pienessä ikkunassa omalla kielelläsi, ja siinä on painike, joka avaa näppäimistöasetukset. **Ilmoita ongelmasta…** avaa GitHubissa lomakkeen, johon Lowspeakin ja macOS:n versiot on valmiiksi täytetty, **Ehdota ominaisuutta…** keskustelun ideoille ja **Lowspeak GitHubissa** projektisivun.

## Saneleminen

### Pidä painettuna ja puhu

Pidä fn painettuna, puhu, päästä irti. Äänityskuvake ilmestyy sillä hetkellä, kun mikrofoni alkaa välittää ääntä; aloita puhuminen silloin, sillä sitä ennen sanomaasi ei äänitetä. Kun päästät fn:n irti, äänitys päättyy, ja teksti seuraa hetken kuluttua.

Lyhyt fn-napautus ei tee mitään, joten fn säilyttää muut käyttötarkoituksensa. Äänitys, joka jää tyhjäksi tai kestää alle 0,3 sekuntia, hylätään.

### Lukitse Shiftillä tai kaksoispainalluksella

Pidempiä tekstejä varten lukitse äänitys, jotta se jatkuu ilman, että pidät näppäintä painettuna. Tapoja on kolme:

- **Shift fn:n ollessa painettuna.** Aloita fn:llä tavalliseen tapaan ja paina Shiftiä äänityksen aikana: äänitys lukittuu, ja voit päästää fn:n irti. Mitään jo sanomaasi ei menetetä, äänitys yksinkertaisesti jatkuu.
- **Ensin Shift, sitten fn.** Pidä Shiftiä painettuna ja paina fn:ää: äänitys alkaa lukittuna.
- **fn kahdesti.** Napauta fn:ää lyhyesti (alle 0,3 sekuntia) ja paina sitä uudelleen 0,4 sekunnin kuluessa. Äänitys alkaa ensimmäisestä painalluksesta.

Kuvake muuttuu pyöristetyksi neliöksi. Lopeta painamalla kerran **fn** tai **Shift**; teksti seuraa tavalliseen tapaan. Jos 🌐-näppäimen asetuksena on Näytä emojit ja symbolit, lopeta Shiftillä: lyhyt fn-painallus avaisi Merkit-ikkunan.

Äänitys pysähtyy itsestään kahden minuutin jälkeen, kun fn on painettuna, ja viiden minuutin jälkeen, kun se on lukittu, ja litteroidaan sitten tavalliseen tapaan.

### Minne teksti menee

Lowspeak sijoittaa tekstin siihen appiin, joka oli edessä, kun lopetit puhumisen, kohdistimen kohdalle, leikepöydän ja ⌘V:n kautta. Se tallentaa kaiken, mitä leikepöydällä oli ennen, ja palauttaa sen puoli sekuntia sen jälkeen, kun vastaanottava appi on lukenut tekstin, tai kymmenen sekunnin kuluttua, jos se ei lue sitä koskaan, ellei jokin toinen appi ole sillä välin muuttanut leikepöytää. Teksti merkitään tilapäiseksi, joten leikepöytähistoriaa keräävät apit, jotka noudattavat merkintää, eivät tallenna sitä.

Voit aloittaa seuraavan sanelun, vaikka edellistä vielä litteroidaan. Tekstit sijoitetaan siinä järjestyksessä, jossa sanoit ne, eikä koskaan silloin, kun äänität tai pidät Shiftiä painettuna, joten sijoitus ei voi osua keskelle seuraavaa lausettasi.

Ilman Käyttöapu-oikeutta, tai jos simuloitua ⌘V:tä ei voida lähettää, teksti jää leikepöydälle ja ilmoitus kertoo siitä; sijoita se itse ⌘V:llä.

### Viimeaikaiset sanelut

Valikon **Viimeaikaiset sanelut** luetteloi kymmenen viimeisintä tekstiä uusin ensin, kukin yhdelle riville lyhennettynä; kun osoitin on tekstin päällä, koko teksti tulee näkyviin. Napsautus kopioi sen leikepöydälle siltä varalta, että teksti päätyi väärään paikkaan tai tarvitset sitä kahdesti. Luettelo on vain muistissa ja tyhjenee, kun Lowspeak lopetetaan.

## Litterointi

### Mikä malli sopii sinulle

| Tarve | Valitse | Miksi |
|---|---|---|
| **Nopeus kuormittamatta Macia** | **Groq › Whisper Large V3 Turbo** | nopein verkkovaihtoehto; mitään ei ladata, ja Macisi muisti ja levytila säästyvät |
| **Paras tulos** | **OpenAI › GPT Transcribe** (oletus) | Lowspeakin tarjoamista malleista tarkin; ottaa sanastosi vastaan avainsanoina |
| **Offline ja yksityisesti** | **Paikallinen › Parakeet Ultra** eurooppalaisille kielille, **Paikallinen › Whisper Large v3 Turbo** kaikille muille | toimii ilman verkkoa, eikä äänesi koskaan poistu Macistasi; Parakeet on kaikista nopein |
| **Ei tiliä, ei kuluja** | mikä tahansa malli kohdasta **Paikallinen** | toimii Macillasi; mitään ei laskuteta |
| **Sekä luotettavuus että nopeus** | verkkomalli sekä asennettu Parakeet Ultra tai Whisper-malli | paikallinen malli astuu esiin, kun verkkopalvelu pettää |
| **Intel-Mac** | **Groq** tai **OpenAI** | paikalliset mallit toimivat siellä suorittimella, katso [Intel-Macit](#intel-macit) |

Paikalliset mallit vievät levyltä latauskokonsa verran tilaa ja valittuina suunnilleen saman verran muistia; Lowspeak pitää mallin ladattuna, jotta sanelu alkaa viiveettä. Jos Macissasi on 8 Gt muistia, suosi Parakeet Ultraa tai verkkomallia suurten Whisper-mallien sijaan.

### Verkossa: OpenAI ja Groq

| Malli | Palveluntarjoaja | Huomautukset |
|---|---|---|
| **GPT Transcribe** (oletus) | OpenAI | tarkin; ottaa sanastosi vastaan avainsanoina |
| **Whisper** | OpenAI | alkuperäinen Whisper-malli |
| **Whisper Large V3** | Groq | erittäin nopea |
| **Whisper Large V3 Turbo** | Groq | nopein verkossa |

Äänitys pakataan AAC-muotoon ennen lähettämistä, noin 6 kt puhesekuntia kohden, joten sanelu toimii myös hitailla yhteyksillä. Kun pidät fn:ää painettuna, Lowspeak avaa jo yhteyden palveluntarjoajaan, joten lähetys alkaa sillä hetkellä, kun päästät irti.

### Macillasi: Whisper ja Parakeet

| Malli | Koko | Käyttää | Kielet |
|---|---|---|---|
| **Parakeet Ultra** | ~630 Mt | Neural Engine | 25 eurooppalaista kieltä |
| **Parakeet v2** | ~600 Mt | Neural Engine | englanti |
| **Whisper Large v3 Turbo** | ~1,6 Gt | GPU (Metal) | noin 99 |
| **Whisper Large v3** | ~3 Gt | GPU (Metal) | noin 99 |
| **Whisper Medium** | ~1,5 Gt | GPU (Metal) | noin 99 |
| **Whisper Small** | ~470 Mt | GPU (Metal) | noin 99 |
| **Whisper Base** | ~150 Mt | GPU (Metal) | noin 99 |
| **Whisper Tiny** | ~75 Mt | GPU (Metal) | noin 99 |

**Hallitse malleja** lataa ja poistaa niitä; latauksen aikana valikkorivi näyttää sen edistymisen, ja Lowspeakin lopettaminen peruu latauksen. Mallin aivan ensimmäinen käyttökerta valmistelee sen Macillesi: noin 10 sekuntia Parakeet Ultralla, enintään 30 sekuntia Whisperillä; sen jälkeen malli latautuu sekunnin murto-osassa. Jos valitsemasi malli poistetaan, Lowspeak vaihtaa toiseen asennettuun malliin.

**Parakeet** on nopein vaihtoehto: se litteroi 40 sekuntia puhetta noin 0,3 sekunnissa. Lowspeak ajaa sitä samoilla malleilla ja samalla dekoodauksella kuin [FluidAudio](https://github.com/FluidInference/FluidAudio) ja tallentaa mallit FluidAudion kansioon (`~/Library/Application Support/FluidAudio/Models`), joten FluidAudioon perustuvat apit jakavat ne. Yli 15 sekunnin äänitykset litteroidaan limittäisinä ikkunoina, jotka yhdistetään menettämättä sanoja saumakohdissa. **Whisper** ymmärtää useampia kieliä ja lukee sanastosi; suuret mallit ovat tarkimpia, pienet selvästi heikompia muilla kielillä kuin englannilla. Whisper-mallit tallennetaan kansioon `~/Library/Application Support/Lowspeak/Models`.

### Kun verkkopalvelu pettää

Jos verkkolitterointi epäonnistuu, koska verkko ei toimi, palveluntarjoajalla on ongelma tai avain on väärä, asennettu paikallinen malli litteroi saman äänityksen: kohdassa **Paikallinen** valittu malli, muuten Parakeet Ultra, sitten Whisper Large v3 Turbo, Large v3 ja pienemmät Whisper-mallit, viimeisenä Parakeet v2. Ilmoitus kertoo tästä kerran; se tulee uudelleen vasta, kun välillä jokin verkkosanelu on onnistunut.

Kun paikallinen varamalli on käytettävissä, Lowspeak antaa verkkopyynnölle 20 sekuntia; ilman sitä se yrittää kolme kertaa, kukin yritys enintään minuutin. Verkkotilassa Lowspeak valmistelee Parakeet-varamallin kerran käynnistyksen yhteydessä, jotta ensimmäinen varalitterointi ei joudu odottamaan sitä.

### Miten äänitys valmistellaan

Lowspeak äänittää mikrofonin omalla näytteenottotaajuudella ja muuntaa äänityksen lopettaessasi mallien odottamaan 16 kHz:iin. Ennen litterointia se poistaa alle 80 Hz:n matalan jyminän, leikkaa hiljaisuuden alusta ja lopusta ja tasoittaa äänenvoimakkuuden, jotta hiljaiset ja äänekkäät puhujat kuulostavat samalta. Äänitystä, jossa ei ole puhetta, ei lähetetä lainkaan. Paikallisessa tilassa Parakeet saa ensin käsittelemättömän äänityksen ja valmistellun vain, jos ensimmäisestä ei saada tekstiä.

### Kielet malleittain

Kohdassa **Kieli** valittu kieli välitetään verkkomalleille ja Whisperille. **Tunnista automaattisesti** -asetuksella malli päättelee kielen itse.

| Malli | Kielet, joilla voit sanella |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, Macillasi) | noin 99, niiden joukossa kaikki valikon 35 kieltä |
| Parakeet Ultra | bulgaria, englanti, espanja, hollanti, italia, kreikka, kroatia, latvia, liettua, malta, portugali, puola, ranska, romania, ruotsi, saksa, slovakki, sloveeni, suomi, tanska, tšekki, ukraina, unkari, venäjä, viro |
| Parakeet v2 | englanti |

Parakeet tunnistaa kielen itse eikä välitä valikon valinnasta. **Kiina (perinteinen)** saa GPT Transcriben kirjoittamaan perinteisiä merkkejä; Whisper kirjoittaa kiinaa kummalla tahansa kirjoitusjärjestelmällä ja portugalia välittämättä kielivariantista.

### Intel-Macit

Lowspeak on universaali appi ja toimii myös Intel-Maceissa, joissa on macOS 14. Kaikki verkossa toimii samoin. Paikallisilla malleilla ei siellä ole laitteistoa, jota varten ne on tehty: Whisper toimii grafiikkasuorittimen sijaan suorittimella ja Parakeet Neural Enginen sijaan suorittimella. Molemmat toimivat, mutta vievät moninkertaisesti enemmän aikaa kuin Apple Siliconilla, suuret Whisper-mallit aivan liian kauan sanelua varten. Intel-Macissa kannattaa litteroida verkossa ja pitää Whisper Small tai Parakeet Ultra [varalla](#kun-verkkopalvelu-pettää).

## Sanasto

**Sanasto…** ottaa vastaan nimet ja termit, jotka Lowspeakin pitää kirjoittaa täsmälleen oikein, pilkuilla erotettuina: tuotenimet, työtoverit, erikoistermit. Enintään 100 termiä, kukin enintään 60 merkkiä; `<` ja `>` poistetaan.

- **GPT Transcribe** saa ne avainsanoina.
- **Whisper** (OpenAI, Groq ja Macillasi) lukee ne vihjeenä ennen äänitystä, enintään 500 merkkiä.
- **Jälkikäsittelyä** ohjeistetaan kirjoittamaan ne täsmälleen niin kuin ne on kirjoitettu.
- **Parakeet** ei käytä niitä.

## Jälkikäsittely

Litteroinnin jälkeen kielimalli voi muokata tekstiä. Valitse kohdasta **Jälkikäsittely**:

- **Pois** (oletus) – teksti sellaisena kuin se litteroitiin.
- **Siisti** – poistaa täytesanat, kuten ”niinku”, ”tota” tai ”siis”, korjaa kieliopin ja välimerkit ja säilyttää vain korjauksen, kun korjaat itseäsi. Se ei lisää mitään eikä muuta merkitystä.
- **Muotoile** – kuten Siisti, ja lisäksi mukauttaa tekstin appiin, johon sanelet: koodimaiset termit koodieditoreissa, komentorivit päätteissä, rento sävy chat-apeissa, asiallinen sävy sähköpostissa.

Jälkikäsittely käyttää Groqia (`openai/gpt-oss-120b`), jos sinulla on Groq-avain, muuten OpenAI:ta (`gpt-4o-mini`), myös silloin, kun litteroit paikallisesti. Kun kohdassa **Kieli** on valittu kieli, jälkikäsittely säilyttää sen kielen. Jos se kestää yli kahdeksan sekuntia, epäonnistuu tai palauttaa vastauksen, joka on katkennut tai paljon pidempi kuin se, mitä sanoit, Lowspeak sijoittaa käsittelemättömän tekstin. Jos palveluntarjoaja hylkää pyynnön tai avainta ei ole asetettu, ilmoitus kertoo siitä kerran.

## Mikrofoni

Valitse kohdasta **Mikrofoni** kiinteä mikrofoni tai **Järjestelmän oletus**. Lowspeak muistaa laitteen; kun se ei ole yhdistettynä, valikko näyttää sen merkinnällä **(ei yhdistetty)** ja Lowspeak äänittää järjestelmän oletusmikrofonilla.

Mikrofoni kytketään päälle vain sanelun ajaksi. Sen valmistautuminen kestää noin 0,07 sekuntia sisäisellä mikrofonilla, noin puoli sekuntia Studio Displayllä ja yhdestä kahteen sekuntia AirPodseilla, jotka vaihtavat ensin kuulokemikrofonitilaan. Äänityskuvake ilmestyy, kun ääntä alkaa tulla; **Toista ääni, kun valmis** lisää siihen hetkeen lyhyen äänen. Se on oletuksena pois päältä, koska joissakin kaiuttimissa itse ääni kuuluu puoli sekuntia myöhässä.

Jos mikrofoni välittää viiden sekunnin ajan pelkkää hiljaisuutta, kuten AirPods tekee, kun toinen laite pitää niiden mikrofonia varattuna, Lowspeak kertoo siitä.

## API-avaimet

**Aseta API-avain…** kysyy avainta ja tunnistaa palveluntarjoajan sen alusta: `sk-` OpenAI:lle, `gsk_` Groqille. Muun muotoinen avain hylätään selityksen kera. Syötä yksi avain kutakin palveluntarjoajaa kohden; kun molemmille on avain, kaikki verkkomallit ovat käytettävissä. **Vaihda API-avain…** korvaa avaimen tai poistaa avaimet valinnalla **Poista kaikki avaimet**.

Avaimet tallennetaan sisäänkirjautumisavainnippuusi nimellä `com.lowspeak.app`. Aiemman version avaintiedosto (`~/Library/Application Support/Lowspeak/.api-key`) siirretään sinne seuraavalla käynnistyskerralla ja poistetaan, kun jokainen avain voidaan lukea takaisin avainnipusta.

## Ilmoitukset

Lowspeak toimii taustalla, joten se raportoi ongelmista ilmoituksina:

| Ilmoitus | Mitä tapahtui |
|---|---|
| **Äänitys epäonnistui** | ei pääsyä mikrofoniin, mikrofonia ei ole liitetty tai mikrofoni ei käynnistynyt |
| **Mikrofonista ei kuulu ääntä** | viisi sekuntia hiljaisuutta, katso [Mikrofoni](#mikrofoni) |
| **Käyttöapuoikeus vaaditaan** | teksti on leikepöydällä; salli Käyttöapu, jotta teksti sijoitetaan automaattisesti |
| **Sijoittaminen epäonnistui** | teksti on leikepöydällä; sijoita se ⌘V:llä |
| **Litterointi epäonnistui** | syyn kera: ei avainta, hylätty avain, pyyntöraja, palvelin- tai verkkovirhe tai ei paikallista mallia |
| **Litteroitu tässä Macissa** | verkkopalvelu petti, ja paikallinen malli otti ohjat |
| **Jälkikäsittely epäonnistui** | palveluntarjoaja hylkää jälkikäsittelyn tai avainta ei ole asetettu; teksti sijoitettiin ilman sitä |
| **Ei API-avainta** | valitsit verkkomallin ilman sen palveluntarjoajan avainta |
| **Lataus epäonnistui**, **Poistaminen epäonnistui**, **Poistaminen ei onnistu** | paikallisen mallin hallinta ei onnistunut, tai toinen toiminto oli vielä käynnissä |
| **Kirjautumiskohteen virhe**, **API-avaimen virhe** | macOS tai avainnippu torjui muutoksen |

Ilmoitukset eivät koskaan sisällä sanelemaasi tekstiä. Pettävästä verkkopalvelusta ja jälkikäsittelyn hylkäävästä palveluntarjoajasta ilmoitetaan kerran, ei jokaisen sanelun jälkeen, kunnes ne taas toimivat.

## Ulkoasu ja saavutettavuus

- **Valikkorivin kuvake** on mallikuva, joka mukautuu vaaleaan, tummaan tai sävytettyyn valikkoriviin.
- **VoiceOver** lukee kuvakkeen nimellä ”Lowspeak”, ja valikko, valintaikkunat ja ilmoitukset ovat tavallisia macOS-säätimiä.
- **Mitään näppäintä ei tarvitse pitää painettuna.** Lukitse äänitys Shiftillä tai fn:n kaksoispainalluksella, niin seuraava painallus lopettaa sen.
- **Pikaopas** on yksinkertainen ikkuna, jossa on tekstejä ja painike ja johon pääsee näppäimistöllä; Esc sulkee sen.

Tunnetut puutteet luetellaan tiedostossa [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver ei ilmoita äänityksen alkamisesta, eikä Lowspeakia ole vielä tarkistettu VoiceOverilla tai Kytkinohjauksella.

## Kielet

Valikko, valintaikkunat ja ilmoitukset on käännetty 35 kielelle:

arabia, englanti, espanja, heprea, hindi, hollanti, indonesia, italia, japani, katalaani, kiina (yksinkertaistettu), kiina (perinteinen), korea, kreikka, kroatia, malaiji, norja, portugali (Brasilia), portugali (Portugali), puola, ranska, romania, ruotsi, saksa, slovakki, sloveeni, suomi, tanska, thai, turkki, tšekki, ukraina, unkari, venäjä ja vietnam.

Kohdassa **Kieli** valitsemasi kieli on myös valikon kieli. **Tunnista automaattisesti** -asetuksella valikko seuraa kielijärjestystä kohdassa **Järjestelmäasetukset › Yleiset › Kieli ja alue** ja käyttää englantia, jos käännöstä ei ole. Valikkotermit noudattavat Applen omia käännöksiä, joten Lowspeak tuntuu samalta kuin muu Macisi. Englantia lukuun ottamatta käännökset ovat konekäännöksiä, eikä kukaan äidinkielinen puhuja ole vielä tarkistanut niitä; korjaukset ovat tervetulleita [issueina](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Näppäimet ja eleet

| Toiminto | Näppäimet |
|---|---|
| Sanele painettuna pitäen | pidä fn painettuna |
| Lukitse käynnissä oleva äänitys | Shift, kun fn on painettuna |
| Aloita lukittu äänitys | Shift+fn tai fn kahdesti nopeasti |
| Lopeta lukittu äänitys | fn tai Shift |
| Lopeta Lowspeak (valikko auki) | ⌘Q |
| Sulje pikaopas | Esc |

Lowspeak estää käyttämiään fn-painalluksia päätymästä muihin appeihin; Shift välittyy tavalliseen tapaan.

## Tietosuoja ja turvallisuus

Lowspeak kuulee, mitä sanot fn:n ollessa painettuna, voi kirjoittaa mihin tahansa appiin ja säilyttää API-avaimiasi, joten se tekee niillä niin vähän kuin mahdollista.

- **Äänesi menee yhteen paikkaan.** Verkkotilassa äänitys menee pakattuna valitsemallesi palveluntarjoajalle, OpenAI:lle tai Groqille, yhdessä sanastosi kanssa. Paikallisessa tilassa mikään ei poistu Macistasi. Jos verkkopyyntö epäonnistuu, paikallinen malli litteroi saman äänityksen Macillasi.
- **Jälkikäsittely**, jos otat sen käyttöön, lähettää litteroidun tekstin ja sanastosi Groqille tai OpenAI:lle. Muotoile-tilassa se lisää tiedon siitä, millaiseen appiin sanelet (koodieditori, pääte, chat tai sähköposti); apin nimeä ei lähetetä.
- **Ei telemetriaa, ei tiliä.** Litteroinnin ja jälkikäsittelyn lisäksi Lowspeak ottaa yhteyttä verkkoon vain avatakseen yhteyden palveluntarjoajaasi fn:n ollessa painettuna, verkkotilassa tai jälkikäsittelyn ollessa päällä, ladatakseen mallin pyynnöstäsi Hugging Facesta HTTPS:n kautta kiinteänä versiona niin, että jokaista tiedostoa verrataan sen tarkistussummaan, sekä etsiäkseen uutta versiota GitHubista kerran päivässä, jos sallit sen. Pyyntö sisältää Lowspeakin version eikä mitään muuta. **Ohje › Ilmoita ongelmasta…** avaa selaimessa lomakkeen GitHubissa; sen osoite sisältää Lowspeakin ja macOS:n versiot, ei mitään sanelemaasi.
- **API-avaimet** pysyvät sisäänkirjautumisavainnipussasi. Ne lähetetään vain omalle palveluntarjoajalleen, ja ne poistetaan virheilmoituksista ennen kuin mitään kirjoitetaan lokiin.
- **Saneltua tekstiä ei tallenneta.** Kymmenen viimeisintä sanelua pysyvät muistissa valikkoa **Viimeaikaiset sanelut** varten ja katoavat, kun Lowspeak lopetetaan. Lokitiedostot kirjaavat pituudet ja ajoitukset, eivät koskaan tekstiä.
- **Leikepöytä** tallennetaan ennen sijoittamista ja palautetaan, kun vastaanottava appi on lukenut tekstin, ellei jokin toinen appi ole sillä välin muuttanut sitä. Sanelu merkitään tilapäiseksi, joten leikepöytähistoriaa keräävät apit, jotka noudattavat merkintää, ohittavat sen.
- **Koodin allekirjoitus** Hardened Runtimella; appi pyytää pääsyä vain mikrofoniin, Käyttöapuun, ilmoituksiin ja verkkoon. Päivityksessä on oltava kehittäjän allekirjoitus, muuten Sparkle hylkää sen.

Jos haluat ilmoittaa haavoittuvuudesta, noudata [tietoturvakäytäntöä](../../../SECURITY.md).

## Nopeus

Mitattu Apple Siliconilla:

- **Parakeet Ultra** litteroi 40 sekuntia puhetta noin 0,3 sekunnissa.
- **Verkossa:** ensimmäinen pyyntö tauon jälkeen kestää Groqilla noin 0,4 sekuntia, koska Lowspeak avaa yhteyden, kun vielä puhut; myöhemmät pyynnöt käyttävät sitä uudelleen, HTTP/3:lla, jos palveluntarjoaja tukee sitä.
- **Lähetyksen koko:** noin 6 kt puhesekuntia kohden, AAC-muodossa 48 kbit/s.
- **Mikrofonin valmistautuminen:** noin 0,07 sekuntia sisäisellä, puoli sekuntia Studio Displayllä, yhdestä kahteen sekuntia AirPodseilla.

## Tunnetut rajoitukset

- **Paikalliset mallit Intel-Maceissa** toimivat suorittimella ja ovat paljon hitaampia; siellä verkkolitterointi on parempi valinta.
- **Se, mitä sanot ennen kuin mikrofoni on valmis, menetetään.** Odota äänityskuvaketta, etenkin AirPodsien kanssa.
- **Muut voivat viedä fn-näppäimen.** Kun macOS:n 🌐-näppäimen asetus, Warp tai macOS:n sanelu varaa fn:n ensin, Lowspeak ei näe sitä; katso [Vianmääritys](#vianmääritys).
- **Parakeet** ei välitä sanastosta eikä kielivalinnasta ja osaa vain eurooppalaisia kieliä.
- **Kirjoitusjärjestelmä ja kielivariantti:** vain GPT Transcriben voi ohjeistaa kirjoittamaan perinteistä kiinaa; mitään mallia ei voi ohjeistaa kirjoittamaan eurooppalaista portugalia brasilialaisen sijaan.
- **Aikarajat:** kaksi minuuttia, kun fn on painettuna, viisi minuuttia lukittuna.
- **Mikrofonitila:** Ohjauskeskus tarjoaa Lowspeakille vaihtoehdot Puheäänen eristys ja Laaja spektri ja näyttää ne aktiivisina, mutta Lowspeak äänittää ilman puheenkäsittelyä, joten valinnalla ei ole vaikutusta.
- **Viimeaikaiset sanelut** ovat vain muistissa ja katoavat, kun appi lopetetaan.
- **Tarkoituksella:** ei asetusikkunaa, ei Dock-kuvaketta, ei App Storessa.

## Vianmääritys

**Mitään ei tapahdu, kun pidän fn:ää painettuna.** Tarkista kolme asiaa: Lowspeak on sallittu kohdassa **Järjestelmäasetukset › Tietosuoja ja suojaus › Käyttöapu** (macOS 27:ssä: **Laitehallinta ja tietojen käyttö**); **Järjestelmäasetukset › Näppäimistö › Kun painetaan 🌐-näppäintä** on asetettu arvoon **Älä tee mitään**; eikä mikään muu appi varaa fn:ää. Warp tekee niin oletuksena; poista se käytöstä komennolla `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` ja käynnistä Warp uudelleen. Myös macOS:n sanelu voidaan asettaa käynnistymään fn:llä.

**Päivityksen jälkeen Lowspeak ei enää reagoi fn:ään.** macOS sitoo Käyttöapu-oikeuden apin allekirjoitukseen. Jos allekirjoitus muuttui, esimerkiksi itse rakennetusta versiosta ladattuun appiin, poista Lowspeak Käyttöapu-luettelosta (macOS 27:ssä: Laitehallinta ja tietojen käyttö) ja lisää se uudelleen.

**Ensimmäiset sanat puuttuvat.** Aloita puhuminen, kun äänityskuvake ilmestyy. AirPodsien mikrofoni tarvitsee yhdestä kahteen sekuntia; kohdassa **Mikrofoni** valittu sisäinen tai USB-mikrofoni on valmis lähes heti.

**”Mikrofonista ei kuulu ääntä.”** Mikrofoni ei välittänyt ääntä viiteen sekuntiin. AirPodsien kanssa jokin toinen laite saattaa käyttää niiden mikrofonia; vaihda sisäiseen mikrofoniin tai yhdistä ne uudelleen. Suljettu MacBookin kansi mykistää myös sisäisen mikrofonin.

**Sanelut jäävät tyhjiksi.** Tuloäänen voimakkuus voi olla liian pieni: apit, kuten Teams, Zoom tai Meet, muuttavat sitä puheluiden aikana, ja macOS säilyttää sen laitekohtaisesti. Nosta sitä kohdassa **Järjestelmäasetukset › Äänet › Tulo**.

**Teksti päätyi väärään paikkaan.** Valikon **Viimeaikaiset sanelut** kopioi sen uudelleen leikepöydälle.

**”Ei API-avainta.”** Valitse valikosta **Aseta API-avain…** tai vaihda paikalliseen malliin.

**Ensimmäinen paikallinen litterointi kestää kauan.** Jokainen malli valmistellaan Macillesi ensimmäisellä käyttökerralla; sen jälkeen se latautuu sekunnin murto-osassa.

**Lowspeak sulkeutuu heti käynnistyksen jälkeen.** Toinen Lowspeak-kopio on jo käynnissä; lopeta se ensin.

**Ilmoituksia ei näy.** Salli ne kohdassa **Järjestelmäasetukset › Ilmoitukset › Lowspeak**.

**Jokin muu.** Kansion `~/Library/Application Support/Lowspeak/` lokit (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) kertovat, mitä tapahtui, ilman sanelemaasi tekstiä. Liitä olennaiset rivit [ongelmailmoitukseen](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Kehittäjille

Rakentaminen, testaus, koodin rakenne ja sen noudattamat säännöt on kuvattu tiedostossa [docs/development.md](../../../docs/development.md) (englanniksi), osallistuminen tiedostossa [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Kolmannen osapuolen komponentit

| Komponentti | Versio | Käyttö | Lisenssi |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) via [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | paikallinen Whisper grafiikkasuorittimella (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | Parakeetin dekoodaus ja pitkien äänitysten yhdistäminen, käännetty Rustiksi kansioon `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) ja sen kehyspaketit | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation ja muut järjestelmäkehykset | MIT; Zlib, Apache-2.0 tai MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | mikrofonin äänitys CoreAudion kautta | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | uudelleennäytteistys 16 kHz:iin | MIT |
| [ureq](https://github.com/algesten/ureq) ja rustls | 3.5 | mallien lataukset | MIT tai Apache-2.0; rustls ja ring lisensseillä ISC ja Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | mallien latausten tarkistussummat | MIT tai Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | API-avaimet avainnipussa | MIT tai Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, virheet, Parakeetin token-sanasto | MIT tai Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | levykuvasta asennetun apin päivitykset | MIT, osin BSD- ja zlib-lisensseillä |

Lisenssitekstit kaikesta, mitä appi sisältää, toimitetaan sen mukana tiedostona `Contents/Resources/Acknowledgements.txt`.

Mallit eivät ole osa Lowspeakia; se lataa ne, kun pyydät:

| Malli | Lähde | Lisenssi |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (OpenAI:n Whisper ggml-muodossa) | MIT |
| Parakeet v2, Parakeet Ultra | NVIDIAn Parakeet TDT:n CoreML-muunnokset, tekijä [FluidInference](https://huggingface.co/FluidInference) | CC-BY-4.0 |

## Lisenssi

Lowspeakin lisenssi on jompikumpi seuraavista

- Apache-lisenssi, versio 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- MIT-lisenssi ([LICENSE-MIT](../../../LICENSE-MIT))

valintasi mukaan.

Ellet nimenomaisesti toisin ilmoita, jokainen Lowspeakiin sisällytettäväksi tarkoituksella toimittamasi lahjoitus, sellaisena kuin se on määritelty Apache-2.0-lisenssissä, lisensoidaan kaksoislisenssillä kuten yllä ilman lisäehtoja.
