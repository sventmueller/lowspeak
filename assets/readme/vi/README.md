<p><img src="../../readme-icon.png" alt="Biểu tượng ứng dụng Lowspeak" width="168" height="168"></p>

# Lowspeak

**Đọc chính tả cho máy Mac, gõ ngay tại vị trí con trỏ của bạn: giữ fn, nói, thả ra.**

Lowspeak nằm trên thanh menu. Giữ phím fn (địa cầu) và nói; khi bạn thả ra, lời bạn nói xuất hiện trong bất kỳ ứng dụng nào bạn đang gõ, như thể chính bạn đã gõ. Lowspeak chép lại trực tuyến bằng OpenAI hoặc Groq, hoặc hoàn toàn trên máy Mac của bạn bằng Whisper hoặc NVIDIA Parakeet, mô hình biến 40 giây lời nói thành văn bản trong khoảng một phần ba giây. Khi mạng hoặc dịch vụ trực tuyến gặp sự cố, một mô hình cục bộ đã cài sẽ thay thế, nên không bản đọc chính tả nào bị mất. Hãy dạy nó những tên riêng và thuật ngữ bạn dùng, để một mô hình ngôn ngữ loại bỏ từ đệm nếu bạn muốn, và khóa bản ghi âm bằng Shift để đọc chính tả rảnh tay những văn bản dài hơn. Lowspeak được viết bằng Rust trên các điều khiển AppKit thực thụ, có 35 ngôn ngữ và chỉ gửi giọng nói của bạn tới nhà cung cấp bạn đã chọn.

