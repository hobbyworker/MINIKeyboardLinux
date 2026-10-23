use std::fmt;
use std::rc::Rc;

use crate::capabilities::{KeyboardCapabilities, ReadSupport};
use crate::configuration::KeyboardConfiguration;
use crate::connection::{ConnectedKeyboard, ConnectionState, KeyboardDeviceError};
use crate::keyboards::{CommandSet, KeyboardFamily, KeyboardIdentification, RecordFormat};
use crate::layout::KeyboardLayout;
use crate::lighting::LightingSupport;
use crate::ports::{
    DeviceLogRepository, KeyboardConnectionRepository, KeyboardDeviceRepository, KeyboardSourceRepository, Updates,
};
use crate::support::{DeviceLogEntry, KeyboardSource};

/// Follows the keyboard as it comes and goes.
#[derive(Clone)]
pub struct ObserveConnectionUseCase {
    repository: Rc<dyn KeyboardConnectionRepository>,
}

impl ObserveConnectionUseCase {
    pub fn new(repository: Rc<dyn KeyboardConnectionRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self) -> Box<dyn Updates<ConnectionState>> {
        self.repository.connection_states()
    }
}

/// Looks for the keyboard again.
#[derive(Clone)]
pub struct RetryConnectionUseCase {
    repository: Rc<dyn KeyboardConnectionRepository>,
}

impl RetryConnectionUseCase {
    pub fn new(repository: Rc<dyn KeyboardConnectionRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self) {
        self.repository.retry();
    }
}

/// Settles the commands a newly connected keyboard takes. The 8890's report ID is in its descriptor, so it is
/// asked nothing; a 884x keyboard is asked who it is, up to `attempts` times, and one that never answers is
/// written in the `FE` form.
#[derive(Clone)]
pub struct IdentifyKeyboardUseCase {
    repository: Rc<dyn KeyboardDeviceRepository>,
    attempts: usize,
}

impl IdentifyKeyboardUseCase {
    pub fn new(repository: Rc<dyn KeyboardDeviceRepository>) -> Self {
        Self::with_attempts(repository, 3)
    }

    pub fn with_attempts(repository: Rc<dyn KeyboardDeviceRepository>, attempts: usize) -> Self {
        Self { repository, attempts: attempts.max(1) }
    }

    pub async fn execute(&self, keyboard: &ConnectedKeyboard) -> Result<KeyboardIdentification, KeyboardDeviceError> {
        match keyboard.family {
            KeyboardFamily::K8890 => Ok(KeyboardIdentification {
                commands: CommandSet::Packets { report_id: keyboard.report_id },
                identity: None,
            }),
            KeyboardFamily::K884x => {
                for _ in 0..self.attempts {
                    match self.repository.ask_identity().await {
                        Ok(Some(identity)) => {
                            return Ok(KeyboardIdentification {
                                commands: CommandSet::Records(identity.record_format()),
                                identity: Some(identity),
                            });
                        }
                        Ok(None) | Err(KeyboardDeviceError::InvalidReply) => continue,
                        Err(error) => return Err(error),
                    }
                }
                Ok(KeyboardIdentification { commands: CommandSet::Records(RecordFormat::Unidentified), identity: None })
            }
        }
    }
}

/// What reading left: the configuration with every layer that arrived, and why it stopped, if it did.
#[derive(Clone, Debug)]
pub struct ReadOutcome {
    pub configuration: KeyboardConfiguration,
    pub error: Option<KeyboardDeviceError>,
}

/// Reads what the keyboard holds into a configuration: every layer's actions and, on the standard format,
/// every layer's lights. A layer whose answers don't all arrive is left as it was.
#[derive(Clone)]
pub struct ReadKeyboardUseCase {
    repository: Rc<dyn KeyboardDeviceRepository>,
}

impl ReadKeyboardUseCase {
    pub fn new(repository: Rc<dyn KeyboardDeviceRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        configuration: KeyboardConfiguration,
        capabilities: KeyboardCapabilities,
        layout: &KeyboardLayout,
    ) -> ReadOutcome {
        let CommandSet::Records(format) = capabilities.commands else {
            return ReadOutcome { configuration, error: Some(KeyboardDeviceError::Unsupported) };
        };
        if capabilities.reading() == ReadSupport::Never {
            return ReadOutcome { configuration, error: Some(KeyboardDeviceError::Unsupported) };
        }
        let mut updated = configuration;
        let error = async {
            for layer in 0..updated.layer_count() {
                let actions = self.repository.read_actions(layer, format, layout).await?;
                updated.apply_read(actions, layer);
            }
            if matches!(capabilities.lighting(), LightingSupport::PerKeyColor { .. }) {
                for layer in 0..updated.layer_count() {
                    let lighting = self.repository.read_lighting(layer).await?;
                    updated.apply_read_lighting(lighting, layer);
                }
            }
            Ok::<(), KeyboardDeviceError>(())
        }
        .await
        .err();
        ReadOutcome { configuration: updated, error }
    }
}

/// What saving came to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveOutcome {
    Saved(KeyboardConfiguration),
    NothingToSave,
}

/// Writes what changed. On a failure the changes stay marked, so saving can be tried again.
#[derive(Clone)]
pub struct SaveKeyboardUseCase {
    repository: Rc<dyn KeyboardDeviceRepository>,
}

impl SaveKeyboardUseCase {
    pub fn new(repository: Rc<dyn KeyboardDeviceRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        configuration: &KeyboardConfiguration,
        capabilities: KeyboardCapabilities,
        layout: &KeyboardLayout,
    ) -> Result<SaveOutcome, KeyboardDeviceError> {
        if !configuration.has_unsaved_changes() {
            return Ok(SaveOutcome::NothingToSave);
        }
        self.repository.write(configuration, capabilities.commands, layout).await?;
        let mut saved = configuration.clone();
        saved.mark_saved();
        Ok(SaveOutcome::Saved(saved))
    }
}

/// The device log.
#[derive(Clone)]
pub struct DeviceLogUseCase {
    repository: Rc<dyn DeviceLogRepository>,
}

impl DeviceLogUseCase {
    pub fn new(repository: Rc<dyn DeviceLogRepository>) -> Self {
        Self { repository }
    }

    pub fn entries(&self) -> Box<dyn Updates<Vec<DeviceLogEntry>>> {
        self.repository.entries()
    }

    pub fn clear(&self) {
        self.repository.clear();
    }
}

/// Switches between the keyboard on USB and a simulated one.
#[derive(Clone)]
pub struct KeyboardSourceUseCase {
    repository: Rc<dyn KeyboardSourceRepository>,
}

impl KeyboardSourceUseCase {
    pub fn new(repository: Rc<dyn KeyboardSourceRepository>) -> Self {
        Self { repository }
    }

    pub fn current(&self) -> KeyboardSource {
        self.repository.current()
    }

    pub fn select(&self, source: KeyboardSource) {
        self.repository.select(source);
    }
}

macro_rules! opaque_debug {
    ($($name:ident),*) => {$(
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }
    )*};
}

opaque_debug!(
    ObserveConnectionUseCase,
    RetryConnectionUseCase,
    IdentifyKeyboardUseCase,
    ReadKeyboardUseCase,
    SaveKeyboardUseCase,
    DeviceLogUseCase,
    KeyboardSourceUseCase
);
