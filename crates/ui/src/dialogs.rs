//! The dialogs over the main window: the keyboard's shape, and how to give the app the keyboard.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use presentation::{EditorModel, LayoutSheetMode, LayoutSheetState, UDEV_RULES_PATH, udev_install_commands};

use crate::i18n::{t, tf};
use crate::shape::ShapeThumbnail;
use crate::widgets::set_active;

/// Sets the shape of a keyboard that can't say it, and how the keyboard is turned. It comes up on its own the
/// first time such a keyboard is plugged in — then without Cancel, there being nothing to go back to.
///
/// - A 884x keyboard picks one of the cross-platform app's layouts.
/// - The 1189:8890, which that app doesn't know and whose own tool draws no shape, is arranged: how many keys,
///   how many to a row, how many knobs.
/// - A keyboard that said its shape only turns.
pub struct LayoutDialog {
    pub dialog: adw::Dialog,
}

/// What the dialog has chosen so far.
#[derive(Clone, Debug)]
struct Choice {
    layout_id: Option<String>,
    quarter_turns: i64,
    keys: u8,
    columns: u8,
    knobs: u8,
}

impl LayoutDialog {
    pub fn new(editor: &Rc<EditorModel>, sheet: &LayoutSheetState) -> Self {
        let choice = Rc::new(RefCell::new(Choice {
            layout_id: None,
            quarter_turns: i64::from(sheet.quarter_turns),
            keys: 0,
            columns: 1,
            knobs: 0,
        }));
        if let LayoutSheetMode::Arrangement { keys, columns, knobs, .. } = sheet.mode {
            let mut choice = choice.borrow_mut();
            (choice.keys, choice.columns, choice.knobs) = (keys, columns, knobs);
        }

        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(16)
            .margin_top(6)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .build();
        let explanation = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let usb = gtk::Label::builder().label(&sheet.usb).xalign(0.0).build();
        usb.add_css_class("monospace");
        usb.add_css_class("dim-label");
        explanation.append(&usb);
        let reported = match (sheet.reported_keys, sheet.reported_knobs) {
            (Some(keys), Some(knobs)) => {
                tf("The keyboard reports its keys and knobs: %@", &[&tf("Keys: %lld · Knobs: %lld", &[&keys, &knobs])])
            }
            _ => t("The keyboard doesn’t say how many keys and knobs it has."),
        };
        let how = if matches!(sheet.mode, LayoutSheetMode::Arrangement { .. }) {
            t(
                "Its real shape isn’t known either. Set how many keys and knobs it has and how many keys are in a row, then rotate it to match how it sits in front of you.",
            )
        } else {
            t("Pick the shape that matches the keyboard, then rotate it to match how it sits in front of you.")
        };
        for text in [reported, how] {
            let label = gtk::Label::builder().label(text).xalign(0.0).wrap(true).build();
            label.add_css_class("dim-label");
            explanation.append(&label);
        }
        content.append(&explanation);

        let preview = ShapeThumbnail::new(16.0);
        let preview_box = gtk::Box::builder().hexpand(true).height_request(220).build();
        preview_box.add_css_class("preview-box");
        preview.widget().set_hexpand(true);
        preview.widget().set_vexpand(true);
        preview_box.append(preview.widget());
        let update_preview: Rc<dyn Fn()> = {
            let (editor, choice, preview, mode) = (editor.clone(), choice.clone(), preview.clone(), sheet.mode.clone());
            Rc::new(move || {
                let choice = choice.borrow();
                preview.set_drawing(match &mode {
                    LayoutSheetMode::Arrangement { .. } => {
                        let columns = choice.columns.min(choice.keys.max(1));
                        editor.preview_arrangement(choice.keys, columns, choice.knobs, choice.quarter_turns)
                    }
                    LayoutSheetMode::Catalog { selected_id, .. } => editor.preview_drawing(
                        Some(choice.layout_id.as_deref().unwrap_or(selected_id)),
                        choice.quarter_turns,
                    ),
                    LayoutSheetMode::Fixed => editor.preview_drawing(None, choice.quarter_turns),
                });
            })
        };

        match &sheet.mode {
            LayoutSheetMode::Arrangement { max_keys, max_knobs, .. } => {
                let row = gtk::Box::builder()
                    .orientation(gtk::Orientation::Horizontal)
                    .spacing(20)
                    .height_request(220)
                    .build();
                row.append(&preview_box);
                row.append(&arrangement(&choice, *max_keys, *max_knobs, update_preview.clone()));
                content.append(&row);
            }
            LayoutSheetMode::Catalog { choices, selected_id } => {
                let grid = gtk::FlowBox::builder()
                    .selection_mode(gtk::SelectionMode::None)
                    .homogeneous(true)
                    .min_children_per_line(3)
                    .max_children_per_line(8)
                    .row_spacing(8)
                    .column_spacing(8)
                    .build();
                let buttons: Rc<RefCell<Vec<(String, gtk::Button)>>> = Rc::default();
                for layout in choices {
                    let face = gtk::Box::new(gtk::Orientation::Vertical, 6);
                    let thumbnail = ShapeThumbnail::new(4.0);
                    thumbnail.set_drawing(Some(layout.drawing.clone()));
                    thumbnail.widget().set_size_request(80, 52);
                    face.append(thumbnail.widget());
                    let counts = gtk::Label::new(Some(&format!("{} + {}", layout.keys, layout.knobs)));
                    counts.add_css_class("caption");
                    counts.add_css_class("numeric");
                    face.append(&counts);
                    face.set_margin_top(6);
                    face.set_margin_bottom(6);
                    let button = gtk::Button::builder().child(&face).build();
                    button.add_css_class("keycap");
                    button.update_property(&[gtk::accessible::Property::Label(&tf(
                        "Keys: %lld · Knobs: %lld",
                        &[&layout.keys, &layout.knobs],
                    ))]);
                    set_active(&button, layout.id == *selected_id);
                    let (id, choice, buttons_handle, update_preview) =
                        (layout.id.clone(), choice.clone(), buttons.clone(), update_preview.clone());
                    button.connect_clicked(move |_| {
                        choice.borrow_mut().layout_id = Some(id.clone());
                        for (other, button) in buttons_handle.borrow().iter() {
                            set_active(button, *other == id);
                        }
                        update_preview();
                    });
                    buttons.borrow_mut().push((layout.id.clone(), button.clone()));
                    grid.append(&button);
                }
                let scroller = gtk::ScrolledWindow::builder()
                    .hscrollbar_policy(gtk::PolicyType::Never)
                    .height_request(220)
                    .child(&grid)
                    .build();
                content.append(&scroller);
                content.append(&preview_box);
            }
            LayoutSheetMode::Fixed => content.append(&preview_box),
        }

        let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        let rotate = gtk::Button::builder()
            .child(&adw::ButtonContent::builder().icon_name("object-rotate-right-symbolic").label(t("Rotate")).build())
            .build();
        let spacer = gtk::Box::builder().hexpand(true).build();
        let cancel = gtk::Button::with_label(&t("Cancel"));
        let done = gtk::Button::with_label(&t("Done"));
        done.add_css_class("suggested-action");
        buttons.append(&rotate);
        buttons.append(&spacer);
        if !sheet.is_required {
            buttons.append(&cancel);
        }
        buttons.append(&done);
        content.append(&buttons);

        let header = adw::HeaderBar::builder().show_end_title_buttons(!sheet.is_required).build();
        let toolbar = adw::ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(&content));
        let dialog = adw::Dialog::builder()
            .title(t("Choose the Keyboard’s Shape"))
            .content_width(680)
            .can_close(!sheet.is_required)
            .child(&toolbar)
            .default_widget(&done)
            .build();
        update_preview();

