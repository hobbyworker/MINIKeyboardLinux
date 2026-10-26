//! Finds the supported keyboards plugged in and keeps one's configuration interface open — the one chosen
//! ([`choice`]) — looking again every second as the vendor tools do. The others are left alone, so another app can
//! have them.
//!
//! - The 1189:8890's interface 1 has no driver; it is opened over USB through libusb ([`usb`]).
//! - A 884x keyboard's interface 0 is a HID device, opened through its hidraw device ([`hidraw`]), as the
//!   cross-platform app opens the first HID entry on interface 0.
//!
//! The looking happens on a thread of its own; the state reaches the main thread through
//! [`connection_states`](KeyboardConnectionRepository::connection_states).

mod choice;
mod hidraw;
mod io;
mod sysfs;
mod usb;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use domain::ports::{KeyboardConnectionRepository, Updates};
use domain::{
    AttachedKeyboard, AttachedKeyboards, Channel, ConnectedKeyboard, ConnectionState, DeviceLogEvent, LogContent,
};

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
    attached: Broadcaster<AttachedKeyboards>,
    open: Arc<Mutex<Option<Arc<IoChannel>>>>,
    /// The simulator follows the connection through these (`simulator.rs`).
    mirrors: Arc<Mutex<Vec<Mirror>>>,
    commands: mpsc::Sender<Command>,
}

/// Other broadcasters that take the same values while they follow this connection.
pub(crate) struct Mirror {
    pub(crate) state: Broadcaster<ConnectionState>,
    pub(crate) attached: Broadcaster<AttachedKeyboards>,
    pub(crate) active: Arc<AtomicBool>,
}

enum Command {
    Retry,
    Select(String),
}

impl HardwareConnection {
    /// Starts looking for keyboards. The one chosen is kept in `selected_keyboard`, to be opened again next time.
    pub fn start(log: DeviceLogStore, selected_keyboard: PathBuf) -> Self {
        let state = Broadcaster::new(ConnectionState::Searching);
        let attached = Broadcaster::new(AttachedKeyboards::default());
        let open = Arc::new(Mutex::new(None));
        let mirrors = Arc::new(Mutex::new(Vec::new()));
        let (commands, receiver) = mpsc::channel();
        let mut watcher = Watcher {
            log,
            state: state.clone(),
            attached: attached.clone(),
            open: open.clone(),
            mirrors: mirrors.clone(),
            selected_keyboard,
            connected: None,
            connected_keyboard: None,
            chosen: None,
            failed: HashMap::new(),
        };
        std::thread::Builder::new()
            .name("keyboard-connection".into())
            .spawn(move || {
                watcher.refresh();
                loop {
                    match receiver.recv_timeout(POLL) {
                        Ok(Command::Retry) => watcher.retry(),
                        Ok(Command::Select(id)) => watcher.select(id),
                        Err(RecvTimeoutError::Timeout) => watcher.refresh(),
                        Err(RecvTimeoutError::Disconnected) => break,
                    }
                }
                watcher.close_channel();
            })
            .expect("a thread for the connection");
        Self { state, attached, open, mirrors, commands }
    }

    pub fn state(&self) -> ConnectionState {
        self.state.value()
    }

    pub fn attached(&self) -> AttachedKeyboards {
        self.attached.value()
    }

    pub(crate) fn mirror_into(&self, mirror: Mirror) {
        self.mirrors.lock().unwrap_or_else(PoisonError::into_inner).push(mirror);
    }
}

impl KeyboardConnectionRepository for HardwareConnection {
    fn connection_states(&self) -> Box<dyn Updates<ConnectionState>> {
        self.state.subscribe()
    }

    fn retry(&self) {
        let _ = self.commands.send(Command::Retry);
    }

    fn attached_keyboards(&self) -> Box<dyn Updates<AttachedKeyboards>> {
        self.attached.subscribe()
    }

    fn select_keyboard(&self, id: &str) {
        let _ = self.commands.send(Command::Select(id.to_owned()));
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
    attached: Broadcaster<AttachedKeyboards>,
    open: Arc<Mutex<Option<Arc<IoChannel>>>>,
    mirrors: Arc<Mutex<Vec<Mirror>>>,
    /// Where the keyboard chosen last time is kept ([`choice::key`]).
    selected_keyboard: PathBuf,
    /// The place to write that is open.
    connected: Option<String>,
    /// The keyboard it is on.
    connected_keyboard: Option<String>,
    /// The one the user chose this session; `None` once it is unplugged — the app doesn't go back to it by itself.
    chosen: Option<String>,
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
                mirror.state.set(new.clone());
            }
        }
    }

    fn publish(&self, keyboards: Vec<AttachedKeyboard>, selected: Option<String>) {
        let attached = AttachedKeyboards { keyboards, selected };
        self.attached.set(attached.clone());
        for mirror in self.mirrors.lock().unwrap_or_else(PoisonError::into_inner).iter() {
            if mirror.active.load(Ordering::SeqCst) {
                mirror.attached.set(attached.clone());
            }
        }
    }

    fn select(&mut self, id: String) {
        let Some(keyboard) = self.attached.value().keyboards.into_iter().find(|keyboard| keyboard.id == id) else {
            return;
        };
        self.chosen = Some(id.clone());
        if let Some(directory) = self.selected_keyboard.parent() {
            let _ = fs::create_dir_all(directory);
        }
        // A write that fails is dropped — the first keyboard opens next time.
        let _ = fs::write(&self.selected_keyboard, choice::key(&keyboard));
        if self.connected_keyboard.as_deref() == Some(id.as_str()) {
            return;
        }
        self.close_channel();
        self.failed.clear();
        self.set_state(ConnectionState::Searching);
        self.refresh();
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
        let keyboards =
            choice::ordered(&candidates.iter().map(|candidate| candidate.keyboard.clone()).collect::<Vec<_>>());
        if self.chosen.as_ref().is_some_and(|chosen| !keyboards.iter().any(|keyboard| &keyboard.id == chosen)) {
            self.chosen = None;
        }

        if let Some(connected) = &self.connected {
            if present.contains(connected.as_str()) {
                self.publish(keyboards, self.connected_keyboard.clone());
                return;
            }
            self.close_channel();
            self.set_state(ConnectionState::Searching);
        }
        let now = Instant::now();
        let usable = |candidate: &&Candidate| match self.failed.get(&candidate.entry) {
            None => true,
            Some(Failure::NotPermitted(when)) => now.duration_since(*when) >= PERMISSION_RETRY,
            Some(Failure::Other) => false,
        };
        let remembered = fs::read_to_string(&self.selected_keyboard).ok();
        let target =
            choice::target(&keyboards, self.chosen.as_deref(), remembered.as_deref().map(str::trim), |keyboard| {
                candidates.iter().any(|candidate| candidate.keyboard.id == keyboard.id && usable(&candidate))
            });
        let selected = target.as_ref().map(|target| target.id.clone());
        let candidate = target.and_then(|target| {
            candidates.iter().find(|candidate| candidate.keyboard.id == target.id && usable(candidate)).cloned()
        });
        self.publish(keyboards, selected);
        let Some(candidate) = candidate else {
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
                self.connected_keyboard = Some(candidate.keyboard.id);
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

    fn close_channel(&mut self) {
        if let Some(channel) = self.open.lock().unwrap_or_else(PoisonError::into_inner).take() {
            channel.close();
        }
        self.connected = None;
        self.connected_keyboard = None;
    }
}
