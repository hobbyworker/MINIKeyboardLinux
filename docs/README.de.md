<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="App-Symbol"></p>

<h1 align="center">Konfigurator für Makrotastaturen unter Linux</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <b>Deutsch</b> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

Eine Linux-App, die Tastenbelegungen auf kleine USB-Makrotastaturen schreibt – auf die Modelle, die es vom einzelnen
Drehknopf bis zu 21 Tasten und mit bis zu vier Drehknöpfen zu kaufen gibt. Für jede Aktion einer Taste oder eines
Drehknopfs (nach links drehen, drücken, nach rechts drehen) legt sie auf den Ebenen 1–3 Tastendrücke, eine
Multimedia-Taste oder eine Mausaktion fest, außerdem die Beleuchtung der Tastatur. Sie ist das Linux-Gegenstück zu
einer macOS-App, die nach denselben Regeln gebaut ist, und verwendet dieselben Befehle.

<p align="center"><img src="../.github/devices.png" width="560" alt="Zwei kleine USB-Makrotastaturen: sechs Tasten und ein Drehknopf, zwölf Tasten und zwei Drehknöpfe"></p>

| Tastatur | Anordnung | Auslesen | Verzögerung | Beleuchtung | Getestet |
|---|---|---|---|---|---|
| `1189:8890` | legst du fest – Tasten (bis zu 12), Tasten pro Reihe, Drehknöpfe (bis zu 2) | nein | keine | drei Modi | ja – 6 Tasten und ein Drehknopf |
| Hersteller `1189` oder `514C`, Produkt `8840` · `8842` · `8830`–`8833` · `8850` · `8851`, neuere Firmware | wie die Tastatur sie meldet – 26 Anordnungen | sofort beim Verbinden | vor jedem Tastendruck | ein Modus und eine Farbe für jede der Tasten 1–16 | ja – `514C:8850`, 12 Tasten und 2 Drehknöpfe |
| dieselben, ältere Firmware | wie die Tastatur sie meldet | sofort beim Verbinden | eine pro Taste | eine Farbe und ein Modus | nein |
| dieselben, ohne Angabe ihres Typs | wählst du – 26 Anordnungen | nein | keine | eine Farbe und ein Modus | nein |

Die `1189:8890` ist die Tastatur, mit der das Windows-Tool des Herstellers, „MINI KeyBoard“, arbeitet; die übrigen
sind die, nach denen die plattformübergreifende App des Herstellers, „MINI_KEYBOARD“, sucht. Die App verwendet die
Befehle beider Tools. Die meisten Tastaturen der zweiten Art melden, wie viele Tasten und Drehknöpfe sie haben und
welche Firmware darauf läuft – dann zeichnet die App die Anordnung so, wie die App des Herstellers sie zeichnen würde.
Eine Tastatur, die das nicht meldet, bekommt die Anordnung, die du unter diesen auswählst. Die App des Herstellers
kennt die `1189:8890` nicht, und ihr eigenes Tool zeichnet keine Form, daher ist ihre tatsächliche Form unbekannt: Du
gibst an, wie viele Tasten und Drehknöpfe sie hat und wie viele Tasten in einer Reihe liegen, und die App zeichnet
daraus eine ordentliche Anordnung.

Die App gibt es in 13 Sprachen – Englisch, Koreanisch, Japanisch, Chinesisch (vereinfacht und traditionell), Deutsch,
Spanisch, Französisch, Italienisch, Portugiesisch (Brasilien), Polnisch, Indonesisch und Vietnamesisch. Sie folgt der
Sprache des Desktops; in allen anderen Sprachen erscheint sie auf Englisch.

