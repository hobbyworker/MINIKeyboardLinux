//! What each part of the window draws, in the presentation's types.

use domain::{
    CommandSet, ControlId, DelaySupport, GridShape, KeyAction, KeyboardLayout, LayerLighting, LightingSupport,
    Rotation, Source,
};

use crate::choices::{ActionCaption, ControlRef, KnobAction, LedColor, MediaChoice, ModifierKey, MouseChoice, Rgb};
use crate::drawing::LayoutGeometry;
use crate::editor::EditorModel;
use crate::screen_state::{
    BindingState, BoardCap, BoardFace, ConnectionStatus, ControlDrawing, ControlFace, DelayPage, KeyArrangement,
    KeyColorSelection, KeyGroup, KeyGroupKind, KeyboardDrawing, KeyboardInfo, LayoutChoice, LayoutSheetMode,
    LayoutSheetState, LightingPage, PressDelay, SelectionState, ShapeDrawing,
};
use crate::state::{ActionText, EditorTab};
use crate::tables::{KeyUsage, places};

impl EditorModel {
    // Header

    pub fn status(&self) -> ConnectionStatus {
        let state = self.state();
        match &state.connection {
            domain::ConnectionState::Searching => ConnectionStatus::Searching,
            domain::ConnectionState::NeedsPermission => ConnectionStatus::NeedsPermission,
            domain::ConnectionState::OpenFailed { code } => ConnectionStatus::OpenFailed { code: *code },
            domain::ConnectionState::Connected(keyboard) => {
                let identity = state.identification.and_then(|identification| identification.identity);
                ConnectionStatus::Connected {
                    usb: keyboard.usb.to_string(),
                    keys: identity.map(|identity| identity.key_count),
                    knobs: identity.map(|identity| identity.knob_count),
                    is_identified: state.identification.is_some(),
                    is_simulated: state.source.is_simulated(),
                }
            }
        }
    }

    pub fn keyboard_info(&self) -> Option<KeyboardInfo> {
        let state = self.state();
        state.keyboard().map(|keyboard| KeyboardInfo::of(keyboard, state.identification.as_ref()))
    }

    // The keyboard

    pub fn drawing(&self) -> Option<KeyboardDrawing> {
        let state = self.state();
        let resolved = state.resolved_layout.as_ref()?;
        let geometry = LayoutGeometry::of(&resolved.layout)?.rotated(resolved.rotation);
        let controls = resolved
            .layout
            .controls
            .iter()
            .filter_map(|control| {
                let rect = *geometry.rects.get(control)?;
                let face = match control {
                    ControlId::Key(number) => ControlFace::Key(*number),
                    ControlId::Knob(_, part) => ControlFace::Knob(KnobAction::of(*part)),
                };
                Some(ControlDrawing {
                    control: ControlRef(*control),
                    face,
                    rect,
                    caption: ActionCaption::of(state.action(*control)),
                    is_selected: state.selection == Some(*control),
                    is_changed: state.configuration.is_changed(*control, state.layer),
                    led_color: (state.tab == EditorTab::Lighting)
                        .then(|| state.key_color(*control).map(Rgb::of))
                        .flatten(),
                })
            })
            .collect();
        Some(KeyboardDrawing { frame: geometry.frame, bounds: geometry.bounds(), controls })
    }

    // Selection

    pub fn selected_control(&self) -> Option<ControlRef> {
        self.state().selection.map(ControlRef)
    }

    pub fn selection_state(&self) -> Option<SelectionState> {
        let state = self.state();
        let selection = state.selection?;
        let binding = match state.action(selection) {
            None => BindingState::Unknown,
            Some(KeyAction::Unassigned) => BindingState::Nothing,
            Some(KeyAction::Keys(macro_)) => BindingState::Presses {
                strokes: macro_.strokes.iter().map(ActionText::stroke).collect(),
                key_delay: macro_.delay,
                used: state.capabilities().map_or(macro_.strokes.len(), |capabilities| capabilities.presses(macro_)),
                limit: state.capabilities().map_or(0, |capabilities| capabilities.press_limit()),
            },
            Some(KeyAction::Media(key)) => {
                MediaChoice::of(*key).map_or(BindingState::OtherMedia(key.usage()), BindingState::Media)
            }
            Some(KeyAction::Mouse(mouse)) => BindingState::Mouse(MouseChoice::of(*mouse)),
        };
        let layer_number = (state.layer_count() > 1).then_some(state.layer + 1);
        Some(SelectionState { title: ControlRef(selection).title(), layer_number, binding })
    }

