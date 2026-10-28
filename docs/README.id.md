<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="Ikon aplikasi"></p>

<h1 align="center">Alat pengatur papan ketik makro untuk Linux</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <b>Bahasa Indonesia</b> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

Aplikasi Linux untuk menulis pengaturan tombol ke papan ketik makro USB berukuran kecil — papan ketik yang dijual
mulai dari yang hanya punya satu kenop hingga yang punya 21 tombol, dengan kenop sampai empat buah. Untuk setiap
tombol dan setiap tindakan kenop (putar ke kiri, tekan, putar ke kanan), Anda dapat mengatur tekanan tombol, tombol
multimedia, atau tindakan tetikus di lapisan 1–3, serta pencahayaan papan ketik. Aplikasi ini adalah versi Linux
dari aplikasi macOS yang dibuat dengan aturan yang sama, dan memakai perintah yang sama.

<p align="center"><img src="../.github/devices.png" width="560" alt="Dua papan ketik makro USB kecil: 6 tombol dan 1 kenop, 12 tombol dan 2 kenop"></p>

| Papan ketik | Tata letak | Membaca kembali | Tunda | Pencahayaan | Diuji langsung |
|---|---|---|---|---|---|
| `1189:8890` | Anda atur sendiri — tombol (hingga 12), tombol per baris, kenop (hingga 2) | tidak bisa | tidak ada | tiga mode | sudah — 6 tombol, 1 kenop |
| produsen `1189` atau `514C`, produk `8840` · `8842` · `8830`–`8833` · `8850` · `8851`, firmware baru | sesuai yang dilaporkan papan ketik — 26 tata letak | langsung saat terhubung | per tekanan | satu mode, dan warna untuk tombol 1–16 | sudah — `514C:8850`, 12 tombol, 2 kenop |
| papan ketik yang sama, firmware lama | sesuai yang dilaporkan papan ketik | langsung saat terhubung | satu per tombol | warna dan mode | belum |
| papan ketik yang sama, tidak memberi tahu jenisnya | Anda pilih sendiri — 26 tata letak | tidak bisa | tidak ada | warna dan mode | belum |

`1189:8890` adalah papan ketik yang ditangani alat pengaturan Windows dari produsen, "MINI KeyBoard"; sisanya adalah
papan ketik yang dicari oleh aplikasi lintas platform dari produsen, "MINI_KEYBOARD". Aplikasi ini memakai perintah dari
kedua alat tersebut. Papan ketik jenis kedua biasanya memberi tahu jumlah tombol dan kenopnya serta firmware yang
dipakainya, sehingga aplikasi menggambar tata letaknya persis seperti aplikasi produsen; papan ketik yang tidak memberi
tahu memakai salah satu tata letak itu yang Anda pilih. `1189:8890` tidak dikenal oleh aplikasi produsen, dan alatnya
sendiri pun tidak menggambar bentuknya, jadi bentuk aslinya tidak diketahui. Karena itu, cukup tentukan jumlah tombol
dan kenopnya serta jumlah tombol dalam satu baris, lalu aplikasi akan menggambarnya dengan rapi sesuai angka itu.

Aplikasi ini tersedia dalam 13 bahasa — Inggris, Korea, Jepang, Tionghoa (Sederhana dan Tradisional), Jerman, Spanyol,
Prancis, Italia, Portugis (Brasil), Polandia, Indonesia, dan Vietnam. Aplikasi mengikuti bahasa desktop; untuk bahasa
lainnya, aplikasi tampil dalam bahasa Inggris.

