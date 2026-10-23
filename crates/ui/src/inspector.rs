//! The selected key or knob action: what it does, and clearing it.

use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use presentation::{ActionCaption, BindingState, EditorModel, SelectionState};

use crate::i18n::{t, tf};
use crate::names::{caption_text, control_title, media_icon, media_title, mouse_icon, mouse_title};
use crate::widgets::{caption, clear, wrap_box};

pub struct Inspector {
    root: gtk::Box,
    editor: Rc<EditorModel>,
    selected: gtk::Box,
    placeholder: gtk::Label,
    held: gtk::Label,
    clear_key: gtk::Button,
    clear_layer: gtk::Button,
    unsaved: gtk::Label,
    shown: RefCell<Option<Option<SelectionState>>>,
}

impl Inspector {
    pub fn new(editor: Rc<EditorModel>) -> Self {
        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(14)
            .width_request(270)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .build();
        let selected = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let placeholder = gtk::Label::builder().label(t("Select a key or knob.")).xalign(0.0).wrap(true).build();
        placeholder.add_css_class("dim-label");
        let held = gtk::Label::builder().xalign(0.0).wrap(true).visible(false).build();
        held.add_css_class("caption");
        held.add_css_class("accent");
        let spacer = gtk::Box::builder().vexpand(true).build();
        let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        let clear_key = gtk::Button::with_label(&t("Clear"));
        let clear_layer = gtk::Button::with_label(&t("Clear Layer"));
        buttons.append(&clear_key);
        buttons.append(&clear_layer);
        let unsaved = gtk::Label::builder().xalign(0.0).build();
        unsaved.add_css_class("caption");
        root.append(&placeholder);
        root.append(&selected);
        root.append(&held);
        root.append(&spacer);
        root.append(&buttons);
        root.append(&unsaved);
        {
            let editor = editor.clone();
            clear_key.connect_clicked(move |_| editor.clear_selected());
        }
        {
            let editor = editor.clone();
            clear_layer.connect_clicked(move |_| editor.clear_layer());
        }
        Self { root, editor, selected, placeholder, held, clear_key, clear_layer, unsaved, shown: RefCell::default() }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.root
    }

    pub fn refresh(&self) {
        let editor = &self.editor;
        let state = editor.selection_state();
        if self.shown.borrow().as_ref() != Some(&state) {
            clear(&self.selected);
            self.placeholder.set_visible(state.is_none());
            if let Some(state) = &state {
                self.build(state);
            }
            *self.shown.borrow_mut() = Some(state);
        }
        match editor.held_modifier_text() {
            Some(held) => {
                self.held.set_label(&tf("Next key with %@", &[&held]));
                self.held.set_visible(true);
            }
            None => self.held.set_visible(false),
        }
        let ready = editor.is_ready();
        self.clear_key.set_sensitive(ready && editor.selected_control().is_some());
        self.clear_layer.set_sensitive(ready);
        if editor.has_unsaved_changes() {
            self.unsaved.set_label(&format!("● {}", t("Unsaved changes")));
            self.unsaved.add_css_class("unsaved");
            self.unsaved.remove_css_class("dim-label");
            self.unsaved.set_visible(true);
        } else if ready {
            self.unsaved.set_label(&format!("✓ {}", t("No unsaved changes")));
            self.unsaved.remove_css_class("unsaved");
            self.unsaved.add_css_class("dim-label");
            self.unsaved.set_visible(true);
        } else {
            self.unsaved.set_visible(false);
        }
    }

    fn build(&self, state: &SelectionState) {
        let title = gtk::Label::builder().label(control_title(state.title)).xalign(0.0).wrap(true).build();
        title.add_css_class("title-3");
        self.selected.append(&title);
        if let Some(layer) = state.layer_number {
            let layer = gtk::Label::builder().label(tf("Layer %lld", &[&layer])).xalign(0.0).build();
            layer.add_css_class("caption");
            layer.add_css_class("dim-label");
            self.selected.append(&layer);
        }
        self.selected.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        match &state.binding {
            BindingState::Unknown => {
                let text = t(
                    "Not known — this keyboard doesn’t tell what its keys do. Whatever you set here is written when you save.",
                );
                let label = gtk::Label::builder().label(text).xalign(0.0).wrap(true).build();
                label.add_css_class("dim-label");
                self.selected.append(&label);
            }
            BindingState::Nothing => {
                let label = gtk::Label::builder().label(t("Nothing")).xalign(0.0).build();
                label.add_css_class("dim-label");
                self.selected.append(&label);
            }
            BindingState::Presses { strokes, key_delay, used, limit } => {
                self.presses(strokes, *key_delay, *used, *limit)
            }
            BindingState::Media(media) => self.selected.append(&large(media_icon(*media), &media_title(*media))),
            BindingState::OtherMedia(usage) => {
                let label =
                    gtk::Label::builder().label(caption_text(&ActionCaption::OtherMedia(*usage))).xalign(0.0).build();
                label.add_css_class("title-4");
                self.selected.append(&label);
            }
            BindingState::Mouse(mouse) => self.selected.append(&large(mouse_icon(*mouse), &mouse_title(*mouse))),
        }
    }

    fn presses(&self, strokes: &[String], key_delay: u32, used: usize, limit: usize) {
        let chips = wrap_box();
        for stroke in strokes {
            let chip = gtk::Label::builder().label(stroke).selectable(true).build();
            chip.add_css_class("stroke-chip");
            chip.add_css_class("title-4");
            chips.append(&chip);
        }
        self.selected.append(&chips);
        if key_delay > 0 {
            self.selected.append(&caption(&tf("Waits %lld ms", &[&key_delay])));
        }
        self.selected.append(&caption(&tf("%lld of %lld presses", &[&used, &limit])));
        let remove = gtk::Button::builder().halign(gtk::Align::Start).sensitive(!strokes.is_empty()).build();
        remove.set_child(Some(
            &adw::ButtonContent::builder().icon_name("edit-undo-symbolic").label(t("Remove Last Press")).build(),
        ));
        remove.add_css_class("small");
        let editor = self.editor.clone();
        remove.connect_clicked(move |_| editor.remove_last_stroke());
        self.selected.append(&remove);
    }
}

fn large(icon: &str, title: &str) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    row.append(&gtk::Image::builder().icon_name(icon).pixel_size(20).build());
    let label = gtk::Label::new(Some(title));
    label.add_css_class("title-4");
    row.append(&label);
    row
}
