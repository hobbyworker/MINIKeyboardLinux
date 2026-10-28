//! The device repository against simulated firmware: the same codecs as with a keyboard on USB.

use std::cell::Cell;
use std::collections::BTreeSet;
use std::rc::Rc;
use std::time::Duration;

use futures_executor::block_on;

use datalayer::{ChannelProvider, DeviceLogStore, KeyboardDevice, ReportChannel, ReportFraming, SimulatedKeyboard};
use domain::ports::{DeviceLogRepository, KeyboardDeviceRepository, LocalFuture};
use domain::usecases::{SaveCheck, SaveKeyboardUseCase, SaveOutcome};
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
    assert_eq!(fresh.actions.len(), layout.controls.len());
    assert!(fresh.actions.values().all(|action| *action == KeyAction::Unassigned));
    assert!(fresh.unread.is_empty());

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

    let read = block_on(device.read_actions(1, RecordFormat::Standard, layout)).unwrap().actions;
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
    let read = block_on(device.read_actions(2, RecordFormat::Legacy, layout)).unwrap().actions;
    assert_eq!(read[&ControlId::Key(4)], presses);
    assert_eq!(keyboard.record(3, 0).unwrap()[11], 0x75);
}

/// A simulated keyboard whose answers stop after `left` more.
struct Stopping {
    keyboard: Rc<SimulatedKeyboard>,
    left: Cell<usize>,
}

impl ReportChannel for Stopping {
    fn framing(&self) -> ReportFraming {
        self.keyboard.framing()
    }

    fn write(&self, report: Vec<u8>) -> LocalFuture<'_, Result<(), KeyboardDeviceError>> {
        self.keyboard.write(report)
    }

    fn discard_pending_replies(&self) {
        self.keyboard.discard_pending_replies();
    }

    fn next_reply(&self, timeout: Duration) -> LocalFuture<'_, Option<Vec<u8>>> {
        if self.left.get() == 0 {
            return Box::pin(async { None });
        }
        self.left.set(self.left.get() - 1);
        self.keyboard.next_reply(timeout)
    }
}

struct StoppingChannels(Rc<Stopping>);

impl ChannelProvider for StoppingChannels {
    fn open_channel(&self) -> Option<Rc<dyn ReportChannel>> {
        Some(self.0.clone())
    }
}

/// Answers that stop part-way leave the rest of the layout unread; a keyboard whose last control comes before the
/// format's count leaves nothing unread; no answer at all is no answer.
#[test]
fn a_read_that_stops_keeps_what_came() {
    let log = DeviceLogStore::new();
    let keyboard = Rc::new(SimulatedKeyboard::new(model("12add2", RecordFormat::Standard)));
    let layout = KeyboardLayout::named("12add2").unwrap();
    let mut configuration = KeyboardConfiguration::new(3);
    configuration.set(KeyAction::Media(MediaKey::MUTE), ControlId::Key(1), 0);
    block_on(device(&keyboard, &log).write(&configuration, CommandSet::Records(RecordFormat::Standard), layout))
        .unwrap();

    let stopping = Rc::new(Stopping { keyboard, left: Cell::new(5) });
    let device = KeyboardDevice::with_timing(
        Rc::new(StoppingChannels(stopping.clone())),
        log.clone(),
        Duration::from_millis(20),
        Duration::from_millis(50),
        Rc::new(|_| Box::pin(async {})),
    );
    let read = block_on(device.read_actions(0, RecordFormat::Standard, layout)).unwrap();
    assert_eq!(read.actions.len(), 5);
    assert_eq!(read.actions[&ControlId::Key(1)], KeyAction::Media(MediaKey::MUTE));
    let rest: BTreeSet<ControlId> =
        layout.controls.iter().copied().filter(|control| !matches!(control, ControlId::Key(1..=5))).collect();
    assert_eq!(read.unread, rest);

    // Records 13–15 and 22–25 hold no control of 12add2: a keyboard that stops after record 21 sent everything.
    stopping.left.set(21);
    let read = block_on(device.read_actions(0, RecordFormat::Standard, layout)).unwrap();
    assert_eq!(read.actions.len(), layout.controls.len());
    assert!(read.unread.is_empty());

    stopping.left.set(0);
    assert_eq!(block_on(device.read_actions(0, RecordFormat::Standard, layout)), Err(KeyboardDeviceError::NoReply));
}

