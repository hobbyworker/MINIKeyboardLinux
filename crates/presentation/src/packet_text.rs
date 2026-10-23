//! Bytes as the device log shows them, in hex.

/// "03 FB FB FB" — the zero padding a report ends with left out, keeping its first byte.
pub(crate) fn summary(bytes: &[u8]) -> String {
    hex(&bytes[..summary_count(bytes)])
}

/// How many bytes the summary shows.
pub(crate) fn summary_count(bytes: &[u8]) -> usize {
    let mut end = bytes.len();
    while end > 1 && bytes[end - 1] == 0 {
        end -= 1;
    }
    end
}

/// Every byte, 16 to a row after the row's offset: "0010  00 00 00 …".
pub(crate) fn rows(bytes: &[u8]) -> Vec<String> {
    bytes.chunks(16).enumerate().map(|(row, chunk)| format!("{:04X}  {}", row * 16, hex(chunk))).collect()
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02X}")).collect::<Vec<_>>().join(" ")
}
