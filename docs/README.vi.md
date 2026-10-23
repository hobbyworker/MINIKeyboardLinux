<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="Biểu tượng ứng dụng"></p>

<h1 align="center">Công cụ cấu hình bàn phím macro cho Linux</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <b>Tiếng Việt</b> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

Ứng dụng Linux dùng để ghi thiết lập phím vào những bàn phím macro USB cỡ nhỏ — loại được bán từ bản chỉ có một núm xoay
cho đến bản 21 phím, với tối đa bốn núm xoay. Với từng phím và từng thao tác của núm xoay (xoay sang trái, nhấn, xoay
sang phải), bạn có thể gán thao tác nhấn phím, phím đa phương tiện hoặc tác vụ chuột cho lớp 1–3, và đặt cả đèn của bàn
phím. Đây là phiên bản Linux của một ứng dụng macOS được làm theo cùng các quy tắc, và dùng cùng các lệnh.

| Bàn phím | Bố cục | Đọc lại | Độ trễ | Đèn | Thử thực tế |
|---|---|---|---|---|---|
| `1189:8890` | bạn tự đặt — phím (tối đa 12), số phím mỗi hàng, núm xoay (tối đa 2) | không | không có | ba chế độ | đã thử — 6 phím, 1 núm xoay |
| nhà sản xuất `1189` hoặc `514C`, sản phẩm `8840` · `8842` · `8830`–`8833` · `8850` (và `514C:8851`), chương trình cơ sở mới | theo thông tin bàn phím báo — 26 bố cục | ngay khi kết nối | cho từng lần nhấn | một chế độ, và màu cho từng phím 1–16 | chưa |
| cùng loại bàn phím, chương trình cơ sở cũ | theo thông tin bàn phím báo | khi bạn yêu cầu | mỗi phím một giá trị | màu và chế độ | chưa |
| cùng loại bàn phím, không cho biết loại | bạn tự chọn — 26 bố cục | không | không có | màu và chế độ | chưa |

`1189:8890` là bàn phím mà công cụ cấu hình Windows của nhà sản xuất, "MINI KeyBoard", hỗ trợ; các bàn phím còn lại là
những bàn phím mà ứng dụng đa nền tảng của nhà sản xuất, "MINI_KEYBOARD", tìm kiếm. Ứng dụng dùng lệnh của cả hai công
cụ. Bàn phím loại thứ hai thường cho biết có bao nhiêu phím, bao nhiêu núm xoay và đang chạy chương trình cơ sở nào, nên
ứng dụng vẽ đúng bố cục mà ứng dụng của nhà sản xuất vẽ; bàn phím không cho biết thì dùng bố cục bạn chọn trong số đó.
Ứng dụng của nhà sản xuất không biết đến `1189:8890`, và chính công cụ của nó cũng không vẽ hình dạng, nên không thể
biết hình dạng thực tế. Vì vậy, bạn chỉ cần đặt số phím, số núm xoay và số phím trên mỗi hàng, ứng dụng sẽ vẽ ra một bố
cục ngay ngắn đúng như vậy.

Ứng dụng có 13 ngôn ngữ — tiếng Anh, tiếng Hàn, tiếng Nhật, tiếng Trung (Giản thể và Phồn thể), tiếng Đức, tiếng Tây Ban
Nha, tiếng Pháp, tiếng Ý, tiếng Bồ Đào Nha (Brazil), tiếng Ba Lan, tiếng Indonesia và tiếng Việt. Ứng dụng theo ngôn ngữ
của môi trường desktop; với các ngôn ngữ khác, ứng dụng hiển thị bằng tiếng Anh.

