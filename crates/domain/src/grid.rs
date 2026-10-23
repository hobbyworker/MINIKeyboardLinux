use crate::layout::{ControlId, KeyboardLayout, KnobPart};

/// The arrangement of a keyboard whose shape no vendor tool draws — the 1189:8890's Windows tool shows the
/// same panel of buttons for every keyboard. It says only what is known: how many keys, how many to a row,
/// how many knobs. A schematic, not the keyboard's real shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridShape {
    keys: u8,
    columns: u8,
    knobs: u8,
}

impl GridShape {
    pub fn new(keys: u8, columns: u8, knobs: u8) -> Option<Self> {
        let valid = (keys > 0 || knobs > 0) && (1..=keys.max(1)).contains(&columns);
        valid.then_some(Self { keys, columns, knobs })
    }

    pub fn keys(&self) -> u8 {
        self.keys
    }

    pub fn columns(&self) -> u8 {
        self.columns
    }

    pub fn knobs(&self) -> u8 {
        self.knobs
    }

    /// "grid-6-3-1": six keys, three to a row, one knob.
    pub fn id(&self) -> String {
        format!("grid-{}-{}-{}", self.keys, self.columns, self.knobs)
    }

    pub fn from_id(id: &str) -> Option<Self> {
        let mut parts = id.split('-');
        if parts.next()? != "grid" {
            return None;
        }
        let keys = parts.next()?.parse().ok()?;
        let columns = parts.next()?.parse().ok()?;
        let knobs = parts.next()?.parse().ok()?;
        if parts.next().is_some() {
            return None;
        }
        Self::new(keys, columns, knobs)
    }

    pub fn rows(&self) -> u8 {
        if self.keys == 0 { 0 } else { self.keys.div_ceil(self.columns) }
    }

    /// The arrangement's controls: keys 1… filling the rows from the top left, then each knob's three
    /// actions.
    pub fn layout(&self) -> KeyboardLayout {
        let keys = (1..=self.keys).map(ControlId::Key);
        let knobs = (1..=self.knobs).flat_map(|knob| {
            [KnobPart::RotateLeft, KnobPart::Press, KnobPart::RotateRight].map(|part| ControlId::Knob(knob, part))
        });
        KeyboardLayout::new(self.id(), keys.chain(knobs).collect())
    }
}

/// How large an arrangement a keyboard can have.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridLimits {
    pub max_keys: u8,
    pub max_knobs: u8,
}

impl GridLimits {
    /// What the vendor's Windows tool can program: twelve keys and two knobs. Until the user says otherwise,
    /// a 1189:8890 is drawn that large, four to a row as the tool's panel has them.
    pub const K8890: Self = Self { max_keys: 12, max_knobs: 2 };

    pub fn contains(&self, shape: &GridShape) -> bool {
        shape.keys <= self.max_keys && shape.knobs <= self.max_knobs
    }

    pub fn assumed(&self) -> GridShape {
        GridShape { keys: self.max_keys, columns: 4.min(self.max_keys.max(1)), knobs: self.max_knobs }
    }
}
