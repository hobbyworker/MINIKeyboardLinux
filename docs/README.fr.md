<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="Icône de l’app"></p>

<h1 align="center">Configurateur de claviers macro pour Linux</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <b>Français</b> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

Une app Linux qui écrit les réglages des touches sur de petits claviers macro USB – ceux que l’on trouve dans le
commerce, d’un seul bouton rotatif jusqu’à 21 touches, avec au plus quatre boutons rotatifs. Pour chaque action d’une
touche ou d’un bouton rotatif (rotation à gauche, appui, rotation à droite), elle définit sur les couches 1 à 3 des
frappes, une touche multimédia ou une action de la souris, ainsi que l’éclairage du clavier. C’est la version Linux
d’une app macOS construite selon les mêmes règles, et elle utilise les mêmes commandes.

| Clavier | Disposition | Lecture | Délai | Éclairage | Testé |
|---|---|---|---|---|---|
| `1189:8890` | définie par vous – touches (jusqu’à 12), touches par rangée, boutons rotatifs (jusqu’à 2) | non | aucun | trois modes | oui – 6 touches et un bouton rotatif |
| fabricant `1189` ou `514C`, produit `8840` · `8842` · `8830`–`8833` · `8850` (et `514C:8851`), programme interne récent | selon ce qu’indique le clavier – 26 dispositions | dès la connexion | avant chaque frappe | un mode, et une couleur pour chacune des touches 1–16 | non |
| les mêmes, ancien programme interne | selon ce qu’indique le clavier | à la demande | un par touche | une couleur et un mode | non |
| les mêmes, sans indiquer leur type | choisie par vous – 26 dispositions | non | aucun | une couleur et un mode | non |

Le `1189:8890` est le clavier que prend en charge l’outil Windows du fabricant, « MINI KeyBoard » ; les autres sont
ceux que recherche l’app multiplateforme du fabricant, « MINI_KEYBOARD ». L’app utilise les commandes des deux outils.
La plupart des claviers du second type indiquent combien de touches et de boutons rotatifs ils possèdent et quel
programme interne ils exécutent : l’app dessine alors la disposition que dessinerait l’app du fabricant. Un clavier
qui ne l’indique pas reçoit celle de ces dispositions que vous choisissez. L’app du fabricant ne connaît pas le
`1189:8890` et son propre outil ne dessine aucune forme, si bien que sa forme réelle est inconnue : vous indiquez à
l’app combien de touches et de boutons rotatifs il possède et combien de touches compte une rangée, et elle en dessine
une disposition bien ordonnée.

L’app est disponible en 13 langues – anglais, coréen, japonais, chinois (simplifié et traditionnel), allemand,
espagnol, français, italien, portugais (Brésil), polonais, indonésien et vietnamien – et suit la langue du bureau.
Dans les autres langues, elle s’affiche en anglais.

