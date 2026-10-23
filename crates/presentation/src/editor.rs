use std::cell::{Ref, RefCell};
use std::fmt;
use std::future::Future;
use std::rc::{Rc, Weak};

use domain::usecases::{
    AppendStrokeUseCase, ChooseLayoutUseCase, ClearUseCase, EditOutcome, EditTarget, IdentifyKeyboardUseCase,
    KeyboardSourceUseCase, ObserveConnectionUseCase, ReadKeyboardUseCase, RemoveLastStrokeUseCase,
    ResolveLayoutUseCase, RetryConnectionUseCase, SaveKeyboardUseCase, SaveOutcome, SetActionUseCase,
    SetKeyDelayUseCase, SetLightingUseCase, SetPressDelayUseCase,
};
use domain::{
    ConnectedKeyboard, ConnectionState, ControlId, DelaySupport, GridShape, KeyAction, KeyColor, KeyStroke,
    KeyboardCapabilities, KeyboardConfiguration, KeyboardDeviceError, KeyboardIdentification, KeyboardLayout,
    KeyboardSource, LayerLighting, LightingSupport, Modifiers, ReadSupport, ResolvedLayout, Rotation, SimulatedModel,
};

use crate::choices::{ControlRef, LedColor, MediaChoice, ModifierKey, MouseChoice, Rgb};
use crate::screen_state::SimulatorChoice;
use crate::state::{Activity, EditorTab, Notice};
use crate::tables::KeyUsage;

/// The use cases the editor works through, wired in by the composition root.
#[derive(Debug)]
pub struct EditorDependencies {
    pub observe_connection: ObserveConnectionUseCase,
    pub retry_connection: RetryConnectionUseCase,
    pub identify_keyboard: IdentifyKeyboardUseCase,
    pub resolve_layout: ResolveLayoutUseCase,
    pub choose_layout: ChooseLayoutUseCase,
    pub read_keyboard: ReadKeyboardUseCase,
    pub save_keyboard: SaveKeyboardUseCase,
    /// The simulator's switch, in debug builds.
    pub keyboard_source: Option<KeyboardSourceUseCase>,
}

/// The main window's state and what the user does in it: the keyboard connected, its layout, the layer, key
/// and page shown, and the app's copy of the keyboard's configuration. The rules are the use cases'; this
/// keeps the screen, turns their outcomes into notices, and gives the views what they draw in the
/// presentation's own types (see `screen.rs`).
pub struct EditorModel {
    pub(crate) state: RefCell<State>,
    deps: EditorDependencies,
    listeners: RefCell<Vec<Rc<dyn Fn()>>>,
}

pub(crate) struct State {
    pub(crate) activity: Option<Activity>,
    pub(crate) notice: Option<Notice>,
    pub(crate) notice_serial: u64,
    /// The layout sheet is up — on its own when a keyboard that can't say its shape needs one set.
    pub(crate) is_choosing_layout: bool,
    pub(crate) tab: EditorTab,
    pub(crate) layer: usize,
    pub(crate) connection: ConnectionState,
    pub(crate) identification: Option<KeyboardIdentification>,
    pub(crate) resolved_layout: Option<ResolvedLayout>,
    pub(crate) configuration: KeyboardConfiguration,
    pub(crate) selection: Option<ControlId>,
    /// Modifiers picked for the next key.
    pub(crate) held: Modifiers,
    /// The keyboard on USB, or a simulated one (debug builds).
    pub(crate) source: KeyboardSource,
}

impl State {
    pub(crate) fn keyboard(&self) -> Option<&ConnectedKeyboard> {
        self.connection.keyboard()
    }

    pub(crate) fn capabilities(&self) -> Option<KeyboardCapabilities> {
        self.identification.map(|identification| KeyboardCapabilities::new(identification.commands))
    }

    /// Connected, identified and not busy.
    pub(crate) fn is_ready(&self) -> bool {
        self.keyboard().is_some() && self.identification.is_some() && self.activity.is_none()
    }

    pub(crate) fn layer_count(&self) -> usize {
        self.capabilities().map_or(3, |capabilities| capabilities.layer_count())
    }

