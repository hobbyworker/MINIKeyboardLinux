//! The device repository against simulated firmware: the same codecs as with a keyboard on USB.

use std::rc::Rc;
use std::time::Duration;

use futures_executor::block_on;

use datalayer::{ChannelProvider, DeviceLogStore, KeyboardDevice, ReportChannel, SimulatedKeyboard};
use domain::ports::{DeviceLogRepository, KeyboardDeviceRepository};
use domain::*;

fn model(layout: &str, format: RecordFormat) -> SimulatedModel {
    SimulatedModel::identified_as(KeyboardLayout::named(layout).unwrap(), format).unwrap()
}

fn device(keyboard: &Rc<SimulatedKeyboard>, log: &DeviceLogStore) -> KeyboardDevice {
    KeyboardDevice::with_timing(
        Rc::new(keyboard.clone()),
        log.clone(),
        Duration::from_millis(20),
        Duration::from_millis(50),
        Rc::new(|_| Box::pin(async {})),
    )
}

#[test]
fn a_standard_keyboard_reads_back_what_was_written() {
    let log = DeviceLogStore::new();
    let keyboard = Rc::new(SimulatedKeyboard::new(model("6add1", RecordFormat::Standard)));
    let device = device(&keyboard, &log);
    assert_eq!(block_on(device.ask_identity()), Ok(Some(KeyboardIdentity::new(6, 1, 10))));

    let layout = KeyboardLayout::named("6add1").unwrap();
    let fresh = block_on(device.read_actions(0, RecordFormat::Standard, layout)).unwrap();
    assert_eq!(fresh.len(), layout.controls.len());
    assert!(fresh.values().all(|action| *action == KeyAction::Unassigned));

    let mut configuration = KeyboardConfiguration::new(3);
    let presses = KeyAction::Keys(KeyMacro::new(vec![
        KeyStroke::new(Modifiers::SUPER, 0x06).with_delay(100),
        KeyStroke::key(0x28).with_delay(60),
    ]));
    configuration.set(presses.clone(), ControlId::Key(3), 1);
    configuration.set(KeyAction::Mouse(MouseAction::MiddleClick), ControlId::Knob(1, KnobPart::Press), 1);
    let colors: Vec<KeyColor> = (0..16).map(|red| KeyColor::new(red, 0, 0)).collect();
    configuration.set_lighting(LayerLighting::PerKey { mode: 5, colors: colors.clone() }, 1);
    block_on(device.write(&configuration, CommandSet::Records(RecordFormat::Standard), layout)).unwrap();

    let read = block_on(device.read_actions(1, RecordFormat::Standard, layout)).unwrap();
    assert_eq!(read[&ControlId::Key(3)], presses);
    assert_eq!(read[&ControlId::Knob(1, KnobPart::Press)], KeyAction::Mouse(MouseAction::MiddleClick));
    assert_eq!(read[&ControlId::Key(1)], KeyAction::Unassigned);
    assert_eq!(block_on(device.read_lighting(1)), Ok(LayerLighting::PerKey { mode: 5, colors }));
    assert_eq!(
        block_on(device.read_lighting(0)),
        Ok(LayerLighting::PerKey { mode: 0, colors: KeyColor::DEFAULT_KEY_COLORS.to_vec() })
    );
}

