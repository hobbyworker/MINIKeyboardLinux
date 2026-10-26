//! What the kernel knows about the USB devices plugged in, from sysfs — readable without any permission, and
//! never asked of the keyboard.

use std::fs;
use std::path::{Path, PathBuf};

use domain::{AttachedKeyboard, DeviceDetails, DeviceInterface, KeyboardFamily, UsbIdentity};

use crate::descriptors;

const USB_DEVICES: &str = "/sys/bus/usb/devices";

/// A place to write the configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Candidate {
    /// What stays the same while the keyboard is plugged in: the interface's sysfs path, and the hidraw device.
    pub(crate) entry: String,
    pub(crate) usb: UsbIdentity,
    pub(crate) family: KeyboardFamily,
    pub(crate) interface_number: u8,
    pub(crate) device: PathBuf,
    pub(crate) kind: CandidateKind,
    /// The keyboard it is on.
    pub(crate) keyboard: AttachedKeyboard,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CandidateKind {
    /// The 1189:8890's interface 1, written over USB.
    Usb { bus: u8, address: u8 },
    /// A 884x keyboard's interface 0, written through its hidraw device.
    Hidraw { node: PathBuf },
}

/// Every place a supported keyboard takes its configuration: the 1189:8890's interface 1, then the hidraw
/// devices on a 884x keyboard's interface 0 — in the order the vendor apps look for USB IDs.
pub(crate) fn candidates() -> Vec<Candidate> {
    let mut found = vec![];
    for device in usb_devices() {
        let Some(usb) = usb_identity(&device) else { continue };
        let Some(family) = KeyboardFamily::of(usb) else { continue };
        let Some(keyboard) = attached_keyboard(&device, usb) else { continue };
        for interface in interfaces(&device) {
            let Some(number) = hex(&interface, "bInterfaceNumber") else { continue };
            if number != family.configuration_interface() {
                continue;
            }
            let entry = interface.to_string_lossy().into_owned();
            match family {
                KeyboardFamily::K8890 => {
                    let is_hid = hex(&interface, "bInterfaceClass") == Some(3)
                        && hex(&interface, "bInterfaceSubClass") == Some(0);
                    let bus = decimal(&device, "busnum");
                    let address = decimal(&device, "devnum");
                    if let (true, Some(bus), Some(address)) = (is_hid, bus, address) {
                        found.push(Candidate {
                            entry,
                            usb,
                            family,
                            interface_number: number,
                            device: device.clone(),
                            kind: CandidateKind::Usb { bus, address },
                            keyboard: keyboard.clone(),
                        });
                    }
                }
                KeyboardFamily::K884x => {
                    for node in hidraw_nodes(&interface) {
                        found.push(Candidate {
                            entry: format!("{entry} {}", node.display()),
                            usb,
                            family,
                            interface_number: number,
                            device: device.clone(),
                            kind: CandidateKind::Hidraw { node },
                            keyboard: keyboard.clone(),
                        });
                    }
                }
            }
        }
    }
    let order = |candidate: &Candidate| KeyboardFamily::SUPPORTED.iter().position(|usb| *usb == candidate.usb);
    found.sort_by_key(order);
    found
}

/// The keyboard a USB device is: the port it is plugged into — the device's sysfs name, `<bus>-<port>.<port>…`,
/// which stays the same when it is plugged into the same port again — and, to tell one plugging from the next,
/// the address the bus gave it.
fn attached_keyboard(device: &Path, usb: UsbIdentity) -> Option<AttachedKeyboard> {
    let location = device.file_name()?.to_string_lossy().into_owned();
    let address = read_text(&device.join("devnum"))?;
    Some(AttachedKeyboard { id: format!("{location} {address}"), usb, location })
}

/// The USB device's first configuration descriptor, with its interfaces and endpoints — the kernel keeps them
/// after the device descriptor.
pub(crate) fn configuration_descriptor(device: &Path) -> Option<Vec<u8>> {
    let bytes = fs::read(device.join("descriptors")).ok()?;
    let configuration = bytes.get(18..)?;
    let total = usize::from(u16::from_le_bytes([*configuration.get(2)?, *configuration.get(3)?]));
    Some(configuration[..total.min(configuration.len())].to_vec())
}

