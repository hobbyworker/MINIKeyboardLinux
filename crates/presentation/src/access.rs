//! What the signed-in user needs to open the keyboards: a udev rule that gives them the 1189:8890's USB device
//! and the 884x keyboards' hidraw devices.

use domain::KeyboardFamily;

/// Where the rule goes. It sorts before systemd's `73-seat-late.rules`, which turns `uaccess` into access.
pub const UDEV_RULES_PATH: &str = "/etc/udev/rules.d/60-minikeyboard.rules";

/// The rule: a line for each way in and vendor — the 1189:8890's USB device, then each vendor's hidraw devices —
/// with the products as udev's alternatives (`8840|8842|…`), so the commands stay short. sysfs writes USB IDs in
/// lower case.
pub fn udev_rules() -> String {
    let mut groups: Vec<(bool, u16, Vec<u16>)> = Vec::new();
    for usb in KeyboardFamily::SUPPORTED {
        let over_usb = matches!(KeyboardFamily::of(usb), Some(KeyboardFamily::K8890));
        match groups.iter_mut().find(|(usb_device, vendor, _)| *usb_device == over_usb && *vendor == usb.vendor_id) {
            Some((_, _, products)) => products.push(usb.product_id),
            None => groups.push((over_usb, usb.vendor_id, vec![usb.product_id])),
        }
    }
    let mut lines = vec!["# Lets the signed-in user configure the supported USB macro keyboards.".to_owned()];
    for (over_usb, vendor, products) in groups {
        let products = products.iter().map(|product| format!("{product:04x}")).collect::<Vec<_>>().join("|");
        lines.push(if over_usb {
            format!(r#"SUBSYSTEM=="usb", ATTR{{idVendor}}=="{vendor:04x}", ATTR{{idProduct}}=="{products}", TAG+="uaccess""#)
        } else {
            format!(r#"KERNEL=="hidraw*", ATTRS{{idVendor}}=="{vendor:04x}", ATTRS{{idProduct}}=="{products}", TAG+="uaccess""#)
        });
    }
    lines.join("\n") + "\n"
}

/// The commands that put the rule in place and apply it to keyboards already plugged in.
pub fn udev_install_commands() -> String {
    format!(
        "sudo tee {UDEV_RULES_PATH} > /dev/null <<'EOF'\n{}EOF\nsudo udevadm control --reload-rules && sudo udevadm trigger\n",
        udev_rules()
    )
}
