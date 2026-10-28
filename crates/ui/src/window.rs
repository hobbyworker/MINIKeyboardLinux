//! The main window: the keyboard on top, what to give the selected key below.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use gtk::{gio, glib};
use presentation::{DeviceLogModel, EditorModel, SimulatorChoice, SimulatorGroup};

use crate::AppInfo;
use crate::canvas::KeyboardCanvas;
use crate::dialogs::{LayoutDialog, permission_dialog};
use crate::header::Header;
use crate::i18n::{t, tf};
use crate::inspector::Inspector;
use crate::log_window::LogWindow;
use crate::names::{commands_text, notice_message, simulator_kind_title};
use crate::pages::ActionPanel;
use crate::preferences::{about_dialog, preferences_dialog};
use crate::settings::Settings;
use crate::updates::{self, Release};

/// The inspector column's width: the inspector's own least (270, and 16 either side).
const INSPECTOR_WIDTH: i32 = 302;

pub struct MainWindow {
    pub window: adw::ApplicationWindow,
    pub(crate) editor: Rc<EditorModel>,
    toasts: adw::ToastOverlay,
    /// "A newer version is out", with the way to its download page.
    banner: adw::Banner,
    /// "Simulator in use", across the window while a simulated keyboard is.
    simulator_bar: gtk::Box,
    release_link: Rc<RefCell<Option<String>>>,
    app_name: String,
    pub(crate) header: Header,
    canvas: KeyboardCanvas,
    canvas_stack: gtk::Stack,
    layout_buttons: gtk::Box,
    panel: ActionPanel,
    inspector: Inspector,
    layout_dialog: Rc<RefCell<Option<adw::Dialog>>>,
    /// "Switch keyboards?", while the editor has a switch that would lose changes not saved.
    switch_dialog: Rc<RefCell<Option<adw::AlertDialog>>>,
    shown_notice: Cell<u64>,
    toast: RefCell<Option<adw::Toast>>,
    actions: RefCell<Vec<(&'static str, gio::SimpleAction)>>,
    simulator: RefCell<Option<gio::SimpleAction>>,
}

impl MainWindow {
    pub fn new(
        app: &adw::Application,
        info: &AppInfo,
        settings: Rc<Settings>,
        editor: Rc<EditorModel>,
        log: Rc<DeviceLogModel>,
    ) -> Rc<Self> {
        let header = Header::new(editor.clone());

        let canvas = KeyboardCanvas::default();
        canvas.set_hexpand(true);
        canvas.set_vexpand(true);
        {
            let editor = editor.clone();
            canvas.connect_select(move |control| editor.select(control));
        }
        let empty = adw::StatusPage::builder()
            .icon_name("input-keyboard-symbolic")
            .description(t("Keys appear here once the keyboard is connected."))
            .build();
        empty.add_css_class("compact");
        let canvas_stack = gtk::Stack::new();
        canvas_stack.add_named(&canvas, Some("canvas"));
        canvas_stack.add_named(&empty, Some("empty"));
        let rotate = gtk::Button::builder()
            .icon_name("object-rotate-right-symbolic")
            .tooltip_text(t("Rotate"))
            .action_name("win.rotate")
            .build();
        rotate.add_css_class("osd");
        rotate.add_css_class("circular");
        let shape = gtk::Button::builder().label(t("Shape…")).action_name("win.shape").build();
        shape.add_css_class("osd");
        let layout_buttons = gtk::Box::builder()
            .spacing(6)
            .halign(gtk::Align::End)
            .valign(gtk::Align::End)
            .margin_end(10)
            .margin_bottom(10)
            .build();
        layout_buttons.append(&rotate);
        layout_buttons.append(&shape);
        let overlay = gtk::Overlay::builder()
            .child(&canvas_stack)
            .margin_start(24)
            .margin_end(24)
            .margin_top(10)
            .margin_bottom(10)
            .build();
        overlay.add_overlay(&layout_buttons);
        let keyboard_area = gtk::Box::builder().height_request(230).build();
        keyboard_area.add_css_class("canvas-area");
        overlay.set_hexpand(true);
        keyboard_area.append(&overlay);

        let panel = ActionPanel::new(editor.clone(), settings.clone());
        panel.widget().set_hexpand(true);
        let inspector = Inspector::new(editor.clone());
        // The inspector at one width, whatever it shows. Left to its wrapping labels it asks for more or less with every
        // selection, the panel beside it narrows and widens, its rows wrap and unwrap — and through the panes the keyboard
        // above shrinks and grows with them.
        let inspector_column = adw::Clamp::builder()
            .orientation(gtk::Orientation::Horizontal)
            .maximum_size(INSPECTOR_WIDTH)
            .tightening_threshold(INSPECTOR_WIDTH)
            .width_request(INSPECTOR_WIDTH)
            .child(inspector.widget())
            .build();
        let bottom = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        bottom.append(panel.widget());
        bottom.append(&gtk::Separator::new(gtk::Orientation::Vertical));
        bottom.append(&inspector_column);

        let paned = gtk::Paned::builder()
            .orientation(gtk::Orientation::Vertical)
            .start_child(&keyboard_area)
            .end_child(&bottom)
            .shrink_start_child(false)
            .shrink_end_child(false)
            .resize_start_child(true)
            .resize_end_child(false)
            .wide_handle(true)
            .position(330)
            .build();
        let banner = adw::Banner::builder().button_label(t("Download")).build();
        let toolbar = adw::ToolbarView::new();
        let simulator_bar = simulator_bar();
        toolbar.add_top_bar(&header.bar);
        toolbar.add_top_bar(&simulator_bar);
        toolbar.add_top_bar(&banner);
        toolbar.set_content(Some(&paned));
        let toasts = adw::ToastOverlay::new();
        toasts.set_child(Some(&toolbar));
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title(&info.name)
            .default_width(1100)
            .default_height(800)
            .width_request(900)
            .height_request(640)
            .content(&toasts)
            .build();

        let main = Rc::new(Self {
            window,
            editor: editor.clone(),
            toasts,
            banner,
            simulator_bar,
            release_link: Rc::default(),
            app_name: info.name.clone(),
            header,
            canvas,
            canvas_stack,
            layout_buttons,
            panel,
            inspector,
            layout_dialog: Rc::default(),
            switch_dialog: Rc::default(),
            shown_notice: Cell::new(0),
            toast: RefCell::default(),
            actions: RefCell::default(),
            simulator: RefCell::default(),
        });
        main.install_actions(app, info, settings.clone(), log);
        main.watch_for_updates(info, settings);
        {
            let (window, link) = (main.window.clone(), main.release_link.clone());
            main.banner.connect_button_clicked(move |_| {
                if let Some(link) = link.borrow().as_deref() {
                    gtk::UriLauncher::new(link).launch(Some(&window), gio::Cancellable::NONE, |_| {});
                }
            });
        }
        {
            let weak = Rc::downgrade(&main);
            editor.subscribe(move || {
                if let Some(main) = weak.upgrade() {
                    main.refresh();
                }
            });
        }
        main.refresh();
        main
    }

