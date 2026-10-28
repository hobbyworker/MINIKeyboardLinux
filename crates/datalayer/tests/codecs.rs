use std::time::Duration;

use datalayer::ReportFraming;
use datalayer::protocols::plan::WritePlan;
use datalayer::protocols::{k884x, k8890, numbers, records};
use domain::*;

fn keys(strokes: Vec<KeyStroke>) -> KeyAction {
    KeyAction::Keys(KeyMacro::new(strokes))
}

// The 1189:8890's packets, as the Windows tool sends them.

#[test]
fn a_key_is_the_layer_its_groups_and_the_save() {
    let action = keys(vec![KeyStroke::new(Modifiers::SUPER, 0x06), KeyStroke::key(0x28)]);
    assert_eq!(
        k8890::download(3, &action, 1, 3).unwrap(),
        vec![
            vec![0xA1, 0x02, 0, 0, 0, 0, 0, 0],
            vec![0x03, 0x21, 0x02, 0x00, 0x08, 0x00, 0, 0],
            vec![0x03, 0x21, 0x02, 0x01, 0x08, 0x06, 0, 0],
            vec![0x03, 0x21, 0x02, 0x02, 0x00, 0x28, 0, 0],
            vec![0xAA, 0xAA, 0, 0, 0, 0, 0, 0],
        ]
    );
}

/// Report ID 0 takes no layer switch and no layer in the type byte.
#[test]
fn report_id_zero_has_no_layers() {
    let packets = k8890::download(1, &keys(vec![KeyStroke::key(0x04)]), 2, 0).unwrap();
    assert_eq!(packets[0], vec![0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0, 0]);
    assert_eq!(packets.len(), 3);
}

#[test]
fn modifiers_alone_and_cleared_keys() {
    let alone = keys(vec![KeyStroke::new(Modifiers::CONTROL | Modifiers::SHIFT, 0)]);
    assert_eq!(k8890::key_packets(14, &alone, 0, 3).unwrap(), vec![vec![14, 0x11, 0, 0, 0x03, 0, 0, 0]]);
    assert_eq!(k8890::key_packets(14, &KeyAction::Unassigned, 0, 3).unwrap(), vec![vec![14, 0x11, 0, 0, 0, 0, 0, 0]]);
}

#[test]
fn media_keys_per_report_id() {
    let media = |key| KeyAction::Media(key);
    assert_eq!(
        k8890::key_packets(2, &media(MediaKey::PLAY_PAUSE), 0, 3).unwrap(),
        vec![vec![2, 0x12, 0xCD, 0x00, 0, 0, 0, 0]]
    );
    assert_eq!(
        k8890::key_packets(2, &media(MediaKey::PLAY_PAUSE), 0, 0).unwrap(),
        vec![vec![2, 0x02, 0x40, 0x00, 0, 0, 0, 0]]
    );
    assert_eq!(
        k8890::key_packets(2, &media(MediaKey::VOLUME_DOWN), 0, 2).unwrap(),
        vec![vec![2, 0x12, 0x80, 0x00, 0, 0, 0, 0]]
    );
    assert_eq!(k8890::key_packets(2, &media(MediaKey::CALCULATOR), 0, 2), Err(KeyboardDeviceError::Unsupported));
}

#[test]
fn mouse() {
    let mouse = |action| KeyAction::Mouse(action);
    assert_eq!(
        k8890::key_packets(4, &mouse(MouseAction::ShiftWheelDown), 0, 3).unwrap(),
        vec![vec![4, 0x13, 0x00, 0, 0, 0xFF, 0x02, 0]]
    );
    assert_eq!(
        k8890::key_packets(4, &mouse(MouseAction::RightClick), 0, 3).unwrap(),
        vec![vec![4, 0x13, 0x02, 0, 0, 0x00, 0x00, 0]]
    );
}

