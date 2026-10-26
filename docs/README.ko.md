<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="앱 아이콘"></p>

<h1 align="center">리눅스용 매크로 키보드 설정 도구</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <b>한국어</b> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

작은 USB 매크로 키보드에 키 설정을 쓰는 리눅스 앱입니다. 노브 하나뿐인 것부터 키 21개까지, 노브는 4개까지로 팔리는
키보드들입니다. 키와 노브의 동작(왼쪽으로 회전 · 누르기 · 오른쪽으로 회전)마다 레이어 1–3 에 키 입력 · 멀티미디어 키 ·
마우스 동작을 정하고, 키보드의 조명도 정합니다. 같은 규칙으로 만든 macOS 앱의 리눅스 판이고, 같은 명령을 씁니다.

<p align="center"><img src="../.github/devices.png" width="560" alt="작은 USB 매크로 키보드 두 대 — 키 6개와 노브 하나, 키 12개와 노브 둘"></p>

| 키보드 | 배열 | 읽어 오기 | 지연 | 조명 | 실기 확인 |
|---|---|---|---|---|---|
| `1189:8890` | 사용자가 정함 — 키(12개까지) · 한 줄의 키 · 노브(2개까지) | 안 됨 | 없음 | 모드 셋 | 했음 — 키 6개 · 노브 하나 |
| 제조사 `1189` 또는 `514C`, 제품 `8840` · `8842` · `8830`–`8833` · `8850` (그리고 `514C:8851`), 새 펌웨어 | 키보드가 알려 준 대로 — 26가지 | 연결하면 바로 | 입력마다 | 모드 하나와 키 1–16 의 색 | 안 했음 |
| 같은 키보드, 옛 펌웨어 | 키보드가 알려 준 대로 | 요청할 때 | 키마다 하나 | 색과 모드 | 안 했음 |
| 같은 키보드, 종류를 알려 주지 않는 것 | 사용자가 고름 — 26가지 | 안 됨 | 없음 | 색과 모드 | 안 했음 |

`1189:8890` 은 제조사의 Windows 설정 도구 "MINI KeyBoard" 가 다루는 키보드이고, 나머지는 제조사의 여러 운영체제용 앱
"MINI_KEYBOARD" 가 찾는 키보드입니다. 앱은 두 도구의 명령을 모두 씁니다. 두 번째 쪽 키보드는 대개 키와 노브가 몇 개인지,
펌웨어가 무엇인지 알려 주므로 제조사 앱이 그리는 배열을 그대로 그리고, 알려 주지 않는 키보드는 그 배열 가운데 사용자가
고른 것을 씁니다. `1189:8890` 은 제조사 앱이 모르는 키보드이고 그 자신의 도구도 모양을 그리지 않아 실제 모양을 알 수
없습니다. 그래서 키와 노브가 몇 개인지, 한 줄에 키가 몇 개인지를 정하면 앱이 그대로 반듯하게 그립니다.

앱은 13개 언어로 나옵니다 — 영어 · 한국어 · 일본어 · 중국어(간체 · 번체) · 독일어 · 스페인어 · 프랑스어 · 이탈리아어 ·
포르투갈어(브라질) · 폴란드어 · 인도네시아어 · 베트남어. 데스크톱의 언어를 따르고, 그 밖의 언어에서는 영어로 나옵니다.

