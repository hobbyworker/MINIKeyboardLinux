<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="Ikona aplikacji"></p>

<h1 align="center">Konfigurator klawiatur makro na Linuksa</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <b>Polski</b> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

Aplikacja na Linuksa, która zapisuje ustawienia klawiszy w małych klawiaturach makro USB. Takie klawiatury sprzedaje
się w wielu wersjach: od modeli z jednym pokrętłem po modele z 21 klawiszami, z maksymalnie czterema pokrętłami. Dla
każdego klawisza i każdej akcji pokrętła (obrót w lewo, naciśnięcie, obrót w prawo) aplikacja ustawia na warstwach 1–3
naciśnięcia klawiszy, klawisz multimedialny lub akcję myszy, a także podświetlenie klawiatury. To linuksowy
odpowiednik aplikacji na macOS opartej na tych samych regułach; obie używają tych samych poleceń.

| Klawiatura | Układ | Odczyt | Opóźnienie | Podświetlenie | Sprawdzono |
|---|---|---|---|---|---|
| `1189:8890` | ustawiasz go sam — klawisze (do 12), klawisze w rzędzie, pokrętła (do 2) | nie | nie | trzy tryby | tak — 6 klawiszy i pokrętło |
| producent `1189` lub `514C`, produkt `8840` · `8842` · `8830`–`8833` · `8850` (oraz `514C:8851`), nowsze oprogramowanie sprzętowe | według tego, co podaje klawiatura — 26 układów | od razu po podłączeniu | przed każdym naciśnięciem | tryb i kolor dla każdego z klawiszy 1–16 | nie |
| te same, starsze oprogramowanie sprzętowe | według tego, co podaje klawiatura | na żądanie | jedno na klawisz | kolor i tryb | nie |
| te same, niepodające swojego typu | wybierasz go sam — 26 układów | nie | nie | kolor i tryb | nie |

`1189:8890` to klawiatura, z którą współpracuje windowsowe narzędzie konfiguracyjne producenta „MINI KeyBoard”;
pozostałe to te, których szuka wieloplatformowa aplikacja producenta „MINI_KEYBOARD”. Aplikacja korzysta z poleceń obu
narzędzi. Większość klawiatur drugiego rodzaju podaje, ile ma klawiszy i pokręteł oraz jakie ma oprogramowanie
sprzętowe, więc aplikacja rysuje taki sam układ, jaki narysowałaby aplikacja producenta; dla klawiatury, która tego
nie podaje, używa tego z tych układów, który wybierzesz. Aplikacja producenta nie zna klawiatury `1189:8890`, a
narzędzie przeznaczone dla tej klawiatury nie rysuje żadnego kształtu, więc jej rzeczywisty kształt nie jest znany.
Dlatego to ty podajesz, ile ma klawiszy i pokręteł oraz ile klawiszy jest w rzędzie, a aplikacja rysuje je dokładnie
tak, w równym układzie.

Aplikacja jest dostępna w 13 językach — angielskim, koreańskim, japońskim, chińskim (uproszczonym i tradycyjnym),
niemieckim, hiszpańskim, francuskim, włoskim, portugalskim (Brazylia), polskim, indonezyjskim i wietnamskim — i używa
języka ustawionego w środowisku graficznym. W pozostałych językach wyświetla się po angielsku.

