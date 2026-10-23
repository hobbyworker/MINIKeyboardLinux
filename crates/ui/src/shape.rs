//! A keyboard's shape drawn small, for choosing one: keys, and each knob as one body.

use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

use gtk::prelude::*;
use presentation::ShapeDrawing;

use crate::canvas::Placement;

/// A drawing area that draws a [`ShapeDrawing`], and draws again when it is given another.
#[derive(Clone, Debug)]
pub struct ShapeThumbnail {
    area: gtk::DrawingArea,
    drawing: Rc<RefCell<Option<ShapeDrawing>>>,
}

impl ShapeThumbnail {
    pub fn new(margin: f64) -> Self {
        let area = gtk::DrawingArea::new();
        let drawing: Rc<RefCell<Option<ShapeDrawing>>> = Rc::default();
        let shown = drawing.clone();
        area.set_draw_func(move |area, context, width, height| {
            let Some(drawing) = shown.borrow().clone() else { return };
            let placement = Placement::fit(drawing.bounds, f64::from(width), f64::from(height), margin);
            let color = area.color();
            context.set_source_rgba(f64::from(color.red()), f64::from(color.green()), f64::from(color.blue()), 0.45);
            for item in &drawing.items {
                let rect = placement.rect(&item.rect);
                let (x, y, w, h) =
                    (f64::from(rect.x()), f64::from(rect.y()), f64::from(rect.width()), f64::from(rect.height()));
                if item.is_knob {
                    context.arc(x + w / 2.0, y + h / 2.0, w.min(h) / 2.0, 0.0, 2.0 * PI);
                } else {
                    rounded_rectangle(context, x, y, w, h, (8.0 * placement.scale()).min(w / 2.0));
                }
                let _ = context.fill();
            }
        });
        Self { area, drawing }
    }

    pub fn widget(&self) -> &gtk::DrawingArea {
        &self.area
    }

    pub fn set_drawing(&self, drawing: Option<ShapeDrawing>) {
        *self.drawing.borrow_mut() = drawing;
        self.area.queue_draw();
    }
}

fn rounded_rectangle(context: &gtk::cairo::Context, x: f64, y: f64, width: f64, height: f64, radius: f64) {
    context.new_sub_path();
    context.arc(x + width - radius, y + radius, radius, -PI / 2.0, 0.0);
    context.arc(x + width - radius, y + height - radius, radius, 0.0, PI / 2.0);
    context.arc(x + radius, y + height - radius, radius, PI / 2.0, PI);
    context.arc(x + radius, y + radius, radius, PI, 3.0 * PI / 2.0);
    context.close_path();
}
