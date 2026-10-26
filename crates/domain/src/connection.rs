use std::fmt;

use crate::keyboards::{KeyboardFamily, UsbIdentity};

/// How the app reaches a keyboard's configuration interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    /// Written over USB directly — an interface no driver takes (the 1189:8890's).
    Usb { out_endpoint: u8 },
    /// Written and read as a HID device.
    Hid,
    /// A keyboard that exists only in the app.
    Simulated,
}

/// A keyboard whose configuration interface the app has open.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConnectedKeyboard {
    pub usb: UsbIdentity,
    pub family: KeyboardFamily,
    pub interface_number: u8,
    pub channel: Channel,
    /// The report ID the configuration goes after, from the interface's report descriptor.
    pub report_id: u8,
    /// Bytes per output report, the report ID included.
    pub report_length: usize,
    /// What the USB device says about itself; `None` for a simulated keyboard.
    pub details: Option<DeviceDetails>,
}

/// A supported keyboard plugged in — one the app can open. Several can be plugged in; the app opens one at a time.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttachedKeyboard {
    /// Which keyboard it is while it stays plugged in.
    pub id: String,
    pub usb: UsbIdentity,
    /// Where it is plugged in — a USB port path. These keyboards carry no serial number, so this is what tells two
    /// with the same USB ID apart, and what finds the one chosen last time again.
    pub location: String,
}

/// The supported keyboards plugged in, in the order the app looks at them, and the one it opens.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AttachedKeyboards {
    pub keyboards: Vec<AttachedKeyboard>,
    /// The one open, being opened or that failed to open; `None` when none is plugged in.
    pub selected: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    Searching,
    Connected(ConnectedKeyboard),
    /// The system lets the app open the keyboard only once it is allowed to — on Linux, a udev rule that
    /// gives the signed-in user its device files.
    NeedsPermission,
    /// Opening failed for another reason (an errno, or a USB library's error code).
    OpenFailed {
        code: i32,
    },
}

impl ConnectionState {
    pub fn keyboard(&self) -> Option<&ConnectedKeyboard> {
        match self {
            Self::Connected(keyboard) => Some(keyboard),
            _ => None,
        }
    }
}

/// Why a transfer with the keyboard failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyboardDeviceError {
    /// No keyboard is open.
    NotConnected,
    /// A report could not be written (an errno, or a USB library's error code).
    WriteFailed { code: i32 },
    /// An answer did not come in time.
    NoReply,
    /// An answer was too short to read.
    InvalidReply,
    /// The keyboard has no command for this.
    Unsupported,
}

impl fmt::Display for KeyboardDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotConnected => f.write_str("no keyboard is open"),
            Self::WriteFailed { code } => write!(f, "a report could not be written ({code})"),
            Self::NoReply => f.write_str("no answer came in time"),
            Self::InvalidReply => f.write_str("an answer was too short"),
            Self::Unsupported => f.write_str("the keyboard has no command for this"),
        }
    }
}

impl std::error::Error for KeyboardDeviceError {}

/// What the USB device and its interfaces say about themselves — read from what the system already knows,
/// never asked of the keyboard. These keyboards carry no strings and their vendor IDs aren't the maker's, so
/// nothing here is guessed at.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DeviceDetails {
    /// `bcdUSB`, e.g. 0x0110.
    pub usb_version: u16,
    /// `bcdDevice`; 0 on keyboards that don't say.
    pub device_release: u16,
    /// Link speed in bits per second.
    pub speed: Option<u64>,
    pub manufacturer: Option<String>,
    pub product_name: Option<String>,
    pub serial_number: Option<String>,
    /// The most current the keyboard asks for, in mA (`bMaxPower`).
    pub max_power: Option<u32>,
    pub interfaces: Vec<DeviceInterface>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeviceInterface {
    pub number: u8,
    pub class: u8,
    pub subclass: u8,
    pub protocol: u8,
    /// The (usage page, usage) of each top-level collection of the interface's HID report descriptor; empty
    /// when the system made no HID device of it.
    pub usages: Vec<UsagePair>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UsagePair {
    pub page: u16,
    pub usage: u16,
}
