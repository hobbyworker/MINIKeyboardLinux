//! The window's header: the keyboard's state — with several plugged in, which one is open — the layer, and reading
//! and saving.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use presentation::{Activity, ConnectionStatus, EditorModel, Firmware, KeyboardInfo};

use crate::i18n::{t, tf};
use crate::names::{channel_text, commands_text, error_code, interface_role_title};

pub struct Header {
    pub bar: adw::HeaderBar,
    editor: Rc<EditorModel>,
    dot: gtk::Box,
    title: gtk::Label,
    detail: gtk::Label,
    keyboards: gtk::MenuButton,
    pub(crate) info: gtk::MenuButton,
    identify: gtk::Button,
    retry: gtk::Button,
    pub allow: gtk::Button,
    spinner: adw::Spinner,
    layers: adw::ToggleGroup,
    layer_count: Cell<usize>,
    updating: Rc<Cell<bool>>,
    read: gtk::Button,
    pub menu: gtk::MenuButton,
}

impl Header {
    pub fn new(editor: Rc<EditorModel>) -> Self {
        let bar = adw::HeaderBar::new();
        let status = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let dot = gtk::Box::builder().valign(gtk::Align::Center).build();
        dot.add_css_class("status-dot");
        let texts = gtk::Box::builder().orientation(gtk::Orientation::Vertical).valign(gtk::Align::Center).build();
        let title = gtk::Label::builder().xalign(0.0).build();
        title.add_css_class("heading");
        let detail = gtk::Label::builder().xalign(0.0).ellipsize(gtk::pango::EllipsizeMode::End).build();
        detail.add_css_class("caption");
        detail.add_css_class("dim-label");
        texts.append(&title);
        texts.append(&detail);
        let keyboards = gtk::MenuButton::builder()
            .icon_name("pan-down-symbolic")
            .tooltip_text(t("Choose a Keyboard"))
            .valign(gtk::Align::Center)
            .build();
        keyboards.add_css_class("flat");
        keyboards.update_property(&[gtk::accessible::Property::Label(&t("Choose a Keyboard"))]);
        let info = gtk::MenuButton::builder()
            .icon_name("help-about-symbolic")
            .tooltip_text(t("Keyboard Information"))
            .valign(gtk::Align::Center)
            .build();
        info.add_css_class("flat");
        info.update_property(&[gtk::accessible::Property::Label(&t("Keyboard Information"))]);
        let identify = gtk::Button::builder().label(t("Retry")).valign(gtk::Align::Center).build();
        let retry = gtk::Button::builder().label(t("Retry")).valign(gtk::Align::Center).build();
        let allow = gtk::Button::builder().label(t("How to Allow…")).valign(gtk::Align::Center).build();
        let spinner = adw::Spinner::new();
        status.append(&dot);
        status.append(&texts);
        status.append(&keyboards);
        status.append(&info);
        status.append(&allow);
        status.append(&identify);
        status.append(&retry);
        status.append(&spinner);
        bar.pack_start(&status);

        let layers = adw::ToggleGroup::new();
        bar.set_title_widget(Some(&layers));

        let menu = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text(t("Main Menu"))
            .primary(true)
            .build();
        let save =
            gtk::Button::builder().label(t("Save")).tooltip_text(t("Save to Keyboard")).action_name("win.save").build();
        save.add_css_class("suggested-action");
        let read = gtk::Button::builder()
            .label(t("Read"))
            .tooltip_text(t("Read from Keyboard"))
            .action_name("win.read")
            .build();
        bar.pack_end(&menu);
        bar.pack_end(&save);
        bar.pack_end(&read);

        let popover = gtk::Popover::new();
        info.set_popover(Some(&popover));
        {
            let editor = editor.clone();
            popover.connect_show(move |popover| {
                popover.set_child(editor.keyboard_info().map(|info| info_view(&info)).as_ref());
            });
        }
        let popover = gtk::Popover::new();
        keyboards.set_popover(Some(&popover));
        {
            let editor = editor.clone();
            popover.connect_show(move |popover| popover.set_child(Some(&keyboard_list(&editor, popover))));
        }
        {
            let editor = editor.clone();
            identify.connect_clicked(move |_| {
                let editor = editor.clone();
                gtk::glib::spawn_future_local(async move { editor.identify().await });
            });
        }
        {
            let editor = editor.clone();
            retry.connect_clicked(move |_| editor.retry_connection());
        }
        let updating = Rc::new(Cell::new(false));
        {
            let (editor, updating) = (editor.clone(), updating.clone());
            layers.connect_active_notify(move |layers| {
                if !updating.get() {
                    editor.set_layer(layers.active() as usize);
                }
            });
        }
        Self {
            bar,
            editor,
            dot,
            title,
            detail,
            keyboards,
            info,
            identify,
            retry,
            allow,
            spinner,
            layers,
            layer_count: Cell::new(0),
            updating,
            read,
            menu,
        }
    }

