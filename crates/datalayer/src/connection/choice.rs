//! Which of the keyboards plugged in the app opens — the rule, apart from sysfs so tests can hold it.
//!
//! The one chosen this session, while it stays plugged in; else the one chosen last time (its USB ID and port),
//! when it is plugged in; else the first, in the order the app looks at keyboards, that can be opened at all.

use std::collections::HashMap;

use domain::{AttachedKeyboard, UsbIdentity};

/// What "the one chosen last time" is kept as — its USB ID and port, e.g. `1189:8890@1-2.3`.
pub(crate) fn key(keyboard: &AttachedKeyboard) -> String {
    format!("{}@{}", keyboard.usb, keyboard.location)
}

/// The keyboard to open. One the user chose (now or last time) is opened even when it can't be, so the failure is
/// shown rather than passed over for another.
pub(crate) fn target(
    keyboards: &[AttachedKeyboard],
    chosen: Option<&str>,
    remembered: Option<&str>,
    can_open: impl Fn(&AttachedKeyboard) -> bool,
) -> Option<AttachedKeyboard> {
    let chosen = chosen.and_then(|chosen| keyboards.iter().find(|keyboard| keyboard.id == chosen));
    let remembered = || remembered.and_then(|remembered| keyboards.iter().find(|keyboard| key(keyboard) == remembered));
    let first_to_open = || keyboards.iter().find(|keyboard| can_open(keyboard));
    chosen.or_else(remembered).or_else(first_to_open).or(keyboards.first()).cloned()
}

/// One entry for each keyboard, in the order its first place to write came — the 1189:8890 first, then the 884x
/// keyboards in the order the cross-platform app looks for them; several with the same USB ID by port.
pub(crate) fn ordered(found: &[AttachedKeyboard]) -> Vec<AttachedKeyboard> {
    let mut unique: Vec<AttachedKeyboard> = vec![];
    for keyboard in found {
        if !unique.iter().any(|other| other.id == keyboard.id) {
            unique.push(keyboard.clone());
        }
    }
    let mut first_of_usb: HashMap<UsbIdentity, usize> = HashMap::new();
    for (index, keyboard) in unique.iter().enumerate() {
        first_of_usb.entry(keyboard.usb).or_insert(index);
    }
    unique.sort_by(|a, b| (first_of_usb[&a.usb], &a.location).cmp(&(first_of_usb[&b.usb], &b.location)));
    unique
}

#[cfg(test)]
mod tests {
    use super::*;

    const K8890: UsbIdentity = UsbIdentity::new(0x1189, 0x8890);
    const K8840: UsbIdentity = UsbIdentity::new(0x1189, 0x8840);

    fn keyboard(id: &str, usb: UsbIdentity, location: &str) -> AttachedKeyboard {
        AttachedKeyboard { id: id.to_owned(), usb, location: location.to_owned() }
    }

    /// One entry per keyboard, in the order their places to write came; the same USB ID by port.
    #[test]
    fn ordered_by_family_then_port() {
        let a = keyboard("a", K8890, "1-2");
        let b = keyboard("b", K8890, "1-1");
        let c = keyboard("c", K8840, "1-0");
        assert_eq!(ordered(&[a.clone(), b.clone(), c.clone(), c.clone()]), vec![b, a, c]);
    }

    #[test]
    fn the_one_chosen_wins() {
        let keyboards = [keyboard("a", K8890, "1"), keyboard("b", K8840, "2")];
        let target = target(&keyboards, Some("b"), Some("1189:8890@1"), |_| true);
        assert_eq!(target, Some(keyboards[1].clone()));
    }

    /// Nothing chosen this session: the one chosen last time, found by its USB ID and port.
    #[test]
    fn then_the_one_chosen_last_time() {
        let keyboards = [keyboard("a", K8890, "1"), keyboard("b", K8840, "2")];
        let remembered = key(&keyboards[1]);
        assert_eq!(remembered, "1189:8840@2");
        let target = target(&keyboards, None, Some(&remembered), |_| true);
        assert_eq!(target, Some(keyboards[1].clone()));
    }

    /// Nothing chosen: the first that can be opened — and with none, the first, to show why it can't.
    #[test]
    fn else_the_first_that_opens() {
        let keyboards = [keyboard("a", K8890, "1"), keyboard("b", K8840, "2")];
        let gone = Some("1189:8890@9");
        assert_eq!(target(&keyboards, Some("x"), gone, |keyboard| keyboard.id == "b"), Some(keyboards[1].clone()));
        assert_eq!(target(&keyboards, None, None, |_| false), Some(keyboards[0].clone()));
        assert_eq!(target(&[], None, None, |_| true), None);
    }
}
