use std::time::Duration;

use domain::{KeyMacro, KeyStroke, Modifiers};

use crate::tables::KeyUsage;

/// The pages of actions a key can be given.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EditorTab {
    Keys,
    Media,
    Mouse,
    Lighting,
    Delay,
}

impl EditorTab {
    pub const ALL: [Self; 5] = [Self::Keys, Self::Media, Self::Mouse, Self::Lighting, Self::Delay];
}

/// A transfer with the keyboard under way.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Activity {
    Identifying,
    Reading,
    Saving,
}

/// A switch to another keyboard the user asked for, waiting for them to agree to lose the changes not saved.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PendingSwitch {
    /// One of the keyboards plugged in, by its ID.
    Keyboard(String),
    /// A simulated keyboard by its ID, or `None` for the keyboard on USB.
    Simulator(Option<String>),
}

/// A message that comes and goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Notice {
    /// Saved; to a simulated keyboard when `simulated` — nothing went to a real one.
    Saved {
        simulated: bool,
    },
    ReadComplete,
    NothingToSave,
    IdentifyFailed,
    ReadFailed,
    /// The code of the failed write.
    WriteFailed(i32),
    KeyboardGone,
    Full(usize),
    SelectControlFirst,
    ModifiersOnlyAlone,
    NeedsColorKey(usize),
    Unsupported,
}

impl Notice {
    pub fn is_error(self) -> bool {
        !matches!(self, Self::Saved { .. } | Self::ReadComplete | Self::NothingToSave)
    }

    /// How long it stays.
    pub fn lifetime(self) -> Duration {
        match self {
            _ if self.is_error() => Duration::from_secs(6),
            Self::Saved { simulated: true } => Duration::from_secs(4),
            _ => Duration::from_millis(2500),
        }
    }
}

/// How actions read on key caps and in lists: the modifiers, then the key — "Ctrl+Shift+C".
#[derive(Clone, Copy, Debug)]
pub struct ActionText;

impl ActionText {
    /// "Ctrl+Alt+Shift+Super", each right-hand one after its left-hand one and marked "R".
    pub fn modifiers(modifiers: Modifiers) -> String {
        const NAMES: [(Modifiers, &str); 8] = [
            (Modifiers::CONTROL, "Ctrl"),
            (Modifiers::RIGHT_CONTROL, "RCtrl"),
            (Modifiers::ALT, "Alt"),
            (Modifiers::RIGHT_ALT, "RAlt"),
            (Modifiers::SHIFT, "Shift"),
            (Modifiers::RIGHT_SHIFT, "RShift"),
            (Modifiers::SUPER, "Super"),
            (Modifiers::RIGHT_SUPER, "RSuper"),
        ];
        NAMES
            .iter()
            .filter(|(modifier, _)| modifiers.contains(*modifier))
            .map(|(_, name)| *name)
            .collect::<Vec<_>>()
            .join("+")
    }

    /// "Super+C", or "Ctrl+Shift" for modifiers alone.
    pub fn stroke(stroke: &KeyStroke) -> String {
        let modifiers = Self::modifiers(stroke.modifiers);
        match (modifiers.is_empty(), stroke.is_modifiers_only()) {
            (_, true) => modifiers,
            (true, false) => Self::key(stroke.key),
            (false, false) => format!("{modifiers}+{}", Self::key(stroke.key)),
        }
    }

    /// The presses one after another.
    pub fn keys(macro_: &KeyMacro) -> String {
        macro_.strokes.iter().map(Self::stroke).collect::<Vec<_>>().join(" ")
    }

    /// A key's label, or its usage in hex when the app has no name for it.
    pub fn key(usage: u8) -> String {
        KeyUsage::named(usage).map_or_else(|| format!("0x{usage:02X}"), |key| key.label.to_owned())
    }
}
