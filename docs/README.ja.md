<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="アプリアイコン"></p>

<h1 align="center">Linux用マクロキーボード設定ツール</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <b>日本語</b> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

小さなUSBマクロキーボードにキー設定を書き込むLinuxアプリです。ノブが1つだけのものからキーが21個あるものまで、ノブは最大4つという構成で売られているキーボードが対象です。キーとノブの操作（反時計回りに回す・押す・時計回りに回す）ごとに、レイヤー1–3にキー入力・マルチメディアキー・マウス操作を割り当て、キーボードのライティングも設定します。同じルールで作られたmacOSアプリのLinux版で、同じコマンドを使います。

| キーボード | 配列 | 読み込み | 遅延 | ライティング | 実機での確認 |
|---|---|---|---|---|---|
| `1189:8890` | ユーザーが決める：キー（最大12個）・1列のキー数・ノブ（最大2個） | 不可 | なし | モード3つ | 済み（キー6個・ノブ1つ） |
| ベンダー`1189`または`514C`、製品`8840`・`8842`・`8830`–`8833`・`8850`（と`514C:8851`）、新しいファームウェア | キーボードが知らせるとおり（26種類） | 接続時にすぐ | 入力ごと | モード1つとキー1–16の色 | 未確認 |
| 同じキーボード、旧ファームウェア | キーボードが知らせるとおり | 要求したとき | キーごとに1つ | 色とモード | 未確認 |
| 同じキーボードで、種類を知らせないもの | ユーザーが選ぶ（26種類） | 不可 | なし | 色とモード | 未確認 |

`1189:8890`はメーカーのWindows用設定ツール「MINI KeyBoard」が扱うキーボードで、それ以外はメーカーのマルチプラットフォームアプリ「MINI_KEYBOARD」が探すキーボードです。このアプリは両方のツールのコマンドを使います。2つ目のグループのキーボードはたいてい、キーとノブの数やファームウェアの種類を知らせてくるので、メーカーのアプリが描く配列をそのまま描きます。知らせてこないキーボードには、それらの配列の中からユーザーが選んだものを使います。`1189:8890`はメーカーのアプリが知らないキーボードで、専用のツールも形を描かないため、実際の形はわかりません。そこで、キーとノブの数と1列のキー数を決めると、アプリがそのとおりに整った形で描きます。

アプリは13の言語で表示されます：英語・韓国語・日本語・中国語（簡体字・繁体字）・ドイツ語・スペイン語・フランス語・イタリア語・ポルトガル語（ブラジル）・ポーランド語・インドネシア語・ベトナム語。デスクトップの言語に従い、それ以外の言語では英語で表示されます。

このリポジトリのソースはMITライセンスです。アプリアイコン（`data/icons/`のものと冒頭の画像）はCC BY-SA 4.0です（[`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)）。

## ダウンロード

[me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref)をソフトウェアセンターで開くか、次のコマンドを実行します。

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

アプリ独自のFlatpakリポジトリが追加され、初回のインストールではGNOMEランタイムもFlathubから取得します。その後は、ソフトウェアセンター（または`flatpak update`）が新しいバージョンを取得します。設定はそのまま残ります。

各バージョンは[Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases)にもFlatpakバンドルとして置いてあります。Intel・AMDは`MINIKeyboard-<バージョン>-x86_64.flatpak`、ARMは`-aarch64.flatpak`です。インターネットにつながっていないPCや、特定のバージョンが必要なときに使います。バンドルは`flatpak install --user <ファイル>`でインストールし、その後のアップデートは同じリポジトリから受け取ります。

## アプリがキーボードを開けるようにする

Linuxでは、udevルールがログイン中のユーザーに与えたUSBデバイスしかアプリは開けません。また、Flatpakはそのルールをインストールできません。最初に、キーボードへのアクセスが必要だとアプリが知らせてきます。**許可する方法…**を開くと、ルールを作成するコマンドとコピーボタンが表示されます。ターミナルで一度実行してから、キーボードを接続し直します。このリポジトリをクローンしている場合は、同じルールが[`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules)にあります。

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- `1189:8890`にはUSBで直接（libusb）書き込みます。設定用インターフェイスにはOUTエンドポイントが1つしかないため、どのドライバーも割り当てられません。
- それ以外のキーボードには、メーカーのアプリと同じく1つ目のUSBインターフェイスのhidrawデバイスを通して書き込みます。その間もキーボードはいつもどおり入力できます。

## ビルド

```sh
make build    # target/release/minikeyboard、PREFIX用（指定しなければ/usr/local）
make install  # make buildのあと：プログラム・翻訳・デスクトップファイル・metainfo・アイコン・ライセンスをDESTDIR/PREFIXへ
make test     # すべてのクレートのテスト — ウィンドウは開かず、キーボードにも触れない
make check    # rustfmtとclippy（警告はエラー扱い）
make run      # 翻訳付きのDebugビルド
```

Rust 1.88以降と、GTK 4・libadwaita 1.7以降・libsoup 3・libusb 1.0の開発用ファイルが必要です。Debian・Ubuntuでは`libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`、Fedoraでは`gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`です。翻訳にはgettextの`msgfmt`が必要です。

アプリのID（`me.hobbyworker.MINIKeyboard`）・名前・リンクはアプリ自身のもので、`data/`のデスクトップファイル・metainfo・アイコンと一致しています。フォークでは、ビルド時に独自の値を指定します：`MINIKEYBOARD_APP_ID`（アイコン名も兼ねます）・`MINIKEYBOARD_APP_NAME`・`MINIKEYBOARD_COPYRIGHT`・`MINIKEYBOARD_WEBSITE`・`MINIKEYBOARD_ISSUE_URL`。翻訳の場所は`make build`が`PREFIX`から決め、実行時に`MINIKEYBOARD_LOCALE_DIR`を指定するとそれを上書きします（ビルドを試すときに）。