/// With the setting off, just what the vendor's tool sends.
#[test]
fn k8890_lighting_as_the_vendors_tool() {
    assert_eq!(
        k8890::download_lighting(2, 0, 3, false),
        vec![
            vec![0xA1, 0x01, 0, 0, 0, 0, 0, 0],
            vec![0xB0, 0x18, 0x02, 0, 0, 0, 0, 0],
            vec![0xAA, 0xA1, 0, 0, 0, 0, 0, 0]
        ]
    );
}

/// The lights, their save, and a save with no key in it (it ends the key repeat the lights' save starts).
#[test]
fn k8890_lighting() {
    assert_eq!(
        k8890::download_lighting(2, 0, 3, true),
        vec![
            vec![0xA1, 0x01, 0, 0, 0, 0, 0, 0],
            vec![0xB0, 0x18, 0x02, 0, 0, 0, 0, 0],
            vec![0xAA, 0xA1, 0, 0, 0, 0, 0, 0],
            vec![0xA1, 0x01, 0, 0, 0, 0, 0, 0],
            vec![0xAA, 0xAA, 0, 0, 0, 0, 0, 0]
        ]
    );
}

// The 884x records, as the cross-platform app writes and reads them.

#[test]
fn standard_entries_take_a_modifier_each() {
    let action = KeyAction::Keys(KeyMacro::new(vec![
        KeyStroke::new(Modifiers::SHIFT | Modifiers::SUPER, 0x1D).with_delay(300),
        KeyStroke::key(0x28).with_delay(50),
    ]));
    let record = records::standard(&action, 4, 2).unwrap();
    assert_eq!(record.len(), 64);
    assert_eq!(record[0..6], [0xFD, 4, 3, 1, 0, 4]);
    assert_eq!(record[6..18], [0x01, 0x2C, 0xF2, 0x00, 0x00, 0xF4, 0x00, 0x00, 0x1D, 0x00, 0x32, 0x28]);
    assert_eq!(records::decode(&record, RecordFormat::Standard), Some(action));
}

#[test]
fn standard_media_and_mouse() {
    let media = records::standard(&KeyAction::Media(MediaKey::WEB_BACK), 1, 0).unwrap();
    assert!(media[3] == 2 && media[5] == 2 && media[8] == 0x24 && media[11] == 0x02);
    assert_eq!(records::decode(&media, RecordFormat::Standard), Some(KeyAction::Media(MediaKey::WEB_BACK)));
    let mouse = records::standard(&KeyAction::Mouse(MouseAction::ControlWheelUp), 1, 0).unwrap();
    assert!(mouse[3] == 3 && mouse[4] == 1 && mouse[5] == 4 && mouse[8] == 0xF1 && mouse[11] == 0 && mouse[20] == 0x01);
    assert_eq!(records::decode(&mouse, RecordFormat::Standard), Some(KeyAction::Mouse(MouseAction::ControlWheelUp)));
    let mut gesture = mouse.clone();
    gesture[4] = 4;
    assert_eq!(records::decode(&gesture, RecordFormat::Standard), None);
}

#[test]
fn legacy_pairs_and_the_wait() {
    let action = KeyAction::Keys(KeyMacro::with_delay(
        vec![KeyStroke::new(Modifiers::CONTROL, 0x06), KeyStroke::key(0x19)],
        500,
    ));
    let record = records::encode(&action, 16, 0, RecordFormat::Legacy).unwrap();
    assert_eq!(record.len(), 50);
    assert_eq!(record[0..14], [0xFD, 16, 1, 5, 0xF4, 0x01, 0, 0, 0, 2, 0x01, 0x06, 0x00, 0x19]);
    assert_eq!(records::decode(&record, RecordFormat::Legacy), Some(action));
    let plain = records::encode(&keys(vec![KeyStroke::key(0x04)]), 1, 0, RecordFormat::Legacy).unwrap();
    assert_eq!(plain[3], 1);
}