Kod źródłowy w tym repozytorium jest objęty licencją MIT. Ikona aplikacji — ta w `data/icons/` i obraz na górze — jest
objęta licencją CC BY-SA 4.0 ([`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)).

## Pobieranie

Otwórz plik [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) w centrum
oprogramowania albo wpisz:

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

Zostanie dodane repozytorium Flatpak aplikacji, a przy pierwszej instalacji z serwisu Flathub zostanie pobrane też
środowisko uruchomieniowe GNOME. Od tej pory nowe wersje dostarcza centrum oprogramowania — albo polecenie
`flatpak update`. Twoje ustawienia zostają zachowane.

Każda wersja jest też dostępna na stronie [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) jako
pakiet Flatpak — `MINIKeyboard-<wersja>-x86_64.flatpak` dla procesorów Intel i AMD, `-aarch64.flatpak` dla procesorów
ARM. Przydaje się on na komputerze bez dostępu do internetu albo wtedy, gdy potrzebujesz konkretnej wersji. Pakiet
instaluje się poleceniem `flatpak install --user <plik>`, a potem otrzymuje uaktualnienia z tego samego repozytorium.

## Dostęp aplikacji do klawiatury

Linux pozwala aplikacji otworzyć urządzenie USB tylko wtedy, gdy reguła udev udostępnia je zalogowanemu użytkownikowi,
a Flatpak nie może zainstalować takiej reguły. Gdy za pierwszym razem aplikacja poinformuje, że potrzebuje dostępu,
**Jak zezwolić…** pokaże polecenia zapisujące regułę wraz z przyciskiem do ich skopiowania. Uruchom je raz w
terminalu, a potem podłącz klawiaturę ponownie. W sklonowanym repozytorium ta sama reguła znajduje się w pliku
[`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules).

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- Klawiatura `1189:8890` jest zapisywana bezpośrednio przez USB (libusb): jej interfejs konfiguracyjny ma tylko punkt
  końcowy OUT, więc nie przejmuje go żaden sterownik.
- Pozostałe klawiatury są zapisywane przez urządzenie hidraw ich pierwszego interfejsu USB, tak jak robi to aplikacja
  producenta. W tym czasie klawiatura nadal normalnie pisze.

## Budowanie

```sh
make build    # target/release/minikeyboard, dla PREFIX (jeśli nie podano, /usr/local)
make install  # po make build: program, tłumaczenia, plik desktop, metainfo, ikony i licencje do DESTDIR/PREFIX
make test     # testy wszystkich crate’ów — nie otwiera się żadne okno i żadna klawiatura nie jest ruszana
make check    # rustfmt i clippy, ostrzeżenia są błędami
make run      # kompilacja Debug z tłumaczeniami
```

Potrzebne są Rust 1.88 lub nowszy oraz pliki deweloperskie GTK 4, libadwaita 1.7 lub nowszej, libsoup 3 i libusb 1.0 —
w Debianie i Ubuntu `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`, w Fedorze
`gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`. Do tłumaczeń potrzebny jest `msgfmt` z gettext.

Identyfikator (`me.hobbyworker.MINIKeyboard`), nazwa i linki należą do samej aplikacji i zgadzają się z plikiem
desktop, plikiem metainfo i ikonami w `data/`. Fork podaje własne w czasie budowania: `MINIKEYBOARD_APP_ID` (to także
nazwa ikony), `MINIKEYBOARD_APP_NAME`, `MINIKEYBOARD_COPYRIGHT`, `MINIKEYBOARD_WEBSITE` i `MINIKEYBOARD_ISSUE_URL`.
Miejsce tłumaczeń `make build` ustala na podstawie `PREFIX`, a zmienna `MINIKEYBOARD_LOCALE_DIR` podana przy
uruchomieniu je zastępuje — przy wypróbowywaniu kompilacji.

Aplikacja może też sama informować o nowych wersjach: kompilacja z podaną zmienną `MINIKEYBOARD_UPDATE_FEED`
odczytuje ten kanał RSS przy uruchomieniu i raz dziennie — chyba że w Preferencjach wyłączono
**Sprawdzaj uaktualnienia automatycznie** — a także po wybraniu **Sprawdź uaktualnienia…**, po czym wyświetla baner
prowadzący do strony pobierania nowej wersji. Rozpowszechniane kompilacje aplikacji z tego nie korzystają —
uaktualnienia dostarcza im Flatpak — więc nie mają ani tej pozycji menu, ani tego ustawienia.

Podmenu **Symulator** pozwala wypróbować aplikację bez danej klawiatury: wybierz `1189:8890`, układ z nowszym
oprogramowaniem sprzętowym albo klawiaturę, która nie podaje swojego typu, a aplikacja zachowa się tak, jakby klawiatura
była podłączona — z tymi samymi poleceniami, bez wysyłania czegokolwiek przez USB. Gdy symulator jest w użyciu, mówi o tym
pomarańczowy pasek u góry okna, a **Wróć do klawiatury przez USB** przywraca prawdziwą klawiaturę. Kompilacje Debug
oferują każdy rodzaj (pozostałe ID raportu `1189:8890`, starsze oprogramowanie sprzętowe), a ze zmienną
`MINIKEYBOARD_SIMULATED_KEYBOARD` (na przykład `8890-3`) aplikacja od razu uruchamia się z wybraną klawiaturą.
**Dziennik urządzenia** pokazuje wszystkie zapisane raporty i wszystkie odpowiedzi.

## Korzystanie

1. Podłącz klawiaturę przez USB. Aplikacja ją znajduje, a klawiaturę drugiego rodzaju pyta, czym jest; taką z nowszym
   oprogramowaniem sprzętowym od razu odczytuje.
2. Dla klawiatury, która nie podaje swojego kształtu, przy pierwszym podłączeniu wybierasz układ — jeden z układów
   aplikacji producenta, a dla `1189:8890` liczbę klawiszy, klawiszy w rzędzie i pokręteł — oraz to, jak leży przed
   tobą. Aplikacja zapamiętuje oba ustawienia dla danego ID USB. Później zmienisz je poleceniami **Kształt…** i
   **Obróć** (Ctrl+T). Klawiaturę, która podała swój kształt, można tylko obracać.
3. Wybierz warstwę, a potem na rysunku klawiatury klawisz lub akcję pokrętła.
4. Przypisz akcję:
   - **Klawisze** — przytrzymaj modyfikatory (Ctrl, Alt, Shift, Super oraz te po prawej stronie) i klikaj klawisze;
     każde kliknięcie dodaje jedno naciśnięcie. **Tylko modyfikatory** zmienia klawisz w klawisz modyfikujący.
     **QWERTY** rozmieszcza klawisze jak na pełnowymiarowej klawiaturze, a **ABC** — w grupach. Kliknąć można tylko
     klawisze, które klawiatura obsługuje.
   - **Multimedia**, **Mysz** — zastępują to, co robił klawisz.
   - **LED** — podświetlenie warstwy; w nowszym oprogramowaniu sprzętowym wybierz jeden z klawiszy 1–16, aby nadać mu
     kolor.
   - **Opóźnienie** — w oprogramowaniu sprzętowym, które zapamiętuje opóźnienia.
5. **Zachowaj** (Ctrl+S) zapisuje zmiany. **Odczytaj** (Ctrl+R) pobiera bieżące ustawienia z klawiatury, która pozwala
   je odczytać.

**Preferencje** (Ctrl+,) pozwalają wybrać język interfejsu (jeden z 13 lub język środowiska graficznego) oraz tryb
jasny lub ciemny; w kompilacji z kanałem uaktualnień na ich stronie Ogólne jest też automatyczne sprawdzanie
uaktualnień. Polecenie **O programie MINIKeyboard** pokazuje wersję i licencje open source.

Opcja **Zapobiegaj powtarzaniu klawiszy po zmianie podświetlenia** (**Preferencje ▸ Ogólne ▸ Obejścia błędów
oprogramowania układowego**) jest domyślnie włączona. Oprogramowanie układowe `1189:8890` po zachowaniu podświetlenia
bez końca powtarza pierwszy znak klawisza (oprogramowanie producenta działa tak samo), dlatego aplikacja po
podświetleniu wysyła jeszcze jedno polecenie zapisu, bez żadnego klawisza. Gdy opcja jest wyłączona, aplikacja wysyła
tylko to, co oprogramowanie producenta. Ta klawiatura zapomina też podświetlenie po odłączeniu.

## Struktura

Reguły znajdują się w crate’ach, których zależności są skierowane tylko do środka — warstwa sięgająca na zewnątrz się
nie kompiluje.

```
crates/
├── domain/        klawiatury, akcje, kontrolki każdego układu i to, co mieści każda klawiatura; porty; przypadki użycia (tylko biblioteka standardowa)
├── datalayer/     polecenia każdej klawiatury i numeracja jej kontrolek; wyszukiwanie klawiatury w sysfs i zapis przez USB (libusb) lub hidraw; symulowane klawiatury
├── presentation/  modele widoków i wartości rysowane przez ekrany — geometria układów, tabele klawiszy i kolorów, reguła udev (bez toolkitu)
├── ui/            GTK 4 i libadwaita — widzi tylko presentation; powiadomienie o nowej wersji (libsoup)
└── minikeyboard/  composition root — jedyny crate, który widzi wszystkie pozostałe
po/                tłumaczenia, jeden plik na język
data/              plik desktop, metainfo i ikony oraz reguła udev
LICENSES/          licencja ikony (CC BY-SA 4.0)
```

Zero ostrzeżeń to standard. Ostrzeżenia się poprawia, a nie wyłącza.

## Ograniczenia

- Zapis sprawdzono dotąd tylko na jednej klawiaturze — `1189:8890` z sześcioma klawiszami i pokrętłem — w Fedorze.
  Pozostałe klawiatury są zapisywane tak, jak robi to aplikacja ich producenta: polecenia są bajt w bajt takie same
  jak w aplikacji na macOS i pilnują ich testy, ale żadnej z nich nie sprawdzono jeszcze na prawdziwej klawiaturze,
  podobnie jak samej drogi do nich (hidraw).
- Klawiatura `1189:8890` i klawiatury, które nie podają swojego typu, nie przekazują swoich bieżących ustawień, więc
  aplikacja pokazuje tylko to, co w niej ustawiono.
- Jedna klawiatura naraz. Jeśli podłączonych jest kilka, aplikacja używa pierwszej znalezionej — w pierwszej
  kolejności `1189:8890`.
- Nie ma akcji ruchu myszy ani przeciągania.

## Licencja

MIT — zobacz [LICENSE](../LICENSE). Ikona aplikacji: CC BY-SA 4.0 — zobacz
[LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt).
