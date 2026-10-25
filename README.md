<p align="center"><img src=".github/icon.png" width="128" height="128" alt="App icon"></p>

<h1 align="center">Macro keyboard configurator for Linux</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><b>English</b> · <a href="docs/README.ko.md">한국어</a> · <a href="docs/README.de.md">Deutsch</a> · <a href="docs/README.es.md">Español</a> · <a href="docs/README.fr.md">Français</a> · <a href="docs/README.id.md">Bahasa Indonesia</a> · <a href="docs/README.it.md">Italiano</a> · <a href="docs/README.ja.md">日本語</a> · <a href="docs/README.pl.md">Polski</a> · <a href="docs/README.pt-BR.md">Português (Brasil)</a> · <a href="docs/README.vi.md">Tiếng Việt</a> · <a href="docs/README.zh-Hans.md">简体中文</a> · <a href="docs/README.zh-Hant.md">繁體中文</a></p>

A Linux app that writes key settings to small USB macro keyboards — the ones sold with anything from a single
knob to 21 keys, and up to four knobs. For each key and knob action (rotate left, press, rotate right) it sets
key presses, a media key or a mouse action on layers 1–3, and the keyboard's lighting. It is the Linux
counterpart of a macOS app built on the same rules, and speaks the same commands.

| Keyboard | Layout | Reads back | Waits | Lighting | Tried |
|---|---|---|---|---|---|
| `1189:8890` | arranged by you — keys (up to 12), keys per row, knobs (up to 2) | no | no | three modes | yes — 6 keys and a knob |
| vendor `1189` or `514C`, product `8840` · `8842` · `8830`–`8833` · `8850` (and `514C:8851`), newer firmware | from what the keyboard says — 26 layouts | when connected | before each press | a mode, and a colour for each of keys 1–16 | no |
| the same, older firmware | from what the keyboard says | when you ask | one per key | a colour and a mode | no |
| the same, not saying what it is | chosen by you — 26 layouts | no | no | a colour and a mode | no |

The `1189:8890` is the keyboard the vendor's Windows tool "MINI KeyBoard" works with; the others are the ones the
vendor's cross-platform app "MINI_KEYBOARD" looks for. The app speaks both tools' commands. Most of the second
kind say how many keys and knobs they have and which firmware they run, and the app draws the layout the vendor's
app would; one that doesn't say gets the one of those layouts you pick. The vendor's app doesn't know the
`1189:8890` and its own tool draws no shape, so its real shape isn't known: you tell the app how many keys and
knobs it has and how many keys are in a row, and it draws a tidy arrangement of that.

The app is available in 13 languages — English, Korean, Japanese, Chinese (Simplified and Traditional), German,
Spanish, French, Italian, Portuguese (Brazil), Polish, Indonesian and Vietnamese — and follows the language of the
desktop. Other languages get English.

The source in this repository is MIT licensed. The app icon — in `data/icons/`, and the picture at the top — is
licensed under CC BY-SA 4.0 ([`LICENSES/CC-BY-SA-4.0.txt`](LICENSES/CC-BY-SA-4.0.txt)).

## Download

Open [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) in your software center, or run:

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

It adds the app's own Flatpak repository, and the first install also fetches the GNOME runtime from Flathub. From then
on your software center — or `flatpak update` — brings new versions. Your settings stay.

Each version is also on the [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) page as a Flatpak
bundle — `MINIKeyboard-<version>-x86_64.flatpak` for Intel and AMD, `-aarch64.flatpak` for ARM — for a PC without a
connection, or for a particular version. A bundle installs with `flatpak install --user <file>` and then takes its
updates from the same repository.

## Letting the app open the keyboard

Linux lets an app open a USB device only when a udev rule gives it to the signed-in user, and a Flatpak can't
install one. The first time, the app says that it needs access: **How to Allow…** shows the commands that write
the rule, with a button to copy them. Run them in a terminal once, then plug the keyboard in again. From a clone
of this repository, the same rule is [`data/60-minikeyboard.rules`](data/60-minikeyboard.rules):

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- The `1189:8890` is written over USB directly (libusb): its configuration interface has only an OUT endpoint, so
  no driver takes it.
- The others are written through the hidraw device of their first USB interface, as the vendor's app does. The
  keyboard keeps typing meanwhile.

## Building

```sh
make build    # target/release/minikeyboard, for PREFIX (/usr/local unless given)
make install  # after make build: the program, translations, desktop file, metainfo, icons and licences, into DESTDIR/PREFIX
make test     # every crate's tests — no window opens, no keyboard is touched
make check    # rustfmt and clippy; warnings are errors
make run      # a debug build with its translations
```

It needs Rust 1.88 or later and the development files of GTK 4, libadwaita 1.7 or later, libsoup 3 and libusb 1.0 —
on Debian and Ubuntu `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`, on Fedora `gtk4-devel
libadwaita-devel libsoup3-devel libusb1-devel` — and gettext's `msgfmt` for the translations.

