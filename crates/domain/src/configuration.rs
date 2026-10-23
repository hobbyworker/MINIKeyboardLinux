use std::collections::{BTreeSet, HashMap};

use crate::actions::KeyAction;
use crate::layout::ControlId;
use crate::lighting::LayerLighting;

/// One layer of what the app holds for a keyboard.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Layer {
    actions: HashMap<ControlId, KeyAction>,
    lighting: Option<LayerLighting>,
    changed_controls: BTreeSet<ControlId>,
    lighting_changed: bool,
}

impl Layer {
    pub fn lighting_changed(&self) -> bool {
        self.lighting_changed
    }
}

/// What the app holds for a keyboard: each layer's actions and lights, and what changed since they were read
/// or saved. What the app doesn't know — a keyboard that can't be read, a key nobody has set — is missing
/// rather than guessed, and is never written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardConfiguration {
    layers: Vec<Layer>,
}

impl KeyboardConfiguration {
    pub fn new(layer_count: usize) -> Self {
        Self { layers: vec![Layer::default(); layer_count.max(1)] }
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    /// The action of `control`, or `None` when the app doesn't know it.
    pub fn action(&self, control: ControlId, layer: usize) -> Option<&KeyAction> {
        self.layers[layer].actions.get(&control)
    }

    pub fn lighting(&self, layer: usize) -> Option<&LayerLighting> {
        self.layers[layer].lighting.as_ref()
    }

    pub fn is_changed(&self, control: ControlId, layer: usize) -> bool {
        self.layers[layer].changed_controls.contains(&control)
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.layers.iter().any(|layer| !layer.changed_controls.is_empty() || layer.lighting_changed)
    }

    /// Sets an action to be written on the next save.
    pub fn set(&mut self, action: KeyAction, control: ControlId, layer: usize) {
        let layer = &mut self.layers[layer];
        layer.actions.insert(control, action);
        layer.changed_controls.insert(control);
    }

    /// Sets a layer's lights to be written on the next save.
    pub fn set_lighting(&mut self, lighting: LayerLighting, layer: usize) {
        let layer = &mut self.layers[layer];
        layer.lighting = Some(lighting);
        layer.lighting_changed = true;
    }

    /// What the keyboard said it holds. It replaces the app's values, saved or not, as the vendor app does
    /// when it reads.
    pub fn apply_read(&mut self, actions: HashMap<ControlId, KeyAction>, layer: usize) {
        let layer = &mut self.layers[layer];
        for (control, action) in actions {
            layer.changed_controls.remove(&control);
            layer.actions.insert(control, action);
        }
    }

    pub fn apply_read_lighting(&mut self, lighting: LayerLighting, layer: usize) {
        let layer = &mut self.layers[layer];
        layer.lighting = Some(lighting);
        layer.lighting_changed = false;
    }

    /// Everything was written.
    pub fn mark_saved(&mut self) {
        for layer in &mut self.layers {
            layer.changed_controls.clear();
            layer.lighting_changed = false;
        }
    }
}