/// What the USB device and its interfaces say about themselves.
pub(crate) fn device_details(device: &Path) -> DeviceDetails {
    let text = |name: &str| read_text(&device.join(name)).filter(|value| !value.is_empty());
    let mut interfaces: Vec<DeviceInterface> = interfaces(device)
        .into_iter()
        .filter_map(|interface| {
            Some(DeviceInterface {
                number: hex(&interface, "bInterfaceNumber")?,
                class: hex(&interface, "bInterfaceClass").unwrap_or(0),
                subclass: hex(&interface, "bInterfaceSubClass").unwrap_or(0),
                protocol: hex(&interface, "bInterfaceProtocol").unwrap_or(0),
                usages: hid_devices(&interface)
                    .iter()
                    .filter_map(|hid| fs::read(hid.join("report_descriptor")).ok())
                    .flat_map(|report| descriptors::top_level_usages(&report))
                    .collect(),
            })
        })
        .collect();
    interfaces.sort_by_key(|interface| interface.number);
    DeviceDetails {
        usb_version: text("version").and_then(|version| bcd_version(&version)).unwrap_or(0),
        device_release: text("bcdDevice").and_then(|release| u16::from_str_radix(&release, 16).ok()).unwrap_or(0),
        speed: text("speed")
            .and_then(|megabits| megabits.parse::<f64>().ok())
            .map(|megabits| (megabits * 1_000_000.0) as u64),
        manufacturer: text("manufacturer"),
        product_name: text("product"),
        serial_number: text("serial"),
        max_power: text("bMaxPower").and_then(|power| power.trim_end_matches("mA").parse().ok()),
        interfaces,
    }
}

/// " 1.10" → 0x0110.
fn bcd_version(text: &str) -> Option<u16> {
    let (major, minor) = text.trim().split_once('.')?;
    Some(major.parse::<u16>().ok()? << 8 | u16::from_str_radix(minor, 16).ok()?)
}

/// Devices, not interfaces or root hubs.
fn usb_devices() -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(USB_DEVICES) else { return vec![] };
    let mut devices: Vec<PathBuf> = entries
        .flatten()
        .filter(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            !name.contains(':') && !name.starts_with("usb")
        })
        .map(|entry| entry.path())
        .collect();
    devices.sort();
    devices
}

fn usb_identity(device: &Path) -> Option<UsbIdentity> {
    let vendor = u16::from_str_radix(&read_text(&device.join("idVendor"))?, 16).ok()?;
    let product = u16::from_str_radix(&read_text(&device.join("idProduct"))?, 16).ok()?;
    Some(UsbIdentity::new(vendor, product))
}

/// The device's interfaces: its directories named `<device>:<configuration>.<interface>`.
fn interfaces(device: &Path) -> Vec<PathBuf> {
    let Some(name) = device.file_name().map(|name| format!("{}:", name.to_string_lossy())) else { return vec![] };
    let Ok(entries) = fs::read_dir(device) else { return vec![] };
    let mut interfaces: Vec<PathBuf> = entries
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().starts_with(&name))
        .map(|entry| entry.path())
        .collect();
    interfaces.sort();
    interfaces
}

/// The HID devices the kernel made of an interface: directories named `<bus>:<vendor>:<product>.<number>`.
fn hid_devices(interface: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(interface) else { return vec![] };
    let mut devices: Vec<PathBuf> = entries
        .flatten()
        .filter(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.len() == 19 && name.matches(':').count() == 2 && name.contains('.')
        })
        .map(|entry| entry.path())
        .collect();
    devices.sort();
    devices
}

/// `/dev/hidrawN` of each HID device on the interface.
fn hidraw_nodes(interface: &Path) -> Vec<PathBuf> {
    hid_devices(interface)
        .iter()
        .filter_map(|hid| fs::read_dir(hid.join("hidraw")).ok())
        .flat_map(|entries| entries.flatten().map(|entry| Path::new("/dev").join(entry.file_name())))
        .collect()
}

fn read_text(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok().map(|text| text.trim().to_owned())
}

fn hex(directory: &Path, name: &str) -> Option<u8> {
    u8::from_str_radix(&read_text(&directory.join(name))?, 16).ok()
}

fn decimal(directory: &Path, name: &str) -> Option<u8> {
    read_text(&directory.join(name))?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::bcd_version;

    #[test]
    fn versions() {
        assert_eq!(bcd_version(" 1.10"), Some(0x0110));
        assert_eq!(bcd_version("2.00"), Some(0x0200));
        assert_eq!(bcd_version("x"), None);
    }
}