    pub fn refresh(&self) {
        let editor = &self.editor;
        let status = editor.status();
        let activity = editor.activity();
        for class in ["connected", "simulated", "identifying", "permission", "disconnected"] {
            self.dot.remove_css_class(class);
        }
        self.dot.add_css_class(match &status {
            ConnectionStatus::Connected { is_simulated: true, .. } => "simulated",
            ConnectionStatus::Connected { is_identified: true, .. } => "connected",
            ConnectionStatus::Connected { .. } => "identifying",
            ConnectionStatus::NeedsPermission => "permission",
            ConnectionStatus::Searching | ConnectionStatus::OpenFailed { .. } => "disconnected",
        });
        let (title, detail) = match &status {
            ConnectionStatus::Searching => (t("Not Connected"), t("Connect the keyboard via USB.")),
            ConnectionStatus::NeedsPermission => (
                t("Access to the keyboard is needed"),
                t("Add the udev rule for this keyboard, then plug it in again."),
            ),
            ConnectionStatus::OpenFailed { code } => (t("The keyboard couldn’t be opened"), error_code(*code)),
            ConnectionStatus::Connected { usb, keys, knobs, is_simulated, .. } => {
                let title = match activity {
                    Some(Activity::Identifying) => t("Asking the keyboard what it is…"),
                    Some(Activity::Reading) => t("Reading…"),
                    Some(Activity::Saving) => t("Saving…"),
                    None if *is_simulated => t("Simulated Keyboard"),
                    None => t("Connected"),
                };
                let detail = match (keys, knobs) {
                    (Some(keys), Some(knobs)) => tf("%@ · %@", &[usb, &tf("Keys: %lld · Knobs: %lld", &[keys, knobs])]),
                    _ => usb.clone(),
                };
                (title, detail)
            }
        };
        self.title.set_label(&title);
        self.detail.set_label(&detail);
        let connected = matches!(status, ConnectionStatus::Connected { .. });
        self.keyboards.set_visible(!editor.keyboard_choices().is_empty());
        self.keyboards.set_sensitive(editor.can_switch_keyboard());
        self.info.set_visible(connected && editor.keyboard_info().is_some());
        self.identify.set_visible(
            matches!(status, ConnectionStatus::Connected { is_identified: false, .. }) && activity.is_none(),
        );
        self.allow.set_visible(status == ConnectionStatus::NeedsPermission);
        self.retry
            .set_visible(matches!(status, ConnectionStatus::NeedsPermission | ConnectionStatus::OpenFailed { .. }));
        self.spinner.set_visible(activity.is_some());

        let count = editor.layer_count();
        self.updating.set(true);
        if self.layer_count.get() != count {
            self.layers.remove_all();
            for layer in 1..=count {
                self.layers.add(adw::Toggle::builder().label(tf("Layer %lld", &[&layer])).build());
            }
            self.layer_count.set(count);
        }
        self.layers.set_visible(count > 1);
        self.layers.set_active(editor.layer() as u32);
        self.layers.set_sensitive(editor.is_ready());
        self.updating.set(false);
        self.read.set_visible(editor.reads_back());
    }
}

