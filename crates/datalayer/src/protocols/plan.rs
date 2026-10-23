//! The payloads that write a configuration's changes, in the order the keyboard's vendor tool writes them.

use std::time::Duration;

use domain::{
    CommandSet, ControlId, KeyAction, KeyboardConfiguration, KeyboardDeviceError, KeyboardLayout, LayerLighting,
    RecordFormat,
};

use super::{k884x, k8890, numbers, records};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Step {
    pub payload: Vec<u8>,
    pub pause_after: Duration,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WritePlan {
    pub steps: Vec<Step>,
}

impl WritePlan {
    pub fn new(
        configuration: &KeyboardConfiguration,
        commands: CommandSet,
        layout: &KeyboardLayout,
    ) -> Result<Self, KeyboardDeviceError> {
        let mut plan = Self::default();
        let layers = 0..configuration.layer_count();
        match commands {
            CommandSet::Packets { report_id } => {
                // One key at a time, each saved on its own (`Download_Click`).
                for layer in layers {
                    for (control, action) in changes(configuration, layer, layout) {
                        let Some(number) = numbers::k8890_number(control) else { continue };
                        plan.append(k8890::download(number, action, layer, report_id)?);
                    }
                    if configuration.layers()[layer].lighting_changed()
                        && let Some(LayerLighting::Mode(mode)) = configuration.lighting(layer)
                    {
                        plan.append(k8890::download_lighting(*mode, layer, report_id));
                    }
                }
            }
            CommandSet::Records(RecordFormat::Standard) => {
                // Every changed record, then — when a light changed — every layer's lights, then one commit if a
                // record was written (`Widget::HID_write`).
                let mut wrote_records = false;
                for layer in layers.clone() {
                    for (control, action) in changes(configuration, layer, layout) {
                        plan.append(vec![records::standard(action, record(control, layout)?, layer)?]);
                        wrote_records = true;
                    }
                }
                if configuration.layers().iter().any(|layer| layer.lighting_changed()) {
                    for layer in layers {
                        if let Some(LayerLighting::PerKey { mode, colors }) = configuration.lighting(layer) {
                            plan.append(vec![k884x::lighting(layer, *mode, colors)]);
                        }
                    }
                }
                if wrote_records {
                    plan.commit();
                }
            }
            CommandSet::Records(RecordFormat::Legacy) => {
                // Each layer's changed records, its lights in record 0, and a commit.
                for layer in layers {
                    let mut wrote_records = false;
                    for (control, action) in changes(configuration, layer, layout) {
                        plan.append(vec![records::encode(
                            action,
                            record(control, layout)?,
                            layer,
                            RecordFormat::Legacy,
                        )?]);
                        wrote_records = true;
                    }
                    if configuration.layers()[layer].lighting_changed()
                        && let Some(LayerLighting::ColorMode { color, mode }) = configuration.lighting(layer)
                    {
                        plan.append(vec![records::legacy_lighting(*color, *mode, layer, records::LEGACY_LENGTH)]);
                        wrote_records = true;
                    }
                    if wrote_records {
                        plan.commit();
                    }
                }
            }
            CommandSet::Records(RecordFormat::Unidentified) => {
                // One record at a time, each saved on its own.
                for layer in layers {
                    for (control, action) in changes(configuration, layer, layout) {
                        plan.append(vec![records::unidentified(action, record(control, layout)?, layer)?]);
                        plan.append(k884x::unidentified_save());
                    }
                    if configuration.layers()[layer].lighting_changed()
                        && let Some(LayerLighting::ColorMode { color, mode }) = configuration.lighting(layer)
                    {
                        plan.append(k884x::unidentified_lighting(*color, *mode, layer));
                    }
                }
            }
        }
        Ok(plan)
    }

    fn append(&mut self, payloads: Vec<Vec<u8>>) {
        self.steps.extend(payloads.into_iter().map(|payload| Step { payload, pause_after: Duration::ZERO }));
    }

    fn commit(&mut self) {
        self.steps.push(Step { payload: k884x::COMMIT.to_vec(), pause_after: k884x::COMMIT_PAUSE });
    }
}

/// Changed controls of one layer that the layout has, in control order.
fn changes<'a>(
    configuration: &'a KeyboardConfiguration,
    layer: usize,
    layout: &KeyboardLayout,
) -> Vec<(ControlId, &'a KeyAction)> {
    let mut controls: Vec<ControlId> =
        layout.controls.iter().copied().filter(|control| configuration.is_changed(*control, layer)).collect();
    controls.sort();
    controls
        .into_iter()
        .filter_map(|control| configuration.action(control, layer).map(|action| (control, action)))
        .collect()
}

/// The record a 884x keyboard keeps `control` in.
fn record(control: ControlId, layout: &KeyboardLayout) -> Result<u8, KeyboardDeviceError> {
    numbers::record(control, layout).ok_or(KeyboardDeviceError::Unsupported)
}