The app's ID (`me.hobbyworker.MINIKeyboard`), name and links are its own and match the desktop file, metainfo and
icons in `data/`. A fork gives its own at build time: `MINIKEYBOARD_APP_ID` (also the icon's name),
`MINIKEYBOARD_APP_NAME`, `MINIKEYBOARD_COPYRIGHT`, `MINIKEYBOARD_WEBSITE` and `MINIKEYBOARD_ISSUE_URL`. `make build`
takes where the translations are from `PREFIX`; `MINIKEYBOARD_LOCALE_DIR` at run time overrides it, for trying a build.

The app can also tell about new versions itself: a build given `MINIKEYBOARD_UPDATE_FEED` reads that RSS feed at
launch and once a day — unless **Check for updates automatically** is switched off in Preferences — and on
**Check for Updates…**, and shows a banner that links to the new version's download page. The app's own builds don't
use it — Flatpak brings their updates — so they have no such menu item or setting.

Debug builds have a **Simulator** submenu for trying the app without the keyboard in question: pick a `1189:8890`
with any report ID, a keyboard of any layout on either firmware, or one that doesn't say what it is, and the app
acts as if it were plugged in — through the same commands, nothing going to USB. `MINIKEYBOARD_SIMULATED_KEYBOARD`
(for example `8890-3`) starts on one. **Device Log** shows every report written and every answer.

## Using it

1. Connect the keyboard over USB. The app finds it, and asks a keyboard of the second kind what it is; one with
   newer firmware is read at once.
2. A keyboard that doesn't say its shape asks for its layout the first time — one of the vendor app's layouts, or
   for the `1189:8890` its keys, keys per row and knobs — and how it sits in front of you. The app remembers both
   for that USB ID. **Shape…** and **Rotate** (Ctrl+T) change them later; a keyboard that said its shape only
   turns.
3. Pick a layer, then the key or knob action on the keyboard.
4. Give it an action:
   - **Keys** — hold modifiers (Ctrl, Alt, Shift, Super, and the right-hand ones), then click keys; each click adds
     a press. **Modifiers Only** turns the key into a modifier key. **QWERTY** lays the keys out as on a full-size
     keyboard, **ABC** in groups; only the keys the keyboard takes can be clicked.
   - **Multimedia**, **Mouse** — replace what the key did.
   - **LED** — the layer's lights; on newer firmware, select one of keys 1–16 to give it a colour.
   - **Delay** — on firmware that keeps waits.
5. **Save** (Ctrl+S) writes what changed. **Read** (Ctrl+R) asks a keyboard that reads back what it holds.

**Preferences** (Ctrl+,) picks the interface language (one of the 13, or the desktop's) and light or dark; in a
build with an update feed, its General page has the automatic update check. **About MINIKeyboard** has the
version and the open source licenses.

**Preferences ▸ General ▸ Keyboard** has **Prevent key repeat after changing the lighting**, on by default. After its
lighting is saved, the `1189:8890`'s firmware repeats a key's first stroke without end — its maker's software does the
same — so the app sends one more save, with no key in it, after the lighting. Off, the app sends just what the maker's
software sends. That keyboard also forgets its lighting when it's unplugged.

## Structure

The rules live in crates whose dependencies point inwards — a layer that reaches outwards doesn't compile.

```
crates/
├── domain/        keyboards, actions, the controls of each layout and what each keyboard can hold; the ports; use cases (the standard library only)
├── datalayer/     the commands of each keyboard and the numbers its controls go by; finding it in sysfs and writing over USB (libusb) or hidraw; simulated keyboards
├── presentation/  view models and the values the screens draw — the layouts' geometry, key and colour tables, the udev rule (no toolkit)
├── ui/            GTK 4 and libadwaita — sees the presentation only; the new-version notice (libsoup)
└── minikeyboard/  the composition root: the only crate that sees them all
po/                translations, one file per language
data/              the desktop file, metainfo and icons, and the udev rule
LICENSES/          the icon's licence (CC BY-SA 4.0)
```

Zero warnings is the bar. Warnings are fixed, not switched off.

## Limitations

- Writing has been tried on one keyboard so far, a `1189:8890` with six keys and a knob, on Fedora. The other
  keyboards are written as their vendor's app writes them — the commands are the macOS app's, byte for byte, and
  tests pin them — but none has been tried yet, and neither has the way to them (hidraw).
- The `1189:8890`, and keyboards that don't say what they are, can't report their settings, so the app shows only
  what was set in it.
- One keyboard at a time. With several plugged in, the app takes the first it finds, a `1189:8890` first.
- Mouse movement and dragging are not offered.

## License

MIT — see [LICENSE](LICENSE). The app icon: CC BY-SA 4.0 — see [LICENSES/CC-BY-SA-4.0.txt](LICENSES/CC-BY-SA-4.0.txt).