    /// What the selected key does, for marking the page's matching button.
    pub fn selected_caption(&self) -> Option<ActionCaption> {
        let state = self.state();
        state.selection.map(|control| ActionCaption::of(state.action(control)))
    }

    // Keys page

    pub fn held_modifiers(&self) -> Vec<ModifierKey> {
        let held = self.state().held;
        ModifierKey::ALL.into_iter().filter(|modifier| held.contains(modifier.modifiers())).collect()
    }

    /// "Ctrl+Super", or `None` when none is held.
    pub fn held_modifier_text(&self) -> Option<String> {
        let held = self.state().held;
        (!held.is_empty()).then(|| ActionText::modifiers(held))
    }

    pub fn press_limit(&self) -> usize {
        self.state().capabilities().map_or(0, |capabilities| capabilities.press_limit())
    }

    /// Whether each modifier takes a place of its own in a key, as on the standard format.
    pub fn modifiers_count_as_presses(&self) -> bool {
        self.state().capabilities().is_some_and(|capabilities| capabilities.modifiers_take_places())
    }

    /// How the keyboard's vendor tool lays out its keys — the Windows tool's groups for the 8890, the
    /// cross-platform app's full-size keyboard for the rest — until the user picks one.
    pub fn vendor_key_arrangement(&self) -> KeyArrangement {
        match self.state().capabilities().map(|capabilities| capabilities.commands) {
            Some(CommandSet::Packets { .. }) => KeyArrangement::Alphabetical,
            _ => KeyArrangement::Qwerty,
        }
    }

    /// Every key, before a keyboard is identified.
    fn takes(&self, key: &KeyUsage) -> bool {
        self.state().capabilities().is_none_or(|capabilities| capabilities.takes(key.usage))
    }

    /// The full-size keyboard. A key the keyboard isn't given keeps its place but is off.
    pub fn key_board(&self) -> Vec<BoardCap> {
        places()
            .iter()
            .map(|place| BoardCap {
                face: place.face,
                row: place.row,
                column: place.column,
                width: place.width,
                is_enabled: match &place.face {
                    BoardFace::Key(key) => self.takes(key),
                    BoardFace::Modifier(_) => true,
                },
            })
            .collect()
    }

    /// The keys a full-size keyboard doesn't have — F13–F24 and the two non-US keys — that the keyboard is given.
    pub fn more_keys(&self) -> Vec<KeyUsage> {
        KeyUsage::EXTRA_FUNCTION_KEYS
            .iter()
            .chain(&KeyUsage::INTERNATIONAL)
            .filter(|key| self.takes(key))
            .copied()
            .collect()
    }

    /// The alphabetical arrangement: groups of the keys the keyboard is given, leaving out empty ones.
    pub fn key_groups(&self) -> Vec<KeyGroup> {
        KeyGroupKind::ALL
            .into_iter()
            .filter_map(|kind| {
                let keys: Vec<KeyUsage> = kind.keys().into_iter().filter(|key| self.takes(key)).collect();
                (!keys.is_empty()).then_some(KeyGroup { kind, keys })
            })
            .collect()
    }

    /// Digits and symbols with their shifted characters, sent as Shift and the key.
    pub fn shift_symbols(&self) -> Vec<KeyUsage> {
        KeyUsage::DIGITS
            .iter()
            .chain(&KeyUsage::SYMBOLS)
            .filter(|key| key.shifted.is_some() && self.takes(key))
            .copied()
            .collect()
    }

    // Media page

    pub fn media_choices(&self) -> Vec<MediaChoice> {
        let keys =
            self.state().capabilities().map_or(&domain::MediaKey::ALL[..], |capabilities| capabilities.media_keys());
        keys.iter().filter_map(|key| MediaChoice::of(*key)).collect()
    }

    // LED page

