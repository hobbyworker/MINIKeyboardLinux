<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="App 圖像"></p>

<h1 align="center">Linux 巨集鍵盤設定工具</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <b>繁體中文</b></p>

這是一款將按鍵設定寫入小型 USB 巨集鍵盤的 Linux App。這類鍵盤在市面上從只有一個旋鈕到多達 21 個按鍵都有，旋鈕最多 4 個。可以為每個按鍵和旋鈕的動作（向左旋轉、按下、向右旋轉）在第 1–3 層設定按鍵輸入、多媒體鍵或滑鼠動作，也可以設定鍵盤的燈光。它是依相同規則打造的 macOS App 的 Linux 版，使用相同的指令。

<p align="center"><img src="../.github/devices.png" width="560" alt="兩台小型 USB 巨集鍵盤：6 個按鍵和 1 個旋鈕，12 個按鍵和 2 個旋鈕"></p>

| 鍵盤 | 佈局 | 讀取 | 延遲 | 燈光 | 實機驗證 |
|---|---|---|---|---|---|
| `1189:8890` | 由使用者設定：按鍵（最多 12 個）、每列按鍵數、旋鈕（最多 2 個） | 不支援 | 無 | 三種模式 | 已驗證（6 個按鍵、1 個旋鈕） |
| 廠商 `1189` 或 `514C`，產品 `8840`、`8842`、`8830`–`8833`、`8850`、`8851`，新版韌體 | 依鍵盤告知的佈局（26 種） | 連接後立即讀取 | 每個輸入 | 一種模式，以及按鍵 1–16 的顏色 | 已驗證（`514C:8850`，12 個按鍵、2 個旋鈕） |
| 同類鍵盤，舊版韌體 | 依鍵盤告知的佈局 | 連接後立即讀取 | 每個按鍵一個 | 顏色和模式 | 未驗證 |
| 同類鍵盤，不告知類型者 | 由使用者選擇（26 種） | 不支援 | 無 | 顏色和模式 | 未驗證 |

`1189:8890` 是廠商的 Windows 設定工具「MINI KeyBoard」所支援的鍵盤，其餘則是廠商的跨平台 App「MINI_KEYBOARD」會尋找的鍵盤。本 App 兩種工具的指令都會使用。第二類鍵盤大多會告知自己有幾個按鍵和旋鈕、使用哪種韌體，因此 App 會照樣繪製廠商 App 所繪製的佈局；不告知的鍵盤則採用使用者從這些佈局中選擇的那一種。`1189:8890` 是廠商 App 不認得的鍵盤，它自己的工具也不繪製外形，因此無從得知實際形狀。所以只要設定按鍵和旋鈕的數量以及每列的按鍵數，App 就會照此繪製出整齊的排列。

App 提供 13 種語言：英文、韓文、日文、中文（簡體、繁體）、德文、西班牙文、法文、義大利文、葡萄牙文（巴西）、波蘭文、印尼文和越南文。App 會依照桌面的語言顯示，其他語言則顯示英文。

