//! Small pieces the windows are built from.

use adw::prelude::*;
use presentation::KeyUsage;

use crate::names::key_name;

/// A titled group of controls in a page, with a note under it.
pub fn section(title: &str, note: Option<&str>, content: &impl IsA<gtk::Widget>) -> gtk::Box {
    let section = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let heading = gtk::Label::builder().label(title).xalign(0.0).build();
    heading.add_css_class("panel-section-title");
    section.append(&heading);
    section.append(content);
    if let Some(note) = note {
        let note = caption(note);
        section.append(&note);
    }
    section
}

/// Small, dimmed text that wraps.
pub fn caption(text: &str) -> gtk::Label {
    let label = gtk::Label::builder().label(text).xalign(0.0).wrap(true).build();
    label.add_css_class("caption");
    label.add_css_class("dim-label");
    label
}

/// A key cap in the action pages.
pub fn keycap(label: &str) -> gtk::Button {
    let button = gtk::Button::with_label(label);
    button.add_css_class("keycap");
    button
}

/// A key cap with an icon above its title.
pub fn tall_keycap(icon: &str, title: &str) -> gtk::Button {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 3);
    content.append(&gtk::Image::from_icon_name(icon));
    content.append(&gtk::Label::builder().label(title).wrap(true).justify(gtk::Justification::Center).build());
    let button = gtk::Button::builder().child(&content).build();
    button.add_css_class("keycap");
    button.add_css_class("tall");
    button
}

/// A key cap's face: the key, and above it the character it makes with Shift.
pub fn key_button(key: &KeyUsage, compact: bool) -> gtk::Button {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.set_valign(gtk::Align::Center);
    if let Some(shifted) = key.shifted {
        let shifted = gtk::Label::new(Some(shifted));
        shifted.add_css_class("shifted");
        content.append(&shifted);
    }
    let label = gtk::Label::new(Some(key.label));
    if compact {
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);
    }
    content.append(&label);
    let button = gtk::Button::builder().child(&content).tooltip_text(key_name(key)).build();
    button.add_css_class("keycap");
    if compact {
        button.add_css_class("compact");
    }
    button
}

/// Marks a cap as the one in use.
pub fn set_active(button: &impl IsA<gtk::Widget>, active: bool) {
    if active {
        button.add_css_class("active");
    } else {
        button.remove_css_class("active");
    }
}

/// Buttons that wrap onto as many lines as they need.
pub fn wrap_box() -> adw::WrapBox {
    adw::WrapBox::builder().child_spacing(4).line_spacing(4).build()
}

/// Removes every child of a box.
pub fn clear(container: &gtk::Box) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
}