Mã nguồn trong kho này được cấp phép MIT. Biểu tượng ứng dụng — trong `data/icons/`, và hình ở đầu trang — được cấp phép
CC BY-SA 4.0 ([`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)).

## Tải xuống

Hãy mở [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) trong trung tâm phần mềm,
hoặc chạy:

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

Thao tác này thêm kho Flatpak riêng của ứng dụng, và lần cài đầu tiên cũng tải runtime GNOME từ Flathub. Từ đó trở đi,
trung tâm phần mềm — hoặc `flatpak update` — sẽ tải phiên bản mới về. Thiết lập của bạn vẫn được giữ nguyên.

Mỗi phiên bản cũng có trên trang [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) dưới dạng gói
Flatpak — `MINIKeyboard-<phiên bản>-x86_64.flatpak` cho Intel và AMD, `-aarch64.flatpak` cho ARM — dành cho PC không có
kết nối mạng, hoặc khi bạn cần một phiên bản cụ thể. Gói được cài bằng `flatpak install --user <tệp>`, rồi sau đó nhận
bản cập nhật từ cùng kho đó.

## Cho phép ứng dụng mở bàn phím

Trên Linux, ứng dụng chỉ mở được thiết bị USB khi có quy tắc udev cấp thiết bị đó cho người dùng đang đăng nhập, và
Flatpak không thể cài quy tắc này. Lần đầu, khi ứng dụng báo cần quyền truy cập, **Cách cho phép…** sẽ hiển thị các lệnh
ghi quy tắc đó cùng nút sao chép. Hãy chạy chúng một lần trong terminal, rồi cắm lại bàn phím. Nếu bạn đã clone kho này,
quy tắc đó chính là [`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules).

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- `1189:8890` được ghi trực tiếp qua USB (libusb). Kênh cấu hình của nó chỉ có một endpoint OUT, nên không trình điều
  khiển nào nhận nó.
- Các bàn phím còn lại được ghi qua thiết bị hidraw của giao diện USB đầu tiên, giống như ứng dụng của nhà sản xuất.
  Trong lúc đó, bàn phím vẫn gõ bình thường.

## Build

```sh
make build    # target/release/minikeyboard, cho PREFIX (nếu không chỉ định thì là /usr/local)
make install  # sau make build: chương trình, bản dịch, tệp desktop, metainfo, biểu tượng và giấy phép, vào DESTDIR/PREFIX
make test     # kiểm thử của mọi crate — không mở cửa sổ nào, không đụng đến bàn phím
make check    # rustfmt và clippy (cảnh báo được coi là lỗi)
make run      # bản build Debug kèm bản dịch
```

Cần Rust 1.88 trở lên và các tệp phát triển của GTK 4, libadwaita 1.7 trở lên, libsoup 3 và libusb 1.0 — trên Debian và
Ubuntu là `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`, trên Fedora là
`gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`. Bản dịch cần `msgfmt` của gettext.

ID của ứng dụng (`me.hobbyworker.MINIKeyboard`), tên và các liên kết là của riêng ứng dụng, và khớp với tệp desktop,
metainfo và biểu tượng trong `data/`. Bản fork đặt giá trị của riêng mình lúc build: `MINIKEYBOARD_APP_ID` (cũng là tên
biểu tượng), `MINIKEYBOARD_APP_NAME`, `MINIKEYBOARD_COPYRIGHT`, `MINIKEYBOARD_WEBSITE` và `MINIKEYBOARD_ISSUE_URL`.
`make build` lấy nơi chứa bản dịch từ `PREFIX`; `MINIKEYBOARD_LOCALE_DIR` lúc chạy sẽ ghi đè giá trị đó, khi bạn muốn
thử một bản build.

Ứng dụng cũng có thể tự báo khi có phiên bản mới: bản build được cung cấp `MINIKEYBOARD_UPDATE_FEED` đọc feed RSS đó lúc
khởi chạy và mỗi ngày một lần — trừ khi bạn tắt **Tự động kiểm tra bản cập nhật** trong Tùy chỉnh — và cả khi bạn nhấp
**Kiểm tra bản cập nhật…**, rồi hiện một biểu ngữ dẫn đến trang tải xuống của phiên bản mới. Các bản build do chính ứng
dụng phát hành không dùng tính năng này — bản cập nhật đến qua Flatpak — nên không có mục menu lẫn thiết lập đó.

Bản build Debug có menu con **Trình mô phỏng** để dùng thử ứng dụng mà không cần bàn phím đó. Chọn `1189:8890` với ID
báo cáo bất kỳ, một bố cục bất kỳ của một trong hai chương trình cơ sở, hoặc bàn phím không cho biết loại, ứng dụng sẽ
hoạt động như thể bàn phím đang được cắm vào — với cùng các lệnh, không có gì được gửi qua USB.
`MINIKEYBOARD_SIMULATED_KEYBOARD` (ví dụ `8890-3`) khởi động ứng dụng với bàn phím đó. Trong **Nhật ký thiết bị**, bạn
có thể xem mọi báo cáo đã ghi và mọi phản hồi nhận được.

## Cách dùng

1. Kết nối bàn phím qua USB. Ứng dụng tìm bàn phím và hỏi loại nếu đó là bàn phím loại thứ hai. Nếu là chương trình cơ
   sở mới, thiết lập được đọc ngay.
2. Với bàn phím không cho biết hình dạng, lần đầu bạn chọn bố cục — một trong các bố cục của ứng dụng nhà sản xuất, hoặc
   với `1189:8890` là số phím, số phím mỗi hàng và số núm xoay — cùng hướng đặt bàn phím trước mặt bạn. Ứng dụng ghi nhớ
   theo từng USB ID. Sau này bạn có thể đổi bằng **Hình dạng…** và **Xoay** (Ctrl+T). Bàn phím đã cho biết hình dạng thì
   chỉ xoay được.
3. Chọn lớp, rồi chọn phím hoặc thao tác núm xoay trên hình bàn phím.
4. Gán tác vụ.
   - **Phím** — giữ phím bổ trợ (Ctrl, Alt, Shift, Super và các phím bên phải), rồi nhấp vào phím. Mỗi lần nhấp thêm một
     lần nhấn. **Chỉ phím bổ trợ** biến phím đó thành phím bổ trợ. **QWERTY** xếp phím như bàn phím đầy đủ, còn **ABC**
     xếp theo nhóm. Chỉ nhấp được những phím mà bàn phím nhận.
   - **Đa phương tiện**, **Chuột** — thay thế tác vụ trước đó của phím.
   - **LED** — đèn của lớp. Với chương trình cơ sở mới, chọn một trong các phím 1–16 để đặt màu cho phím đó.
   - **Độ trễ** — trên chương trình cơ sở có ghi nhớ độ trễ.
5. **Lưu** (Ctrl+S) ghi những gì đã thay đổi. **Đọc** (Ctrl+R) lấy thiết lập hiện tại từ bàn phím đọc được.

Trong **Tùy chỉnh** (Ctrl+,), bạn chọn ngôn ngữ giao diện (một trong 13 ngôn ngữ, hoặc ngôn ngữ của môi trường desktop)
và chế độ sáng hoặc tối. Ở bản build có feed cập nhật, trang Chung trong đó có mục tự động kiểm tra bản cập nhật. Trong
**Giới thiệu về MINIKeyboard**, bạn xem được phiên bản và giấy phép nguồn mở.

## Cấu trúc

Các quy tắc nằm trong các crate. Phụ thuộc chỉ hướng vào trong, và lớp nào vươn ra ngoài sẽ không biên dịch được.

```
crates/
├── domain/        bàn phím, tác vụ, các nút điều khiển của từng bố cục và những gì mỗi bàn phím chứa được; port; use case (chỉ thư viện chuẩn)
├── datalayer/     lệnh của từng bàn phím và số hiệu các nút điều khiển; tìm bàn phím trong sysfs và ghi qua USB (libusb) hoặc hidraw; bàn phím mô phỏng
├── presentation/  view model và các giá trị màn hình vẽ — hình vẽ bố cục, bảng phím và màu, quy tắc udev (không có toolkit)
├── ui/            GTK 4 và libadwaita — chỉ thấy presentation; thông báo phiên bản mới (libsoup)
└── minikeyboard/  phần lắp ráp (composition root) — nơi duy nhất biết mọi crate
po/                bản dịch, mỗi ngôn ngữ một tệp
data/              tệp desktop, metainfo và biểu tượng; quy tắc udev
LICENSES/          giấy phép của biểu tượng (CC BY-SA 4.0)
```

Tiêu chuẩn là không có cảnh báo nào. Cảnh báo được sửa chứ không bị tắt đi.

## Hạn chế

- Đến nay, việc ghi mới được kiểm tra trên một bàn phím là `1189:8890` có 6 phím và 1 núm xoay, trên Fedora. Các bàn
  phím còn lại được ghi đúng theo cách ứng dụng của nhà sản xuất ghi — các lệnh giống hệt lệnh của ứng dụng macOS đến
  từng byte và được kiểm thử giữ đúng, nhưng chưa bàn phím nào được thử trên thiết bị thật, và đường ghi đến chúng
  (hidraw) cũng chưa.
- `1189:8890` và các bàn phím không cho biết loại không báo lại thiết lập hiện tại, nên chỉ thấy được những gì đã đặt
  trong ứng dụng.
- Mỗi lần chỉ một bàn phím. Nếu cắm nhiều bàn phím, ứng dụng dùng bàn phím tìm thấy đầu tiên — `1189:8890` được ưu tiên.
- Không hỗ trợ di chuyển chuột và kéo thả.

## Giấy phép

MIT — xem [LICENSE](../LICENSE). Biểu tượng ứng dụng: CC BY-SA 4.0 — xem
[LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt).