本儲存庫的原始碼採用 MIT 授權條款。App 圖像（`data/icons/` 中的圖像以及最上方的圖片）採用 CC BY-SA 4.0 授權條款（[`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)）。

## macOS、Windows、Linux 和網頁

MINIKeyboard 有四個依相同規則製作的版本——相同的指令、相同的 13 種語言。三個 App 支援相同的鍵盤；在瀏覽器中執行的網頁版支援其中除 `1189:8890` 以外的鍵盤。

| 儲存庫 | GitHub |
|---|---|
| **MINIKeyboardApple**<br><img alt="macOS 14+" src="https://img.shields.io/badge/macOS-14%2B-0A84FF?logo=apple&logoColor=white"> <img alt="Universal: Apple Silicon + Intel" src="https://img.shields.io/badge/Universal-Apple%20Silicon%20%2B%20Intel-5E5CE6"> <img alt="Swift 6" src="https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardApple](https://github.com/hobbyworker/MINIKeyboardApple) |
| **MINIKeyboardWin**<br><img alt="Windows 10 (2004)+ · 11" src="https://img.shields.io/badge/Windows-10%20(2004)%2B%20·%2011-0078D4?logo=windows&logoColor=white"> <img alt="x64 · ARM64" src="https://img.shields.io/badge/x64%20·%20ARM64-5E5CE6"> <img alt=".NET 10 · WinUI 3" src="https://img.shields.io/badge/.NET%2010-WinUI%203-512BD4?logo=dotnet&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardWin](https://github.com/hobbyworker/MINIKeyboardWin) |
| **MINIKeyboardLinux**<br><img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white"> <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white"> <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardLinux](https://github.com/hobbyworker/MINIKeyboardLinux) |
| **MINIKeyboardWeb**<br><img alt="Chrome · Edge" src="https://img.shields.io/badge/Chrome%20·%20Edge-WebHID-4285F4?logo=googlechrome&logoColor=white"> <img alt="Windows · macOS · Linux" src="https://img.shields.io/badge/Windows%20·%20macOS%20·%20Linux-5E5CE6"> <img alt="TypeScript · Svelte 5" src="https://img.shields.io/badge/TypeScript-Svelte%205-3178C6?logo=typescript&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardWeb](https://github.com/hobbyworker/MINIKeyboardWeb) |

## 下載

可以在軟體中心開啟 [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref)，或是執行：

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

這會加入 App 自己的 Flatpak 儲存庫，第一次安裝時，也會從 Flathub 下載 GNOME 執行環境。此後，軟體中心（或 `flatpak update`）會取得新版本。你的設定會保留下來。

每個版本也以 Flatpak 套件的形式放在 [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases)：Intel 和 AMD 用 `MINIKeyboard-<版本>-x86_64.flatpak`，ARM 用 `-aarch64.flatpak`。適用於沒有網路連線的 PC，或需要特定版本的情況。套件以 `flatpak install --user <檔案>` 安裝，之後的更新會從同一個儲存庫取得。

## 讓 App 能夠開啟鍵盤

在 Linux 上，只有當 udev 規則把 USB 裝置交給已登入的使用者時，App 才能開啟它，而 Flatpak 無法安裝這樣的規則。第一次使用時，若 App 告知需要存取權限，**如何允許…**會顯示寫入該規則的指令和複製按鈕。在終端機中執行一次，然後重新插入鍵盤。若已 clone 本儲存庫，同樣的規則就是 [`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules)。

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- `1189:8890` 直接透過 USB（libusb）寫入。它的設定介面只有一個 OUT 端點，因此不會有任何驅動程式接管它。
- 其餘鍵盤和廠商 App 一樣，透過第一個 USB 介面的 hidraw 裝置寫入。這段期間，鍵盤仍可照常輸入。

## 建置

```sh
make build    # target/release/minikeyboard，供 PREFIX 使用（未指定時為 /usr/local）
make install  # 在 make build 之後：將程式、翻譯、桌面檔案、metainfo、圖像和授權條款安裝到 DESTDIR/PREFIX
make test     # 所有 crate 的測試 — 不會開啟視窗，也不會碰到鍵盤
make check    # rustfmt 和 clippy（警告視為錯誤）
make run      # 附帶翻譯的 Debug 建置
```

需要 Rust 1.88 或以上版本，以及 GTK 4、libadwaita 1.7 或以上版本、libsoup 3 和 libusb 1.0 的開發檔案：Debian 和 Ubuntu 上是 `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`，Fedora 上是 `gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`。翻譯需要 gettext 的 `msgfmt`。

App 的 ID（`me.hobbyworker.MINIKeyboard`）、名稱和連結是 App 自己的，與 `data/` 中的桌面檔案、metainfo 和圖像一致。fork 的專案在建置時提供自己的值：`MINIKEYBOARD_APP_ID`（也是圖像名稱）、`MINIKEYBOARD_APP_NAME`、`MINIKEYBOARD_COPYRIGHT`、`MINIKEYBOARD_WEBSITE`、`MINIKEYBOARD_ISSUE_URL`。翻譯的位置由 `make build` 依 `PREFIX` 決定，執行時的 `MINIKEYBOARD_LOCALE_DIR` 會覆寫它（用於試用建置）。

App 也可以自己告知新版本：提供了 `MINIKEYBOARD_UPDATE_FEED` 的建置會在啟動時和每天一次讀取該 RSS 來源（在偏好設定中關閉**自動檢查更新**後就不會讀取），按一下**檢查更新…**時也會讀取，然後顯示橫幅，引導你前往新版本的下載頁面。本 App 發佈的建置不使用它——更新由 Flatpak 取得——因此既沒有該選單項目，也沒有該設定。

在**模擬器**子選單中，可以在沒有該鍵盤的情況下試用 App。選擇 `1189:8890`、新韌體的佈局，或不告知類型的鍵盤之一，App 就會像鍵盤已經插上一樣，以相同的指令運作，而不會向 USB 送出任何內容。使用期間，視窗頂端的橘色橫條會提示這一點，按下**返回 USB 上的鍵盤**即可返回。Debug 建置提供所有種類（`1189:8890` 的其他報告 ID、舊韌體），設定 `MINIKEYBOARD_SIMULATED_KEYBOARD`（例如 `8890-3`）即可直接以該鍵盤啟動。在**裝置日誌**中可以查看所有寫入的報告和收到的回應。

