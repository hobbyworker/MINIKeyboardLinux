use std::sync::LazyLock;

/// A key on the HID keyboard page and how to show it. Labels are the same in every language — letters,
/// digits and what PC keyboards print on their keys; the app names the keys in the user's language.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyUsage {
    pub usage: u8,
    /// What a key cap shows: "A", "F5", "⌫".
    pub label: &'static str,
    /// The shifted character a key cap shows above, if any.
    pub shifted: Option<&'static str>,
}

const fn key(usage: u8, label: &'static str) -> KeyUsage {
    KeyUsage { usage, label, shifted: None }
}

const fn shifted(usage: u8, label: &'static str, shifted: &'static str) -> KeyUsage {
    KeyUsage { usage, label, shifted: Some(shifted) }
}

impl KeyUsage {
    /// Every key both vendor tools offer, and F13–F24 and the two non-US keys the cross-platform app adds.
    pub fn all() -> &'static [KeyUsage] {
        static ALL: LazyLock<Vec<KeyUsage>> = LazyLock::new(|| {
            [
                &KeyUsage::LETTERS[..],
                &KeyUsage::DIGITS,
                &KeyUsage::EDITING,
                &KeyUsage::SYMBOLS,
                &KeyUsage::FUNCTION_KEYS,
                &KeyUsage::SYSTEM,
                &KeyUsage::NAVIGATION,
                &KeyUsage::KEYPAD,
                &KeyUsage::EXTRA_FUNCTION_KEYS,
                &KeyUsage::INTERNATIONAL,
            ]
            .concat()
        });
        &ALL
    }

    pub fn named(usage: u8) -> Option<&'static KeyUsage> {
        Self::all().iter().find(|key| key.usage == usage)
    }

    pub const LETTERS: [KeyUsage; 26] = [
        key(0x04, "A"),
        key(0x05, "B"),
        key(0x06, "C"),
        key(0x07, "D"),
        key(0x08, "E"),
        key(0x09, "F"),
        key(0x0A, "G"),
        key(0x0B, "H"),
        key(0x0C, "I"),
        key(0x0D, "J"),
        key(0x0E, "K"),
        key(0x0F, "L"),
        key(0x10, "M"),
        key(0x11, "N"),
        key(0x12, "O"),
        key(0x13, "P"),
        key(0x14, "Q"),
        key(0x15, "R"),
        key(0x16, "S"),
        key(0x17, "T"),
        key(0x18, "U"),
        key(0x19, "V"),
        key(0x1A, "W"),
        key(0x1B, "X"),
        key(0x1C, "Y"),
        key(0x1D, "Z"),
    ];

    pub const DIGITS: [KeyUsage; 10] = [
        shifted(0x1E, "1", "!"),
        shifted(0x1F, "2", "@"),
        shifted(0x20, "3", "#"),
        shifted(0x21, "4", "$"),
        shifted(0x22, "5", "%"),
        shifted(0x23, "6", "^"),
        shifted(0x24, "7", "&"),
        shifted(0x25, "8", "*"),
        shifted(0x26, "9", "("),
        shifted(0x27, "0", ")"),
    ];

    pub const EDITING: [KeyUsage; 7] = [
        key(0x28, "↵"),
        key(0x29, "Esc"),
        key(0x2A, "⌫"),
        key(0x2B, "↹"),
        key(0x2C, "Space"),
        key(0x39, "⇪"),
        key(0x65, "Menu"),
    ];

    pub const SYMBOLS: [KeyUsage; 11] = [
        shifted(0x2D, "-", "_"),
        shifted(0x2E, "=", "+"),
        shifted(0x2F, "[", "{"),
        shifted(0x30, "]", "}"),
        shifted(0x31, "\\", "|"),
        shifted(0x33, ";", ":"),
        shifted(0x34, "'", "\""),
        shifted(0x35, "`", "~"),
        shifted(0x36, ",", "<"),
        shifted(0x37, ".", ">"),
        shifted(0x38, "/", "?"),
    ];

    pub const FUNCTION_KEYS: [KeyUsage; 12] = [
        key(0x3A, "F1"),
        key(0x3B, "F2"),
        key(0x3C, "F3"),
        key(0x3D, "F4"),
        key(0x3E, "F5"),
        key(0x3F, "F6"),
        key(0x40, "F7"),
        key(0x41, "F8"),
        key(0x42, "F9"),
        key(0x43, "F10"),
        key(0x44, "F11"),
        key(0x45, "F12"),
    ];

    pub const EXTRA_FUNCTION_KEYS: [KeyUsage; 12] = [
        key(0x68, "F13"),
        key(0x69, "F14"),
        key(0x6A, "F15"),
        key(0x6B, "F16"),
        key(0x6C, "F17"),
        key(0x6D, "F18"),
        key(0x6E, "F19"),
        key(0x6F, "F20"),
        key(0x70, "F21"),
        key(0x71, "F22"),
        key(0x72, "F23"),
        key(0x73, "F24"),
    ];

    pub const SYSTEM: [KeyUsage; 3] = [key(0x46, "PrtSc"), key(0x47, "ScrLk"), key(0x48, "Pause")];

    pub const NAVIGATION: [KeyUsage; 10] = [
        key(0x49, "Ins"),
        key(0x4A, "Home"),
        key(0x4B, "PgUp"),
        key(0x4C, "Del"),
        key(0x4D, "End"),
        key(0x4E, "PgDn"),
        key(0x4F, "→"),
        key(0x50, "←"),
        key(0x51, "↓"),
        key(0x52, "↑"),
    ];

    pub const KEYPAD: [KeyUsage; 17] = [
        key(0x53, "Num"),
        key(0x54, "÷"),
        key(0x55, "×"),
        key(0x56, "−"),
        key(0x57, "+"),
        key(0x58, "⌤"),
        key(0x59, "1"),
        key(0x5A, "2"),
        key(0x5B, "3"),
        key(0x5C, "4"),
        key(0x5D, "5"),
        key(0x5E, "6"),
        key(0x5F, "7"),
        key(0x60, "8"),
        key(0x61, "9"),
        key(0x62, "0"),
        key(0x63, "."),
    ];

    /// The two keys ISO keyboards have and ANSI ones don't ("Non-US # and ~", "Non-US \ and |").
    pub const INTERNATIONAL: [KeyUsage; 2] = [shifted(0x32, "#", "~"), shifted(0x64, "\\", "|")];
}
