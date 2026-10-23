//! The connected keyboard as the view model draws it, turned the way it sits. The body and the caps are
//! drawn; over each cap lies a button — for clicking, focus, its tooltip and its name for assistive tools. A
//! key shows its number and what it does.

use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene, gsk};
use presentation::{ControlDrawing, ControlFace, ControlRef, KeyboardDrawing, KnobAction, LayoutRect};

use crate::names::{caption_text, control_title};

/// Where a drawing in layout space lands in a widget: scaled to fit, in the middle.
#[derive(Clone, Copy, Debug)]
pub struct Placement {
    scale: f64,
    origin_x: f64,
    origin_y: f64,
    bounds: LayoutRect,
}

impl Placement {
    pub fn fit(bounds: LayoutRect, width: f64, height: f64, margin: f64) -> Self {
        let scale = ((width - 2.0 * margin) / bounds.width).min((height - 2.0 * margin) / bounds.height).max(0.01);
        Self {
            scale,
            origin_x: (width - bounds.width * scale) / 2.0,
            origin_y: (height - bounds.height * scale) / 2.0,
            bounds,
        }
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn rect(&self, rect: &LayoutRect) -> graphene::Rect {
        graphene::Rect::new(
            (self.origin_x + (rect.x - self.bounds.x) * self.scale) as f32,
            (self.origin_y + (rect.y - self.bounds.y) * self.scale) as f32,
            (rect.width * self.scale) as f32,
            (rect.height * self.scale) as f32,
        )
    }
}

/// What a click on a control does.
type SelectHandler = Rc<dyn Fn(ControlRef)>;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct KeyboardCanvas {
        pub(super) drawing: RefCell<Option<KeyboardDrawing>>,
        pub(super) buttons: RefCell<Vec<gtk::Button>>,
        pub(super) on_select: RefCell<Option<SelectHandler>>,
    }

