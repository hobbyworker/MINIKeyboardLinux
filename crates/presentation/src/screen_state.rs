//! What each part of the window draws, in the presentation's own types.

use std::ops::RangeInclusive;

use domain::{
    Channel, CommandSet, ConnectedKeyboard, DeviceDetails, DeviceInterface, KeyboardIdentification, RecordFormat,
    SimulatedKind, SimulatedModel,
};

use crate::choices::{
    ActionCaption, ControlRef, ControlTitle, KnobAction, LedColor, MediaChoice, ModifierKey, MouseChoice, Rgb,
};
use crate::drawing::{LayoutGeometry, LayoutRect};
use crate::tables::KeyUsage;

/// The keyboard as the main window draws it: its body and every control, turned the way it sits.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardDrawing {
    pub frame: LayoutRect,
    pub bounds: LayoutRect,
    pub controls: Vec<ControlDrawing>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ControlFace {
    Key(u8),
    Knob(KnobAction),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ControlDrawing {
    pub control: ControlRef,
    pub face: ControlFace,
    pub rect: LayoutRect,
    pub caption: ActionCaption,
    pub is_selected: bool,
    pub is_changed: bool,
    /// The key's colour, on the LED page of a keyboard that lights each key.
    pub led_color: Option<Rgb>,
}

/// A keyboard's shape drawn small, for choosing one: keys, and each knob as one body.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeDrawing {
    pub frame: LayoutRect,
    pub bounds: LayoutRect,
    pub items: Vec<ShapeItem>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShapeItem {
    pub rect: LayoutRect,
    pub is_knob: bool,
}

impl ShapeDrawing {
    pub(crate) fn of(geometry: &LayoutGeometry) -> Self {
        let mut keys: Vec<(u8, LayoutRect)> = geometry
            .rects
            .iter()
            .filter_map(|(control, rect)| match control {
                domain::ControlId::Key(number) => Some((*number, *rect)),
                domain::ControlId::Knob(..) => None,
            })
            .collect();
        keys.sort_by_key(|(number, _)| *number);
        let items = keys
            .into_iter()
            .map(|(_, rect)| ShapeItem { rect, is_knob: false })
            .chain(geometry.knob_outlines().into_iter().map(|(_, rect)| ShapeItem { rect, is_knob: true }))
            .collect();
        Self { frame: geometry.frame, bounds: geometry.bounds(), items }
    }
}

/// The header's account of the keyboard.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConnectionStatus {
    Searching,
    NeedsPermission,
    OpenFailed {
        code: i32,
    },
    /// `keys` and `knobs` are what the keyboard said, when it said.
    Connected {
        usb: String,
        keys: Option<u8>,
        knobs: Option<u8>,
        is_identified: bool,
        is_simulated: bool,
    },
}

/// The inspector's account of the selected control.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelectionState {
    pub title: ControlTitle,
    /// Shown when the keyboard has layers.
    pub layer_number: Option<usize>,
    pub binding: BindingState,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BindingState {
    Unknown,
    Nothing,
    /// Each press as text, the wait for the whole key, and how many places the presses take of the limit.
    Presses {
        strokes: Vec<String>,
        key_delay: u32,
        used: usize,
        limit: usize,
    },
    Media(MediaChoice),
    OtherMedia(u16),
    Mouse(MouseChoice),
}

/// How the Keys page lays out the keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyArrangement {
    /// As on a full-size keyboard — the cross-platform app's page.
    Qwerty,
    /// In groups, the letters in alphabetical order — the Windows tool's page.
    Alphabetical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoardFace {
    Key(KeyUsage),
    /// Held for the next key, as in the modifier row of the other arrangement.
    Modifier(ModifierKey),
}

/// One cap of the full-size keyboard, placed in key widths.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardCap {
    pub face: BoardFace,
    pub row: usize,
    /// From the left edge.
    pub column: f64,
    pub width: f64,
    /// Off for a key the keyboard isn't given; it stays in its place.
    pub is_enabled: bool,
}