#[test]
fn legacy_media_and_mouse() {
    let media = records::encode(&KeyAction::Media(MediaKey::MUTE), 1, 0, RecordFormat::Legacy).unwrap();
    assert!(media[3] == 2 && media[9] == 2 && media[10] == 0xE2 && media[11] == 0);
    assert_eq!(records::decode(&media, RecordFormat::Legacy), Some(KeyAction::Media(MediaKey::MUTE)));
    let wheel = records::encode(&KeyAction::Mouse(MouseAction::AltWheelDown), 1, 0, RecordFormat::Legacy).unwrap();
    assert!(wheel[3] == 3 && wheel[9] == 1 && wheel[10] == 0x04 && wheel[11] == 0 && wheel[14] == 0xFF);
    assert_eq!(records::decode(&wheel, RecordFormat::Legacy), Some(KeyAction::Mouse(MouseAction::AltWheelDown)));
}

#[test]
fn modifiers_alone_sit_after_the_last_pair() {
    let alone = keys(vec![KeyStroke::new(Modifiers::ALT, 0)]);
    let record = records::encode(&alone, 1, 0, RecordFormat::Legacy).unwrap();
    assert!(record[9] == 0 && record[10] == 0x04 && record[11] == 0);
    assert_eq!(records::decode(&record, RecordFormat::Legacy), Some(alone.clone()));
    let standard = records::standard(&alone, 1, 0).unwrap();
    assert!(standard[5] == 1 && standard[8] == 0xF3);
    assert_eq!(records::decode(&standard, RecordFormat::Standard), Some(alone));
}

/// The form keyboards of the 1189:8840 kind take: as the legacy pairs, marked `FE`, a media key with a count
/// of 0 and a wheel move with 3.
#[test]
fn unidentified_records() {
    let key = records::unidentified(&keys(vec![KeyStroke::new(Modifiers::SUPER, 0x06)]), 17, 0).unwrap();
    assert_eq!(key[0..12], [0xFE, 17, 1, 1, 0, 0, 0, 0, 0, 1, 0x08, 0x06]);
    let media = records::unidentified(&KeyAction::Media(MediaKey::VOLUME_UP), 1, 1).unwrap();
    assert_eq!(media[0..12], [0xFE, 1, 2, 2, 0, 0, 0, 0, 0, 0, 0xE9, 0x00]);
    let wheel = records::unidentified(&KeyAction::Mouse(MouseAction::WheelUp), 1, 0).unwrap();
    assert_eq!(wheel[0..15], [0xFE, 1, 1, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0x01]);
    let click = records::unidentified(&KeyAction::Mouse(MouseAction::LeftClick), 1, 0).unwrap();
    assert_eq!(click[0..12], [0xFE, 1, 1, 3, 0, 0, 0, 0, 0, 1, 0, 0x01]);
}

#[test]
fn cleared() {
    let record = records::standard(&KeyAction::Unassigned, 2, 0).unwrap();
    assert_eq!(record[0..6], [0xFD, 2, 1, 1, 0, 0]);
    assert_eq!(records::decode(&record, RecordFormat::Standard), Some(KeyAction::Unassigned));
    assert_eq!(records::decode(&[0xFD, 2, 1, 0], RecordFormat::Legacy), Some(KeyAction::Unassigned));
    assert_eq!(records::decode(&[0xFD, 2, 1, 9], RecordFormat::Legacy), None);
}

/// What a record holds of an action, to compare a read after a save with: each kind as written on both formats that
/// read back — but a legacy record has no wait per press, and the `FE` form reads nothing back.
#[test]
fn stored_forms() {
    let presses = |delay| {
        keys(vec![
            KeyStroke::new(Modifiers::CONTROL | Modifiers::RIGHT_SHIFT, 0x06).with_delay(delay),
            KeyStroke::key(0x28),
        ])
    };
    let every_format = [
        KeyAction::Unassigned,
        keys(vec![KeyStroke::new(Modifiers::ALT, 0)]),
        KeyAction::Media(MediaKey::MY_COMPUTER),
        KeyAction::Mouse(MouseAction::RightClick),
        KeyAction::Mouse(MouseAction::ShiftWheelDown),
    ];
    for action in every_format.iter().chain([&presses(250)]) {
        assert_eq!(records::stored_form(action, RecordFormat::Standard).as_ref(), Some(action), "{action:?}");
    }
    let waiting =
        KeyAction::Keys(KeyMacro::with_delay(vec![KeyStroke::new(Modifiers::SHIFT, 0x04), KeyStroke::key(0x05)], 1500));
    for action in every_format.iter().chain([&presses(0), &waiting]) {
        assert_eq!(records::stored_form(action, RecordFormat::Legacy).as_ref(), Some(action), "{action:?}");
    }
    assert_eq!(records::stored_form(&presses(250), RecordFormat::Legacy), Some(presses(0)));
    assert_eq!(records::stored_form(&KeyAction::Media(MediaKey::MUTE), RecordFormat::Unidentified), None);
}

