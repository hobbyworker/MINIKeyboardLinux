//! Looking at the windows without a screen in front of you. With `MINIKEYBOARD_SCREENSHOT=<file.png>`, the app
//! takes the steps in `MINIKEYBOARD_SCREENSHOT_STEPS`, waits for them to settle
//! (`MINIKEYBOARD_SCREENSHOT_DELAY`, in milliseconds), draws a window into the file — the main one with its
//! dialogs, with `MINIKEYBOARD_SCREENSHOT_WINDOW=log` the device log, with `popover` the keyboard information —
//! and quits.
//!
//! The steps, separated by `;`, are what a user would do:
//! `arrangement:6,3,1`, `layout:9add2`, `select:key:3`, `select:knob:1:press`, `modifier:super`, `press:0x06`,
//! `tab:media`, `keys:qwerty`, `media:8`, `mouse:3`, `mode:2`, `color:255,0,0`, `layer:2`, `info`, and
//! `action:<name>` for any of the window's actions (`device-log`, `preferences`, `about`, `allow`, `shape` …), and
//! `update:<version>` for the banner of a newer version.

use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use gtk::glib;
use presentation::{ControlRef, EditorTab, KeyUsage, KnobAction, MediaChoice, ModifierKey, MouseChoice, Rgb};

use crate::settings::Settings;
use crate::updates::Release;
use crate::window::MainWindow;

pub fn schedule(app: &adw::Application, main: &Rc<MainWindow>, settings: &Rc<Settings>) {
    let Some(path) = std::env::var_os("MINIKEYBOARD_SCREENSHOT").map(PathBuf::from) else { return };
    let delay =
        std::env::var("MINIKEYBOARD_SCREENSHOT_DELAY").ok().and_then(|delay| delay.parse().ok()).unwrap_or(1500);
    let steps = std::env::var("MINIKEYBOARD_SCREENSHOT_STEPS").unwrap_or_default();
    let target = std::env::var("MINIKEYBOARD_SCREENSHOT_WINDOW").unwrap_or_default();
    let (app, main, settings) = (app.clone(), main.clone(), settings.clone());
    // The keyboard connects and is identified first.
    glib::timeout_add_local_once(Duration::from_millis(400), move || {
        for step in steps.split(';').map(str::trim).filter(|step| !step.is_empty()) {
            if !take(&main, &settings, step) {
                eprintln!("Unknown screenshot step: {step}");
            }
        }
        glib::timeout_add_local_once(Duration::from_millis(delay), move || {
            let widget: gtk::Widget = match target.as_str() {
                "log" => app
                    .windows()
                    .into_iter()
                    .find(|window| *window != main.window.clone().upcast::<gtk::Window>())
                    .map_or_else(|| main.window.clone().upcast(), |window| window.upcast()),
                // A popover is a surface of its own, so it is drawn on its own.
                "popover" => {
                    main.header.info.popover().map_or_else(|| main.window.clone().upcast(), |popover| popover.upcast())
                }
                _ => main.window.clone().upcast(),
            };
            if save(&widget, &path).is_none() {
                eprintln!("The window couldn’t be drawn into {}.", path.display());
            }
            app.quit();
        });
    });
}

fn take(main: &MainWindow, settings: &Settings, step: &str) -> bool {
    let editor = &main.editor;
    let (verb, argument) = step.split_once(':').unwrap_or((step, ""));
    let numbers = || argument.split(',').filter_map(|part| part.trim().parse::<u8>().ok()).collect::<Vec<_>>();
    match verb {
        "arrangement" => match numbers()[..] {
            [keys, columns, knobs] => editor.choose_arrangement(keys, columns, knobs, 0),
            _ => return false,
        },
        "layout" => editor.choose_layout(Some(argument), 0),
        "select" => {
            let parts: Vec<&str> = argument.split(':').collect();
            let control = match parts[..] {
                ["key", number] => number.parse().ok().map(ControlRef::key),
                ["knob", number, action] => number
                    .parse()
                    .ok()
                    .zip(knob_action(action))
                    .map(|(number, action)| ControlRef::knob(number, action)),
                _ => None,
            };
            let Some(control) = control else { return false };
            editor.select(control);
        }
        "modifier" => {
            let Some(modifier) =
                ModifierKey::ALL.into_iter().find(|modifier| modifier.label().eq_ignore_ascii_case(argument))
            else {
                return false;
            };
            editor.toggle(modifier);
        }
        "press" => {
            let Some(key) = u8::from_str_radix(argument.trim_start_matches("0x"), 16).ok().and_then(KeyUsage::named)
            else {
                return false;
            };
            editor.press(key);
        }
        "tab" => {
            let tab = match argument {
                "keys" => EditorTab::Keys,
                "media" => EditorTab::Media,
                "mouse" => EditorTab::Mouse,
                "lighting" => EditorTab::Lighting,
                "delay" => EditorTab::Delay,
                _ => return false,
            };
            editor.set_tab(tab);
        }
        "keys" => {
            settings.update(|choices| choices.key_arrangement = Some(argument.to_owned()));
            editor.set_tab(EditorTab::Keys);
        }
        "media" => match argument.parse::<usize>().ok().and_then(|index| MediaChoice::ALL.get(index)) {
            Some(media) => editor.assign_media(*media),
            None => return false,
        },
        "mouse" => match argument.parse::<usize>().ok().and_then(|index| MouseChoice::ALL.get(index)) {
            Some(mouse) => editor.assign_mouse(*mouse),
            None => return false,
        },
        "mode" => match numbers()[..] {
            [mode] => editor.set_lighting_mode(mode),
            _ => return false,
        },
        "color" => match numbers()[..] {
            [red, green, blue] => editor.set_key_color(Rgb::new(red, green, blue)),
            _ => return false,
        },
        "layer" => match numbers()[..] {
            [layer] if layer > 0 => editor.set_layer(usize::from(layer) - 1),
            _ => return false,
        },
        "info" => main.header.info.popup(),
        "update" => {
            main.show_release(&Release { version: argument.to_owned(), link: "https://example.com/".to_owned() })
        }
        "action" => {
            return gtk::prelude::WidgetExt::activate_action(&main.window, &format!("win.{argument}"), None).is_ok();
        }
        _ => return false,
    }
    true
}

fn knob_action(name: &str) -> Option<KnobAction> {
    Some(match name {
        "left" => KnobAction::RotateLeft,
        "press" => KnobAction::Press,
        "right" => KnobAction::RotateRight,
        "press-left" => KnobAction::PressRotateLeft,
        "press-right" => KnobAction::PressRotateRight,
        _ => return None,
    })
}

fn save(widget: &impl IsA<gtk::Widget>, path: &PathBuf) -> Option<()> {
    let paintable = gtk::WidgetPaintable::new(Some(widget));
    let snapshot = gtk::Snapshot::new();
    paintable.snapshot(&snapshot, f64::from(widget.width()), f64::from(widget.height()));
    let node = snapshot.to_node()?;
    let renderer = widget.native()?.renderer()?;
    renderer.render_texture(node, None).save_to_png(path).ok()
}
