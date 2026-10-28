use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::rc::Rc;

use crate::capabilities::{KeyboardCapabilities, ReadSupport};
use crate::configuration::KeyboardConfiguration;
use crate::connection::{AttachedKeyboards, ConnectedKeyboard, ConnectionState, KeyboardDeviceError};
use crate::keyboards::{CommandSet, KeyboardFamily, KeyboardIdentification, RecordFormat};
use crate::layout::{ControlId, KeyboardLayout};
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

/// Follows the supported keyboards plugged in and the one the app opens.
#[derive(Clone)]
pub struct ObserveAttachedKeyboardsUseCase {
    repository: Rc<dyn KeyboardConnectionRepository>,
}

impl ObserveAttachedKeyboardsUseCase {
    pub fn new(repository: Rc<dyn KeyboardConnectionRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self) -> Box<dyn Updates<AttachedKeyboards>> {
        self.repository.attached_keyboards()
    }
}

/// Opens another of the keyboards plugged in.
#[derive(Clone)]
pub struct SelectKeyboardUseCase {
    repository: Rc<dyn KeyboardConnectionRepository>,
}

impl SelectKeyboardUseCase {
    pub fn new(repository: Rc<dyn KeyboardConnectionRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: &str) {
        self.repository.select_keyboard(id);
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

/// What reading left: the configuration with every layer that arrived, the controls whose answers didn't, and
/// why it stopped, if it did.
#[derive(Clone, Debug)]
pub struct ReadOutcome {
    pub configuration: KeyboardConfiguration,
    /// The layout's controls that weren't read, by layer — only layers with some; empty when everything came.
    pub unread: BTreeMap<usize, BTreeSet<ControlId>>,
    pub error: Option<KeyboardDeviceError>,
}

/// Reads what the keyboard holds into a configuration: every layer's actions and, on the standard format,
/// every layer's lights. Controls whose answers don't come are left as they were, and the read goes on to the
/// next layer; an error ends it, with the layers read before applied.
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
        let format = match capabilities.commands {
            CommandSet::Records(format) if capabilities.reading() != ReadSupport::Never => format,
            _ => {
                let error = Some(KeyboardDeviceError::Unsupported);
                return ReadOutcome { configuration, unread: BTreeMap::new(), error };
            }
        };
        let mut updated = configuration;
        let mut unread = BTreeMap::new();
        let error = async {
            for layer in 0..updated.layer_count() {
                let read = self.repository.read_actions(layer, format, layout).await?;
                updated.apply_read(read.actions, layer);
                if !read.unread.is_empty() {
                    unread.insert(layer, read.unread);
                }
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
        ReadOutcome { configuration: updated, unread, error }
    }
}

/// What saving came to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveOutcome {
    /// Written, and what reading it back found. Every change is marked saved but those that read back otherwise.
    Saved(KeyboardConfiguration, SaveCheck),
    NothingToSave,
}

/// What reading back after a save found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveCheck {
    /// The keyboard doesn't read back.
    NotChecked,
    /// Everything written reads back as the keyboard's records hold it.
    Matched,
    /// The keyboard holds these otherwise: controls by layer, and the layers whose lights differ.
    Differs { controls: BTreeMap<usize, BTreeSet<ControlId>>, lighting: BTreeSet<usize> },
    /// Nothing differs, but something written couldn't be read back to compare.
    CouldNotCheck,
}

/// Writes what changed, then reads it back on a keyboard that reads back. On a failed write the changes stay
/// marked, so saving can be tried again; changes the keyboard holds otherwise than written stay marked too.
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
        let check = self.check(configuration, capabilities, layout).await;
        let mut saved = configuration.clone();
        match &check {
            SaveCheck::Differs { controls, lighting } => saved.mark_saved_except(controls, lighting),
            SaveCheck::NotChecked | SaveCheck::Matched | SaveCheck::CouldNotCheck => saved.mark_saved(),
        }
        Ok(SaveOutcome::Saved(saved, check))
    }

    /// Reads back the layers written, on a keyboard of the standard or legacy format: each changed control against
    /// its action as the keyboard's records hold it, and — where the keyboard reads lights back — each changed
    /// layer's lights. A layer whose read fails can't be told; a control outside the read isn't compared. With
    /// nothing to compare, it matches.
    async fn check(
        &self,
        configuration: &KeyboardConfiguration,
        capabilities: KeyboardCapabilities,
        layout: &KeyboardLayout,
    ) -> SaveCheck {
        let CommandSet::Records(format @ (RecordFormat::Standard | RecordFormat::Legacy)) = capabilities.commands
        else {
            return SaveCheck::NotChecked;
        };
        if capabilities.reading() == ReadSupport::Never {
            return SaveCheck::NotChecked;
        }
        let mut controls: BTreeMap<usize, BTreeSet<ControlId>> = BTreeMap::new();
        let mut lighting = BTreeSet::new();
        let mut unchecked = false;
        for (index, layer) in configuration.layers().iter().enumerate() {
            // Only the layout's controls were written.
            let changed: Vec<ControlId> =
                layer.changed_controls().iter().copied().filter(|control| layout.has(*control)).collect();
            if changed.is_empty() {
                continue;
            }
            let Ok(read) = self.repository.read_actions(index, format, layout).await else {
                unchecked = true;
                continue;
            };
            for control in changed {
                let read_back = read.actions.get(&control);
                if read_back.is_none() && !read.unread.contains(&control) {
                    continue;
                }
                let expected =
                    configuration.action(control, index).and_then(|action| self.repository.stored_form(action, format));
                match (expected, read_back) {
                    (Some(expected), Some(read_back)) if *read_back != expected => {
                        controls.entry(index).or_default().insert(control);
                    }
                    (Some(_), Some(_)) => {}
                    // Unread, or not a form the records can hold.
                    _ => unchecked = true,
                }
            }
        }
        if matches!(capabilities.lighting(), LightingSupport::PerKeyColor { .. }) {
            for (index, layer) in configuration.layers().iter().enumerate() {
                let Some(written) = configuration.lighting(index).filter(|_| layer.lighting_changed()) else {
                    continue;
                };
                match self.repository.read_lighting(index).await {
                    Ok(read) if read == *written => {}
                    Ok(_) => {
                        lighting.insert(index);
                    }
                    Err(_) => unchecked = true,
                }
            }
        }
        if !controls.is_empty() || !lighting.is_empty() {
            SaveCheck::Differs { controls, lighting }
        } else if unchecked {
            SaveCheck::CouldNotCheck
        } else {
            SaveCheck::Matched
        }
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
    ObserveAttachedKeyboardsUseCase,
    SelectKeyboardUseCase,
    RetryConnectionUseCase,
    IdentifyKeyboardUseCase,
    ReadKeyboardUseCase,
    SaveKeyboardUseCase,
    DeviceLogUseCase,
    KeyboardSourceUseCase
);