    fn install_actions(
        self: &Rc<Self>,
        app: &adw::Application,
        info: &AppInfo,
        settings: Rc<Settings>,
        log: Rc<DeviceLogModel>,
    ) {
        let editor = &self.editor;
        let mut actions = vec![];
        let mut add = |name: &'static str, handler: Box<dyn Fn()>| {
            let action = gio::SimpleAction::new(name, None);
            action.connect_activate(move |_, _| handler());
            self.window.add_action(&action);
            actions.push((name, action));
        };
        {
            let editor = editor.clone();
            add(
                "read",
                Box::new(move || {
                    let editor = editor.clone();
                    glib::spawn_future_local(async move { editor.read().await });
                }),
            );
        }
        {
            let editor = editor.clone();
            add(
                "save",
                Box::new(move || {
                    let editor = editor.clone();
                    glib::spawn_future_local(async move { editor.save().await });
                }),
            );
        }
        let with = |handler: fn(&EditorModel)| {
            let editor = editor.clone();
            Box::new(move || handler(&editor)) as Box<dyn Fn()>
        };
        add("clear-key", with(EditorModel::clear_selected));
        add("clear-layer", with(EditorModel::clear_layer));
        add("shape", with(|editor| editor.set_choosing_layout(true)));
        add("rotate", with(EditorModel::rotate));
        {
            let window = self.window.clone();
            let log_window: Rc<RefCell<Option<Rc<LogWindow>>>> = Rc::default();
            add(
                "device-log",
                Box::new(move || {
                    let mut log_window = log_window.borrow_mut();
                    let shown = log_window.get_or_insert_with(|| {
                        let created = LogWindow::new(log.clone());
                        created.window.set_transient_for(None::<&gtk::Window>);
                        created.window.set_application(window.application().as_ref());
                        created
                    });
                    shown.window.present();
                }),
            );
        }
        {
            let (window, settings, info) = (self.window.clone(), settings.clone(), info.clone());
            add("preferences", Box::new(move || preferences_dialog(&settings, &info, &window).present(Some(&window))));
        }
        {
            let (window, info) = (self.window.clone(), info.clone());
            add("about", Box::new(move || about_dialog(&info).present(Some(&window))));
        }
        if info.update_feed.is_some() {
            let (main, info) = (Rc::downgrade(self), info.clone());
            add(
                "check-updates",
                Box::new(move || {
                    if let Some(main) = main.upgrade() {
                        main.check_for_updates(&info, true);
                    }
                }),
            );
        }
        {
            let window = self.window.clone();
            add("allow", Box::new(move || permission_dialog().present(Some(&window))));
        }
        self.header.allow.set_action_name(Some("win.allow"));
        *self.actions.borrow_mut() = actions;