    pub(crate) fn available_tabs(&self) -> Vec<EditorTab> {
        let Some(capabilities) = self.capabilities() else { return EditorTab::ALL.to_vec() };
        EditorTab::ALL
            .into_iter()
            .filter(|tab| *tab != EditorTab::Delay || capabilities.delays() != DelaySupport::Unavailable)
            .collect()
    }

    pub(crate) fn reads_back(&self) -> bool {
        self.capabilities().is_some_and(|capabilities| capabilities.reading() != ReadSupport::Never)
    }

    pub(crate) fn action(&self, control: ControlId) -> Option<&KeyAction> {
        self.configuration.action(control, self.layer)
    }

    pub(crate) fn selected_action(&self) -> Option<&KeyAction> {
        self.selection.and_then(|control| self.action(control))
    }

    pub(crate) fn lighting(&self) -> Option<&LayerLighting> {
        self.configuration.lighting(self.layer)
    }

    /// The colour of a key on a keyboard that lights each key.
    pub(crate) fn key_color(&self, control: ControlId) -> Option<KeyColor> {
        let LightingSupport::PerKeyColor { keys, .. } = self.capabilities()?.lighting() else { return None };
        let Some(LayerLighting::PerKey { colors, .. }) = self.lighting() else { return None };
        colors.get(SetLightingUseCase::color_index(control, keys)?).copied()
    }

    fn target(&self) -> Option<EditTarget> {
        Some(EditTarget { capabilities: self.capabilities()?, layer: self.layer, control: self.selection })
    }

    fn show(&mut self, notice: Notice) {
        self.notice = Some(notice);
        self.notice_serial += 1;
    }

    fn report(&mut self, outcome: EditOutcome) {
        match outcome {
            EditOutcome::Applied => {}
            EditOutcome::NeedsSelection => self.show(Notice::SelectControlFirst),
            EditOutcome::Unsupported => self.show(Notice::Unsupported),
            EditOutcome::Full(limit) => self.show(Notice::Full(limit)),
            EditOutcome::ModifiersOnlyAlone => self.show(Notice::ModifiersOnlyAlone),
            EditOutcome::NeedsColorKey(keys) => self.show(Notice::NeedsColorKey(keys)),
        }
    }
}

impl EditorModel {
    pub fn new(deps: EditorDependencies) -> Rc<Self> {
        Rc::new(Self {
            state: RefCell::new(State {
                activity: None,
                notice: None,
                notice_serial: 0,
                is_choosing_layout: false,
                tab: EditorTab::Keys,
                layer: 0,
                connection: ConnectionState::Searching,
                identification: None,
                resolved_layout: None,
                configuration: KeyboardConfiguration::new(3),
                selection: None,
                held: Modifiers::NONE,
                source: KeyboardSource::Hardware,
            }),
            deps,
            listeners: RefCell::default(),
        })
    }

    /// Calls `listener` after every change.
    pub fn subscribe(&self, listener: impl Fn() + 'static) {
        self.listeners.borrow_mut().push(Rc::new(listener));
    }

