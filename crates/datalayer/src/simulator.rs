//! Keyboards that exist only in the app, for trying every keyboard without owning it. They take the same
//! reports a real one does, through the same codecs, and answer as their firmware would.

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use domain::ports::{KeyboardConnectionRepository, KeyboardSourceRepository, LocalFuture, Updates};
use domain::{
    Channel, ConnectedKeyboard, ConnectionState, DeviceLogEvent, KeyColor, KeyboardDeviceError, KeyboardSource,
    LightingSupport, LogContent, RecordFormat, SimulatedKind, SimulatedModel,
};

use crate::channel::{Broadcaster, ChannelProvider, ReportChannel, ReportFraming, ready};
use crate::connection::HardwareConnection;
use crate::log::DeviceLogStore;
use crate::protocols::k884x;

/// A keyboard that exists only in the app. A 884x keyboard that answers says who it is and reads back what
/// was written to it.
pub struct SimulatedKeyboard {
    model: SimulatedModel,
    framing: ReportFraming,
    state: RefCell<Firmware>,
}

#[derive(Default)]
struct Firmware {
    replies: VecDeque<Vec<u8>>,
    /// Records by layer (from 1) and record number, as written.
    records: HashMap<u8, HashMap<u8, Vec<u8>>>,
    /// Standard-format lights by layer (from 0): the mode and 16 colours, as written.
    lighting: HashMap<u8, Vec<u8>>,
    /// Every payload written, in order.
    written: Vec<Vec<u8>>,
}

impl SimulatedKeyboard {
    pub fn new(model: SimulatedModel) -> Self {
        let framing = match model.kind {
            SimulatedKind::K8890 { report_id } => {
                ReportFraming { report_id, length: if report_id == 0 { 64 } else { 65 } }
            }
            SimulatedKind::Identified(_) | SimulatedKind::Unidentified => {
                ReportFraming { report_id: k884x::REPORT_ID, length: k884x::REPORT_LENGTH }
            }
        };
        Self { model, framing, state: RefCell::default() }
    }

    pub fn keyboard(&self) -> ConnectedKeyboard {
        let family = self.model.family();
        ConnectedKeyboard {
            usb: self.model.usb,
            family,
            interface_number: family.configuration_interface(),
            channel: Channel::Simulated,
            report_id: self.framing.report_id,
            report_length: self.framing.length,
            details: None,
        }
    }

    /// Every payload written, in order.
    pub fn written(&self) -> Vec<Vec<u8>> {
        self.state.borrow().written.clone()
    }

    /// The record kept in `record` of `layer` (from 1), as written.
    pub fn record(&self, layer: u8, record: u8) -> Option<Vec<u8>> {
        self.state.borrow().records.get(&layer)?.get(&record).cloned()
    }

    fn receive(&self, report: &[u8]) {
        let payload = if self.framing.report_id == 0 { report } else { report.get(1..).unwrap_or_default() };
        let mut state = self.state.borrow_mut();
        state.written.push(payload.to_vec());
        let identity = match self.model.kind {
            SimulatedKind::Identified(identity) => identity,
            SimulatedKind::Unidentified => {
                if let [0xFE, record, layer, ..] = payload {
                    state.records.entry(*layer).or_default().insert(*record, payload.to_vec());
                }
                return;
            }
            SimulatedKind::K8890 { .. } => return,
        };
        let byte = |index: usize| payload.get(index).copied().unwrap_or(0);
        match (byte(0), byte(1)) {
            (0xFB, _) => self.reply(&mut state, &[0xFB, identity.key_count, identity.knob_count, identity.kind]),
            (0xFA, 0xB0) => {
                let lighting = state.lighting.get(&byte(2)).cloned().unwrap_or_else(Self::initial_lighting);
                self.reply(&mut state, &[&[0xB0][..], &lighting].concat());
            }
            (0xFA, _) => {
                let layer = byte(3);
                for id in 1..=k884x::read_reply_count(identity.record_format()) as u8 {
                    let record = state.records.get(&layer).and_then(|records| records.get(&id)).cloned();
                    let record = record.unwrap_or_else(|| Self::initial_record(id, layer));
                    self.reply(&mut state, &record);
                }
            }
            (0xFD, record) if record != 0xFE => {
                state.records.entry(byte(2)).or_default().insert(record, payload.to_vec());
            }
            (0xFE, 0xB0) if identity.record_format() == RecordFormat::Standard => {
                let end = 4 + 3 * LightingSupport::COLOR_KEY_COUNT;
                state.lighting.insert(byte(2), payload.get(3..end).unwrap_or_default().to_vec());
            }
            (0xFE, 0xB0) => {
                state.records.entry(byte(2)).or_default().insert(0, payload.to_vec());
            }
            _ => {}
        }
    }

    fn reply(&self, state: &mut Firmware, payload: &[u8]) {
        state.replies.push_back(self.framing.frame(payload));
    }

    /// What the cross-platform app starts a record as: `FD <record> <layer> 01`.
    fn initial_record(id: u8, layer: u8) -> Vec<u8> {
        let mut record = vec![0xFD, id, layer, 0x01];
        record.resize(64, 0);
        record
    }

