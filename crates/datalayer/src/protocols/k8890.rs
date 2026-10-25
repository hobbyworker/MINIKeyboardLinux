//! The 1189:8890's commands, as the vendor's Windows tool sends them (`FormMain.Download_Click`,
//! `Send_SwLayer`, `Send_WriteFlash_Cmd`, `Send_WriteFlashLED_Cmd`). Every packet is eight bytes after the
//! report ID; the keyboard answers none of them.
//!
//! One key is written as the layer switch `A1 <layer>`, the key's packets and `AA AA`; lights as the layer
//! switch, `B0 <type> <mode>` and `AA A1`. On report ID 0 there is no layer switch and no layer in the type
//! byte.

use domain::{KeyAction, KeyboardDeviceError, MediaKey};

use super::k884x::layer_byte;

pub const PAYLOAD_LENGTH: usize = 8;

pub fn pad(bytes: &[u8]) -> Vec<u8> {
    let mut packet = bytes.to_vec();
    packet.resize(packet.len().max(PAYLOAD_LENGTH), 0);
    packet
}

/// Type nibbles (`KeyParam.KeyType_Num`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Keys = 1,
    Media = 2,
    Mouse = 3,
    Lighting = 8,
}

/// A key's packets and the save that follows. `layer` counts from 0.
pub fn download(
    number: u8,
    action: &KeyAction,
    layer: usize,
    report_id: u8,
) -> Result<Vec<Vec<u8>>, KeyboardDeviceError> {
    let mut packets = layer_switch(layer, report_id);
    packets.extend(key_packets(number, action, layer, report_id)?);
    packets.push(pad(&[0xAA, 0xAA]));
    Ok(packets)
}

/// The lights and their save — and, with `ends_key_repeat`, a save with no key in it. After `AA A1` the keyboard
/// repeats a key's first stroke without end, until the next key save or until it is plugged in again (the vendor's
/// tool does the same); the bare `AA AA` ends that and leaves the keys as they were.
pub fn download_lighting(mode: u8, layer: usize, report_id: u8, ends_key_repeat: bool) -> Vec<Vec<u8>> {
    let mut packets = layer_switch(layer, report_id);
    packets.push(pad(&[0xB0, type_byte(Kind::Lighting, layer, report_id), mode]));
    packets.push(pad(&[0xAA, 0xA1]));
    if ends_key_repeat {
        packets.extend(layer_switch(layer, report_id));
        packets.push(pad(&[0xAA, 0xAA]));
    }
    packets
}

pub fn layer_switch(layer: usize, report_id: u8) -> Vec<Vec<u8>> {
    if report_id == 0 { vec![] } else { vec![pad(&[0xA1, layer_byte(layer + 1)])] }
}

/// `(layer << 4) | kind`, or the kind alone on report ID 0.
pub fn type_byte(kind: Kind, layer: usize, report_id: u8) -> u8 {
    if report_id == 0 { kind as u8 } else { layer_byte(layer + 1) << 4 | kind as u8 }
}

/// A key press list is `count + 1` packets: the first carries the first press's modifiers, the rest one
/// (modifiers, key) group each. A key with modifiers alone has no group, its modifiers in the first packet; a
/// cleared key has nothing at all.
pub fn key_packets(
    number: u8,
    action: &KeyAction,
    layer: usize,
    report_id: u8,
) -> Result<Vec<Vec<u8>>, KeyboardDeviceError> {
    match action {
        KeyAction::Unassigned => Ok(vec![vec![number, type_byte(Kind::Keys, layer, report_id), 0, 0, 0, 0, 0, 0]]),
        KeyAction::Keys(macro_) => {
            let kind = type_byte(Kind::Keys, layer, report_id);
            let groups: Vec<_> = macro_.strokes.iter().filter(|stroke| !stroke.is_modifiers_only()).collect();
            if groups.len() > 5 {
                return Err(KeyboardDeviceError::Unsupported);
            }
            let first = macro_.strokes.first().map(|stroke| stroke.modifiers.bits()).unwrap_or(0);
            let count = groups.len() as u8;
            let mut packets = vec![vec![number, kind, count, 0, first, 0, 0, 0]];
            for (index, group) in (1u8..).zip(&groups) {
                packets.push(vec![number, kind, count, index, group.modifiers.bits(), group.key, 0, 0]);
            }
            Ok(packets)
        }
        KeyAction::Media(key) => {
            let (low, high) = media_bytes(*key, report_id).ok_or(KeyboardDeviceError::Unsupported)?;
            Ok(vec![vec![number, type_byte(Kind::Media, layer, report_id), low, high, 0, 0, 0, 0]])
        }
        KeyAction::Mouse(action) => Ok(vec![vec![
            number,
            type_byte(Kind::Mouse, layer, report_id),
            action.buttons(),
            0,
            0,
            action.wheel(),
            action.modifiers().bits(),
            0,
        ]]),
    }
}

/// The two bytes of a media key. Report ID 3 takes the consumer usage; the other two take codes of their
/// own, known for the Windows tool's six keys only.
pub fn media_bytes(key: MediaKey, report_id: u8) -> Option<(u8, u8)> {
    let table: &[(MediaKey, (u8, u8))] = match report_id {
        0 => &[
            (MediaKey::PLAY_PAUSE, (0x40, 0x00)),
            (MediaKey::PREVIOUS_TRACK, (0x80, 0x00)),
            (MediaKey::NEXT_TRACK, (0x00, 0x01)),
            (MediaKey::MUTE, (0x04, 0x00)),
            (MediaKey::VOLUME_UP, (0x02, 0x00)),
            (MediaKey::VOLUME_DOWN, (0x01, 0x00)),
        ],
        2 => &[
            (MediaKey::PLAY_PAUSE, (0x00, 0x04)),
            (MediaKey::PREVIOUS_TRACK, (0x00, 0x0B)),
            (MediaKey::NEXT_TRACK, (0x00, 0x0A)),
            (MediaKey::MUTE, (0x00, 0x01)),
            (MediaKey::VOLUME_UP, (0x40, 0x00)),
            (MediaKey::VOLUME_DOWN, (0x80, 0x00)),
        ],
        _ => return Some(((key.usage() & 0xFF) as u8, (key.usage() >> 8) as u8)),
    };
    table.iter().find(|(known, _)| *known == key).map(|(_, bytes)| *bytes)
}