Der Quellcode in diesem Repository steht unter der MIT-Lizenz. Das App-Symbol – die Dateien in `data/icons/` und das
Bild ganz oben – steht unter CC BY-SA 4.0 ([`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)).

## macOS, Windows und Linux

MINIKeyboard gibt es in drei Ausgaben nach denselben Regeln – dieselben Tastaturen, dieselben Befehle, dieselben 13 Sprachen.

| Repository | GitHub |
|---|---|
| **MINIKeyboardApple**<br><img alt="macOS 14+" src="https://img.shields.io/badge/macOS-14%2B-0A84FF?logo=apple&logoColor=white"> <img alt="Universal: Apple Silicon + Intel" src="https://img.shields.io/badge/Universal-Apple%20Silicon%20%2B%20Intel-5E5CE6"> <img alt="Swift 6" src="https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardApple](https://github.com/hobbyworker/MINIKeyboardApple) |
| **MINIKeyboardWin**<br><img alt="Windows 10 (2004)+ · 11" src="https://img.shields.io/badge/Windows-10%20(2004)%2B%20·%2011-0078D4?logo=windows&logoColor=white"> <img alt="x64 · ARM64" src="https://img.shields.io/badge/x64%20·%20ARM64-5E5CE6"> <img alt=".NET 10 · WinUI 3" src="https://img.shields.io/badge/.NET%2010-WinUI%203-512BD4?logo=dotnet&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardWin](https://github.com/hobbyworker/MINIKeyboardWin) |
| **MINIKeyboardLinux**<br><img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white"> <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white"> <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardLinux](https://github.com/hobbyworker/MINIKeyboardLinux) |

## Herunterladen

Öffne [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) in deinem Software-Center
oder gib ein:

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

Damit wird das Flatpak-Repository der App hinzugefügt; bei der ersten Installation wird außerdem die
GNOME-Laufzeitumgebung von Flathub geladen. Danach holt dein Software-Center – oder `flatpak update` – neue Versionen.
Deine Einstellungen bleiben erhalten.

Jede Version gibt es außerdem unter [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) als
Flatpak-Bundle – `MINIKeyboard-<Version>-x86_64.flatpak` für Intel und AMD, `-aarch64.flatpak` für ARM. Die Bundles
sind für einen PC ohne Internetverbindung gedacht oder für den Fall, dass du eine bestimmte Version brauchst. Ein
Bundle installierst du mit `flatpak install --user <Datei>`; danach bekommt es seine Updates aus demselben Repository.

## Zugriff auf die Tastatur erlauben

Unter Linux darf eine App ein USB-Gerät nur öffnen, wenn eine udev-Regel es dem angemeldeten Benutzer überlässt, und
ein Flatpak kann diese Regel nicht installieren. Meldet die App beim ersten Mal, dass sie Zugriff braucht, zeigt
**Zugriff einrichten…** die Befehle, die die Regel schreiben, samt einer Schaltfläche zum Kopieren. Führe sie einmal
in einem Terminal aus und stecke die Tastatur dann neu ein. In einem Klon dieses Repositorys liegt dieselbe Regel
unter [`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules).

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- Auf die `1189:8890` wird direkt über USB (libusb) geschrieben: Ihre Konfigurationsschnittstelle hat nur einen
  OUT-Endpunkt, deshalb bindet sich kein Treiber an sie.
- Die übrigen Tastaturen werden wie in der App des Herstellers über das hidraw-Gerät ihrer ersten USB-Schnittstelle
  beschrieben. Die Tastatur tippt währenddessen ganz normal weiter.

## Build

```sh
make build    # target/release/minikeyboard, für PREFIX (ohne Angabe /usr/local)
make install  # nach make build: Programm, Übersetzungen, Desktop-Datei, Metainfo, Symbole und Lizenzen nach DESTDIR/PREFIX
make test     # Tests aller Crates – es öffnet sich kein Fenster, und keine Tastatur wird angerührt
make check    # rustfmt und clippy, Warnungen gelten als Fehler
make run      # ein Debug-Build mit seinen Übersetzungen
```

Nötig sind Rust 1.88 oder neuer und die Entwicklungsdateien von GTK 4, libadwaita 1.7 oder neuer, libsoup 3 und
libusb 1.0 – unter Debian und Ubuntu `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`, unter Fedora
`gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`. Für die Übersetzungen braucht es `msgfmt` aus gettext.

ID (`me.hobbyworker.MINIKeyboard`), Name und Links der App sind ihre eigenen und passen zur Desktop-Datei, zur
Metainfo und zu den Symbolen in `data/`. Ein Fork gibt beim Build seine eigenen an: `MINIKEYBOARD_APP_ID` (zugleich
der Name des Symbols), `MINIKEYBOARD_APP_NAME`, `MINIKEYBOARD_COPYRIGHT`, `MINIKEYBOARD_WEBSITE` und
`MINIKEYBOARD_ISSUE_URL`. Wo die Übersetzungen liegen, leitet `make build` aus `PREFIX` ab; `MINIKEYBOARD_LOCALE_DIR`
beim Start überschreibt das – zum Ausprobieren eines Builds.

Die App kann neue Versionen auch selbst melden: Ein Build mit `MINIKEYBOARD_UPDATE_FEED` liest diesen RSS-Feed beim
Start und einmal am Tag – außer wenn in den Einstellungen **Automatisch nach Updates suchen** ausgeschaltet ist – sowie
bei **Nach Updates suchen…** und zeigt dann ein Banner an, das zur Download-Seite der neuen Version führt. Die Builds
der App selbst nutzen das nicht – ihre Updates bringt Flatpak –, daher haben sie weder diesen Menüpunkt noch diese
Einstellung.

Im Untermenü **Simulator** kannst du die App ohne die betreffende Tastatur ausprobieren: Wähle eine `1189:8890`, eine
Anordnung auf der neueren Firmware oder eine Tastatur, die ihren Typ nicht angibt – dann verhält sich die App, als wäre
sie angeschlossen, mit denselben Befehlen, nur dass nichts über USB hinausgeht. Solange eine simulierte Tastatur aktiv ist,
sagt das ein orangefarbener Balken oben im Fenster, und **Zurück zur Tastatur am USB** kehrt zurück. Debug-Builds bieten
jede Art an (die anderen Report-IDs der `1189:8890`, die ältere Firmware), und mit `MINIKEYBOARD_SIMULATED_KEYBOARD`
(zum Beispiel `8890-3`) startet die App gleich mit einer davon. **Geräteprotokoll** zeigt jeden geschriebenen Report und
jede Antwort.

## Verwendung

1. Schließe die Tastatur über USB an. Die App findet sie und fragt eine Tastatur der zweiten Art nach ihrem Typ; eine,
   die antwortet, wird sofort ausgelesen.
2. Bei einer Tastatur, die ihre Form nicht angibt, legst du beim ersten Mal die Anordnung fest – eine der Anordnungen
   aus der App des Herstellers, bei der `1189:8890` die Zahl der Tasten, der Tasten pro Reihe und der Drehknöpfe – und
   wie sie vor dir liegt. Die App merkt sich beides für diese USB-ID. Später änderst du das mit **Form…** und
   **Drehen** (Ctrl+T); eine Tastatur, die ihre Form angegeben hat, lässt sich nur drehen.
3. Wähle eine Ebene und dann auf der Tastatur die Taste oder die Drehknopf-Aktion.
4. Gib ihr eine Aktion:
   - **Tasten** – Halte Sondertasten gedrückt (Ctrl, Alt, Shift, Super und die rechten), dann klicke auf Tasten; jeder
     Klick fügt einen Tastendruck hinzu. **Nur Sondertasten** macht die Taste zur Sondertaste. **QWERTY** ordnet die
     Tasten wie auf einer Tastatur in voller Größe an, **ABC** in Gruppen; anklicken lassen sich nur Tasten, die die
     Tastatur annimmt.
   - **Multimedia**, **Maus** – ersetzen, was die Taste vorher tat.
   - **LED** – die Beleuchtung der Ebene; bei neuerer Firmware wählst du eine der Tasten 1–16 aus, um ihr eine Farbe
     zu geben.
   - **Verzögerung** – bei Firmware, die Wartezeiten speichert.
5. **Sichern** (Ctrl+S) schreibt, was sich geändert hat. **Lesen** (Ctrl+R) holt bei einer Tastatur, die sich auslesen
   lässt, die aktuellen Einstellungen.

In den **Einstellungen** (Ctrl+,) wählst du die Sprache der Oberfläche (eine der 13 oder die des Desktops) und Hell
oder Dunkel; in einem Build mit Update-Feed findest du dort unter „Allgemein“ die automatische Suche nach Updates.
Unter **Über „MINIKeyboard“** stehen die Version und die Open-Source-Lizenzen.

**Tastenwiederholung nach dem Ändern der Beleuchtung verhindern** (**Einstellungen ▸ Allgemein ▸ Firmware-Fehler
umgehen**) ist standardmäßig aktiviert. Die Firmware der `1189:8890` wiederholt nach dem Sichern der Beleuchtung das
erste Zeichen einer Taste endlos – die Software des Herstellers verhält sich genauso –, darum sendet die App nach der
Beleuchtung noch einen Sicherungsbefehl ohne Taste. Ist die Option deaktiviert, sendet die App nur das, was auch die
Software des Herstellers sendet. Diese Tastatur vergisst ihre Beleuchtung außerdem, wenn man sie absteckt.

## Aufbau

Die Regeln stecken in Crates, deren Abhängigkeiten nur nach innen zeigen – eine Schicht, die nach außen greift, lässt
sich nicht kompilieren.

```
crates/
├── domain/        Tastaturen, Aktionen, die Bedienelemente jeder Anordnung und was jede Tastatur aufnehmen kann; die Ports; Use Cases (nur die Standardbibliothek)
├── datalayer/     die Befehle jeder Tastatur und die Nummern ihrer Bedienelemente; Tastatur in sysfs finden und über USB (libusb) oder hidraw schreiben; simulierte Tastaturen
├── presentation/  View-Models und die Werte, die die Ansichten zeichnen – die Geometrie der Anordnungen, Tasten- und Farbtabellen, die udev-Regel (kein Toolkit)
├── ui/            GTK 4 und libadwaita – sieht nur presentation; der Hinweis auf neue Versionen (libsoup)
└── minikeyboard/  die Composition Root – das einzige Crate, das alle kennt
po/                Übersetzungen, eine Datei pro Sprache
data/              Desktop-Datei, Metainfo und Symbole, die udev-Regel
LICENSES/          die Lizenz des Symbols (CC BY-SA 4.0)
```

Null Warnungen ist der Maßstab. Warnungen werden behoben, nicht abgeschaltet.

## Einschränkungen

- Geschrieben wurde bisher auf zwei Tastaturen, unter Fedora: eine `1189:8890` mit sechs Tasten und einem Drehknopf
  und eine `514C:8850` mit zwölf Tasten und zwei Drehknöpfen auf neuerer Firmware, geöffnet über hidraw. Die übrigen
  Tastaturen werden so beschrieben, wie die App ihres Herstellers sie beschreibt – die Befehle sind bis aufs Byte die
  der macOS-App, und Tests halten sie fest –, aber an einer echten Tastatur wurde noch keine davon ausprobiert.
- Die `1189:8890` und Tastaturen, die ihren Typ nicht angeben, können ihre Einstellungen nicht melden; die App zeigt
  daher nur, was in ihr festgelegt wurde.
- Immer nur eine Tastatur ist geöffnet. Sind mehrere angeschlossen, wählt die Schaltfläche neben dem
  Verbindungsstatus, welche; die App öffnet die zuletzt gewählte, sonst die erste, die sie findet – eine `1189:8890`
  zuerst.
- Mausbewegungen und Ziehen werden nicht angeboten.

## Lizenz

MIT – siehe [LICENSE](../LICENSE). Das App-Symbol: CC BY-SA 4.0 – siehe
[LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt).