impl BoardCap {
    /// A row's width, in key widths.
    pub const ROW_WIDTH: f64 = crate::tables::FULL_KEYBOARD_WIDTH;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyGroupKind {
    Letters,
    Numbers,
    Editing,
    Symbols,
    FunctionKeys,
    Navigation,
    Numpad,
}

impl KeyGroupKind {
    pub const ALL: [Self; 7] = [
        Self::Letters,
        Self::Numbers,
        Self::Editing,
        Self::Symbols,
        Self::FunctionKeys,
        Self::Navigation,
        Self::Numpad,
    ];

    pub(crate) fn keys(self) -> Vec<KeyUsage> {
        match self {
            Self::Letters => KeyUsage::LETTERS.to_vec(),
            Self::Numbers => KeyUsage::DIGITS.to_vec(),
            Self::Editing => KeyUsage::EDITING.to_vec(),
            Self::Symbols => [&KeyUsage::SYMBOLS[..], &KeyUsage::INTERNATIONAL].concat(),
            Self::FunctionKeys => [&KeyUsage::FUNCTION_KEYS[..], &KeyUsage::EXTRA_FUNCTION_KEYS].concat(),
            Self::Navigation => [&KeyUsage::SYSTEM[..], &KeyUsage::NAVIGATION].concat(),
            Self::Numpad => KeyUsage::KEYPAD.to_vec(),
        }
    }
}

/// A group of keys in the alphabetical arrangement, with only the keys the keyboard is given.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyGroup {
    pub kind: KeyGroupKind,
    pub keys: Vec<KeyUsage>,
}

