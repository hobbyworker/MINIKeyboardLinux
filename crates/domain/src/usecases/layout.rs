use std::fmt;
use std::rc::Rc;

use crate::capabilities::KeyboardCapabilities;
use crate::connection::ConnectedKeyboard;
use crate::grid::GridShape;
use crate::keyboards::{KeyboardIdentification, UsbIdentity};
use crate::layout::{KeyboardLayout, Rotation};
use crate::ports::LayoutProfileRepository;
use crate::support::{LayoutProfile, ResolvedLayout, Source};

/// The layout to draw for a newly identified keyboard. A keyboard that said how many keys and knobs it has
/// gets the layout the cross-platform app draws for that — its records follow it, so it is not the user's to
/// change, only how it is turned. A 884x keyboard that doesn't say gets the layout the user chose from the
/// app's, and the 1189:8890 — which that app doesn't know, and whose own tool draws no shape — the arrangement
/// the user gave it ([`GridShape`]). Until they choose: what the vendor tool would show.
#[derive(Clone)]
pub struct ResolveLayoutUseCase {
    profiles: Rc<dyn LayoutProfileRepository>,
}

impl ResolveLayoutUseCase {
    pub fn new(profiles: Rc<dyn LayoutProfileRepository>) -> Self {
        Self { profiles }
    }

    pub fn execute(&self, keyboard: &ConnectedKeyboard, identification: &KeyboardIdentification) -> ResolvedLayout {
        let saved = self.profiles.profile(keyboard.usb);
        let rotation = saved.as_ref().map(|profile| profile.rotation).unwrap_or_default();
        if let Some(identity) = &identification.identity
            && let Some(layout) =
                KeyboardLayout::named(KeyboardLayout::identified_id(identity, keyboard.usb.product_id))
        {
            return ResolvedLayout {
                layout: layout.clone(),
                rotation,
                source: Source::Identified,
                choices: vec![],
                grid: None,
            };
        }
        let capabilities = KeyboardCapabilities::new(identification.commands);
        if let Some(grid) = capabilities.grid_limits() {
            if let Some(saved) = &saved
                && let Some(shape) = GridShape::from_id(&saved.layout_id)
                && grid.contains(&shape)
            {
                return ResolvedLayout {
                    layout: shape.layout(),
                    rotation: saved.rotation,
                    source: Source::Chosen,
                    choices: vec![],
                    grid: Some(grid),
                };
            }
            return ResolvedLayout {
                layout: grid.assumed().layout(),
                rotation,
                source: Source::Assumed,
                choices: vec![],
                grid: Some(grid),
            };
        }
        let choices = Self::choices(&capabilities);
        if let Some(saved) = &saved
            && let Some(layout) = choices.iter().find(|layout| layout.id == saved.layout_id)
        {
            return ResolvedLayout {
                layout: layout.clone(),
                rotation: saved.rotation,
                source: Source::Chosen,
                choices: choices.clone(),
                grid: None,
            };
        }
        let layout = Self::assumed(&choices);
        ResolvedLayout { layout, rotation, source: Source::Assumed, choices, grid: None }
    }

    /// Every layout the keyboard can address, fewest controls first.
    pub fn choices(capabilities: &KeyboardCapabilities) -> Vec<KeyboardLayout> {
        let mut choices: Vec<KeyboardLayout> =
            KeyboardLayout::all().iter().filter(|layout| capabilities.addresses(layout)).cloned().collect();
        choices.sort_by(|a, b| (a.key_count(), a.knob_count(), &a.id).cmp(&(b.key_count(), b.knob_count(), &b.id)));
        choices
    }

    /// What each vendor tool draws for a keyboard it knows nothing of: the cross-platform app's 15 keys and
    /// three knobs, or where the keyboard can't have that, the layout with the most controls — the Windows
    /// tool's 12 keys and two knobs.
    fn assumed(choices: &[KeyboardLayout]) -> KeyboardLayout {
        choices
            .iter()
            .find(|layout| layout.id == KeyboardLayout::FALLBACK_ID)
            .or_else(|| {
                choices.iter().max_by(|a, b| a.controls.len().cmp(&b.controls.len()).then_with(|| b.id.cmp(&a.id)))
            })
            .or_else(|| KeyboardLayout::named(KeyboardLayout::FALLBACK_ID))
            .cloned()
            .expect("the cross-platform app's fallback layout is among its layouts")
    }
}

/// Keeps the user's choice of layout and rotation for a keyboard.
#[derive(Clone)]
pub struct ChooseLayoutUseCase {
    profiles: Rc<dyn LayoutProfileRepository>,
}

impl ChooseLayoutUseCase {
    pub fn new(profiles: Rc<dyn LayoutProfileRepository>) -> Self {
        Self { profiles }
    }

    /// The resolved layout with `layout` and `rotation` in place. The layout of a keyboard that said its shape
    /// stays as it is; only the rotation is kept.
    pub fn execute(
        &self,
        resolved: &ResolvedLayout,
        layout: &KeyboardLayout,
        rotation: Rotation,
        keyboard: UsbIdentity,
    ) -> ResolvedLayout {
        let allowed = match resolved.grid {
            Some(grid) => GridShape::from_id(&layout.id).is_some_and(|shape| grid.contains(&shape)),
            None => resolved.choices.contains(layout),
        };
        let chosen = if resolved.source == Source::Identified || !allowed { &resolved.layout } else { layout };
        self.profiles.save(LayoutProfile { layout_id: chosen.id.clone(), rotation }, keyboard);
        let source = if resolved.source == Source::Identified { Source::Identified } else { Source::Chosen };
        ResolvedLayout {
            layout: chosen.clone(),
            rotation,
            source,
            choices: resolved.choices.clone(),
            grid: resolved.grid,
        }
    }
}

impl fmt::Debug for ResolveLayoutUseCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResolveLayoutUseCase").finish_non_exhaustive()
    }
}

impl fmt::Debug for ChooseLayoutUseCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChooseLayoutUseCase").finish_non_exhaustive()
    }
}
