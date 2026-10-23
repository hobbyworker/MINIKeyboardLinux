//! What writing to the keyboard needs from its USB descriptors, parsed from the raw bytes.

use domain::UsagePair;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Endpoint {
    pub address: u8,
    pub max_packet_size: u16,
}

/// One interface of a configuration descriptor, alternate setting 0.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Interface {
    /// Every interrupt OUT endpoint, in descriptor order.
    pub out_endpoints: Vec<Endpoint>,
    /// From the interface's HID descriptor.
    pub report_descriptor_length: Option<u16>,
}

impl Interface {
    pub fn out_endpoint(&self) -> Option<Endpoint> {
        self.out_endpoints.first().copied()
    }
}

/// Interface `number` (alternate setting 0) in a configuration descriptor, or `None` when the configuration
/// has no such interface.
pub fn interface(number: u8, configuration: &[u8]) -> Option<Interface> {
    let mut found: Option<Interface> = None;
    let mut inside = false;
    let mut offset = 0;
    while offset + 2 <= configuration.len() {
        let length = usize::from(configuration[offset]);
        if length < 2 || offset + length > configuration.len() {
            break;
        }
        let descriptor = &configuration[offset..offset + length];
        match descriptor[1] {
            // Interface
            0x04 if length >= 9 => {
                inside = descriptor[2] == number && descriptor[3] == 0;
                if inside {
                    found = Some(Interface::default());
                }
            }
            // HID: the first class descriptor is the report descriptor.
            0x21 if inside && length >= 9 => {
                if descriptor[6] == 0x22
                    && let Some(found) = &mut found
                {
                    found.report_descriptor_length = Some(u16::from_le_bytes([descriptor[7], descriptor[8]]));
                }
            }
            // Endpoint
            0x05 if inside && length >= 7 => {
                let address = descriptor[2];
                let is_interrupt = descriptor[3] & 0x03 == 0x03;
                if is_interrupt
                    && address & 0x80 == 0
                    && let Some(found) = &mut found
                {
                    let max_packet_size = u16::from_le_bytes([descriptor[4], descriptor[5]]) & 0x7FF;
                    found.out_endpoints.push(Endpoint { address, max_packet_size });
                }
            }
            _ => {}
        }
        offset += length;
    }
    found
}

/// The output report a HID report descriptor declares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputReport {
    /// 0 when the descriptor numbers no reports.
    pub id: u8,
    /// In bytes, without the report ID.
    pub length: usize,
}

/// The first output report in a HID report descriptor, with its whole length (every Output item of that ID).
pub fn first_output_report(report_descriptor: &[u8]) -> Option<OutputReport> {
    let (mut size, mut count, mut id) = (0usize, 0usize, 0u8);
    let mut stack = vec![];
    let mut first: Option<u8> = None;
    let mut bits = 0;
    for item in items(report_descriptor) {
        match (item.kind, item.tag) {
            (1, 0x7) => size = item.value as usize,
            (1, 0x8) => id = item.value as u8,
            (1, 0x9) => count = item.value as usize,
            (1, 0xA) => stack.push((size, count, id)),
            (1, 0xB) => {
                if let Some(saved) = stack.pop() {
                    (size, count, id) = saved;
                }
            }
            // Output
            (0, 0x9) => {
                let first = *first.get_or_insert(id);
                if id == first {
                    bits += size * count;
                }
            }
            _ => {}
        }
    }
    let first = first?;
    (bits > 0).then(|| OutputReport { id: first, length: bits.div_ceil(8) })
}

/// The first usage page the descriptor names — its top collection's. Vendor-defined from 0xFF00.
pub fn usage_page(report_descriptor: &[u8]) -> Option<u32> {
    items(report_descriptor).find(|item| item.kind == 1 && item.tag == 0x0).map(|item| item.value)
}

/// The (usage page, usage) of each top-level collection — what an interface is for.
pub fn top_level_usages(report_descriptor: &[u8]) -> Vec<UsagePair> {
    let mut pairs: Vec<UsagePair> = vec![];
    let (mut page, mut usage, mut depth) = (0u32, None::<u32>, 0usize);
    for item in items(report_descriptor) {
        match (item.kind, item.tag) {
            (1, 0x0) => page = item.value,
            (2, 0x0) => {
                if usage.is_none() {
                    usage = Some(item.value);
                }
            }
            // Collection, End Collection
            (0, 0xA) => {
                if depth == 0
                    && let Some(usage) = usage
                {
                    // A four-byte usage carries its own page.
                    let pair = if item_size_is_extended(usage) {
                        UsagePair { page: (usage >> 16) as u16, usage: (usage & 0xFFFF) as u16 }
                    } else {
                        UsagePair { page: page as u16, usage: usage as u16 }
                    };
                    if !pairs.contains(&pair) {
                        pairs.push(pair);
                    }
                }
                depth += 1;
                usage = None;
            }
            (0, 0xC) => {
                depth = depth.saturating_sub(1);
                usage = None;
            }
            (0, _) => usage = None,
            _ => {}
        }
    }
    pairs
}

fn item_size_is_extended(usage: u32) -> bool {
    usage > 0xFFFF
}

/// A short item: its type (0 main, 1 global, 2 local), tag and value.
struct Item {
    kind: u8,
    tag: u8,
    value: u32,
}

/// Every short item. Long items are skipped.
fn items(bytes: &[u8]) -> impl Iterator<Item = Item> + '_ {
    let mut offset = 0;
    std::iter::from_fn(move || {
        loop {
            let prefix = *bytes.get(offset)?;
            if prefix == 0xFE {
                // Long item: data size, tag, data.
                offset += 3 + usize::from(*bytes.get(offset + 1)?);
                continue;
            }
            let size = [0, 1, 2, 4][usize::from(prefix & 0x03)];
            let data = bytes.get(offset + 1..offset + 1 + size)?;
            let value = data.iter().rev().fold(0u32, |value, byte| value << 8 | u32::from(*byte));
            offset += 1 + size;
            return Some(Item { kind: prefix >> 2 & 0x03, tag: prefix >> 4, value });
        }
    })
}
