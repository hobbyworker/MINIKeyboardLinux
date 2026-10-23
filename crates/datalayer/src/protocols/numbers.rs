//! Where each control is stored: the 1189:8890's numbers, and the records of the cross-platform app's
//! layouts on a 884x keyboard.

use domain::ControlId::{self, Key, Knob};
use domain::KeyboardLayout;
use domain::KnobPart::{Press, PressRotateLeft, PressRotateRight, RotateLeft, RotateRight};

/// How the 1189:8890 numbers its controls: keys 1–12, then three actions to a knob from 13 — turn left,
/// press, turn right. The Windows tool has no number for a third knob, or for turning a knob pressed
/// (`KeyboardCapabilities::can_address` says the same in the domain's words).
pub fn k8890_number(control: ControlId) -> Option<u8> {
    match control {
        Key(number) => (1..=12).contains(&number).then_some(number),
        Knob(number, part) => {
            ((1..=2).contains(&number) && part <= RotateRight).then(|| 13 + (number - 1) * 3 + part.index())
        }
    }
}

/// The record a 884x keyboard keeps `control` of `layout` in: a key in the record of its number, a knob
/// action where the cross-platform app puts it. In 0add1, records 19 and 21 turn its one knob while it is
/// pressed — a second knob would have its press in record 20, as in 0add2.
pub fn record(control: ControlId, layout: &KeyboardLayout) -> Option<u8> {
    if !layout.has(control) {
        return None;
    }
    match control {
        Key(number) => Some(number),
        Knob(..) => KNOBS
            .iter()
            .find(|(id, _)| *id == layout.id)
            .and_then(|(_, records)| records.iter().find(|(knob, _)| *knob == control))
            .map(|(_, record)| *record),
    }
}

/// The control stored in `record`, or `None` when the layout has none there.
pub fn control_in_record(record_id: u8, layout: &KeyboardLayout) -> Option<ControlId> {
    layout.controls.iter().copied().find(|control| record(*control, layout) == Some(record_id))
}

type KnobRecords = &'static [(ControlId, u8)];

#[rustfmt::skip]
static KNOBS: &[(&str, KnobRecords)] = &[
    ("0add1", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(1, PressRotateLeft), 19), (Knob(1, PressRotateRight), 21)]),
    ("0add2", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21)]),
    ("0add3", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21), (Knob(3, RotateLeft), 22), (Knob(3, Press), 23), (Knob(3, RotateRight), 24)]),
    ("3add1", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18)]),
    ("4add1", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18)]),
    ("4add2", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18)]),
    ("4add3", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21), (Knob(3, RotateLeft), 22), (Knob(3, Press), 23), (Knob(3, RotateRight), 24)]),
    ("6add1", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18)]),
    ("6add2", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21)]),
    ("9add2", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21)]),
    ("9add3", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21), (Knob(3, RotateLeft), 22), (Knob(3, Press), 23), (Knob(3, RotateRight), 24)]),
    ("11add3", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21), (Knob(3, RotateLeft), 22), (Knob(3, Press), 23), (Knob(3, RotateRight), 24)]),
    ("12add2", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21)]),
    ("12add3", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21), (Knob(3, RotateLeft), 22), (Knob(3, Press), 23), (Knob(3, RotateRight), 24)]),
    ("12add4", &[(Knob(4, RotateLeft), 13), (Knob(4, Press), 14), (Knob(4, RotateRight), 15), (Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21), (Knob(3, RotateLeft), 22), (Knob(3, Press), 23), (Knob(3, RotateRight), 24)]),
    ("15add3", &[(Knob(1, RotateLeft), 16), (Knob(1, Press), 17), (Knob(1, RotateRight), 18), (Knob(2, RotateLeft), 19), (Knob(2, Press), 20), (Knob(2, RotateRight), 21), (Knob(3, RotateLeft), 22), (Knob(3, Press), 23), (Knob(3, RotateRight), 24)]),
    ("16add3", &[(Knob(1, RotateLeft), 17), (Knob(1, Press), 18), (Knob(1, RotateRight), 19), (Knob(2, RotateLeft), 20), (Knob(2, Press), 21), (Knob(2, RotateRight), 22), (Knob(3, RotateLeft), 23), (Knob(3, Press), 24), (Knob(3, RotateRight), 25)]),
    ("21add1", &[(Knob(1, RotateRight), 22), (Knob(1, Press), 23), (Knob(1, RotateLeft), 24)]),
];
