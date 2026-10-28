use std::time::SystemTime;

use crate::grid::GridLimits;
use crate::keyboards::{KeyboardFamily, KeyboardIdentity, RecordFormat, UsbIdentity};
use crate::layout::{KeyboardLayout, Rotation};

/// The layout a user chose for a keyboard that doesn't say its shape, and how the keyboard is turned.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LayoutProfile {
    pub layout_id: String,
    pub rotation: Rotation,
}

/// Where the layout drawn for the connected keyboard came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    /// The keyboard said how many keys and knobs it has.
    Identified,
    /// The user chose it.
    Chosen,
    /// Nobody has chosen one yet: what the keyboard's vendor tool draws for a keyboard it knows nothing of.
    Assumed,
}

/// The layout the app draws for the connected keyboard, and where it came from.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResolvedLayout {
    pub layout: KeyboardLayout,
    pub rotation: Rotation,
    pub source: Source,
    /// The cross-platform app's layouts the user may choose from; empty when the keyboard said its shape, or
    /// is arranged instead.
    pub choices: Vec<KeyboardLayout>,
    /// For a keyboard no vendor tool draws: how large an arrangement ([`GridShape`](crate::GridShape)) the
    /// user may give it.
    pub grid: Option<GridLimits>,
}

impl ResolvedLayout {
    pub fn needs_choice(&self) -> bool {
        self.source == Source::Assumed
    }
}

/// What one line of the device log holds.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogContent {
    /// A report written to the keyboard, its report ID first unless it is 0.
    Output(Vec<u8>),
    /// A report the keyboard sent.
    Input(Vec<u8>),
    Event(DeviceLogEvent),
}

/// One line of the device log: a report written or read — every byte of it — or something that happened.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeviceLogEntry {
    /// Counts up from 1 as lines are added.
    pub id: u64,
    pub time: SystemTime,
    pub content: LogContent,
}

/// What happened to the connection, for the device log.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceLogEvent {
    Searching,
    Connected {
        usb: UsbIdentity,
        interface: u8,
    },
    NeedsPermission,
    OpenFailed {
        code: i32,
    },
    WriteFailed {
        code: i32,
    },
    /// A simulated keyboard took the place of the one on USB.
    Simulated {
        model_id: String,
    },
}

/// Which keyboard the app talks to: the one on USB, or one that exists only in the app.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum KeyboardSource {
    #[default]
    Hardware,
    Simulated(SimulatedModel),
}

impl KeyboardSource {
    pub fn is_simulated(&self) -> bool {
        matches!(self, Self::Simulated(_))
    }
}

/// What a simulated keyboard is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimulatedKind {
    /// A 1189:8890 whose descriptor declares this report ID.
    K8890 { report_id: u8 },
    /// A 884x keyboard that answers identification.
    Identified(KeyboardIdentity),
    /// A 884x keyboard that doesn't.
    Unidentified,
}

/// A keyboard the simulator plays: its USB ID, the commands it takes and, for a 884x keyboard that answers,
/// what it says about itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SimulatedModel {
    pub usb: UsbIdentity,
    pub kind: SimulatedKind,
}

impl SimulatedModel {
    pub fn id(&self) -> String {
        match self.kind {
            SimulatedKind::K8890 { report_id } => format!("8890-{report_id}"),
            SimulatedKind::Identified(identity) => {
                format!("{}-{}-{}-{}", self.usb, identity.key_count, identity.knob_count, identity.kind)
            }
            SimulatedKind::Unidentified => format!("{}-unidentified", self.usb),
        }
    }

    pub fn family(&self) -> KeyboardFamily {
        KeyboardFamily::of(self.usb).unwrap_or(KeyboardFamily::K884x)
    }

    /// The 8890 on each report ID, then every layout a 884x keyboard can be identified as on each format,
    /// then a 884x keyboard that doesn't answer.
    pub fn all() -> Vec<Self> {
        let mut models: Vec<Self> = [3, 0, 2]
            .into_iter()
            .map(|report_id| Self { usb: KeyboardFamily::K8890_USB, kind: SimulatedKind::K8890 { report_id } })
            .collect();
        for format in [RecordFormat::Standard, RecordFormat::Legacy] {
            models.extend(KeyboardLayout::all().iter().filter_map(|layout| Self::identified_as(layout, format)));
        }
        models.push(Self { usb: UsbIdentity::new(0x1189, 0x8840), kind: SimulatedKind::Unidentified });
        models
    }

    /// A keyboard that says what makes the cross-platform app draw `layout` with `format`: kind 10 for the
    /// standard format, 0 or 1 for the legacy one, with a product ID the rule looks at. `None` when no answer
    /// leads there — 4add1 and 4add2 exist only on the legacy format.
    pub fn identified_as(layout: &KeyboardLayout, format: RecordFormat) -> Option<Self> {
        let candidates: &[(u8, u16)] = if format == RecordFormat::Standard {
            &[(10, 0x8840), (10, 0x8850), (10, 0x8851)]
        } else {
            &[(0, 0x8840), (0, 0x8850), (0, 0x8851), (1, 0x8840)]
        };
        let keys = u8::try_from(layout.key_count()).ok()?;
        let knobs = u8::try_from(layout.knob_count()).ok()?;
        candidates.iter().find_map(|&(kind, product_id)| {
            let identity = KeyboardIdentity::new(keys, knobs, kind);
            (KeyboardLayout::identified_id(&identity, product_id) == layout.id).then(|| {
                // 8851 under 514C, the vendor every build of the cross-platform app looks for it under.
                let vendor = if product_id == 0x8851 { 0x514C } else { 0x1189 };
                Self { usb: UsbIdentity::new(vendor, product_id), kind: SimulatedKind::Identified(identity) }
            })
        })
    }
}
