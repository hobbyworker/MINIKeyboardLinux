use std::cell::RefCell;
use std::fmt;
use std::future::Future;
use std::rc::{Rc, Weak};
use std::time::SystemTime;

use domain::usecases::DeviceLogUseCase;
use domain::{DeviceLogEntry, DeviceLogEvent, LogContent};

use crate::packet_text;

/// Which way a line of the log went.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LogDirection {
    Output,
    Input,
    Event,
}

/// What happened to the connection; the window names it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogEvent {
    Searching,
    Connected { usb: String, interface: u8 },
    NeedsPermission,
    OpenFailed { code: i32 },
    WriteFailed { code: i32 },
    Simulated { id: String },
}

/// One line of the device log window.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LogLine {
    pub id: u64,
    pub time: SystemTime,
    pub direction: LogDirection,
    /// Every byte of the report written or read; empty for an event.
    pub bytes: Vec<u8>,
    pub event: Option<LogEvent>,
}

impl LogLine {
    /// The report up to the zero padding at its end: "03 FB FB FB".
    pub fn summary(&self) -> String {
        packet_text::summary(&self.bytes)
    }

    /// Whether [`summary`](Self::summary) leaves bytes out.
    pub fn is_shortened(&self) -> bool {
        packet_text::summary_count(&self.bytes) < self.bytes.len()
    }

    /// Every byte, 16 to a row after the row's offset: "0010  00 00 00 …".
    pub fn rows(&self) -> Vec<String> {
        packet_text::rows(&self.bytes)
    }

    /// Every byte on one line.
    pub fn hex(&self) -> String {
        packet_text::hex(&self.bytes)
    }

    fn of(entry: &DeviceLogEntry) -> Self {
        let (direction, bytes, event) = match &entry.content {
            LogContent::Output(bytes) => (LogDirection::Output, bytes.clone(), None),
            LogContent::Input(bytes) => (LogDirection::Input, bytes.clone(), None),
            LogContent::Event(event) => (
                LogDirection::Event,
                vec![],
                Some(match event {
                    DeviceLogEvent::Searching => LogEvent::Searching,
                    DeviceLogEvent::Connected { usb, interface } => {
                        LogEvent::Connected { usb: usb.to_string(), interface: *interface }
                    }
                    DeviceLogEvent::NeedsPermission => LogEvent::NeedsPermission,
                    DeviceLogEvent::OpenFailed { code } => LogEvent::OpenFailed { code: *code },
                    DeviceLogEvent::WriteFailed { code } => LogEvent::WriteFailed { code: *code },
                    DeviceLogEvent::Simulated { model_id } => LogEvent::Simulated { id: model_id.clone() },
                }),
            ),
        };
        Self { id: entry.id, time: entry.time, direction, bytes, event }
    }
}

/// The device log window.
pub struct DeviceLogModel {
    lines: RefCell<Vec<LogLine>>,
    log: DeviceLogUseCase,
    listeners: RefCell<Vec<Rc<dyn Fn()>>>,
}

impl DeviceLogModel {
    pub fn new(log: DeviceLogUseCase) -> Rc<Self> {
        Rc::new(Self { lines: RefCell::default(), log, listeners: RefCell::default() })
    }

    pub fn lines(&self) -> Vec<LogLine> {
        self.lines.borrow().clone()
    }

    pub fn clear(&self) {
        self.log.clear();
    }

    /// Calls `listener` after every change.
    pub fn subscribe(&self, listener: impl Fn() + 'static) {
        self.listeners.borrow_mut().push(Rc::new(listener));
    }

    /// Follows the log until the model goes away.
    pub fn run(self: &Rc<Self>) -> impl Future<Output = ()> + 'static {
        let model: Weak<Self> = Rc::downgrade(self);
        let mut entries = self.log.entries();
        async move {
            while let Some(entries) = entries.next().await {
                let Some(model) = model.upgrade() else { return };
                *model.lines.borrow_mut() = entries.iter().map(LogLine::of).collect();
                let listeners = model.listeners.borrow().clone();
                for listener in listeners {
                    listener();
                }
            }
        }
    }
}

impl fmt::Debug for DeviceLogModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceLogModel").field("lines", &self.lines.borrow().len()).finish_non_exhaustive()
    }
}
