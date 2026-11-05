<p align="center"><img src="../.github/icon.png" width="128" height="128" alt="ไอคอนแอป"></p>

<h1 align="center">เครื่องมือตั้งค่าแป้นพิมพ์มาโครสำหรับ Linux</h1>

<p align="center">
  <img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white">
  <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white">
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"></a>
</p>

<p align="center"><a href="../README.md">English</a> · <a href="README.ko.md">한국어</a> · <a href="README.ar.md">العربية</a> · <a href="README.de.md">Deutsch</a> · <a href="README.es.md">Español</a> · <a href="README.es-419.md">Español (Latinoamérica)</a> · <a href="README.fr.md">Français</a> · <a href="README.id.md">Bahasa Indonesia</a> · <a href="README.it.md">Italiano</a> · <a href="README.ja.md">日本語</a> · <a href="README.pl.md">Polski</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.pt-PT.md">Português (Portugal)</a> · <a href="README.ru.md">Русский</a> · <b>ไทย</b> · <a href="README.vi.md">Tiếng Việt</a> · <a href="README.zh-Hans.md">简体中文</a> · <a href="README.zh-Hant.md">繁體中文</a></p>

แอป Linux ที่เขียนการตั้งค่าปุ่มลงในแป้นพิมพ์มาโคร USB ขนาดเล็ก — แบบที่วางขายตั้งแต่รุ่นที่มีปุ่มหมุนเพียงปุ่มเดียวไปจนถึงรุ่นที่มีปุ่ม 21 ปุ่ม และมีปุ่มหมุนได้สูงสุดสี่ปุ่ม สำหรับปุ่มแต่ละปุ่มและการทำงานแต่ละแบบของปุ่มหมุน (หมุนซ้าย, กด, หมุนขวา) แอปจะกำหนดการกดปุ่ม ปุ่มสื่อ หรือการทำงานของเมาส์ในเลเยอร์ 1–3 และตั้งค่าแสงไฟของแป้นพิมพ์ได้ด้วย แอปนี้เป็นรุ่น Linux ของแอป macOS ที่สร้างตามกฎเดียวกัน และใช้คำสั่งเดียวกัน

<p align="center"><img src="../.github/devices.png" width="560" alt="แป้นพิมพ์มาโคร USB ขนาดเล็ก 2 ตัว: ปุ่ม 6 ปุ่มกับปุ่มหมุน 1 ปุ่ม และปุ่ม 12 ปุ่มกับปุ่มหมุน 2 ปุ่ม"></p>

| แป้นพิมพ์ | เลย์เอาต์ | อ่านค่ากลับ | หน่วงเวลา | แสงไฟ | ทดสอบกับเครื่องจริง |
|---|---|---|---|---|---|
| `1189:8890` | คุณจัดเอง — ปุ่ม (สูงสุด 12), ปุ่มต่อแถว, ปุ่มหมุน (สูงสุด 2) | ไม่ได้ | ไม่มี | สามโหมด | ทดสอบแล้ว — ปุ่ม 6 ปุ่มและปุ่มหมุน 1 ปุ่ม |
| ผู้ผลิต `1189` หรือ `514C`, ผลิตภัณฑ์ `8840` · `8842` · `8830`–`8833` · `8850` · `8851`, เฟิร์มแวร์รุ่นใหม่ | ตามที่แป้นพิมพ์แจ้ง — 26 เลย์เอาต์ | ทันทีที่เชื่อมต่อ | ก่อนการกดแต่ละครั้ง | หนึ่งโหมด และสีของปุ่ม 1–16 แต่ละปุ่ม | ทดสอบแล้ว — `514C:8850`, ปุ่ม 12 ปุ่มและปุ่มหมุน 2 ปุ่ม |
| แป้นพิมพ์กลุ่มเดียวกัน, เฟิร์มแวร์รุ่นเก่า | ตามที่แป้นพิมพ์แจ้ง | ทันทีที่เชื่อมต่อ | ปุ่มละหนึ่งค่า | สีและโหมด | ยังไม่ได้ทดสอบ |
| แป้นพิมพ์กลุ่มเดียวกันที่ไม่บอกประเภท | คุณเลือกเอง — 26 เลย์เอาต์ | ไม่ได้ | ไม่มี | สีและโหมด | ยังไม่ได้ทดสอบ |

