//! Finds a supported keyboard and keeps its configuration interface open, looking again every second as the
//! vendor tools do.
//!
//! - The 1189:8890's interface 1 has no driver; it is opened over USB through libusb ([`usb`]).
//! - A 884x keyboard's interface 0 is a HID device, opened through its hidraw device ([`hidraw`]), as the
//!   cross-platform app opens the first HID entry on interface 0.
//!
//! The looking happens on a thread of its own; the state reaches the main thread through
//! [`connection_states`](KeyboardConnectionRepository::connection_states).

mod hidraw;
mod io;
mod sysfs;
mod usb;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use domain::ports::{KeyboardConnectionRepository, Updates};
use domain::{Channel, ConnectedKeyboard, ConnectionState, DeviceLogEvent, LogContent};

use crate::channel::{Broadcaster, ChannelProvider, ReportChannel, ReportFraming};
use crate::log::DeviceLogStore;
use crate::protocols::k884x;

use io::{HardwareChannel, IoChannel};
use sysfs::{Candidate, CandidateKind};

/// How often the keyboards plugged in are looked at.
const POLL: Duration = Duration::from_secs(1);
/// How soon a keyboard the user may not open is tried again — a udev rule may have been added since.
const PERMISSION_RETRY: Duration = Duration::from_secs(3);

/// The connection to the keyboard on USB.
pub struct HardwareConnection {
    state: Broadcaster<ConnectionState>,
    open: Arc<Mutex<Option<Arc<IoChannel>>>>,
    /// The simulator follows the connection through these (`simulator.rs`).
    mirrors: Arc<Mutex<Vec<Mirror>>>,
    commands: mpsc::Sender<Command>,
}

/// Another broadcaster that takes the same states while it follows this connection.
struct Mirror {
    target: Broadcaster<ConnectionState>,
    active: Arc<AtomicBool>,
}

enum Command {
    Retry,
}

impl HardwareConnection {
    /// Starts looking for a keyboard.
    pub fn start(log: DeviceLogStore) -> Self {
        let state = Broadcaster::new(ConnectionState::Searching);
        let open = Arc::new(Mutex::new(None));
        let mirrors = Arc::new(Mutex::new(Vec::new()));
        let (commands, receiver) = mpsc::channel();
        let mut watcher = Watcher {
            log,
            state: state.clone(),
            open: open.clone(),
            mirrors: mirrors.clone(),
            connected: None,
            failed: HashMap::new(),
        };
        std::thread::Builder::new()
            .name("keyboard-connection".into())
            .spawn(move || {
                watcher.refresh();
                loop {
                    match receiver.recv_timeout(POLL) {
                        Ok(Command::Retry) => watcher.retry(),
                        Err(RecvTimeoutError::Timeout) => watcher.refresh(),
                        Err(RecvTimeoutError::Disconnected) => break,
                    }
                }
                watcher.disconnect();
            })
            .expect("a thread for the connection");
        Self { state, open, mirrors, commands }
    }

    pub fn state(&self) -> ConnectionState {
        self.state.value()
    }

    pub(crate) fn mirror_into(&self, target: Broadcaster<ConnectionState>, active: Arc<AtomicBool>) {
        self.mirrors.lock().unwrap_or_else(PoisonError::into_inner).push(Mirror { target, active });
    }
}

impl KeyboardConnectionRepository for HardwareConnection {
    fn connection_states(&self) -> Box<dyn Updates<ConnectionState>> {
        self.state.subscribe()
    }

    fn retry(&self) {
        let _ = self.commands.send(Command::Retry);
    }
}

impl ChannelProvider for HardwareConnection {
    fn open_channel(&self) -> Option<Rc<dyn ReportChannel>> {
        let open = self.open.lock().unwrap_or_else(PoisonError::into_inner);
        open.as_ref().map(|channel| Rc::new(HardwareChannel(channel.clone())) as Rc<dyn ReportChannel>)
    }
}

impl fmt::Debug for HardwareConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HardwareConnection").field("state", &self.state.value()).finish_non_exhaustive()
    }
}

/// Why a place to write didn't open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Failure {
    NotPermitted(Instant),
    Other,
}

/// The connection's thread.
struct Watcher {
    log: DeviceLogStore,
    state: Broadcaster<ConnectionState>,
    open: Arc<Mutex<Option<Arc<IoChannel>>>>,
    mirrors: Arc<Mutex<Vec<Mirror>>>,
    connected: Option<String>,
    failed: HashMap<String, Failure>,
}

