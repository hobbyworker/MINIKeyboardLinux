//! What the windows show and what the user does in them, in the presentation's own types: the views take
//! these and hand them back, and never see the domain's.
//!
//! The view models live on the main thread. Their work that waits is `async`: the interface spawns it on its
//! main loop. They say when they changed through [`subscribe`](EditorModel::subscribe); the interface reads
//! what it draws again.

mod access;
mod choices;
mod device_log;
mod drawing;
mod editor;
mod packet_text;
mod screen;
mod screen_state;
mod state;
mod tables;
#[cfg(test)]
mod tests;

pub use access::{UDEV_RULES_PATH, udev_install_commands, udev_rules};
pub use choices::{
    ActionCaption, ControlRef, ControlTitle, KnobAction, LedColor, MediaChoice, ModifierKey, MouseChoice, Rgb,
};
pub use device_log::{DeviceLogModel, LogDirection, LogEvent, LogLine};
pub use drawing::LayoutRect;
pub use editor::{EditorDependencies, EditorModel};
pub use screen_state::{
    BindingState, BoardCap, BoardFace, ChannelPath, ConnectionStatus, ControlDrawing, ControlFace, DelayPage, Firmware,
    InfoChannel, InfoCommands, InfoInterface, InterfaceRole, KeyArrangement, KeyColorSelection, KeyGroup, KeyGroupKind,
    KeyboardDrawing, KeyboardInfo, LayoutChoice, LayoutSheetMode, LayoutSheetState, LightingPage, PressDelay,
    SelectionState, ShapeDrawing, ShapeItem, SimulatorChoice, SimulatorGroup, SimulatorKind,
};
pub use state::{ActionText, Activity, EditorTab, Notice};
pub use tables::KeyUsage;
