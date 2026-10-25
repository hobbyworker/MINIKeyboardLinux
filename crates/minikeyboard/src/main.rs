//! The composition root: the one place that knows the concrete repositories. Everything else depends on the
//! domain's ports.
//!
//! The app's ID, name and links are the project's own, matching the desktop file, metainfo and icons in `data/`.
//! A fork gives its own through the build's environment — `MINIKEYBOARD_APP_ID`, `MINIKEYBOARD_APP_NAME` and the
//! rest below.

use std::path::PathBuf;
use std::rc::Rc;

use domain::ports::{KeyboardConnectionRepository, KeyboardDeviceRepository, LayoutProfileRepository};
use domain::usecases::*;
use presentation::{DeviceLogModel, EditorDependencies, EditorModel};
use ui::{AppInfo, Launch};

/// A build-time value, or the default.
macro_rules! brand {
    ($name:literal, $default:expr) => {
        match option_env!($name) {
            Some(value) if !value.is_empty() => value,
            _ => $default,
        }
    };
}

const APP_ID: &str = brand!("MINIKEYBOARD_APP_ID", "me.hobbyworker.MINIKeyboard");
const APP_NAME: &str = brand!("MINIKEYBOARD_APP_NAME", "MINIKeyboard");
const COPYRIGHT: &str = brand!("MINIKEYBOARD_COPYRIGHT", "© 2026 hobbyworker");
const WEBSITE: &str = brand!("MINIKEYBOARD_WEBSITE", "https://github.com/hobbyworker/MINIKeyboardLinux");
const ISSUE_URL: &str = brand!("MINIKEYBOARD_ISSUE_URL", "https://github.com/hobbyworker/MINIKeyboardLinux/issues");
/// The new-version notice's feed — off unless a build gives one; the app's own builds leave updates to Flatpak.
const UPDATE_FEED: &str = brand!("MINIKEYBOARD_UPDATE_FEED", "");
/// Where the translations are installed; `MINIKEYBOARD_LOCALE_DIR` at run time wins, for trying a build.
const LOCALE_DIR: &str = brand!("MINIKEYBOARD_LOCALE_DIR", "/usr/share/locale");

fn optional(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

fn main() -> gtk::glib::ExitCode {
    let config = gtk::glib::user_config_dir().join(APP_ID);
    let launch = Launch {
        info: AppInfo {
            id: APP_ID.to_owned(),
            name: APP_NAME.to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            copyright: optional(COPYRIGHT),
            website: optional(WEBSITE),
            issue_url: optional(ISSUE_URL),
            update_feed: optional(UPDATE_FEED),
        },
        text_domain: "minikeyboard".to_owned(),
        locale_dir: std::env::var_os("MINIKEYBOARD_LOCALE_DIR")
            .map_or_else(|| PathBuf::from(LOCALE_DIR), PathBuf::from),
        settings_path: config.join("settings.json"),
    };
    // First: the language, before any thread starts.
    let settings = ui::prepare(&launch);

    let log = datalayer::DeviceLogStore::new();
    let profiles: Rc<dyn LayoutProfileRepository> =
        Rc::new(datalayer::FileLayoutProfiles::new(config.join("layouts.json")));
    // Preferences ▸ General ▸ Keyboard, asked at every write.
    let prevents_lighting_key_repeat: Rc<dyn Fn() -> bool> = {
        let settings = settings.clone();
        Rc::new(move || settings.choices().prevent_lighting_key_repeat)
    };
    let (connection, device, keyboard_source) = connect(&log, prevents_lighting_key_repeat);

    let editor = EditorModel::new(EditorDependencies {
        observe_connection: ObserveConnectionUseCase::new(connection.clone()),
        retry_connection: RetryConnectionUseCase::new(connection),
        identify_keyboard: IdentifyKeyboardUseCase::new(device.clone()),
        resolve_layout: ResolveLayoutUseCase::new(profiles.clone()),
        choose_layout: ChooseLayoutUseCase::new(profiles),
        read_keyboard: ReadKeyboardUseCase::new(device.clone()),
        save_keyboard: SaveKeyboardUseCase::new(device),
        keyboard_source,
    });
    let device_log = DeviceLogModel::new(DeviceLogUseCase::new(Rc::new(log)));
    ui::run(launch, settings, editor, device_log)
}

type Repositories =
    (Rc<dyn KeyboardConnectionRepository>, Rc<dyn KeyboardDeviceRepository>, Option<KeyboardSourceUseCase>);

/// The keyboard on USB or a simulated one, switched from the Simulator menu. `MINIKEYBOARD_SIMULATED_KEYBOARD`
/// starts on a simulated keyboard (a `SimulatedModel` ID, e.g. `8890-3`).
#[cfg(debug_assertions)]
fn connect(log: &datalayer::DeviceLogStore, prevents_lighting_key_repeat: Rc<dyn Fn() -> bool>) -> Repositories {
    let initial = std::env::var("MINIKEYBOARD_SIMULATED_KEYBOARD")
        .ok()
        .and_then(|id| domain::SimulatedModel::all().into_iter().find(|model| model.id() == id))
        .map_or(domain::KeyboardSource::Hardware, domain::KeyboardSource::Simulated);
    let source = Rc::new(datalayer::KeyboardSourceSwitch::new(log.clone(), initial));
    let device = Rc::new(
        datalayer::KeyboardDevice::new(source.clone(), log.clone())
            .with_lighting_repeat_guard(prevents_lighting_key_repeat),
    );
    (source.clone(), device, Some(KeyboardSourceUseCase::new(source)))
}

#[cfg(not(debug_assertions))]
fn connect(log: &datalayer::DeviceLogStore, prevents_lighting_key_repeat: Rc<dyn Fn() -> bool>) -> Repositories {
    let hardware = Rc::new(datalayer::HardwareConnection::start(log.clone()));
    let device = Rc::new(
        datalayer::KeyboardDevice::new(hardware.clone(), log.clone())
            .with_lighting_repeat_guard(prevents_lighting_key_repeat),
    );
    (hardware, device, None)
}
