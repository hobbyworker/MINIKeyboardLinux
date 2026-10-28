<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="Ícone do app"></p>

<h1 align="center">Configurador de teclados de macro para Linux</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <b>Português (Brasil)</b> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

Um app para Linux que grava as configurações das teclas em pequenos teclados de macro USB. Eles são vendidos em
vários tamanhos: desde os que têm só um botão giratório até os de 21 teclas, com até quatro botões giratórios. Para
cada tecla e cada ação de botão giratório (girar à esquerda, pressionar, girar à direita), o app define nas
camadas 1–3 toques de teclas, uma tecla multimídia ou uma ação do mouse, além da iluminação do teclado. É a versão
para Linux de um app para macOS feito com as mesmas regras, e usa os mesmos comandos.

<p align="center"><img src="../.github/devices.png" width="560" alt="Dois pequenos teclados de macros USB: seis teclas e um botão giratório, doze teclas e dois botões giratórios"></p>

| Teclado | Disposição | Leitura | Atraso | Iluminação | Testado |
|---|---|---|---|---|---|
| `1189:8890` | definida por você — teclas (até 12), teclas por fileira, botões giratórios (até 2) | não | não | três modos | sim — 6 teclas e um botão giratório |
| fabricante `1189` ou `514C`, produto `8840` · `8842` · `8830`–`8833` · `8850` · `8851`, firmware novo | conforme o que o teclado informa — 26 disposições | ao conectar | antes de cada toque | um modo e uma cor para cada uma das teclas 1–16 | sim — `514C:8850`, 12 teclas e 2 botões giratórios |
| os mesmos, firmware antigo | conforme o que o teclado informa | ao conectar | um por tecla | uma cor e um modo | não |
| os mesmos, sem informar o tipo | escolhida por você — 26 disposições | não | não | uma cor e um modo | não |

O `1189:8890` é o teclado com que trabalha a ferramenta de configuração para Windows do fabricante, “MINI KeyBoard”;
os demais são os que o app multiplataforma do fabricante, “MINI_KEYBOARD”, procura. O app usa os comandos das duas
ferramentas. A maioria dos teclados do segundo tipo informa quantas teclas e botões giratórios tem e qual firmware
usa, então o app desenha a mesma disposição que o app do fabricante desenharia; para um teclado que não informa
isso, ele usa a disposição que você escolher entre essas. O app do fabricante não conhece o `1189:8890`, e a
própria ferramenta dele não desenha nenhuma forma, então não se sabe qual é a forma real. Por isso, você informa
quantas teclas e botões giratórios ele tem e quantas teclas há em cada fileira, e o app desenha tudo isso de
forma organizada.

O app está disponível em 13 idiomas — inglês, coreano, japonês, chinês (simplificado e tradicional), alemão,
espanhol, francês, italiano, português (Brasil), polonês, indonésio e vietnamita — e segue o idioma da área de
trabalho. Em qualquer outro idioma, ele aparece em inglês.

