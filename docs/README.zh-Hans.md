<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="应用图标"></p>

<h1 align="center">Linux 宏键盘设置工具</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.vi.md">Tiếng Việt</a> · <b>简体中文</b> · <a href="README.zh-Hant.md">繁體中文</a></p>

这是一款向小型 USB 宏键盘写入按键设置的 Linux 应用。这类键盘在售的型号从只有一个旋钮到多达 21 个按键不等，旋钮最多 4 个。对于每个按键和旋钮的动作（向左旋转、按下、向右旋转），可以在第 1–3 层设置按键输入、多媒体键或鼠标动作，还可以设置键盘的灯光。它是按同样规则打造的 macOS 应用的 Linux 版，使用相同的命令。

| 键盘 | 布局 | 读取 | 延迟 | 灯光 | 实机验证 |
|---|---|---|---|---|---|
| `1189:8890` | 由用户设定：按键（最多 12 个）、每行按键数、旋钮（最多 2 个） | 不支持 | 无 | 三种模式 | 已验证（6 个按键、1 个旋钮） |
| 厂商 `1189` 或 `514C`，产品 `8840`、`8842`、`8830`–`8833`、`8850`（以及 `514C:8851`），新版固件 | 按键盘告知的布局（26 种） | 连接后立即读取 | 每个输入 | 一种模式，以及按键 1–16 的颜色 | 未验证 |
| 同类键盘，旧版固件 | 按键盘告知的布局 | 请求时 | 每个按键一个 | 颜色和模式 | 未验证 |
| 同类键盘，不告知类型的 | 由用户选择（26 种） | 不支持 | 无 | 颜色和模式 | 未验证 |

`1189:8890` 是厂商的 Windows 设置工具“MINI KeyBoard”所支持的键盘，其余则是厂商的跨平台应用“MINI_KEYBOARD”会查找的键盘。本应用同时使用这两个工具的命令。第二类键盘大多会告知自己有几个按键和旋钮、使用什么固件，因此应用会照原样绘制厂商应用所绘制的布局；不告知的键盘则采用用户从这些布局中选择的那一种。`1189:8890` 是厂商应用不认识的键盘，它自己的工具也不绘制外形，因此无法得知其实际形状。所以只要设定按键和旋钮的数量以及每行的按键数，应用就会照此绘制出整齐的排列。

应用提供 13 种语言：英语、韩语、日语、中文（简体、繁体）、德语、西班牙语、法语、意大利语、葡萄牙语（巴西）、波兰语、印度尼西亚语和越南语。应用跟随桌面的语言，其他语言则显示为英语。

本仓库的源代码采用 MIT 许可证。应用图标（`data/icons/` 中的图标以及顶部的图片）采用 CC BY-SA 4.0 许可证（[`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)）。

## 下载

可以在软件中心中打开 [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref)，或者运行：

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

这会添加应用自己的 Flatpak 仓库，首次安装时还会从 Flathub 获取 GNOME 运行时。此后，软件中心（或 `flatpak update`）会获取新版本。你的设置会保留。

每个版本也以 Flatpak 包的形式发布在 [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases)：Intel 和 AMD 用 `MINIKeyboard-<版本>-x86_64.flatpak`，ARM 用 `-aarch64.flatpak`。适用于没有网络连接的 PC，或需要特定版本的情况。包用 `flatpak install --user <文件>` 安装，之后的更新从同一个仓库获取。

## 让应用能够打开键盘

在 Linux 上，只有当 udev 规则把 USB 设备交给已登录的用户时，应用才能打开它，而 Flatpak 无法安装这样的规则。首次使用时，如果应用提示需要访问权限，**如何允许…**会显示写入该规则的命令和复制按钮。在终端中运行一次，然后重新插入键盘。如果克隆了本仓库，同样的规则就是 [`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules)。

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- `1189:8890` 直接通过 USB（libusb）写入。它的设置接口只有一个 OUT 端点，因此没有任何驱动会接管它。
- 其余键盘与厂商应用一样，通过第一个 USB 接口的 hidraw 设备写入。写入期间，键盘仍可照常输入。

## 构建

```sh
make build    # target/release/minikeyboard，用于 PREFIX（未指定时为 /usr/local）
make install  # 在 make build 之后：把程序、翻译、桌面文件、metainfo、图标和许可证安装到 DESTDIR/PREFIX
make test     # 所有 crate 的测试 — 不会打开窗口，也不会触碰键盘
make check    # rustfmt 和 clippy（警告视为错误）
make run      # 带翻译的 Debug 构建
```

需要 Rust 1.88 或更高版本，以及 GTK 4、libadwaita 1.7 或更高版本、libsoup 3 和 libusb 1.0 的开发文件：Debian 和 Ubuntu 上是 `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`，Fedora 上是 `gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`。翻译需要 gettext 的 `msgfmt`。

