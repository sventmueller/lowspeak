<p><img src="../../readme-icon.png" alt="Ikona aplikacji Lowspeak" width="168" height="168"></p>

# Lowspeak

**Dyktowanie na Macu, które pisze tam, gdzie stoi kursor: przytrzymaj fn, mów, puść.**

Lowspeak mieszka na pasku menu. Przytrzymaj klawisz fn (Globus) i mów; gdy go puścisz, Twoje słowa pojawią się w aplikacji, w której właśnie piszesz, tak jakby wpisano je z klawiatury. Transkrypcję wykonuje online przez OpenAI lub Groq albo w całości na Twoim Macu przez Whisper lub NVIDIA Parakeet, który zamienia 40 sekund mowy w tekst w około jedną trzecią sekundy. Gdy zawiedzie sieć lub usługa online, pracę przejmuje zainstalowany model lokalny, więc żadne dyktowanie nie przepada. Naucz go nazw i terminów, których używasz, pozwól modelowi językowemu usuwać przerywniki, jeśli chcesz, i zablokuj nagrywanie klawiszem Shift, aby dłuższe teksty dyktować bez użycia rąk. Program napisano w Ruście na prawdziwych kontrolkach AppKit, mówi w 35 językach i wysyła Twój głos tylko do wybranego przez Ciebie dostawcy.

