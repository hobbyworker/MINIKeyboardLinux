//! What the screens offer and show, in their own words. The domain's types stay behind the view model: views
//! hand these back, and the view model turns them into the domain's.

use domain::{ControlId, KeyAction, KeyColor, KnobPart, LightColor, MediaKey, Modifiers, MouseAction};

use crate::state::ActionText;

/// A modifier key, as the Keys page offers it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModifierKey {
    Control,
    Alt,
    Shift,
    Super,
    RightControl,
    RightAlt,
    RightShift,
    RightSuper,
}

impl ModifierKey {
    pub const ALL: [Self; 8] = [
        Self::Control,
        Self::Alt,
        Self::Shift,
        Self::Super,
        Self::RightControl,
        Self::RightAlt,
        Self::RightShift,
        Self::RightSuper,
    ];

    /// "Ctrl", or "RCtrl" for a right-hand one.
    pub fn label(self) -> &'static str {
        match self {
            Self::Control => "Ctrl",
            Self::Alt => "Alt",
            Self::Shift => "Shift",
            Self::Super => "Super",
            Self::RightControl => "RCtrl",
            Self::RightAlt => "RAlt",
            Self::RightShift => "RShift",
            Self::RightSuper => "RSuper",
        }
    }

    pub(crate) fn modifiers(self) -> Modifiers {
        match self {
            Self::Control => Modifiers::CONTROL,
            Self::Alt => Modifiers::ALT,
            Self::Shift => Modifiers::SHIFT,
            Self::Super => Modifiers::SUPER,
            Self::RightControl => Modifiers::RIGHT_CONTROL,
            Self::RightAlt => Modifiers::RIGHT_ALT,
            Self::RightShift => Modifiers::RIGHT_SHIFT,
            Self::RightSuper => Modifiers::RIGHT_SUPER,
        }
    }
}

/// A media key the app names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MediaChoice {
    PlayPause,
    Stop,
    PreviousTrack,
    NextTrack,
    Mute,
    VolumeUp,
    VolumeDown,
    BrightnessUp,
    BrightnessDown,
    BassUp,
    BassDown,
    TrebleUp,
    TrebleDown,
    MediaPlayer,
    Email,
    Calculator,
    MyComputer,
    WebHome,
    WebBack,
    WebForward,
    WebRefresh,
}

impl MediaChoice {
    pub const ALL: [Self; 21] = [
        Self::PlayPause,
        Self::Stop,
        Self::PreviousTrack,
        Self::NextTrack,
        Self::Mute,
        Self::VolumeUp,
        Self::VolumeDown,
        Self::BrightnessUp,
        Self::BrightnessDown,
        Self::BassUp,
        Self::BassDown,
        Self::TrebleUp,
        Self::TrebleDown,
        Self::MediaPlayer,
        Self::Email,
        Self::Calculator,
        Self::MyComputer,
        Self::WebHome,
        Self::WebBack,
        Self::WebForward,
        Self::WebRefresh,
    ];

    pub(crate) fn key(self) -> MediaKey {
        match self {
            Self::PlayPause => MediaKey::PLAY_PAUSE,
            Self::Stop => MediaKey::STOP,
            Self::PreviousTrack => MediaKey::PREVIOUS_TRACK,
            Self::NextTrack => MediaKey::NEXT_TRACK,
            Self::Mute => MediaKey::MUTE,
            Self::VolumeUp => MediaKey::VOLUME_UP,
            Self::VolumeDown => MediaKey::VOLUME_DOWN,
            Self::BrightnessUp => MediaKey::BRIGHTNESS_UP,
            Self::BrightnessDown => MediaKey::BRIGHTNESS_DOWN,
            Self::BassUp => MediaKey::BASS_UP,
            Self::BassDown => MediaKey::BASS_DOWN,
            Self::TrebleUp => MediaKey::TREBLE_UP,
            Self::TrebleDown => MediaKey::TREBLE_DOWN,
            Self::MediaPlayer => MediaKey::MEDIA_PLAYER,
            Self::Email => MediaKey::EMAIL,
            Self::Calculator => MediaKey::CALCULATOR,
            Self::MyComputer => MediaKey::MY_COMPUTER,
            Self::WebHome => MediaKey::WEB_HOME,
            Self::WebBack => MediaKey::WEB_BACK,
            Self::WebForward => MediaKey::WEB_FORWARD,
            Self::WebRefresh => MediaKey::WEB_REFRESH,
        }
    }

    pub(crate) fn of(key: MediaKey) -> Option<Self> {
        Self::ALL.into_iter().find(|choice| choice.key() == key)
    }
}

/// A mouse action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseChoice {
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

impl MouseChoice {
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

