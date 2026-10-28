//! The records 884x keyboards store actions in, as the cross-platform app writes and reads them
//! (`PHY2_KEY_Value` and `PHY_KEY_Value`), and the `FE` record of keyboards that don't say which they take.
//!
//! A record starts `<marker> <record> <layer + 1> <kind>`; the kind is 1 for key presses, 2 for a media key,
//! 3 for a mouse action, 5 for key presses after a wait (legacy format) and 8 for lights.
//!
//! - Standard (64 bytes): `[5]` the entry count, then three-byte entries from `[6]` — the wait before the
//!   entry in milliseconds, high byte first, and a code: a key usage, or `F1`–`F8` for a modifier held until
//!   the next key.
//! - Legacy (50 bytes): `[4…5]` the wait, low byte first, `[9]` the count, then (modifiers, key) pairs from
//!   `[10]`. Modifiers with no key after them sit in the pair after the last.
//! - `FE`: as legacy, without a wait.

use domain::{
    KeyAction, KeyMacro, KeyStroke, KeyboardDeviceError, LightColor, MediaKey, Modifiers, MouseAction, RecordFormat,
};

use super::k884x::layer_byte;

pub const STANDARD_LENGTH: usize = 64;
pub const LEGACY_LENGTH: usize = 50;
pub const STANDARD_ENTRIES: usize = 18;
pub const LEGACY_PAIRS: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Keys = 1,
    Media = 2,
    Mouse = 3,
    DelayedKeys = 5,
    Lighting = 8,
}

impl Kind {
    fn of(byte: u8) -> Option<Self> {
        [Self::Keys, Self::Media, Self::Mouse, Self::DelayedKeys, Self::Lighting]
            .into_iter()
            .find(|kind| *kind as u8 == byte)
    }
}

// Encoding

/// The record that stores `action` for the control in record `record_id` of `layer` (from 0).
pub fn encode(
    action: &KeyAction,
    record_id: u8,
    layer: usize,
    format: RecordFormat,
) -> Result<Vec<u8>, KeyboardDeviceError> {
    match format {
        RecordFormat::Standard => standard(action, record_id, layer),
        RecordFormat::Legacy => pairs(action, 0xFD, record_id, layer, LEGACY_LENGTH, true),
        RecordFormat::Unidentified => unidentified(action, record_id, layer),
    }
}

pub fn standard(action: &KeyAction, record_id: u8, layer: usize) -> Result<Vec<u8>, KeyboardDeviceError> {
    let mut record = header(0xFD, record_id, layer, STANDARD_LENGTH);
    match action {
        KeyAction::Unassigned => record[3] = Kind::Keys as u8,
        KeyAction::Keys(macro_) => {
            record[3] = Kind::Keys as u8;
            let mut entries: Vec<(u32, u8)> = vec![];
            for stroke in &macro_.strokes {
                let mut codes = modifier_codes(stroke.modifiers);
                if !stroke.is_modifiers_only() {
                    codes.push(stroke.key);
                }
                for (index, code) in codes.into_iter().enumerate() {
                    entries.push((if index == 0 { stroke.delay } else { 0 }, code));
                }
            }
            if entries.len() > STANDARD_ENTRIES {
                return Err(KeyboardDeviceError::Unsupported);
            }
            record[5] = entries.len() as u8;
            for (index, (delay, code)) in entries.into_iter().enumerate() {
                let delay = u16::try_from(delay).unwrap_or(u16::MAX).to_be_bytes();
                record[6 + 3 * index..9 + 3 * index].copy_from_slice(&[delay[0], delay[1], code]);
            }
        }
        KeyAction::Media(key) => {
            record[3] = Kind::Media as u8;
            record[5] = 2;
            record[8] = (key.usage() & 0xFF) as u8;
            record[11] = (key.usage() >> 8) as u8;
        }
        KeyAction::Mouse(mouse) => {
            record[3] = Kind::Mouse as u8;
            // 1 for a mouse action; the cross-platform app's 2025-06 builds put 4 here for a gesture.
            record[4] = 1;
            record[5] = 4;
            record[8] = modifier_codes(mouse.modifiers()).first().copied().unwrap_or(0);
            record[11] = mouse.buttons();
            record[20] = mouse.wheel();
        }
    }
    Ok(record)
}