#[test]
fn legacy_lighting() {
    let record = records::legacy_lighting(LightColor::Blue, 3, 2, records::LEGACY_LENGTH);
    assert_eq!(record[0..12], [0xFE, 0xB0, 3, 8, 0, 0, 0, 0, 0, 1, 0, 0x63]);
}

#[test]
fn answers() {
    assert_eq!(k884x::identity(&[0x03, 0xFB, 6, 1, 10]), Some(KeyboardIdentity::new(6, 1, 10)));
    assert_eq!(k884x::identity(&[0x03, 0xFB]), None);
    let reply: Vec<u8> = [0x03, 0xB0, 4].into_iter().chain(std::iter::repeat_n(9, 48)).collect();
    assert_eq!(
        k884x::lighting_reply(&reply),
        Some(LayerLighting::PerKey { mode: 4, colors: vec![KeyColor::new(9, 9, 9); 16] })
    );
    // `FA 19 00` and `FA 0F 03`, as the cross-platform app asks: 25 records, and 15 keys then 3 knobs of 3.
    assert_eq!(k884x::read_records(0, RecordFormat::Standard), Some(vec![0xFA, 0x19, 0x00, 1]));
    assert_eq!(k884x::read_reply_count(RecordFormat::Standard), 25);
    assert_eq!(k884x::read_records(2, RecordFormat::Legacy), Some(vec![0xFA, 0x0F, 0x03, 3]));
    assert_eq!(k884x::read_reply_count(RecordFormat::Legacy), 24);
    assert_eq!(k884x::read_records(0, RecordFormat::Unidentified), None);
}