impl Watcher {
    fn set_state(&self, new: ConnectionState) {
        if new == self.state.value() {
            return;
        }
        let event = match &new {
            ConnectionState::Searching => DeviceLogEvent::Searching,
            ConnectionState::Connected(keyboard) => {
                DeviceLogEvent::Connected { usb: keyboard.usb, interface: keyboard.interface_number }
            }
            ConnectionState::NeedsPermission => DeviceLogEvent::NeedsPermission,
            ConnectionState::OpenFailed { code } => DeviceLogEvent::OpenFailed { code: *code },
        };
        self.log.append(LogContent::Event(event));
        self.state.set(new.clone());
        for mirror in self.mirrors.lock().unwrap_or_else(PoisonError::into_inner).iter() {
            if mirror.active.load(Ordering::SeqCst) {
                mirror.target.set(new.clone());
            }
        }
    }

    fn retry(&mut self) {
        self.failed.clear();
        if self.connected.is_none() {
            self.set_state(ConnectionState::Searching);
        }
        self.refresh();
    }

    fn refresh(&mut self) {
        let candidates = sysfs::candidates();
        let present: HashSet<&str> = candidates.iter().map(|candidate| candidate.entry.as_str()).collect();
        self.failed.retain(|entry, _| present.contains(entry.as_str()));

        if let Some(connected) = &self.connected {
            if present.contains(connected.as_str()) {
                return;
            }
            self.disconnect();
        }
        let now = Instant::now();
        let usable = |candidate: &&Candidate| match self.failed.get(&candidate.entry) {
            None => true,
            Some(Failure::NotPermitted(when)) => now.duration_since(*when) >= PERMISSION_RETRY,
            Some(Failure::Other) => false,
        };
        let Some(candidate) = candidates.iter().find(usable).cloned() else {
            // A failure stays on screen while that keyboard is still plugged in.
            let failing =
                matches!(self.state.value(), ConnectionState::NeedsPermission | ConnectionState::OpenFailed { .. });
            if !(failing && !present.is_empty()) {
                self.set_state(ConnectionState::Searching);
            }
            return;
        };
        self.open(candidate);
    }

    fn open(&mut self, candidate: Candidate) {
        let opened = match &candidate.kind {
            CandidateKind::Usb { bus, address } => {
                let configuration = sysfs::configuration_descriptor(&candidate.device).unwrap_or_default();
                match usb::open(*bus, *address, candidate.interface_number, &configuration) {
                    Ok(opened) => {
                        let channel = Channel::Usb { out_endpoint: opened.out_endpoint };
                        Ok((Box::new(opened.port) as Box<dyn io::Port>, opened.framing, channel, opened.max_power))
                    }
                    Err(usb::OpenError::NotConfiguration) => Err(None),
                    Err(usb::OpenError::NotPermitted) => Err(Some(ConnectionState::NeedsPermission)),
                    Err(usb::OpenError::Failed(code)) => Err(Some(ConnectionState::OpenFailed { code })),
                }
            }
            CandidateKind::Hidraw { node } => match hidraw::HidrawPort::open(node) {
                Ok(port) => {
                    let framing = ReportFraming { report_id: k884x::REPORT_ID, length: k884x::REPORT_LENGTH };
                    Ok((Box::new(port) as Box<dyn io::Port>, framing, Channel::Hid, None))
                }
                Err(hidraw::OpenError::NotPermitted) => Err(Some(ConnectionState::NeedsPermission)),
                Err(hidraw::OpenError::Failed(code)) => Err(Some(ConnectionState::OpenFailed { code })),
            },
        };
        match opened {
            Ok((port, framing, channel, max_power)) => {
                let mut details = sysfs::device_details(&candidate.device);
                details.max_power = max_power.or(details.max_power);
                let keyboard = ConnectedKeyboard {
                    usb: candidate.usb,
                    family: candidate.family,
                    interface_number: candidate.interface_number,
                    channel,
                    report_id: framing.report_id,
                    report_length: framing.length,
                    details: Some(details),
                };
                *self.open.lock().unwrap_or_else(PoisonError::into_inner) = Some(IoChannel::start(port, framing));
                self.connected = Some(candidate.entry);
                self.set_state(ConnectionState::Connected(keyboard));
            }
            Err(None) => {
                self.failed.insert(candidate.entry, Failure::Other);
                self.set_state(ConnectionState::Searching);
            }
            Err(Some(state)) => {
                let failure = if state == ConnectionState::NeedsPermission {
                    Failure::NotPermitted(Instant::now())
                } else {
                    Failure::Other
                };
                self.failed.insert(candidate.entry, failure);
                self.set_state(state);
            }
        }
    }

    fn disconnect(&mut self) {
        if let Some(channel) = self.open.lock().unwrap_or_else(PoisonError::into_inner).take() {
            channel.close();
        }
        if self.connected.take().is_some() {
            self.set_state(ConnectionState::Searching);
        }
    }
}
