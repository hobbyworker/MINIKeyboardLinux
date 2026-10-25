//! The keyboards as the domain's ports see them: their commands byte for byte, finding them on USB and
//! writing to them through libusb or hidraw, the device log, the user's layouts on disk, and simulated keyboards
//! that answer as their firmware would.

mod channel;
mod connection;
pub mod descriptors;
mod device;
mod log;
mod profiles;
pub mod protocols;
mod simulator;

pub use channel::{ChannelProvider, ReportChannel, ReportFraming, sleep};
pub use connection::HardwareConnection;
pub use device::{KeyboardDevice, Pause};
pub use log::DeviceLogStore;
pub use profiles::FileLayoutProfiles;
pub use simulator::{KeyboardSourceSwitch, SimulatedKeyboard};
