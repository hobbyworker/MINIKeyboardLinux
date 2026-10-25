//! What the user chose for the interface itself — its language, light or dark, how the Keys page lays out
//! the keys — kept in a small JSON file beside the app's other settings.

use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Light, dark or the system's, for the app's windows. Switching is live.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Appearance {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Choices {
    /// A language code ("ko", "pt-BR"), or `None` to follow the system.
    pub language: Option<String>,
    pub appearance: Appearance,
    /// "qwerty" or "abc", or `None` until the user picks one.
    pub key_arrangement: Option<String>,
    /// Whether a build with an update feed looks at it by itself — at launch and once a day.
    pub check_for_updates: bool,
    /// Whether the 1189:8890's lighting save ends with the save that stops its key repeat (the vendor's tool doesn't
    /// send it).
    pub prevent_lighting_key_repeat: bool,
}

impl Default for Choices {
    fn default() -> Self {
        Self {
            language: None,
            appearance: Appearance::default(),
            key_arrangement: None,
            check_for_updates: true,
            prevent_lighting_key_repeat: true,
        }
    }
}

/// The choices, read once and written on every change.
#[derive(Debug)]
pub struct Settings {
    path: PathBuf,
    choices: RefCell<Choices>,
}

impl Settings {
    /// A file that can't be read gives the defaults.
    pub fn load(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let choices = fs::read(&path).ok().and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default();
        Self { path, choices: RefCell::new(choices) }
    }

    pub fn choices(&self) -> Choices {
        self.choices.borrow().clone()
    }

    pub fn update(&self, change: impl FnOnce(&mut Choices)) {
        change(&mut self.choices.borrow_mut());
        let Ok(json) = serde_json::to_vec_pretty(&*self.choices.borrow()) else { return };
        if let Some(directory) = self.path.parent() {
            let _ = fs::create_dir_all(directory);
        }
        let partial = self.path.with_extension("json.partial");
        if fs::write(&partial, json).is_ok() {
            let _ = fs::rename(&partial, &self.path);
        }
    }
}

/// The languages the app speaks, each in its own script — someone looking for theirs recognises it there.
/// ⚠️ Must match the translations installed, or the list offers a language that falls back to English.
pub const LANGUAGES: [(&str, &str); 13] = [
    ("en", "English"),
    ("ko", "한국어"),
    ("ja", "日本語"),
    ("zh-Hans", "简体中文"),
    ("zh-Hant", "繁體中文"),
    ("es", "Español"),
    ("pt-BR", "Português (Brasil)"),
    ("fr", "Français"),
    ("de", "Deutsch"),
    ("it", "Italiano"),
    ("pl", "Polski"),
    ("id", "Bahasa Indonesia"),
    ("vi", "Tiếng Việt"),
];

/// gettext's name for a language: "pt-BR" → "pt_BR", "zh-Hans" → "zh_CN".
pub fn gettext_language(code: &str) -> String {
    match code {
        "zh-Hans" => "zh_CN".to_owned(),
        "zh-Hant" => "zh_TW".to_owned(),
        code => code.replace('-', "_"),
    }
}
