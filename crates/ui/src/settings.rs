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
pub const LANGUAGES: [(&str, &str); 17] = [
    ("en", "English"),
    ("ko", "한국어"),
    ("ja", "日本語"),
    ("zh-Hans", "简体中文"),
    ("zh-Hant", "繁體中文"),
    ("es", "Español"),
    ("es-419", "Español (Latinoamérica)"),
    ("pt-BR", "Português (Brasil)"),
    ("pt-PT", "Português (Portugal)"),
    ("fr", "Français"),
    ("de", "Deutsch"),
    ("it", "Italiano"),
    ("pl", "Polski"),
    ("ru", "Русский"),
    ("id", "Bahasa Indonesia"),
    ("vi", "Tiếng Việt"),
    ("th", "ไทย"),
];

/// gettext's name for a language: "pt-BR" → "pt_BR", "pt-PT" → "pt" (gettext's own Portuguese is Portugal's),
/// "zh-Hans" → "zh_CN", "es-419" → "es_419".
pub fn gettext_language(code: &str) -> String {
    match code {
        "pt-PT" => "pt".to_owned(),
        "zh-Hans" => "zh_CN".to_owned(),
        "zh-Hant" => "zh_TW".to_owned(),
        code => code.replace('-', "_"),
    }
}

/// The regions whose Spanish is Latin America's — its countries and the US.
const LATIN_AMERICAN_SPANISH: [&str; 22] = [
    "AR", "BO", "BR", "BZ", "CL", "CO", "CR", "CU", "DO", "EC", "GT", "HN", "MX", "NI", "PA", "PE", "PR", "PY", "SV",
    "US", "UY", "VE",
];

/// `LANGUAGE` for following the system, or `None` to leave it as it is. gettext looks for es_MX, es_AR … and then for
/// es — Spain's — so when the system's language is the Spanish of Latin America or the US, the app's es_419 goes first.
/// `language` is `LANGUAGE`, `locale` the one gettext takes the language of messages from (LC_ALL, LC_MESSAGES or LANG).
pub fn system_language(language: Option<&str>, locale: Option<&str>) -> Option<String> {
    let language = language.filter(|list| !list.is_empty());
    let first = match language {
        Some(list) => list.split(':').next()?,
        None => locale?,
    };
    let name = first.split(['.', '@']).next()?;
    let mut parts = name.split(['_', '-']);
    if parts.next()? != "es" {
        return None;
    }
    let region = parts.next()?;
    if !LATIN_AMERICAN_SPANISH.contains(&region.to_ascii_uppercase().as_str()) {
        return None;
    }
    Some(format!("es_419:{}", language.unwrap_or(name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_language_has_the_name_of_its_po_file() {
        assert_eq!(gettext_language("pt-BR"), "pt_BR");
        assert_eq!(gettext_language("pt-PT"), "pt");
        assert_eq!(gettext_language("zh-Hans"), "zh_CN");
        assert_eq!(gettext_language("zh-Hant"), "zh_TW");
        assert_eq!(gettext_language("es-419"), "es_419");
        assert_eq!(gettext_language("ko"), "ko");
    }

    #[test]
    fn the_spanish_of_latin_america_and_the_us_follows_the_system_in_latin_american_spanish() {
        assert_eq!(system_language(None, Some("es_MX.UTF-8")).as_deref(), Some("es_419:es_MX"));
        assert_eq!(system_language(Some(""), Some("es_US.UTF-8")).as_deref(), Some("es_419:es_US"));
        assert_eq!(system_language(Some("es_AR:es"), Some("es_AR.UTF-8")).as_deref(), Some("es_419:es_AR:es"));
        assert_eq!(system_language(None, Some("es_CO.utf8@euro")).as_deref(), Some("es_419:es_CO"));
    }

    #[test]
    fn any_other_language_follows_the_system_as_it_is() {
        assert_eq!(system_language(None, Some("es_ES.UTF-8")), None);
        assert_eq!(system_language(None, Some("es")), None);
        assert_eq!(system_language(Some("es_419"), Some("es_MX.UTF-8")), None);
        assert_eq!(system_language(Some("en_US:es_MX"), Some("es_MX.UTF-8")), None);
        assert_eq!(system_language(None, Some("ko_KR.UTF-8")), None);
        assert_eq!(system_language(None, Some("C")), None);
        assert_eq!(system_language(None, None), None);
    }
}
