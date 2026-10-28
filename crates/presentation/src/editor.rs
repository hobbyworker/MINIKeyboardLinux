use std::cell::{Ref, RefCell};
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::task::Poll;

use domain::usecases::{
    AppendStrokeUseCase, ChooseLayoutUseCase, ClearUseCase, EditOutcome, EditTarget, IdentifyKeyboardUseCase,
    KeyboardSourceUseCase, ObserveAttachedKeyboardsUseCase, ObserveConnectionUseCase, ReadKeyboardUseCase,
    RemoveLastStrokeUseCase, ResolveLayoutUseCase, RetryConnectionUseCase, SaveCheck, SaveKeyboardUseCase, SaveOutcome,
    SelectKeyboardUseCase, SetActionUseCase, SetKeyDelayUseCase, SetLightingUseCase, SetPressDelayUseCase,
};
use domain::{
    AttachedKeyboards, ConnectedKeyboard, ConnectionState, ControlId, DelaySupport, GridShape, KeyAction, KeyColor,
    KeyStroke, KeyboardCapabilities, KeyboardConfiguration, KeyboardDeviceError, KeyboardIdentification,
    KeyboardLayout, KeyboardSource, LayerLighting, LightingSupport, Modifiers, ReadSupport, ResolvedLayout, Rotation,
    SimulatedModel,
};