        let menu = gio::Menu::new();
        let section = |items: &[(String, &str)]| {
            let section = gio::Menu::new();
            for (label, action) in items {
                section.append(Some(label), Some(action));
            }
            section
        };
        menu.append_section(
            None,
            &section(&[(t("Read from Keyboard"), "win.read"), (t("Save to Keyboard"), "win.save")]),
        );
        menu.append_section(
            None,
            &section(&[(t("Clear Key"), "win.clear-key"), (t("Clear Layer"), "win.clear-layer")]),
        );
        menu.append_section(None, &section(&[(t("Shape…"), "win.shape"), (t("Rotate"), "win.rotate")]));
        menu.append_section(None, &section(&[(t("Device Log"), "win.device-log")]));
        if self.editor.can_select_source() {
            menu.append_section(None, &self.simulator_menu());
        }
        let mut last = vec![(t("Preferences"), "win.preferences")];
        if info.update_feed.is_some() {
            last.push((t("Check for Updates…"), "win.check-updates"));
        }
        last.push((tf("About %@", &[&info.name]), "win.about"));
        menu.append_section(None, &section(&last));
        self.header.menu.set_menu_model(Some(&menu));

        for (action, accels) in [
            ("win.read", &["<Control>r"][..]),
            ("win.save", &["<Control>s"]),
            ("win.clear-key", &["<Control>BackSpace"]),
            ("win.clear-layer", &["<Control><Shift>BackSpace"]),
            ("win.rotate", &["<Control>t"]),
            ("win.device-log", &["<Control><Alt>l"]),
            ("win.preferences", &["<Control>comma"]),
            ("window.close", &["<Control>w"]),
            ("app.quit", &["<Control>q"]),
        ] {
            app.set_accels_for_action(action, accels);
        }
    }

    /// In a build with a feed: a look at launch, then once a day while the window is open, if the user leaves it on.
    fn watch_for_updates(self: &Rc<Self>, info: &AppInfo, settings: Rc<Settings>) {
        if info.update_feed.is_none() {
            return;
        }
        let look = {
            let (main, info) = (Rc::downgrade(self), info.clone());
            move || {
                let Some(main) = main.upgrade() else { return glib::ControlFlow::Break };
                if settings.choices().check_for_updates {
                    main.check_for_updates(&info, false);
                }
                glib::ControlFlow::Continue
            }
        };
        let first = look.clone();
        glib::timeout_add_seconds_local_once(3, move || {
            let _ = first();
        });
        glib::timeout_add_seconds_local(24 * 60 * 60, look);
    }

    /// Reads the feed. Asked from the menu, the user hears how it went either way; on its own, only of a newer version.
    fn check_for_updates(self: &Rc<Self>, info: &AppInfo, asked: bool) {
        let Some(feed) = info.update_feed.clone() else { return };
        let (main, info) = (Rc::downgrade(self), info.clone());
        glib::spawn_future_local(async move {
            let agent = format!("{}/{}", info.name, info.version);
            let newest = updates::fetch(&feed, &agent).await.ok().and_then(|feed| updates::newest(&feed));
            let Some(main) = main.upgrade() else { return };
            match newest {
                Some(release) if updates::is_newer(&release.version, &info.version) => main.show_release(&release),
                Some(_) if asked => main.toasts.add_toast(adw::Toast::new(&tf("%@ is up to date.", &[&info.name]))),
                None if asked => main.toasts.add_toast(adw::Toast::new(&t("Couldn’t check for updates."))),
                _ => {}
            }
        });
    }

    pub(crate) fn show_release(&self, release: &Release) {
        self.banner.set_title(&tf("%1$@ %2$@ is available.", &[&self.app_name, &release.version]));
        *self.release_link.borrow_mut() = Some(release.link.clone());
        self.banner.set_revealed(true);
    }

