//! The full-size keyboard of the Keys page, laid out as the cross-platform app's page: 15 key widths of main
//! block, the navigation keys and the keypad, with a PC's modifiers in the bottom row.

use std::sync::LazyLock;

use crate::choices::ModifierKey;
use crate::screen_state::BoardFace;

use super::KeyUsage;

/// A row's width in key widths: 15, a quarter, 3 of navigation, a quarter, 4 of keypad.
pub(crate) const FULL_KEYBOARD_WIDTH: f64 = 22.5;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Place {
    pub(crate) face: BoardFace,
    pub(crate) row: usize,
    pub(crate) column: f64,
    pub(crate) width: f64,
}

#[derive(Clone, Copy)]
enum Slot {
    Key(u8, f64),
    Modifier(ModifierKey, f64),
    Gap(f64),
}

impl Slot {
    fn width(self) -> f64 {
        match self {
            Self::Key(_, width) | Self::Modifier(_, width) | Self::Gap(width) => width,
        }
    }
}

use Slot::{Gap, Key as K, Modifier as M};

fn keys(usages: &[u8]) -> Vec<Slot> {
    usages.iter().map(|usage| K(*usage, 1.0)).collect()
}

fn rows() -> Vec<Vec<Slot>> {
    let row = |parts: Vec<Vec<Slot>>| parts.concat();
    vec![
        row(vec![
            vec![K(0x29, 1.0), Gap(1.0)],
            keys(&[0x3A, 0x3B, 0x3C, 0x3D]),
            vec![Gap(0.5)],
            keys(&[0x3E, 0x3F, 0x40, 0x41]),
            vec![Gap(0.5)],
            keys(&[0x42, 0x43, 0x44, 0x45]),
            vec![Gap(0.25)],
            keys(&[0x46, 0x47, 0x48]),
        ]),
        row(vec![
            keys(&[0x35, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x2D, 0x2E]),
            vec![K(0x2A, 2.0), Gap(0.25)],
            keys(&[0x49, 0x4A, 0x4B]),
            vec![Gap(0.25)],
            keys(&[0x53, 0x54, 0x55, 0x56]),
        ]),
        row(vec![
            vec![K(0x2B, 1.5)],
            keys(&[0x14, 0x1A, 0x08, 0x15, 0x17, 0x1C, 0x18, 0x0C, 0x12, 0x13, 0x2F, 0x30]),
            vec![K(0x31, 1.5), Gap(0.25)],
            keys(&[0x4C, 0x4D, 0x4E]),
            vec![Gap(0.25)],
            keys(&[0x5F, 0x60, 0x61, 0x57]),
        ]),
        row(vec![
            vec![K(0x39, 1.75)],
            keys(&[0x04, 0x16, 0x07, 0x09, 0x0A, 0x0B, 0x0D, 0x0E, 0x0F, 0x33, 0x34]),
            vec![K(0x28, 2.25), Gap(3.5)],
            keys(&[0x5C, 0x5D, 0x5E]),
        ]),
        row(vec![
            vec![M(ModifierKey::Shift, 2.25)],
            keys(&[0x1D, 0x1B, 0x06, 0x19, 0x05, 0x11, 0x10, 0x36, 0x37, 0x38]),
            vec![M(ModifierKey::RightShift, 2.75), Gap(1.25), K(0x52, 1.0), Gap(1.25)],
            keys(&[0x59, 0x5A, 0x5B, 0x58]),
        ]),
        row(vec![
            vec![
                M(ModifierKey::Control, 1.25),
                M(ModifierKey::Super, 1.25),
                M(ModifierKey::Alt, 1.25),
                K(0x2C, 6.25),
                M(ModifierKey::RightAlt, 1.25),
                M(ModifierKey::RightSuper, 1.25),
                K(0x65, 1.25),
                M(ModifierKey::RightControl, 1.25),
                Gap(0.25),
            ],
            keys(&[0x50, 0x51, 0x4F]),
            vec![Gap(0.25), K(0x62, 2.0), K(0x63, 1.0)],
        ]),
    ]
}

/// Every cap, row by row from the left.
pub(crate) fn places() -> &'static [Place] {
    static PLACES: LazyLock<Vec<Place>> = LazyLock::new(|| {
        let mut places = vec![];
        for (row, slots) in rows().into_iter().enumerate() {
            let mut column = 0.0;
            for slot in slots {
                let face = match slot {
                    K(usage, _) => {
                        Some(BoardFace::Key(*KeyUsage::named(usage).expect("every key on the board has a name")))
                    }
                    M(modifier, _) => Some(BoardFace::Modifier(modifier)),
                    Gap(_) => None,
                };
                if let Some(face) = face {
                    places.push(Place { face, row, column, width: slot.width() });
                }
                column += slot.width();
            }
        }
        places
    });
    &PLACES
}