`1189:8890` คือแป้นพิมพ์ที่เครื่องมือตั้งค่าบน Windows ของผู้ผลิต “MINI KeyBoard” รองรับ ส่วนแป้นพิมพ์อื่นคือแป้นพิมพ์ที่แอปข้ามแพลตฟอร์มของผู้ผลิต “MINI_KEYBOARD” ค้นหา แอปนี้ใช้คำสั่งของเครื่องมือทั้งสอง แป้นพิมพ์กลุ่มที่สองส่วนใหญ่จะบอกว่ามีปุ่มและปุ่มหมุนกี่ปุ่ม และใช้เฟิร์มแวร์แบบใด แอปจึงวาดเลย์เอาต์แบบเดียวกับที่แอปของผู้ผลิตวาด ส่วนแป้นพิมพ์ที่ไม่บอกจะใช้เลย์เอาต์ที่คุณเลือกจากเลย์เอาต์เหล่านั้น แอปของผู้ผลิตไม่รู้จัก `1189:8890` และเครื่องมือของแป้นพิมพ์นี้เองก็ไม่ได้วาดรูปทรงไว้ จึงไม่ทราบรูปทรงจริงของแป้นพิมพ์ เพียงบอกแอปว่ามีปุ่มและปุ่มหมุนกี่ปุ่ม และแถวหนึ่งมีกี่ปุ่ม แอปก็จะวาดให้เป็นระเบียบตามนั้น

แอปมีให้ใช้ 18 ภาษา ได้แก่ อังกฤษ เกาหลี ญี่ปุ่น จีน (ตัวย่อและตัวเต็ม) เยอรมัน สเปน (สเปนและละตินอเมริกา) ฝรั่งเศส อิตาลี โปรตุเกส (บราซิลและโปรตุเกส) โปแลนด์ รัสเซีย อินโดนีเซีย เวียดนาม ไทย และอาหรับ โดยจะแสดงตามภาษาของเดสก์ท็อป ส่วนภาษาอื่นจะแสดงเป็นภาษาอังกฤษ

ซอร์สโค้ดในที่เก็บนี้ใช้สัญญาอนุญาต MIT ส่วนไอคอนแอป — ทั้งใน `data/icons/` และภาพด้านบน — ใช้สัญญาอนุญาต CC BY-SA 4.0 ([`LICENSES/CC-BY-SA-4.0.txt`](../LICENSES/CC-BY-SA-4.0.txt))

## macOS, Windows, Linux และเว็บ

MINIKeyboard มีสี่รุ่นที่สร้างตามกฎเดียวกัน — ใช้คำสั่งเดียวกันและมี 18 ภาษาเหมือนกัน แอปทั้งสามรองรับแป้นพิมพ์ชุดเดียวกัน ส่วนรุ่นเว็บซึ่งทำงานในเบราว์เซอร์รองรับแป้นพิมพ์ทั้งหมดนั้นยกเว้น `1189:8890`

