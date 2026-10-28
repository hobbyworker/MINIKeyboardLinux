use std::ops::RangeInclusive;

use crate::actions::{KeyMacro, MediaKey};
use crate::grid::GridLimits;
use crate::keyboards::{CommandSet, RecordFormat};
use crate::layout::{ControlId, KeyboardLayout, KnobPart};
use crate::lighting::LightingSupport;

/// What a connected keyboard can hold and do, from the commands it takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardCapabilities {
    pub commands: CommandSet,
}

/// How a keyboard keeps waits.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DelaySupport {
    /// No waits.
    Unavailable,
    /// A wait before each press, in milliseconds.
    PerPress(RangeInclusive<u32>),
    /// One wait for the whole key, in milliseconds.
    PerKey(RangeInclusive<u32>),
}

/// Whether the keyboard tells what it holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReadSupport {
    Never,
    /// Asked as soon as it is connected, as the cross-platform app does, and again whenever the user asks.
    OnConnect,
}

impl KeyboardCapabilities {
    pub const fn new(commands: CommandSet) -> Self {
        Self { commands }
    }

    /// The 8890 with report ID 0 takes no layer command; every other keyboard has three layers.
    pub fn layer_count(&self) -> usize {
        if self.commands == (CommandSet::Packets { report_id: 0 }) { 1 } else { 3 }
    }

    /// How many presses one key holds — counted as [`presses`](Self::presses) counts them.
    pub fn press_limit(&self) -> usize {
        match self.commands {
            CommandSet::Packets { .. } => 5,
            CommandSet::Records(RecordFormat::Standard) => 18,
            CommandSet::Records(RecordFormat::Legacy) => 20,
            CommandSet::Records(RecordFormat::Unidentified) => 18,
        }
    }

    /// Whether each modifier takes a place of its own, as in the standard format.
    pub fn modifiers_take_places(&self) -> bool {
        self.commands == CommandSet::Records(RecordFormat::Standard)
    }

    /// The places `macro_` takes in a key.
    pub fn presses(&self, macro_: &KeyMacro) -> usize {
        if !self.modifiers_take_places() {
            return macro_.strokes.len();
        }
        macro_.strokes.iter().map(|stroke| stroke.modifiers.count() + usize::from(!stroke.is_modifiers_only())).sum()
    }

    pub fn fits(&self, macro_: &KeyMacro) -> bool {
        macro_.is_well_formed() && self.presses(macro_) <= self.press_limit()
    }

    /// Whether the keyboard is given `key`, a usage on the HID keyboard page — the keys its vendor tool
    /// offers. The Windows tool for the 8890 has 95: A (0x04) to Keypad . (0x63) and Menu (0x65), without
    /// the two non-US keys (0x32, 0x64) or Keypad Enter (0x58). The cross-platform app has every usage from
    /// 0x04 to 0x65, and F13–F24 (0x68–0x73).
    pub fn takes(&self, key: u8) -> bool {
        match self.commands {
            CommandSet::Packets { .. } => (0x04..=0x63).contains(&key) && key != 0x32 && key != 0x58 || key == 0x65,
            CommandSet::Records(_) => (0x04..=0x65).contains(&key) || (0x68..=0x73).contains(&key),
        }
    }

    /// Whether the keyboard is given every key `macro_` presses.
    pub fn takes_keys_of(&self, macro_: &KeyMacro) -> bool {
        macro_.strokes.iter().all(|stroke| stroke.is_modifiers_only() || self.takes(stroke.key))
    }

    pub fn delays(&self) -> DelaySupport {
        match self.commands {
            CommandSet::Records(RecordFormat::Standard) => DelaySupport::PerPress(0..=60000),
            CommandSet::Records(RecordFormat::Legacy) => DelaySupport::PerKey(0..=6000),
            CommandSet::Packets { .. } | CommandSet::Records(RecordFormat::Unidentified) => DelaySupport::Unavailable,
        }
    }

    pub fn media_keys(&self) -> &'static [MediaKey] {
        match self.commands {
            // Report ID 3 takes any consumer usage, but the Windows tool offers its six on every report ID.
            CommandSet::Packets { .. } => &MediaKey::BASIC,
            CommandSet::Records(_) => &MediaKey::ALL,
        }
    }

    pub fn lighting(&self) -> LightingSupport {
        match self.commands {
            CommandSet::Packets { .. } => LightingSupport::Modes(LightingSupport::K8890_MODES.to_vec()),
            CommandSet::Records(RecordFormat::Standard) => {
                LightingSupport::PerKeyColor { modes: 0..=5, keys: LightingSupport::COLOR_KEY_COUNT }
            }
            CommandSet::Records(RecordFormat::Legacy | RecordFormat::Unidentified) => {
                LightingSupport::ColorAndMode { modes: 0..=5 }
            }
        }
    }

    pub fn reading(&self) -> ReadSupport {
        match self.commands {
            CommandSet::Records(RecordFormat::Standard | RecordFormat::Legacy) => ReadSupport::OnConnect,
            CommandSet::Packets { .. } | CommandSet::Records(RecordFormat::Unidentified) => ReadSupport::Never,
        }
    }

    /// Keyboards no vendor tool draws are arranged by the user within these ([`GridShape`](crate::GridShape));
    /// `None` for keyboards that take the cross-platform app's layouts.
    pub fn grid_limits(&self) -> Option<GridLimits> {
        match self.commands {
            CommandSet::Packets { .. } => Some(GridLimits::K8890),
            CommandSet::Records(_) => None,
        }
    }

    /// Whether the keyboard can take `control`. The 1189:8890 has twelve keys and two knobs that turn and
    /// press; a 884x keyboard stores every control of its layout.
    pub fn can_address(&self, control: ControlId) -> bool {
        if !matches!(self.commands, CommandSet::Packets { .. }) {
            return true;
        }
        match control {
            ControlId::Key(number) => (1..=12).contains(&number),
            ControlId::Knob(number, part) => (1..=2).contains(&number) && part <= KnobPart::RotateRight,
        }
    }

    pub fn addresses(&self, layout: &KeyboardLayout) -> bool {
        layout.controls.iter().all(|control| self.can_address(*control))
    }
}