应用的 ID（`me.hobbyworker.MINIKeyboard`）、名称和链接是应用自己的，与 `data/` 中的桌面文件、metainfo 和图标一致。复刻版本在构建时提供自己的值：`MINIKEYBOARD_APP_ID`（也是图标名称）、`MINIKEYBOARD_APP_NAME`、`MINIKEYBOARD_COPYRIGHT`、`MINIKEYBOARD_WEBSITE`、`MINIKEYBOARD_ISSUE_URL`。翻译的位置由 `make build` 根据 `PREFIX` 确定，运行时的 `MINIKEYBOARD_LOCALE_DIR` 会覆盖它（用于试用构建）。

应用也可以自己通知新版本：提供了 `MINIKEYBOARD_UPDATE_FEED` 的构建会在启动时和每天一次读取该 RSS 源（在首选项中关闭**自动检查更新**后则不读取），点击**检查更新…**时也会读取，然后显示一条横幅，引导你前往新版本的下载页面。本应用发布的构建不使用它——更新由 Flatpak 获取——因此既没有该菜单项，也没有该设置。

Debug 构建中有**模拟器**子菜单，可以在没有对应键盘的情况下试用应用。选择任一报告 ID 的 `1189:8890`、两种固件的任一布局，或者不告知类型的键盘中的一个，应用就会像键盘已经插上一样，用同样的命令运行，而不会向 USB 发送任何内容。设置 `MINIKEYBOARD_SIMULATED_KEYBOARD`（例如 `8890-3`）即可直接以该键盘启动。在**设备日志**中可以查看写入的所有报告和收到的所有回复。

## 使用方法

1. 通过 USB 连接键盘。应用会找到键盘，并询问第二类键盘的类型；如果是新版固件，会立即读取设置。
2. 对于不告知形状的键盘，首次使用时要设定布局（厂商应用的布局之一；`1189:8890` 则是按键、每行按键数和旋钮的数量）以及它在你面前的摆放方向。应用会按 USB ID 记住这些设置。之后可以通过**形状…**、**旋转**（Ctrl+T）更改。已告知形状的键盘只能旋转。
3. 选择层，然后在键盘图上选择按键或旋钮动作。
4. 指定动作：
   - **按键**：按住修饰键（Ctrl、Alt、Shift、Super 以及右侧的修饰键）再点击按键，每点击一次就添加一个输入。**仅修饰键**会把该按键变成修饰键。**QWERTY** 按全尺寸键盘的样子排列按键，**ABC** 则分组排列。只能点击键盘支持的按键。
   - **多媒体**、**鼠标**：替换按键原有的动作。
   - **LED**：该层的灯光。新版固件可以从按键 1–16 中选择一个并为其设置颜色。
   - **延迟**：适用于能记住延迟的固件。
5. **保存**（Ctrl+S）会写入更改的内容。**读取**（Ctrl+R）会从支持读取的键盘获取当前设置。

在**首选项**（Ctrl+,）中可以选择界面语言（13 种之一，或跟随桌面）以及浅色或深色外观。在有更新源的构建中，其“通用”页面中有自动检查更新的选项。在**关于MINIKeyboard**中可以查看版本和开源许可证。

**首选项 ▸ 通用 ▸ 键盘**中的**更改灯光后防止按键重复**默认开启。`1189:8890` 的固件在保存灯光后会让按键的第一个字符不停重复（厂商软件也是如此），所以应用会在灯光之后再发送一次不含按键的保存命令。关闭后，应用只发送与厂商软件相同的内容。另外，这款键盘拔下后会忘记灯光设置。

## 结构

规则位于各个 crate 中。依赖只指向内侧，向外依赖的层无法通过编译。

```
crates/
├── domain/        键盘、动作、布局的控件、每种键盘能存储的内容，端口，用例（仅标准库）
├── datalayer/     各键盘的命令与控件编号，在 sysfs 中查找键盘以及通过 USB（libusb）、hidraw 写入，模拟键盘
├── presentation/  视图模型和界面绘制的值 — 布局的图形、按键和颜色表、udev 规则（不含工具包）
├── ui/            GTK 4 和 libadwaita — 只看 presentation，新版本通知（libsoup）
└── minikeyboard/  组装（composition root）— 唯一知道所有 crate 的地方
po/                翻译，每种语言一个文件
data/              桌面文件、metainfo、图标，udev 规则
LICENSES/          图标的许可证（CC BY-SA 4.0）
```

以零警告为标准。警告要修复，而不是关闭。

## 限制

- 目前只在 Fedora 上用一台键盘验证过写入：一台有 6 个按键、1 个旋钮的 `1189:8890`。其余键盘按厂商应用的方式写入；命令与 macOS 应用的逐字节相同，并由测试保证，但还没有在任何一台实机上验证过，通往它们的途径（hidraw）也尚未验证。
- `1189:8890` 和不告知类型的键盘不会告知当前设置，因此只会显示在应用中设定的内容。
- 一次只能使用一个键盘。插入多个时，使用最先找到的那个（`1189:8890` 优先）。
- 不支持鼠标移动和拖动。

## 许可证

MIT — 参见 [LICENSE](../LICENSE)。应用图标采用 CC BY-SA 4.0 — 参见 [LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt)。
