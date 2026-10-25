use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use futures::executor::{LocalPool, LocalSpawner};
use futures::task::LocalSpawnExt;

use domain::ports::{
    KeyboardConnectionRepository, KeyboardDeviceRepository, LayerActions, LayoutProfileRepository, LocalFuture, Updates,
};
use domain::usecases::*;
use domain::{
    Channel, CommandSet, ConnectedKeyboard, ConnectionState, KeyAction, KeyColor, KeyboardConfiguration,
    KeyboardDeviceError, KeyboardFamily, KeyboardIdentity, KeyboardLayout, KnobPart, LayerLighting, LayoutProfile,
    MediaKey, RecordFormat, UsbIdentity,
};
use presentation::*;

/// A keyboard that is plugged in when told.
struct FakeConnection {
    sender: async_channel::Sender<ConnectionState>,
    receiver: RefCell<Option<async_channel::Receiver<ConnectionState>>>,
}

struct Receiving(async_channel::Receiver<ConnectionState>);

impl Updates<ConnectionState> for Receiving {
    fn next(&mut self) -> LocalFuture<'_, Option<ConnectionState>> {
        Box::pin(async move { self.0.recv().await.ok() })
    }
}

impl FakeConnection {
    fn new() -> Self {
        let (sender, receiver) = async_channel::unbounded();
        sender.try_send(ConnectionState::Searching).unwrap();
        Self { sender, receiver: RefCell::new(Some(receiver)) }
    }
}

impl KeyboardConnectionRepository for FakeConnection {
    fn connection_states(&self) -> Box<dyn Updates<ConnectionState>> {
        Box::new(Receiving(self.receiver.borrow_mut().take().expect("one subscriber")))
    }

    fn retry(&self) {}
}

#[derive(Default)]
struct FakeDevice {
    identity: Cell<Option<KeyboardIdentity>>,
    actions: RefCell<LayerActions>,
    fail_write: Cell<Option<KeyboardDeviceError>>,
    writes: RefCell<Vec<KeyboardConfiguration>>,
    reads: Cell<usize>,
}

impl KeyboardDeviceRepository for FakeDevice {
    fn ask_identity(&self) -> LocalFuture<'_, Result<Option<KeyboardIdentity>, KeyboardDeviceError>> {
        let identity = self.identity.get();
        Box::pin(async move { Ok(identity) })
    }

    fn read_actions<'a>(
        &'a self,
        _: usize,
        _: RecordFormat,
        _: &'a KeyboardLayout,
    ) -> LocalFuture<'a, Result<LayerActions, KeyboardDeviceError>> {
        self.reads.set(self.reads.get() + 1);
        let actions = self.actions.borrow().clone();
        Box::pin(async move { Ok(actions) })
    }

    fn read_lighting(&self, _: usize) -> LocalFuture<'_, Result<LayerLighting, KeyboardDeviceError>> {
        Box::pin(async { Ok(LayerLighting::PerKey { mode: 0, colors: KeyColor::DEFAULT_KEY_COLORS.to_vec() }) })
    }

    fn write<'a>(
        &'a self,
        configuration: &'a KeyboardConfiguration,
        _: CommandSet,
        _: &'a KeyboardLayout,
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

#[derive(Default)]
struct Profiles(RefCell<HashMap<UsbIdentity, LayoutProfile>>);

impl LayoutProfileRepository for Profiles {
    fn profile(&self, keyboard: UsbIdentity) -> Option<LayoutProfile> {
        self.0.borrow().get(&keyboard).cloned()
    }

    fn save(&self, profile: LayoutProfile, keyboard: UsbIdentity) {
        self.0.borrow_mut().insert(keyboard, profile);
    }
}

struct Fixture {
    connection: Rc<FakeConnection>,
    device: Rc<FakeDevice>,
    profiles: Rc<Profiles>,
    editor: Rc<EditorModel>,
    pool: LocalPool,
    spawner: LocalSpawner,
}