use crate::choices::{ControlRef, LedColor, MediaChoice, ModifierKey, MouseChoice, Rgb};
use crate::screen_state::SimulatorChoice;
use crate::state::{Activity, EditorTab, Notice, PendingSwitch};
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
    /// The simulator's switch.
    pub keyboard_source: Option<KeyboardSourceUseCase>,
    /// The keyboards plugged in, and choosing which one is open.
    pub attached_keyboards: Option<(ObserveAttachedKeyboardsUseCase, SelectKeyboardUseCase)>,
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
    /// The keyboard on USB, or a simulated one.
    pub(crate) source: KeyboardSource,
    /// The keyboards on USB and the one open.
    pub(crate) attached: AttachedKeyboards,
    /// Counts up with each change of keyboard — to another, or to none — so work that ran across one can tell.
    pub(crate) connection_serial: u64,
    /// A switch the user asked for that would lose the changes not saved, until they agree or not.
    pub(crate) pending_switch: Option<PendingSwitch>,
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

    /// Changes not saved that the keyboard open could still take — a switch to another loses them.
    fn has_changes_to_lose(&self) -> bool {
        self.keyboard().is_some() && self.identification.is_some() && self.configuration.has_unsaved_changes()
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
                attached: AttachedKeyboards::default(),
                connection_serial: 0,
                pending_switch: None,
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

    /// Follows the keyboard, and the keyboards plugged in, until the model goes away; one that comes is identified
    /// at once.
    pub fn run(self: &Rc<Self>) -> impl Future<Output = ()> + 'static {
        let model: Weak<Self> = Rc::downgrade(self);
        if let Some(source) = &self.deps.keyboard_source {
            let current = source.current();
            self.update(|state| state.source = current);
        }
        let mut states = self.deps.observe_connection.execute();
        let connection = {
            let model = model.clone();
            async move {
                while let Some(state) = states.next().await {
                    let Some(model) = model.upgrade() else { return };
                    model.connection_changed(state).await;
                }
            }
        };
        let mut attached = self.deps.attached_keyboards.as_ref().map(|(observe, _)| observe.execute());
        let keyboards = async move {
            let Some(attached) = &mut attached else { return };
            while let Some(keyboards) = attached.next().await {
                let Some(model) = model.upgrade() else { return };
                model.update(|state| state.attached = keyboards);
            }
        };
        both(connection, keyboards)
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

    /// Opens or closes the layout sheet — it opens only with a keyboard to choose for.
    pub fn set_choosing_layout(&self, choosing: bool) {
        self.update(|state| {
            state.is_choosing_layout = choosing && state.resolved_layout.is_some() && state.keyboard().is_some();
        });
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
            state.connection_serial += 1;
            state.pending_switch = None;
            state.identification = None;
            state.selection = None;
            state.held = Modifiers::NONE;
            // A sheet for the keyboard that went goes with it; the next one asks again if it needs to.
            state.is_choosing_layout = false;
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

    /// Opens another of the keyboards plugged in — not while work with the open one is under way, and with changes
    /// not saved only once the user agrees ([`pending_switch`](Self::pending_switch)). The connection reports
    /// "searching" first, as after re-plugging.
    pub fn select_keyboard(&self, id: &str) {
        if self.deps.attached_keyboards.is_none() || self.state().attached.selected.as_deref() == Some(id) {
            return;
        }
        self.request_switch(PendingSwitch::Keyboard(id.to_owned()));
    }

    /// Whether another keyboard can be opened, or the simulator switched: not while work with the keyboard is under
    /// way.
    pub fn can_switch_keyboard(&self) -> bool {
        self.state().activity.is_none()
    }

    /// The switch waiting for the user to agree to lose the changes not saved.
    pub fn pending_switch(&self) -> Option<PendingSwitch> {
        self.state().pending_switch.clone()
    }

    /// Makes the switch the user was asked about, dropping the changes not saved.
    pub fn confirm_switch(&self) {
        let Some(switch) = self.state().pending_switch.clone() else { return };
        self.update(|state| state.pending_switch = None);
        if self.can_switch_keyboard() {
            self.switch(switch);
        }
    }

    /// Forgets the switch the user was asked about; the keyboard and its changes stay.
    pub fn cancel_switch(&self) {
        if self.state().pending_switch.is_some() {
            self.update(|state| state.pending_switch = None);
        }
    }

    /// A switch the user asked for: at once, or with changes the keyboard open could still take, once they agree.
    fn request_switch(&self, switch: PendingSwitch) {
        if !self.can_switch_keyboard() {
            return;
        }
        if self.state().has_changes_to_lose() {
            self.update(|state| state.pending_switch = Some(switch));
        } else {
            self.switch(switch);
        }
    }

    /// Opens the keyboard the switch names; the connection then reports it as it comes.
    fn switch(&self, switch: PendingSwitch) {
        match switch {
            PendingSwitch::Keyboard(id) => {
                if let Some((_, select)) = &self.deps.attached_keyboards {
                    select.execute(&id);
                }
            }
            PendingSwitch::Simulator(id) => {
                let (Some(keyboard_source), Some(new)) = (&self.deps.keyboard_source, source_named(id.as_deref()))
                else {
                    return;
                };
                keyboard_source.select(new.clone());
                self.update(|state| state.source = new);
            }
        }
    }

    /// Settles the keyboard's commands and layout, then reads it if it reads back on its own. A keyboard of a type the
    /// app doesn't know is then said to be untested — in place of what the read said, since it tells what to do.
    pub async fn identify(&self) {
        let (keyboard, serial) = {
            let state = self.state();
            match (state.keyboard(), state.identification, state.activity) {
                (Some(keyboard), None, None) => (keyboard.clone(), state.connection_serial),
                _ => return,
            }
        };
        self.update(|state| state.activity = Some(Activity::Identifying));
        let identified = self.deps.identify_keyboard.execute(&keyboard).await;
        if self.keyboard_changed_since(serial).await {
            return;
        }
        let identified = match identified {
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
        if let Some(identity) = identified.identity.filter(|identity| !identity.is_known_kind())
            && self.state().connection_serial == serial
        {
            self.update(|state| state.show(Notice::UntestedFirmware(identity.kind)));
        }
    }

    /// When work that began at `serial` ends: whether the keyboard changed meanwhile. If it did, the work's result is
    /// to be dropped — the activity ends here, telling the user the keyboard went — and the keyboard there now, passed
    /// over while the work was under way, is identified.
    async fn keyboard_changed_since(&self, serial: u64) -> bool {
        if self.state().connection_serial == serial {
            return false;
        }
        self.update(|state| {
            state.activity = None;
            state.show(Notice::KeyboardGone);
        });
        Box::pin(self.identify()).await;
        true
    }

    // Reading and saving

    pub async fn read(&self) {
        self.read_with(true).await;
    }

    async fn read_with(&self, announce: bool) {
        let (configuration, capabilities, layout, serial) = {
            let state = self.state();
            let (Some(capabilities), Some(resolved)) = (state.capabilities(), &state.resolved_layout) else { return };
            if !(state.is_ready() && state.reads_back()) {
                return;
            }
            (state.configuration.clone(), capabilities, resolved.layout.clone(), state.connection_serial)
        };
        self.update(|state| state.activity = Some(Activity::Reading));
        let outcome = self.deps.read_keyboard.execute(configuration, capabilities, &layout).await;
        if self.keyboard_changed_since(serial).await {
            return;
        }
        self.update(|state| {
            state.activity = None;
            state.configuration = outcome.configuration;
            if outcome.error.is_some() {
                state.show(Notice::ReadFailed);
            } else if !outcome.unread.is_empty() {
                state.show(Notice::ReadIncomplete);
            } else if announce {
                state.show(Notice::ReadComplete);
            }
        });
    }

    pub async fn save(&self) {
        let (configuration, capabilities, layout, serial) = {
            let state = self.state();
            let (Some(capabilities), Some(resolved)) = (state.capabilities(), &state.resolved_layout) else { return };
            if !state.is_ready() {
                return;
            }
            (state.configuration.clone(), capabilities, resolved.layout.clone(), state.connection_serial)
        };
        if !configuration.has_unsaved_changes() {
            self.update(|state| state.show(Notice::NothingToSave));
            return;
        }
        self.update(|state| state.activity = Some(Activity::Saving));
        let result = self.deps.save_keyboard.execute(&configuration, capabilities, &layout).await;
        if self.keyboard_changed_since(serial).await {
            return;
        }
        self.update(|state| {
            state.activity = None;
            match result {
                Ok(SaveOutcome::Saved(saved, check)) => {
                    state.configuration = saved;
                    let simulated = matches!(state.source, KeyboardSource::Simulated(_));
                    state.show(match check {
                        SaveCheck::NotChecked | SaveCheck::Matched => Notice::Saved { simulated },
                        SaveCheck::Differs { .. } => Notice::SavedDiffers,
                        SaveCheck::CouldNotCheck => Notice::SavedUnchecked,
                    });
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

    /// Whether there is a layout to turn or change — the connected keyboard's. With none connected the last one is
    /// still drawn, dimmed, but there is nothing to change it on.
    pub fn has_layout(&self) -> bool {
        let state = self.state();
        state.resolved_layout.is_some() && state.keyboard().is_some()
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

    /// Every keyboard the simulator plays, when it is wired in.
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

    /// Switches to a simulated keyboard, or with `None` to the keyboard on USB — not while work with the keyboard is
    /// under way, and with changes not saved only once the user agrees. The switch reports "searching" first, so the
    /// window starts over as after re-plugging.
    pub fn select_simulator(&self, id: Option<&str>) {
        if self.deps.keyboard_source.is_none() {
            return;
        }
        let Some(new) = source_named(id) else { return };
        if new == self.state().source {
            return;
        }
        self.request_switch(PendingSwitch::Simulator(id.map(str::to_owned)));
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

/// The simulated keyboard a simulator choice's ID names, or with `None` the keyboard on USB.
fn source_named(id: Option<&str>) -> Option<KeyboardSource> {
    match id {
        Some(id) => SimulatedModel::all().into_iter().find(|model| model.id() == id).map(KeyboardSource::Simulated),
        None => Some(KeyboardSource::Hardware),
    }
}

/// Runs two futures together until both are done.
fn both(
    first: impl Future<Output = ()> + 'static,
    second: impl Future<Output = ()> + 'static,
) -> impl Future<Output = ()> + 'static {
    let mut futures: [Option<Pin<Box<dyn Future<Output = ()>>>>; 2] = [Some(Box::pin(first)), Some(Box::pin(second))];
    std::future::poll_fn(move |context| {
        for slot in &mut futures {
            if slot.as_mut().is_some_and(|future| future.as_mut().poll(context).is_ready()) {
                *slot = None;
            }
        }
        if futures.iter().all(Option::is_none) { Poll::Ready(()) } else { Poll::Pending }
    })
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