        {
            let (choice, update_preview) = (choice.clone(), update_preview.clone());
            rotate.connect_clicked(move |_| {
                let mut current = choice.borrow_mut();
                current.quarter_turns = (current.quarter_turns + 1) % 4;
                drop(current);
                update_preview();
            });
        }
        {
            let dialog = dialog.clone();
            cancel.connect_clicked(move |_| {
                dialog.close();
            });
        }
        {
            let (editor, mode) = (editor.clone(), sheet.mode.clone());
            done.connect_clicked(move |_| {
                let choice = choice.borrow().clone();
                match &mode {
                    LayoutSheetMode::Arrangement { .. } => {
                        let columns = choice.columns.min(choice.keys.max(1));
                        editor.choose_arrangement(choice.keys, columns, choice.knobs, choice.quarter_turns);
                    }
                    LayoutSheetMode::Catalog { selected_id, .. } => {
                        editor.choose_layout(
                            Some(choice.layout_id.as_deref().unwrap_or(selected_id)),
                            choice.quarter_turns,
                        );
                    }
                    LayoutSheetMode::Fixed => editor.choose_layout(None, choice.quarter_turns),
                }
                // Choosing ends the choice, and the window closes the dialog; this ends it even when the
                // keyboard went away meanwhile.
                editor.set_choosing_layout(false);
            });
        }
        Self { dialog }
    }
}