이 저장소의 소스는 MIT 라이선스입니다. 앱 아이콘 — `data/icons/` 의 것과 맨 위의 그림 — 은 CC BY-SA 4.0 입니다
([`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)).

## macOS · Windows · 리눅스

MINIKeyboard 는 같은 규칙으로 만든 세 판이 있습니다 — 같은 키보드, 같은 명령, 같은 13개 언어.

| 저장소 | GitHub |
|---|---|
| **MINIKeyboardApple**<br><img alt="macOS 14+" src="https://img.shields.io/badge/macOS-14%2B-0A84FF?logo=apple&logoColor=white"> <img alt="Universal: Apple Silicon + Intel" src="https://img.shields.io/badge/Universal-Apple%20Silicon%20%2B%20Intel-5E5CE6"> <img alt="Swift 6" src="https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardApple](https://github.com/hobbyworker/MINIKeyboardApple) |
| **MINIKeyboardWin**<br><img alt="Windows 10 (2004)+ · 11" src="https://img.shields.io/badge/Windows-10%20(2004)%2B%20·%2011-0078D4?logo=windows&logoColor=white"> <img alt="x64 · ARM64" src="https://img.shields.io/badge/x64%20·%20ARM64-5E5CE6"> <img alt=".NET 10 · WinUI 3" src="https://img.shields.io/badge/.NET%2010-WinUI%203-512BD4?logo=dotnet&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardWin](https://github.com/hobbyworker/MINIKeyboardWin) |
| **MINIKeyboardLinux**<br><img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white"> <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white"> <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardLinux](https://github.com/hobbyworker/MINIKeyboardLinux) |

## 내려받기

[me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) 를 소프트웨어 센터에서 열거나,

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

앱의 Flatpak 저장소가 추가되고, 처음 설치할 때는 GNOME 런타임도 Flathub 에서 받아 옵니다. 그 뒤로는 소프트웨어 센터가 —
또는 `flatpak update` 가 — 새 버전을 가져옵니다. 설정은 그대로 남습니다.

판마다 [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) 에도 Flatpak 묶음이 있습니다 — Intel · AMD 는
`MINIKeyboard-<버전>-x86_64.flatpak`, ARM 은 `-aarch64.flatpak`. 인터넷이 없는 PC 나 특정 판이 필요할 때 씁니다. 묶음은
`flatpak install --user <파일>` 로 설치하고, 그 뒤 업데이트는 같은 저장소에서 받습니다.

## 앱이 키보드를 열게 하기

리눅스는 udev 규칙이 로그인한 사용자에게 내어 준 USB 장치만 앱이 열게 하고, Flatpak 은 그 규칙을 설치하지 못합니다. 처음에
앱이 접근 권한이 필요하다고 알리면, **허용하는 방법…** 이 규칙을 쓰는 명령과 복사 단추를 보여 줍니다. 터미널에서 한 번 실행한
뒤 키보드를 다시 꽂습니다. 이 저장소를 클론했다면 같은 규칙이 [`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules)
입니다.

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- `1189:8890` 은 USB 로 직접(libusb) 씁니다. 설정 통로에 OUT 엔드포인트 하나뿐이라 어떤 드라이버도 붙지 않습니다.
- 나머지 키보드는 제조사 앱처럼 첫 번째 USB 인터페이스의 hidraw 장치로 씁니다. 그동안에도 키보드는 그대로 입력됩니다.

## 빌드

```sh
make build    # target/release/minikeyboard, PREFIX 용 (주지 않으면 /usr/local)
make install  # make build 뒤에: 프로그램 · 번역 · 데스크톱 파일 · metainfo · 아이콘 · 라이선스를 DESTDIR/PREFIX 로
make test     # 모든 크레이트의 테스트 — 창이 뜨지 않고 키보드도 건드리지 않는다
make check    # rustfmt 와 clippy, 경고는 에러
make run      # 번역을 갖춘 Debug 빌드
```

Rust 1.88 이상과 GTK 4 · libadwaita 1.7 이상 · libsoup 3 · libusb 1.0 의 개발 파일이 필요합니다 — Debian · Ubuntu 는
`libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`, Fedora 는 `gtk4-devel libadwaita-devel libsoup3-devel
libusb1-devel`. 번역에는 gettext 의 `msgfmt` 가 필요합니다.

앱의 ID(`me.hobbyworker.MINIKeyboard`) · 이름 · 링크는 앱 자신의 것이고, `data/` 의 데스크톱 파일 · metainfo · 아이콘과
맞습니다. 포크는 빌드할 때 자기 것을 줍니다 — `MINIKEYBOARD_APP_ID`(아이콘 이름이기도 합니다) · `MINIKEYBOARD_APP_NAME` ·
`MINIKEYBOARD_COPYRIGHT` · `MINIKEYBOARD_WEBSITE` · `MINIKEYBOARD_ISSUE_URL`. 번역을 찾는 자리는 `make build` 가 `PREFIX` 에서
정하고, 실행할 때의 `MINIKEYBOARD_LOCALE_DIR` 가 그것을 덮어씁니다(빌드를 써 볼 때).

앱이 새 버전을 직접 알려 줄 수도 있습니다. `MINIKEYBOARD_UPDATE_FEED` 를 준 빌드는 실행할 때와 하루에 한 번 그 RSS 피드를
읽고 — 기본 설정에서 **업데이트 자동 확인** 을 끄면 읽지 않습니다 — **업데이트 확인…** 을 누를 때도 읽어서, 새 버전의 내려받기
쪽으로 이어 주는 배너를 띄웁니다. 앱이 내는 빌드는 이것을 쓰지 않고 — 업데이트는 Flatpak 이 가져옵니다 — 그 메뉴 항목도 설정도
없습니다.

**시뮬레이터** 하위 메뉴로 그 키보드 없이 앱을 써 볼 수 있습니다. `1189:8890`, 새 펌웨어의 배열들, 종류를 알려 주지 않는
키보드 가운데 하나를 고르면 앱이 꽂힌 것처럼 같은 명령으로 동작하고, USB 로는 아무것도 나가지 않습니다. 쓰는 동안은 창 위의
주황 띠가 그렇다고 알리고, **USB에 꽂힌 키보드로 돌아가기** 로 돌아옵니다. Debug 빌드는 모든 종류(`1189:8890` 의 다른 리포트
ID · 옛 펌웨어)를 고를 수 있고, `MINIKEYBOARD_SIMULATED_KEYBOARD`(예: `8890-3`)로 그 키보드로 시작합니다. **장치 기록** 에서
쓴 리포트와 받은 답을 모두 볼 수 있습니다.

## 쓰는 법

1. 키보드를 USB 로 연결합니다. 앱이 키보드를 찾고, 두 번째 쪽 키보드에는 종류를 묻습니다. 새 펌웨어면 바로 읽어 옵니다.
2. 모양을 알려 주지 않는 키보드는 처음에 배열 — 제조사 앱의 배열 가운데 하나, `1189:8890` 은 키 · 한 줄의 키 · 노브의
   개수 — 과 앞에 놓인 방향을 정합니다. 앱은 USB ID 마다 기억합니다. 나중에 **모양…** 과 **회전**(Ctrl+T)으로 바꿉니다.
   모양을 알려 준 키보드는 돌리기만 합니다.
3. 레이어를 고르고, 키보드 그림에서 키나 노브 동작을 고릅니다.
4. 동작을 줍니다.
   - **키** — 보조 키(Ctrl · Alt · Shift · Super 와 오른쪽 것들)를 잡고 키를 누릅니다. 누를 때마다 입력이 하나씩 붙습니다.
     **보조 키만** 은 그 키를 보조 키로 만듭니다. **QWERTY** 는 키를 풀사이즈 키보드처럼, **ABC** 는 묶음으로 늘어놓습니다.
     키보드가 받는 키만 누를 수 있습니다.
   - **멀티미디어** · **마우스** — 키에 있던 동작을 대신합니다.
   - **LED** — 레이어의 조명. 새 펌웨어는 키 1–16 가운데 하나를 골라 색을 줍니다.
   - **지연** — 지연을 기억하는 펌웨어에서.
5. **저장**(Ctrl+S)이 바뀐 것을 씁니다. **읽기**(Ctrl+R)는 읽어 올 수 있는 키보드에서 지금 설정을 가져옵니다.

**기본 설정**(Ctrl+,)에서 화면 언어(13개 가운데 하나, 또는 데스크톱의 언어)와 라이트 · 다크를 고릅니다. 업데이트 피드가 있는
빌드는 그 일반 쪽에 업데이트 자동 확인이 있습니다. **MINIKeyboard 정보** 에서 버전과 오픈소스 라이선스를 봅니다.

**기본 설정 ▸ 일반 ▸ 펌웨어 버그 우회** 의 **조명을 바꾼 뒤 키 반복 막기** 는 처음부터 켜져 있습니다.
`1189:8890` 은 조명을 저장하면 그 뒤로 키의 첫 입력이 끝없이 반복되는 펌웨어 버그가 있어서(제조사 프로그램도
같습니다), 앱이 조명 뒤에 키 없는 저장을 한 번 더 보냅니다. 끄면 제조사 프로그램과 똑같이 보냅니다. 이 키보드는 뽑았다
꽂으면 조명이 꺼집니다.

## 구조

규칙은 크레이트에 있습니다. 의존은 안쪽으로만 향하고, 바깥으로 손을 뻗는 레이어는 컴파일되지 않습니다.

```
crates/
├── domain/        키보드 · 동작 · 배열의 컨트롤 · 키보드마다 담을 수 있는 것, 포트, 유스케이스 (표준 라이브러리만)
├── datalayer/     키보드마다의 명령과 컨트롤의 번호, sysfs 에서 키보드 찾기와 USB(libusb) · hidraw 로 쓰기, 시뮬레이터 키보드
├── presentation/  뷰모델과 화면이 그리는 값 — 배열의 그림, 키 · 색 표, udev 규칙 (툴킷 없음)
├── ui/            GTK 4 와 libadwaita — presentation 만 본다. 새 버전 알림(libsoup)
└── minikeyboard/  조립(composition root) — 모든 크레이트를 아는 유일한 곳
po/                번역, 언어마다 파일 하나
data/              데스크톱 파일 · metainfo · 아이콘, udev 규칙
LICENSES/          아이콘의 라이선스 (CC BY-SA 4.0)
```

경고 0 이 기준입니다. 경고는 끄지 않고 고칩니다.

## 한계

- 쓰기는 지금까지 키보드 한 대, 키 6개 · 노브 하나의 `1189:8890` 으로 Fedora 에서 확인했습니다. 나머지 키보드는 제조사 앱이
  쓰는 대로 씁니다 — 명령은 macOS 앱의 것과 바이트까지 같고 테스트가 지키지만, 실기로는 아직 하나도 확인하지 못했고 그 길
  (hidraw)도 아직입니다.
- `1189:8890` 과 종류를 알려 주지 않는 키보드는 지금 설정을 알려 주지 않으므로, 앱에서 정한 것만 보입니다.
- 한 번에 키보드 하나. 여럿을 꽂으면 먼저 찾은 것 — `1189:8890` 이 먼저 — 을 씁니다.
- 마우스 이동과 끌기는 없습니다.

## 라이선스

MIT — [LICENSE](../LICENSE) 를 보세요. 앱 아이콘은 CC BY-SA 4.0 — [LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt) 를 보세요.