    impl std::fmt::Debug for KeyboardCanvas {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("KeyboardCanvas").field("drawing", &self.drawing.borrow().is_some()).finish_non_exhaustive()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for KeyboardCanvas {
        const NAME: &'static str = "MkKeyboardCanvas";
        type Type = super::KeyboardCanvas;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for KeyboardCanvas {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for KeyboardCanvas {
        fn measure(&self, orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            match orientation {
                gtk::Orientation::Horizontal => (240, 720, -1, -1),
                _ => (200, 300, -1, -1),
            }
        }

        fn size_allocate(&self, width: i32, height: i32, _baseline: i32) {
            let drawing = self.drawing.borrow();
            let Some(drawing) = drawing.as_ref() else { return };
            let placement = Placement::fit(drawing.bounds, f64::from(width), f64::from(height), 14.0);
            for (button, control) in self.buttons.borrow().iter().zip(&drawing.controls) {
                let rect = placement.rect(&control.rect);
                let (minimum_width, ..) = button.measure(gtk::Orientation::Horizontal, -1);
                let (minimum_height, ..) = button.measure(gtk::Orientation::Vertical, -1);
                let width = (rect.width().round() as i32).max(minimum_width);
                let height = (rect.height().round() as i32).max(minimum_height);
                let x = (rect.x() + rect.width() / 2.0).round() as i32 - width / 2;
                let y = (rect.y() + rect.height() / 2.0).round() as i32 - height / 2;
                button.size_allocate(&gtk::Allocation::new(x, y, width, height), -1);
            }
        }

        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = self.obj();
            if let Some(drawing) = self.drawing.borrow().as_ref() {
                let placement =
                    Placement::fit(drawing.bounds, f64::from(widget.width()), f64::from(widget.height()), 14.0);
                draw_body(snapshot, &placement, drawing);
                let accent = adw::StyleManager::default().accent_color_rgba();
                for control in &drawing.controls {
                    draw_control(snapshot, &placement, control, &accent);
                }
            }
            let mut child = widget.first_child();
            while let Some(current) = child {
                widget.snapshot_child(&current, snapshot);
                child = current.next_sibling();
            }
        }
    }
}

glib::wrapper! {
    pub struct KeyboardCanvas(ObjectSubclass<imp::KeyboardCanvas>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for KeyboardCanvas {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl KeyboardCanvas {
    pub fn connect_select(&self, handler: impl Fn(ControlRef) + 'static) {
        *self.imp().on_select.borrow_mut() = Some(Rc::new(handler));
    }

    /// Draws `drawing`; the buttons are made again only when the controls are others.
    pub fn set_drawing(&self, drawing: Option<KeyboardDrawing>) {
        let imp = self.imp();
        let same_controls = {
            let current = imp.drawing.borrow();
            match (current.as_ref(), drawing.as_ref()) {
                (Some(current), Some(new)) => {
                    current.controls.len() == new.controls.len()
                        && current
                            .controls
                            .iter()
                            .zip(&new.controls)
                            .all(|(a, b)| a.control == b.control && a.face == b.face)
                }
                (None, None) => true,
                _ => false,
            }
        };
        if !same_controls {
            for button in imp.buttons.borrow_mut().drain(..) {
                button.unparent();
            }
            if let Some(drawing) = &drawing {
                let buttons = drawing.controls.iter().map(|control| self.make_button(control)).collect();
                *imp.buttons.borrow_mut() = buttons;
            }
        }
        if let Some(drawing) = &drawing {
            for (button, control) in imp.buttons.borrow().iter().zip(&drawing.controls) {
                update_button(button, control);
            }
        }
        *imp.drawing.borrow_mut() = drawing;
        self.queue_allocate();
        self.queue_draw();
    }

    fn make_button(&self, control: &ControlDrawing) -> gtk::Button {
        let button = gtk::Button::new();
        button.add_css_class("canvas-control");
        match control.face {
            ControlFace::Key(number) => {
                let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
                let number = gtk::Label::builder()
                    .label(number.to_string())
                    .halign(gtk::Align::Start)
                    .margin_start(6)
                    .margin_top(3)
                    .build();
                number.add_css_class("key-number");
                let caption = gtk::Label::builder()
                    .wrap(true)
                    .wrap_mode(gtk::pango::WrapMode::WordChar)
                    .lines(3)
                    .ellipsize(gtk::pango::EllipsizeMode::End)
                    .justify(gtk::Justification::Center)
                    .vexpand(true)
                    .valign(gtk::Align::Center)
                    .margin_start(3)
                    .margin_end(3)
                    .max_width_chars(8)
                    .build();
                caption.add_css_class("caption");
                content.append(&number);
                content.append(&caption);
                button.set_child(Some(&content));
            }
            ControlFace::Knob(action) => {
                button.add_css_class("knob");
                let icon = match action {
                    KnobAction::RotateLeft | KnobAction::PressRotateLeft => "object-rotate-left-symbolic",
                    KnobAction::Press => "media-record-symbolic",
                    KnobAction::RotateRight | KnobAction::PressRotateRight => "object-rotate-right-symbolic",
                };
                button.set_child(Some(&gtk::Image::from_icon_name(icon)));
            }
        }
        let target = control.control;
        let canvas = self.downgrade();
        button.connect_clicked(move |_| {
            let Some(canvas) = canvas.upgrade() else { return };
            let handler = canvas.imp().on_select.borrow().clone();
            if let Some(handler) = handler {
                handler(target);
            }
        });
        button.set_parent(self);
        button
    }
}

fn update_button(button: &gtk::Button, control: &ControlDrawing) {
    let title = control_title(control.control.title());
    let caption = caption_text(&control.caption);
    let tooltip = if control.caption.is_assigned() { format!("{title}: {caption}") } else { title.clone() };
    button.set_tooltip_text(Some(&tooltip));
    button.update_property(&[gtk::accessible::Property::Label(&tooltip)]);
    if control.led_color.is_some() {
        button.add_css_class("lit");
    } else {
        button.remove_css_class("lit");
    }
    if let ControlFace::Key(_) = control.face
        && let Some(caption_label) =
            button.child().and_then(|content| content.last_child()).and_downcast::<gtk::Label>()
    {
        caption_label.set_label(&caption);
    }
}

fn rgba(white: f32, alpha: f32) -> gdk::RGBA {
    gdk::RGBA::new(white, white, white, alpha)
}

/// The keyboard's body: a dark slab with a soft shadow.
fn draw_body(snapshot: &gtk::Snapshot, placement: &Placement, drawing: &KeyboardDrawing) {
    let rect = placement.rect(&drawing.frame);
    let rounded = gsk::RoundedRect::from_rect(rect, (24.0 * placement.scale()) as f32);
    snapshot.append_outset_shadow(&rounded, &gdk::RGBA::new(0.0, 0.0, 0.0, 0.3), 0.0, 3.0, 0.0, 8.0);
    snapshot.push_rounded_clip(&rounded);
    snapshot.append_linear_gradient(
        &rect,
        &graphene::Point::new(rect.x(), rect.y()),
        &graphene::Point::new(rect.x(), rect.y() + rect.height()),
        &[gsk::ColorStop::new(0.0, rgba(0.28, 1.0)), gsk::ColorStop::new(1.0, rgba(0.19, 1.0))],
    );
    snapshot.pop();
}

/// A key's cap or a knob's action, its outline showing whether it is selected and whether it changed.
fn draw_control(snapshot: &gtk::Snapshot, placement: &Placement, control: &ControlDrawing, accent: &gdk::RGBA) {
    let rect = placement.rect(&control.rect);
    let (radius, top, bottom, outline, outline_width) = match control.face {
        ControlFace::Key(_) => {
            let (top, bottom) = match control.led_color {
                Some(color) => {
                    let base = gdk::RGBA::new(
                        f32::from(color.red) / 255.0,
                        f32::from(color.green) / 255.0,
                        f32::from(color.blue) / 255.0,
                        1.0,
                    );
                    (lighten(&base, 0.12), base)
                }
                None => (rgba(0.46, 1.0), rgba(0.38, 1.0)),
            };
            let (outline, width) = if control.is_selected { (*accent, 3.0) } else { (rgba(1.0, 0.15), 1.0) };
            ((10.0 * placement.scale()) as f32, top, bottom, outline, width)
        }
        ControlFace::Knob(action) => {
            let shade = if action == KnobAction::Press { 0.46 } else { 0.36 };
            let (outline, width) = if control.is_selected {
                (*accent, 3.0)
            } else if control.caption.is_assigned() {
                (gdk::RGBA::new(0.2, 0.82, 0.48, 0.75), 1.5)
            } else {
                (rgba(1.0, 0.15), 1.5)
            };
            (rect.width().min(rect.height()) / 2.0, rgba(shade + 0.04, 1.0), rgba(shade - 0.04, 1.0), outline, width)
        }
    };
    let rounded = gsk::RoundedRect::from_rect(rect, radius);
    snapshot.push_rounded_clip(&rounded);
    snapshot.append_linear_gradient(
        &rect,
        &graphene::Point::new(rect.x(), rect.y()),
        &graphene::Point::new(rect.x(), rect.y() + rect.height()),
        &[gsk::ColorStop::new(0.0, top), gsk::ColorStop::new(1.0, bottom)],
    );
    snapshot.pop();
    snapshot.append_border(&rounded, &[outline_width; 4], &[outline; 4]);
    if control.is_changed {
        let inset = if matches!(control.face, ControlFace::Key(_)) { 5.0 } else { 1.0 };
        let dot = graphene::Rect::new(rect.x() + rect.width() - inset - 7.0, rect.y() + inset, 7.0, 7.0);
        let dot = gsk::RoundedRect::from_rect(dot, 3.5);
        snapshot.push_rounded_clip(&dot);
        snapshot.append_color(&gdk::RGBA::new(1.0, 0.47, 0.0, 1.0), dot.bounds());
        snapshot.pop();
    }
}

fn lighten(color: &gdk::RGBA, amount: f32) -> gdk::RGBA {
    let up = |channel: f32| channel + (1.0 - channel) * amount;
    gdk::RGBA::new(up(color.red()), up(color.green()), up(color.blue()), color.alpha())
}