/// How many keys, how many to a row, how many knobs. A keyboard has at least one of either: taking the last one
/// away brings one of the other.
fn arrangement(choice: &Rc<RefCell<Choice>>, max_keys: u8, max_knobs: u8, changed: Rc<dyn Fn()>) -> gtk::Box {
    let column = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(18).width_request(200).build();
    let current = choice.borrow().clone();
    let (keys_box, keys) = count_slider(&t("Keys"), 0, max_keys, current.keys);
    let (columns_box, columns) = count_slider(&t("Keys per row"), 1, current.keys.max(2), current.columns);
    columns_box.set_sensitive(current.keys >= 2);
    let knobs_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    let knobs_title = gtk::Label::builder().label(t("Knobs")).xalign(0.0).build();
    knobs_title.add_css_class("dim-label");
    let knobs = adw::ToggleGroup::builder().halign(gtk::Align::Start).build();
    for count in 0..=max_knobs {
        knobs.add(adw::Toggle::builder().label(count.to_string()).build());
    }
    knobs.set_active(u32::from(current.knobs));
    knobs_box.append(&knobs_title);
    knobs_box.append(&knobs);
    column.append(&keys_box);
    column.append(&columns_box);
    column.append(&knobs_box);

    let settling = Rc::new(Cell::new(false));
    {
        let (choice, columns, columns_box, knobs, changed, settling) =
            (choice.clone(), columns.clone(), columns_box.clone(), knobs.clone(), changed.clone(), settling.clone());
        keys.connect_value_changed(move |scale| {
            if settling.get() {
                return;
            }
            settling.set(true);
            let value = scale.value().round() as u8;
            let mut current = choice.borrow_mut();
            current.keys = value;
            columns.adjustment().set_upper(f64::from(value.max(2)));
            current.columns = current.columns.min(value.max(1));
            columns.set_value(f64::from(current.columns));
            columns_box.set_sensitive(value >= 2);
            if value == 0 && current.knobs == 0 {
                current.knobs = 1;
                knobs.set_active(1);
            }
            drop(current);
            settling.set(false);
            changed();
        });
    }
    {
        let (choice, changed, settling) = (choice.clone(), changed.clone(), settling.clone());
        columns.connect_value_changed(move |scale| {
            if settling.get() {
                return;
            }
            choice.borrow_mut().columns = scale.value().round() as u8;
            changed();
        });
    }
    {
        let (choice, keys, settling) = (choice.clone(), keys.clone(), settling.clone());
        knobs.connect_active_notify(move |group| {
            if settling.get() {
                return;
            }
            let value = group.active() as u8;
            let mut current = choice.borrow_mut();
            current.knobs = value;
            let bring_key = value == 0 && current.keys == 0;
            drop(current);
            if bring_key {
                keys.set_value(1.0);
            }
            changed();
        });
    }
    column
}

/// A whole number on a slider that stops at each one, with the number beside its name.
fn count_slider(title: &str, lower: u8, upper: u8, value: u8) -> (gtk::Box, gtk::Scale) {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let name = gtk::Label::builder().label(title).xalign(0.0).hexpand(true).build();
    name.add_css_class("dim-label");
    let number = gtk::Label::new(Some(&value.to_string()));
    number.add_css_class("numeric");
    number.add_css_class("heading");
    row.append(&name);
    row.append(&number);
    let scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, f64::from(lower), f64::from(upper), 1.0);
    scale.set_round_digits(0);
    scale.set_digits(0);
    scale.set_value(f64::from(value));
    scale.update_property(&[gtk::accessible::Property::Label(title)]);
    scale.connect_value_changed(move |scale| number.set_label(&(scale.value().round() as u8).to_string()));
    container.append(&row);
    container.append(&scale);
    (container, scale)
}

/// How to give the signed-in user the keyboard: the udev rule, ready to copy into a terminal.
pub fn permission_dialog() -> adw::Dialog {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(6)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();
    let explanation = gtk::Label::builder()
        .label(tf(
            "Linux lets an app open a USB keyboard only when a udev rule gives the signed-in user its device. Run these commands in a terminal once — they write %@ — then plug the keyboard in again.",
            &[&UDEV_RULES_PATH],
        ))
        .xalign(0.0)
        .wrap(true)
        .build();
    content.append(&explanation);
    let commands = udev_install_commands();
    let view = gtk::TextView::builder()
        .editable(false)
        .monospace(true)
        .wrap_mode(gtk::WrapMode::None)
        .top_margin(8)
        .bottom_margin(8)
        .left_margin(8)
        .right_margin(8)
        .build();
    view.buffer().set_text(&commands);
    let scroller = gtk::ScrolledWindow::builder().height_request(260).child(&view).build();
    scroller.add_css_class("card");
    content.append(&scroller);
    let copy = gtk::Button::builder()
        .child(&adw::ButtonContent::builder().icon_name("edit-copy-symbolic").label(t("Copy")).build())
        .halign(gtk::Align::End)
        .build();
    copy.connect_clicked(move |button| button.clipboard().set_text(&commands));
    content.append(&copy);
    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&adw::HeaderBar::new());
    toolbar.set_content(Some(&content));
    adw::Dialog::builder().title(t("Allow Access to the Keyboard")).content_width(640).child(&toolbar).build()
}