Kode sumber di repositori ini berlisensi MIT. Ikon aplikasi — yang ada di `data/icons/` dan gambar di bagian atas —
berlisensi CC BY-SA 4.0 ([`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)).

## macOS, Windows, dan Linux

MINIKeyboard hadir dalam tiga edisi yang dibuat dengan aturan yang sama — papan ketik yang sama, perintah yang sama, 13 bahasa yang sama.

| Repositori | GitHub |
|---|---|
| **MINIKeyboardApple**<br><img alt="macOS 14+" src="https://img.shields.io/badge/macOS-14%2B-0A84FF?logo=apple&logoColor=white"> <img alt="Universal: Apple Silicon + Intel" src="https://img.shields.io/badge/Universal-Apple%20Silicon%20%2B%20Intel-5E5CE6"> <img alt="Swift 6" src="https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardApple](https://github.com/hobbyworker/MINIKeyboardApple) |
| **MINIKeyboardWin**<br><img alt="Windows 10 (2004)+ · 11" src="https://img.shields.io/badge/Windows-10%20(2004)%2B%20·%2011-0078D4?logo=windows&logoColor=white"> <img alt="x64 · ARM64" src="https://img.shields.io/badge/x64%20·%20ARM64-5E5CE6"> <img alt=".NET 10 · WinUI 3" src="https://img.shields.io/badge/.NET%2010-WinUI%203-512BD4?logo=dotnet&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardWin](https://github.com/hobbyworker/MINIKeyboardWin) |
| **MINIKeyboardLinux**<br><img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white"> <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white"> <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardLinux](https://github.com/hobbyworker/MINIKeyboardLinux) |

## Unduh

Buka [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) di pusat perangkat lunak, atau
jalankan:

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

Perintah ini menambahkan repositori Flatpak milik aplikasi ini, dan pemasangan pertama juga mengunduh runtime GNOME dari
Flathub. Setelah itu, pusat perangkat lunak — atau `flatpak update` — akan mengambil versi baru. Pengaturan Anda tetap
tersimpan.

Setiap versi juga tersedia di [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) sebagai paket
Flatpak — `MINIKeyboard-<versi>-x86_64.flatpak` untuk Intel dan AMD, serta `-aarch64.flatpak` untuk ARM. Paket ini
berguna untuk PC tanpa koneksi internet atau saat Anda memerlukan versi tertentu. Paket dipasang dengan
`flatpak install --user <file>`, lalu pembaruannya diambil dari repositori yang sama.

## Mengizinkan aplikasi membuka papan ketik

Linux hanya mengizinkan aplikasi membuka perangkat USB yang diberikan aturan udev kepada pengguna yang sedang masuk,
dan Flatpak tidak dapat memasang aturan itu. Saat pertama kali aplikasi memberi tahu bahwa ia memerlukan izin akses,
**Cara mengizinkan…** menampilkan perintah untuk menulis aturan itu beserta tombol untuk menyalinnya. Jalankan
perintah itu sekali di terminal, lalu colokkan kembali papan ketiknya. Jika Anda meng-clone repositori ini, aturan
yang sama ada di [`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules):

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- `1189:8890` ditulis langsung melalui USB (libusb). Saluran konfigurasinya hanya punya satu endpoint OUT, jadi tidak
  ada driver yang mengambilnya.
- Papan ketik lainnya ditulis melalui perangkat hidraw dari antarmuka USB pertamanya, seperti yang dilakukan aplikasi
  produsen. Selama itu, papan ketik tetap bisa dipakai mengetik seperti biasa.

## Build

```sh
make build    # target/release/minikeyboard, untuk PREFIX (/usr/local jika tidak diberikan)
make install  # setelah make build: program, terjemahan, file .desktop, metainfo, ikon, dan lisensi ke DESTDIR/PREFIX
make test     # tes semua crate — tidak ada jendela yang terbuka, tidak ada papan ketik yang disentuh
make check    # rustfmt dan clippy; peringatan dianggap galat
make run      # build Debug beserta terjemahannya
```

Diperlukan Rust 1.88 atau yang lebih baru, serta file pengembangan GTK 4, libadwaita 1.7 atau yang lebih baru,
libsoup 3, dan libusb 1.0 — di Debian dan Ubuntu `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`,
di Fedora `gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`. Untuk terjemahan, diperlukan `msgfmt` dari
gettext.

ID aplikasi (`me.hobbyworker.MINIKeyboard`), nama, dan tautannya adalah milik aplikasi ini sendiri dan sesuai dengan
file .desktop, metainfo, serta ikon di `data/`. Fork memberikan miliknya sendiri saat build: `MINIKEYBOARD_APP_ID`
(sekaligus nama ikonnya), `MINIKEYBOARD_APP_NAME`, `MINIKEYBOARD_COPYRIGHT`, `MINIKEYBOARD_WEBSITE`, dan
`MINIKEYBOARD_ISSUE_URL`. Lokasi terjemahan ditentukan oleh `make build` dari `PREFIX`, dan `MINIKEYBOARD_LOCALE_DIR`
saat dijalankan akan menimpanya — untuk mencoba hasil build.

Aplikasi juga bisa memberi tahu sendiri adanya versi baru. Build yang diberi `MINIKEYBOARD_UPDATE_FEED` membaca feed
RSS itu saat dijalankan dan sekali sehari — kecuali **Periksa pembaruan secara otomatis** dimatikan di Preferensi —
dan juga saat **Periksa Pembaruan…** diklik, lalu menampilkan banner yang mengarah ke halaman unduhan versi baru.
Build yang dirilis aplikasi ini sendiri tidak memakainya — pembaruannya diambil oleh Flatpak — sehingga tidak punya
item menu maupun pengaturan itu.

Submenu **Simulator** memungkinkan Anda mencoba aplikasi tanpa papan ketik tersebut. Pilih `1189:8890`, tata letak dari
firmware baru, atau papan ketik yang tidak memberi tahu jenisnya, dan aplikasi akan bekerja seolah-olah papan ketik itu
tercolok — dengan perintah yang sama, tanpa ada yang dikirim ke USB. Selama digunakan, bilah oranye di bagian atas jendela
memberi tahu hal itu, dan **Kembali ke papan ketik di USB** mengembalikannya. Build Debug menawarkan semua jenis (ID laporan
lain dari `1189:8890`, firmware lama), dan dengan `MINIKEYBOARD_SIMULATED_KEYBOARD` (misalnya `8890-3`), aplikasi langsung
dimulai dengan papan ketik itu. Di **Log Perangkat**, Anda bisa melihat semua laporan yang ditulis dan semua jawaban yang
diterima.

## Cara menggunakan

1. Hubungkan papan ketik melalui USB. Aplikasi menemukan papan ketik dan menanyakan jenisnya jika papan ketik itu dari
   jenis kedua. Jika papan ketik itu menjawab, pengaturannya langsung dibaca.
2. Untuk papan ketik yang tidak memberi tahu bentuknya, pertama kali Anda menentukan tata letaknya — salah satu tata
   letak aplikasi produsen, atau untuk `1189:8890`, jumlah tombol, tombol per baris, dan kenop — serta arah posisinya
   di depan Anda. Aplikasi mengingatnya untuk setiap ID USB. Nanti Anda bisa mengubahnya dengan **Bentuk…** dan
   **Putar** (Ctrl+T). Papan ketik yang sudah memberi tahu bentuknya hanya bisa diputar.
3. Pilih lapisan, lalu pilih tombol atau tindakan kenop pada gambar papan ketik.
4. Berikan tindakannya.
   - **Tombol** — tahan tombol pengubah (Ctrl, Alt, Shift, Super, dan yang di sisi kanan), lalu klik tombol. Setiap
     klik menambahkan satu tekanan. **Hanya pengubah** menjadikan tombol itu sebagai tombol pengubah. **QWERTY**
     menata tombol seperti pada papan ketik ukuran penuh, sedangkan **ABC** menatanya per kelompok. Hanya tombol yang
     diterima papan ketik yang bisa diklik.
   - **Multimedia**, **Tetikus** — menggantikan tindakan tombol sebelumnya.
   - **LED** — pencahayaan lapisan. Pada firmware baru, pilih salah satu tombol 1–16 untuk memberinya warna.
   - **Tunda** — pada firmware yang menyimpan tunda.
5. **Simpan** (Ctrl+S) menulis perubahan. **Baca** (Ctrl+R) mengambil pengaturan saat ini dari papan ketik yang bisa
   dibaca.

Di **Preferensi** (Ctrl+,), Anda memilih bahasa antarmuka (salah satu dari 13 bahasa, atau bahasa desktop) serta
tampilan terang atau gelap. Pada build yang punya feed pembaruan, halaman Umum-nya berisi pemeriksaan pembaruan
otomatis. Di **Mengenai MINIKeyboard**, Anda bisa melihat versi dan lisensi sumber terbuka.

Di **Preferensi ▸ Umum ▸ Penanganan bug firmware**, **Cegah pengulangan tombol setelah mengubah pencahayaan** aktif
secara bawaan. Firmware `1189:8890` mengulang karakter pertama sebuah tombol tanpa henti setelah pencahayaannya
disimpan (perangkat lunak pabrikannya juga begitu), jadi aplikasi mengirim satu perintah simpan lagi, tanpa tombol,
setelah pencahayaan. Jika dinonaktifkan, aplikasi hanya mengirim hal yang sama dengan perangkat lunak pabrikan. Papan
ketik ini juga melupakan pencahayaannya saat dicabut.

## Struktur

Aturannya berada di crate. Ketergantungannya hanya mengarah ke dalam, dan lapisan yang menjangkau ke luar tidak akan
terkompilasi.

```
crates/
├── domain/        papan ketik, tindakan, kontrol setiap tata letak, dan apa yang bisa ditampung tiap papan ketik; port; use case (hanya pustaka standar)
├── datalayer/     perintah setiap papan ketik dan nomor kontrolnya; menemukan papan ketik di sysfs dan menulis lewat USB (libusb) atau hidraw; papan ketik simulasi
├── presentation/  view model dan nilai yang digambar layar — gambar tata letak, tabel tombol dan warna, aturan udev (tanpa toolkit)
├── ui/            GTK 4 dan libadwaita — hanya melihat presentation; pemberitahuan versi baru (libsoup)
└── minikeyboard/  composition root — satu-satunya tempat yang mengenal semua crate
po/                terjemahan, satu file per bahasa
data/              file .desktop, metainfo, dan ikon, serta aturan udev
LICENSES/          lisensi ikon (CC BY-SA 4.0)
```

Standarnya adalah nol peringatan. Peringatan diperbaiki, bukan dimatikan.

## Batasan

- Sejauh ini, penulisan sudah diperiksa pada dua papan ketik di Fedora: `1189:8890` dengan 6 tombol dan 1 kenop, dan
  `514C:8850` dengan 12 tombol dan 2 kenop, dengan firmware baru, yang dibuka melalui hidraw. Papan ketik lainnya
  ditulis persis seperti cara aplikasi produsennya menulis — perintahnya sama dengan milik aplikasi macOS sampai ke
  byte-nya, dan tes menjaganya — tetapi belum ada satu pun yang diperiksa langsung pada perangkatnya.
- `1189:8890` dan papan ketik yang tidak memberi tahu jenisnya tidak melaporkan pengaturan saat ini, jadi yang terlihat
  hanya apa yang diatur di aplikasi.
- Hanya satu papan ketik yang dibuka dalam satu waktu. Jika beberapa papan ketik tercolok, tombol di samping status
  sambungan memilih yang mana; aplikasi membuka yang terakhir dipilih, atau jika tidak ada, yang pertama ditemukan —
  `1189:8890` didahulukan.
- Gerakan dan seret tetikus tidak tersedia.

## Lisensi

MIT — lihat [LICENSE](../LICENSE). Ikon aplikasi: CC BY-SA 4.0 — lihat
[LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt).