    pub(crate) fn action(self) -> MouseAction {
        match self {
            Self::LeftClick => MouseAction::LeftClick,
            Self::MiddleClick => MouseAction::MiddleClick,
            Self::RightClick => MouseAction::RightClick,
            Self::WheelUp => MouseAction::WheelUp,
            Self::WheelDown => MouseAction::WheelDown,
            Self::ControlWheelUp => MouseAction::ControlWheelUp,
            Self::ControlWheelDown => MouseAction::ControlWheelDown,
            Self::ShiftWheelUp => MouseAction::ShiftWheelUp,
            Self::ShiftWheelDown => MouseAction::ShiftWheelDown,
            Self::AltWheelUp => MouseAction::AltWheelUp,
            Self::AltWheelDown => MouseAction::AltWheelDown,
        }
    }

    pub(crate) fn of(action: MouseAction) -> Self {
        Self::ALL.into_iter().find(|choice| choice.action() == action).expect("every mouse action has a choice")
    }
}

/// A colour a layer of a legacy-format or unidentified keyboard lights in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LedColor {
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
}

impl LedColor {
    pub const ALL: [Self; 7] =
        [Self::Red, Self::Orange, Self::Yellow, Self::Green, Self::Cyan, Self::Blue, Self::Purple];

    pub(crate) fn color(self) -> LightColor {
        LightColor::ALL[self as usize]
    }

    pub(crate) fn of(color: LightColor) -> Self {
        Self::ALL[usize::from(color.code() - 1)]
    }
}

/// A key's colour on a keyboard that lights each key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub(crate) fn of(color: KeyColor) -> Self {
        Self::new(color.red, color.green, color.blue)
    }

    pub(crate) fn key_color(self) -> KeyColor {
        KeyColor::new(self.red, self.green, self.blue)
    }

    /// "#00FF00".
    pub fn hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

/// One action of a knob.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KnobAction {
    RotateLeft,
    Press,
    RotateRight,
    PressRotateLeft,
    PressRotateRight,
}

impl KnobAction {
    pub(crate) fn of(part: KnobPart) -> Self {
        match part {
            KnobPart::RotateLeft => Self::RotateLeft,
            KnobPart::Press => Self::Press,
            KnobPart::RotateRight => Self::RotateRight,
            KnobPart::PressRotateLeft => Self::PressRotateLeft,
            KnobPart::PressRotateRight => Self::PressRotateRight,
        }
    }

    pub(crate) fn part(self) -> KnobPart {
        match self {
            Self::RotateLeft => KnobPart::RotateLeft,
            Self::Press => KnobPart::Press,
            Self::RotateRight => KnobPart::RotateRight,
            Self::PressRotateLeft => KnobPart::PressRotateLeft,
            Self::PressRotateRight => KnobPart::PressRotateRight,
        }
    }
}

/// What a control is called: "Key 3", "Knob 1 · Press".
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ControlTitle {
    Key(u8),
    Knob(u8, KnobAction),
}

/// A key or knob action on the keyboard, as views hand it back to the view model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ControlRef(pub(crate) ControlId);

impl ControlRef {
    pub fn key(number: u8) -> Self {
        Self(ControlId::Key(number))
    }

    pub fn knob(number: u8, action: KnobAction) -> Self {
        Self(ControlId::Knob(number, action.part()))
    }

    pub fn title(self) -> ControlTitle {
        match self.0 {
            ControlId::Key(number) => ControlTitle::Key(number),
            ControlId::Knob(number, part) => ControlTitle::Knob(number, KnobAction::of(part)),
        }
    }
}

/// What a key does, as a key cap and the inspector show it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActionCaption {
    /// The app doesn't know: the keyboard can't be read, or nobody has set it.
    Unknown,
    Nothing,
    /// "Super+C ↵".
    Presses(String),
    Media(MediaChoice),
    /// A media key the app has no name for, by its consumer usage.
    OtherMedia(u16),
    Mouse(MouseChoice),
}

impl ActionCaption {
    pub fn is_assigned(&self) -> bool {
        !matches!(self, Self::Unknown | Self::Nothing)
    }

    pub(crate) fn of(action: Option<&KeyAction>) -> Self {
        match action {
            None => Self::Unknown,
            Some(KeyAction::Unassigned) => Self::Nothing,
            Some(KeyAction::Keys(macro_)) if macro_.strokes.is_empty() => Self::Nothing,
            Some(KeyAction::Keys(macro_)) => Self::Presses(ActionText::keys(macro_)),
            Some(KeyAction::Media(key)) => MediaChoice::of(*key).map_or(Self::OtherMedia(key.usage()), Self::Media),
            Some(KeyAction::Mouse(action)) => Self::Mouse(MouseChoice::of(*action)),
        }
    }
}
