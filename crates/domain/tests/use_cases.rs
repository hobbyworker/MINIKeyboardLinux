use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use futures_executor::block_on;

use domain::ports::{KeyboardDeviceRepository, LayerActions, LocalFuture};
use domain::usecases::*;
use domain::*;

const KEY1: ControlId = ControlId::Key(1);

fn target(commands: CommandSet, control: Option<ControlId>, layer: usize) -> EditTarget {
    EditTarget { capabilities: KeyboardCapabilities::new(commands), layer, control }
}

fn key1(commands: CommandSet) -> EditTarget {
    target(commands, Some(KEY1), 0)
}

const K8890: CommandSet = CommandSet::Packets { report_id: 3 };
const STANDARD: CommandSet = CommandSet::Records(RecordFormat::Standard);
const LEGACY: CommandSet = CommandSet::Records(RecordFormat::Legacy);

fn keys(strokes: Vec<KeyStroke>) -> KeyAction {
    KeyAction::Keys(KeyMacro::new(strokes))
}

#[test]
fn presses_add_up() {
    let mut configuration = KeyboardConfiguration::new(3);
    let target = key1(STANDARD);
    assert_eq!(
        AppendStrokeUseCase.execute(KeyStroke::new(Modifiers::SUPER, 0x06), &mut configuration, &target),
        EditOutcome::Applied
    );
    assert_eq!(AppendStrokeUseCase.execute(KeyStroke::key(0x28), &mut configuration, &target), EditOutcome::Applied);
    assert_eq!(
        configuration.action(KEY1, 0),
        Some(&keys(vec![KeyStroke::new(Modifiers::SUPER, 0x06), KeyStroke::key(0x28)]))
    );
    assert!(configuration.is_changed(KEY1, 0));
    assert_eq!(configuration.action(KEY1, 1), None);
}

#[test]
fn nothing_without_a_selection() {
    let mut configuration = KeyboardConfiguration::new(3);
    let target = target(STANDARD, None, 0);
    assert_eq!(
        AppendStrokeUseCase.execute(KeyStroke::key(0x04), &mut configuration, &target),
        EditOutcome::NeedsSelection
    );
    assert!(!configuration.has_unsaved_changes());
}

/// Modifiers alone make a key a modifier key; a key pressed next takes them, as the vendor tools' modifier
/// buttons work.
#[test]
fn modifiers_alone() {
    let mut configuration = KeyboardConfiguration::new(3);
    let target = key1(K8890);
    assert_eq!(
        AppendStrokeUseCase.execute(KeyStroke::new(Modifiers::CONTROL, 0), &mut configuration, &target),
        EditOutcome::Applied
    );
    assert_eq!(configuration.action(KEY1, 0), Some(&keys(vec![KeyStroke::new(Modifiers::CONTROL, 0)])));
    assert_eq!(
        AppendStrokeUseCase.execute(KeyStroke::new(Modifiers::SHIFT, 0x06), &mut configuration, &target),
        EditOutcome::Applied
    );
    assert_eq!(
        configuration.action(KEY1, 0),
        Some(&keys(vec![KeyStroke::new(Modifiers::CONTROL | Modifiers::SHIFT, 0x06)]))
    );
    assert_eq!(
        AppendStrokeUseCase.execute(KeyStroke::new(Modifiers::ALT, 0), &mut configuration, &target),
        EditOutcome::ModifiersOnlyAlone
    );
}

/// A key the keyboard's vendor tool doesn't offer isn't added, pressed or set as a whole action.
#[test]
fn a_key_the_keyboard_is_not_given_stays_out() {
    let mut configuration = KeyboardConfiguration::new(3);
    let k8890 = key1(K8890);
    assert_eq!(AppendStrokeUseCase.execute(KeyStroke::key(0x68), &mut configuration, &k8890), EditOutcome::Unsupported);
    assert_eq!(
        SetActionUseCase.execute(keys(vec![KeyStroke::key(0x58)]), &mut configuration, &k8890),
        EditOutcome::Unsupported
    );
    assert!(!configuration.has_unsaved_changes());
    assert_eq!(
        AppendStrokeUseCase.execute(KeyStroke::key(0x68), &mut configuration, &key1(STANDARD)),
        EditOutcome::Applied
    );
}

#[test]
fn a_full_key_takes_no_more() {
    let mut configuration = KeyboardConfiguration::new(3);
    let target = key1(K8890);
    for _ in 0..5 {
        assert_eq!(
            AppendStrokeUseCase.execute(KeyStroke::key(0x04), &mut configuration, &target),
            EditOutcome::Applied
        );
    }
    assert_eq!(AppendStrokeUseCase.execute(KeyStroke::key(0x04), &mut configuration, &target), EditOutcome::Full(5));
}

