//! An open configuration interface on a thread of its own: writes and reads block there, and the main thread
//! waits on futures.

use std::sync::mpsc;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use domain::KeyboardDeviceError;
use domain::ports::LocalFuture;

use crate::channel::{ReportChannel, ReportFraming, ready};

/// The device file or USB handle a channel writes through. Error values are the code the log shows: an errno
/// for hidraw, a libusb error for USB.
pub(crate) trait Port: Send {
    fn write(&mut self, report: &[u8]) -> Result<(), i32>;

    /// The next answer of at least the length answers have, or `None` when none comes in time.
    fn read(&mut self, timeout: Duration) -> Option<Vec<u8>>;

    /// Drops what came before.
    fn discard(&mut self);
}

enum Job {
    Write(Vec<u8>, async_channel::Sender<Result<(), i32>>),
    Read(Duration, async_channel::Sender<Option<Vec<u8>>>),
    Discard,
}

/// The thread that owns the port, and the queue of jobs it works through in order.
#[derive(Debug)]
pub(crate) struct IoChannel {
    framing: ReportFraming,
    jobs: Mutex<Option<mpsc::Sender<Job>>>,
}

impl IoChannel {
    pub(crate) fn start(mut port: Box<dyn Port>, framing: ReportFraming) -> Arc<Self> {
        let (sender, receiver) = mpsc::channel::<Job>();
        std::thread::Builder::new()
            .name("keyboard-io".into())
            .spawn(move || {
                // Ends when the channel is closed and the queue is empty; the port closes with it.
                for job in receiver {
                    match job {
                        Job::Write(report, done) => {
                            let _ = done.send_blocking(port.write(&report));
                        }
                        Job::Read(timeout, done) => {
                            let _ = done.send_blocking(port.read(timeout));
                        }
                        Job::Discard => port.discard(),
                    }
                }
            })
            .expect("a thread for the keyboard");
        Arc::new(Self { framing, jobs: Mutex::new(Some(sender)) })
    }

    /// No more jobs: what is queued still runs, then the port closes.
    pub(crate) fn close(&self) {
        self.jobs.lock().unwrap_or_else(PoisonError::into_inner).take();
    }

    fn submit(&self, job: Job) -> bool {
        let jobs = self.jobs.lock().unwrap_or_else(PoisonError::into_inner);
        jobs.as_ref().is_some_and(|jobs| jobs.send(job).is_ok())
    }
}

/// The main thread's handle on an [`IoChannel`].
pub(crate) struct HardwareChannel(pub(crate) Arc<IoChannel>);

impl ReportChannel for HardwareChannel {
    fn framing(&self) -> ReportFraming {
        self.0.framing
    }

    fn write(&self, report: Vec<u8>) -> LocalFuture<'_, Result<(), KeyboardDeviceError>> {
        let (done, result) = async_channel::bounded(1);
        if !self.0.submit(Job::Write(report, done)) {
            return ready(Err(KeyboardDeviceError::NotConnected));
        }
        Box::pin(async move {
            match result.recv().await {
                Ok(Ok(())) => Ok(()),
                Ok(Err(code)) => Err(KeyboardDeviceError::WriteFailed { code }),
                Err(_) => Err(KeyboardDeviceError::NotConnected),
            }
        })
    }

    fn discard_pending_replies(&self) {
        self.0.submit(Job::Discard);
    }

    fn next_reply(&self, timeout: Duration) -> LocalFuture<'_, Option<Vec<u8>>> {
        let (done, result) = async_channel::bounded(1);
        if !self.0.submit(Job::Read(timeout, done)) {
            return ready(None);
        }
        Box::pin(async move { result.recv().await.ok().flatten() })
    }
}