[![Pobierz Lowspeak dla macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Pobierz Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: podpisany i poświadczony notarialnie przez Apple, dla macOS 14 lub nowszego na Apple Silicon i Intel.

**Trafił się błąd?** [Otwórz zgłoszenie](https://github.com/sventmueller/lowspeak/issues/new/choose) albo wybierz **Pomoc › Zgłoś problem…** w menu Lowspeaka: formularz otworzy się z już wpisaną wersją. Jeśli Lowspeak jest dla Ciebie przydatny, [gwiazdka na GitHubie](https://github.com/sventmueller/lowspeak) pomoże innym go znaleźć.

[![Rust, edycja 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 lub nowszy](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Uniwersalny: Apple Silicon i Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit przez objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp na Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet na Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Online z OpenAI lub Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 języków interfejsu](https://img.shields.io/badge/languages-35-4C9A2A)
![Podpisany z Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Licencja: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#licencja)

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
**Polski** ·
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

## Spis treści

- [Najważniejsze cechy](#najważniejsze-cechy)
- [Wymagania](#wymagania)
- [Instalacja](#instalacja)
- [Pierwsze kroki](#pierwsze-kroki)
  - [Uprawnienia](#uprawnienia)
  - [Wybór sposobu transkrypcji](#wybór-sposobu-transkrypcji)
  - [Menu](#menu)
  - [Skrócona instrukcja](#skrócona-instrukcja)
- [Dyktowanie](#dyktowanie)
  - [Przytrzymaj i mów](#przytrzymaj-i-mów)
  - [Blokada klawiszem Shift lub podwójnym naciśnięciem](#blokada-klawiszem-shift-lub-podwójnym-naciśnięciem)
  - [Gdzie trafia tekst](#gdzie-trafia-tekst)
  - [Ostatnio podyktowane](#ostatnio-podyktowane)
- [Transkrypcja](#transkrypcja)
  - [Który model wybrać](#który-model-wybrać)
  - [Online: OpenAI i Groq](#online-openai-i-groq)
  - [Na Twoim Macu: Whisper i Parakeet](#na-twoim-macu-whisper-i-parakeet)
  - [Gdy usługa online zawiedzie](#gdy-usługa-online-zawiedzie)
  - [Jak przygotowywane jest nagranie](#jak-przygotowywane-jest-nagranie)
  - [Języki według modelu](#języki-według-modelu)
  - [Komputery Mac z procesorem Intel](#komputery-mac-z-procesorem-intel)
- [Słownictwo](#słownictwo)
- [Przetwarzanie końcowe](#przetwarzanie-końcowe)
- [Mikrofon](#mikrofon)
- [Klucze API](#klucze-api)
- [Powiadomienia](#powiadomienia)
- [Wygląd i dostępność](#wygląd-i-dostępność)
- [Języki](#języki)
- [Klawisze i gesty](#klawisze-i-gesty)
- [Prywatność i bezpieczeństwo](#prywatność-i-bezpieczeństwo)
- [Szybkość](#szybkość)
- [Znane ograniczenia](#znane-ograniczenia)
- [Rozwiązywanie problemów](#rozwiązywanie-problemów)
- [Dla programistów](#dla-programistów)
  - [Komponenty zewnętrzne](#komponenty-zewnętrzne)
- [Licencja](#licencja)

## Najważniejsze cechy

- **Przytrzymaj fn, mów, puść.** Tekst trafia do aplikacji na pierwszym planie, tam gdzie stoi kursor: do poczty, czatu, edytora, terminala lub przeglądarki. Bez przełączania się do innego okna, bez klikania w przycisk.
- **Zablokuj nagrywanie przy dłuższych tekstach.** Naciśnij Shift, trzymając fn, albo naciśnij fn dwa razy, a nagrywanie będzie trwać bez trzymania klawisza, nawet do pięciu minut. Jedno naciśnięcie fn lub Shift je kończy.
- **Właściwy model do zadania.** Groq, gdy liczy się szybkość bez obciążania pamięci Maca, GPT Transcribe od OpenAI dla najlepszego wyniku, Whisper lub NVIDIA Parakeet na Twoim Macu do pracy offline i zachowania prywatności głosu. Przełączasz jednym kliknięciem.
- **Żadne dyktowanie nie przepada.** Gdy zawiedzie sieć lub usługa online, zainstalowany model lokalny przepisuje to samo nagranie na Twoim Macu.
- **Twoje słowa, poprawnie zapisane.** Słownictwo z nazwami i terminami fachowymi podpowiada OpenAI, Groq, Whisperowi i przetwarzaniu końcowemu.
- **Czysty tekst, jeśli chcesz.** Opcjonalne przetwarzanie końcowe usuwa przerywniki, poprawia gramatykę, a gdy się poprawiasz, zostawia tylko poprawkę; może też sformatować tekst pod rodzaj aplikacji, do której dyktujesz.
- **Mów dalej.** Zacznij następne dyktowanie, zanim poprzednie zostanie przepisane; teksty pojawiają się w tej kolejności, w jakiej były wypowiadane.
- **Schowek pozostaje Twój.** Lowspeak wkleja przez schowek i chwilę później przywraca to, co w nim było. Ostatnie dziesięć dyktowań zostaje w menu do ponownego skopiowania.
- **Natywny na wskroś.** Prawdziwa pozycja na pasku menu z menu AppKit, standardowymi alertami i powiadomieniami. Bez Electrona, bez okna, bez ikony w Docku.
- **Prywatny.** Bez konta, bez telemetrii. Dźwięk trafia tylko do wybranego przez Ciebie dostawcy, a w trybie lokalnym nigdzie.
- **W Twoim języku.** Dyktowanie działa w dziesiątkach języków, a menu, okna dialogowe i powiadomienia są dostępne w 35.

## Wymagania

- macOS 14 Sonoma lub nowszy. Lowspeak powstaje i jest testowany na macOS 27.
- Mac z procesorem Apple Silicon lub Intel. Lokalna transkrypcja jest stworzona z myślą o Apple Silicon, gdzie Whisper działa na GPU, a Parakeet na Neural Engine; na komputerach Mac z procesorem Intel oba działają na procesorze, znacznie wolniej, zobacz [Komputery Mac z procesorem Intel](#komputery-mac-z-procesorem-intel).
- Około 17 MB na aplikację oraz od 75 MB do 3 GB na każdy pobrany model lokalny.
- Do transkrypcji online klucz API od [OpenAI](https://platform.openai.com/api-keys) lub [Groq](https://console.groq.com/keys); obaj dostawcy pobierają opłaty za to, co transkrybujesz. Lokalna transkrypcja nie wymaga konta.

## Instalacja

**Z obrazu dysku.** Pobierz [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), otwórz go i przeciągnij Lowspeak do folderu Programy. Obraz jest podpisany certyfikatem Developer ID i poświadczony notarialnie przez Apple, więc otwiera się bez ostrzeżeń.

**Przez Homebrew.** Cask znajduje się we własnym tapie:

```sh
brew install --cask sventmueller/tap/lowspeak
```

`brew uninstall --zap --cask lowspeak` usuwa też ustawienia, modele, dzienniki i pamięć podręczną Lowspeaka; Twoje klucze API pozostają w pęku kluczy logowania.

**Uaktualnienia.** Lowspeak z obrazu dysku lub z Homebrew sam dba o aktualność dzięki [Sparkle](https://sparkle-project.org). Przy drugim uruchomieniu pyta, czy może raz dziennie samodzielnie szukać uaktualnień. **Sprawdź uaktualnienia…** w menu sprawdza od razu; **Sprawdzaj uaktualnienia automatycznie** i **Pobieraj i instaluj uaktualnienia automatycznie** tuż pod nią pozwalają zmienić decyzję później. Gdy sprawdzanie znajdzie nową wersję, kiedy pracujesz w innej aplikacji, na pierścieniu na pasku menu pojawia się kropka, a **Sprawdź uaktualnienia…** przenosi uaktualnienie na pierwszy plan. Lowspeak instaluje uaktualnienie tylko wtedy, gdy ma ono podpis autora; Twoje ustawienia, klucze i modele pozostaną. `brew upgrade --cask lowspeak` uaktualnia też kopię z Homebrew. Wersja 1.0.0 nie ma mechanizmu uaktualnień: raz zainstaluj nowszą wersję w jej miejsce ze [strony wydań](https://github.com/sventmueller/lowspeak/releases). Własna kompilacja również nie ma mechanizmu uaktualnień.

**Ze źródeł.** Wymagania wstępne znajdziesz w części [Kompilacja i wdrażanie](../../../docs/development.md#build--deploy), potem:

```sh
make deploy
```

To polecenie kompiluje Lowspeak, zastępuje kopię w folderze Programy i uruchamia ją.

**Otwieraj podczas logowania** w menu uruchamia Lowspeak razem z Makiem, za pomocą rzeczy otwieranych podczas logowania w samym macOS. Naraz działa tylko jedna kopia; druga od razu się zamyka.

## Pierwsze kroki

### Uprawnienia

Przy pierwszym uruchomieniu Lowspeak prosi o trzy uprawnienia:

- **Dostępność** (Ustawienia systemowe › Prywatność i ochrona › Dostępność; w macOS 27 lista nazywa się Sterowanie urządzeniami i dostęp do danych). Lowspeak potrzebuje tego uprawnienia, aby wykrywać klawisz fn i wklejać za pomocą ⌘V. macOS proponuje otwarcie listy; włącz przełącznik przy Lowspeaku. Komunikaty Lowspeaka podają nazwę listy taką, jaką pokazuje Twój macOS.
- **Mikrofon.** Pytanie pojawia się raz; bez tego uprawnienia Lowspeak niczego nie nagrywa.
- **Powiadomienia.** Lowspeak zgłasza nimi problemy, które inaczej łatwo przeoczyć, takie jak cichy mikrofon czy nieudana transkrypcja.

### Wybór sposobu transkrypcji

Skonfiguruj co najmniej jeden sposób; w menu możesz w każdej chwili przełączać się między nimi.

- **Online:** wybierz **Ustaw klucz API…** i wklej klucz od OpenAI (zaczyna się od `sk-`) lub Groq (`gsk_`). Lowspeak rozpoznaje dostawcę po kluczu. Potem wybierz model w **OpenAI** lub **Groq**.
- **Na Twoim Macu:** wybierz **Zarządzaj modelami** › **Pobierz Parakeet Ultra** lub model Whisper, a potem wskaż go w **Lokalnie**.

W wyborze pomoże część [Który model wybrać](#który-model-wybrać). Potem przytrzymaj fn, powiedz coś i puść.

### Menu

Kliknij pierścień na pasku menu:

| Pozycja | Co robi |
|---|---|
| **Ostatnio podyktowane** | ostatnie dziesięć tekstów; kliknięcie ponownie kopiuje wybrany tekst do schowka |
| **Lokalnie** | modele na Twoim Macu, pogrupowane na Whisper i Parakeet; te niepobrane są wyszarzone |
| **OpenAI**, **Groq** | modele online; bez klucza są wyszarzone, a menu proponuje **Ustaw klucz API…** |
| **Język** | język, w którym dyktujesz, lub **Wykrywaj automatycznie**; menu podąża za tym wyborem |
| **Przetwarzanie końcowe** | **Wyłączone**, **Oczyszczanie** lub **Formatowanie**, zobacz [Przetwarzanie końcowe](#przetwarzanie-końcowe) |
| **Słownictwo…** | nazwy i terminy do zapisywania dokładnie w podanej formie; menu pokazuje, ile ich wpisano |
| **Mikrofon** | **Domyślny systemowy** lub wybrany na stałe mikrofon oraz **Dźwięk gotowości** |
| **Zarządzaj modelami** | pobieranie i usuwanie modeli lokalnych, z widocznym postępem |
| **Ustaw klucz API…** / **Zmień klucz API…** | dodawanie, zastępowanie lub usuwanie kluczy |
| **Otwieraj podczas logowania** | uruchamianie Lowspeaka razem z Makiem; znacznik pokazuje, że opcja jest włączona |
| **Sprawdź uaktualnienia…** | sprawdzenie od razu, czy jest nowa wersja; dwie pozycje pod nią decydują, czy Lowspeak sprawdza raz dziennie i instaluje uaktualnienia bez pytania. Tylko w aplikacji z obrazu dysku |
| **Pomoc** | **Skrócona instrukcja**, **Zgłoś problem…**, **Zaproponuj funkcję…**, **Lowspeak w serwisie GitHub** |
| **Lowspeak – informacje** | wersja, prawa autorskie, strona projektu i licencja |
| **Zakończ Lowspeak** | ⌘Q, gdy menu jest otwarte |

Znacznik w **Lokalnie**, **OpenAI** i **Groq** wskazuje używany model. Ikona pokazuje, co robi Lowspeak:

| Ikona | Znaczenie |
|---|---|
| Pierścień z kropką | Gotowy |
| Wypełnione koło | Nagrywanie, dopóki fn jest przytrzymany |
| Zaokrąglony kwadrat | Nagrywanie zablokowane; następne naciśnięcie fn lub Shift je kończy |
| Migający pierścień | Transkrypcja |
| Pierścień z kropką w prawym górnym rogu | Czeka nowa wersja, zobacz **Sprawdź uaktualnienia…** |
| ↓ 42 % | Pobieranie modelu |

Lowspeak nie ma okna ustawień: wszystko jest w tym menu, a każdy wybór zostaje zapamiętany.

### Skrócona instrukcja

**Pomoc › Skrócona instrukcja** pokazuje gesty klawisza fn w małym oknie, w Twoim języku, z przyciskiem otwierającym ustawienia klawiatury. **Zgłoś problem…** otwiera formularz w serwisie GitHub z już wpisanymi wersjami Lowspeaka i macOS, **Zaproponuj funkcję…** dyskusję na pomysły, a **Lowspeak w serwisie GitHub** stronę projektu.

## Dyktowanie

### Przytrzymaj i mów

Przytrzymaj fn, mów, puść. Ikona nagrywania pojawia się w chwili, gdy mikrofon zaczyna dostarczać dźwięk; wtedy zacznij mówić, bo to, co powiesz wcześniej, nie zostanie nagrane. Puszczenie fn kończy nagrywanie, a tekst pojawia się chwilę później.

Krótkie stuknięcie fn nic nie robi, więc fn zachowuje swoje inne zastosowania. Nagranie, które pozostaje puste lub trwa krócej niż 0,3 sekundy, jest odrzucane.

### Blokada klawiszem Shift lub podwójnym naciśnięciem

Przy dłuższych tekstach zablokuj nagrywanie, aby trwało bez trzymania klawisza. Są na to trzy sposoby:

- **Shift podczas trzymania fn.** Zacznij jak zwykle od fn i naciśnij Shift w trakcie nagrywania: nagrywanie się blokuje i możesz puścić fn. Nic z tego, co zostało już powiedziane, nie przepada, nagrywanie po prostu trwa dalej.
- **Shift, potem fn.** Przytrzymaj Shift i naciśnij fn: nagrywanie od razu startuje zablokowane.
- **Dwa razy fn.** Stuknij krótko fn (krócej niż 0,3 sekundy) i naciśnij go ponownie w ciągu 0,4 sekundy. Nagranie obejmuje czas od pierwszego naciśnięcia.

Ikona zmienia się w zaokrąglony kwadrat. Aby zakończyć, naciśnij raz **fn** lub **Shift**; tekst pojawi się jak zwykle. Jeśli klawisz 🌐 otwiera Emoji i symbole, kończ klawiszem Shift: krótkie naciśnięcie fn otworzyłoby przeglądarkę znaków.

Nagrywanie kończy się samo po dwóch minutach, gdy fn jest przytrzymany, i po pięciu minutach, gdy jest zablokowane, a potem jest przepisywane jak zwykle.

### Gdzie trafia tekst

Lowspeak wkleja tekst do aplikacji, która była na pierwszym planie w chwili zakończenia mówienia, w miejscu kursora, przez schowek i ⌘V. Zachowuje wszystko, co wcześniej było w schowku, i przywraca to pół sekundy po tym, jak aplikacja odczyta tekst, albo po dziesięciu sekundach, jeśli nigdy tego nie zrobi, chyba że w międzyczasie inna aplikacja zmieniła zawartość schowka. Tekst jest oznaczony jako tymczasowy, więc aplikacje z historią schowka, które respektują to oznaczenie, go nie rejestrują.

Następne dyktowanie możesz zacząć, zanim poprzednie zostanie przepisane. Teksty są wklejane w kolejności, w jakiej były wypowiadane, i nigdy podczas nagrywania ani trzymania klawisza Shift, więc wklejony tekst nie może trafić w środek Twojego następnego zdania.

Bez uprawnienia do funkcji dostępności albo gdy nie da się wysłać symulowanego ⌘V, tekst zostaje w schowku, a powiadomienie o tym informuje; wklej go samodzielnie za pomocą ⌘V.

### Ostatnio podyktowane

**Ostatnio podyktowane** w menu pokazuje ostatnie dziesięć tekstów, od najnowszego, każdy skrócony do jednego wiersza; po wskazaniu kursorem widać cały tekst. Kliknięcie kopiuje go do schowka, co przydaje się, gdy tekst trafił w złe miejsce albo potrzebujesz go dwa razy. Lista istnieje tylko w pamięci i jest pusta po zakończeniu Lowspeaka.

## Transkrypcja

### Który model wybrać

| Czego potrzebujesz | Wybierz | Dlaczego |
|---|---|---|
| **Szybkości bez obciążania Maca** | **Groq › Whisper Large V3 Turbo** | najszybsza opcja online; nic nie jest pobierane, a Mac zachowuje swoją pamięć i miejsce na dysku |
| **Najlepszego wyniku** | **OpenAI › GPT Transcribe** (domyślny) | najdokładniejszy z modeli oferowanych przez Lowspeak; przyjmuje Twoje słownictwo jako słowa kluczowe |
| **Pracy offline i prywatności** | **Lokalnie › Parakeet Ultra** dla języków europejskich, **Lokalnie › Whisper Large v3 Turbo** dla wszystkich innych | działa bez sieci, a Twój głos nigdy nie opuszcza Maca; Parakeet jest najszybszy ze wszystkich |
| **Bez konta, bez kosztów** | dowolny model w **Lokalnie** | działa na Twoim Macu; nic nie jest naliczane |
| **Niezawodności i szybkości naraz** | model online plus zainstalowany Parakeet Ultra lub model Whisper | model lokalny wkracza, gdy usługa online zawiedzie |
| **Transkrypcji na Macu z procesorem Intel** | **Groq** lub **OpenAI** | modele lokalne działają tam na procesorze, zobacz [Komputery Mac z procesorem Intel](#komputery-mac-z-procesorem-intel) |

Modele lokalne zajmują na dysku tyle, ile wynosi ich rozmiar do pobrania, a gdy są wybrane, mniej więcej tyle samo pamięci; Lowspeak trzyma model załadowany, aby dyktowanie zaczynało się bez opóźnienia. Na Macu z 8 GB pamięci wybieraj raczej Parakeet Ultra lub model online niż duże modele Whisper.

### Online: OpenAI i Groq

| Model | Dostawca | Uwagi |
|---|---|---|
| **GPT Transcribe** (domyślny) | OpenAI | najdokładniejszy; przyjmuje Twoje słownictwo jako słowa kluczowe |
| **Whisper** | OpenAI | oryginalny model Whisper |
| **Whisper Large V3** | Groq | bardzo szybki |
| **Whisper Large V3 Turbo** | Groq | najszybszy online |

Przed wysłaniem nagranie jest kompresowane do AAC, około 6 KB na sekundę mowy, więc dyktowanie działa także przy wolnych połączeniach. Gdy trzymasz fn, Lowspeak już otwiera połączenie z dostawcą, więc wysyłanie zaczyna się w chwili, gdy puścisz klawisz.

### Na Twoim Macu: Whisper i Parakeet

| Model | Rozmiar | Działa na | Języki |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 języków europejskich |
| **Parakeet v2** | ~600 MB | Neural Engine | angielski |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | około 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | około 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | około 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | około 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | około 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | około 99 |

**Zarządzaj modelami** pobiera je i usuwa; podczas pobierania pasek menu pokazuje postęp, a zakończenie Lowspeaka przerywa pobieranie. Przy pierwszym użyciu model jest przygotowywany dla Twojego Maca: około 10 sekund w przypadku Parakeet Ultra, do 30 sekund w przypadku Whispera; potem ładuje się w ułamku sekundy. Jeśli wybrany model zostanie usunięty, Lowspeak przełącza się na inny zainstalowany.

**Parakeet** to najszybszy wybór: przepisuje 40 sekund mowy w około 0,3 sekundy. Lowspeak uruchamia go z tymi samymi modelami i tym samym dekodowaniem co [FluidAudio](https://github.com/FluidInference/FluidAudio) i przechowuje modele w folderze FluidAudio (`~/Library/Application Support/FluidAudio/Models`), więc aplikacje oparte na FluidAudio z nich współkorzystają. Nagrania dłuższe niż 15 sekund są przepisywane w nakładających się oknach, które są łączone bez gubienia słów na stykach. **Whisper** rozumie więcej języków i czyta Twoje słownictwo; duże modele są najdokładniejsze, a małe znacznie słabsze poza angielskim. Modele Whisper są przechowywane w `~/Library/Application Support/Lowspeak/Models`.

### Gdy usługa online zawiedzie

Jeśli transkrypcja online się nie powiedzie, bo sieć nie działa, dostawca ma problem lub klucz jest błędny, to samo nagranie przepisuje zainstalowany model lokalny: ten wybrany w **Lokalnie**, a w przeciwnym razie Parakeet Ultra, potem Whisper Large v3 Turbo, Large v3 i mniejsze modele Whisper, a na końcu Parakeet v2. Powiadomienie informuje o tym raz; pojawi się ponownie dopiero wtedy, gdy w międzyczasie uda się dyktowanie online.

Mając model lokalny w odwodzie, Lowspeak daje żądaniu online 20 sekund; bez niego próbuje trzy razy, za każdym razem do minuty. W trybie online Lowspeak raz, przy uruchomieniu, przygotowuje zapasowy model Parakeet, aby pierwsze przejęcie nie musiało na niego czekać.

### Jak przygotowywane jest nagranie

Lowspeak nagrywa z natywną częstotliwością próbkowania mikrofonu, a po zakończeniu konwertuje nagranie do 16 kHz, których oczekują modele. Przed transkrypcją usuwa niskie dudnienie poniżej 80 Hz, przycina ciszę na początku i na końcu oraz wyrównuje głośność, więc osoby mówiące cicho i głośno wypadają podobnie. Nagranie bez mowy w ogóle nie jest wysyłane. W trybie lokalnym Parakeet najpierw dostaje nietknięte nagranie, a przygotowane tylko wtedy, gdy z tamtego nie powstanie żaden tekst.

### Języki według modelu

Język wybrany w **Język** jest przekazywany modelom online i Whisperowi. Przy **Wykrywaj automatycznie** model sam ustala język.

| Model | Języki, w których możesz dyktować |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, na Twoim Macu) | około 99, w tym wszystkie 35 z menu |
| Parakeet Ultra | bułgarski, chorwacki, czeski, duński, holenderski, angielski, estoński, fiński, francuski, niemiecki, grecki, węgierski, włoski, łotewski, litewski, maltański, polski, portugalski, rumuński, rosyjski, słowacki, słoweński, hiszpański, szwedzki, ukraiński |
| Parakeet v2 | angielski |

Parakeet sam rozpoznaje język i ignoruje wybór w menu. **Chiński (tradycyjny)** sprawia, że GPT Transcribe pisze znakami tradycyjnymi; Whisper zapisuje chiński dowolnym z dwóch pism, a portugalski bez względu na wariant.

### Komputery Mac z procesorem Intel

Lowspeak jest aplikacją uniwersalną i działa także na komputerach Mac z procesorem Intel z macOS 14. Wszystko, co online, działa tak samo. Modele lokalne nie mają tam sprzętu, dla którego zostały stworzone: Whisper działa na procesorze zamiast na GPU, a Parakeet na procesorze zamiast na Neural Engine. Oba działają, ale trwa to wielokrotnie dłużej niż na Apple Silicon, w przypadku dużych modeli Whisper zdecydowanie za długo do dyktowania. Na Macu z procesorem Intel korzystaj z transkrypcji online, a Whisper Small lub Parakeet Ultra trzymaj jako [model zapasowy](#gdy-usługa-online-zawiedzie).

## Słownictwo

**Słownictwo…** przyjmuje nazwy i terminy, które Lowspeak ma zapisywać dokładnie w podanej formie, rozdzielone przecinkami: nazwy produktów, nazwiska współpracowników, terminy fachowe. Do 100 terminów, każdy do 60 znaków; `<` i `>` są usuwane.

- **GPT Transcribe** otrzymuje je jako słowa kluczowe.
- **Whisper** (OpenAI, Groq i na Twoim Macu) czyta je jako podpowiedź przed nagraniem, do 500 znaków.
- **Przetwarzanie końcowe** dostaje polecenie, aby zapisywać je dokładnie tak, jak je wpisano.
- **Parakeet** z nich nie korzysta.

## Przetwarzanie końcowe

Po transkrypcji model językowy może przeredagować tekst. Wybierz w **Przetwarzanie końcowe**:

- **Wyłączone** (domyślnie) – tekst w takiej postaci, w jakiej został przepisany.
- **Oczyszczanie** – usuwa przerywniki, takie jak „yyy”, „no wiesz” czy „w zasadzie”, poprawia gramatykę i interpunkcję, a gdy się poprawiasz, zostawia tylko poprawkę. Niczego nie dodaje i nie zmienia sensu.
- **Formatowanie** – jak Oczyszczanie, a do tego dopasowuje tekst do aplikacji, do której dyktujesz: terminy w stylu kodu w edytorach kodu, wiersze poleceń w terminalach, swobodny ton w czatach, profesjonalny w poczcie e-mail.

Przetwarzanie końcowe korzysta z Groq (`openai/gpt-oss-120b`), jeśli masz klucz Groq, a w przeciwnym razie z OpenAI (`gpt-4o-mini`), także wtedy, gdy transkrypcja odbywa się lokalnie. Gdy w **Język** wybrano język, zachowuje ten język. Jeśli trwa dłużej niż osiem sekund, nie powiedzie się albo zwróci odpowiedź uciętą lub znacznie dłuższą niż to, co powiedziano, Lowspeak wkleja nieprzetworzony tekst. Jeśli dostawca odrzuci żądanie lub nie ustawiono klucza, powiadomienie poinformuje o tym raz.

## Mikrofon

W **Mikrofon** wybierz konkretny mikrofon lub **Domyślny systemowy**. Lowspeak zapamiętuje urządzenie; gdy nie jest podłączone, menu pokazuje je z dopiskiem **(niepodłączony)**, a Lowspeak nagrywa z domyślnego mikrofonu systemowego.

Mikrofon jest włączany tylko na czas dyktowania. Jego przygotowanie trwa około 0,07 sekundy w przypadku wbudowanego mikrofonu, około pół sekundy w przypadku Studio Display i od jednej do dwóch sekund w przypadku AirPods, które najpierw przełączają się w tryb zestawu słuchawkowego. Ikona nagrywania pojawia się, gdy dociera dźwięk; **Dźwięk gotowości** dodaje w tym momencie krótki sygnał. Domyślnie jest wyłączony, bo na niektórych głośnikach sam sygnał rozbrzmiewa z półsekundowym opóźnieniem.

Jeśli mikrofon przez pięć sekund dostarcza tylko ciszę, jak AirPods, gdy ich mikrofon zajmuje inne urządzenie, Lowspeak Cię o tym powiadomi.

## Klucze API

**Ustaw klucz API…** prosi o klucz i rozpoznaje dostawcę po jego początku: `sk-` dla OpenAI, `gsk_` dla Groq. Klucz w innym formacie jest odrzucany z wyjaśnieniem. Wprowadź jeden klucz na dostawcę; z kluczami obu dostawców dostępne są wszystkie modele online. **Zmień klucz API…** zastępuje klucz albo, za pomocą **Usuń wszystkie klucze**, usuwa klucze.

Klucze są przechowywane w Twoim pęku kluczy logowania pod nazwą `com.lowspeak.app`. Plik kluczy z wcześniejszej wersji (`~/Library/Application Support/Lowspeak/.api-key`) zostaje tam przeniesiony przy następnym uruchomieniu i usunięty, gdy każdy klucz da się z powrotem odczytać z pęku kluczy.

## Powiadomienia

Lowspeak działa w tle, więc zgłasza problemy w postaci powiadomień:

| Powiadomienie | Co się stało |
|---|---|
| **Nagrywanie nie powiodło się** | brak dostępu do mikrofonu, brak podłączonego mikrofonu lub mikrofonu nie udało się uruchomić |
| **Brak sygnału z mikrofonu** | pięć sekund ciszy, zobacz [Mikrofon](#mikrofon) |
| **Wymagane uprawnienie do funkcji dostępności** | tekst jest w schowku; nadaj uprawnienie Dostępność, aby wklejać automatycznie |
| **Wklejanie nie powiodło się** | tekst jest w schowku; wklej go za pomocą ⌘V |
| **Transkrypcja nie powiodła się** | z podaniem przyczyny: brak klucza, odrzucony klucz, przekroczony limit zapytań, błąd serwera lub sieci albo brak modelu lokalnego |
| **Transkrypcja wykonana na tym Macu** | usługa online zawiodła i pracę przejął model lokalny |
| **Przetwarzanie końcowe nie powiodło się** | dostawca odrzuca przetwarzanie końcowe lub nie ustawiono klucza; tekst wklejono bez niego |
| **Brak klucza API** | wybrano model online bez klucza dla jego dostawcy |
| **Pobieranie nie powiodło się**, **Usuwanie nie powiodło się**, **Nie można usunąć** | zarządzanie modelem lokalnym się nie udało albo trwała jeszcze inna operacja |
| **Błąd uruchamiania podczas logowania**, **Błąd klucza API** | macOS lub pęk kluczy odrzucił zmianę |

Powiadomienia nigdy nie zawierają podyktowanego tekstu. Niedziałająca usługa online i dostawca przetwarzania końcowego, który odrzuca żądania, są zgłaszani raz, a nie po każdym dyktowaniu, dopóki znów nie zaczną działać.

## Wygląd i dostępność

- **Ikona na pasku menu** to obraz szablonowy, który dostosowuje się do jasnego, ciemnego lub zabarwionego paska menu.
- **VoiceOver** odczytuje ikonę jako „Lowspeak”, a menu, alerty i powiadomienia to standardowe elementy sterujące macOS.
- **Nie trzeba trzymać żadnego klawisza.** Zablokuj nagrywanie klawiszem Shift lub podwójnym naciśnięciem fn, a następne naciśnięcie je zakończy.
- **Skrócona instrukcja** to zwykłe okno z etykietami i przyciskiem, dostępne z klawiatury; Esc je zamyka.

Znane braki wymienia [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver nie ogłasza rozpoczęcia nagrywania, a Lowspeak nie został jeszcze sprawdzony z VoiceOver ani ze Sterowaniem przełącznikami.

## Języki

Menu, okna dialogowe i powiadomienia są przetłumaczone na 35 języków:

arabski, kataloński, chiński (uproszczony), chiński (tradycyjny), chorwacki, czeski, duński, holenderski, angielski, fiński, francuski, niemiecki, grecki, hebrajski, hindi, węgierski, indonezyjski, włoski, japoński, koreański, malajski, norweski, polski, portugalski (Brazylia), portugalski (Portugalia), rumuński, rosyjski, słowacki, słoweński, hiszpański, szwedzki, tajski, turecki, ukraiński i wietnamski.

Język wybrany w **Język** jest też językiem menu. Przy **Wykrywaj automatycznie** menu podąża za kolejnością języków w **Ustawienia systemowe › Ogólne › Język i region**, a tam, gdzie nie ma tłumaczenia, używa angielskiego. Terminy w menu odpowiadają własnym tłumaczeniom Apple, więc Lowspeak brzmi jak reszta Twojego Maca. Poza angielskim tłumaczenia są maszynowe i nie zostały jeszcze sprawdzone przez rodzimych użytkowników języka; poprawki są mile widziane jako [zgłoszenia](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Klawisze i gesty

| Czynność | Klawisze |
|---|---|
| Dyktowanie, dopóki klawisz jest przytrzymany | przytrzymaj fn |
| Zablokowanie trwającego nagrywania | Shift podczas trzymania fn |
| Rozpoczęcie zablokowanego nagrywania | Shift+fn albo szybko dwa razy fn |
| Zakończenie zablokowanego nagrywania | fn lub Shift |
| Zakończenie Lowspeaka (menu otwarte) | ⌘Q |
| Zamknięcie skróconej instrukcji | Esc |

Lowspeak nie przepuszcza do innych aplikacji naciśnięć fn, których sam używa; Shift działa jak zwykle.

## Prywatność i bezpieczeństwo

Lowspeak słyszy to, co mówisz, gdy trzymasz fn, może pisać w dowolnej aplikacji i przechowuje Twoje klucze API, więc robi z tym wszystkim tak mało, jak to możliwe.

- **Twój głos trafia w jedno miejsce.** W trybie online skompresowane nagranie trafia do wybranego przez Ciebie dostawcy, OpenAI lub Groq, razem z Twoim słownictwem. W trybie lokalnym nic nie opuszcza Twojego Maca. Gdy żądanie online się nie powiedzie, model lokalny przepisuje to samo nagranie na Twoim Macu.
- **Przetwarzanie końcowe**, jeśli je włączysz, wysyła przepisany tekst i Twoje słownictwo do Groq lub OpenAI. W trybie Formatowanie dodaje rodzaj aplikacji, do której dyktujesz (edytor kodu, terminal, czat lub e-mail); nazwa aplikacji nie jest wysyłana.
- **Bez telemetrii, bez konta.** Poza transkrypcją i przetwarzaniem końcowym Lowspeak łączy się z siecią tylko po to, aby otworzyć połączenie z Twoim dostawcą, gdy trzymasz fn, w trybie online lub z włączonym przetwarzaniem końcowym, aby pobrać model, gdy o to poprosisz, z Hugging Face przez HTTPS, w ustalonej wersji i ze sprawdzeniem sumy kontrolnej każdego pliku, oraz aby raz dziennie, jeśli na to pozwolisz, sprawdzić w serwisie GitHub, czy jest nowa wersja. To żądanie zawiera wersję Lowspeaka i nic poza tym. **Pomoc › Zgłoś problem…** otwiera w przeglądarce formularz w serwisie GitHub; jego adres zawiera wersję Lowspeaka i wersję macOS, nic z tego, co podyktowano.
- **Klucze API** pozostają w Twoim pęku kluczy logowania. Są wysyłane wyłącznie do swojego dostawcy i usuwane z komunikatów o błędach, zanim cokolwiek trafi do dziennika.
- **Podyktowany tekst nie jest przechowywany.** Ostatnie dziesięć dyktowań zostaje w pamięci na potrzeby **Ostatnio podyktowane** i znika po zakończeniu Lowspeaka. Pliki dziennika zapisują długości i czasy, nigdy tekst.
- **Schowek** jest zachowywany przed wklejeniem i przywracany, gdy aplikacja odczyta tekst, chyba że w międzyczasie zmieniła go inna aplikacja. Podyktowany tekst jest oznaczony jako tymczasowy, więc aplikacje z historią schowka, które respektują to oznaczenie, go pomijają.
- **Podpisanie kodu** z Hardened Runtime; aplikacja prosi tylko o dostęp do mikrofonu, Dostępności, powiadomień i sieci. Uaktualnienie musi mieć podpis autora, inaczej Sparkle je odrzuca.

Aby zgłosić lukę w zabezpieczeniach, postępuj zgodnie z [zasadami bezpieczeństwa](../../../SECURITY.md).

## Szybkość

Zmierzone na Apple Silicon:

- **Parakeet Ultra** przepisuje 40 sekund mowy w około 0,3 sekundy.
- **Online:** pierwsze żądanie po przerwie trwa w Groq około 0,4 sekundy, bo Lowspeak otwiera połączenie, gdy jeszcze mówisz; kolejne żądania używają go ponownie, przez HTTP/3 tam, gdzie dostawca to oferuje.
- **Rozmiar wysyłanych danych:** około 6 KB na sekundę mowy, jako AAC z przepływnością 48 kbit/s.
- **Przygotowanie mikrofonu:** około 0,07 sekundy w przypadku wbudowanego, pół sekundy w Studio Display, od jednej do dwóch sekund w AirPods.

## Znane ograniczenia

- **Modele lokalne na komputerach Mac z procesorem Intel** działają na procesorze i są znacznie wolniejsze; lepszym wyborem jest tam transkrypcja online.
- **To, co powiesz, zanim mikrofon będzie gotowy, przepada.** Poczekaj na ikonę nagrywania, zwłaszcza z AirPods.
- **Klawisz fn mogą przejąć inni.** Gdy ustawienie klawisza 🌐 w macOS, Warp lub Dyktowanie w macOS przechwycą fn wcześniej, Lowspeak go nie widzi; zobacz [Rozwiązywanie problemów](#rozwiązywanie-problemów).
- **Parakeet** ignoruje słownictwo i wybór języka oraz zna tylko języki europejskie.
- **Pismo i wariant:** tylko modelowi GPT Transcribe można polecić pisanie chińskim pismem tradycyjnym; żadnemu modelowi nie można polecić pisania po portugalsku w odmianie europejskiej zamiast brazylijskiej.
- **Limity czasu:** dwie minuty, gdy fn jest przytrzymany, pięć minut po zablokowaniu.
- **Tryb mikrofonu:** Centrum sterowania oferuje dla Lowspeaka Izolację głosu i Szerokie spektrum i pokazuje je jako aktywne, ale Lowspeak nagrywa bez przetwarzania głosu, więc ten wybór nie ma żadnego efektu.
- **Ostatnio podyktowane** istnieją tylko w pamięci i znikają po zakończeniu aplikacji.
- **Z założenia:** bez okna ustawień, bez ikony w Docku, nie w App Store.

## Rozwiązywanie problemów

**Nic się nie dzieje, gdy przytrzymuję fn.** Sprawdź trzy rzeczy: Lowspeak ma pozwolenie w **Ustawienia systemowe › Prywatność i ochrona › Dostępność** (w macOS 27: **Sterowanie urządzeniami i dostęp do danych**); opcja **Ustawienia systemowe › Klawiatura › Efekt naciśnięcia 🌐** ma wartość **Brak efektu**; i żadna inna aplikacja nie przejmuje fn. Warp robi to domyślnie; wyłącz to poleceniem `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` i uruchom Warp ponownie. Także Dyktowanie w macOS można ustawić tak, by uruchamiało się klawiszem fn.

**Po uaktualnieniu Lowspeak nie reaguje już na fn.** macOS wiąże uprawnienie Dostępność z podpisem aplikacji. Jeśli podpis się zmienił, na przykład przy przejściu z własnej kompilacji na pobraną aplikację, usuń Lowspeak z listy Dostępność (w macOS 27: Sterowanie urządzeniami i dostęp do danych) i dodaj go ponownie.

**Brakuje pierwszych słów.** Zacznij mówić, gdy pojawi się ikona nagrywania. Z AirPods mikrofon potrzebuje od jednej do dwóch sekund; wbudowany mikrofon lub mikrofon USB wybrany w **Mikrofon** jest gotowy niemal od razu.

**„Brak sygnału z mikrofonu”.** Mikrofon przez pięć sekund nie dostarczył dźwięku. W przypadku AirPods ich mikrofonu może używać inne urządzenie; przełącz na wbudowany mikrofon albo połącz je ponownie. Zamknięta pokrywa MacBooka także wycisza wbudowany mikrofon.

**Dyktowania wychodzą puste.** Głośność wejścia może być za niska: aplikacje takie jak Teams, Zoom czy Meet zmieniają ją podczas rozmów, a macOS zapamiętuje ją osobno dla każdego urządzenia. Zwiększ ją w **Ustawienia systemowe › Dźwięk › Wejście**.

**Tekst trafił w złe miejsce.** **Ostatnio podyktowane** w menu ponownie kopiuje go do schowka.

**„Brak klucza API”.** Wybierz w menu **Ustaw klucz API…** lub przełącz się na model lokalny.

**Pierwsza lokalna transkrypcja trwa długo.** Każdy model jest przygotowywany dla Twojego Maca przy pierwszym użyciu; od tej pory ładuje się w ułamku sekundy.

**Lowspeak zamyka się zaraz po uruchomieniu.** Działa już inna kopia Lowspeaka; najpierw ją zakończ.

**Nie pojawiają się żadne powiadomienia.** Zezwól na nie w **Ustawienia systemowe › Powiadomienia › Lowspeak**.

**Coś innego.** Dzienniki w `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) opisują, co się stało, bez podyktowanego tekstu. Dołącz odpowiednie wiersze do [zgłoszenia problemu](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Dla programistów

Kompilacja, testowanie, struktura kodu i zasady, których przestrzega, są opisane w [docs/development.md](../../../docs/development.md), a zasady wnoszenia wkładu w [CONTRIBUTING.md](../../../CONTRIBUTING.md) (po angielsku).

### Komponenty zewnętrzne

| Komponent | Wersja | Do czego | Licencja |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) przez [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | lokalny Whisper na GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | dekodowanie Parakeet i scalanie długich nagrań, przełożone na Rust w `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) i jego crate'y frameworków | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation i inne frameworki systemowe | MIT; Zlib, Apache-2.0 lub MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | przechwytywanie dźwięku z mikrofonu przez CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | przepróbkowanie do 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) z rustls | 3.5 | pobieranie modeli | MIT lub Apache-2.0; rustls i ring na licencjach ISC i Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | sumy kontrolne pobieranych modeli | MIT lub Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | klucze API w pęku kluczy | MIT lub Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, błędy, słownik Parakeet | MIT lub Apache-2.0; Apache-2.0 |
| [Sparkle](https://sparkle-project.org) | 2.10.1 | uaktualnienia aplikacji z obrazu dysku | MIT, z częściami na licencjach BSD i zlib |

Teksty licencji wszystkiego, co zawiera aplikacja, są do niej dołączone jako `Contents/Resources/Acknowledgements.txt`.

Modele nie są częścią Lowspeaka; pobiera je, gdy o to poprosisz:

| Model | Źródło | Licencja |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper od OpenAI w formacie ggml) | MIT |
| Parakeet v2, Parakeet Ultra | [FluidInference](https://huggingface.co/FluidInference) – konwersje CoreML modelu Parakeet TDT od NVIDIA | CC-BY-4.0 |

## Licencja

Lowspeak jest udostępniany na licencji, do wyboru, albo

- Licencja Apache, wersja 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Licencja MIT ([LICENSE-MIT](../../../LICENSE-MIT))

według Twojego wyboru.

O ile wyraźnie nie zaznaczysz inaczej, każdy wkład celowo przekazany do włączenia do Lowspeaka przez Ciebie, w rozumieniu licencji Apache-2.0, będzie objęty podwójną licencją jak wyżej, bez dodatkowych warunków.
