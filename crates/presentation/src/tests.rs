//! The drawing and the tables, from inside the crate.

use std::collections::HashSet;

use domain::{
    ControlId, DeviceDetails, DeviceInterface, GridShape, KeyMacro, KeyStroke, KeyboardLayout, MediaKey, Modifiers,
    Rotation, UsagePair,
};

use crate::choices::{ActionCaption, MediaChoice, ModifierKey, MouseChoice, Rgb};
use crate::drawing::{LayoutGeometry, LayoutRect};
use crate::packet_text;
use crate::screen_state::{BoardFace, ChannelPath, Firmware, InfoChannel, InfoCommands, InterfaceRole, KeyboardInfo};
use crate::state::ActionText;
use crate::tables::{FULL_KEYBOARD_WIDTH, KeyUsage, places};

/// A knob is one body in the shape, on its press action.
#[test]
fn knobs_are_one_body_each() {
    let six_and_one = LayoutGeometry::of(KeyboardLayout::named("6add1").unwrap()).unwrap();
    let press = six_and_one.rects[&ControlId::Knob(1, domain::KnobPart::Press)];
    assert_eq!(six_and_one.knob_outlines(), vec![(1, press)]);
    assert_eq!(LayoutGeometry::of(KeyboardLayout::named("0add1").unwrap()).unwrap().knob_outlines().len(), 1);
    for layout in KeyboardLayout::all() {
        let geometry = LayoutGeometry::of(layout).unwrap_or_else(|| panic!("{}", layout.id));
        assert_eq!(
            geometry.rects.keys().copied().collect::<HashSet<_>>(),
            layout.controls.iter().copied().collect(),
            "{}",
            layout.id
        );
        assert_eq!(geometry.knob_outlines().len(), layout.knob_count(), "{}", layout.id);
    }
}

#[test]
fn rotation_turns_the_geometry() {
    let geometry = LayoutGeometry::of(KeyboardLayout::named("6add1").unwrap()).unwrap();
    let quarter = geometry.rotated(Rotation::Quarter);
    assert!((quarter.bounds().width - geometry.bounds().height).abs() < 0.001);
    assert!((quarter.bounds().height - geometry.bounds().width).abs() < 0.001);
    let full = quarter.rotated(Rotation::Quarter).rotated(Rotation::Quarter).rotated(Rotation::Quarter);
    for (control, rect) in &geometry.rects {
        let back = full.rects[control];
        assert!((back.mid_x() - rect.mid_x()).abs() < 0.001 && (back.mid_y() - rect.mid_y()).abs() < 0.001);
    }
    assert_eq!(geometry.rotated(Rotation::Upright), geometry);
}

/// An arrangement: keys fill the rows from the top left; a knob stands at the middle of the keys, two at the
/// first and last rows, each with its turns either side of its press.
#[test]
fn arrangements() {
    let seven = LayoutGeometry::arrangement(&GridShape::new(7, 3, 2).unwrap());
    let (key1, key4) = (seven.rects[&ControlId::Key(1)], seven.rects[&ControlId::Key(4)]);
    assert!(key4.x == key1.x && key4.y > key1.y);
    assert_eq!(seven.rects[&ControlId::Key(7)].x, key1.x);

    let one = LayoutGeometry::arrangement(&GridShape::new(6, 3, 1).unwrap());
    let keys: Vec<LayoutRect> =
        one.rects.iter().filter(|(control, _)| control.is_key()).map(|(_, rect)| *rect).collect();
    let middle = (keys.iter().map(|rect| rect.y).fold(f64::INFINITY, f64::min)
        + keys.iter().map(LayoutRect::max_y).fold(0.0, f64::max))
        / 2.0;
    let press = one.rects[&ControlId::Knob(1, domain::KnobPart::Press)];
    assert!((press.mid_y() - middle).abs() < 0.001);
    assert_eq!(one.rects[&ControlId::Knob(1, domain::KnobPart::RotateLeft)].mid_y(), press.mid_y());

    let two = LayoutGeometry::arrangement(&GridShape::new(12, 4, 2).unwrap());
    assert!(
        (two.rects[&ControlId::Knob(1, domain::KnobPart::Press)].mid_y() - two.rects[&ControlId::Key(1)].mid_y()).abs()
            < 0.001
    );
    assert!(
        (two.rects[&ControlId::Knob(2, domain::KnobPart::Press)].mid_y() - two.rects[&ControlId::Key(9)].mid_y()).abs()
            < 0.001
    );
}

