//! The open configuration interface, as the device repository uses it, and the streams of values the
//! repositories hand out.

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use domain::KeyboardDeviceError;
use domain::ports::{LocalFuture, Updates};

/// How payloads become reports on a keyboard's configuration interface: the report ID first unless it is 0,
/// then the payload, zero-padded to the report's length.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReportFraming {
    pub report_id: u8,
    /// Bytes per report, the report ID included.
    pub length: usize,
}

impl ReportFraming {
    pub fn frame(&self, payload: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.length.max(payload.len() + 1));
        if self.report_id != 0 {
            bytes.push(self.report_id);
        }
        bytes.extend_from_slice(payload);
        if bytes.len() < self.length {
            bytes.resize(self.length, 0);
        }
        bytes
    }
}

/// The open configuration interface: reports out, and the keyboard's answers in.
pub trait ReportChannel {
    fn framing(&self) -> ReportFraming;

    /// Writes one report, as framed.
    fn write(&self, report: Vec<u8>) -> LocalFuture<'_, Result<(), KeyboardDeviceError>>;

    /// Drops answers that came before a new request.
    fn discard_pending_replies(&self);

    /// The next answer, its report ID first, or `None` when none comes in time.
    fn next_reply(&self, timeout: Duration) -> LocalFuture<'_, Option<Vec<u8>>>;
}

/// Whoever has the channel open — the connection to the keyboard on USB, or the simulator.
pub trait ChannelProvider {
    fn open_channel(&self) -> Option<Rc<dyn ReportChannel>>;
}

/// Values from a channel, for a port's [`Updates`].
pub(crate) struct ChannelUpdates<T>(pub(crate) async_channel::Receiver<T>);

impl<T> Updates<T> for ChannelUpdates<T> {
    fn next(&mut self) -> LocalFuture<'_, Option<T>> {
        Box::pin(async move { self.0.recv().await.ok() })
    }
}

/// Keeps every subscriber of a value up to date, from any thread: the current value first, then each change.
#[derive(Debug)]
pub(crate) struct Broadcaster<T> {
    inner: Arc<Mutex<BroadcastState<T>>>,
}

#[derive(Debug)]
struct BroadcastState<T> {
    value: T,
    subscribers: Vec<async_channel::Sender<T>>,
}

impl<T> Clone for Broadcaster<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T: Clone + PartialEq + 'static> Broadcaster<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { inner: Arc::new(Mutex::new(BroadcastState { value, subscribers: vec![] })) }
    }

    pub(crate) fn value(&self) -> T {
        self.lock().value.clone()
    }

    /// Sends a changed value to every subscriber; the same value again sends nothing.
    pub(crate) fn set(&self, value: T) {
        let mut state = self.lock();
        if state.value == value {
            return;
        }
        state.value = value.clone();
        state.subscribers.retain(|subscriber| subscriber.try_send(value.clone()).is_ok());
    }

    pub(crate) fn subscribe(&self) -> Box<dyn Updates<T>> {
        let (sender, receiver) = async_channel::unbounded();
        let mut state = self.lock();
        let _ = sender.try_send(state.value.clone());
        state.subscribers.push(sender);
        Box::new(ChannelUpdates(receiver))
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, BroadcastState<T>> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// A future that is ready at once.
pub(crate) fn ready<'a, T: 'a>(value: T) -> LocalFuture<'a, T> {
    Box::pin(std::future::ready(value))
}

/// Waits off the main thread: a thread sleeps, and the future completes when it wakes.
pub fn sleep(duration: Duration) -> LocalFuture<'static, ()> {
    let (sender, receiver) = async_channel::bounded::<()>(1);
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        let _ = sender.send_blocking(());
    });
    Box::pin(async move {
        let _ = receiver.recv().await;
    })
}