#[test]
fn framing() {
    assert_eq!(ReportFraming { report_id: 3, length: 9 }.frame(&[0xAA, 0xAA]), vec![3, 0xAA, 0xAA, 0, 0, 0, 0, 0, 0]);
    assert_eq!(ReportFraming { report_id: 0, length: 8 }.frame(&[0xAA]), vec![0xAA, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(ReportFraming { report_id: 3, length: 65 }.frame(&k884x::IDENTIFY).len(), 65);
}

// The order a save writes in.

fn six_and_one() -> &'static KeyboardLayout {
    KeyboardLayout::named("6add1").unwrap()
}

fn configuration(edit: impl FnOnce(&mut KeyboardConfiguration)) -> KeyboardConfiguration {
    let mut configuration = KeyboardConfiguration::new(3);
    edit(&mut configuration);
    configuration
}

fn prefixes(plan: &WritePlan) -> Vec<Vec<u8>> {
    plan.steps.iter().map(|step| step.payload[..3].to_vec()).collect()
}

/// Every changed record, then all three layers' lights, then one commit (`Widget::HID_write`).
#[test]
fn standard_plan() {
    let configuration = configuration(|configuration| {
        configuration.set(KeyAction::Media(MediaKey::MUTE), ControlId::Key(2), 1);
        configuration.set(KeyAction::Unassigned, ControlId::Knob(1, KnobPart::Press), 0);
        for layer in 0..3 {
            configuration.apply_read_lighting(
                LayerLighting::PerKey { mode: 0, colors: KeyColor::DEFAULT_KEY_COLORS.to_vec() },
                layer,
            );
        }
        configuration.set_lighting(LayerLighting::PerKey { mode: 2, colors: KeyColor::DEFAULT_KEY_COLORS.to_vec() }, 2);
    });
    let plan = WritePlan::new(&configuration, CommandSet::Records(RecordFormat::Standard), six_and_one()).unwrap();
    assert_eq!(
        prefixes(&plan),
        vec![
            vec![0xFD, 17, 1],
            vec![0xFD, 2, 2],
            vec![0xFE, 0xB0, 0],
            vec![0xFE, 0xB0, 1],
            vec![0xFE, 0xB0, 2],
            vec![0xFD, 0xFE, 0xFF]
        ]
    );
    assert_eq!(plan.steps.last().unwrap().pause_after, Duration::from_millis(200));
    assert_eq!(plan.steps[4].payload[3], 2);
}

/// Lights alone are written without a commit, as the app does.
#[test]
fn standard_lights_alone() {
    let configuration = configuration(|configuration| {
        configuration.set_lighting(LayerLighting::PerKey { mode: 1, colors: KeyColor::DEFAULT_KEY_COLORS.to_vec() }, 0);
    });
    let plan = WritePlan::new(&configuration, CommandSet::Records(RecordFormat::Standard), six_and_one()).unwrap();
    assert_eq!(prefixes(&plan), vec![vec![0xFE, 0xB0, 0]]);
}

/// Each layer's records and lights, then that layer's commit.
#[test]
fn legacy_plan() {
    let configuration = configuration(|configuration| {
        configuration.set(KeyAction::Media(MediaKey::MUTE), ControlId::Key(1), 0);
        configuration.set_lighting(LayerLighting::ColorMode { color: LightColor::Green, mode: 1 }, 0);
        configuration.set(KeyAction::Media(MediaKey::MUTE), ControlId::Key(6), 2);
    });
    let plan = WritePlan::new(&configuration, CommandSet::Records(RecordFormat::Legacy), six_and_one()).unwrap();
    assert_eq!(
        prefixes(&plan),
        vec![vec![0xFD, 1, 1], vec![0xFE, 0xB0, 1], vec![0xFD, 0xFE, 0xFF], vec![0xFD, 6, 3], vec![0xFD, 0xFE, 0xFF]]
    );
}

#[test]
fn unidentified_plan() {
    let configuration = configuration(|configuration| {
        configuration.set(KeyAction::Media(MediaKey::MUTE), ControlId::Knob(1, KnobPart::RotateRight), 0);
        configuration.set_lighting(LayerLighting::ColorMode { color: LightColor::Red, mode: 2 }, 1);
    });
    let plan = WritePlan::new(&configuration, CommandSet::Records(RecordFormat::Unidentified), six_and_one()).unwrap();
    let payloads: Vec<Vec<u8>> = plan.steps.into_iter().map(|step| step.payload).collect();
    assert_eq!(
        payloads,
        vec![
            records::unidentified(&KeyAction::Media(MediaKey::MUTE), 18, 0).unwrap(),
            vec![0xAA, 0xAA],
            vec![0xFD, 0xFE, 0xFF],
            vec![0xAA, 0xAA],
            vec![0xFB, 0xFB, 0xFB, 0x01],
            vec![0xFA, 0x0F, 0x03, 1, 0x7F],
            vec![0xFA, 0x0F, 0x03, 2, 0x7F],
            vec![0xFA, 0x0F, 0x03, 3, 0x7F],
            vec![0xFE, 0xB0, 2, 8, 0, 0, 0, 0, 0, 1, 0, 0x12],
            vec![0xFD, 0xFE, 0xFF],
        ]
    );
}

/// One key at a time on the 8890, knobs numbered from 13.
#[test]
fn k8890_plan() {
    let configuration = configuration(|configuration| {
        configuration.set(keys(vec![KeyStroke::key(0x04)]), ControlId::Knob(1, KnobPart::RotateRight), 0);
        configuration.set_lighting(LayerLighting::Mode(1), 0);
    });
    let plan = WritePlan::new(&configuration, CommandSet::Packets { report_id: 3 }, six_and_one()).unwrap();
    let payloads: Vec<Vec<u8>> = plan.steps.into_iter().map(|step| step.payload).collect();
    assert_eq!(
        payloads,
        vec![
            vec![0xA1, 1, 0, 0, 0, 0, 0, 0],
            vec![0xB0, 0x18, 1, 0, 0, 0, 0, 0],
            vec![0xAA, 0xA1, 0, 0, 0, 0, 0, 0],
            vec![0xA1, 1, 0, 0, 0, 0, 0, 0],
            vec![0xAA, 0xAA, 0, 0, 0, 0, 0, 0],
            vec![0xA1, 1, 0, 0, 0, 0, 0, 0],
            vec![15, 0x11, 1, 0, 0, 0, 0, 0],
            vec![15, 0x11, 1, 1, 0, 0x04, 0, 0],
            vec![0xAA, 0xAA, 0, 0, 0, 0, 0, 0],
        ]
    );
}

/// What the app doesn't know is never written.
#[test]
fn only_changes() {
    let configuration = configuration(|configuration| {
        configuration.apply_read([(ControlId::Key(1), KeyAction::Media(MediaKey::MUTE))].into(), 0);
    });
    assert!(
        WritePlan::new(&configuration, CommandSet::Records(RecordFormat::Standard), six_and_one())
            .unwrap()
            .steps
            .is_empty()
    );
}

// Where each control is stored: the 8890's numbers, and the records of the cross-platform app's layouts.

#[test]
fn k8890_numbers() {
    assert_eq!(numbers::k8890_number(ControlId::Key(1)), Some(1));
    assert_eq!(numbers::k8890_number(ControlId::Key(12)), Some(12));
    assert_eq!(numbers::k8890_number(ControlId::Key(13)), None);
    assert_eq!(numbers::k8890_number(ControlId::Knob(1, KnobPart::RotateLeft)), Some(13));
    assert_eq!(numbers::k8890_number(ControlId::Knob(1, KnobPart::Press)), Some(14));
    assert_eq!(numbers::k8890_number(ControlId::Knob(2, KnobPart::RotateRight)), Some(18));
    assert_eq!(numbers::k8890_number(ControlId::Knob(3, KnobPart::Press)), None);
    assert_eq!(numbers::k8890_number(ControlId::Knob(1, KnobPart::PressRotateLeft)), None);
}

/// Whatever the domain lets the 8890 take has a number.
#[test]
fn every_addressable_control_has_a_number() {
    let capabilities = KeyboardCapabilities::new(CommandSet::Packets { report_id: 3 });
    let controls = (1..=16)
        .map(ControlId::Key)
        .chain((1..=3).flat_map(|knob| KnobPart::ALL.map(|part| ControlId::Knob(knob, part))));
    for control in controls {
        assert_eq!(capabilities.can_address(control), numbers::k8890_number(control).is_some(), "{control:?}");
    }
}

/// Every control of every layout has a record of its own, from 1 to 25.
#[test]
fn every_control_has_a_record() {
    for layout in KeyboardLayout::all() {
        let records: Vec<u8> = layout.controls.iter().filter_map(|control| numbers::record(*control, layout)).collect();
        assert_eq!(records.len(), layout.controls.len(), "{}", layout.id);
        assert_eq!(records.iter().collect::<std::collections::HashSet<_>>().len(), records.len(), "{}", layout.id);
        assert!(records.iter().all(|record| (1..=25).contains(record)), "{}", layout.id);
        for control in &layout.controls {
            assert_eq!(numbers::control_in_record(numbers::record(*control, layout).unwrap(), layout), Some(*control));
        }
    }
    let dial = KeyboardLayout::named("0add1").unwrap();
    assert_eq!(numbers::record(ControlId::Knob(1, KnobPart::PressRotateLeft), dial), Some(19));
    assert_eq!(numbers::record(ControlId::Knob(1, KnobPart::PressRotateRight), dial), Some(21));
    assert_eq!(
        numbers::record(ControlId::Knob(4, KnobPart::Press), KeyboardLayout::named("12add4").unwrap()),
        Some(14)
    );
    assert_eq!(numbers::record(ControlId::Key(7), six_and_one()), None);
}