impl Fixture {
    fn new() -> Self {
        let connection = Rc::new(FakeConnection::new());
        let device = Rc::new(FakeDevice::default());
        let profiles = Rc::new(Profiles::default());
        let editor = EditorModel::new(EditorDependencies {
            observe_connection: ObserveConnectionUseCase::new(connection.clone()),
            retry_connection: RetryConnectionUseCase::new(connection.clone()),
            identify_keyboard: IdentifyKeyboardUseCase::with_attempts(device.clone(), 1),
            resolve_layout: ResolveLayoutUseCase::new(profiles.clone()),
            choose_layout: ChooseLayoutUseCase::new(profiles.clone()),
            read_keyboard: ReadKeyboardUseCase::new(device.clone()),
            save_keyboard: SaveKeyboardUseCase::new(device.clone()),
            keyboard_source: None,
        });
        let pool = LocalPool::new();
        let spawner = pool.spawner();
        spawner.spawn_local(editor.run()).unwrap();
        let mut fixture = Self { connection, device, profiles, editor, pool, spawner };
        fixture.settle();
        fixture
    }

    fn plug(&mut self, keyboard: ConnectedKeyboard) {
        self.connection.sender.try_send(ConnectionState::Connected(keyboard)).unwrap();
        self.settle();
    }

    fn settle(&mut self) {
        self.pool.run_until_stalled();
    }

