//! What the user does with a keyboard, as rules over the ports.

mod device;
mod editing;
mod layout;

pub use device::{
    DeviceLogUseCase, IdentifyKeyboardUseCase, KeyboardSourceUseCase, ObserveConnectionUseCase, ReadKeyboardUseCase,
    ReadOutcome, RetryConnectionUseCase, SaveKeyboardUseCase, SaveOutcome,
};
pub use editing::{
    AppendStrokeUseCase, ClearUseCase, EditOutcome, EditTarget, RemoveLastStrokeUseCase, SetActionUseCase,
    SetKeyDelayUseCase, SetLightingUseCase, SetPressDelayUseCase,
};
pub use layout::{ChooseLayoutUseCase, ResolveLayoutUseCase};