/// The keyboards plugged in, the open one checked; choosing another opens it.
fn keyboard_list(editor: &Rc<EditorModel>, popover: &gtk::Popover) -> gtk::Box {
    let list = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let heading = gtk::Label::builder().label(t("Choose a Keyboard")).xalign(0.0).build();
    heading.add_css_class("heading");
    list.append(&heading);
    let mut group: Option<gtk::CheckButton> = None;
    for choice in editor.keyboard_choices() {
        let button = gtk::CheckButton::with_label(&choice.title);
        button.set_group(group.as_ref());
        button.set_active(choice.is_selected);
        group.get_or_insert_with(|| button.clone());
        let (editor, popover) = (editor.clone(), popover.clone());
        button.connect_toggled(move |button| {
            if button.is_active() {
                editor.select_keyboard(&choice.id);
                popover.popdown();
            }
        });
        list.append(&button);
    }
    list
}

/// What is known about the keyboard plugged in. Nothing is guessed: what the keyboard doesn't say shows as not
/// provided.
fn info_view(info: &KeyboardInfo) -> gtk::Box {
    let root = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .width_request(420)
        .build();
    let heading = gtk::Label::builder().label(t("Keyboard Information")).xalign(0.0).build();
    heading.add_css_class("heading");
    root.append(&heading);
    let grid = gtk::Grid::builder().column_spacing(14).row_spacing(8).build();
    let row = RefCell::new(0);
    let add = |title: &str, value: gtk::Widget| {
        let title = gtk::Label::builder().label(title).xalign(1.0).yalign(0.0).build();
        title.add_css_class("dim-label");
        let index = *row.borrow();
        grid.attach(&title, 0, index, 1, 1);
        grid.attach(&value, 1, index, 1, 1);
        *row.borrow_mut() += 1;
    };
    let text = |value: &str| -> gtk::Widget {
        gtk::Label::builder().label(value).xalign(0.0).selectable(true).wrap(true).build().upcast()
    };
    let provided = |value: &Option<String>| -> gtk::Widget {
        match value {
            Some(value) => text(value),
            None => {
                let label = gtk::Label::builder().label(t("Not provided by the keyboard")).xalign(0.0).build();
                label.add_css_class("dim-label");
                label.upcast()
            }
        }
    };
    let usb_id = gtk::Label::builder().label(&info.usb_id).xalign(0.0).selectable(true).build();
    usb_id.add_css_class("monospace");
    add("USB ID", usb_id.upcast());
    add(&t("Name"), provided(&info.name));
    add(&t("Manufacturer"), provided(&info.manufacturer));
    add(&t("Serial Number"), provided(&info.serial_number));
    // No USB descriptor names the chip.
    add(&t("Chip"), provided(&None));
    add(
        &t("Firmware"),
        match &info.firmware {
            Firmware::Reported { kind, keys, knobs } => {
                let column = gtk::Box::new(gtk::Orientation::Vertical, 2);
                column.append(&text(&kind.to_string()));
                let detail =
                    gtk::Label::builder().label(tf("Keys: %lld · Knobs: %lld", &[keys, knobs])).xalign(0.0).build();
                detail.add_css_class("caption");
                detail.add_css_class("dim-label");
                column.append(&detail);
                column.upcast()
            }
            Firmware::Release(release) => text(release),
            Firmware::Unknown => provided(&None),
        },
    );
    if let Some(usb) = &info.usb {
        add("USB", text(usb));
    }
    if let Some(power) = info.power {
        add(&t("Power"), text(&format!("{power}\u{00A0}mA")));
    }
    if !info.interfaces.is_empty() {
        let column = gtk::Box::new(gtk::Orientation::Vertical, 3);
        for interface in &info.interfaces {
            let roles: Vec<String> = interface.roles.iter().map(|role| interface_role_title(*role)).collect();
            let mark = if interface.is_configuration { "  ←" } else { "" };
            column.append(&text(&format!("{}  {}{mark}", interface.number, roles.join(", "))));
        }
        add(&t("Interfaces"), column.upcast());
    }
    add(&t("Configuration Channel"), text(&channel_text(&info.channel)));
    add(&t("Commands"), text(&commands_text(info.commands)));
    root.append(&grid);
    root
}