    /// The keyboard on USB, or a simulated one: the kinds users have, and in a debug build every kind (the 8890's
    /// other report IDs, the legacy records).
    fn simulator_menu(&self) -> gio::Menu {
        let action = gio::SimpleAction::new_stateful("simulator", Some(glib::VariantTy::STRING), &"".to_variant());
        {
            let editor = self.editor.clone();
            action.connect_activate(move |action, parameter| {
                let Some(id) = parameter.and_then(|parameter| parameter.get::<String>()) else { return };
                editor.select_simulator((!id.is_empty()).then_some(id.as_str()));
                // The one in use: a switch that waits for an answer, or isn't made, leaves it as it was.
                action.set_state(&editor.active_simulator_id().unwrap_or_default().to_variant());
            });
        }
        self.window.add_action(&action);
        *self.simulator.borrow_mut() = Some(action);

        let mut choices = self.editor.simulator_choices();
        if !cfg!(debug_assertions) {
            choices.retain(SimulatorChoice::is_for_everyone);
        }
        let menu = gio::Menu::new();
        let usb = gio::MenuItem::new(Some(&t("Keyboard on USB")), None);
        usb.set_action_and_target_value(Some("win.simulator"), Some(&"".to_variant()));
        menu.append_item(&usb);
        let groups = gio::Menu::new();
        // A group of one is an item of its own, under the group's name; an empty one isn't there.
        let group = |title: &str, group: SimulatorGroup| {
            let members: Vec<&SimulatorChoice> = choices.iter().filter(|choice| choice.group == group).collect();
            match members.as_slice() {
                [] => {}
                [only] => {
                    let item = gio::MenuItem::new(Some(title), None);
                    item.set_action_and_target_value(Some("win.simulator"), Some(&only.id.to_variant()));
                    groups.append_item(&item);
                }
                many => {
                    let menu = gio::Menu::new();
                    for choice in many {
                        menu.append_item(&simulator_item(choice));
                    }
                    groups.append_submenu(Some(title), &menu);
                }
            }
        };
        group("1189:8890", SimulatorGroup::K8890);
        group(&commands_text(presentation::InfoCommands::Standard), SimulatorGroup::Standard);
        group(&commands_text(presentation::InfoCommands::Legacy), SimulatorGroup::Legacy);
        for choice in choices.iter().filter(|choice| choice.group == SimulatorGroup::Unidentified) {
            groups.append_item(&simulator_item(choice));
        }
        menu.append_section(None, &groups);
        let outer = gio::Menu::new();
        outer.append_submenu(Some(&t("Simulator")), &menu);
        outer
    }

    pub fn refresh(&self) {
        let editor = &self.editor;
        self.header.refresh();
        let drawing = editor.drawing();
        self.canvas_stack.set_visible_child_name(if drawing.is_some() { "canvas" } else { "empty" });
        self.canvas.set_drawing(drawing);
        let simulated = editor.active_simulator_id().is_some();
        self.simulator_bar.set_visible(simulated);
        self.canvas.set_simulated(simulated);
        let ready = editor.is_ready();
        self.canvas.set_sensitive(ready);
        self.canvas.set_opacity(if ready { 1.0 } else { 0.45 });
        self.layout_buttons.set_visible(editor.has_layout());
        self.panel.refresh();
        self.inspector.refresh();
        self.update_actions();
        self.update_notice();
        self.update_layout_dialog();
        self.update_switch_dialog();
    }

    fn update_actions(&self) {
        let editor = &self.editor;
        let ready = editor.is_ready();
        for (name, action) in self.actions.borrow().iter() {
            action.set_enabled(match *name {
                "read" => editor.can_read(),
                "save" => ready && editor.has_unsaved_changes(),
                "clear-key" => ready && editor.selected_control().is_some(),
                "clear-layer" => ready,
                "shape" | "rotate" => editor.has_layout(),
                _ => true,
            });
        }
        if let Some(simulator) = self.simulator.borrow().as_ref() {
            // The Simulator menu's items, and the simulator bar's way back to the keyboard on USB.
            simulator.set_enabled(editor.can_switch_keyboard());
            simulator.set_state(&editor.active_simulator_id().unwrap_or_default().to_variant());
        }
    }

