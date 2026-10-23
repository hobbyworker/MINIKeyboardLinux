use std::ops::RangeInclusive;

/// A colour with one byte per channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl KeyColor {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// The colours keys 1–16 start with in the cross-platform app (`KeyBoard_KeyLed`), the same on every
    /// layer.
    pub const DEFAULT_KEY_COLORS: [Self; 16] = [
        Self::new(0xff, 0x00, 0x00),
        Self::new(0xff, 0x80, 0x30),
        Self::new(0xff, 0xff, 0x30),
        Self::new(0x00, 0xff, 0x00),
        Self::new(0x00, 0xff, 0xff),
        Self::new(0x00, 0x00, 0xff),
        Self::new(0x80, 0x00, 0x80),
        Self::new(0x8b, 0x00, 0x00),
        Self::new(0xff, 0xa5, 0x00),
        Self::new(0xff, 0xff, 0x96),
        Self::new(0x7d, 0xff, 0x00),
        Self::new(0x00, 0x8b, 0x8b),
        Self::new(0x00, 0x00, 0x8b),
        Self::new(0xff, 0x00, 0xff),
        Self::new(0xff, 0x66, 0x66),
        Self::new(0xff, 0xc8, 0x64),
    ];
}

/// The colours a legacy-format or unidentified 884x keyboard lights in, as the high nibble it takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LightColor {
    Red = 1,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
}

impl LightColor {
    pub const ALL: [Self; 7] =
        [Self::Red, Self::Orange, Self::Yellow, Self::Green, Self::Cyan, Self::Blue, Self::Purple];

    pub const fn code(self) -> u8 {
        self as u8
    }
}

/// What one layer's lights do, in the form the keyboard takes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LayerLighting {
    /// The 1189:8890: one of its modes ([`LightingSupport::Modes`]).
    Mode(u8),
    /// A legacy-format or unidentified 884x keyboard: a colour and a mode, 0–5.
    ColorMode { color: LightColor, mode: u8 },
    /// A standard-format keyboard: a mode, 0–5, and a colour for each of keys 1–16.
    PerKey { mode: u8, colors: Vec<KeyColor> },
}

impl LayerLighting {
    pub fn mode(&self) -> u8 {
        match self {
            Self::Mode(mode) | Self::ColorMode { mode, .. } | Self::PerKey { mode, .. } => *mode,
        }
    }
}

/// What a keyboard can do with its lights.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LightingSupport {
    /// The 1189:8890's three modes, one per layer.
    Modes(Vec<u8>),
    /// A colour and a mode per layer.
    ColorAndMode { modes: RangeInclusive<u8> },
    /// A mode per layer and a colour for each of the first keys.
    PerKeyColor { modes: RangeInclusive<u8>, keys: usize },
}

impl LightingSupport {
    /// The 1189:8890's modes 0–2, named for what they do on a 1189:8890 with six keys and a knob.
    pub const K8890_MODES: [u8; 3] = [0, 1, 2];
    /// Keys whose colour the standard format stores.
    pub const COLOR_KEY_COUNT: usize = 16;
}
