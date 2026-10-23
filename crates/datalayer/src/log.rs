use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::SystemTime;

use domain::ports::{DeviceLogRepository, Updates};
use domain::{DeviceLogEntry, LogContent};

use crate::channel::ChannelUpdates;

/// The device log, kept in memory: what the connection and the device write, the log window reads. Lines can
/// be added from any thread.
#[derive(Clone, Debug, Default)]
pub struct DeviceLogStore {
    inner: Arc<Mutex<LogState>>,
}

#[derive(Debug, Default)]
struct LogState {
    items: VecDeque<DeviceLogEntry>,
    next_id: u64,
    subscribers: Vec<async_channel::Sender<Vec<DeviceLogEntry>>>,
}

impl DeviceLogStore {
    const LIMIT: usize = 2000;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&self, content: LogContent) {
        let mut state = self.lock();
        state.next_id += 1;
        let entry = DeviceLogEntry { id: state.next_id, time: SystemTime::now(), content };
        state.items.push_back(entry);
        while state.items.len() > Self::LIMIT {
            state.items.pop_front();
        }
        Self::publish(&mut state);
    }

    /// Every line, oldest first.
    pub fn snapshot(&self) -> Vec<DeviceLogEntry> {
        self.lock().items.iter().cloned().collect()
    }

    /// Each subscriber holds only the newest list: one that falls behind skips to it.
    fn publish(state: &mut LogState) {
        let items: Vec<DeviceLogEntry> = state.items.iter().cloned().collect();
        state.subscribers.retain(|subscriber| subscriber.force_send(items.clone()).is_ok());
    }

    fn lock(&self) -> MutexGuard<'_, LogState> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl DeviceLogRepository for DeviceLogStore {
    fn entries(&self) -> Box<dyn Updates<Vec<DeviceLogEntry>>> {
        let (sender, receiver) = async_channel::bounded(1);
        let mut state = self.lock();
        let _ = sender.force_send(state.items.iter().cloned().collect());
        state.subscribers.push(sender);
        Box::new(ChannelUpdates(receiver))
    }

    fn clear(&self) {
        let mut state = self.lock();
        state.items.clear();
        Self::publish(&mut state);
    }
}
