use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use domain::ports::LayoutProfileRepository;
use domain::{LayoutProfile, Rotation, UsbIdentity};

/// The layouts users chose, by keyboard, in a JSON file: `{"1189:8890": {"layout": "grid-6-3-1", "rotation": 2}}`.
/// A file that can't be read is treated as empty; a write that fails is dropped — the user chooses again.
#[derive(Clone, Debug)]
pub struct FileLayoutProfiles {
    path: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Stored {
    layout: String,
    /// Quarter turns clockwise.
    rotation: u8,
}

impl FileLayoutProfiles {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn load(&self) -> BTreeMap<String, Stored> {
        fs::read(&self.path).ok().and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
    }
}

impl LayoutProfileRepository for FileLayoutProfiles {
    fn profile(&self, keyboard: UsbIdentity) -> Option<LayoutProfile> {
        let stored = self.load().remove(&keyboard.to_string())?;
        Some(LayoutProfile { layout_id: stored.layout, rotation: Rotation::from_quarter_turns(stored.rotation.into()) })
    }

    fn save(&self, profile: LayoutProfile, keyboard: UsbIdentity) {
        let mut all = self.load();
        all.insert(
            keyboard.to_string(),
            Stored { layout: profile.layout_id, rotation: profile.rotation.quarter_turns() },
        );
        let Ok(json) = serde_json::to_vec_pretty(&all) else { return };
        if let Some(directory) = self.path.parent() {
            let _ = fs::create_dir_all(directory);
        }
        // Written beside and moved over, so a crash never leaves half a file.
        let partial = self.path.with_extension("json.partial");
        if fs::write(&partial, json).is_ok() {
            let _ = fs::rename(&partial, &self.path);
        }
    }
}