アプリが新しいバージョンを自分で知らせることもできます。`MINIKEYBOARD_UPDATE_FEED`を指定したビルドは、起動時と1日1回そのRSSフィードを読み込み（設定で**アップデートを自動的に確認**をオフにすると読み込みません）、**アップデートを確認…**を押したときにも読み込んで、新しいバージョンのダウンロードページへ案内するバナーを表示します。このアプリが配布するビルドはこれを使わず（アップデートはFlatpakが取得します）、そのメニュー項目も設定もありません。

**シミュレータ**サブメニューで、対象のキーボードがなくてもアプリを試せます。`1189:8890`、新しいファームウェアの配列、種類を知らせないキーボードのいずれかを選ぶと、アプリは接続されているかのように同じコマンドで動作し、USBには何も送りません。使っている間はウインドウ上部のオレンジ色の帯がそのことを知らせ、**USB に接続したキーボードに戻る**で元に戻ります。Debugビルドではすべての種類（`1189:8890`のほかのレポートID、古いファームウェア）を選べ、`MINIKEYBOARD_SIMULATED_KEYBOARD`（例：`8890-3`）を指定すると、そのキーボードで起動します。**デバイスログ**で、書き込んだレポートと受け取った応答をすべて確認できます。

## 使い方

1. キーボードをUSBで接続します。アプリがキーボードを見つけ、2つ目のグループのキーボードには種類を問い合わせます。新しいファームウェアであれば、すぐに設定を読み込みます。
2. 形を知らせないキーボードでは、最初に配列（メーカーのアプリの配列のいずれか。`1189:8890`ではキー・1列のキー・ノブの数）と、手前に置いた向きを決めます。アプリはUSB IDごとにこれを記憶します。あとから**形…**、**回転**（Ctrl+T）で変更できます。形を知らせたキーボードは回転だけができます。
3. レイヤーを選び、キーボードの図でキーやノブの操作を選びます。
4. 操作を割り当てます。
   - **キー**：修飾キー（Ctrl・Alt・Shift・Superと右側のもの）を押したままキーをクリックします。クリックするたびに入力が1つずつ追加されます。**修飾キーのみ**は、そのキーを修飾キーにします。**QWERTY**はキーをフルサイズキーボードのように、**ABC**はグループごとに並べます。クリックできるのは、キーボードが受け付けるキーだけです。
   - **マルチメディア**・**マウス**：キーにあった操作を置き換えます。
   - **LED**：レイヤーのライティングです。新しいファームウェアでは、キー1–16のいずれかを選んで色を付けます。
   - **遅延**：遅延を記憶するファームウェアで使えます。
5. **保存**（Ctrl+S）で変更を書き込みます。**読み込む**（Ctrl+R）は、読み込みに対応したキーボードから現在の設定を取得します。

**設定**（Ctrl+,）では、表示言語（13言語のいずれか、またはデスクトップの言語）とライト・ダークを選びます。アップデートのフィードがあるビルドでは、その「一般」ページでアップデートの自動確認を設定できます。**MINIKeyboardについて**で、バージョンとオープンソースライセンスを確認できます。

**設定 ▸ 一般 ▸ ファームウェアの不具合への対処**の**ライティング変更後のキーの連続入力を防ぐ**は、初期状態でオンです。`1189:8890` のファームウェアには、ライティングを保存するとキーの最初の文字が止まらずに繰り返されるバグがあるため（メーカーのソフトウェアでも同じです）、アプリはライティングのあとにキーを含まない保存コマンドをもう一度送ります。オフにすると、メーカーのソフトウェアと同じ内容だけを送ります。このキーボードは、抜き差しするとライティングが消えます。

## 構成

ルールはクレートにあります。依存関係は内側にだけ向かい、外側に手を伸ばすレイヤーはコンパイルできません。

```
crates/
├── domain/        キーボード・操作・配列のコントロール・キーボードごとに保持できるもの、ポート、ユースケース（標準ライブラリのみ）
├── datalayer/     キーボードごとのコマンドとコントロールの番号、sysfsでのキーボードの検出とUSB（libusb）・hidrawでの書き込み、シミュレータのキーボード
├── presentation/  ビューモデルと画面が描く値 — 配列の図形、キー・色の表、udevルール（ツールキットなし）
├── ui/            GTK 4とlibadwaita — presentationだけを参照。新しいバージョンの通知（libsoup）
└── minikeyboard/  組み立て（composition root）— すべてのクレートを知る唯一の場所
po/                翻訳、言語ごとに1ファイル
data/              デスクトップファイル・metainfo・アイコン、udevルール
LICENSES/          アイコンのライセンス（CC BY-SA 4.0）
```

警告ゼロが基準です。警告は抑制せずに修正します。

## 制限事項

- 書き込みはこれまで1台のキーボード、キー6個・ノブ1つの`1189:8890`を使ってFedoraで確認しました。それ以外のキーボードには、メーカーのアプリと同じように書き込みます。コマンドはmacOSアプリのものとバイト単位まで同じで、テストで守っていますが、実機ではまだ1台も確認できておらず、そこへの経路（hidraw）も未確認です。
- `1189:8890`と種類を知らせないキーボードは現在の設定を知らせないため、アプリで設定したものだけが表示されます。
- 一度に扱えるキーボードは1台です。複数接続すると、最初に見つけたもの（`1189:8890`が優先）を使います。
- マウスの移動とドラッグには対応していません。

## ライセンス

MIT — [LICENSE](../LICENSE)を参照してください。アプリアイコンはCC BY-SA 4.0 — [LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt)を参照してください。