    fn run<F: Future<Output = ()> + 'static>(&mut self, work: F) {
        self.spawner.spawn_local(work).unwrap();
        self.settle();
    }

    fn save(&mut self) {
        let editor = self.editor.clone();
        self.run(async move { editor.save().await });
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

fn k8840() -> ConnectedKeyboard {
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

fn key(usage: u8) -> &'static KeyUsage {
    KeyUsage::named(usage).unwrap()
}

const A: u8 = 0x04;
const C: u8 = 0x06;

fn control(drawing: &KeyboardDrawing, control: ControlRef) -> &ControlDrawing {
    drawing.controls.iter().find(|drawn| drawn.control == control).unwrap()
}

#[test]
fn the_8890_is_arranged() {
    let mut fixture = Fixture::new();
    fixture.plug(k8890());
    let editor = &fixture.editor;
    assert!(editor.is_ready());
    assert_eq!(
        editor.status(),
        ConnectionStatus::Connected {
            usb: "1189:8890".into(),
            keys: None,
            knobs: None,
            is_identified: true,
            is_simulated: false
        }
    );
    assert!(editor.is_choosing_layout());
    assert!(!editor.reads_back());
    assert_eq!(editor.available_tabs(), vec![EditorTab::Keys, EditorTab::Media, EditorTab::Mouse, EditorTab::Lighting]);
    assert_eq!(fixture.device.reads.get(), 0);
    let sheet = editor.layout_sheet().unwrap();
    assert_eq!(sheet.mode, LayoutSheetMode::Arrangement { keys: 12, columns: 4, knobs: 2, max_keys: 12, max_knobs: 2 });
    assert!(sheet.is_required);

    editor.choose_arrangement(6, 3, 1, 0);
    assert!(!editor.is_choosing_layout());
    assert_eq!(fixture.profiles.0.borrow()[&KeyboardFamily::K8890_USB].layout_id, "grid-6-3-1");
    assert_eq!(editor.drawing().unwrap().controls.len(), 9);

    editor.select(ControlRef::key(1));
    editor.toggle(ModifierKey::Super);
    assert_eq!(editor.held_modifier_text().as_deref(), Some("Super"));
    editor.press(key(C));
    assert!(editor.held_modifiers().is_empty());
    assert_eq!(
        editor.selection_state().unwrap().binding,
        BindingState::Presses { strokes: vec!["Super+C".into()], key_delay: 0, used: 1, limit: 5 }
    );
    let drawing = editor.drawing().unwrap();
    assert!(control(&drawing, ControlRef::key(1)).is_changed);
    assert_eq!(control(&drawing, ControlRef::key(1)).caption, ActionCaption::Presses("Super+C".into()));
    assert!(editor.has_unsaved_changes());

    fixture.save();
    let editor = &fixture.editor;
    assert_eq!(editor.notice(), Some(Notice::Saved { simulated: false }));
    assert!(!editor.has_unsaved_changes());
    assert_eq!(fixture.device.writes.borrow().len(), 1);

    // The Windows tool's keys, in its arrangement: no F13–F24, non-US keys or Keypad Enter.
    assert_eq!(editor.vendor_key_arrangement(), KeyArrangement::Alphabetical);
    assert!(editor.more_keys().is_empty());
    let off: Vec<BoardFace> =
        editor.key_board().into_iter().filter(|cap| !cap.is_enabled).map(|cap| cap.face).collect();
    assert_eq!(off, vec![BoardFace::Key(*key(0x58))]);
    let groups = editor.key_groups();
    assert_eq!(groups.iter().find(|group| group.kind == KeyGroupKind::FunctionKeys).unwrap().keys.len(), 12);
    assert_eq!(groups.iter().map(|group| group.keys.len()).sum::<usize>(), 95);
    editor.press(key(0x58));
    assert_eq!(editor.notice(), Some(Notice::Unsupported));
    assert!(!editor.has_unsaved_changes());
}

#[test]
fn a_standard_keyboard_is_read_on_connect() {
    let mut fixture = Fixture::new();
    fixture.device.identity.set(Some(KeyboardIdentity::new(6, 1, 10)));
    *fixture.device.actions.borrow_mut() = [(domain::ControlId::Key(2), KeyAction::Media(MediaKey::MUTE))].into();
    fixture.plug(k8840());
    let editor = &fixture.editor;
    assert!(editor.is_ready() && fixture.device.reads.get() > 0);
    assert_eq!(
        editor.status(),
        ConnectionStatus::Connected {
            usb: "1189:8840".into(),
            keys: Some(6),
            knobs: Some(1),
            is_identified: true,
            is_simulated: false
        }
    );
    assert!(!editor.is_choosing_layout());
    assert_eq!(editor.layout_sheet().unwrap().mode, LayoutSheetMode::Fixed);
    assert_eq!(
        control(&editor.drawing().unwrap(), ControlRef::key(2)).caption,
        ActionCaption::Media(MediaChoice::Mute)
    );
    assert_eq!(editor.available_tabs(), EditorTab::ALL.to_vec());
    assert_eq!(editor.notice(), None);
    assert_eq!(editor.vendor_key_arrangement(), KeyArrangement::Qwerty);
    assert!(editor.key_board().iter().all(|cap| cap.is_enabled));
    assert_eq!(editor.more_keys().len(), 14);

    editor.set_tab(EditorTab::Lighting);
    editor.select(ControlRef::key(1));
    let first = KeyColor::DEFAULT_KEY_COLORS[0];
    assert_eq!(
        control(&editor.drawing().unwrap(), ControlRef::key(1)).led_color,
        Some(Rgb::new(first.red, first.green, first.blue))
    );
    let Some(LightingPage::PerKeyColor { modes, keys, selected, .. }) = editor.lighting_page() else {
        panic!("not the per-key page")
    };
    assert!(modes == (0..=5).collect::<Vec<_>>() && keys == 16);
    assert_eq!(selected.unwrap().title, ControlTitle::Key(1));
    editor.set_key_color(Rgb::new(1, 2, 3));
    editor.set_lighting_mode(4);
    let Some(LightingPage::PerKeyColor { active_mode, selected, .. }) = editor.lighting_page() else { panic!() };
    assert!(active_mode == Some(4) && selected.unwrap().color == Rgb::new(1, 2, 3));

    editor.set_tab(EditorTab::Delay);
    editor.press(key(A));
    editor.press(key(C));
    editor.set_delay(1, 250);
    assert_eq!(
        editor.delay_page(),
        DelayPage::PerPress {
            range: 0..=60000,
            presses: vec![
                PressDelay { index: 0, text: "A".into(), delay: 0 },
                PressDelay { index: 1, text: "C".into(), delay: 250 },
            ],
        }
    );
}

#[test]
fn edits_need_a_selection_and_room() {
    let mut fixture = Fixture::new();
    fixture.plug(k8890());
    let editor = &fixture.editor;
    editor.press(key(A));
    assert_eq!(editor.notice(), Some(Notice::SelectControlFirst));
    editor.select(ControlRef::key(1));
    for _ in 0..6 {
        editor.press(key(A));
    }
    assert_eq!(editor.notice(), Some(Notice::Full(5)));
    assert_eq!(
        editor.media_choices(),
        vec![
            MediaChoice::PlayPause,
            MediaChoice::PreviousTrack,
            MediaChoice::NextTrack,
            MediaChoice::Mute,
            MediaChoice::VolumeUp,
            MediaChoice::VolumeDown
        ]
    );
    editor.assign_media(MediaChoice::Calculator);
    assert_eq!(editor.notice(), Some(Notice::Unsupported));
    editor.assign_media(MediaChoice::Mute);
    assert_eq!(editor.selected_caption(), Some(ActionCaption::Media(MediaChoice::Mute)));
    editor.toggle(ModifierKey::Shift);
    editor.assign_held_modifiers();
    assert_eq!(
        editor.selection_state().unwrap().binding,
        BindingState::Presses { strokes: vec!["Shift".into()], key_delay: 0, used: 1, limit: 5 }
    );
    editor.press(key(A));
    assert_eq!(
        editor.selection_state().unwrap().binding,
        BindingState::Presses { strokes: vec!["Shift+A".into()], key_delay: 0, used: 1, limit: 5 }
    );
    editor.clear_selected();
    assert_eq!(editor.selection_state().unwrap().binding, BindingState::Nothing);
    editor.select(ControlRef::key(2));
    assert_eq!(editor.selection_state().unwrap().binding, BindingState::Unknown);
    editor.set_lighting_mode(2);
    assert_eq!(editor.lighting_page(), Some(LightingPage::Modes { modes: vec![0, 1, 2], active: Some(2) }));
}

#[test]
fn a_failed_save_keeps_the_changes() {
    let mut fixture = Fixture::new();
    fixture.device.fail_write.set(Some(KeyboardDeviceError::WriteFailed { code: -7 }));
    fixture.plug(k8890());
    fixture.save();
    assert_eq!(fixture.editor.notice(), Some(Notice::NothingToSave));
    fixture.editor.set_lighting_mode(1);
    fixture.save();
    assert_eq!(fixture.editor.notice(), Some(Notice::WriteFailed(-7)));
    assert!(fixture.editor.has_unsaved_changes());
}

#[test]
fn unplugging_forgets_the_keyboard() {
    let mut fixture = Fixture::new();
    fixture.plug(k8890());
    fixture.editor.select(ControlRef::key(2));
    fixture.connection.sender.try_send(ConnectionState::Searching).unwrap();
    fixture.settle();
    let editor = &fixture.editor;
    assert_eq!(editor.status(), ConnectionStatus::Searching);
    assert_eq!(editor.selected_control(), None);
    assert!(!editor.is_ready());
    assert_eq!(editor.keyboard_info(), None);
}

/// A 884x keyboard that doesn't answer picks one of the app's layouts; rotating turns the drawing.
#[test]
fn the_layout_sheet() {
    let mut fixture = Fixture::new();
    fixture.plug(k8840());
    let editor = &fixture.editor;
    let Some(LayoutSheetMode::Catalog { choices, selected_id }) = editor.layout_sheet().map(|sheet| sheet.mode) else {
        panic!("no catalog")
    };
    assert!(choices.len() == 26 && selected_id == "15add3");
    editor.choose_layout(Some("9add2"), 1);
    assert_eq!(editor.layout_sheet().unwrap().quarter_turns, 1);
    let upright = editor.preview_drawing(Some("9add2"), 0).unwrap();
    let turned = editor.preview_drawing(Some("9add2"), 1).unwrap();
    assert!((upright.bounds.width - turned.bounds.height).abs() < 0.001);
    assert!(editor.preview_arrangement(0, 1, 0, 0).is_none());
    editor.rotate();
    assert_eq!(editor.layout_sheet().unwrap().quarter_turns, 2);
}

#[test]
fn listeners_hear_every_change() {
    let mut fixture = Fixture::new();
    let heard = Rc::new(Cell::new(0));
    let counter = heard.clone();
    fixture.editor.subscribe(move || counter.set(counter.get() + 1));
    fixture.plug(k8890());
    assert!(heard.get() > 0);
    let before = heard.get();
    fixture.editor.set_tab(EditorTab::Media);
    assert_eq!(heard.get(), before + 1);
}

#[test]
fn a_notice_goes_only_when_it_is_still_shown() {
    let mut fixture = Fixture::new();
    fixture.plug(k8890());
    let editor = &fixture.editor;
    editor.press(key(A));
    let first = editor.notice_serial();
    editor.select(ControlRef::knob(1, KnobAction::Press));
    editor.assign_media(MediaChoice::Calculator);
    assert_eq!(editor.notice(), Some(Notice::Unsupported));
    editor.expire_notice(first);
    assert_eq!(editor.notice(), Some(Notice::Unsupported));
    editor.expire_notice(editor.notice_serial());
    assert_eq!(editor.notice(), None);
    let _ = KnobPart::Press;
}
