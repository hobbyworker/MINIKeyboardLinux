//! Preferences — what the app sends a keyboard and the automatic update check (in a build with a feed), the interface
//! language, and light or dark — and About.

use std::rc::Rc;

use adw::prelude::*;

use crate::AppInfo;
use crate::i18n::{t, tf};
use crate::settings::{Appearance, LANGUAGES, Settings};

/// Light, dark or the system's, at once.
pub fn apply_appearance(appearance: Appearance) {
    adw::StyleManager::default().set_color_scheme(match appearance {
        Appearance::System => adw::ColorScheme::Default,
        Appearance::Light => adw::ColorScheme::ForceLight,
        Appearance::Dark => adw::ColorScheme::ForceDark,
    });
}

pub fn preferences_dialog(
    settings: &Rc<Settings>,
    app: &AppInfo,
    window: &adw::ApplicationWindow,
) -> adw::PreferencesDialog {
    let choices = settings.choices();
    let group = adw::PreferencesGroup::builder()
        .title(gtk::glib::markup_escape_text(&t("Language & Appearance")))
        .description(gtk::glib::markup_escape_text(&t(
            "The base language is English. You can follow the system language or choose one yourself.",
        )))
        .build();

    let languages: Vec<String> =
        std::iter::once(t("Follow System")).chain(LANGUAGES.iter().map(|(_, name)| (*name).to_owned())).collect();
    let language = adw::ComboRow::builder()
        .title(t("Language"))
        .model(&gtk::StringList::new(&languages.iter().map(String::as_str).collect::<Vec<_>>()))
        .build();
    let selected = choices
        .language
        .as_deref()
        .and_then(|code| LANGUAGES.iter().position(|(known, _)| *known == code))
        .map_or(0, |index| index + 1);
    language.set_selected(selected as u32);

    let appearances = [t("Follow System"), t("Light"), t("Dark")];
    let appearance = adw::ComboRow::builder()
        .title(t("Appearance"))
        .model(&gtk::StringList::new(&appearances.iter().map(String::as_str).collect::<Vec<_>>()))
        .build();
    appearance.set_selected(match choices.appearance {
        Appearance::System => 0,
        Appearance::Light => 1,
        Appearance::Dark => 2,
    });
    group.add(&language);
    group.add(&appearance);

    let page = adw::PreferencesPage::builder().title(t("UI")).icon_name("applications-graphics-symbolic").build();
    page.add(&group);
    let dialog = adw::PreferencesDialog::builder().title(t("Preferences")).build();
    // General: what the app sends a keyboard, and the update check in a build with a feed (as the macOS app does).
    let general = adw::PreferencesPage::builder().title(t("General")).icon_name("emblem-system-symbolic").build();
    {
        let repeat = adw::SwitchRow::builder()
            .title(t("Prevent key repeat after changing the lighting"))
            .active(choices.prevent_lighting_key_repeat)
            .build();
        let settings = settings.clone();
        repeat.connect_active_notify(move |row| {
            settings.update(|choices| choices.prevent_lighting_key_repeat = row.is_active());
        });
        let keyboard = adw::PreferencesGroup::builder()
            .title(gtk::glib::markup_escape_text(&t("Keyboard")))
            .description(gtk::glib::markup_escape_text(&t("After its lighting is saved, the 1189:8890 keyboard repeats a key's first stroke without end when the key is pressed (its maker's software does the same). With this on, the app sends one more save after the lighting to stop that; the keys stay as they were. Off, the app sends just what the maker's software sends.")))
            .build();
        keyboard.add(&repeat);
        general.add(&keyboard);
    }
    if app.update_feed.is_some() {
        let check = adw::SwitchRow::builder()
            .title(t("Check for updates automatically"))
            .active(choices.check_for_updates)
            .build();
        let settings = settings.clone();
        check.connect_active_notify(move |row| settings.update(|choices| choices.check_for_updates = row.is_active()));
        let updates = adw::PreferencesGroup::new();
        updates.add(&check);
        general.add(&updates);
    }
    dialog.add(&general);
    dialog.add(&page);

    {
        let settings = settings.clone();
        appearance.connect_selected_notify(move |row| {
            let appearance = match row.selected() {
                1 => Appearance::Light,
                2 => Appearance::Dark,
                _ => Appearance::System,
            };
            settings.update(|choices| choices.appearance = appearance);
            apply_appearance(appearance);
        });
    }
    {
        let (settings, name, window) = (settings.clone(), app.name.clone(), window.clone());
        language.connect_selected_notify(move |row| {
            let code = match row.selected() {
                0 => None,
                index => LANGUAGES.get(index as usize - 1).map(|(code, _)| (*code).to_owned()),
            };
            settings.update(|choices| choices.language = code);
            ask_to_relaunch(&window, &name);
        });
    }
    dialog
}

/// The language is read once, at launch.
fn ask_to_relaunch(window: &adw::ApplicationWindow, name: &str) {
    let alert = adw::AlertDialog::builder()
        .heading(t("Language Changed"))
        .body(tf("%@ needs to relaunch to apply the change.", &[&name]))
        .build();
    alert.add_responses(&[("later", &t("Later")), ("relaunch", &t("Relaunch Now"))]);
    alert.set_response_appearance("relaunch", adw::ResponseAppearance::Suggested);
    alert.set_default_response(Some("relaunch"));
    let application = window.application();
    alert.connect_response(None, move |_, response| {
        if response != "relaunch" {
            return;
        }
        // A new instance first, then this one goes; the new one takes the app ID once this has let go.
        if let Ok(executable) = std::env::current_exe()
            && std::process::Command::new(executable).spawn().is_ok()
            && let Some(application) = &application
        {
            application.quit();
        }
    });
    alert.present(Some(window));
}

pub fn about_dialog(app: &AppInfo) -> adw::AboutDialog {
    let about = adw::AboutDialog::builder()
        .application_name(&app.name)
        .application_icon(&app.id)
        .version(&app.version)
        .license_type(gtk::License::MitX11)
        .build();
    if let Some(copyright) = &app.copyright {
        about.set_copyright(copyright);
    }
    if let Some(website) = &app.website {
        about.set_website(website);
    }
    if let Some(issues) = &app.issue_url {
        about.set_issue_url(issues);
    }
    about.add_legal_section("GTK", None, gtk::License::Lgpl21, None);
    about.add_legal_section("libadwaita", None, gtk::License::Lgpl21, None);
    about.add_legal_section("libusb", None, gtk::License::Lgpl21, None);
    about.add_legal_section("gtk-rs", None, gtk::License::MitX11, None);
    about
}