/// The LED page, in the form the keyboard takes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LightingPage {
    /// The 1189:8890's modes.
    Modes {
        modes: Vec<u8>,
        active: Option<u8>,
    },
    ColorAndMode {
        colors: Vec<LedColor>,
        modes: Vec<u8>,
        active_color: Option<LedColor>,
        active_mode: Option<u8>,
    },
    PerKeyColor {
        modes: Vec<u8>,
        active_mode: Option<u8>,
        keys: usize,
        selected: Option<KeyColorSelection>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyColorSelection {
    pub title: ControlTitle,
    pub color: Rgb,
}

/// The Delay page.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DelayPage {
    Unavailable,
    /// The selected key's presses, each with its wait; empty when it has none.
    PerPress {
        range: RangeInclusive<u32>,
        presses: Vec<PressDelay>,
    },
    /// The selected key's one wait; `None` when no key is selected.
    PerKey {
        range: RangeInclusive<u32>,
        value: Option<u32>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PressDelay {
    pub index: usize,
    pub text: String,
    pub delay: u32,
}

/// The layout sheet.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutSheetState {
    pub usb: String,
    /// What the keyboard said of its keys and knobs, when it said.
    pub reported_keys: Option<u8>,
    pub reported_knobs: Option<u8>,
    pub mode: LayoutSheetMode,
    /// Quarter turns clockwise, 0–3.
    pub quarter_turns: u8,
    /// The first time: no Cancel.
    pub is_required: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LayoutSheetMode {
    /// The keyboard said its shape: it only turns.
    Fixed,
    /// One of the cross-platform app's layouts.
    Catalog { choices: Vec<LayoutChoice>, selected_id: String },
    /// Keys, keys to a row and knobs, within the limits.
    Arrangement { keys: u8, columns: u8, knobs: u8, max_keys: u8, max_knobs: u8 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutChoice {
    pub id: String,
    pub keys: usize,
    pub knobs: usize,
    pub drawing: ShapeDrawing,
}

/// What is known about the keyboard plugged in — from what the system says of the USB device and, on
/// keyboards that answer, from the answer. Nothing is guessed: what the keyboard doesn't say is `None`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardInfo {
    pub usb_id: String,
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub firmware: Firmware,
    /// "USB 1.1 · 12 Mb/s".
    pub usb: Option<String>,
    /// mA.
    pub power: Option<u32>,
    pub interfaces: Vec<InfoInterface>,
    pub channel: InfoChannel,
    pub commands: InfoCommands,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Firmware {
    /// What a 884x keyboard answered: the type, keys and knobs.
    Reported {
        kind: u8,
        keys: u8,
        knobs: u8,
    },
    /// `bcdDevice`, "1.02".
    Release(String),
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InfoInterface {
    pub number: u8,
    pub roles: Vec<InterfaceRole>,
    /// The interface the app writes to.
    pub is_configuration: bool,
}

/// The interface the app writes to, and how.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InfoChannel {
    pub interface: u8,
    pub path: ChannelPath,
    pub report_id: u8,
    /// Bytes, the report ID included.
    pub report_length: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChannelPath {
    /// Straight to the interface's interrupt OUT endpoint.
    Usb {
        out_endpoint: u8,
    },
    /// Through the hidraw device the system made of the interface.
    Hid,
    Simulated,
}

/// The commands the keyboard takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InfoCommands {
    /// The 1189:8890's packets.
    Packets {
        report_id: u8,
    },
    /// A 884x keyboard's records: on the standard firmware, the legacy one, or one that doesn't answer.
    Standard,
    Legacy,
    Unidentified,
    /// Not known yet.
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InterfaceRole {
    Keyboard,
    Mouse,
    Media,
    Configuration,
    Other,
}

impl KeyboardInfo {
    pub(crate) fn of(keyboard: &ConnectedKeyboard, identification: Option<&KeyboardIdentification>) -> Self {
        let details = keyboard.details.as_ref();
        let firmware = if let Some(identity) = identification.and_then(|identification| identification.identity) {
            Firmware::Reported { kind: identity.kind, keys: identity.key_count, knobs: identity.knob_count }
        } else if let Some(release) = details.map(|details| details.device_release).filter(|release| *release != 0) {
            Firmware::Release(format!("{:X}.{:02X}", release >> 8, release & 0xFF))
        } else {
            Firmware::Unknown
        };
        let interfaces = details
            .map(|details| details.interfaces.as_slice())
            .unwrap_or_default()
            .iter()
            .map(|interface| InfoInterface {
                number: interface.number,
                roles: Self::roles(interface, keyboard.interface_number),
                is_configuration: interface.number == keyboard.interface_number,
            })
            .collect();
        let path = match keyboard.channel {
            Channel::Usb { out_endpoint } => ChannelPath::Usb { out_endpoint },
            Channel::Hid => ChannelPath::Hid,
            Channel::Simulated => ChannelPath::Simulated,
        };
        let commands = match identification.map(|identification| identification.commands) {
            Some(CommandSet::Packets { report_id }) => InfoCommands::Packets { report_id },
            Some(CommandSet::Records(RecordFormat::Standard)) => InfoCommands::Standard,
            Some(CommandSet::Records(RecordFormat::Legacy)) => InfoCommands::Legacy,
            Some(CommandSet::Records(RecordFormat::Unidentified)) => InfoCommands::Unidentified,
            None => InfoCommands::Unknown,
        };
        Self {
            usb_id: keyboard.usb.to_string(),
            name: details.and_then(|details| details.product_name.clone()),
            manufacturer: details.and_then(|details| details.manufacturer.clone()),
            serial_number: details.and_then(|details| details.serial_number.clone()),
            firmware,
            usb: details.map(Self::usb_text),
            power: details.and_then(|details| details.max_power),
            interfaces,
            channel: InfoChannel {
                interface: keyboard.interface_number,
                path,
                report_id: keyboard.report_id,
                report_length: keyboard.report_length,
            },
            commands,
        }
    }

    /// What an interface is for, by the usages of its HID device, or by its descriptor when the system made
    /// no HID device of it.
    pub(crate) fn roles(interface: &DeviceInterface, configuration: u8) -> Vec<InterfaceRole> {
        let mut roles = vec![];
        for pair in &interface.usages {
            let role = match (pair.page, pair.usage) {
                (0x01, 0x06) => InterfaceRole::Keyboard,
                (0x01, 0x01 | 0x02) => InterfaceRole::Mouse,
                (0x0C, _) => InterfaceRole::Media,
                (0xFF00.., _) => InterfaceRole::Configuration,
                _ => InterfaceRole::Other,
            };
            if !roles.contains(&role) {
                roles.push(role);
            }
        }
        if roles.is_empty() && interface.number == configuration {
            roles.push(InterfaceRole::Configuration);
        }
        if roles.is_empty() {
            roles.push(match (interface.class, interface.subclass, interface.protocol) {
                (3, 1, 1) => InterfaceRole::Keyboard,
                (3, 1, 2) => InterfaceRole::Mouse,
                _ => InterfaceRole::Other,
            });
        }
        roles
    }

    pub(crate) fn usb_text(details: &DeviceDetails) -> String {
        let version = details.usb_version;
        let (major, minor, sub) = (version >> 8 & 0xFF, version >> 4 & 0x0F, version & 0x0F);
        let mut text = format!("USB {major:X}.{minor:X}");
        if sub != 0 {
            text += &format!(".{sub:X}");
        }
        if let Some(speed) = details.speed {
            text += &match speed {
                1_000_000_000.. => format!(" · {} Gb/s", speed / 1_000_000_000),
                1_000_000.. => format!(" · {} Mb/s", speed / 1_000_000),
                _ => format!(" · {} kb/s", speed / 1000),
            };
        }
        text
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimulatorGroup {
    K8890,
    Standard,
    Legacy,
    Unidentified,
}

impl SimulatorGroup {
    pub const ALL: [Self; 4] = [Self::K8890, Self::Standard, Self::Legacy, Self::Unidentified];
}

/// What the simulated keyboard is; the menu names it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SimulatorKind {
    K8890 {
        report_id: u8,
    },
    /// It answers with these, on this USB ID.
    Identified {
        keys: u8,
        knobs: u8,
        kind: u8,
        usb: String,
    },
    /// It doesn't answer.
    Unidentified {
        usb: String,
    },
}

/// One of the keyboards plugged in, to choose which the app opens.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardChoice {
    pub id: String,
    /// Its USB ID, numbered by port when several have the same one.
    pub title: String,
    pub is_selected: bool,
}

/// A keyboard the simulator can play.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SimulatorChoice {
    pub id: String,
    pub group: SimulatorGroup,
    pub kind: SimulatorKind,
}

impl SimulatorChoice {
    /// Whether everyone's menu has it, not only a debug build's: the 8890 as it ships (report ID 3), each 884x
    /// layout on its standard records, and a 884x that doesn't answer. The 8890's other report IDs and the
    /// legacy records are for testing the app.
    pub fn is_for_everyone(&self) -> bool {
        match self.group {
            SimulatorGroup::K8890 => matches!(self.kind, SimulatorKind::K8890 { report_id: 3 }),
            SimulatorGroup::Legacy => false,
            SimulatorGroup::Standard | SimulatorGroup::Unidentified => true,
        }
    }

    pub(crate) fn of(model: &SimulatedModel) -> Self {
        let (group, kind) = match model.kind {
            SimulatedKind::K8890 { report_id } => (SimulatorGroup::K8890, SimulatorKind::K8890 { report_id }),
            SimulatedKind::Identified(identity) => (
                if identity.record_format() == RecordFormat::Standard {
                    SimulatorGroup::Standard
                } else {
                    SimulatorGroup::Legacy
                },
                SimulatorKind::Identified {
                    keys: identity.key_count,
                    knobs: identity.knob_count,
                    kind: identity.kind,
                    usb: model.usb.to_string(),
                },
            ),
            SimulatedKind::Unidentified => {
                (SimulatorGroup::Unidentified, SimulatorKind::Unidentified { usb: model.usb.to_string() })
            }
        };
        Self { id: model.id(), group, kind }
    }
}