## 使用方式

1. 用 USB 連接鍵盤。App 會找到鍵盤，並詢問第二類鍵盤的類型；若鍵盤告知了類型，會立即讀取設定。
2. 對於不告知形狀的鍵盤，第一次使用時要設定佈局（廠商 App 的佈局之一；`1189:8890` 則是按鍵、每列按鍵數和旋鈕的數量）以及它擺在你面前的方向。App 會依 USB ID 記住這些設定。之後可以透過**形狀…**、**旋轉**（Ctrl+T）變更。已告知形狀的鍵盤只能旋轉。
3. 選擇層，然後在鍵盤圖上選擇按鍵或旋鈕動作。
4. 指定動作：
   - **按鍵**：按住變更鍵（Ctrl、Alt、Shift、Super 及右側的變更鍵）再按一下按鍵，每按一下就加入一個輸入。**僅變更鍵**會將該按鍵變成變更鍵。**QWERTY** 依全尺寸鍵盤的樣子排列按鍵，**ABC** 則分組排列。只能按一下鍵盤支援的按鍵。
   - **多媒體**、**滑鼠**：取代按鍵原有的動作。
   - **LED**：該層的燈光。新版韌體可以從按鍵 1–16 中選擇一個並為它設定顏色。
   - **延遲**：適用於會記住延遲的韌體。
5. **儲存**（Ctrl+S）會寫入變更的內容。**讀取**（Ctrl+R）會從支援讀取的鍵盤取得目前的設定。

在**偏好設定**（Ctrl+,）中可以選擇介面語言（13 種之一，或跟隨桌面）以及淺色或深色。在有更新來源的建置中，其「一般」頁面有自動檢查更新的選項。在**關於MINIKeyboard**中可以查看版本和開放原始碼授權。

**偏好設定 ▸ 一般 ▸ 韌體問題規避**中的**變更燈光後防止按鍵重複**預設為開啟。`1189:8890` 的韌體在儲存燈光後，會讓按鍵的第一個字元不停重複（廠商軟體也一樣），所以 App 會在燈光之後再送出一次不含按鍵的儲存指令。關閉後，App 只會送出與廠商軟體相同的內容。另外，這款鍵盤拔掉後會忘記燈光設定。

## 架構

規則位於各個 crate 中。相依性只指向內側，向外依賴的層無法通過編譯。

```
crates/
├── domain/        鍵盤、動作、佈局的控制項、各鍵盤可儲存的內容，Port，使用案例（僅標準函式庫）
├── datalayer/     各鍵盤的指令和控制項編號，在 sysfs 中尋找鍵盤以及透過 USB（libusb）、hidraw 寫入，模擬鍵盤
├── presentation/  視圖模型和畫面繪製的值 — 佈局的圖形、按鍵和顏色表、udev 規則（不含工具套件）
├── ui/            GTK 4 和 libadwaita — 只看 presentation，新版本通知（libsoup）
└── minikeyboard/  組裝（composition root）— 唯一知道所有 crate 的地方
po/                翻譯，每種語言一個檔案
data/              桌面檔案、metainfo、圖像，udev 規則
LICENSES/          圖像的授權條款（CC BY-SA 4.0）
```

以零警告為標準。警告要修正，而不是關閉。

## 限制

- 目前已在 Fedora 上用兩台鍵盤確認過寫入：一台有 6 個按鍵、1 個旋鈕的 `1189:8890`，以及一台透過 hidraw 開啟、使用新版韌體、有 12 個按鍵和 2 個旋鈕的 `514C:8850`。其餘鍵盤依廠商 App 的方式寫入；指令與 macOS App 的逐位元組相同，並由測試確保，但還沒有在實機上確認過。
- `1189:8890` 和不告知類型的鍵盤不會告知目前的設定，因此只會顯示在 App 中設定的內容。
- 一次只開啟一個鍵盤。插上多個時，用連線狀態旁的按鈕選擇；App 會開啟上次選擇的那個，否則開啟最先找到的那個（`1189:8890` 優先）。
- 不支援滑鼠移動和拖移。

## 授權條款

MIT — 請參閱 [LICENSE](../LICENSE)。App 圖像採用 CC BY-SA 4.0 — 請參閱 [LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt)。
