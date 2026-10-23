<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="Icona dell’app"></p>

<h1 align="center">Configuratore di tastiere macro per Linux</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <b>Italiano</b> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

Un’app per Linux che scrive le impostazioni dei tasti su piccole tastiere macro USB – quelle in commercio che vanno da
una sola manopola fino a 21 tasti, con al massimo quattro manopole. Per ogni azione di un tasto o di una manopola
(ruota a sinistra, premi, ruota a destra) imposta sui livelli 1–3 pressioni di tasti, un tasto multimediale o
un’azione del mouse, oltre all’illuminazione della tastiera. È la versione per Linux di un’app per macOS costruita
sulle stesse regole, e usa gli stessi comandi.

| Tastiera | Disposizione | Lettura | Ritardo | Illuminazione | Provata |
|---|---|---|---|---|---|
| `1189:8890` | la decidi tu – tasti (fino a 12), tasti per riga, manopole (fino a 2) | no | nessuno | tre modalità | sì – 6 tasti e una manopola |
| produttore `1189` o `514C`, prodotto `8840` · `8842` · `8830`–`8833` · `8850` (e `514C:8851`), firmware più recente | secondo quanto indica la tastiera – 26 disposizioni | subito alla connessione | prima di ogni pressione | una modalità e un colore per ciascuno dei tasti 1–16 | no |
| le stesse, firmware precedente | secondo quanto indica la tastiera | su richiesta | uno per tasto | un colore e una modalità | no |
| le stesse, senza indicare il proprio tipo | la scegli tu – 26 disposizioni | no | nessuno | un colore e una modalità | no |

La `1189:8890` è la tastiera gestita dallo strumento Windows del produttore, “MINI KeyBoard”; le altre sono quelle che
cerca l’app multipiattaforma del produttore, “MINI_KEYBOARD”. L’app usa i comandi di entrambi gli strumenti. La
maggior parte delle tastiere del secondo tipo indica quanti tasti e manopole ha e quale firmware usa, e l’app disegna
la disposizione che disegnerebbe l’app del produttore; una tastiera che non lo indica riceve, tra queste disposizioni,
quella che scegli tu. L’app del produttore non conosce la `1189:8890` e il suo stesso strumento non ne disegna la
forma, quindi la sua forma reale non è nota: indichi all’app quanti tasti e manopole ha e quanti tasti ci sono in una
riga, e l’app ne disegna una disposizione ordinata.

L’app è disponibile in 13 lingue – inglese, coreano, giapponese, cinese (semplificato e tradizionale), tedesco,
spagnolo, francese, italiano, portoghese (Brasile), polacco, indonesiano e vietnamita – e segue la lingua del desktop.
Nelle altre lingue appare in inglese.