    fn initial_lighting() -> Vec<u8> {
        std::iter::once(0)
            .chain(KeyColor::DEFAULT_KEY_COLORS.iter().flat_map(|color| [color.red, color.green, color.blue]))
            .collect()
    }
}

impl ReportChannel for SimulatedKeyboard {
    fn framing(&self) -> ReportFraming {
        self.framing
    }

    fn write(&self, report: Vec<u8>) -> LocalFuture<'_, Result<(), KeyboardDeviceError>> {
        self.receive(&report);
        ready(Ok(()))
    }

    fn discard_pending_replies(&self) {
        self.state.borrow_mut().replies.clear();
    }

    /// The firmware answers as soon as it is asked, so a reply that isn't there won't come.
    fn next_reply(&self, _timeout: Duration) -> LocalFuture<'_, Option<Vec<u8>>> {
        ready(self.state.borrow_mut().replies.pop_front())
    }
}

/// The one simulated keyboard, as the device repository's channels.
impl ChannelProvider for Rc<SimulatedKeyboard> {
    fn open_channel(&self) -> Option<Rc<dyn ReportChannel>> {
        Some(self.clone())
    }
}

impl fmt::Debug for SimulatedKeyboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimulatedKeyboard").field("model", &self.model).finish_non_exhaustive()
    }
}

/// The keyboard the app talks to, switchable while it runs: the one on USB or a simulated one. A switch is
/// reported as "searching" and then the new keyboard, so the app starts over as after re-plugging. The USB
/// connection is made the first time it is chosen — an app that stays on a simulated keyboard never opens a
/// real one.
pub struct KeyboardSourceSwitch {
    log: DeviceLogStore,
    source: RefCell<KeyboardSource>,
    hardware: RefCell<Option<HardwareConnection>>,
    simulated: RefCell<HashMap<SimulatedModel, Rc<SimulatedKeyboard>>>,
    state: Broadcaster<ConnectionState>,
    /// Whether the hardware connection's states are passed on.
    following_hardware: Arc<AtomicBool>,
}

impl KeyboardSourceSwitch {
    pub fn new(log: DeviceLogStore, initial: KeyboardSource) -> Self {
        let switch = Self {
            log,
            source: RefCell::new(KeyboardSource::Hardware),
            hardware: RefCell::default(),
            simulated: RefCell::default(),
            state: Broadcaster::new(ConnectionState::Searching),
            following_hardware: Arc::new(AtomicBool::new(false)),
        };
        switch.follow(&initial);
        *switch.source.borrow_mut() = initial;
        switch
    }

    fn follow(&self, source: &KeyboardSource) {
        match source {
            KeyboardSource::Hardware => {
                let mut hardware = self.hardware.borrow_mut();
                let connection = hardware.get_or_insert_with(|| {
                    let connection = HardwareConnection::start(self.log.clone());
                    connection.mirror_into(self.state.clone(), self.following_hardware.clone());
                    connection
                });
                self.following_hardware.store(true, Ordering::SeqCst);
                self.state.set(connection.state());
            }
            KeyboardSource::Simulated(model) => {
                self.following_hardware.store(false, Ordering::SeqCst);
                let keyboard = self.simulated_keyboard(*model);
                self.state.set(ConnectionState::Connected(keyboard.keyboard()));
            }
        }
    }

    fn simulated_keyboard(&self, model: SimulatedModel) -> Rc<SimulatedKeyboard> {
        self.simulated
            .borrow_mut()
            .entry(model)
            .or_insert_with(|| {
                self.log.append(LogContent::Event(DeviceLogEvent::Simulated { model_id: model.id() }));
                Rc::new(SimulatedKeyboard::new(model))
            })
            .clone()
    }
}

impl KeyboardConnectionRepository for KeyboardSourceSwitch {
    fn connection_states(&self) -> Box<dyn Updates<ConnectionState>> {
        self.state.subscribe()
    }

    fn retry(&self) {
        if let (KeyboardSource::Hardware, Some(hardware)) = (&*self.source.borrow(), &*self.hardware.borrow()) {
            hardware.retry();
        }
    }
}

impl KeyboardSourceRepository for KeyboardSourceSwitch {
    fn current(&self) -> KeyboardSource {
        self.source.borrow().clone()
    }

    fn select(&self, source: KeyboardSource) {
        if *self.source.borrow() == source {
            return;
        }
        self.following_hardware.store(false, Ordering::SeqCst);
        self.state.set(ConnectionState::Searching);
        self.follow(&source);
        *self.source.borrow_mut() = source;
    }
}

impl ChannelProvider for KeyboardSourceSwitch {
    fn open_channel(&self) -> Option<Rc<dyn ReportChannel>> {
        match &*self.source.borrow() {
            KeyboardSource::Hardware => self.hardware.borrow().as_ref()?.open_channel(),
            KeyboardSource::Simulated(model) => {
                self.simulated.borrow().get(model).map(|keyboard| keyboard.clone() as Rc<dyn ReportChannel>)
            }
        }
    }
}

impl fmt::Debug for KeyboardSourceSwitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyboardSourceSwitch").field("source", &self.source.borrow()).finish_non_exhaustive()
    }
}
