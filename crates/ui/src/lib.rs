//! The GTK 4 and libadwaita windows. They see the presentation's types only: the view models come in from the
//! composition root, and the views read what they draw from them and hand the user's doings back.

mod canvas;
mod dialogs;
mod header;
pub mod i18n;
mod inspector;
mod log_window;
mod names;
mod pages;
mod preferences;
mod screenshot;
mod settings;
mod shape;
mod updates;
mod widgets;
mod window;

use std::cell::RefCell;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::sync::OnceLock;

use adw::prelude::*;
use gtk::glib;
use presentation::{DeviceLogModel, EditorModel};

pub use settings::{Appearance, Choices, LANGUAGES, Settings, gettext_language};

/// What the app is called and where it keeps things — the shipped app's own, or a build's defaults.
#[derive(Clone, Debug)]
pub struct AppInfo {
    /// The application ID, also the icon's name.
    pub id: String,
    pub name: String,
    pub version: String,
    pub copyright: Option<String>,
    pub website: Option<String>,
    pub issue_url: Option<String>,
    /// Where the new-version notice looks (`updates.rs`), or `None` for a build without it.
    pub update_feed: Option<String>,
}

/// How the interface starts.
#[derive(Clone, Debug)]
pub struct Launch {
    pub info: AppInfo,
    /// gettext's domain, and where its translations are.
    pub text_domain: String,
    pub locale_dir: PathBuf,
    /// The interface's own choices: language, appearance, the Keys page's layout.
    pub settings_path: PathBuf,
}

/// Reads the interface's choices and sets up its language. Call it first — before any other thread starts,
/// as the language is set in the environment gettext reads.
pub fn prepare(launch: &Launch) -> Rc<Settings> {
    INHERITED_LANGUAGE.get_or_init(|| std::env::var_os("LANGUAGE"));
    let settings = Settings::load(&launch.settings_path);
    if let Some(language) = settings.choices().language {
        set_language(&gettext_language(&language));
    } else if let Some(language) =
        settings::system_language(std::env::var("LANGUAGE").ok().as_deref(), messages_locale().as_deref())
    {
        set_language(&language);
    }
    i18n::init(&launch.text_domain, &launch.locale_dir);
    Rc::new(settings)
}

/// The locale gettext takes the language of messages from when `LANGUAGE` says nothing.
fn messages_locale() -> Option<String> {
    ["LC_ALL", "LC_MESSAGES", "LANG"]
        .into_iter()
        .find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()))
}

/// `LANGUAGE` as the app found it, before a chosen language replaced it.
static INHERITED_LANGUAGE: OnceLock<Option<OsString>> = OnceLock::new();

#[allow(unsafe_code)]
fn set_language(language: &str) {
    // SAFETY: called before the app starts any thread, so nothing reads the environment meanwhile.
    unsafe { std::env::set_var("LANGUAGE", language) };
}

/// Starts the app again, for a new language to take effect.
fn relaunch() -> bool {
    let inherited = INHERITED_LANGUAGE.get().cloned().flatten();
    std::env::current_exe().is_ok_and(|executable| relaunch_command(executable, inherited).spawn().is_ok())
}

/// The new instance gets `LANGUAGE` as this one found it, not the one a chosen language put in this one's environment
/// — or Follow System would keep showing the language just left.
fn relaunch_command(executable: PathBuf, inherited_language: Option<OsString>) -> Command {
    let mut command = Command::new(executable);
    match inherited_language {
        Some(language) => command.env("LANGUAGE", language),
        None => command.env_remove("LANGUAGE"),
    };
    command
}

/// Runs the app until its last window closes.
pub fn run(launch: Launch, settings: Rc<Settings>, editor: Rc<EditorModel>, log: Rc<DeviceLogModel>) -> glib::ExitCode {
    let app = adw::Application::builder().application_id(&launch.info.id).build();
    app.connect_startup(|_| {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(include_str!("style.css"));
        if let Some(display) = gtk::gdk::Display::default() {
            gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        }
    });
    {
        let quit = gtk::gio::SimpleAction::new("quit", None);
        let app_handle = app.downgrade();
        quit.connect_activate(move |_, _| {
            if let Some(app) = app_handle.upgrade() {
                app.quit();
            }
        });
        app.add_action(&quit);
    }
    let main: Rc<RefCell<Option<Rc<window::MainWindow>>>> = Rc::default();
    app.connect_activate(move |app| {
        if let Some(main) = main.borrow().as_ref() {
            main.window.present();
            return;
        }
        preferences::apply_appearance(settings.choices().appearance);
        let created = window::MainWindow::new(app, &launch.info, settings.clone(), editor.clone(), log.clone());
        glib::spawn_future_local(editor.run());
        glib::spawn_future_local(log.run());
        created.window.present();
        screenshot::schedule(app, &created, &settings);
        *main.borrow_mut() = Some(created);
    });
    // The app takes no arguments of GTK's.
    app.run_with_args::<&str>(&[])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn language_of(command: &Command) -> Option<Option<OsString>> {
        command.get_envs().find(|(name, _)| *name == "LANGUAGE").map(|(_, value)| value.map(ToOwned::to_owned))
    }

    #[test]
    fn a_relaunch_drops_the_language_the_app_set_itself() {
        let command = relaunch_command(PathBuf::from("/app/bin/minikeyboard"), None);
        assert_eq!(language_of(&command), Some(None));
    }

    #[test]
    fn a_relaunch_keeps_the_language_the_app_was_started_with() {
        let command = relaunch_command(PathBuf::from("/app/bin/minikeyboard"), Some(OsString::from("ko_KR:ko")));
        assert_eq!(language_of(&command), Some(Some(OsString::from("ko_KR:ko"))));
    }
}
