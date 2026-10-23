use crate::actions::{KeyAction, KeyMacro, KeyStroke};
use crate::capabilities::{DelaySupport, KeyboardCapabilities};
use crate::configuration::KeyboardConfiguration;
use crate::layout::{ControlId, KeyboardLayout};
use crate::lighting::{KeyColor, LayerLighting, LightColor, LightingSupport};

/// What came of an edit of the app's copy of the configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EditOutcome {
    Applied,
    /// No key or knob is selected.
    NeedsSelection,
    /// The keyboard has nothing for this, so nothing changed.
    Unsupported,
    /// The key holds no more presses; the value is how many it holds.
    Full(usize),
    /// Modifiers alone make a key a modifier key, so they can't follow or precede other presses.
    ModifiersOnlyAlone,
    /// Colours are kept for the first keys only; the value is how many.
    NeedsColorKey(usize),
}

/// Where an edit applies: the selected control on the shown layer of a keyboard with these capabilities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EditTarget {
    pub capabilities: KeyboardCapabilities,
    pub layer: usize,
    pub control: Option<ControlId>,
}

/// Adds a press to the selected key. A key that did something else starts over with it; a key that holds
/// modifiers alone takes them into the press.
#[derive(Clone, Copy, Debug, Default)]
pub struct AppendStrokeUseCase;

impl AppendStrokeUseCase {
    pub fn execute(
        &self,
        stroke: KeyStroke,
        configuration: &mut KeyboardConfiguration,
        target: &EditTarget,
    ) -> EditOutcome {
        let Some(control) = target.control else { return EditOutcome::NeedsSelection };
        if !stroke.is_modifiers_only() && !target.capabilities.takes(stroke.key) {
            return EditOutcome::Unsupported;
        }
        let mut macro_ = match configuration.action(control, target.layer) {
            Some(KeyAction::Keys(current)) => current.clone(),
            _ => KeyMacro::default(),
        };
        let mut stroke = stroke;
        if let [only] = macro_.strokes.as_slice()
            && only.is_modifiers_only()
        {
            stroke.modifiers.insert(only.modifiers);
            stroke.delay = only.delay;
            macro_.strokes.clear();
        }
        macro_.strokes.push(stroke);
        if !macro_.is_well_formed() {
            return EditOutcome::ModifiersOnlyAlone;
        }
        if !target.capabilities.fits(&macro_) {
            return EditOutcome::Full(target.capabilities.press_limit());
        }
        configuration.set(KeyAction::Keys(macro_), control, target.layer);
        EditOutcome::Applied
    }
}

/// Takes the last press off the selected key; the key is cleared when none is left.
#[derive(Clone, Copy, Debug, Default)]
pub struct RemoveLastStrokeUseCase;

impl RemoveLastStrokeUseCase {
    pub fn execute(&self, configuration: &mut KeyboardConfiguration, target: &EditTarget) -> EditOutcome {
        let Some(control) = target.control else { return EditOutcome::NeedsSelection };
        let Some(KeyAction::Keys(macro_)) = configuration.action(control, target.layer) else {
            return EditOutcome::Unsupported;
        };
        if macro_.strokes.is_empty() {
            return EditOutcome::Unsupported;
        }
        let mut macro_ = macro_.clone();
        macro_.strokes.pop();
        configuration.set(keys_or_nothing(macro_), control, target.layer);
        EditOutcome::Applied
    }
}

/// Replaces the selected key's action with a media key or a mouse action.
#[derive(Clone, Copy, Debug, Default)]
pub struct SetActionUseCase;

impl SetActionUseCase {
    pub fn execute(
        &self,
        action: KeyAction,
        configuration: &mut KeyboardConfiguration,
        target: &EditTarget,
    ) -> EditOutcome {
        let Some(control) = target.control else { return EditOutcome::NeedsSelection };
        match &action {
            KeyAction::Media(key) => {
                if !target.capabilities.media_keys().contains(key) {
                    return EditOutcome::Unsupported;
                }
            }
            KeyAction::Keys(macro_) => {
                if !target.capabilities.takes_keys_of(macro_) {
                    return EditOutcome::Unsupported;
                }
                if !macro_.is_well_formed() {
                    return EditOutcome::ModifiersOnlyAlone;
                }
                if !target.capabilities.fits(macro_) {
                    return EditOutcome::Full(target.capabilities.press_limit());
                }
            }
            KeyAction::Unassigned | KeyAction::Mouse(_) => {}
        }
        configuration.set(action, control, target.layer);
        EditOutcome::Applied
    }
}

/// Sets the wait before one press of the selected key (standard format).
#[derive(Clone, Copy, Debug, Default)]
pub struct SetPressDelayUseCase;

impl SetPressDelayUseCase {
    pub fn execute(
        &self,
        press: usize,
        milliseconds: u32,
        configuration: &mut KeyboardConfiguration,
        target: &EditTarget,
    ) -> EditOutcome {
        let DelaySupport::PerPress(range) = target.capabilities.delays() else { return EditOutcome::Unsupported };
        let Some(control) = target.control else { return EditOutcome::NeedsSelection };
        let Some(KeyAction::Keys(macro_)) = configuration.action(control, target.layer) else {
            return EditOutcome::Unsupported;
        };
        if press >= macro_.strokes.len() {
            return EditOutcome::Unsupported;
        }
        let mut macro_ = macro_.clone();
        macro_.strokes[press].delay = milliseconds.clamp(*range.start(), *range.end());
        configuration.set(KeyAction::Keys(macro_), control, target.layer);
        EditOutcome::Applied
    }
}

/// Sets the one wait the selected key keeps (legacy format). Its presses stay; a key that held a media key or
/// a mouse action has none to keep.
#[derive(Clone, Copy, Debug, Default)]
pub struct SetKeyDelayUseCase;

