//! The 1189:8890's configuration interface, written over USB directly. Interface 1 is HID class with only an
//! OUT endpoint, so no HID driver takes it and there is no hidraw device to write; the Windows tool writes
//! the same reports to it through its HID stack. Nothing comes back.

use std::time::Duration;

use rusb::UsbContext;

use crate::channel::ReportFraming;
use crate::descriptors;

use super::io::Port;

/// The Windows tool's write timeout (`HidLib.WriteDevice`).
const WRITE_TIMEOUT: Duration = Duration::from_millis(500);

pub(crate) struct UsbPort {
    handle: rusb::DeviceHandle<rusb::Context>,
    endpoint: u8,
}

/// What opening the interface found.
pub(crate) struct Opened {
    pub(crate) port: UsbPort,
    pub(crate) framing: ReportFraming,
    pub(crate) out_endpoint: u8,
    /// `bMaxPower` in mA.
    pub(crate) max_power: Option<u32>,
}

pub(crate) enum OpenError {
    /// Not the configuration interface after all — its reports are not vendor-defined.
    NotConfiguration,
    /// The signed-in user may not open the device — no udev rule gives it to them.
    NotPermitted,
    Failed(i32),
}

/// Opens interface `number` of the device at `bus`/`address` and reads what writing needs from its
/// descriptors: the interrupt OUT endpoint, and the output report's ID and length. Reads only — nothing is
/// written to the keyboard.
pub(crate) fn open(bus: u8, address: u8, number: u8, configuration: &[u8]) -> Result<Opened, OpenError> {
    let summary =
        descriptors::interface(number, configuration).ok_or(OpenError::Failed(code(rusb::Error::NotFound)))?;
    let endpoint = summary.out_endpoint().ok_or(OpenError::Failed(code(rusb::Error::NotSupported)))?;
    let report_length = summary.report_descriptor_length.filter(|length| *length > 0);
    let report_length = report_length.ok_or(OpenError::Failed(code(rusb::Error::NotSupported)))?;

    let context = rusb::Context::new().map_err(failed)?;
    let device = context
        .devices()
        .map_err(failed)?
        .iter()
        .find(|device| device.bus_number() == bus && device.address() == address)
        .ok_or(OpenError::Failed(code(rusb::Error::NoDevice)))?;
    let handle = device.open().map_err(failed)?;
    // Where a kernel driver took the interface after all, it is given back when the app lets go.
    let _ = handle.set_auto_detach_kernel_driver(true);
    handle.claim_interface(number).map_err(failed)?;

    // GET_DESCRIPTOR for the interface's HID report descriptor — a standard read request.
    let mut report_descriptor = vec![0u8; usize::from(report_length)];
    let read = handle
        .read_control(0x81, 6, 0x22 << 8, u16::from(number), &mut report_descriptor, Duration::from_secs(1))
        .map_err(failed)?;
    report_descriptor.truncate(read);
    if descriptors::usage_page(&report_descriptor).unwrap_or(0) < 0xFF00 {
        return Err(OpenError::NotConfiguration);
    }
    let output = descriptors::first_output_report(&report_descriptor)
        .ok_or(OpenError::Failed(code(rusb::Error::NotSupported)))?;
    let framing = ReportFraming { report_id: output.id, length: output.length + usize::from(output.id != 0) };
    // bMaxPower is byte 8 of the configuration descriptor, in units of 2 mA.
    let max_power = configuration.get(8).map(|units| u32::from(*units) * 2);
    Ok(Opened {
        port: UsbPort { handle, endpoint: endpoint.address },
        framing,
        out_endpoint: endpoint.address,
        max_power,
    })
}

impl Port for UsbPort {
    fn write(&mut self, report: &[u8]) -> Result<(), i32> {
        match self.handle.write_interrupt(self.endpoint, report, WRITE_TIMEOUT) {
            Ok(written) if written == report.len() => Ok(()),
            Ok(_) => Err(code(rusb::Error::Io)),
            Err(error) => Err(code(error)),
        }
    }

    /// The interface has no IN endpoint.
    fn read(&mut self, _timeout: Duration) -> Option<Vec<u8>> {
        None
    }

    fn discard(&mut self) {}
}

fn failed(error: rusb::Error) -> OpenError {
    match error {
        rusb::Error::Access => OpenError::NotPermitted,
        error => OpenError::Failed(code(error)),
    }
}

/// libusb's error codes.
pub(crate) fn code(error: rusb::Error) -> i32 {
    match error {
        rusb::Error::Io => -1,
        rusb::Error::InvalidParam => -2,
        rusb::Error::Access => -3,
        rusb::Error::NoDevice => -4,
        rusb::Error::NotFound => -5,
        rusb::Error::Busy => -6,
        rusb::Error::Timeout => -7,
        rusb::Error::Overflow => -8,
        rusb::Error::Pipe => -9,
        rusb::Error::Interrupted => -10,
        rusb::Error::NoMem => -11,
        rusb::Error::NotSupported => -12,
        rusb::Error::BadDescriptor | rusb::Error::Other => -99,
    }
}
