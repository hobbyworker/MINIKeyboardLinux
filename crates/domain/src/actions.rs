use std::fmt;
use std::ops::BitOr;

/// The modifier keys of a key press, as the bits of a HID keyboard report (usages 0xE0–0xE7). The vendor
/// tools call Super "Win".
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Modifiers(u8);

impl Modifiers {
    pub const NONE: Self = Self(0);
    pub const CONTROL: Self = Self(0x01);
    pub const SHIFT: Self = Self(0x02);
    pub const ALT: Self = Self(0x04);
    pub const SUPER: Self = Self(0x08);
    pub const RIGHT_CONTROL: Self = Self(0x10);
    pub const RIGHT_SHIFT: Self = Self(0x20);
    pub const RIGHT_ALT: Self = Self(0x40);
    pub const RIGHT_SUPER: Self = Self(0x80);

    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Sets what is clear and clears what is set.
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    /// Each modifier on its own, lowest bit first.
    pub fn each() -> impl Iterator<Item = Self> {
        (0..8).map(|bit| Self(1 << bit))
    }

    /// The modifiers set here, lowest bit first.
    pub fn members(self) -> impl Iterator<Item = Self> {
        Self::each().filter(move |modifier| self.contains(*modifier))
    }

    pub const fn count(self) -> usize {
        self.0.count_ones() as usize
    }
}

impl BitOr for Modifiers {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}

impl fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Modifiers({:#04x})", self.0)
    }
}

/// One press: modifiers held while a key goes down. A key of 0 holds the modifiers alone, which turns the
/// keyboard key into a modifier key — so such a press is only ever a key's whole action.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct KeyStroke {
    pub modifiers: Modifiers,
    /// The key's usage on the HID keyboard page, or 0 for none.
    pub key: u8,
    /// Milliseconds to wait before this press. Only standard-format keyboards keep it.
    pub delay: u32,
}

impl KeyStroke {
    pub const fn new(modifiers: Modifiers, key: u8) -> Self {
        Self { modifiers, key, delay: 0 }
    }

    pub const fn key(key: u8) -> Self {
        Self::new(Modifiers::NONE, key)
    }

    pub const fn with_delay(self, delay: u32) -> Self {
        Self { delay, ..self }
    }

    pub const fn is_modifiers_only(&self) -> bool {
        self.key == 0
    }
}

/// Key presses one keyboard key sends, in order.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct KeyMacro {
    pub strokes: Vec<KeyStroke>,
    /// Milliseconds to wait, for the whole key. Only legacy-format keyboards keep it.
    pub delay: u32,
}

impl KeyMacro {
    pub fn new(strokes: Vec<KeyStroke>) -> Self {
        Self { strokes, delay: 0 }
    }

    pub fn with_delay(strokes: Vec<KeyStroke>, delay: u32) -> Self {
        Self { strokes, delay }
    }

    /// One modifiers-only press, or nothing at all, but never one among others.
    pub fn is_well_formed(&self) -> bool {
        self.strokes.len() <= 1 || !self.strokes.iter().any(KeyStroke::is_modifiers_only)
    }
}

/// What a keyboard key or knob action does.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyAction {
    /// Nothing: the key is cleared.
    Unassigned,
    Keys(KeyMacro),
    Media(MediaKey),
    Mouse(MouseAction),
}

/// A key on the HID consumer page — media and application keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MediaKey(pub u16);

impl MediaKey {
    pub const PLAY_PAUSE: Self = Self(0xCD);
    pub const STOP: Self = Self(0xB7);
    pub const PREVIOUS_TRACK: Self = Self(0xB6);
    pub const NEXT_TRACK: Self = Self(0xB5);
    pub const MUTE: Self = Self(0xE2);
    pub const VOLUME_UP: Self = Self(0xE9);
    pub const VOLUME_DOWN: Self = Self(0xEA);
    pub const BRIGHTNESS_UP: Self = Self(0x6F);
    pub const BRIGHTNESS_DOWN: Self = Self(0x70);
    pub const BASS_UP: Self = Self(0x152);
    pub const BASS_DOWN: Self = Self(0x153);
    pub const TREBLE_UP: Self = Self(0x154);
    pub const TREBLE_DOWN: Self = Self(0x155);
    pub const MEDIA_PLAYER: Self = Self(0x183);
    pub const EMAIL: Self = Self(0x18A);
    pub const CALCULATOR: Self = Self(0x192);
    pub const MY_COMPUTER: Self = Self(0x194);
    pub const WEB_HOME: Self = Self(0x223);
    pub const WEB_BACK: Self = Self(0x224);
    pub const WEB_FORWARD: Self = Self(0x225);
    pub const WEB_REFRESH: Self = Self(0x227);

