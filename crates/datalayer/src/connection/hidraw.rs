//! A 884x keyboard's interface 0, opened as its hidraw device: requests go out as output reports, answers
//! come in as input reports. The interface is also the keyboard's keyboard; reading its hidraw device
//! doesn't take the typing away from the system.

use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::os::fd::AsRawFd;
use std::path::Path;
use std::time::{Duration, Instant};

use super::io::Port;

/// Answers are 64-byte reports; shorter ones are the keyboard's typing, media keys or mouse.
const MINIMUM_REPLY_LENGTH: usize = 32;

pub(crate) struct HidrawPort {
    file: File,
}

/// Why a device file didn't open.
pub(crate) enum OpenError {
    /// The signed-in user may not open it — no udev rule gives it to them.
    NotPermitted,
    Failed(i32),
}

impl HidrawPort {
    pub(crate) fn open(path: &Path) -> Result<Self, OpenError> {
        match File::options().read(true).write(true).open(path) {
            Ok(file) => Ok(Self { file }),
            Err(error) if error.kind() == ErrorKind::PermissionDenied => Err(OpenError::NotPermitted),
            Err(error) => Err(OpenError::Failed(error.raw_os_error().unwrap_or(-1))),
        }
    }
}

impl Port for HidrawPort {
    /// A numbered report keeps its ID as the first byte, as hidraw takes it.
    fn write(&mut self, report: &[u8]) -> Result<(), i32> {
        match self.file.write(report) {
            Ok(written) if written == report.len() => Ok(()),
            Ok(_) => Err(libc::EIO),
            Err(error) => Err(error.raw_os_error().unwrap_or(-1)),
        }
    }

    fn read(&mut self, timeout: Duration) -> Option<Vec<u8>> {
        let deadline = Instant::now() + timeout;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            if !wait_readable(&self.file, left) {
                return None;
            }
            let mut buffer = [0u8; 256];
            let length = self.file.read(&mut buffer).ok()?;
            if length >= MINIMUM_REPLY_LENGTH {
                return Some(buffer[..length].to_vec());
            }
        }
    }

    fn discard(&mut self) {
        let mut buffer = [0u8; 256];
        while wait_readable(&self.file, Duration::ZERO) {
            if self.file.read(&mut buffer).is_err() {
                return;
            }
        }
    }
}

/// Whether the file has something to read within `timeout`.
#[allow(unsafe_code)]
fn wait_readable(file: &File, timeout: Duration) -> bool {
    let mut poll = libc::pollfd { fd: file.as_raw_fd(), events: libc::POLLIN, revents: 0 };
    let milliseconds = i32::try_from(timeout.as_millis()).unwrap_or(i32::MAX);
    // SAFETY: `poll` is one valid pollfd for an open file, and the count says one.
    let ready = unsafe { libc::poll(&mut poll, 1, milliseconds) };
    ready > 0 && poll.revents & libc::POLLIN != 0
}
