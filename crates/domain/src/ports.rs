//! What the outer layers provide: the keyboard, its connection, the device log and the user's choices.
//!
//! Everything here lives on the main thread, as the interface does. Work that waits — on the keyboard, or
//! for time to pass — is a [`LocalFuture`]: whoever drives the main loop polls it, and whoever implements the
//! port does the waiting elsewhere.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::actions::KeyAction;
use crate::configuration::KeyboardConfiguration;
use crate::connection::{ConnectionState, KeyboardDeviceError};
use crate::keyboards::{CommandSet, KeyboardIdentity, RecordFormat, UsbIdentity};
use crate::layout::{ControlId, KeyboardLayout};
use crate::lighting::LayerLighting;
use crate::support::{DeviceLogEntry, KeyboardSource, LayoutProfile};

/// A future polled on the main thread.
pub type LocalFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// Values that keep coming: the current one, then changes.
pub trait Updates<T> {
    /// The next value, or `None` when no more will come.
    fn next(&mut self) -> LocalFuture<'_, Option<T>>;
}

/// Finding the keyboard and keeping its configuration interface open.
pub trait KeyboardConnectionRepository {
    /// The current state, then every change.
    fn connection_states(&self) -> Box<dyn Updates<ConnectionState>>;

    /// Looks for the keyboard again — after access was given, or another app let go of it.
    fn retry(&self);
}

/// The actions one layer holds, by control.
pub type LayerActions = HashMap<ControlId, KeyAction>;

/// Talking to the open keyboard.
pub trait KeyboardDeviceRepository {
    /// Asks a 884x keyboard who it is (`FB FB FB`) once. `None` when no answer comes in time.
    fn ask_identity(&self) -> LocalFuture<'_, Result<Option<KeyboardIdentity>, KeyboardDeviceError>>;

    /// The actions one layer holds, for the controls of `layout` — standard and legacy formats only.
    fn read_actions<'a>(
        &'a self,
        layer: usize,
        format: RecordFormat,
        layout: &'a KeyboardLayout,
    ) -> LocalFuture<'a, Result<LayerActions, KeyboardDeviceError>>;

    /// One layer's lights — the standard format only.
    fn read_lighting(&self, layer: usize) -> LocalFuture<'_, Result<LayerLighting, KeyboardDeviceError>>;

    /// Writes every changed action and light of `configuration`, in the order the keyboard's vendor tool
    /// writes them, and has the keyboard keep them.
    fn write<'a>(
        &'a self,
        configuration: &'a KeyboardConfiguration,
        commands: CommandSet,
        layout: &'a KeyboardLayout,
    ) -> LocalFuture<'a, Result<(), KeyboardDeviceError>>;
}

/// The layouts users chose, by keyboard.
pub trait LayoutProfileRepository {
    fn profile(&self, keyboard: UsbIdentity) -> Option<LayoutProfile>;
    fn save(&self, profile: LayoutProfile, keyboard: UsbIdentity);
}

/// The reports exchanged with the keyboard, for the device log.
pub trait DeviceLogRepository {
    /// The current entries, then the whole list after changes.
    fn entries(&self) -> Box<dyn Updates<Vec<DeviceLogEntry>>>;

    fn clear(&self);
}

/// Choosing between the keyboard on USB and a simulated one.
pub trait KeyboardSourceRepository {
    fn current(&self) -> KeyboardSource;
    fn select(&self, source: KeyboardSource);
}
