//! The tables the screens draw from: keys and how to show them, the full-size keyboard, the colour swatches.

mod full_keyboard;
mod key_usages;
mod palette;

pub(crate) use full_keyboard::{FULL_KEYBOARD_WIDTH, places};
pub use key_usages::KeyUsage;