[![Tải Lowspeak cho macOS](https://img.shields.io/badge/Download-Lowspeak_for_macOS-0A84FF?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)

**[Tải về Lowspeak.dmg](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg)**: được Apple ký và công chứng, dành cho macOS 14 trở lên, trên Apple Silicon và Intel.

**Bạn phát hiện lỗi?** Hãy [mở một issue](https://github.com/sventmueller/lowspeak/issues/new/choose), hoặc chọn **Trợ giúp › Báo cáo sự cố…** trong menu của Lowspeak: biểu mẫu sẽ mở ra với phiên bản của bạn đã được điền sẵn. Nếu Lowspeak hữu ích với bạn, một [sao trên GitHub](https://github.com/sventmueller/lowspeak) sẽ giúp người khác tìm thấy nó.

[![Rust phiên bản 2024](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![macOS 14 trở lên](https://img.shields.io/badge/macOS-14%2B-000000?logo=apple&logoColor=white)](https://www.apple.com/macos/)
![Universal: Apple Silicon và Intel](https://img.shields.io/badge/Universal-Apple_Silicon_%7C_Intel-333333?logo=apple&logoColor=white)
[![AppKit qua objc2](https://img.shields.io/badge/AppKit-objc2_0.6-147EFB?logo=apple&logoColor=white)](https://github.com/madsmtm/objc2)
[![whisper.cpp trên Metal](https://img.shields.io/badge/whisper.cpp-Metal-0A84FF)](https://github.com/ggml-org/whisper.cpp)
[![Parakeet trên Neural Engine](https://img.shields.io/badge/Parakeet-Neural_Engine-76B900?logo=nvidia&logoColor=white)](https://huggingface.co/FluidInference)
![Trực tuyến với OpenAI hoặc Groq](https://img.shields.io/badge/online-OpenAI_%7C_Groq-333333)
![35 ngôn ngữ giao diện](https://img.shields.io/badge/languages-35-4C9A2A)
![Được ký với Hardened Runtime](https://img.shields.io/badge/Hardened_Runtime-signed-4C9A2A)
[![Giấy phép: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-4C9A2A)](#giấy-phép)

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
**Tiếng Việt** ·
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

## Nội dung

- [Tính năng nổi bật](#tính-năng-nổi-bật)
- [Yêu cầu](#yêu-cầu)
- [Cài đặt](#cài-đặt)
- [Bắt đầu](#bắt-đầu)
  - [Quyền truy cập](#quyền-truy-cập)
  - [Chọn cách chép lại](#chọn-cách-chép-lại)
  - [Menu](#menu)
  - [Tham khảo nhanh](#tham-khảo-nhanh)
- [Đọc chính tả](#đọc-chính-tả)
  - [Giữ để nói](#giữ-để-nói)
  - [Khóa bằng Shift hoặc nhấn hai lần](#khóa-bằng-shift-hoặc-nhấn-hai-lần)
  - [Văn bản được dán vào đâu](#văn-bản-được-dán-vào-đâu)
  - [Đọc chính tả gần đây](#đọc-chính-tả-gần-đây)
- [Chép lại](#chép-lại)
  - [Mô hình nào phù hợp với bạn](#mô-hình-nào-phù-hợp-với-bạn)
  - [Trực tuyến: OpenAI và Groq](#trực-tuyến-openai-và-groq)
  - [Trên máy Mac của bạn: Whisper và Parakeet](#trên-máy-mac-của-bạn-whisper-và-parakeet)
  - [Khi dịch vụ trực tuyến gặp sự cố](#khi-dịch-vụ-trực-tuyến-gặp-sự-cố)
  - [Cách bản ghi âm được chuẩn bị](#cách-bản-ghi-âm-được-chuẩn-bị)
  - [Ngôn ngữ theo mô hình](#ngôn-ngữ-theo-mô-hình)
  - [Máy Mac Intel](#máy-mac-intel)
- [Từ vựng](#từ-vựng)
- [Hậu xử lý](#hậu-xử-lý)
- [Micrô](#micrô)
- [Khóa API](#khóa-api)
- [Thông báo](#thông-báo)
- [Giao diện và trợ năng](#giao-diện-và-trợ-năng)
- [Ngôn ngữ](#ngôn-ngữ)
- [Phím và thao tác](#phím-và-thao-tác)
- [Quyền riêng tư và bảo mật](#quyền-riêng-tư-và-bảo-mật)
- [Tốc độ](#tốc-độ)
- [Hạn chế đã biết](#hạn-chế-đã-biết)
- [Khắc phục sự cố](#khắc-phục-sự-cố)
- [Dành cho nhà phát triển](#dành-cho-nhà-phát-triển)
  - [Thành phần của bên thứ ba](#thành-phần-của-bên-thứ-ba)
- [Giấy phép](#giấy-phép)

## Tính năng nổi bật

- **Giữ fn, nói, thả ra.** Văn bản được gõ vào ứng dụng đang ở phía trước, ngay tại con trỏ: thư, trò chuyện, trình soạn thảo, terminal hay trình duyệt. Không phải chuyển sang cửa sổ nào, không phải bấm nút nào.
- **Khóa lại cho văn bản dài.** Nhấn Shift trong khi giữ fn, hoặc nhấn fn hai lần, và bản ghi âm tiếp tục chạy mà không cần giữ phím, tối đa năm phút. Nhấn fn hoặc Shift một lần để kết thúc.
- **Mô hình phù hợp cho từng việc.** Groq cho tốc độ mà không tốn bộ nhớ của máy Mac, GPT Transcribe của OpenAI cho kết quả tốt nhất, Whisper hoặc NVIDIA Parakeet trên máy Mac để làm việc ngoại tuyến và giữ giọng nói của bạn riêng tư. Chuyển đổi chỉ bằng một cú nhấp.
- **Không mất bản đọc chính tả nào.** Khi mạng hoặc dịch vụ trực tuyến gặp sự cố, một mô hình cục bộ đã cài sẽ chép lại chính bản ghi âm đó trên máy Mac của bạn.
- **Từ ngữ của bạn, đúng chính tả.** Một bộ từ vựng gồm tên riêng và thuật ngữ chuyên môn định hướng cho OpenAI, Groq, Whisper và hậu xử lý.
- **Văn bản gọn gàng nếu bạn muốn.** Hậu xử lý tùy chọn loại bỏ từ đệm, sửa ngữ pháp và chỉ giữ lại phần đã sửa khi bạn tự đính chính, hoặc định dạng luôn văn bản theo loại ứng dụng bạn đang đọc chính tả vào.
- **Cứ tiếp tục nói.** Bắt đầu lần đọc chính tả tiếp theo trong khi lần trước vẫn đang được chép lại; các văn bản đến theo đúng thứ tự bạn nói.
- **Bảng nhớ tạm vẫn là của bạn.** Lowspeak dán qua bảng nhớ tạm và trả lại nội dung cũ ngay sau đó. Mười bản đọc chính tả gần nhất vẫn nằm trong menu để sao chép lại.
- **Chính gốc từ trong ra ngoài.** Một mục thanh menu thật với menu AppKit, cảnh báo và thông báo tiêu chuẩn. Không Electron, không cửa sổ, không biểu tượng trên Dock.
- **Riêng tư.** Không tài khoản, không telemetry. Âm thanh chỉ được gửi tới nhà cung cấp bạn chọn, hoặc không đi đâu cả ở chế độ cục bộ.
- **Bằng ngôn ngữ của bạn.** Đọc chính tả hoạt động với hàng chục ngôn ngữ, còn menu, hộp thoại và thông báo có 35 ngôn ngữ.

## Yêu cầu

- macOS 14 Sonoma trở lên. Lowspeak được phát triển và thử nghiệm trên macOS 27.
- Máy Mac dùng Apple Silicon hoặc bộ xử lý Intel. Chép lại cục bộ được xây dựng cho Apple Silicon, nơi Whisper chạy trên GPU và Parakeet trên Neural Engine; trên máy Mac Intel, cả hai chạy trên bộ xử lý và chậm hơn nhiều, xem [Máy Mac Intel](#máy-mac-intel).
- Khoảng 10 MB cho ứng dụng, cộng thêm 75 MB đến 3 GB cho mỗi mô hình cục bộ bạn tải về.
- Để chép lại trực tuyến, cần một khóa API từ [OpenAI](https://platform.openai.com/api-keys) hoặc [Groq](https://console.groq.com/keys); cả hai đều tính phí theo lượng bạn chép lại. Chép lại cục bộ không cần tài khoản.

## Cài đặt

**Từ ảnh đĩa.** Tải về [`Lowspeak.dmg`](https://github.com/sventmueller/lowspeak/releases/latest/download/Lowspeak.dmg), mở ra rồi kéo Lowspeak vào thư mục Ứng dụng. Ảnh đĩa được ký bằng Developer ID và được Apple công chứng, nên mở ra mà không có cảnh báo nào.

**Cập nhật.** Lowspeak không tự cập nhật. Các phiên bản mới xuất hiện trên [trang phát hành](https://github.com/sventmueller/lowspeak/releases); **Trợ giúp › Lowspeak trên GitHub** dẫn tới đó. Cài phiên bản mới đè lên bản cũ; cài đặt, khóa và mô hình của bạn vẫn được giữ nguyên.

**Từ mã nguồn.** Xem [Xây dựng & triển khai](../../../docs/development.md#build--deploy) để biết các điều kiện cần có, rồi:

```sh
make deploy
```

Lệnh này build Lowspeak, thay thế bản sao trong thư mục Ứng dụng của bạn và mở nó.

**Mở khi đăng nhập** trong menu khởi động Lowspeak cùng máy Mac, thông qua mục đăng nhập của chính macOS. Mỗi lúc chỉ một bản chạy; bản thứ hai sẽ thoát ngay.

## Bắt đầu

### Quyền truy cập

Ở lần mở đầu tiên, Lowspeak xin ba quyền:

- **Trợ năng** (Cài đặt hệ thống › Quyền riêng tư & Bảo mật › Trợ năng; trên macOS 27, danh sách này có tên Điều khiển thiết bị và truy cập dữ liệu). Lowspeak cần quyền này để nhận biết phím fn và để dán bằng ⌘V. macOS sẽ đề nghị mở danh sách; hãy bật công tắc cho Lowspeak. Các thông báo của chính Lowspeak gọi tên danh sách đúng như macOS của bạn hiển thị.
- **Micrô.** Chỉ hỏi một lần; không có quyền này Lowspeak không ghi được gì.
- **Thông báo.** Lowspeak dùng thông báo cho những sự cố mà bạn sẽ không thấy bằng cách khác, chẳng hạn micrô im lặng hoặc chép lại không thành công.

### Chọn cách chép lại

Thiết lập ít nhất một cách; bạn có thể chuyển đổi bất cứ lúc nào trong menu.

- **Trực tuyến:** chọn **Đặt khóa API…** rồi dán khóa từ OpenAI (bắt đầu bằng `sk-`) hoặc Groq (`gsk_`). Lowspeak nhận ra nhà cung cấp qua khóa. Sau đó chọn một mô hình trong **OpenAI** hoặc **Groq**.
- **Trên máy Mac của bạn:** chọn **Quản lý mô hình** › **Tải về Parakeet Ultra** hoặc một mô hình Whisper, rồi chọn mô hình đó trong **Cục bộ**.

[Mô hình nào phù hợp với bạn](#mô-hình-nào-phù-hợp-với-bạn) giúp bạn lựa chọn. Sau đó giữ fn, nói gì đó rồi thả ra.

### Menu

Bấm vào vòng tròn trên thanh menu:

| Mục | Chức năng |
|---|---|
| **Đọc chính tả gần đây** | mười văn bản gần nhất; bấm vào một mục để sao chép lại vào bảng nhớ tạm |
| **Cục bộ** | các mô hình trên máy Mac, chia thành nhóm Whisper và Parakeet; mô hình chưa tải về sẽ bị làm mờ |
| **OpenAI**, **Groq** | các mô hình trực tuyến; khi chưa có khóa, chúng bị làm mờ và menu đề xuất **Đặt khóa API…** |
| **Ngôn ngữ** | ngôn ngữ bạn dùng để đọc chính tả, hoặc **Tự động phát hiện**; menu cũng theo ngôn ngữ này |
| **Hậu xử lý** | **Tắt**, **Làm gọn** hoặc **Định dạng**, xem [Hậu xử lý](#hậu-xử-lý) |
| **Từ vựng…** | tên riêng và thuật ngữ cần viết chính xác; menu hiện số lượng bạn đã nhập |
| **Micrô** | **Mặc định hệ thống** hoặc một micrô cố định, và **Phát âm thanh khi sẵn sàng** |
| **Quản lý mô hình** | tải về và xóa mô hình cục bộ, có hiển thị tiến độ |
| **Đặt khóa API…** / **Cập nhật khóa API…** | thêm, thay thế hoặc xóa khóa |
| **Mở khi đăng nhập** | khởi động Lowspeak cùng máy Mac; dấu kiểm cho biết tùy chọn đang bật |
| **Trợ giúp** | **Tham khảo nhanh**, **Báo cáo sự cố…**, **Đề xuất tính năng…**, **Lowspeak trên GitHub** |
| **Giới thiệu về Lowspeak** | phiên bản, bản quyền, trang dự án và giấy phép |
| **Thoát Lowspeak** | ⌘Q khi menu đang mở |

Dấu kiểm trong **Cục bộ**, **OpenAI** và **Groq** đánh dấu mô hình đang dùng. Biểu tượng cho biết Lowspeak đang làm gì:

| Biểu tượng | Ý nghĩa |
|---|---|
| Vòng tròn có chấm | Sẵn sàng |
| Hình tròn đặc | Đang ghi âm khi giữ fn |
| Hình vuông bo góc | Bản ghi âm đã khóa; lần nhấn fn hoặc Shift tiếp theo sẽ kết thúc |
| Vòng tròn nhấp nháy | Đang chép lại |
| ↓ 42% | Đang tải về một mô hình |

Lowspeak không có cửa sổ cài đặt: mọi thứ nằm trong menu này, và nó ghi nhớ mọi lựa chọn.

### Tham khảo nhanh

**Trợ giúp › Tham khảo nhanh** hiển thị các thao tác với fn trong một cửa sổ nhỏ, bằng ngôn ngữ của bạn, kèm một nút mở cài đặt bàn phím. **Báo cáo sự cố…** mở một biểu mẫu trên GitHub với phiên bản Lowspeak và macOS của bạn đã được điền sẵn, **Đề xuất tính năng…** mở một cuộc thảo luận cho ý tưởng, còn **Lowspeak trên GitHub** mở trang dự án.

## Đọc chính tả

### Giữ để nói

Giữ fn, nói, thả ra. Biểu tượng ghi âm xuất hiện ngay khi micrô bắt đầu truyền âm thanh; hãy bắt đầu nói lúc đó, vì những gì bạn nói trước đó không được ghi lại. Thả fn sẽ kết thúc ghi âm, và văn bản xuất hiện ngay sau đó.

Một lần nhấn fn ngắn không làm gì cả, nên fn vẫn giữ được các công dụng khác. Bản ghi âm trống hoặc ngắn hơn 0,3 giây sẽ bị bỏ qua.

### Khóa bằng Shift hoặc nhấn hai lần

Với văn bản dài, hãy khóa bản ghi âm để nó chạy mà không cần giữ phím. Có ba cách:

- **Shift trong khi giữ fn.** Bắt đầu với fn như bình thường và nhấn Shift trong lúc ghi âm: bản ghi âm được khóa, và bạn có thể thả fn. Những gì bạn đã nói không bị mất, bản ghi âm chỉ đơn giản tiếp tục.
- **Shift, rồi fn.** Giữ Shift và nhấn fn: bản ghi âm bắt đầu ở trạng thái khóa.
- **fn hai lần.** Nhấn nhanh fn (ngắn hơn 0,3 giây) rồi nhấn lại trong vòng 0,4 giây. Bản ghi âm chạy từ lần nhấn đầu tiên.

Biểu tượng chuyển thành hình vuông bo góc. Nhấn **fn** hoặc **Shift** một lần để dừng; văn bản xuất hiện như thường lệ. Nếu phím 🌐 của bạn được đặt để mở Biểu tượng & Ký hiệu, hãy dừng bằng Shift: một lần nhấn fn ngắn sẽ mở trình xem đó.

Bản ghi âm tự dừng sau hai phút khi giữ fn, và sau năm phút khi đã khóa, rồi được chép lại như thường lệ.

### Văn bản được dán vào đâu

Lowspeak dán văn bản vào ứng dụng đang ở phía trước lúc bạn ngừng nói, tại vị trí con trỏ, qua bảng nhớ tạm và ⌘V. Nó lưu mọi thứ có trong bảng nhớ tạm trước đó và trả lại nửa giây sau khi ứng dụng đã đọc văn bản, hoặc sau mười giây nếu ứng dụng không bao giờ đọc, trừ khi một ứng dụng khác đã thay đổi bảng nhớ tạm trong lúc đó. Văn bản được đánh dấu là tạm thời, nên các ứng dụng lịch sử bảng nhớ tạm tôn trọng dấu này sẽ không ghi lại nó.

Bạn có thể bắt đầu lần đọc chính tả tiếp theo trong khi lần trước vẫn đang được chép lại. Các văn bản được dán theo đúng thứ tự bạn nói, và không bao giờ trong lúc bạn đang ghi âm hay giữ Shift, nên một lần dán không thể rơi vào giữa câu tiếp theo của bạn.

Khi không có quyền Trợ năng, hoặc khi không gửi được ⌘V mô phỏng, văn bản vẫn nằm trong bảng nhớ tạm và một thông báo sẽ cho bạn biết; hãy tự dán bằng ⌘V.

### Đọc chính tả gần đây

**Đọc chính tả gần đây** trong menu liệt kê mười văn bản gần nhất, mới nhất ở trên, mỗi văn bản rút gọn thành một dòng; di con trỏ lên sẽ hiện toàn bộ văn bản. Bấm vào để sao chép vào bảng nhớ tạm, khi một văn bản bị dán nhầm chỗ hoặc bạn cần dùng hai lần. Danh sách chỉ nằm trong bộ nhớ và trống sau khi Lowspeak thoát.

## Chép lại

### Mô hình nào phù hợp với bạn

| Bạn muốn | Hãy chọn | Lý do |
|---|---|---|
| **Tốc độ mà không tốn tài nguyên máy Mac** | **Groq › Whisper Large V3 Turbo** | lựa chọn trực tuyến nhanh nhất; không phải tải gì về, máy Mac giữ nguyên bộ nhớ và dung lượng đĩa |
| **Kết quả tốt nhất** | **OpenAI › GPT Transcribe** (mặc định) | chính xác nhất trong các mô hình Lowspeak cung cấp; nhận từ vựng của bạn làm từ khóa |
| **Ngoại tuyến và riêng tư** | **Cục bộ › Parakeet Ultra** cho các ngôn ngữ châu Âu, **Cục bộ › Whisper Large v3 Turbo** cho mọi ngôn ngữ khác | hoạt động không cần mạng, và giọng nói của bạn không bao giờ rời khỏi máy Mac; Parakeet nhanh nhất trong tất cả |
| **Không tài khoản, không chi phí** | bất kỳ mô hình **Cục bộ** nào | chạy trên máy Mac của bạn; không bị tính phí gì |
| **Vừa ổn định vừa nhanh** | một mô hình trực tuyến, cộng với Parakeet Ultra hoặc một mô hình Whisper đã cài | mô hình cục bộ thay thế khi dịch vụ trực tuyến gặp sự cố |
| **Máy Mac Intel** | **Groq** hoặc **OpenAI** | trên những máy đó, mô hình cục bộ chạy bằng bộ xử lý, xem [Máy Mac Intel](#máy-mac-intel) |

Mô hình cục bộ chiếm dung lượng đĩa bằng kích cỡ tải về và, khi đang được chọn, chiếm lượng bộ nhớ tương đương; Lowspeak giữ mô hình luôn được tải để đọc chính tả bắt đầu ngay, không chậm trễ. Trên máy Mac có 8 GB bộ nhớ, hãy ưu tiên Parakeet Ultra hoặc một mô hình trực tuyến thay vì các mô hình Whisper lớn.

### Trực tuyến: OpenAI và Groq

| Mô hình | Nhà cung cấp | Ghi chú |
|---|---|---|
| **GPT Transcribe** (mặc định) | OpenAI | chính xác nhất; nhận từ vựng của bạn làm từ khóa |
| **Whisper** | OpenAI | mô hình Whisper nguyên bản |
| **Whisper Large V3** | Groq | rất nhanh |
| **Whisper Large V3 Turbo** | Groq | nhanh nhất trong các mô hình trực tuyến |

Bản ghi âm được nén sang AAC trước khi tải lên, khoảng 6 KB cho mỗi giây lời nói, nên đọc chính tả vẫn hoạt động trên kết nối chậm. Trong khi bạn giữ fn, Lowspeak đã mở sẵn kết nối tới nhà cung cấp, nên việc tải lên bắt đầu ngay khi bạn thả phím.

### Trên máy Mac của bạn: Whisper và Parakeet

| Mô hình | Kích cỡ | Chạy trên | Ngôn ngữ |
|---|---|---|---|
| **Parakeet Ultra** | ~630 MB | Neural Engine | 25 ngôn ngữ châu Âu |
| **Parakeet v2** | ~600 MB | Neural Engine | Anh |
| **Whisper Large v3 Turbo** | ~1,6 GB | GPU (Metal) | khoảng 99 |
| **Whisper Large v3** | ~3 GB | GPU (Metal) | khoảng 99 |
| **Whisper Medium** | ~1,5 GB | GPU (Metal) | khoảng 99 |
| **Whisper Small** | ~470 MB | GPU (Metal) | khoảng 99 |
| **Whisper Base** | ~150 MB | GPU (Metal) | khoảng 99 |
| **Whisper Tiny** | ~75 MB | GPU (Metal) | khoảng 99 |

**Quản lý mô hình** tải về và xóa các mô hình; trong lúc tải về, thanh menu hiển thị tiến độ, và thoát Lowspeak sẽ hủy việc tải. Lần dùng đầu tiên của một mô hình sẽ chuẩn bị nó cho máy Mac của bạn: khoảng 10 giây với Parakeet Ultra, tối đa 30 giây với Whisper; sau đó mô hình được tải trong chưa đầy một giây. Nếu mô hình bạn chọn bị xóa, Lowspeak chuyển sang một mô hình khác đã cài.

**Parakeet** là lựa chọn nhanh nhất: nó chép lại 40 giây lời nói trong khoảng 0,3 giây. Lowspeak chạy nó với cùng các mô hình và cách giải mã như [FluidAudio](https://github.com/FluidInference/FluidAudio) và lưu các mô hình trong thư mục của FluidAudio (`~/Library/Application Support/FluidAudio/Models`), nên các ứng dụng xây dựng trên FluidAudio dùng chung được. Bản ghi âm dài hơn 15 giây được chép lại theo các đoạn cửa sổ chồng lên nhau, rồi ghép lại mà không mất từ nào ở chỗ nối. **Whisper** hiểu nhiều ngôn ngữ hơn và đọc từ vựng của bạn; các mô hình lớn chính xác nhất, các mô hình nhỏ kém hơn nhiều với những ngôn ngữ ngoài tiếng Anh. Mô hình Whisper được lưu trong `~/Library/Application Support/Lowspeak/Models`.

### Khi dịch vụ trực tuyến gặp sự cố

Nếu việc chép lại trực tuyến không thành công, do mất mạng, nhà cung cấp gặp sự cố hoặc khóa bị sai, một mô hình cục bộ đã cài sẽ chép lại chính bản ghi âm đó: mô hình được chọn trong **Cục bộ**, nếu không thì Parakeet Ultra, tiếp đến Whisper Large v3 Turbo, Large v3 và các mô hình Whisper nhỏ hơn, cuối cùng là Parakeet v2. Một thông báo cho bạn biết một lần; nó chỉ xuất hiện lại sau khi có một lần đọc chính tả trực tuyến thành công ở giữa.

Khi có mô hình cục bộ để dự phòng, Lowspeak cho yêu cầu trực tuyến 20 giây; nếu không có, nó thử ba lần, mỗi lần tối đa một phút. Ở chế độ trực tuyến, Lowspeak chuẩn bị sẵn mô hình Parakeet dự phòng một lần khi khởi động, nên lần dự phòng đầu tiên không phải chờ.

### Cách bản ghi âm được chuẩn bị

Lowspeak ghi âm ở tần số lấy mẫu riêng của micrô và chuyển sang 16 kHz mà các mô hình cần ngay khi bạn dừng. Trước khi chép lại, nó loại bỏ tiếng ù trầm dưới 80 Hz, cắt khoảng lặng ở đầu và cuối, và cân bằng âm lượng, nên người nói nhỏ và người nói to cho kết quả như nhau. Bản ghi âm không có lời nói sẽ không được gửi đi. Ở chế độ cục bộ, Parakeet nhận bản ghi âm nguyên gốc trước, và chỉ nhận bản đã chuẩn bị nếu bản gốc không cho ra văn bản nào.

### Ngôn ngữ theo mô hình

Ngôn ngữ trong **Ngôn ngữ** được chuyển cho các mô hình trực tuyến và cho Whisper. Với **Tự động phát hiện**, mô hình tự xác định ngôn ngữ.

| Mô hình | Ngôn ngữ bạn có thể đọc chính tả |
|---|---|
| GPT Transcribe, Whisper (OpenAI, Groq, trên máy Mac của bạn) | khoảng 99, trong đó có cả 35 ngôn ngữ của menu |
| Parakeet Ultra | Bulgaria, Croatia, Séc, Đan Mạch, Hà Lan, Anh, Estonia, Phần Lan, Pháp, Đức, Hy Lạp, Hungary, Ý, Latvia, Litva, Malta, Ba Lan, Bồ Đào Nha, Romania, Nga, Slovakia, Slovenia, Tây Ban Nha, Thụy Điển, Ukraina |
| Parakeet v2 | Anh |

Parakeet tự nhận diện ngôn ngữ và bỏ qua lựa chọn trong menu. **Trung Quốc (Phồn thể)** yêu cầu GPT Transcribe viết chữ phồn thể; Whisper viết tiếng Trung bằng một trong hai hệ chữ và viết tiếng Bồ Đào Nha không phân biệt biến thể.

### Máy Mac Intel

Lowspeak là ứng dụng universal và cũng chạy trên máy Mac Intel với macOS 14. Mọi thứ trực tuyến hoạt động như nhau. Trên những máy đó, các mô hình cục bộ không có phần cứng mà chúng được xây dựng cho: Whisper chạy trên bộ xử lý thay vì GPU, và Parakeet chạy trên bộ xử lý thay vì Neural Engine. Cả hai đều hoạt động, nhưng mất thời gian lâu hơn nhiều lần so với trên Apple Silicon, còn các mô hình Whisper lớn thì quá chậm để đọc chính tả. Trên máy Mac Intel, hãy chép lại trực tuyến và giữ Whisper Small hoặc Parakeet Ultra làm [dự phòng](#khi-dịch-vụ-trực-tuyến-gặp-sự-cố).

## Từ vựng

**Từ vựng…** nhận những tên riêng và thuật ngữ mà Lowspeak cần viết chính xác, phân tách bằng dấu phẩy: tên sản phẩm, đồng nghiệp, thuật ngữ chuyên môn. Tối đa 100 thuật ngữ, mỗi thuật ngữ tối đa 60 ký tự; `<` và `>` sẽ bị loại bỏ.

- **GPT Transcribe** nhận chúng làm từ khóa.
- **Whisper** (OpenAI, Groq và trên máy Mac của bạn) đọc chúng như một gợi ý trước bản ghi âm, tối đa 500 ký tự.
- **Hậu xử lý** được yêu cầu viết chúng đúng y như đã ghi.
- **Parakeet** không dùng chúng.

## Hậu xử lý

Sau khi chép lại, một mô hình ngôn ngữ có thể chỉnh sửa văn bản. Chọn trong **Hậu xử lý**:

- **Tắt** (mặc định) — văn bản đúng như đã chép lại.
- **Làm gọn** — loại bỏ từ đệm như “ừm”, “kiểu như” hay “thì là”, sửa ngữ pháp và dấu câu, và chỉ giữ lại phần đã sửa khi bạn tự đính chính. Nó không thêm gì và không thay đổi ý nghĩa.
- **Định dạng** — như Làm gọn, đồng thời điều chỉnh văn bản theo ứng dụng bạn đang đọc chính tả vào: thuật ngữ kiểu mã trong trình soạn mã, dòng lệnh trong terminal, giọng thân mật trong ứng dụng trò chuyện, giọng chuyên nghiệp trong thư điện tử.

Hậu xử lý dùng Groq (`openai/gpt-oss-120b`) nếu bạn có khóa Groq, nếu không thì dùng OpenAI (`gpt-4o-mini`), kể cả khi bạn chép lại cục bộ. Khi đã chọn một ngôn ngữ trong **Ngôn ngữ**, nó giữ nguyên ngôn ngữ đó. Nếu mất hơn tám giây, không thành công, hoặc trả về câu trả lời bị cắt cụt hay dài hơn nhiều so với những gì bạn nói, Lowspeak dán văn bản chưa xử lý. Nếu nhà cung cấp từ chối yêu cầu hoặc chưa đặt khóa, một thông báo sẽ cho biết một lần.

## Micrô

Trong **Micrô**, hãy chọn một micrô cố định hoặc **Mặc định hệ thống**. Lowspeak ghi nhớ thiết bị đó; khi nó không được kết nối, menu hiển thị nó kèm **(Không kết nối)** và Lowspeak ghi âm bằng micrô mặc định của hệ thống.

Micrô chỉ được bật trong lúc bạn đọc chính tả. Việc chuẩn bị micrô mất khoảng 0,07 giây với micrô tích hợp, khoảng nửa giây với Studio Display, và một đến hai giây với AirPods, vốn phải chuyển sang chế độ tai nghe có micrô trước. Biểu tượng ghi âm xuất hiện khi âm thanh tới; **Phát âm thanh khi sẵn sàng** thêm một âm thanh ngắn vào thời điểm đó. Tùy chọn này tắt theo mặc định, vì trên một số loa, chính âm thanh đó phát trễ nửa giây.

Nếu một micrô chỉ truyền về sự im lặng trong năm giây, như AirPods khi một thiết bị khác đang giữ micrô của chúng, Lowspeak sẽ báo cho bạn.

## Khóa API

**Đặt khóa API…** hỏi khóa và nhận ra nhà cung cấp qua phần đầu của khóa: `sk-` cho OpenAI, `gsk_` cho Groq. Khóa ở định dạng khác sẽ bị từ chối kèm lời giải thích. Nhập một khóa cho mỗi nhà cung cấp; khi có khóa cho cả hai, mọi mô hình trực tuyến đều dùng được. **Cập nhật khóa API…** thay thế khóa hoặc, với **Xóa tất cả khóa**, xóa chúng.

Các khóa được lưu trong chuỗi khóa đăng nhập của bạn dưới tên `com.lowspeak.app`. Tệp khóa của phiên bản trước (`~/Library/Application Support/Lowspeak/.api-key`) được chuyển vào đó ở lần mở tiếp theo và bị xóa khi mọi khóa đều đọc lại được từ chuỗi khóa.

## Thông báo

Lowspeak hoạt động ở chế độ nền, nên nó báo sự cố dưới dạng thông báo:

| Thông báo | Điều đã xảy ra |
|---|---|
| **Ghi âm không thành công** | không có quyền truy cập micrô, không có micrô nào được kết nối, hoặc không thể khởi động micrô |
| **Micrô không có tín hiệu** | năm giây im lặng, xem [Micrô](#micrô) |
| **Cần quyền Trợ năng** | văn bản nằm trong bảng nhớ tạm; hãy cho phép Trợ năng để dán tự động |
| **Dán không thành công** | văn bản nằm trong bảng nhớ tạm; hãy dán bằng ⌘V |
| **Chép lại không thành công** | kèm lý do: không có khóa, khóa bị từ chối, vượt giới hạn yêu cầu, lỗi máy chủ hoặc lỗi mạng, hoặc không có mô hình cục bộ |
| **Đã chép lại trên máy Mac này** | dịch vụ trực tuyến gặp sự cố và một mô hình cục bộ đã thay thế |
| **Hậu xử lý không thành công** | nhà cung cấp từ chối hậu xử lý hoặc chưa đặt khóa; văn bản đã được dán mà không qua hậu xử lý |
| **Không có khóa API** | bạn đã chọn một mô hình trực tuyến mà không có khóa cho nhà cung cấp của nó |
| **Tải về không thành công**, **Xóa không thành công**, **Không thể xóa** | thao tác quản lý mô hình cục bộ không thành công, hoặc một thao tác khác vẫn đang chạy |
| **Lỗi mục đăng nhập**, **Lỗi khóa API** | macOS hoặc chuỗi khóa đã từ chối thay đổi |

Thông báo không bao giờ chứa văn bản bạn đọc chính tả. Dịch vụ trực tuyến gặp sự cố và nhà cung cấp hậu xử lý từ chối yêu cầu chỉ được báo một lần, không phải sau mỗi lần đọc chính tả, cho tới khi chúng hoạt động trở lại.

## Giao diện và trợ năng

- **Biểu tượng trên thanh menu** là một ảnh mẫu, tự thay đổi theo thanh menu sáng, tối hoặc có màu.
- **VoiceOver** đọc biểu tượng là “Lowspeak”, còn menu, cảnh báo và thông báo là các điều khiển macOS tiêu chuẩn.
- **Không cần giữ phím nào.** Khóa bản ghi âm bằng Shift hoặc nhấn fn hai lần, và lần nhấn tiếp theo sẽ kết thúc nó.
- **Tham khảo nhanh** là một cửa sổ đơn giản với nhãn và một nút, dùng được từ bàn phím; Esc đóng cửa sổ này.

Những thiếu sót đã biết được liệt kê trong [ACCESSIBILITY.md](../../../ACCESSIBILITY.md): VoiceOver không thông báo khi bản ghi âm bắt đầu, và Lowspeak chưa được kiểm tra với VoiceOver hay Điều khiển công tắc.

## Ngôn ngữ

Menu, hộp thoại và thông báo được dịch sang 35 ngôn ngữ:

Ả Rập, Catalan, Trung Quốc (Giản thể), Trung Quốc (Phồn thể), Croatia, Séc, Đan Mạch, Hà Lan, Anh, Phần Lan, Pháp, Đức, Hy Lạp, Do Thái, Hindi, Hungary, Indonesia, Ý, Nhật, Hàn, Mã Lai, Na Uy, Ba Lan, Bồ Đào Nha (Brazil), Bồ Đào Nha (Bồ Đào Nha), Romania, Nga, Slovakia, Slovenia, Tây Ban Nha, Thụy Điển, Thái, Thổ Nhĩ Kỳ, Ukraina và Việt Nam.

Ngôn ngữ bạn chọn trong **Ngôn ngữ** cũng là ngôn ngữ của menu. Với **Tự động phát hiện**, menu theo thứ tự ngôn ngữ trong **Cài đặt hệ thống › Cài đặt chung › Ngôn ngữ & Vùng**, và dùng tiếng Anh nếu chưa có bản dịch. Thuật ngữ menu theo bản dịch của chính Apple, nên Lowspeak đọc như phần còn lại của máy Mac. Ngoài tiếng Anh, các bản dịch đều do máy dịch và chưa được người bản ngữ xem lại; mọi chỉnh sửa đều được hoan nghênh dưới dạng [issue](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Phím và thao tác

| Thao tác | Phím |
|---|---|
| Đọc chính tả khi giữ | giữ fn |
| Khóa bản ghi âm đang chạy | Shift trong khi giữ fn |
| Bắt đầu bản ghi âm đã khóa | Shift+fn, hoặc nhấn nhanh fn hai lần |
| Kết thúc bản ghi âm đã khóa | fn hoặc Shift |
| Thoát Lowspeak (khi menu đang mở) | ⌘Q |
| Đóng Tham khảo nhanh | Esc |

Lowspeak giữ lại những lần nhấn fn mà nó dùng, không để chúng tới các ứng dụng khác; Shift vẫn được truyền đi như bình thường.

## Quyền riêng tư và bảo mật

Lowspeak nghe những gì bạn nói khi giữ fn, có thể gõ vào bất kỳ ứng dụng nào và giữ các khóa API của bạn, nên nó làm ít nhất có thể với những thứ đó.

- **Giọng nói của bạn chỉ đi tới một nơi.** Ở chế độ trực tuyến, bản ghi âm được nén và gửi tới nhà cung cấp bạn chọn, OpenAI hoặc Groq, cùng với từ vựng của bạn. Ở chế độ cục bộ, không có gì rời khỏi máy Mac của bạn. Khi một yêu cầu trực tuyến không thành công, một mô hình cục bộ chép lại chính bản ghi âm đó trên máy Mac của bạn.
- **Hậu xử lý**, nếu bạn bật, sẽ gửi văn bản đã chép lại và từ vựng của bạn tới Groq hoặc OpenAI. Ở chế độ Định dạng, nó gửi kèm loại ứng dụng bạn đang đọc chính tả vào (trình soạn mã, terminal, trò chuyện hoặc thư điện tử); tên ứng dụng không được gửi đi.
- **Không telemetry, không tài khoản.** Ngoài việc chép lại và hậu xử lý, Lowspeak chỉ kết nối mạng để mở kết nối tới nhà cung cấp của bạn khi giữ fn, ở chế độ trực tuyến hoặc khi hậu xử lý đang bật, và để tải về một mô hình khi bạn yêu cầu, từ Hugging Face qua HTTPS, ở một phiên bản cố định và mọi tệp đều được đối chiếu với kiểm tra tổng của nó. **Trợ giúp › Báo cáo sự cố…** mở một biểu mẫu trên GitHub trong trình duyệt của bạn; địa chỉ của nó chứa phiên bản Lowspeak và phiên bản macOS, không có gì bạn đã đọc chính tả.
- **Khóa API** nằm trong chuỗi khóa đăng nhập của bạn. Chúng chỉ được gửi tới đúng nhà cung cấp của mình và được xóa khỏi thông báo lỗi trước khi bất cứ thứ gì được ghi vào nhật ký.
- **Văn bản đọc chính tả không được lưu.** Mười bản đọc chính tả gần nhất nằm trong bộ nhớ cho **Đọc chính tả gần đây** và biến mất khi Lowspeak thoát. Các tệp nhật ký ghi lại độ dài và thời gian, không bao giờ ghi văn bản.
- **Bảng nhớ tạm** được lưu lại trước khi dán và khôi phục khi ứng dụng đã đọc văn bản, trừ khi một ứng dụng khác đã thay đổi nó trong lúc đó. Bản đọc chính tả được đánh dấu là tạm thời, nên các ứng dụng lịch sử bảng nhớ tạm tôn trọng dấu này sẽ bỏ qua nó.
- **Ký mã** với Hardened Runtime; ứng dụng chỉ xin quyền dùng micrô, Trợ năng, thông báo và mạng.

Để báo cáo lỗ hổng bảo mật, hãy làm theo [chính sách bảo mật](../../../SECURITY.md).

## Tốc độ

Đo trên Apple Silicon:

- **Parakeet Ultra** chép lại 40 giây lời nói trong khoảng 0,3 giây.
- **Trực tuyến:** yêu cầu đầu tiên sau một khoảng nghỉ mất khoảng 0,4 giây tại Groq, vì Lowspeak mở kết nối trong khi bạn vẫn đang nói; các yêu cầu sau dùng lại kết nối đó, qua HTTP/3 nếu nhà cung cấp hỗ trợ.
- **Dung lượng tải lên:** khoảng 6 KB cho mỗi giây lời nói, dưới dạng AAC ở 48 kbit/s.
- **Chuẩn bị micrô:** khoảng 0,07 giây với micrô tích hợp, nửa giây với Studio Display, một đến hai giây với AirPods.

## Hạn chế đã biết

- **Mô hình cục bộ trên máy Mac Intel** chạy trên bộ xử lý và chậm hơn nhiều; trên các máy đó, chép lại trực tuyến là lựa chọn tốt hơn.
- **Những gì bạn nói trước khi micrô sẵn sàng sẽ bị mất.** Hãy chờ biểu tượng ghi âm, nhất là với AirPods.
- **Phím fn có thể bị chiếm.** Khi cài đặt phím 🌐 của macOS, Warp hoặc Đọc chính tả của macOS giành fn trước, Lowspeak không nhận được phím này; xem [Khắc phục sự cố](#khắc-phục-sự-cố).
- **Parakeet** bỏ qua từ vựng và lựa chọn ngôn ngữ, và chỉ biết các ngôn ngữ châu Âu.
- **Hệ chữ và biến thể:** chỉ GPT Transcribe có thể được yêu cầu viết tiếng Trung phồn thể; không mô hình nào có thể được yêu cầu viết tiếng Bồ Đào Nha châu Âu thay vì Bồ Đào Nha Brazil.
- **Giới hạn thời gian:** hai phút khi giữ fn, năm phút khi đã khóa.
- **Chế độ mic:** Trung tâm điều khiển đề xuất Tách lời nói và Âm phổ rộng cho Lowspeak và báo chúng đang hoạt động, nhưng Lowspeak ghi âm không qua xử lý giọng nói, nên lựa chọn đó không có tác dụng.
- **Đọc chính tả gần đây** chỉ nằm trong bộ nhớ và biến mất sau khi thoát.
- **Có chủ ý:** không có cửa sổ cài đặt, không có biểu tượng trên Dock, không tự động cập nhật, không có trên App Store.

## Khắc phục sự cố

**Không có gì xảy ra khi tôi giữ fn.** Hãy kiểm tra ba điều: Lowspeak đã được cho phép trong **Cài đặt hệ thống › Quyền riêng tư & Bảo mật › Trợ năng** (trên macOS 27: **Điều khiển thiết bị và truy cập dữ liệu**); **Cài đặt hệ thống › Bàn phím › Nhấn phím 🌐 để** được đặt thành **Không làm gì**; và không có ứng dụng nào khác giành fn. Warp làm vậy theo mặc định; hãy tắt bằng `defaults write dev.warp.Warp-Stable VoiceInputToggleKey '""'` rồi khởi động lại Warp. Đọc chính tả của macOS cũng có thể được đặt để bắt đầu bằng fn.

**Sau khi cập nhật, Lowspeak không còn phản hồi fn.** macOS gắn quyền Trợ năng với chữ ký của ứng dụng. Nếu chữ ký thay đổi, ví dụ từ bản tự build sang ứng dụng tải về, hãy xóa Lowspeak khỏi danh sách Trợ năng (trên macOS 27: Điều khiển thiết bị và truy cập dữ liệu) rồi thêm lại.

**Mất những từ đầu tiên.** Hãy bắt đầu nói khi biểu tượng ghi âm xuất hiện. Với AirPods, micrô cần một đến hai giây; micrô tích hợp hoặc micrô USB chọn trong **Micrô** gần như sẵn sàng ngay lập tức.

**“Micrô không có tín hiệu.”** Micrô không truyền âm thanh nào trong năm giây. Với AirPods, một thiết bị khác có thể đang dùng micrô của chúng; hãy chuyển sang micrô tích hợp hoặc kết nối lại AirPods. Gập nắp MacBook cũng làm tắt tiếng micrô tích hợp.

**Bản đọc chính tả trống trơn.** Âm lượng đầu vào có thể quá thấp: các ứng dụng như Teams, Zoom hay Meet thay đổi nó trong cuộc gọi, và macOS lưu nó riêng cho từng thiết bị. Hãy tăng nó trong **Cài đặt hệ thống › Âm thanh › Đầu vào**.

**Văn bản bị dán nhầm chỗ.** **Đọc chính tả gần đây** trong menu sao chép lại nó vào bảng nhớ tạm.

**“Không có khóa API.”** Chọn **Đặt khóa API…** trong menu, hoặc chuyển sang một mô hình cục bộ.

**Lần chép lại cục bộ đầu tiên mất nhiều thời gian.** Mỗi mô hình được chuẩn bị cho máy Mac của bạn ở lần dùng đầu tiên; từ đó trở đi nó được tải trong chưa đầy một giây.

**Lowspeak thoát ngay sau khi mở.** Một bản Lowspeak khác đang chạy; hãy thoát bản đó trước.

**Không có thông báo nào xuất hiện.** Hãy cho phép chúng trong **Cài đặt hệ thống › Thông báo › Lowspeak**.

**Vấn đề khác.** Các nhật ký trong `~/Library/Application Support/Lowspeak/` (`appstate.log`, `audio.log`, `hotkey.log`, `crash.log`) mô tả những gì đã xảy ra, không chứa văn bản bạn đọc chính tả. Hãy đính kèm các dòng liên quan vào một [báo cáo sự cố](https://github.com/sventmueller/lowspeak/issues/new/choose).

## Dành cho nhà phát triển

Cách build, kiểm thử, cấu trúc mã nguồn và các quy tắc mà mã tuân theo được mô tả trong [docs/development.md](../../../docs/development.md) (bằng tiếng Anh); cách đóng góp được mô tả trong [CONTRIBUTING.md](../../../CONTRIBUTING.md).

### Thành phần của bên thứ ba

| Thành phần | Phiên bản | Dùng cho | Giấy phép |
|---|---|---|---|
| [whisper.cpp](https://github.com/ggml-org/whisper.cpp) qua [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16 | Whisper cục bộ trên GPU (Metal) | MIT; whisper-rs Unlicense |
| [FluidAudio](https://github.com/FluidInference/FluidAudio) | 0.17.7 | giải mã Parakeet và ghép bản ghi âm dài, chuyển sang Rust trong `src/transcription/parakeet` | Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) và các crate framework của nó | 0.6 / 0.3 | AppKit, CoreML, CoreAudio, AVFoundation và các framework hệ thống khác | MIT; Zlib, Apache-2.0 hoặc MIT |
| [cpal](https://github.com/RustAudio/cpal) | 0.18 | thu âm từ micrô qua CoreAudio | Apache-2.0 |
| [rubato](https://github.com/HEnquist/rubato) | 5 | lấy mẫu lại về 16 kHz | MIT |
| [ureq](https://github.com/algesten/ureq) với rustls | 3.5 | tải về mô hình | MIT hoặc Apache-2.0; rustls và ring theo ISC và Apache-2.0 |
| [sha1, sha2](https://github.com/RustCrypto/hashes) | 0.11 | kiểm tra tổng của các mô hình tải về | MIT hoặc Apache-2.0 |
| [security-framework](https://github.com/kornelski/rust-security-framework) | 3.7 | khóa API trong chuỗi khóa | MIT hoặc Apache-2.0 |
| [serde](https://serde.rs), serde_json, anyhow, unicode-general-category | 1 | JSON, lỗi, bộ từ vựng của Parakeet | MIT hoặc Apache-2.0; Apache-2.0 |

Văn bản giấy phép của mọi thành phần bên trong ứng dụng được kèm theo ứng dụng dưới dạng `Contents/Resources/Acknowledgements.txt`.

Các mô hình không phải là một phần của Lowspeak; nó tải chúng về khi bạn yêu cầu:

| Mô hình | Nguồn | Giấy phép |
|---|---|---|
| Whisper Tiny … Large v3 Turbo | [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp) (Whisper của OpenAI ở định dạng ggml) | MIT |
| Parakeet v2, Parakeet Ultra | Bản chuyển đổi CoreML của [FluidInference](https://huggingface.co/FluidInference) từ Parakeet TDT của NVIDIA | CC-BY-4.0 |

## Giấy phép

Lowspeak được cấp phép theo một trong hai giấy phép sau

- Giấy phép Apache, Phiên bản 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE))
- Giấy phép MIT ([LICENSE-MIT](../../../LICENSE-MIT))

tùy bạn chọn.

Trừ khi bạn tuyên bố rõ ràng khác đi, mọi đóng góp được bạn cố ý gửi để đưa vào Lowspeak, theo định nghĩa trong giấy phép Apache-2.0, sẽ được cấp phép kép như trên, không có thêm điều khoản hay điều kiện nào.