#[test]
fn a_media_key_replaces_presses() {
    let mut configuration = KeyboardConfiguration::new(3);
    let target = key1(K8890);
    AppendStrokeUseCase.execute(KeyStroke::key(0x04), &mut configuration, &target);
    assert_eq!(
        SetActionUseCase.execute(KeyAction::Media(MediaKey::MUTE), &mut configuration, &target),
        EditOutcome::Applied
    );
    assert_eq!(configuration.action(KEY1, 0), Some(&KeyAction::Media(MediaKey::MUTE)));
    // The 8890 knows six media keys.
    assert_eq!(
        SetActionUseCase.execute(KeyAction::Media(MediaKey::CALCULATOR), &mut configuration, &target),
        EditOutcome::Unsupported
    );
    // A key pressed now starts over.
    AppendStrokeUseCase.execute(KeyStroke::key(0x05), &mut configuration, &target);
    assert_eq!(configuration.action(KEY1, 0), Some(&keys(vec![KeyStroke::key(0x05)])));
}

#[test]
fn removing_presses() {
    let mut configuration = KeyboardConfiguration::new(3);
    let target = key1(LEGACY);
    AppendStrokeUseCase.execute(KeyStroke::key(0x04), &mut configuration, &target);
    AppendStrokeUseCase.execute(KeyStroke::key(0x05), &mut configuration, &target);
    assert_eq!(RemoveLastStrokeUseCase.execute(&mut configuration, &target), EditOutcome::Applied);
    assert_eq!(configuration.action(KEY1, 0), Some(&keys(vec![KeyStroke::key(0x04)])));
    assert_eq!(RemoveLastStrokeUseCase.execute(&mut configuration, &target), EditOutcome::Applied);
    assert_eq!(configuration.action(KEY1, 0), Some(&KeyAction::Unassigned));
    assert_eq!(RemoveLastStrokeUseCase.execute(&mut configuration, &target), EditOutcome::Unsupported);
}

#[test]
fn delays() {
    let mut configuration = KeyboardConfiguration::new(3);
    let standard = key1(STANDARD);
    AppendStrokeUseCase.execute(KeyStroke::key(0x04), &mut configuration, &standard);
    AppendStrokeUseCase.execute(KeyStroke::key(0x05), &mut configuration, &standard);
    assert_eq!(SetPressDelayUseCase.execute(1, 70000, &mut configuration, &standard), EditOutcome::Applied);
    assert_eq!(
        configuration.action(KEY1, 0),
        Some(&keys(vec![KeyStroke::key(0x04), KeyStroke::key(0x05).with_delay(60000)]))
    );
    assert_eq!(SetKeyDelayUseCase.execute(100, &mut configuration, &standard), EditOutcome::Unsupported);

    let legacy = key1(LEGACY);
    assert_eq!(SetKeyDelayUseCase.execute(9000, &mut configuration, &legacy), EditOutcome::Applied);
    let Some(KeyAction::Keys(macro_)) = configuration.action(KEY1, 0) else { panic!("the key lost its presses") };
    assert_eq!(macro_.delay, 6000);
    assert_eq!(SetPressDelayUseCase.execute(0, 5, &mut configuration, &key1(K8890)), EditOutcome::Unsupported);
}

#[test]
fn clearing() {
    let mut configuration = KeyboardConfiguration::new(3);
    assert_eq!(ClearUseCase.execute(&mut configuration, &key1(STANDARD)), EditOutcome::Applied);
    assert_eq!(configuration.action(KEY1, 0), Some(&KeyAction::Unassigned));
    let layout = KeyboardLayout::named("6add1").unwrap();
    ClearUseCase.execute_all(layout, &mut configuration, 2);
    assert!(layout.controls.iter().all(|control| configuration.action(*control, 2) == Some(&KeyAction::Unassigned)));
    configuration.mark_saved();
    assert!(!configuration.has_unsaved_changes());
}