    pub fn lighting_page(&self) -> Option<LightingPage> {
        let state = self.state();
        let current = state.lighting();
        Some(match state.capabilities()?.lighting() {
            LightingSupport::Modes(modes) => {
                let active = match current {
                    Some(LayerLighting::Mode(mode)) => Some(*mode),
                    _ => None,
                };
                LightingPage::Modes { modes, active }
            }
            LightingSupport::ColorAndMode { modes } => {
                let (active_color, active_mode) = match current {
                    Some(LayerLighting::ColorMode { color, mode }) => (Some(LedColor::of(*color)), Some(*mode)),
                    _ => (None, None),
                };
                LightingPage::ColorAndMode {
                    colors: LedColor::ALL.to_vec(),
                    modes: modes.collect(),
                    active_color,
                    active_mode,
                }
            }
            LightingSupport::PerKeyColor { modes, keys } => {
                let selected = state.selection.and_then(|control| {
                    state
                        .key_color(control)
                        .map(|color| KeyColorSelection { title: ControlRef(control).title(), color: Rgb::of(color) })
                });
                LightingPage::PerKeyColor {
                    modes: modes.collect(),
                    active_mode: current.map(LayerLighting::mode),
                    keys,
                    selected,
                }
            }
        })
    }

    // Delay page

    pub fn delay_page(&self) -> DelayPage {
        let state = self.state();
        match state.capabilities().map(|capabilities| capabilities.delays()) {
            Some(DelaySupport::PerPress(range)) => {
                let presses = match state.selected_action() {
                    Some(KeyAction::Keys(macro_)) => macro_
                        .strokes
                        .iter()
                        .enumerate()
                        .map(|(index, stroke)| PressDelay {
                            index,
                            text: ActionText::stroke(stroke),
                            delay: stroke.delay,
                        })
                        .collect(),
                    _ => vec![],
                };
                DelayPage::PerPress { range, presses }
            }
            Some(DelaySupport::PerKey(range)) => {
                let value = state.selection.map(|_| match state.selected_action() {
                    Some(KeyAction::Keys(macro_)) => macro_.delay,
                    _ => 0,
                });
                DelayPage::PerKey { range, value }
            }
            Some(DelaySupport::Unavailable) | None => DelayPage::Unavailable,
        }
    }

    // Layout sheet

    pub fn layout_sheet(&self) -> Option<LayoutSheetState> {
        let state = self.state();
        let (resolved, keyboard) = (state.resolved_layout.as_ref()?, state.keyboard()?);
        let mode = if resolved.source == Source::Identified {
            LayoutSheetMode::Fixed
        } else if let Some(grid) = resolved.grid {
            let shape = GridShape::from_id(&resolved.layout.id).unwrap_or_else(|| grid.assumed());
            LayoutSheetMode::Arrangement {
                keys: shape.keys(),
                columns: shape.columns(),
                knobs: shape.knobs(),
                max_keys: grid.max_keys,
                max_knobs: grid.max_knobs,
            }
        } else {
            let choices = resolved
                .choices
                .iter()
                .filter_map(|layout| {
                    LayoutGeometry::of(layout).map(|geometry| LayoutChoice {
                        id: layout.id.clone(),
                        keys: layout.key_count(),
                        knobs: layout.knob_count(),
                        drawing: ShapeDrawing::of(&geometry),
                    })
                })
                .collect();
            LayoutSheetMode::Catalog { choices, selected_id: resolved.layout.id.clone() }
        };
        let identity = state.identification.and_then(|identification| identification.identity);
        Some(LayoutSheetState {
            usb: keyboard.usb.to_string(),
            reported_keys: identity.map(|identity| identity.key_count),
            reported_knobs: identity.map(|identity| identity.knob_count),
            mode,
            quarter_turns: resolved.rotation.quarter_turns(),
            is_required: resolved.needs_choice(),
        })
    }

    /// One of the app's layouts by its name, or with `None` the keyboard's own, drawn small and turned.
    pub fn preview_drawing(&self, layout_id: Option<&str>, quarter_turns: i64) -> Option<ShapeDrawing> {
        let layout = match layout_id.and_then(KeyboardLayout::named) {
            Some(layout) => layout.clone(),
            None => self.state().resolved_layout.as_ref()?.layout.clone(),
        };
        let geometry = LayoutGeometry::of(&layout)?;
        Some(ShapeDrawing::of(&geometry.rotated(Rotation::from_quarter_turns(quarter_turns))))
    }

    /// An arrangement drawn small and turned; `None` when it is not one a keyboard can have.
    pub fn preview_arrangement(&self, keys: u8, columns: u8, knobs: u8, quarter_turns: i64) -> Option<ShapeDrawing> {
        let shape = GridShape::new(keys, columns, knobs)?;
        Some(ShapeDrawing::of(
            &LayoutGeometry::arrangement(&shape).rotated(Rotation::from_quarter_turns(quarter_turns)),
        ))
    }
}