    /// A message that comes and goes, as a toast.
    fn update_notice(&self) {
        let editor = &self.editor;
        let serial = editor.notice_serial();
        match editor.notice() {
            Some(notice) if serial != self.shown_notice.get() => {
                self.shown_notice.set(serial);
                let previous = self.toast.borrow_mut().take();
                if let Some(previous) = previous {
                    previous.dismiss();
                }
                let toast = adw::Toast::builder()
                    .title(notice_message(notice))
                    .use_markup(false)
                    .timeout(notice.lifetime().as_secs_f64().ceil() as u32)
                    .build();
                let editor = editor.clone();
                toast.connect_dismissed(move |_| editor.expire_notice(serial));
                self.toasts.add_toast(toast.clone());
                *self.toast.borrow_mut() = Some(toast);
            }
            None => {
                let shown = self.toast.borrow_mut().take();
                if let Some(shown) = shown {
                    shown.dismiss();
                }
            }
            Some(_) => {}
        }
    }

    /// The layout dialog is up while the editor says it is.
    fn update_layout_dialog(&self) {
        let editor = &self.editor;
        let open = self.layout_dialog.borrow().is_some();
        let sheet = editor.layout_sheet();
        match (editor.is_choosing_layout(), sheet, open) {
            (true, Some(sheet), false) => {
                let dialog = LayoutDialog::new(editor, &sheet).dialog;
                {
                    let (editor, stored) = (editor.clone(), self.layout_dialog.clone());
                    dialog.connect_closed(move |_| {
                        stored.borrow_mut().take();
                        if editor.is_choosing_layout() {
                            editor.set_choosing_layout(false);
                        }
                    });
                }
                *self.layout_dialog.borrow_mut() = Some(dialog.clone());
                dialog.present(Some(&self.window));
            }
            (false, _, true) | (true, None, true) => {
                let dialog = self.layout_dialog.borrow_mut().take();
                if let Some(dialog) = dialog {
                    dialog.force_close();
                }
            }
            _ => {}
        }
    }

    /// The question before a switch that would lose changes not saved: up while the editor has one waiting.
    fn update_switch_dialog(&self) {
        let open = self.switch_dialog.borrow().is_some();
        match (self.editor.pending_switch().is_some(), open) {
            (true, false) => {
                let alert = adw::AlertDialog::builder()
                    .heading(t("Switch Keyboards?"))
                    .body(t("Changes you haven’t saved will be lost."))
                    .build();
                alert.add_responses(&[("cancel", &t("Cancel")), ("switch", &t("Switch"))]);
                alert.set_response_appearance("switch", adw::ResponseAppearance::Destructive);
                alert.set_default_response(Some("cancel"));
                alert.set_close_response("cancel");
                {
                    let (editor, stored) = (self.editor.clone(), self.switch_dialog.clone());
                    alert.connect_response(None, move |_, response| {
                        stored.borrow_mut().take();
                        if response == "switch" {
                            editor.confirm_switch();
                        } else {
                            editor.cancel_switch();
                        }
                    });
                }
                *self.switch_dialog.borrow_mut() = Some(alert.clone());
                alert.present(Some(&self.window));
            }
            (false, true) => {
                let alert = self.switch_dialog.borrow_mut().take();
                if let Some(alert) = alert {
                    alert.force_close();
                }
            }
            _ => {}
        }
    }
}

/// While a simulated keyboard is in use, a bar across the window says so, loudly — nothing saved then reaches a
/// keyboard — with the way back to the keyboard on USB (the Simulator menu's own item).
fn simulator_bar() -> gtk::Box {
    let bar = gtk::Box::builder().spacing(14).visible(false).build();
    bar.add_css_class("simulator-bar");
    let icon =
        gtk::Image::builder().icon_name("dialog-warning-symbolic").pixel_size(22).valign(gtk::Align::Center).build();
    let title = gtk::Label::builder().label(t("Simulator in use")).xalign(0.0).build();
    title.add_css_class("title");
    let body = gtk::Label::builder()
        .label(t("This isn't a real keyboard. Nothing you save here is written to any keyboard."))
        .xalign(0.0)
        .wrap(true)
        .build();
    let texts = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(2).hexpand(true).build();
    texts.append(&title);
    texts.append(&body);
    let back = gtk::Button::builder()
        .label(t("Back to the Keyboard on USB"))
        .valign(gtk::Align::Center)
        .action_name("win.simulator")
        .action_target(&"".to_variant())
        .build();
    bar.append(&icon);
    bar.append(&texts);
    bar.append(&back);
    bar
}

fn simulator_item(choice: &SimulatorChoice) -> gio::MenuItem {
    let item = gio::MenuItem::new(Some(&simulator_kind_title(&choice.kind)), None);
    item.set_action_and_target_value(Some("win.simulator"), Some(&choice.id.to_variant()));
    item
}