/// Nothing overlaps, however the keyboard is turned, and the body holds everything.
#[test]
fn arrangements_dont_overlap() {
    let overlaps =
        |a: &LayoutRect, b: &LayoutRect| a.x < b.max_x() && b.x < a.max_x() && a.y < b.max_y() && b.y < a.max_y();
    for keys in 0..=12 {
        for columns in 1..=keys.max(1) {
            for knobs in 0..=2 {
                let Some(shape) = GridShape::new(keys, columns, knobs) else { continue };
                for rotation in Rotation::ALL {
                    let geometry = LayoutGeometry::arrangement(&shape).rotated(rotation);
                    assert_eq!(
                        geometry.rects.keys().copied().collect::<HashSet<_>>(),
                        shape.layout().controls.into_iter().collect()
                    );
                    let rects: Vec<&LayoutRect> = geometry.rects.values().collect();
                    for (i, a) in rects.iter().enumerate() {
                        for b in &rects[i + 1..] {
                            assert!(!overlaps(a, b), "{} {rotation:?}", shape.id());
                        }
                        assert!(
                            a.x >= geometry.frame.x && a.max_x() <= geometry.frame.max_x() + 0.001,
                            "{}",
                            shape.id()
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn strokes() {
    let stroke = KeyStroke::new(Modifiers::SUPER | Modifiers::SHIFT | Modifiers::CONTROL, 0x1D);
    assert_eq!(ActionText::stroke(&stroke), "Ctrl+Shift+Super+Z");
    assert_eq!(ActionText::stroke(&KeyStroke::new(Modifiers::ALT | Modifiers::RIGHT_SUPER, 0)), "Alt+RSuper");
    assert_eq!(ActionText::keys(&KeyMacro::new(vec![KeyStroke::key(0x04), KeyStroke::key(0x28)])), "A ↵");
    assert_eq!(ActionText::key(0xA0), "0xA0");
}

#[test]
fn key_usages_are_unique_and_named() {
    assert_eq!(KeyUsage::all().iter().map(|key| key.usage).collect::<HashSet<_>>().len(), KeyUsage::all().len());
    assert_eq!(KeyUsage::named(0x04).unwrap().label, "A");
    assert_eq!(KeyUsage::named(0x45).unwrap().label, "F12");
    assert_eq!(KeyUsage::named(0x68).unwrap().label, "F13");
    assert_eq!(KeyUsage::named(0x73).unwrap().label, "F24");
    assert_eq!(KeyUsage::named(0), None);
}

/// The full-size keyboard has every key but those a full-size keyboard doesn't, each once, and no two caps
/// overlap or run past the row.
#[test]
fn the_full_keyboard() {
    let keys: Vec<u8> = places()
        .iter()
        .filter_map(|place| match place.face {
            BoardFace::Key(key) => Some(key.usage),
            BoardFace::Modifier(_) => None,
        })
        .collect();
    let elsewhere: HashSet<u8> =
        KeyUsage::EXTRA_FUNCTION_KEYS.iter().chain(&KeyUsage::INTERNATIONAL).map(|key| key.usage).collect();
    assert_eq!(keys.len(), keys.iter().collect::<HashSet<_>>().len());
    let all: HashSet<u8> = KeyUsage::all().iter().map(|key| key.usage).collect();
    assert_eq!(keys.into_iter().collect::<HashSet<_>>(), all.difference(&elsewhere).copied().collect());
    let modifiers: HashSet<ModifierKey> = places()
        .iter()
        .filter_map(|place| match place.face {
            BoardFace::Modifier(modifier) => Some(modifier),
            BoardFace::Key(_) => None,
        })
        .collect();
    assert_eq!(modifiers, ModifierKey::ALL.into_iter().collect());
    for row in 0..6 {
        let mut caps: Vec<_> = places().iter().filter(|place| place.row == row).collect();
        caps.sort_by(|a, b| a.column.total_cmp(&b.column));
        assert!(caps.windows(2).all(|pair| pair[0].column + pair[0].width <= pair[1].column), "row {row}");
        assert!(caps.last().map_or(0.0, |cap| cap.column + cap.width) <= FULL_KEYBOARD_WIDTH, "row {row}");
    }
}

/// A report in the device log: up to its zero padding on one line, every byte 16 to a row.
#[test]
fn packets() {
    let mut report = vec![0x03, 0xFB, 0xFB, 0xFB];
    report.resize(65, 0);
    assert_eq!(packet_text::summary(&report), "03 FB FB FB");
    assert_eq!(packet_text::summary_count(&report), 4);
    let rows = packet_text::rows(&report);
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0], "0000  03 FB FB FB 00 00 00 00 00 00 00 00 00 00 00 00");
    assert_eq!(rows[4], "0040  00");
    assert_eq!(packet_text::summary(&[0, 0]), "00");
    assert!(packet_text::rows(&[]).is_empty());
}

/// Every media key and mouse action the domain knows has a name on screen.
#[test]
fn choices_cover_the_domain() {
    assert!(MediaKey::ALL.iter().all(|key| MediaChoice::of(*key).is_some()));
    assert_eq!(MediaChoice::ALL.len(), MediaKey::ALL.len());
    assert_eq!(domain::MouseAction::ALL.map(MouseChoice::of).len(), MouseChoice::ALL.len());
    assert_eq!(ActionCaption::of(Some(&domain::KeyAction::Media(MediaKey(0x999)))), ActionCaption::OtherMedia(0x999));
    assert_eq!(ActionCaption::of(None), ActionCaption::Unknown);
    assert_eq!(Rgb::new(0, 255, 0).hex(), "#00FF00");
    assert_eq!(Rgb::PALETTE.len(), 56);
}

/// The information popover, from what a 1189:8890 reports.
#[test]
fn keyboard_info() {
    let interface = |number, class, subclass, protocol, usages: Vec<UsagePair>| DeviceInterface {
        number,
        class,
        subclass,
        protocol,
        usages,
    };
    let keyboard = domain::ConnectedKeyboard {
        usb: domain::KeyboardFamily::K8890_USB,
        family: domain::KeyboardFamily::K8890,
        interface_number: 1,
        channel: domain::Channel::Usb { out_endpoint: 0x02 },
        report_id: 3,
        report_length: 65,
        details: Some(DeviceDetails {
            usb_version: 0x0110,
            device_release: 0,
            speed: Some(12_000_000),
            manufacturer: None,
            product_name: None,
            serial_number: None,
            max_power: Some(100),
            interfaces: vec![
                interface(0, 3, 1, 1, vec![UsagePair { page: 1, usage: 6 }]),
                interface(1, 3, 0, 0, vec![]),
                interface(2, 3, 0, 0, vec![UsagePair { page: 1, usage: 6 }, UsagePair { page: 0x0C, usage: 1 }]),
                interface(3, 3, 1, 2, vec![]),
            ],
        }),
    };
    let identification =
        domain::KeyboardIdentification { commands: domain::CommandSet::Packets { report_id: 3 }, identity: None };
    let info = KeyboardInfo::of(&keyboard, Some(&identification));
    assert_eq!(info.usb_id, "1189:8890");
    assert_eq!(info.firmware, Firmware::Unknown);
    assert_eq!(info.usb.as_deref(), Some("USB 1.1 · 12 Mb/s"));
    assert_eq!(
        info.interfaces.iter().map(|interface| interface.roles.clone()).collect::<Vec<_>>(),
        vec![
            vec![InterfaceRole::Keyboard],
            vec![InterfaceRole::Configuration],
            vec![InterfaceRole::Keyboard, InterfaceRole::Media],
            vec![InterfaceRole::Mouse]
        ]
    );
    assert_eq!(
        info.interfaces.iter().map(|interface| interface.is_configuration).collect::<Vec<_>>(),
        vec![false, true, false, false]
    );
    assert_eq!(
        info.channel,
        InfoChannel { interface: 1, path: ChannelPath::Usb { out_endpoint: 0x02 }, report_id: 3, report_length: 65 }
    );
    assert_eq!(info.commands, InfoCommands::Packets { report_id: 3 });
}

/// Every keyboard the app looks for, in sysfs's lower case: the 1189:8890's USB device, then one hidraw line per
/// vendor with its products as alternatives — and nothing else.
#[test]
fn udev_rules_cover_every_keyboard() {
    let rules = crate::access::udev_rules();
    let lines: Vec<&str> = rules.lines().filter(|line| !line.starts_with('#')).collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], r#"SUBSYSTEM=="usb", ATTR{idVendor}=="1189", ATTR{idProduct}=="8890", TAG+="uaccess""#);
    let allowed = |line: &str| -> Vec<(String, String)> {
        let vendor = line.split(r#"idVendor}==""#).nth(1).and_then(|rest| rest.split('"').next()).unwrap_or_default();
        let products =
            line.split(r#"idProduct}==""#).nth(1).and_then(|rest| rest.split('"').next()).unwrap_or_default();
        products.split('|').map(|product| (vendor.to_owned(), product.to_owned())).collect()
    };
    let mut granted: Vec<(String, String)> = lines.iter().flat_map(|line| allowed(line)).collect();
    let mut wanted: Vec<(String, String)> = domain::KeyboardFamily::SUPPORTED
        .iter()
        .map(|usb| (format!("{:04x}", usb.vendor_id), format!("{:04x}", usb.product_id)))
        .collect();
    granted.sort();
    wanted.sort();
    assert_eq!(granted, wanted);
    assert!(lines[1..].iter().all(|line| line.starts_with(r#"KERNEL=="hidraw*", "#)));
    assert!(lines.iter().all(|line| line.ends_with(r#"TAG+="uaccess""#)));
}

/// The commands are short enough to copy without fuss: `tee`, the rule's four lines, `EOF`, and one line to apply it.
#[test]
fn the_udev_commands_are_short() {
    let commands = crate::access::udev_install_commands();
    assert_eq!(commands.lines().count(), 7);
    assert!(commands.ends_with("sudo udevadm control --reload-rules && sudo udevadm trigger\n"));
}

/// The rule file the repository ships is the one the app shows.
#[test]
fn the_shipped_udev_rule_is_the_apps() {
    assert_eq!(include_str!("../../../data/60-minikeyboard.rules"), crate::access::udev_rules());
}