O código-fonte deste repositório está sob a licença MIT. O ícone do app — o de `data/icons/` e a imagem no topo —
está sob a licença CC BY-SA 4.0 ([`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt)).

## macOS, Windows e Linux

O MINIKeyboard tem três edições feitas com as mesmas regras — os mesmos teclados, os mesmos comandos, os mesmos 13 idiomas.

| Repositório | GitHub |
|---|---|
| **MINIKeyboardApple**<br><img alt="macOS 14+" src="https://img.shields.io/badge/macOS-14%2B-0A84FF?logo=apple&logoColor=white"> <img alt="Universal: Apple Silicon + Intel" src="https://img.shields.io/badge/Universal-Apple%20Silicon%20%2B%20Intel-5E5CE6"> <img alt="Swift 6" src="https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardApple](https://github.com/hobbyworker/MINIKeyboardApple) |
| **MINIKeyboardWin**<br><img alt="Windows 10 (2004)+ · 11" src="https://img.shields.io/badge/Windows-10%20(2004)%2B%20·%2011-0078D4?logo=windows&logoColor=white"> <img alt="x64 · ARM64" src="https://img.shields.io/badge/x64%20·%20ARM64-5E5CE6"> <img alt=".NET 10 · WinUI 3" src="https://img.shields.io/badge/.NET%2010-WinUI%203-512BD4?logo=dotnet&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardWin](https://github.com/hobbyworker/MINIKeyboardWin) |
| **MINIKeyboardLinux**<br><img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white"> <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white"> <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardLinux](https://github.com/hobbyworker/MINIKeyboardLinux) |

## Download

Abra [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) na central de programas ou rode:

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

Isso adiciona o repositório Flatpak do próprio app, e a primeira instalação também baixa o runtime do GNOME do
Flathub. A partir daí, a central de programas — ou o `flatpak update` — traz as versões novas. Suas configurações são
mantidas.

Cada versão também está em [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) como pacote Flatpak:
`MINIKeyboard-<versão>-x86_64.flatpak` para Intel e AMD, e `-aarch64.flatpak` para ARM. Serve para um PC sem conexão
com a internet ou quando você precisa de uma versão específica. O pacote é instalado com
`flatpak install --user <arquivo>` e depois recebe as atualizações do mesmo repositório.

## Permitir que o app abra o teclado

O Linux só deixa um app abrir um dispositivo USB quando uma regra do udev o concede ao usuário que iniciou a sessão,
e um Flatpak não consegue instalar essa regra. Na primeira vez, quando o app avisar que precisa de permissão de
acesso, **Como permitir…** mostra os comandos que gravam a regra e um botão para copiá-los. Rode-os uma vez em um
terminal e conecte o teclado de novo. Se você clonou este repositório, a mesma regra está em
[`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules):

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- O `1189:8890` é gravado diretamente por USB (libusb): a interface de configuração dele só tem um endpoint OUT,
  então nenhum driver o assume.
- Os demais teclados são gravados pelo dispositivo hidraw da primeira interface USB deles, como faz o app do
  fabricante. Enquanto isso, você continua digitando normalmente com o teclado.

## Compilação

```sh
make build    # target/release/minikeyboard, para PREFIX (/usr/local se não for informado)
make install  # depois de make build: o programa, as traduções, o arquivo .desktop, o metainfo, os ícones e as licenças, em DESTDIR/PREFIX
make test     # testes de todos os crates — nenhuma janela abre, nenhum teclado é tocado
make check    # rustfmt e clippy; avisos são erros
make run      # uma compilação Debug com as traduções
```

É preciso ter o Rust 1.88 ou posterior e os arquivos de desenvolvimento do GTK 4, libadwaita 1.7 ou posterior,
libsoup 3 e libusb 1.0: no Debian e no Ubuntu, `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev`; no
Fedora, `gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel`. Para as traduções, é preciso o `msgfmt` do
gettext.

O ID do app (`me.hobbyworker.MINIKeyboard`), o nome e os links são do próprio app e correspondem ao
arquivo .desktop, ao metainfo e aos ícones em `data/`. Um fork informa os seus na compilação: `MINIKEYBOARD_APP_ID`
(que também é o nome do ícone), `MINIKEYBOARD_APP_NAME`, `MINIKEYBOARD_COPYRIGHT`, `MINIKEYBOARD_WEBSITE` e
`MINIKEYBOARD_ISSUE_URL`. O `make build` define a partir de `PREFIX` o local onde ficam as traduções, e o
`MINIKEYBOARD_LOCALE_DIR`, na hora da execução, o substitui — para experimentar uma compilação.

O app também pode avisar por conta própria sobre versões novas. Uma compilação com `MINIKEYBOARD_UPDATE_FEED` lê esse
feed RSS ao iniciar e uma vez por dia — a menos que **Buscar atualizações automaticamente** esteja desativado em
Preferências — e também ao clicar em **Buscar Atualizações…**, e mostra um banner que leva à página de download da
versão nova. As compilações do próprio app não usam isso — as atualizações delas vêm pelo Flatpak —, então elas não
têm esse item de menu nem esse ajuste.

O submenu **Simulador** permite experimentar o app sem o teclado em questão: escolha um `1189:8890`, uma disposição do
firmware mais novo ou um teclado que não informe o tipo, e o app age como se ele estivesse conectado — com os mesmos
comandos, sem enviar nada por USB. Enquanto ele estiver em uso, uma barra laranja no topo da janela avisa, e **Voltar ao
teclado via USB** volta ao teclado real. As compilações Debug oferecem todos os tipos (os outros IDs do relatório do
`1189:8890`, o firmware antigo), e com `MINIKEYBOARD_SIMULATED_KEYBOARD` (por exemplo, `8890-3`) o app já inicia com esse
teclado. **Registro do Dispositivo** mostra todos os relatórios gravados e todas as respostas.

## Como usar

1. Conecte o teclado por USB. O app o encontra e, se ele for do segundo tipo, pergunta qual é o teclado; se
   ele responder, já o lê na hora.
2. Na primeira vez, para um teclado que não informa a forma, você define a disposição — uma das disposições do app
   do fabricante ou, no `1189:8890`, o número de teclas, de teclas por fileira e de botões giratórios — e a posição
   em que ele fica na sua frente. O app guarda as duas coisas para aquele ID USB. Depois, você pode mudá-las com
   **Forma…** e **Girar** (Ctrl+T). Um teclado que informou a forma só pode ser girado.
3. Escolha uma camada e, no desenho do teclado, uma tecla ou uma ação de botão giratório.
4. Atribua uma ação:
   - **Teclas** — segure as teclas modificadoras (Ctrl, Alt, Shift, Super e as do lado direito) e clique nas teclas;
     cada clique adiciona um toque. **Só modificadoras** transforma a tecla em uma tecla modificadora. **QWERTY**
     dispõe as teclas como num teclado completo, e **ABC**, em grupos. Só é possível clicar nas teclas que o teclado
     aceita.
   - **Multimídia**, **Mouse** — substituem o que a tecla fazia.
   - **LED** — a iluminação da camada. No firmware novo, escolha uma das teclas 1–16 para dar uma cor a ela.
   - **Atraso** — no firmware que guarda os atrasos.
5. **Salvar** (Ctrl+S) grava o que mudou. **Ler** (Ctrl+R) traz a configuração atual dos teclados que permitem a
   leitura.

Em **Preferências** (Ctrl+,), você escolhe o idioma da interface (um dos 13 ou o da área de trabalho) e o modo claro
ou escuro. Em uma compilação com feed de atualizações, a página Geral tem a busca automática de atualizações. Em
**Sobre o App MINIKeyboard**, você vê a versão e as licenças de código aberto.

Em **Preferências ▸ Geral ▸ Contornos para falhas do firmware**, **Evitar a repetição de teclas após mudar a
iluminação** vem ativado por padrão. O firmware do `1189:8890` repete sem parar o primeiro caractere de uma tecla
depois que a iluminação é salva (o software do fabricante faz o mesmo), então o app envia mais um comando de salvar,
sem nenhuma tecla, depois da iluminação. Desativado, o app envia apenas o mesmo que o software do fabricante. Esse
teclado também esquece a iluminação quando é desconectado.

## Estrutura

As regras ficam em crates cujas dependências apontam só para dentro: uma camada que tente alcançar algo de fora não
compila.

```
crates/
├── domain/        teclados, ações, os controles de cada disposição e o que cada teclado comporta; as portas; casos de uso (só a biblioteca padrão)
├── datalayer/     os comandos de cada teclado e a numeração dos controles; encontrar o teclado no sysfs e gravar por USB (libusb) ou hidraw; teclados simulados
├── presentation/  view models e os valores que as telas desenham — a geometria das disposições, tabelas de teclas e cores, a regra do udev (sem toolkit)
├── ui/            GTK 4 e libadwaita — só vê presentation; o aviso de versão nova (libsoup)
└── minikeyboard/  a composition root: o único lugar que conhece todos os crates
po/                traduções, um arquivo por idioma
data/              o arquivo .desktop, o metainfo e os ícones, e a regra do udev
LICENSES/          a licença do ícone (CC BY-SA 4.0)
```

Zero avisos é o padrão. Avisos são corrigidos, não desativados.

## Limitações

- Até agora, a gravação foi testada em dois teclados, no Fedora: um `1189:8890` com seis teclas e um botão giratório e
  um `514C:8850` com doze teclas e dois botões giratórios, com firmware novo, aberto pelo hidraw. Os demais teclados
  são gravados como o app do fabricante os grava — os comandos são os do app para macOS, byte a byte, e os testes os
  fixam —, mas nenhum deles foi testado em um teclado real ainda.
- O `1189:8890` e os teclados que não informam o tipo não conseguem informar a configuração atual, então o app
  mostra só o que foi definido nele.
- Um teclado aberto por vez. Com vários conectados, o botão ao lado do status da conexão escolhe qual; o app abre o
  escolhido da última vez ou, se não houver, o primeiro que encontrar — um `1189:8890` primeiro.
- Não há ações para mover o mouse nem para arrastar.

## Licença

MIT — veja [LICENSE](../LICENSE). O ícone do app: CC BY-SA 4.0 — veja
[LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt).