#[test]
fn a_legacy_keyboard_keeps_its_wait() {
    let log = DeviceLogStore::new();
    let keyboard = Rc::new(SimulatedKeyboard::new(model("4add1", RecordFormat::Legacy)));
    let device = device(&keyboard, &log);
    assert_eq!(block_on(device.ask_identity()), Ok(Some(KeyboardIdentity::new(4, 1, 0))));
    let layout = KeyboardLayout::named("4add1").unwrap();
    let mut configuration = KeyboardConfiguration::new(3);
    let presses = KeyAction::Keys(KeyMacro::with_delay(vec![KeyStroke::new(Modifiers::SHIFT, 0x04)], 1500));
    configuration.set(presses.clone(), ControlId::Key(4), 2);
    configuration.set_lighting(LayerLighting::ColorMode { color: LightColor::Purple, mode: 5 }, 2);
    block_on(device.write(&configuration, CommandSet::Records(RecordFormat::Legacy), layout)).unwrap();
    let read = block_on(device.read_actions(2, RecordFormat::Legacy, layout)).unwrap();
    assert_eq!(read[&ControlId::Key(4)], presses);
    assert_eq!(keyboard.record(3, 0).unwrap()[11], 0x75);
}

#[test]
fn an_unidentified_keyboard_answers_nothing() {
    let log = DeviceLogStore::new();
    let model = SimulatedModel { usb: UsbIdentity::new(0x1189, 0x8840), kind: SimulatedKind::Unidentified };
    let keyboard = Rc::new(SimulatedKeyboard::new(model));
    let device = device(&keyboard, &log);
    assert_eq!(block_on(device.ask_identity()), Ok(None));
    let mut configuration = KeyboardConfiguration::new(3);
    configuration.set(KeyAction::Media(MediaKey::MUTE), ControlId::Key(1), 0);
    let layout = KeyboardLayout::named("6add1").unwrap();
    block_on(device.write(&configuration, CommandSet::Records(RecordFormat::Unidentified), layout)).unwrap();
    assert_eq!(keyboard.record(1, 1).unwrap()[0], 0xFE);
}

/// The 8890's packets go out after its report ID, padded to its report's length.
#[test]
fn the_8890() {
    let log = DeviceLogStore::new();
    let model = SimulatedModel { usb: KeyboardFamily::K8890_USB, kind: SimulatedKind::K8890 { report_id: 3 } };
    let keyboard = Rc::new(SimulatedKeyboard::new(model));
    let device = device(&keyboard, &log);
    let mut configuration = KeyboardConfiguration::new(3);
    configuration.set(KeyAction::Keys(KeyMacro::new(vec![KeyStroke::key(0x04)])), ControlId::Key(1), 0);
    block_on(device.write(
        &configuration,
        CommandSet::Packets { report_id: 3 },
        KeyboardLayout::named("6add1").unwrap(),
    ))
    .unwrap();
    let written = keyboard.written();
    assert_eq!(written.len(), 4);
    assert!(written.iter().all(|payload| payload.len() == 64));
    assert_eq!(written[2][..8], [1, 0x11, 1, 1, 0, 0x04, 0, 0]);

    let mut entries = log.entries();
    let entries = block_on(entries.next()).unwrap();
    let reports: Vec<&Vec<u8>> = entries
        .iter()
        .filter_map(|entry| match &entry.content {
            LogContent::Output(report) => Some(report),
            _ => None,
        })
        .collect();
    assert_eq!(reports.len(), 4);
    let mut first = vec![0x03, 0xA1, 0x01];
    first.resize(65, 0);
    assert_eq!(*reports[0], first);
}

#[test]
fn nothing_open() {
    struct Nothing;
    impl ChannelProvider for Nothing {
        fn open_channel(&self) -> Option<Rc<dyn ReportChannel>> {
            None
        }
    }
    let device = KeyboardDevice::new(Rc::new(Nothing), DeviceLogStore::new());
    assert_eq!(block_on(device.ask_identity()), Err(KeyboardDeviceError::NotConnected));
}

#[test]
fn the_log_keeps_the_newest_lines() {
    let log = DeviceLogStore::new();
    for index in 0..2100u32 {
        log.append(LogContent::Output(index.to_be_bytes().to_vec()));
    }
    let lines = log.snapshot();
    assert_eq!(lines.len(), 2000);
    assert_eq!(lines[0].id, 101);
    log.clear();
    assert_eq!(block_on(log.entries().next()), Some(vec![]));
}