/// The legacy record, or with marker `FE` and no wait the record of an unidentified keyboard.
pub fn pairs(
    action: &KeyAction,
    marker: u8,
    record_id: u8,
    layer: usize,
    length: usize,
    keeps_delay: bool,
) -> Result<Vec<u8>, KeyboardDeviceError> {
    let mut record = header(marker, record_id, layer, length);
    match action {
        KeyAction::Unassigned => record[3] = Kind::Keys as u8,
        KeyAction::Keys(macro_) => {
            let delay = if keeps_delay { u16::try_from(macro_.delay).unwrap_or(u16::MAX) } else { 0 };
            record[3] = if delay > 0 { Kind::DelayedKeys } else { Kind::Keys } as u8;
            record[4..6].copy_from_slice(&delay.to_le_bytes());
            if let [only] = macro_.strokes.as_slice()
                && only.is_modifiers_only()
            {
                record[10] = only.modifiers.bits();
            } else {
                if macro_.strokes.len() > LEGACY_PAIRS || 10 + 2 * macro_.strokes.len() > length {
                    return Err(KeyboardDeviceError::Unsupported);
                }
                record[9] = macro_.strokes.len() as u8;
                for (index, stroke) in macro_.strokes.iter().enumerate() {
                    record[10 + 2 * index] = stroke.modifiers.bits();
                    record[11 + 2 * index] = stroke.key;
                }
            }
        }
        KeyAction::Media(key) => {
            record[3] = Kind::Media as u8;
            record[9] = 2;
            record[10] = (key.usage() & 0xFF) as u8;
            record[11] = (key.usage() >> 8) as u8;
        }
        KeyAction::Mouse(mouse) => {
            record[3] = Kind::Mouse as u8;
            record[9] = 1;
            record[10] = mouse.modifiers().bits();
            record[11] = mouse.buttons();
            record[14] = mouse.wheel();
        }
    }
    Ok(record)
}

/// The `FE` record: the legacy form with these differences, which keyboards of the 1189:8840 kind are known
/// to take — a media key has a count of 0, a wheel move a count of 3.
pub fn unidentified(action: &KeyAction, record_id: u8, layer: usize) -> Result<Vec<u8>, KeyboardDeviceError> {
    let mut record = pairs(action, 0xFE, record_id, layer, STANDARD_LENGTH, false)?;
    match action {
        KeyAction::Media(_) => record[9] = 0,
        KeyAction::Mouse(mouse) if mouse.wheel() != 0 => record[9] = 3,
        _ => {}
    }
    Ok(record)
}

/// A legacy-format layer's lights, which it keeps in record 0: `FE B0 <layer + 1> 08`, a count of 1 and
/// `colour << 4 | mode` in `[11]`.
pub fn legacy_lighting(color: LightColor, mode: u8, layer: usize, length: usize) -> Vec<u8> {
    let mut record = vec![0; length];
    record[..4].copy_from_slice(&[0xFE, 0xB0, layer_byte(layer + 1), Kind::Lighting as u8]);
    record[9] = 1;
    record[11] = color.code() << 4 | (mode & 0x0F);
    record
}

fn header(marker: u8, record_id: u8, layer: usize, length: usize) -> Vec<u8> {
    let mut record = vec![0; length];
    record[..3].copy_from_slice(&[marker, record_id, layer_byte(layer + 1)]);
    record
}

/// `F1`–`F8` for the modifiers set, lowest bit first: left Control, Shift, Alt, Super, then right.
pub fn modifier_codes(modifiers: Modifiers) -> Vec<u8> {
    Modifiers::each()
        .zip(0xF1u8..)
        .filter(|(modifier, _)| modifiers.contains(*modifier))
        .map(|(_, code)| code)
        .collect()
}

pub fn modifiers_of_code(code: u8) -> Option<Modifiers> {
    (0xF1..=0xF8).contains(&code).then(|| Modifiers::from_bits(1 << (code - 0xF1)))
}