#[test]
fn lighting() {
    let mut configuration = KeyboardConfiguration::new(3);
    let k8890 = key1(K8890);
    assert_eq!(SetLightingUseCase.set_mode(2, &mut configuration, &k8890), EditOutcome::Applied);
    assert_eq!(configuration.lighting(0), Some(&LayerLighting::Mode(2)));
    assert_eq!(SetLightingUseCase.set_mode(3, &mut configuration, &k8890), EditOutcome::Unsupported);

    let legacy = target(LEGACY, Some(KEY1), 1);
    assert_eq!(SetLightingUseCase.set_color(LightColor::Red, &mut configuration, &legacy), EditOutcome::Applied);
    assert_eq!(SetLightingUseCase.set_mode(4, &mut configuration, &legacy), EditOutcome::Applied);
    assert_eq!(configuration.lighting(1), Some(&LayerLighting::ColorMode { color: LightColor::Red, mode: 4 }));

    let color = KeyColor::new(1, 2, 3);
    let standard = target(STANDARD, Some(ControlId::Key(16)), 2);
    assert_eq!(SetLightingUseCase.set_key_color(color, &mut configuration, &standard), EditOutcome::Applied);
    let Some(LayerLighting::PerKey { mode, colors }) = configuration.lighting(2) else { panic!("no colours") };
    assert_eq!(*mode, 0);
    assert_eq!(colors[15], color);
    let knob = target(STANDARD, Some(ControlId::Knob(1, KnobPart::Press)), 2);
    assert_eq!(SetLightingUseCase.set_key_color(color, &mut configuration, &knob), EditOutcome::NeedsColorKey(16));
}

#[test]
fn reading_replaces_what_the_app_had() {
    let mut configuration = KeyboardConfiguration::new(3);
    configuration.set(KeyAction::Media(MediaKey::MUTE), KEY1, 0);
    configuration.apply_read(HashMap::from([(KEY1, KeyAction::Mouse(MouseAction::LeftClick))]), 0);
    assert_eq!(configuration.action(KEY1, 0), Some(&KeyAction::Mouse(MouseAction::LeftClick)));
    assert!(!configuration.has_unsaved_changes());
}

/// A keyboard that answers as told.
#[derive(Default)]
struct FakeDevice {
    identities: RefCell<VecDeque<Result<Option<KeyboardIdentity>, KeyboardDeviceError>>>,
    actions: RefCell<LayerActions>,
    lighting: Option<LayerLighting>,
    fail_write: Cell<Option<KeyboardDeviceError>>,
    asked: Cell<usize>,
    writes: RefCell<Vec<KeyboardConfiguration>>,
}

impl FakeDevice {
    fn lighting_result(&self) -> LayerLighting {
        self.lighting
            .clone()
            .unwrap_or(LayerLighting::PerKey { mode: 1, colors: KeyColor::DEFAULT_KEY_COLORS.to_vec() })
    }
}

impl KeyboardDeviceRepository for FakeDevice {
    fn ask_identity(&self) -> LocalFuture<'_, Result<Option<KeyboardIdentity>, KeyboardDeviceError>> {
        self.asked.set(self.asked.get() + 1);
        let next = self.identities.borrow_mut().pop_front().unwrap_or(Ok(None));
        Box::pin(async move { next })
    }

    fn read_actions<'a>(
        &'a self,
        _layer: usize,
        _format: RecordFormat,
        _layout: &'a KeyboardLayout,
    ) -> LocalFuture<'a, Result<LayerActions, KeyboardDeviceError>> {
        let actions = self.actions.borrow().clone();
        Box::pin(async move { Ok(actions) })
    }

    fn read_lighting(&self, _layer: usize) -> LocalFuture<'_, Result<LayerLighting, KeyboardDeviceError>> {
        let lighting = self.lighting_result();
        Box::pin(async move { Ok(lighting) })
    }

    fn write<'a>(
        &'a self,
        configuration: &'a KeyboardConfiguration,
        _commands: CommandSet,
        _layout: &'a KeyboardLayout,
    ) -> LocalFuture<'a, Result<(), KeyboardDeviceError>> {
        Box::pin(async move {
            if let Some(error) = self.fail_write.get() {
                return Err(error);
            }
            self.writes.borrow_mut().push(configuration.clone());
            Ok(())
        })
    }
}

fn k884x() -> ConnectedKeyboard {
    ConnectedKeyboard {
        usb: UsbIdentity::new(0x1189, 0x8840),
        family: KeyboardFamily::K884x,
        interface_number: 0,
        channel: Channel::Hid,
        report_id: 3,
        report_length: 65,
        details: None,
    }
}

fn k8890() -> ConnectedKeyboard {
    ConnectedKeyboard {
        usb: KeyboardFamily::K8890_USB,
        family: KeyboardFamily::K8890,
        interface_number: 1,
        channel: Channel::Usb { out_endpoint: 2 },
        report_id: 3,
        report_length: 65,
        details: None,
    }
}