| ที่เก็บ | GitHub |
|---|---|
| **MINIKeyboardApple**<br><img alt="macOS 14+" src="https://img.shields.io/badge/macOS-14%2B-0A84FF?logo=apple&logoColor=white"> <img alt="Universal: Apple Silicon + Intel" src="https://img.shields.io/badge/Universal-Apple%20Silicon%20%2B%20Intel-5E5CE6"> <img alt="Swift 6" src="https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardApple](https://github.com/hobbyworker/MINIKeyboardApple) |
| **MINIKeyboardWin**<br><img alt="Windows 10 (2004)+ · 11" src="https://img.shields.io/badge/Windows-10%20(2004)%2B%20·%2011-0078D4?logo=windows&logoColor=white"> <img alt="x64 · ARM64" src="https://img.shields.io/badge/x64%20·%20ARM64-5E5CE6"> <img alt=".NET 10 · WinUI 3" src="https://img.shields.io/badge/.NET%2010-WinUI%203-512BD4?logo=dotnet&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardWin](https://github.com/hobbyworker/MINIKeyboardWin) |
| **MINIKeyboardLinux**<br><img alt="Linux" src="https://img.shields.io/badge/Linux-GTK%204%20·%20libadwaita-4A86CF?logo=linux&logoColor=white"> <img alt="Flatpak: x86_64 · aarch64" src="https://img.shields.io/badge/Flatpak-x86__64%20·%20aarch64-4A90D9?logo=flatpak&logoColor=white"> <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-B7410E?logo=rust&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardLinux](https://github.com/hobbyworker/MINIKeyboardLinux) |
| **MINIKeyboardWeb**<br><img alt="Chrome · Edge" src="https://img.shields.io/badge/Chrome%20·%20Edge-WebHID-4285F4?logo=googlechrome&logoColor=white"> <img alt="Windows · macOS · Linux" src="https://img.shields.io/badge/Windows%20·%20macOS%20·%20Linux-5E5CE6"> <img alt="TypeScript · Svelte 5" src="https://img.shields.io/badge/TypeScript-Svelte%205-3178C6?logo=typescript&logoColor=white"> <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-yellow"> | [github.com/hobbyworker/MINIKeyboardWeb](https://github.com/hobbyworker/MINIKeyboardWeb) |

## ดาวน์โหลด

เปิด [me.hobbyworker.MINIKeyboard.flatpakref](https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref) ในศูนย์ซอฟต์แวร์ หรือเรียกใช้คำสั่งนี้:

```sh
flatpak install --user https://mini-kb.hobbyworker.me/updates/linux/me.hobbyworker.MINIKeyboard.flatpakref
```

ขั้นตอนนี้จะเพิ่มที่เก็บ Flatpak ของแอปเอง และการติดตั้งครั้งแรกจะดึงรันไทม์ GNOME จาก Flathub มาด้วย หลังจากนั้นศูนย์ซอฟต์แวร์ — หรือ `flatpak update` — จะนำเวอร์ชั่นใหม่มาให้ การตั้งค่าของคุณจะยังอยู่เหมือนเดิม

แต่ละเวอร์ชั่นยังมีให้ในหน้า [Releases](https://github.com/hobbyworker/MINIKeyboardLinux/releases) เป็นบันเดิล Flatpak — `MINIKeyboard-<เวอร์ชั่น>-x86_64.flatpak` สำหรับ Intel และ AMD, `-aarch64.flatpak` สำหรับ ARM — สำหรับคอมพิวเตอร์ที่ไม่ได้เชื่อมต่ออินเทอร์เน็ต หรือเมื่อต้องการเวอร์ชั่นใดเวอร์ชั่นหนึ่งโดยเฉพาะ บันเดิลติดตั้งได้ด้วย `flatpak install --user <ไฟล์>` และหลังจากนั้นจะรับอัปเดตจากที่เก็บเดียวกัน

## การให้แอปเปิดแป้นพิมพ์ได้

Linux จะให้แอปเปิดอุปกรณ์ USB ได้ก็ต่อเมื่อมีกฎ udev ที่มอบอุปกรณ์นั้นให้ผู้ใช้ที่เข้าระบบอยู่ และ Flatpak ติดตั้งกฎนี้เองไม่ได้ ในครั้งแรก แอปจะแจ้งว่าต้องได้รับสิทธิ์เข้าถึง และ **วิธีอนุญาต…** จะแสดงคำสั่งที่เขียนกฎนั้น พร้อมปุ่มสำหรับคัดลอก ให้เรียกใช้คำสั่งเหล่านั้นในเทอร์มินัลหนึ่งครั้ง แล้วเสียบแป้นพิมพ์ใหม่อีกครั้ง ถ้าโคลนที่เก็บนี้ไว้ กฎเดียวกันนี้คือ [`data/60-minikeyboard.rules`](../data/60-minikeyboard.rules):

```sh
sudo install -m 644 data/60-minikeyboard.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

- แอปเขียนลง `1189:8890` ผ่าน USB โดยตรง (libusb) เพราะช่องทางการตั้งค่าของแป้นพิมพ์นี้มีเพียง OUT endpoint เดียว จึงไม่มีไดรเวอร์ใดรับไปจัดการ
- สำหรับแป้นพิมพ์อื่น แอปเขียนผ่านอุปกรณ์ hidraw ของอินเทอร์เฟซ USB ตัวแรก เช่นเดียวกับแอปของผู้ผลิต ระหว่างนั้นแป้นพิมพ์ยังพิมพ์ได้ตามปกติ

## การบิลด์

```sh
make build    # target/release/minikeyboard สำหรับ PREFIX (/usr/local ถ้าไม่ได้ระบุ)
make install  # หลัง make build: โปรแกรม, คำแปล, ไฟล์ desktop, metainfo, ไอคอน และสัญญาอนุญาต ลงใน DESTDIR/PREFIX
make test     # การทดสอบของทุก crate — ไม่เปิดหน้าต่าง และไม่แตะแป้นพิมพ์
make check    # rustfmt และ clippy โดยคำเตือนถือเป็นข้อผิดพลาด
make run      # บิลด์ debug พร้อมคำแปล
```

ต้องใช้ Rust 1.88 ขึ้นไป และไฟล์สำหรับพัฒนาของ GTK 4, libadwaita 1.7 ขึ้นไป, libsoup 3 และ libusb 1.0 — บน Debian และ Ubuntu คือ `libgtk-4-dev libadwaita-1-dev libsoup-3.0-dev libusb-1.0-0-dev` บน Fedora คือ `gtk4-devel libadwaita-devel libsoup3-devel libusb1-devel` — และ `msgfmt` ของ gettext สำหรับคำแปล

ID ของแอป (`me.hobbyworker.MINIKeyboard`) ชื่อ และลิงก์เป็นของแอปนี้เอง และตรงกับไฟล์ desktop, metainfo และไอคอนใน `data/` ถ้าเป็น fork ให้ระบุค่าของตัวเองตอนบิลด์: `MINIKEYBOARD_APP_ID` (ใช้เป็นชื่อไอคอนด้วย), `MINIKEYBOARD_APP_NAME`, `MINIKEYBOARD_COPYRIGHT`, `MINIKEYBOARD_WEBSITE` และ `MINIKEYBOARD_ISSUE_URL` ตำแหน่งของคำแปลนั้น `make build` จะกำหนดจาก `PREFIX` ส่วน `MINIKEYBOARD_LOCALE_DIR` ตอนเรียกใช้จะแทนที่ค่านั้นได้ เมื่อต้องการลองบิลด์

แอปยังแจ้งเวอร์ชั่นใหม่เองได้ด้วย บิลด์ที่ระบุ `MINIKEYBOARD_UPDATE_FEED` จะอ่านฟีด RSS นั้นตอนเปิดแอปและวันละครั้ง — ยกเว้นเมื่อปิด **ตรวจหารายการอัปเดตโดยอัตโนมัติ** ใน “ปรับแต่ง” — และเมื่อกด **ตรวจหารายการอัปเดต…** แล้วแสดงแบนเนอร์ที่ลิงก์ไปยังหน้าดาวน์โหลดของเวอร์ชั่นใหม่ บิลด์ที่แอปเผยแพร่เองไม่ได้ใช้ส่วนนี้ — Flatpak เป็นผู้นำอัปเดตมาให้ — จึงไม่มีรายการเมนูหรือการตั้งค่านี้

เมนูย่อย **โปรแกรมจำลอง** ช่วยให้ลองใช้แอปได้โดยไม่ต้องมีแป้นพิมพ์รุ่นนั้น เลือก `1189:8890`, เลย์เอาต์ของเฟิร์มแวร์รุ่นใหม่ หรือแป้นพิมพ์ที่ไม่บอกประเภท แล้วแอปจะทำงานเหมือนมีแป้นพิมพ์เสียบอยู่ — ด้วยคำสั่งเดียวกัน โดยไม่มีอะไรส่งออกไปทาง USB ระหว่างใช้งาน แถบสีส้มที่ด้านบนของหน้าต่างจะแจ้งให้ทราบ และ **กลับไปใช้แป้นพิมพ์ที่เชื่อมต่อผ่าน USB** จะพากลับมา บิลด์ Debug มีให้เลือกทุกแบบ (Report ID อื่นของ `1189:8890` และเฟิร์มแวร์รุ่นเก่า) และ `MINIKEYBOARD_SIMULATED_KEYBOARD` (เช่น `8890-3`) จะเริ่มแอปด้วยแป้นพิมพ์จำลองตัวนั้น **ประวัติอุปกรณ์** แสดงรีพอร์ตทุกรายการที่เขียนไปและคำตอบทุกรายการ

## วิธีใช้

1. เชื่อมต่อแป้นพิมพ์ผ่าน USB แอปจะค้นหาแป้นพิมพ์ และถามประเภทจากแป้นพิมพ์กลุ่มที่สอง ถ้าแป้นพิมพ์ตอบ แอปจะอ่านค่าทันที
2. แป้นพิมพ์ที่ไม่บอกรูปทรงจะให้เลือกเลย์เอาต์ในครั้งแรก — หนึ่งในเลย์เอาต์ของแอปผู้ผลิต หรือสำหรับ `1189:8890` คือจำนวนปุ่ม, ปุ่มต่อแถว และปุ่มหมุน — รวมถึงทิศทางที่แป้นพิมพ์วางอยู่ตรงหน้าคุณ แอปจะจำทั้งสองอย่างไว้ตาม USB ID นั้น เปลี่ยนภายหลังได้ด้วย **รูปทรง…** และ **หมุน** (Ctrl+T) ส่วนแป้นพิมพ์ที่บอกรูปทรงแล้วจะหมุนได้อย่างเดียว
3. เลือกเลเยอร์ แล้วเลือกปุ่มหรือการทำงานของปุ่มหมุนบนภาพแป้นพิมพ์
4. กำหนดการทำงาน:
   - **ปุ่ม** — กดปุ่มปรับค่าค้างไว้ (Ctrl, Alt, Shift, Super และปุ่มทางด้านขวา) แล้วคลิกปุ่ม การคลิกแต่ละครั้งจะเพิ่มการกดหนึ่งครั้ง **ปุ่มปรับค่าเท่านั้น** ทำให้ปุ่มนั้นกลายเป็นปุ่มปรับค่า **QWERTY** เรียงปุ่มแบบแป้นพิมพ์ขนาดเต็ม ส่วน **ABC** เรียงเป็นกลุ่ม คลิกได้เฉพาะปุ่มที่แป้นพิมพ์รองรับ
   - **มัลติมีเดีย**, **เมาส์** — แทนที่สิ่งที่ปุ่มเคยทำ
   - **LED** — แสงไฟของเลเยอร์ ในเฟิร์มแวร์รุ่นใหม่ ให้เลือกปุ่มใดปุ่มหนึ่งจากปุ่ม 1–16 เพื่อกำหนดสีให้ปุ่มนั้น
   - **หน่วงเวลา** — สำหรับเฟิร์มแวร์ที่เก็บค่าหน่วงเวลาได้
5. **บันทึก** (Ctrl+S) จะเขียนสิ่งที่เปลี่ยนแปลง **อ่าน** (Ctrl+R) จะดึงการตั้งค่าปัจจุบันจากแป้นพิมพ์ที่อ่านค่าได้

**ปรับแต่ง** (Ctrl+,) ใช้เลือกภาษาที่แอปแสดง (หนึ่งใน 18 ภาษา หรือภาษาของเดสก์ท็อป) และรูปแบบสว่างหรือมืด ในบิลด์ที่มีฟีดอัปเดต หน้า “ทั่วไป” ในนั้นจะมีการตรวจหารายการอัปเดตอัตโนมัติ **เกี่ยวกับ MINIKeyboard** แสดงเวอร์ชั่นและสัญญาอนุญาตโอเพนซอร์ส

ใน **ปรับแต่ง ▸ ทั่วไป ▸ วิธีเลี่ยงข้อบกพร่องของเฟิร์มแวร์** มี **ป้องกันการกดปุ่มซ้ำหลังเปลี่ยนแสงไฟ** ซึ่งเปิดไว้เป็นค่าเริ่มต้น หลังบันทึกแสงไฟ เฟิร์มแวร์ของ `1189:8890` จะพิมพ์อักขระแรกของปุ่มซ้ำไม่หยุด — ซอฟต์แวร์ของผู้ผลิตก็เป็นเช่นนี้ — แอปจึงส่งคำสั่งบันทึกอีกหนึ่งครั้งหลังแสงไฟ โดยไม่มีปุ่มใดอยู่ในนั้น เมื่อปิด แอปจะส่งเฉพาะสิ่งที่ซอฟต์แวร์ของผู้ผลิตส่ง นอกจากนี้ แป้นพิมพ์นี้ยังลืมการตั้งค่าแสงไฟเมื่อถอดออกด้วย

## โครงสร้าง

กฎทั้งหมดอยู่ใน crate ต่างๆ ที่การพึ่งพาชี้เข้าด้านในเท่านั้น — เลเยอร์ที่เอื้อมออกไปด้านนอกจะคอมไพล์ไม่ผ่าน

```
crates/
├── domain/        แป้นพิมพ์, การทำงาน, ตัวควบคุมของแต่ละเลย์เอาต์ และสิ่งที่แต่ละแป้นพิมพ์เก็บได้; port; use case (ไลบรารีมาตรฐานเท่านั้น)
├── datalayer/     คำสั่งของแต่ละแป้นพิมพ์และหมายเลขที่ตัวควบคุมใช้; การค้นหาแป้นพิมพ์ใน sysfs และการเขียนผ่าน USB (libusb) หรือ hidraw; แป้นพิมพ์จำลอง
├── presentation/  view model และค่าที่หน้าจอวาด — รูปทรงเรขาคณิตของเลย์เอาต์, ตารางปุ่มและสี, กฎ udev (ไม่มี toolkit)
├── ui/            GTK 4 และ libadwaita — เห็นเฉพาะ presentation; การแจ้งเวอร์ชั่นใหม่ (libsoup)
└── minikeyboard/  composition root: crate เดียวที่เห็นทุกส่วน
po/                คำแปล ภาษาละหนึ่งไฟล์
data/              ไฟล์ desktop, metainfo และไอคอน รวมถึงกฎ udev
LICENSES/          สัญญาอนุญาตของไอคอน (CC BY-SA 4.0)
```

เกณฑ์คือไม่มีคำเตือนเลย คำเตือนต้องแก้ ไม่ใช่ปิดไว้

## ข้อจำกัด

- จนถึงตอนนี้ได้ทดสอบการเขียนกับแป้นพิมพ์ 2 ตัวบน Fedora ได้แก่ `1189:8890` ที่มีปุ่ม 6 ปุ่มและปุ่มหมุน 1 ปุ่ม และ `514C:8850` ที่มีปุ่ม 12 ปุ่มและปุ่มหมุน 2 ปุ่ม บนเฟิร์มแวร์รุ่นใหม่ ซึ่งเปิดผ่าน hidraw ส่วนแป้นพิมพ์อื่น แอปจะเขียนแบบเดียวกับที่แอปของผู้ผลิตเขียน — คำสั่งตรงกับของแอป macOS ทุกไบต์ และมีการทดสอบยืนยันไว้ — แต่ยังไม่ได้ทดสอบกับแป้นพิมพ์จริงเลยสักตัว
- `1189:8890` และแป้นพิมพ์ที่ไม่บอกประเภทไม่สามารถแจ้งการตั้งค่าปัจจุบันได้ แอปจึงแสดงเฉพาะสิ่งที่ตั้งไว้ในแอป
- เปิดแป้นพิมพ์ได้ทีละตัว ถ้าเสียบไว้หลายตัว ให้เลือกด้วยปุ่มข้างสถานะการเชื่อมต่อ แอปจะเปิดตัวที่เลือกไว้ครั้งก่อน หรือถ้าไม่มี ก็จะเปิดตัวแรกที่พบ โดย `1189:8890` มาก่อน
- ไม่รองรับการเลื่อนเมาส์และการลาก

## สัญญาอนุญาต

MIT — ดู [LICENSE](../LICENSE) ไอคอนแอป: CC BY-SA 4.0 — ดู [LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt)
