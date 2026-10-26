//! The configurator's rules: the keyboards it knows, the actions their keys and knobs take, what each
//! keyboard can hold, and the use cases that edit, read and save them. The standard library only — the
//! ports in [`ports`] are what the outer layers implement.

mod actions;
mod capabilities;
mod configuration;
mod connection;
mod grid;
mod keyboards;
mod layout;
mod layouts;
mod lighting;
pub mod ports;
mod support;
pub mod usecases;

pub use actions::{KeyAction, KeyMacro, KeyStroke, MediaKey, Modifiers, MouseAction};
pub use capabilities::{DelaySupport, KeyboardCapabilities, ReadSupport};
pub use configuration::{KeyboardConfiguration, Layer};
pub use connection::{
    AttachedKeyboard, AttachedKeyboards, Channel, ConnectedKeyboard, ConnectionState, DeviceDetails, DeviceInterface,
    KeyboardDeviceError, UsagePair,
};
pub use grid::{GridLimits, GridShape};
pub use keyboards::{CommandSet, KeyboardFamily, KeyboardIdentification, KeyboardIdentity, RecordFormat, UsbIdentity};
pub use layout::{ControlId, KeyboardLayout, KnobPart, Rotation};
pub use lighting::{KeyColor, LayerLighting, LightColor, LightingSupport};
pub use support::{
    DeviceLogEntry, DeviceLogEvent, KeyboardSource, LayoutProfile, LogContent, ResolvedLayout, SimulatedKind,
    SimulatedModel, Source,
};