/// What a record of `format` gives back of `action` once written — encoded, then decoded as a read decodes it.
/// `None` when the format can't hold it, or doesn't read back (the `FE` form).
pub fn stored_form(action: &KeyAction, format: RecordFormat) -> Option<KeyAction> {
    if format == RecordFormat::Unidentified {
        return None;
    }
    decode(&encode(action, 1, 0, format).ok()?, format)
}

// Decoding

/// The action a record the keyboard reported stores, or `None` for a kind the app doesn't know.
pub fn decode(record: &[u8], format: RecordFormat) -> Option<KeyAction> {
    let byte = |index: usize| record.get(index).copied().unwrap_or(0);
    let Some(kind) = Kind::of(byte(3)) else {
        return (byte(3) == 0).then_some(KeyAction::Unassigned);
    };
    let standard = format == RecordFormat::Standard;
    match kind {
        Kind::Keys | Kind::DelayedKeys if standard => {
            let mut strokes = vec![];
            let mut held = Modifiers::NONE;
            let mut held_delay: Option<u32> = None;
            for index in 0..usize::from(byte(5)).min(STANDARD_ENTRIES) {
                let code = byte(8 + 3 * index);
                let delay = u32::from(u16::from_be_bytes([byte(6 + 3 * index), byte(7 + 3 * index)]));
                if let Some(modifier) = modifiers_of_code(code) {
                    held.insert(modifier);
                    held_delay = held_delay.or(Some(delay));
                } else if code != 0 {
                    strokes.push(KeyStroke::new(held, code).with_delay(held_delay.unwrap_or(delay)));
                    held = Modifiers::NONE;
                    held_delay = None;
                }
            }
            if !held.is_empty() && strokes.is_empty() {
                strokes.push(KeyStroke::new(held, 0).with_delay(held_delay.unwrap_or(0)));
            }
            Some(if strokes.is_empty() { KeyAction::Unassigned } else { KeyAction::Keys(KeyMacro::new(strokes)) })
        }
        Kind::Keys | Kind::DelayedKeys => {
            let delay = if kind == Kind::DelayedKeys { u32::from(u16::from_le_bytes([byte(4), byte(5)])) } else { 0 };
            let count = usize::from(byte(9)).min(LEGACY_PAIRS);
            let mut strokes: Vec<KeyStroke> = (0..count)
                .filter_map(|index| {
                    let modifiers = Modifiers::from_bits(byte(10 + 2 * index));
                    let key = byte(11 + 2 * index);
                    (!modifiers.is_empty() || key != 0).then_some(KeyStroke::new(modifiers, key))
                })
                .collect();
            if count == 0 && byte(10) != 0 {
                strokes = vec![KeyStroke::new(Modifiers::from_bits(byte(10)), 0)];
            }
            if strokes.len() > 1 {
                strokes.retain(|stroke| !stroke.is_modifiers_only());
            }
            Some(if strokes.is_empty() && delay == 0 {
                KeyAction::Unassigned
            } else {
                KeyAction::Keys(KeyMacro::with_delay(strokes, delay))
            })
        }
        Kind::Media => {
            let usage = if standard {
                u16::from_le_bytes([byte(8), byte(11)])
            } else {
                u16::from_le_bytes([byte(10), byte(11)])
            };
            Some(if usage == 0 { KeyAction::Unassigned } else { KeyAction::Media(MediaKey(usage)) })
        }
        // A gesture (4 in byte 4), which the app doesn't make, isn't read as a mouse action.
        Kind::Mouse if standard && byte(4) == 4 => None,
        Kind::Mouse if standard => mouse(byte(11), byte(20), modifiers_of_code(byte(8)).unwrap_or_default()),
        Kind::Mouse => {
            let held = Modifiers::from_bits(byte(10)).members().next().unwrap_or_default();
            mouse(byte(11), byte(14), held)
        }
        Kind::Lighting => None,
    }
}

fn mouse(buttons: u8, wheel: u8, modifiers: Modifiers) -> Option<KeyAction> {
    if buttons == 0 && wheel == 0 {
        return Some(KeyAction::Unassigned);
    }
    MouseAction::from_parts(buttons, wheel, modifiers).map(KeyAction::Mouse)
}