/// A control beyond the records a read asks for — legacy 16add3's third knob turned right, in record 25 of a read of
/// 24 — is outside the read: neither read nor unread, and not compared after a save that changes it.
#[test]
fn a_control_beyond_the_read_is_outside_it() {
    let log = DeviceLogStore::new();
    let keyboard = Rc::new(SimulatedKeyboard::new(model("16add3", RecordFormat::Legacy)));
    let device = Rc::new(device(&keyboard, &log));
    let layout = KeyboardLayout::named("16add3").unwrap();
    let outside = ControlId::Knob(3, KnobPart::RotateRight);
    let read = block_on(device.read_actions(0, RecordFormat::Legacy, layout)).unwrap();
    assert_eq!(read.actions.len(), layout.controls.len() - 1);
    assert!(!read.actions.contains_key(&outside));
    assert!(read.unread.is_empty());

    let mut configuration = KeyboardConfiguration::new(3);
    configuration.set(KeyAction::Media(MediaKey::MUTE), outside, 0);
    let capabilities = KeyboardCapabilities::new(CommandSet::Records(RecordFormat::Legacy));
    let outcome = block_on(SaveKeyboardUseCase::new(device).execute(&configuration, capabilities, layout));
    let Ok(SaveOutcome::Saved(saved, SaveCheck::Matched)) = outcome else { panic!("{outcome:?}") };
    assert!(!saved.has_unsaved_changes());
    assert_eq!(keyboard.record(1, 25).unwrap()[3], 2);
}

/// A record of a kind the app doesn't know leaves its control unread.
#[test]
fn a_record_of_an_unknown_kind_is_unread() {
    let log = DeviceLogStore::new();
    let keyboard = Rc::new(SimulatedKeyboard::new(model("6add1", RecordFormat::Standard)));
    let mut record = vec![0xFD, 2, 1, 0x09];
    record.resize(64, 0);
    block_on(keyboard.write(keyboard.framing().frame(&record))).unwrap();
    let read = block_on(device(&keyboard, &log).read_actions(
        0,
        RecordFormat::Standard,
        KeyboardLayout::named("6add1").unwrap(),
    ))
    .unwrap();
    assert_eq!(read.unread, BTreeSet::from([ControlId::Key(2)]));
    assert!(!read.actions.contains_key(&ControlId::Key(2)));
}

/// Saving to a simulated keyboard that reads back reads back what it wrote — on both formats.
#[test]
fn a_save_to_a_simulated_keyboard_reads_back_as_written() {
    for (layout, format) in [("12add2", RecordFormat::Standard), ("4add1", RecordFormat::Legacy)] {
        let log = DeviceLogStore::new();
        let keyboard = Rc::new(SimulatedKeyboard::new(model(layout, format)));
        let layout = KeyboardLayout::named(layout).unwrap();
        let mut configuration = KeyboardConfiguration::new(3);
        let presses = if format == RecordFormat::Standard {
            KeyMacro::new(vec![KeyStroke::new(Modifiers::CONTROL, 0x06).with_delay(40), KeyStroke::key(0x28)])
        } else {
            KeyMacro::with_delay(vec![KeyStroke::new(Modifiers::CONTROL, 0x06), KeyStroke::key(0x28)], 300)
        };
        configuration.set(KeyAction::Keys(presses), ControlId::Key(1), 0);
        configuration.set(KeyAction::Media(MediaKey::MUTE), ControlId::Knob(1, KnobPart::Press), 1);
        configuration.set(KeyAction::Mouse(MouseAction::ControlWheelUp), ControlId::Knob(1, KnobPart::RotateLeft), 1);
        configuration.set(KeyAction::Unassigned, ControlId::Key(2), 2);
        configuration.set_lighting(
            if format == RecordFormat::Standard {
                LayerLighting::PerKey { mode: 3, colors: vec![KeyColor::new(9, 8, 7); 16] }
            } else {
                LayerLighting::ColorMode { color: LightColor::Cyan, mode: 2 }
            },
            2,
        );
        let save = SaveKeyboardUseCase::new(Rc::new(device(&keyboard, &log)));
        let outcome =
            block_on(save.execute(&configuration, KeyboardCapabilities::new(CommandSet::Records(format)), layout));
        let Ok(SaveOutcome::Saved(saved, SaveCheck::Matched)) = outcome else { panic!("{format:?}: {outcome:?}") };
        assert!(!saved.has_unsaved_changes());
    }
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