Il codice sorgente di questo repository è distribuito con licenza MIT. L’icona dell’app – quella in `data/icons/` e
l’immagine in alto – è distribuita con licenza CC BY-SA 4.0
([`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)).

## Download

Apri [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) nel tuo centro software,
oppure:

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

In questo modo viene aggiunto il repository Flatpak dell’app, e alla prima installazione viene scaricato da Flathub
anche il runtime di GNOME. Da quel momento in poi è il tuo centro software – oppure `flatpak update` – a portare le
nuove versioni. Le tue impostazioni restano.

Ogni versione si trova anche nella pagina [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) come
pacchetto Flatpak – `MINIKeyboard-<versione>-x86_64.flatpak` per Intel e AMD, `-aarch64.flatpak` per ARM –, per un PC
senza connessione a Internet o quando ti serve una versione precisa. Un pacchetto si installa con
`flatpak install --user <file>` e poi riceve gli aggiornamenti dallo stesso repository.

## Consentire l’accesso alla tastiera

Linux consente a un’app di aprire un dispositivo USB solo se una regola udev lo affida all’utente che ha effettuato
l’accesso, e un Flatpak non può installare quella regola. La prima volta, quando l’app segnala che le serve l’accesso,
**Come consentire…** mostra i comandi che scrivono la regola, con un pulsante per copiarli. Eseguili una volta in un
terminale, poi ricollega la tastiera. Se hai clonato questo repository, la stessa regola si trova in
[`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules).

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- Sulla `1189:8890` si scrive direttamente tramite USB (libusb): la sua interfaccia di configurazione ha un solo
  endpoint OUT, quindi nessun driver la prende in carico.
- Le altre tastiere vengono scritte tramite il dispositivo hidraw della loro prima interfaccia USB, come fa l’app del
  produttore. Nel frattempo la tastiera continua a digitare normalmente.

## Compilazione

```sh
make build    # target/release/minikeyboard, per PREFIX (/usr/local se non indicato)
make install  # dopo make build: il programma, le traduzioni, il file desktop, metainfo, le icone e le licenze, in DESTDIR/PREFIX
make test     # test di tutti i crate – non si apre nessuna finestra e non si tocca nessuna tastiera
make check    # rustfmt e clippy, gli avvisi sono errori
make run      # una build Debug con le sue traduzioni
```

Servono Rust 1.88 o successivo e i file di sviluppo di GTK 4, libadwaita 1.7 o successivo, libsoup 3 e libusb 1.0 – su
Debian e Ubuntu `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`, su Fedora
`gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`. Per le traduzioni serve `msgfmt` di gettext.

L’ID (`me.hobbyworker.MINIKeyboard`), il nome e i link sono quelli dell’app stessa e corrispondono al file desktop,
al file metainfo e alle icone in `data/`. Un fork fornisce i propri al momento della build: `MINIKEYBOARD_APP_ID` (che è
anche il nome dell’icona), `MINIKEYBOARD_APP_NAME`, `MINIKEYBOARD_COPYRIGHT`, `MINIKEYBOARD_WEBSITE` e
`MINIKEYBOARD_ISSUE_URL`. `make build` ricava da `PREFIX` il percorso delle traduzioni; `MINIKEYBOARD_LOCALE_DIR`
all’avvio lo sostituisce, per provare una build.

L’app può anche segnalare da sola le nuove versioni: una build a cui si fornisce `MINIKEYBOARD_UPDATE_FEED` legge quel
feed RSS all’avvio e una volta al giorno – a meno che nelle Preferenze non sia disattivato
**Cerca aggiornamenti automaticamente** – oltre che quando scegli **Cerca aggiornamenti…**, e mostra un banner che
porta alla pagina di download della nuova versione. Le build distribuite dell’app non lo usano – i loro aggiornamenti
arrivano tramite Flatpak –, quindi non hanno né quella voce di menu né quell’impostazione.

Le build Debug hanno un sottomenu **Simulatore** per provare l’app senza la tastiera in questione: scegli una
`1189:8890` con qualsiasi ID report, una tastiera con qualsiasi disposizione su uno dei due firmware oppure una che
non indica il proprio tipo, e l’app si comporta come se fosse collegata – con gli stessi comandi, senza che nulla
passi per l’USB. Con `MINIKEYBOARD_SIMULATED_KEYBOARD` (per esempio `8890-3`) l’app si avvia direttamente con una di
queste. **Registro dispositivo** mostra ogni report scritto e ogni risposta.

## Utilizzo

1. Collega la tastiera tramite USB. L’app la trova e, se è del secondo tipo, le chiede che tastiera è; se ha un
   firmware più recente, viene letta subito.
2. Per una tastiera che non indica la propria forma, la prima volta scegli la disposizione – una di quelle dell’app
   del produttore oppure, per la `1189:8890`, il numero di tasti, di tasti per riga e di manopole – e come è
   posizionata davanti a te. L’app ricorda entrambe le cose per quell’ID USB. In seguito le cambi con **Forma…** e
   **Ruota** (Ctrl+T); una tastiera che ha indicato la propria forma si può solo ruotare.
3. Scegli un livello, poi il tasto o l’azione della manopola sulla tastiera.
4. Assegna un’azione:
   - **Tasti** – tieni premuti i modificatori (Ctrl, Alt, Shift, Super e quelli a destra), poi fai clic sui tasti;
     ogni clic aggiunge una pressione. **Solo modificatori** trasforma il tasto in un tasto modificatore. **QWERTY**
     dispone i tasti come su una tastiera completa, **ABC** per gruppi; si può fare clic solo sui tasti che la
     tastiera accetta.
   - **Multimedia**, **Mouse** – sostituiscono ciò che faceva il tasto.
   - **LED** – l’illuminazione del livello; con il firmware più recente, seleziona uno dei tasti 1–16 per assegnargli
     un colore.
   - **Ritardo** – con i firmware che memorizzano le attese.
5. **Salva** (Ctrl+S) scrive ciò che è cambiato. **Leggi** (Ctrl+R) recupera le impostazioni attuali da una tastiera
   che consente di rileggerle.

Nelle **Preferenze** (Ctrl+,) scegli la lingua dell’interfaccia (una delle 13, oppure quella del desktop) e l’aspetto
chiaro o scuro; in una build con un feed degli aggiornamenti, la loro pagina Generali contiene la ricerca automatica
degli aggiornamenti. In **Informazioni su MINIKeyboard** trovi la versione e le licenze open source.

## Struttura

Le regole stanno in crate le cui dipendenze puntano solo verso l’interno: uno strato che cerca di raggiungere
l’esterno non compila.

```
crates/
├── domain/        tastiere, azioni, i controlli di ogni disposizione e ciò che ogni tastiera può contenere; le porte; casi d’uso (solo la libreria standard)
├── datalayer/     i comandi di ogni tastiera e i numeri dei suoi controlli; trovare la tastiera in sysfs e scrivere via USB (libusb) o hidraw; tastiere simulate
├── presentation/  view model e valori che le schermate disegnano – la geometria delle disposizioni, le tabelle di tasti e colori, la regola udev (nessun toolkit)
├── ui/            GTK 4 e libadwaita – vede solo presentation; l’avviso di nuova versione (libsoup)
└── minikeyboard/  la composition root – l’unico crate che li vede tutti
po/                traduzioni, un file per lingua
data/              il file desktop, metainfo e le icone, e la regola udev
LICENSES/          la licenza dell’icona (CC BY-SA 4.0)
```

Zero avvisi è il requisito. Gli avvisi si correggono, non si disattivano.

## Limiti

- Finora la scrittura è stata provata su una sola tastiera, una `1189:8890` con sei tasti e una manopola, su Fedora.
  Le altre tastiere vengono scritte come le scrive l’app del loro produttore – i comandi sono quelli dell’app per
  macOS, byte per byte, e i test li fissano –, ma nessuna è stata ancora provata su una tastiera reale, e nemmeno la
  via per raggiungerle (hidraw).
- La `1189:8890` e le tastiere che non indicano il proprio tipo non possono riferire le loro impostazioni, quindi si
  vede solo ciò che è stato impostato nell’app.
- Una tastiera alla volta. Se ne sono collegate diverse, l’app prende la prima che trova – prima una `1189:8890`.
- Lo spostamento del mouse e il trascinamento non sono disponibili.

## Licenza

MIT – vedi [LICENSE](../LICENSE). L’icona dell’app: CC BY-SA 4.0 – vedi
[LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt).