    pub(crate) fn state(&self) -> Ref<'_, State> {
        self.state.borrow()
    }

    /// Changes the state, then tells the listeners — after the change is done, so they can read it.
    fn update<R>(&self, change: impl FnOnce(&mut State) -> R) -> R {
        let result = change(&mut self.state.borrow_mut());
        let listeners = self.listeners.borrow().clone();
        for listener in listeners {
            listener();
        }
        result
    }

    /// Follows the keyboard until the model goes away; one that comes is identified at once.
    pub fn run(self: &Rc<Self>) -> impl Future<Output = ()> + 'static {
        let model: Weak<Self> = Rc::downgrade(self);
        if let Some(source) = &self.deps.keyboard_source {
            let current = source.current();
            self.update(|state| state.source = current);
        }
        let mut states = self.deps.observe_connection.execute();
        async move {
            while let Some(state) = states.next().await {
                let Some(model) = model.upgrade() else { return };
                model.connection_changed(state).await;
            }
        }
    }

    // State the views read

    pub fn activity(&self) -> Option<Activity> {
        self.state().activity
    }

    pub fn notice(&self) -> Option<Notice> {
        self.state().notice
    }

    /// Counts up with each notice shown, so the one to take away can be told from a newer one.
    pub fn notice_serial(&self) -> u64 {
        self.state().notice_serial
    }

    pub fn is_choosing_layout(&self) -> bool {
        self.state().is_choosing_layout
    }

    pub fn set_choosing_layout(&self, choosing: bool) {
        self.update(|state| state.is_choosing_layout = choosing);
    }

    pub fn tab(&self) -> EditorTab {
        self.state().tab
    }

    pub fn set_tab(&self, tab: EditorTab) {
        self.update(|state| state.tab = tab);
    }

    pub fn layer(&self) -> usize {
        self.state().layer
    }

    pub fn set_layer(&self, layer: usize) {
        self.update(|state| state.layer = layer.min(state.layer_count() - 1));
    }

    pub fn is_ready(&self) -> bool {
        self.state().is_ready()
    }

    pub fn layer_count(&self) -> usize {
        self.state().layer_count()
    }

    pub fn available_tabs(&self) -> Vec<EditorTab> {
        self.state().available_tabs()
    }

    /// Whether the keyboard reads back at all — then the header has Read.
    pub fn reads_back(&self) -> bool {
        self.state().reads_back()
    }

    pub fn can_read(&self) -> bool {
        let state = self.state();
        state.is_ready() && state.reads_back()
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.state().configuration.has_unsaved_changes()
    }

    // Connection

    async fn connection_changed(&self, new: ConnectionState) {
        let identify = self.update(|state| {
            let previous = state.keyboard().cloned();
            state.connection = new;
            if state.keyboard() == previous.as_ref() {
                return false;
            }
            state.identification = None;
            state.selection = None;
            state.held = Modifiers::NONE;
            let Some(keyboard) = state.keyboard() else { return false };
            if Some(keyboard.usb) != previous.map(|previous| previous.usb) {
                state.resolved_layout = None;
            }
            true
        });
        if identify {
            self.identify().await;
        }
    }

    pub fn retry_connection(&self) {
        self.deps.retry_connection.execute();
    }

    /// Settles the keyboard's commands and layout, then reads it if it reads back on its own.
    pub async fn identify(&self) {
        let keyboard = {
            let state = self.state();
            match (state.keyboard(), state.identification, state.activity) {
                (Some(keyboard), None, None) => keyboard.clone(),
                _ => return,
            }
        };
        self.update(|state| state.activity = Some(Activity::Identifying));
        let identified = match self.deps.identify_keyboard.execute(&keyboard).await {
            Ok(identified) => identified,
            Err(_) => {
                self.update(|state| {
                    state.activity = None;
                    state.show(Notice::IdentifyFailed);
                });
                return;
            }
        };
        let resolve_layout = &self.deps.resolve_layout;
        let read_now = self.update(|state| {
            state.activity = None;
            if state.keyboard() != Some(&keyboard) {
                return false;
            }
            state.identification = Some(identified);
            let capabilities = KeyboardCapabilities::new(identified.commands);
            let resolved = resolve_layout.execute(&keyboard, &identified);
            state.is_choosing_layout = resolved.needs_choice();
            state.resolved_layout = Some(resolved);
            state.configuration = KeyboardConfiguration::new(capabilities.layer_count());
            state.layer = state.layer.min(capabilities.layer_count() - 1);
            if !state.available_tabs().contains(&state.tab) {
                state.tab = EditorTab::Keys;
            }
            capabilities.reading() == ReadSupport::OnConnect
        });
        if read_now {
            self.read_with(false).await;
        }
    }

    // Reading and saving

    pub async fn read(&self) {
        self.read_with(true).await;
    }

    async fn read_with(&self, announce: bool) {
        let (configuration, capabilities, layout) = {
            let state = self.state();
            let (Some(capabilities), Some(resolved)) = (state.capabilities(), &state.resolved_layout) else { return };
            if !(state.is_ready() && state.reads_back()) {
                return;
            }
            (state.configuration.clone(), capabilities, resolved.layout.clone())
        };
        self.update(|state| state.activity = Some(Activity::Reading));
        let outcome = self.deps.read_keyboard.execute(configuration, capabilities, &layout).await;
        self.update(|state| {
            state.activity = None;
            state.configuration = outcome.configuration;
            if outcome.error.is_some() {
                state.show(Notice::ReadFailed);
            } else if announce {
                state.show(Notice::ReadComplete);
            }
        });
    }

    pub async fn save(&self) {
        let (configuration, capabilities, layout) = {
            let state = self.state();
            let (Some(capabilities), Some(resolved)) = (state.capabilities(), &state.resolved_layout) else { return };
            if !state.is_ready() {
                return;
            }
            (state.configuration.clone(), capabilities, resolved.layout.clone())
        };
        if !configuration.has_unsaved_changes() {
            self.update(|state| state.show(Notice::NothingToSave));
            return;
        }
        self.update(|state| state.activity = Some(Activity::Saving));
        let result = self.deps.save_keyboard.execute(&configuration, capabilities, &layout).await;
        self.update(|state| {
            state.activity = None;
            match result {
                Ok(SaveOutcome::Saved(saved)) => {
                    state.configuration = saved;
                    state.show(Notice::Saved);
                }
                Ok(SaveOutcome::NothingToSave) => state.show(Notice::NothingToSave),
                Err(KeyboardDeviceError::WriteFailed { code }) => state.show(Notice::WriteFailed(code)),
                Err(KeyboardDeviceError::Unsupported) => state.show(Notice::Unsupported),
                Err(_) => state.show(Notice::KeyboardGone),
            }
        });
    }

    // Layout

    /// The layout and rotation the user chose; on a keyboard that said its shape, only the rotation.
    fn apply_layout(&self, layout: &KeyboardLayout, rotation: Rotation) {
        let choose_layout = &self.deps.choose_layout;
        self.update(|state| {
            let (Some(keyboard), Some(resolved)) = (state.keyboard(), &state.resolved_layout) else { return };
            let chosen = choose_layout.execute(resolved, layout, rotation, keyboard.usb);
            if let Some(selection) = state.selection
                && !chosen.layout.has(selection)
            {
                state.selection = None;
            }
            state.resolved_layout = Some(chosen);
            state.is_choosing_layout = false;
        });
    }

    /// One of the cross-platform app's layouts by its name, or with `None` the keyboard's own — only turned.
    pub fn choose_layout(&self, id: Option<&str>, quarter_turns: i64) {
        let layout = match id.and_then(KeyboardLayout::named) {
            Some(layout) => layout.clone(),
            None => match &self.state().resolved_layout {
                Some(resolved) => resolved.layout.clone(),
                None => return,
            },
        };
        self.apply_layout(&layout, Rotation::from_quarter_turns(quarter_turns));
    }

    /// An arrangement: keys, keys to a row, knobs.
    pub fn choose_arrangement(&self, keys: u8, columns: u8, knobs: u8, quarter_turns: i64) {
        let Some(shape) = GridShape::new(keys, columns, knobs) else { return };
        self.apply_layout(&shape.layout(), Rotation::from_quarter_turns(quarter_turns));
    }

    /// Whether there is a layout to turn or change.
    pub fn has_layout(&self) -> bool {
        self.state().resolved_layout.is_some()
    }

    pub fn rotate(&self) {
        let Some((layout, rotation)) =
            self.state().resolved_layout.as_ref().map(|resolved| (resolved.layout.clone(), resolved.rotation))
        else {
            return;
        };
        self.apply_layout(&layout, rotation.next());
    }

    // Editing

    pub fn select(&self, control: ControlRef) {
        self.update(|state| {
            if state.is_ready() {
                state.selection = Some(control.0);
            }
        });
    }

    fn edit(&self, change: impl FnOnce(&mut KeyboardConfiguration, &EditTarget) -> EditOutcome) {
        self.update(|state| {
            if !state.is_ready() {
                return;
            }
            let Some(target) = state.target() else { return };
            let outcome = change(&mut state.configuration, &target);
            state.report(outcome);
        });
    }

    /// A key, with the modifiers held for it.
    pub fn press(&self, key: &KeyUsage) {
        let stroke = KeyStroke::new(self.state().held, key.usage);
        self.edit(|configuration, target| AppendStrokeUseCase.execute(stroke, configuration, target));
        self.update(|state| state.held = Modifiers::NONE);
    }

    /// A shifted symbol: Shift and its key.
    pub fn press_shifted(&self, key: &KeyUsage) {
        let stroke = KeyStroke::new(self.state().held | Modifiers::SHIFT, key.usage);
        self.edit(|configuration, target| AppendStrokeUseCase.execute(stroke, configuration, target));
        self.update(|state| state.held = Modifiers::NONE);
    }

    pub fn toggle(&self, modifier: ModifierKey) {
        self.update(|state| state.held.toggle(modifier.modifiers()));
    }

    /// The held modifiers alone, which makes the key a modifier key.
    pub fn assign_held_modifiers(&self) {
        let held = self.state().held;
        if held.is_empty() {
            return;
        }
        self.edit(|configuration, target| AppendStrokeUseCase.execute(KeyStroke::new(held, 0), configuration, target));
        self.update(|state| state.held = Modifiers::NONE);
    }

    pub fn remove_last_stroke(&self) {
        self.edit(|configuration, target| RemoveLastStrokeUseCase.execute(configuration, target));
    }

    pub fn assign_media(&self, media: MediaChoice) {
        self.edit(|configuration, target| {
            SetActionUseCase.execute(KeyAction::Media(media.key()), configuration, target)
        });
    }

    pub fn assign_mouse(&self, mouse: MouseChoice) {
        self.edit(|configuration, target| {
            SetActionUseCase.execute(KeyAction::Mouse(mouse.action()), configuration, target)
        });
    }

    pub fn set_delay(&self, press: usize, milliseconds: u32) {
        self.edit(|configuration, target| SetPressDelayUseCase.execute(press, milliseconds, configuration, target));
    }

    pub fn set_key_delay(&self, milliseconds: u32) {
        self.edit(|configuration, target| SetKeyDelayUseCase.execute(milliseconds, configuration, target));
    }

    pub fn clear_selected(&self) {
        self.edit(|configuration, target| ClearUseCase.execute(configuration, target));
    }

    pub fn clear_layer(&self) {
        let Some(layout) = self.state().resolved_layout.as_ref().map(|resolved| resolved.layout.clone()) else {
            return;
        };
        self.edit(|configuration, target| ClearUseCase.execute_all(&layout, configuration, target.layer));
    }

    pub fn set_lighting_mode(&self, mode: u8) {
        self.edit(|configuration, target| SetLightingUseCase.set_mode(mode, configuration, target));
    }

    pub fn set_light_color(&self, color: LedColor) {
        self.edit(|configuration, target| SetLightingUseCase.set_color(color.color(), configuration, target));
    }

    pub fn set_key_color(&self, color: Rgb) {
        self.edit(|configuration, target| SetLightingUseCase.set_key_color(color.key_color(), configuration, target));
    }

    // Simulator

    pub fn can_select_source(&self) -> bool {
        self.deps.keyboard_source.is_some()
    }

    /// Every keyboard the simulator plays, when it is wired in (debug builds).
    pub fn simulator_choices(&self) -> Vec<SimulatorChoice> {
        if self.deps.keyboard_source.is_none() {
            return vec![];
        }
        SimulatedModel::all().iter().map(SimulatorChoice::of).collect()
    }

    /// The simulated keyboard in use, or `None` for the keyboard on USB.
    pub fn active_simulator_id(&self) -> Option<String> {
        match &self.state().source {
            KeyboardSource::Simulated(model) => Some(model.id()),
            KeyboardSource::Hardware => None,
        }
    }

    /// Switches to a simulated keyboard, or with `None` to the keyboard on USB. The switch reports "searching"
    /// first, so the window starts over as after re-plugging.
    pub fn select_simulator(&self, id: Option<&str>) {
        let Some(keyboard_source) = &self.deps.keyboard_source else { return };
        let new = match id {
            Some(id) => match SimulatedModel::all().into_iter().find(|model| model.id() == id) {
                Some(model) => KeyboardSource::Simulated(model),
                None => return,
            },
            None => KeyboardSource::Hardware,
        };
        if new == self.state().source {
            return;
        }
        keyboard_source.select(new.clone());
        self.update(|state| state.source = new);
    }

    // Notices

    pub fn dismiss_notice(&self) {
        self.update(|state| state.notice = None);
    }

    /// Takes the notice away when it is still the one shown as `serial`.
    pub fn expire_notice(&self, serial: u64) {
        if self.state().notice_serial == serial && self.state().notice.is_some() {
            self.dismiss_notice();
        }
    }
}

impl fmt::Debug for EditorModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.state();
        f.debug_struct("EditorModel")
            .field("connection", &state.connection)
            .field("activity", &state.activity)
            .field("layer", &state.layer)
            .finish_non_exhaustive()
    }
}