#[test]
fn the_8890_is_asked_nothing() {
    let device = Rc::new(FakeDevice::default());
    let identification = block_on(IdentifyKeyboardUseCase::new(device.clone()).execute(&k8890())).unwrap();
    assert_eq!(identification.commands, CommandSet::Packets { report_id: 3 });
    assert_eq!(device.asked.get(), 0);
}

#[test]
fn a_884x_keyboard_is_asked_until_it_answers() {
    let device = Rc::new(FakeDevice::default());
    let identity = KeyboardIdentity::new(6, 1, 10);
    *device.identities.borrow_mut() =
        VecDeque::from([Err(KeyboardDeviceError::InvalidReply), Ok(None), Ok(Some(identity))]);
    let identification = block_on(IdentifyKeyboardUseCase::new(device.clone()).execute(&k884x())).unwrap();
    assert_eq!(identification, KeyboardIdentification { commands: STANDARD, identity: Some(identity) });
    assert_eq!(device.asked.get(), 3);
}

#[test]
fn a_keyboard_that_never_answers_takes_fe_records() {
    let device = Rc::new(FakeDevice::default());
    let identification = block_on(IdentifyKeyboardUseCase::new(device.clone()).execute(&k884x())).unwrap();
    assert_eq!(
        identification,
        KeyboardIdentification { commands: CommandSet::Records(RecordFormat::Unidentified), identity: None }
    );
    assert_eq!(device.asked.get(), 3);
}

#[test]
fn a_write_failure_ends_the_asking() {
    let device = Rc::new(FakeDevice::default());
    *device.identities.borrow_mut() = VecDeque::from([Err(KeyboardDeviceError::WriteFailed { code: 1 })]);
    let result = block_on(IdentifyKeyboardUseCase::new(device.clone()).execute(&k884x()));
    assert_eq!(result, Err(KeyboardDeviceError::WriteFailed { code: 1 }));
    assert_eq!(device.asked.get(), 1);
}

#[test]
fn reading_the_standard_format_reads_lights_too() {
    let device = Rc::new(FakeDevice::default());
    *device.actions.borrow_mut() = HashMap::from([(KEY1, KeyAction::Media(MediaKey::MUTE))]);
    let layout = KeyboardLayout::named("6add1").unwrap();
    let read = ReadKeyboardUseCase::new(device.clone());
    let outcome = block_on(read.execute(KeyboardConfiguration::new(3), KeyboardCapabilities::new(STANDARD), layout));
    assert_eq!(outcome.error, None);
    assert_eq!(outcome.configuration.action(KEY1, 2), Some(&KeyAction::Media(MediaKey::MUTE)));
    assert_eq!(outcome.configuration.lighting(1), Some(&device.lighting_result()));

    let legacy = block_on(read.execute(KeyboardConfiguration::new(3), KeyboardCapabilities::new(LEGACY), layout));
    assert_eq!(legacy.configuration.lighting(0), None);

    let k8890 = block_on(read.execute(KeyboardConfiguration::new(3), KeyboardCapabilities::new(K8890), layout));
    assert_eq!(k8890.error, Some(KeyboardDeviceError::Unsupported));
}

#[test]
fn saving_marks_the_changes_saved() {
    let device = Rc::new(FakeDevice::default());
    let layout = KeyboardLayout::named("6add1").unwrap();
    let capabilities = KeyboardCapabilities::new(STANDARD);
    let save = SaveKeyboardUseCase::new(device.clone());
    let mut configuration = KeyboardConfiguration::new(3);
    assert_eq!(block_on(save.execute(&configuration, capabilities, layout)), Ok(SaveOutcome::NothingToSave));
    configuration.set(KeyAction::Media(MediaKey::MUTE), KEY1, 0);
    let Ok(SaveOutcome::Saved(saved)) = block_on(save.execute(&configuration, capabilities, layout)) else {
        panic!("not saved")
    };
    assert!(!saved.has_unsaved_changes());
    assert_eq!(saved.action(KEY1, 0), Some(&KeyAction::Media(MediaKey::MUTE)));
    assert_eq!(*device.writes.borrow(), vec![configuration.clone()]);

    device.fail_write.set(Some(KeyboardDeviceError::WriteFailed { code: 7 }));
    assert_eq!(
        block_on(save.execute(&configuration, capabilities, layout)),
        Err(KeyboardDeviceError::WriteFailed { code: 7 })
    );
}
