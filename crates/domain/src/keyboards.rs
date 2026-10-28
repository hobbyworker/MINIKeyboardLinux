use std::fmt;

/// A USB device's vendor and product IDs. These keyboards carry no name or serial number, so this is all
/// there is to tell them apart by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UsbIdentity {
    pub vendor_id: u16,
    pub product_id: u16,
}

impl UsbIdentity {
    pub const fn new(vendor_id: u16, product_id: u16) -> Self {
        Self { vendor_id, product_id }
    }
}

/// "1189:8890".
impl fmt::Display for UsbIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04X}:{:04X}", self.vendor_id, self.product_id)
    }
}

/// The two kinds of keyboard the app configures, told apart by USB ID. Each came with a vendor tool of its
/// own, and the two tools speak different commands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyboardFamily {
    /// 1189:8890, the keyboard of the vendor's Windows tool. Its configuration interface (USB interface 1)
    /// takes eight-byte packets and answers nothing.
    K8890,
    /// The keyboards of the vendor's cross-platform app: vendor 1189 or 514C with product 8842, 8840,
    /// 8830–8833, 8850 or 8851. Configured through 64-byte reports on interface 0; most say how many keys and
    /// knobs they have, and the newer ones read back what they hold.
    K884x,
}

impl KeyboardFamily {
    pub const K8890_USB: UsbIdentity = UsbIdentity::new(0x1189, 0x8890);

    /// Every keyboard the app looks for, in the order it looks: the 1189:8890, then the eight product IDs the
    /// cross-platform app looks for under vendor 1189, then the same eight under 514C.
    pub const SUPPORTED: [UsbIdentity; 17] = {
        const PRODUCTS: [u16; 8] = [0x8842, 0x8840, 0x8830, 0x8831, 0x8832, 0x8833, 0x8850, 0x8851];
        let mut all = [Self::K8890_USB; 17];
        let mut index = 0;
        while index < 8 {
            all[1 + index] = UsbIdentity::new(0x1189, PRODUCTS[index]);
            all[9 + index] = UsbIdentity::new(0x514C, PRODUCTS[index]);
            index += 1;
        }
        all
    };

    pub fn of(usb: UsbIdentity) -> Option<Self> {
        if !Self::SUPPORTED.contains(&usb) {
            return None;
        }
        Some(if usb == Self::K8890_USB { Self::K8890 } else { Self::K884x })
    }

    /// The USB interface that takes the configuration.
    pub const fn configuration_interface(self) -> u8 {
        match self {
            Self::K8890 => 1,
            Self::K884x => 0,
        }
    }
}

/// How a 884x keyboard stores a key's action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecordFormat {
    /// Firmware that answers identification with type 10 or above: `FD` records of three-byte entries — a delay
    /// and a code, every modifier an entry of its own. They read back.
    Standard,
    /// Firmware that answers with a type below 10: `FD` records of (modifier, key) pairs and one delay for the
    /// whole key. They read back.
    Legacy,
    /// A keyboard that does not answer identification: `FE` records of (modifier, key) pairs, written the way
    /// keyboards of the 1189:8840 kind are known to take them. Nothing reads back.
    Unidentified,
}

/// The commands a connected keyboard takes, settled once it is connected and identified.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandSet {
    /// The 1189:8890: packets after a report ID — 3, or 0 or 2 where the keyboard's descriptor says so.
    Packets { report_id: u8 },
    /// A 884x keyboard.
    Records(RecordFormat),
}

/// What a 884x keyboard answers when asked who it is (`FB FB FB`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardIdentity {
    pub key_count: u8,
    pub knob_count: u8,
    /// 10 or above for the standard record format, below 10 for the legacy one, as the cross-platform app tells them
    /// apart. On a keyboard of four keys and a knob it also picks between two layouts.
    pub kind: u8,
}

impl KeyboardIdentity {
    pub const fn new(key_count: u8, knob_count: u8, kind: u8) -> Self {
        Self { key_count, knob_count, kind }
    }

    pub const fn record_format(&self) -> RecordFormat {
        if self.kind >= 10 { RecordFormat::Standard } else { RecordFormat::Legacy }
    }

    /// Whether the type is one the app knows: 0 and 1, which the cross-platform app tells two layouts of four keys
    /// and a knob apart by, 10 for its standard firmware, and 11, the 514C:8850 seen to work. Any other is handled by
    /// the same rule — 12 and above as standard — but nothing says it behaves as that rule expects.
    pub const fn is_known_kind(&self) -> bool {
        matches!(self.kind, 0 | 1 | 10 | 11)
    }
}

/// What the app learned about a keyboard once it was connected.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardIdentification {
    pub commands: CommandSet,
    /// What a 884x keyboard said about itself; `None` on the 8890 and on a keyboard that did not answer.
    pub identity: Option<KeyboardIdentity>,
}
