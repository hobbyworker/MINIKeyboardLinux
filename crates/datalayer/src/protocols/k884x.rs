//! The 884x keyboards' requests and answers, as the cross-platform app sends and reads them. Every request
//! is a 64-byte payload after report ID 3; every answer a 64-byte input report, its first byte the report ID.

use std::time::Duration;

use domain::{KeyColor, KeyboardIdentity, LayerLighting, LightColor, LightingSupport, RecordFormat};

use super::records;

pub const REPORT_ID: u8 = 0x03;
/// The report ID and 64 bytes.
pub const REPORT_LENGTH: usize = 65;

/// `FB FB FB`: how many keys and knobs, and the type (`Widget::Read_KeyBoard_KeyNum`).
pub const IDENTIFY: [u8; 3] = [0xFB, 0xFB, 0xFB];

/// `FD FE FF`: keep what was written. The app waits 200 ms after it.
pub const COMMIT: [u8; 3] = [0xFD, 0xFE, 0xFF];
pub const COMMIT_PAUSE: Duration = Duration::from_millis(200);

/// `FA <key records> <knobs> <layer + 1>`, as the cross-platform app asks (`read_Hidkey_Data`): the standard format
/// `FA 19 00` for its 25 records, the legacy one `FA 0F 03` — 15 key records, then 3 knobs of 3 records, 24 in all.
/// A standard keyboard (514C:8850) answers `FA <n> …` with records 1…n; how legacy firmware counts hasn't been seen,
/// so the read keeps what comes if fewer do.
pub fn read_records(layer: usize, format: RecordFormat) -> Option<Vec<u8>> {
    let layer = layer_byte(layer + 1);
    match format {
        RecordFormat::Standard => Some(vec![0xFA, 25, 0, layer]),
        RecordFormat::Legacy => Some(vec![0xFA, 15, 3, layer]),
        RecordFormat::Unidentified => None,
    }
}

/// How many answers follow [`read_records`], one record each, records 1… in order.
pub fn read_reply_count(format: RecordFormat) -> usize {
    if format == RecordFormat::Standard { 25 } else { 15 + 3 * 3 }
}

/// `FA B0 <layer>`: one layer's lights (standard format, `Read_RgbLed_DataDsp`). The layer counts from 0.
pub fn read_lighting(layer: usize) -> Vec<u8> {
    vec![0xFA, 0xB0, layer_byte(layer)]
}

/// `FE B0 <layer> <mode> <r g b × 16>` (standard format). The layer counts from 0.
pub fn lighting(layer: usize, mode: u8, colors: &[KeyColor]) -> Vec<u8> {
    let mut payload = vec![0xFE, 0xB0, layer_byte(layer), mode];
    for color in colors.iter().take(LightingSupport::COLOR_KEY_COUNT) {
        payload.extend([color.red, color.green, color.blue]);
    }
    payload
}

// Answers, the report ID first.

pub fn identity(reply: &[u8]) -> Option<KeyboardIdentity> {
    match reply {
        [_, _, keys, knobs, kind, ..] => Some(KeyboardIdentity::new(*keys, *knobs, *kind)),
        _ => None,
    }
}

/// A record answer carries the record after the report ID.
pub fn record(reply: &[u8]) -> &[u8] {
    reply.get(1..).unwrap_or_default()
}

/// `[2]` the mode, `[3…50]` sixteen colours.
pub fn lighting_reply(reply: &[u8]) -> Option<LayerLighting> {
    let count = LightingSupport::COLOR_KEY_COUNT;
    if reply.len() < 3 + 3 * count {
        return None;
    }
    let (triples, _) = reply[3..3 + 3 * count].as_chunks::<3>();
    let colors = triples.iter().map(|[red, green, blue]| KeyColor::new(*red, *green, *blue)).collect();
    Some(LayerLighting::PerKey { mode: reply[2], colors })
}

// Unidentified keyboards.

/// After an `FE` record: `AA AA`, `FD FE FF`, `AA AA`.
pub fn unidentified_save() -> Vec<Vec<u8>> {
    vec![vec![0xAA, 0xAA], COMMIT.to_vec(), vec![0xAA, 0xAA]]
}

/// Lights of an unidentified keyboard: `FE B0 <layer + 1> 08 … 01 00 <code>` and the commit, after the
/// identification and reads the cross-platform app sends first on the same connection — without them the
/// light is known to change but not to stay.
pub fn unidentified_lighting(color: LightColor, mode: u8, layer: usize) -> Vec<Vec<u8>> {
    let mut payloads = vec![vec![0xFB, 0xFB, 0xFB, 0x01]];
    payloads.extend((1..=3).map(|record| vec![0xFA, 0x0F, 0x03, record, 0x7F]));
    payloads.push(records::legacy_lighting(color, mode, layer, 12));
    payloads.push(COMMIT.to_vec());
    payloads
}

/// Layers count from 0 or 1 in a byte; there are at most three.
pub(crate) fn layer_byte(layer: usize) -> u8 {
    u8::try_from(layer).expect("a layer fits in a byte")
}