    /// The consumer-page usage.
    pub const fn usage(self) -> u16 {
        self.0
    }

    /// The cross-platform app's media buttons, in its order.
    pub const ALL: [Self; 21] = [
        Self::PLAY_PAUSE,
        Self::STOP,
        Self::PREVIOUS_TRACK,
        Self::NEXT_TRACK,
        Self::MY_COMPUTER,
        Self::BRIGHTNESS_UP,
        Self::BRIGHTNESS_DOWN,
        Self::MEDIA_PLAYER,
        Self::MUTE,
        Self::VOLUME_UP,
        Self::VOLUME_DOWN,
        Self::CALCULATOR,
        Self::WEB_HOME,
        Self::EMAIL,
        Self::BASS_UP,
        Self::BASS_DOWN,
        Self::TREBLE_UP,
        Self::TREBLE_DOWN,
        Self::WEB_REFRESH,
        Self::WEB_FORWARD,
        Self::WEB_BACK,
    ];

    /// The Windows tool's six.
    pub const BASIC: [Self; 6] =
        [Self::PLAY_PAUSE, Self::PREVIOUS_TRACK, Self::NEXT_TRACK, Self::MUTE, Self::VOLUME_UP, Self::VOLUME_DOWN];
}

/// The mouse buttons and wheel moves both vendor tools offer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseAction {
    LeftClick,
    MiddleClick,
    RightClick,
    WheelUp,
    WheelDown,
    ControlWheelUp,
    ControlWheelDown,
    ShiftWheelUp,
    ShiftWheelDown,
    AltWheelUp,
    AltWheelDown,
}

impl MouseAction {
    pub const ALL: [Self; 11] = [
        Self::LeftClick,
        Self::MiddleClick,
        Self::RightClick,
        Self::WheelUp,
        Self::WheelDown,
        Self::ControlWheelUp,
        Self::ControlWheelDown,
        Self::ShiftWheelUp,
        Self::ShiftWheelDown,
        Self::AltWheelUp,
        Self::AltWheelDown,
    ];

    /// HID mouse button bits.
    pub const fn buttons(self) -> u8 {
        match self {
            Self::LeftClick => 0x01,
            Self::RightClick => 0x02,
            Self::MiddleClick => 0x04,
            _ => 0,
        }
    }

    /// One notch up (0x01) or down (0xFF), or 0.
    pub const fn wheel(self) -> u8 {
        match self {
            Self::WheelUp | Self::ControlWheelUp | Self::ShiftWheelUp | Self::AltWheelUp => 0x01,
            Self::WheelDown | Self::ControlWheelDown | Self::ShiftWheelDown | Self::AltWheelDown => 0xFF,
            _ => 0,
        }
    }

    /// The modifier held while the wheel turns.
    pub const fn modifiers(self) -> Modifiers {
        match self {
            Self::ControlWheelUp | Self::ControlWheelDown => Modifiers::CONTROL,
            Self::ShiftWheelUp | Self::ShiftWheelDown => Modifiers::SHIFT,
            Self::AltWheelUp | Self::AltWheelDown => Modifiers::ALT,
            _ => Modifiers::NONE,
        }
    }

    /// The action with these buttons, or this wheel move and modifier; `None` when there is none.
    pub fn from_parts(buttons: u8, wheel: u8, modifiers: Modifiers) -> Option<Self> {
        if buttons != 0 {
            Self::ALL.into_iter().find(|action| action.buttons() == buttons)
        } else if wheel != 0 {
            Self::ALL
                .into_iter()
                .find(|action| action.buttons() == 0 && action.wheel() == wheel && action.modifiers() == modifiers)
        } else {
            None
        }
    }
}