impl SetKeyDelayUseCase {
    pub fn execute(
        &self,
        milliseconds: u32,
        configuration: &mut KeyboardConfiguration,
        target: &EditTarget,
    ) -> EditOutcome {
        let DelaySupport::PerKey(range) = target.capabilities.delays() else { return EditOutcome::Unsupported };
        let Some(control) = target.control else { return EditOutcome::NeedsSelection };
        let mut macro_ = match configuration.action(control, target.layer) {
            Some(KeyAction::Keys(current)) => current.clone(),
            Some(KeyAction::Media(_) | KeyAction::Mouse(_)) => return EditOutcome::Unsupported,
            Some(KeyAction::Unassigned) | None => KeyMacro::default(),
        };
        macro_.delay = milliseconds.clamp(*range.start(), *range.end());
        configuration.set(keys_or_nothing(macro_), control, target.layer);
        EditOutcome::Applied
    }
}

/// Clears the selected key, or every control of the layout on the shown layer.
#[derive(Clone, Copy, Debug, Default)]
pub struct ClearUseCase;

impl ClearUseCase {
    pub fn execute(&self, configuration: &mut KeyboardConfiguration, target: &EditTarget) -> EditOutcome {
        let Some(control) = target.control else { return EditOutcome::NeedsSelection };
        configuration.set(KeyAction::Unassigned, control, target.layer);
        EditOutcome::Applied
    }

    pub fn execute_all(
        &self,
        layout: &KeyboardLayout,
        configuration: &mut KeyboardConfiguration,
        layer: usize,
    ) -> EditOutcome {
        for control in &layout.controls {
            configuration.set(KeyAction::Unassigned, *control, layer);
        }
        EditOutcome::Applied
    }
}

/// Sets the shown layer's lights.
#[derive(Clone, Copy, Debug, Default)]
pub struct SetLightingUseCase;

impl SetLightingUseCase {
    /// The 8890's mode, or the mode of a layer of the other formats — keeping its colours.
    pub fn set_mode(&self, mode: u8, configuration: &mut KeyboardConfiguration, target: &EditTarget) -> EditOutcome {
        let current = configuration.lighting(target.layer);
        let lighting = match target.capabilities.lighting() {
            LightingSupport::Modes(modes) => {
                if !modes.contains(&mode) {
                    return EditOutcome::Unsupported;
                }
                LayerLighting::Mode(mode)
            }
            LightingSupport::ColorAndMode { modes } => {
                if !modes.contains(&mode) {
                    return EditOutcome::Unsupported;
                }
                let color = match current {
                    Some(LayerLighting::ColorMode { color, .. }) => *color,
                    _ => LightColor::Cyan,
                };
                LayerLighting::ColorMode { color, mode }
            }
            LightingSupport::PerKeyColor { modes, .. } => {
                if !modes.contains(&mode) {
                    return EditOutcome::Unsupported;
                }
                let colors = match current {
                    Some(LayerLighting::PerKey { colors, .. }) => colors.clone(),
                    _ => KeyColor::DEFAULT_KEY_COLORS.to_vec(),
                };
                LayerLighting::PerKey { mode, colors }
            }
        };
        configuration.set_lighting(lighting, target.layer);
        EditOutcome::Applied
    }

    /// The colour of a layer that takes one (legacy format and unidentified keyboards).
    pub fn set_color(
        &self,
        color: LightColor,
        configuration: &mut KeyboardConfiguration,
        target: &EditTarget,
    ) -> EditOutcome {
        let LightingSupport::ColorAndMode { modes } = target.capabilities.lighting() else {
            return EditOutcome::Unsupported;
        };
        let mode = match configuration.lighting(target.layer) {
            Some(LayerLighting::ColorMode { mode, .. }) => *mode,
            _ => *modes.start(),
        };
        configuration.set_lighting(LayerLighting::ColorMode { color, mode }, target.layer);
        EditOutcome::Applied
    }

    /// The colour of the selected key (standard format), which must be one of the first keys.
    pub fn set_key_color(
        &self,
        key_color: KeyColor,
        configuration: &mut KeyboardConfiguration,
        target: &EditTarget,
    ) -> EditOutcome {
        let LightingSupport::PerKeyColor { modes, keys } = target.capabilities.lighting() else {
            return EditOutcome::Unsupported;
        };
        let Some(control) = target.control else { return EditOutcome::NeedsSelection };
        let Some(index) = Self::color_index(control, keys) else { return EditOutcome::NeedsColorKey(keys) };
        let (mode, mut colors) = match configuration.lighting(target.layer) {
            Some(LayerLighting::PerKey { mode, colors }) => (*mode, colors.clone()),
            _ => (*modes.start(), KeyColor::DEFAULT_KEY_COLORS.to_vec()),
        };
        colors[index] = key_color;
        configuration.set_lighting(LayerLighting::PerKey { mode, colors }, target.layer);
        EditOutcome::Applied
    }

    /// Where a key's colour is kept: keys 1–16 have colours 1–16. (The app colours records 1–16, and a key is
    /// stored in the record of its number.)
    pub fn color_index(control: ControlId, keys: usize) -> Option<usize> {
        match control {
            ControlId::Key(number) if (1..=keys).contains(&usize::from(number)) => Some(usize::from(number) - 1),
            _ => None,
        }
    }
}

/// The key's presses and wait, or nothing when it has neither.
fn keys_or_nothing(macro_: KeyMacro) -> KeyAction {
    if macro_.strokes.is_empty() && macro_.delay == 0 { KeyAction::Unassigned } else { KeyAction::Keys(macro_) }
}
