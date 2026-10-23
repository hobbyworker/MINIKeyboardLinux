use std::collections::BTreeSet;

use crate::keyboards::KeyboardIdentity;

/// One action of a knob. Some knobs also turn while pressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KnobPart {
    RotateLeft,
    Press,
    RotateRight,
    PressRotateLeft,
    PressRotateRight,
}

impl KnobPart {
    pub const ALL: [Self; 5] =
        [Self::RotateLeft, Self::Press, Self::RotateRight, Self::PressRotateLeft, Self::PressRotateRight];

    /// 0 for a turn to the left, then in the order above.
    pub const fn index(self) -> u8 {
        self as u8
    }
}

/// A key, or one action of a knob — what the user selects and binds. Keys come before knobs, each in
/// number order, a knob's actions in [`KnobPart`] order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ControlId {
    Key(u8),
    Knob(u8, KnobPart),
}

impl ControlId {
    pub const fn is_key(&self) -> bool {
        matches!(self, Self::Key(_))
    }

    pub const fn knob_number(&self) -> Option<u8> {
        match self {
            Self::Knob(number, _) => Some(*number),
            Self::Key(_) => None,
        }
    }
}

/// How far a keyboard is turned clockwise from the way its layout is drawn — kept with the user's choice.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Rotation {
    #[default]
    Upright,
    Quarter,
    Half,
    ThreeQuarters,
}

impl Rotation {
    pub const ALL: [Self; 4] = [Self::Upright, Self::Quarter, Self::Half, Self::ThreeQuarters];

    /// Quarter turns clockwise, 0–3.
    pub const fn quarter_turns(self) -> u8 {
        self as u8
    }

    /// Any number of quarter turns, either way.
    pub const fn from_quarter_turns(turns: i64) -> Self {
        Self::ALL[turns.rem_euclid(4) as usize]
    }

    pub const fn next(self) -> Self {
        Self::from_quarter_turns(self as i64 + 1)
    }
}

/// A keyboard's controls, named as the cross-platform app names its layouts ("6add1" — six keys and a knob),
/// or as an arrangement the user set ([`GridShape`](crate::GridShape)). Where they are drawn is the
/// presentation's business, where they are stored the device's.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardLayout {
    pub id: String,
    pub controls: Vec<ControlId>,
}

impl KeyboardLayout {
    /// What the cross-platform app draws when a keyboard's answer matches no layout.
    pub const FALLBACK_ID: &str = "15add3";

    pub fn new(id: impl Into<String>, controls: Vec<ControlId>) -> Self {
        Self { id: id.into(), controls }
    }

    pub fn key_count(&self) -> usize {
        self.controls.iter().filter(|control| control.is_key()).count()
    }

    pub fn knob_count(&self) -> usize {
        self.controls.iter().filter_map(ControlId::knob_number).collect::<BTreeSet<_>>().len()
    }

    pub fn has(&self, control: ControlId) -> bool {
        self.controls.contains(&control)
    }

    /// The cross-platform app's layouts.
    pub fn all() -> &'static [KeyboardLayout] {
        crate::layouts::all()
    }

    pub fn named(id: &str) -> Option<&'static KeyboardLayout> {
        Self::all().iter().find(|layout| layout.id == id)
    }

    /// The layout the cross-platform app draws for what a keyboard says of itself
    /// (`Widget::Identify_KeyBoard_style`). The kind also tells two keyboards of four keys and a knob apart;
    /// the product ID tells two others apart.
    pub fn identified_id(identity: &KeyboardIdentity, product_id: u16) -> &'static str {
        match (identity.key_count, identity.knob_count) {
            (0, 1) => "0add1",
            (0, 2) => {
                if product_id == 0x8850 {
                    "0add2"
                } else {
                    "0add1"
                }
            }
            (0, 3) => "0add3",
            (1, 0) => "1add0",
            (2, 0) => "2add0",
            (3, 0) => "3add0",
            (3, 1) => "3add1",
            (4, 0) => "4Key",
            (4, 1) if identity.kind == 0 => {
                if product_id == 0x8851 {
                    "4add2"
                } else {
                    "4add1"
                }
            }
            (4, 1) if identity.kind == 1 => "4add2",
            (4, 3) => "4add3",
            (5, 0) => "5Key_Mute",
            (6, 0) => "6Key",
            (6, 1) => "6add1",
            (6, 2) => "6add2",
            (9, 2) => "9add2",
            (9, 3) => "9add3",
            (11, 3) => "11add3",
            (12, 0) => "12add0",
            (12, 2) => "12add2",
            (12, 3) => "12add3",
            (12, 4) => "12add4",
            (16, 0) => "16add0",
            (16, 3) => "16add3",
            (21, 1) => "21add1",
            _ => Self::FALLBACK_ID,
        }
    }
}