Le code source de ce dépôt est sous licence MIT. L’icône de l’app – celle de `data/icons/` et l’image en haut de la
page – est sous licence CC BY-SA 4.0 ([`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)).

## Téléchargement

Ouvrez [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) dans votre logithèque, ou
tapez :

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

Le dépôt Flatpak de l’app est alors ajouté, et la première installation récupère aussi l’environnement d’exécution
GNOME depuis Flathub. Ensuite, votre logithèque – ou `flatpak update` – vous apporte les nouvelles versions. Vos
réglages sont conservés.

Chaque version se trouve aussi sur la page [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) sous
forme de paquet Flatpak – `MINIKeyboard-<version>-x86_64.flatpak` pour Intel et AMD, `-aarch64.flatpak` pour ARM –,
pour un PC sans connexion à Internet ou lorsqu’il vous faut une version précise. Un paquet s’installe avec
`flatpak install --user <fichier>`, puis reçoit ses mises à jour depuis le même dépôt.

## Autoriser l’accès au clavier

Sous Linux, une app ne peut ouvrir un périphérique USB que si une règle udev le confie à l’utilisateur connecté, et un
Flatpak ne peut pas installer cette règle. La première fois, l’app vous signale qu’elle a besoin d’un accès :
**Comment autoriser…** affiche les commandes qui écrivent la règle, avec un bouton pour les copier. Exécutez-les une
fois dans un terminal, puis rebranchez le clavier. Si vous avez cloné ce dépôt, la même règle se trouve dans
[`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules).

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- Le `1189:8890` est écrit directement via USB (libusb) : son interface de configuration n’a qu’un point de
  terminaison OUT, si bien qu’aucun pilote ne s’y attache.
- Les autres claviers sont écrits via le périphérique hidraw de leur première interface USB, comme le fait l’app du
  fabricant. Pendant ce temps, le clavier continue de saisir normalement.

## Compilation

```sh
make build    # target/release/minikeyboard, pour PREFIX (/usr/local s’il n’est pas fourni)
make install  # après make build : le programme, les traductions, le fichier desktop, metainfo, les icônes et les licences, dans DESTDIR/PREFIX
make test     # tests de toutes les crates – aucune fenêtre ne s’ouvre, aucun clavier n’est touché
make check    # rustfmt et clippy, les avertissements sont des erreurs
make run      # une compilation Debug avec ses traductions
```

Il faut Rust 1.88 ou plus récent, ainsi que les fichiers de développement de GTK 4, de libadwaita 1.7 ou plus récent,
de libsoup 3 et de libusb 1.0 – sous Debian et Ubuntu
`libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`, sous Fedora
`gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`. Les traductions nécessitent `msgfmt` de gettext.

L’identifiant (`me.hobbyworker.MINIKeyboard`), le nom et les liens de l’app lui sont propres et correspondent au
fichier desktop, au fichier metainfo et aux icônes de `data/`. Un fork fournit les siens à la compilation :
`MINIKEYBOARD_APP_ID` (c’est aussi le nom de l’icône), `MINIKEYBOARD_APP_NAME`, `MINIKEYBOARD_COPYRIGHT`,
`MINIKEYBOARD_WEBSITE` et `MINIKEYBOARD_ISSUE_URL`. `make build` déduit de `PREFIX` l’emplacement des traductions ;
`MINIKEYBOARD_LOCALE_DIR` au lancement le remplace, pour essayer une compilation.

L’app peut aussi signaler elle-même les nouvelles versions : une compilation à laquelle on fournit
`MINIKEYBOARD_UPDATE_FEED` lit ce flux RSS au lancement et une fois par jour – sauf si
**Rechercher les mises à jour automatiquement** est désactivé dans les Préférences – ainsi que lorsque vous choisissez
**Rechercher les mises à jour…**, puis affiche une bannière qui mène à la page de téléchargement de la nouvelle
version. Les compilations distribuées de l’app ne l’utilisent pas – leurs mises à jour arrivent par Flatpak –, si bien
qu’elles n’ont ni cet élément de menu ni ce réglage.

Les compilations Debug comportent un sous-menu **Simulateur** pour essayer l’app sans le clavier concerné : choisissez
un `1189:8890` avec n’importe quel ID de rapport, un clavier de n’importe quelle disposition avec l’un ou l’autre
programme interne, ou un clavier qui n’indique pas son type, et l’app se comporte comme s’il était branché – avec les
mêmes commandes, sans que rien ne parte vers l’USB. `MINIKEYBOARD_SIMULATED_KEYBOARD` (par exemple `8890-3`) la fait
démarrer directement avec l’un d’eux. **Journal de l’appareil** affiche chaque rapport écrit et chaque réponse.

## Utilisation

1. Branchez le clavier en USB. L’app le trouve et, s’il est du second type, lui demande ce qu’il est ; s’il a un
   programme interne récent, il est lu aussitôt.
2. Pour un clavier qui n’indique pas sa forme, vous choisissez la première fois sa disposition – l’une des
   dispositions de l’app du fabricant ou, pour le `1189:8890`, le nombre de touches, de touches par rangée et de
   boutons rotatifs – et son orientation devant vous. L’app retient les deux pour cet ID USB. **Forme…** et
   **Faire pivoter** (Ctrl+T) permettent de les modifier par la suite ; un clavier qui a indiqué sa forme peut
   seulement pivoter.
3. Choisissez une couche, puis la touche ou l’action du bouton rotatif sur le clavier.
4. Attribuez-lui une action :
   - **Touches** – maintenez des touches de modification (Ctrl, Alt, Shift, Super, et celles de droite), puis cliquez
     sur des touches ; chaque clic ajoute une frappe. **Modificateurs seuls** fait de la touche une touche de
     modification. **QWERTY** dispose les touches comme sur un clavier complet, **ABC** par groupes ; seules les
     touches que le clavier accepte peuvent être cliquées.
   - **Multimédia**, **Souris** – remplacent ce que faisait la touche.
   - **LED** – l’éclairage de la couche ; avec un programme interne récent, sélectionnez l’une des touches 1–16 pour
     lui donner une couleur.
   - **Délai** – avec un programme interne qui conserve les attentes.
5. **Enregistrer** (Ctrl+S) écrit ce qui a changé. **Lire** (Ctrl+R) récupère les réglages actuels d’un clavier qui
   permet de les relire.

Dans les **Préférences** (Ctrl+,), vous choisissez la langue de l’interface (l’une des 13, ou celle du bureau) et
l’apparence claire ou sombre ; dans une compilation dotée d’un flux de mises à jour, leur page Général propose la
recherche automatique des mises à jour. **À propos de MINIKeyboard** affiche la version et les licences open source.

Dans **Préférences ▸ Général ▸ Clavier**, **Empêcher la répétition des touches après un changement d’éclairage** est
activé par défaut. Après l’enregistrement de l’éclairage, le firmware du `1189:8890` répète sans fin le premier
caractère d’une touche (le logiciel du fabricant fait de même) ; l’app envoie donc une commande d’enregistrement
supplémentaire, sans touche, après l’éclairage. Désactivé, l’app envoie uniquement ce qu’envoie le logiciel du
fabricant. Ce clavier oublie aussi son éclairage quand on le débranche.

## Structure

Les règles se trouvent dans des crates dont les dépendances pointent uniquement vers l’intérieur – une couche qui
cherche à atteindre l’extérieur ne compile pas.

```
crates/
├── domain/        claviers, actions, les contrôles de chaque disposition et ce que chaque clavier peut contenir ; les ports ; cas d’utilisation (bibliothèque standard uniquement)
├── datalayer/     les commandes de chaque clavier et les numéros de ses contrôles ; trouver le clavier dans sysfs et écrire via USB (libusb) ou hidraw ; claviers simulés
├── presentation/  modèles de vue et valeurs que dessinent les écrans – la géométrie des dispositions, les tables de touches et de couleurs, la règle udev (sans toolkit)
├── ui/            GTK 4 et libadwaita – ne voit que presentation ; l’avis de nouvelle version (libsoup)
└── minikeyboard/  la racine de composition – la seule crate qui les voit toutes
po/                traductions, un fichier par langue
data/              le fichier desktop, metainfo et les icônes, et la règle udev
LICENSES/          la licence de l’icône (CC BY-SA 4.0)
```

Zéro avertissement : c’est la règle. Les avertissements sont corrigés, pas désactivés.

## Limites

- L’écriture n’a été essayée jusqu’ici que sur un seul clavier, un `1189:8890` à six touches et un bouton rotatif,
  sous Fedora. Les autres claviers sont écrits comme l’app de leur fabricant les écrit – les commandes sont celles de
  l’app macOS, octet pour octet, et des tests les figent –, mais aucun n’a encore été essayé sur un vrai clavier, pas
  plus que le chemin qui y mène (hidraw).
- Le `1189:8890` et les claviers qui n’indiquent pas leur type ne peuvent pas communiquer leurs réglages ; seuls ceux
  définis dans l’app sont donc affichés.
- Un seul clavier à la fois. Si plusieurs sont branchés, l’app prend le premier qu’elle trouve – un `1189:8890` en
  priorité.
- Le déplacement de la souris et le glissement ne sont pas proposés.

## Licence

MIT – voir [LICENSE](../LICENSE). L’icône de l’app : CC BY-SA 4.0 – voir
[LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt).
