//! Where a layout's controls are drawn, and the keyboard's body around them.

mod app_geometry;

use std::collections::{BTreeMap, HashMap};

use domain::{ControlId, GridShape, KeyboardLayout, KnobPart, Rotation};

/// A rectangle in a layout's drawing space.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl LayoutRect {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    pub fn max_x(&self) -> f64 {
        self.x + self.width
    }

    pub fn max_y(&self) -> f64 {
        self.y + self.height
    }

    pub fn mid_x(&self) -> f64 {
        self.x + self.width / 2.0
    }

    pub fn mid_y(&self) -> f64 {
        self.y + self.height / 2.0
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        (self.x..=self.max_x()).contains(&x) && (self.y..=self.max_y()).contains(&y)
    }
}

/// Room between a knob and its turns, in the app's layouts' space (a key there is 80 or so).
pub(crate) const TURN_GAP: f64 = 4.0;

/// How far apart two rectangles' centres are.
pub(crate) fn distance(a: &LayoutRect, b: &LayoutRect) -> f64 {
    (a.mid_x() - b.mid_x()).hypot(a.mid_y() - b.mid_y())
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LayoutGeometry {
    pub(crate) frame: LayoutRect,
    pub(crate) rects: HashMap<ControlId, LayoutRect>,
}

impl LayoutGeometry {
    /// The cross-platform app's own geometry for its layouts; an arrangement's is drawn from its counts.
    pub(crate) fn of(layout: &KeyboardLayout) -> Option<Self> {
        if let Some(shape) = GridShape::from_id(&layout.id) {
            return Some(Self::arrangement(&shape));
        }
        app_geometry::named(&layout.id).map(|geometry| geometry.with_even_turns())
    }

    /// The turns redrawn: all one size, none touching its knob. The original app sizes a knob's turns by the
    /// knob (0.6 of it), so a big knob's turns grow with it, and puts some over the edge of their knob. Here
    /// every turn is as large as the layout's smallest — smaller still where one would come closer than
    /// `TURN_GAP` to its knob — and stands in its original direction from the knob's centre, `TURN_GAP` off the
    /// knob.
    pub(crate) fn with_even_turns(&self) -> Self {
        let turns: Vec<(ControlId, LayoutRect, LayoutRect)> = self
            .rects
            .iter()
            .filter_map(|(control, rect)| match control {
                ControlId::Knob(number, part) if *part != KnobPart::Press => {
                    self.rects.get(&ControlId::Knob(*number, KnobPart::Press)).map(|press| (*control, *rect, *press))
                }
                _ => None,
            })
            .collect();
        let size = turns
            .iter()
            .map(|(_, rect, press)| rect.width.min(2.0 * (distance(rect, press) - press.width / 2.0 - TURN_GAP)))
            .reduce(f64::min);
        let Some(size) = size else { return self.clone() };
        let mut rects = self.rects.clone();
        for (control, rect, press) in &turns {
            let scale = (press.width / 2.0 + TURN_GAP + size / 2.0) / distance(rect, press);
            let x = press.mid_x() + (rect.mid_x() - press.mid_x()) * scale;
            let y = press.mid_y() + (rect.mid_y() - press.mid_y()) * scale;
            rects.insert(*control, LayoutRect::new(x - size / 2.0, y - size / 2.0, size, size));
        }
        Self { frame: self.frame, rects }
    }

    /// The body and every control.
    pub(crate) fn bounds(&self) -> LayoutRect {
        let (mut min_x, mut min_y, mut max_x, mut max_y) =
            (self.frame.x, self.frame.y, self.frame.max_x(), self.frame.max_y());
        for rect in self.rects.values() {
            min_x = min_x.min(rect.x);
            min_y = min_y.min(rect.y);
            max_x = max_x.max(rect.max_x());
            max_y = max_y.max(rect.max_y());
        }
        LayoutRect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// As it looks with the keyboard turned: every rectangle turned about the middle of the bounds. Squares
    /// stay squares; a wide key stands up.
    pub(crate) fn rotated(&self, rotation: Rotation) -> Self {
        if rotation == Rotation::Upright {
            return self.clone();
        }
        let bounds = self.bounds();
        let (center_x, center_y) = (bounds.mid_x(), bounds.mid_y());
        let turn = |rect: &LayoutRect| {
            let (dx, dy) = (rect.mid_x() - center_x, rect.mid_y() - center_y);
            let (x, y) = match rotation {
                Rotation::Upright => (dx, dy),
                Rotation::Quarter => (-dy, dx),
                Rotation::Half => (-dx, -dy),
                Rotation::ThreeQuarters => (dy, -dx),
            };
            let sideways = matches!(rotation, Rotation::Quarter | Rotation::ThreeQuarters);
            let (width, height) = if sideways { (rect.height, rect.width) } else { (rect.width, rect.height) };
            LayoutRect::new(center_x + x - width / 2.0, center_y + y - height / 2.0, width, height)
        };
        Self {
            frame: turn(&self.frame),
            rects: self.rects.iter().map(|(control, rect)| (*control, turn(rect))).collect(),
        }
    }

    /// Each knob as one body, for drawing the keyboard's shape — its actions are separate controls only for
    /// binding. The body sits on the press action, as large as it; a knob without one sits between its turns,
    /// as large as the layout's other knobs.
    pub(crate) fn knob_outlines(&self) -> Vec<(u8, LayoutRect)> {
        let mut parts: BTreeMap<u8, Vec<(KnobPart, LayoutRect)>> = BTreeMap::new();
        for (control, rect) in &self.rects {
            if let ControlId::Knob(number, part) = control {
                parts.entry(*number).or_default().push((*part, *rect));
            }
        }
        let press_size = parts
            .values()
            .flatten()
            .filter(|(part, _)| *part == KnobPart::Press)
            .map(|(_, rect)| rect.width)
            .fold(None, |largest: Option<f64>, width| Some(largest.map_or(width, |largest| largest.max(width))));
        parts
            .into_iter()
            .map(|(number, rects)| {
                if let Some((_, press)) = rects.iter().find(|(part, _)| *part == KnobPart::Press) {
                    return (number, *press);
                }
                let min_x = rects.iter().map(|(_, rect)| rect.x).fold(f64::INFINITY, f64::min);
                let max_x = rects.iter().map(|(_, rect)| rect.max_x()).fold(f64::NEG_INFINITY, f64::max);
                let min_y = rects.iter().map(|(_, rect)| rect.y).fold(f64::INFINITY, f64::min);
                let max_y = rects.iter().map(|(_, rect)| rect.max_y()).fold(f64::NEG_INFINITY, f64::max);
                let size = press_size.unwrap_or((max_x - min_x).min(max_y - min_y));
                (number, LayoutRect::new((min_x + max_x - size) / 2.0, (min_y + max_y - size) / 2.0, size, size))
            })
            .collect()
    }

    /// An arrangement drawn tidily, since the keyboard's real shape is not known: keys fill the rows from the
    /// top left; knobs stand to the right of the keys — one at their middle, two at the first and last rows (or
    /// around the middle when the rows are too close) — each with its turns either side of its press.
    pub(crate) fn arrangement(shape: &GridShape) -> Self {
        // In the same space as the app's layouts: a key 100 wide.
        const KEY: f64 = 100.0;
        const KEY_GAP: f64 = 20.0;
        const PRESS: f64 = 80.0;
        const TURN: f64 = 44.0;
        const TURN_OFFSET: f64 = 72.0;
        const KNOB_GAP: f64 = 30.0;
        const KNOB_SPACING: f64 = 110.0;
        const MARGIN: f64 = 40.0;

        let pitch = KEY + KEY_GAP;
        let (keys, columns, knobs) = (shape.keys(), shape.columns(), shape.knobs());
        let mut rects = HashMap::new();
        for index in 0..keys {
            rects.insert(
                ControlId::Key(index + 1),
                LayoutRect::new(f64::from(index % columns) * pitch, f64::from(index / columns) * pitch, KEY, KEY),
            );
        }
        let block_width = if keys == 0 { 0.0 } else { f64::from(columns) * pitch - KEY_GAP };
        let block_height = if keys == 0 { PRESS } else { f64::from(shape.rows()) * pitch - KEY_GAP };
        let knob_width = 2.0 * TURN_OFFSET + TURN;
        for index in 0..knobs {
            let index_f = f64::from(index);
            let center = if keys == 0 {
                (knob_width / 2.0 + index_f * (knob_width + KNOB_GAP), block_height / 2.0)
            } else {
                let (first, last) = (KEY / 2.0, block_height - KEY / 2.0);
                let y = if knobs == 1 {
                    block_height / 2.0
                } else if last - first >= KNOB_SPACING {
                    first + (last - first) * index_f / f64::from(knobs - 1)
                } else {
                    block_height / 2.0 + (index_f - f64::from(knobs - 1) / 2.0) * KNOB_SPACING
                };
                (block_width + KNOB_GAP + knob_width / 2.0, y)
            };
            for (part, offset, size) in [
                (KnobPart::RotateLeft, -TURN_OFFSET, TURN),
                (KnobPart::Press, 0.0, PRESS),
                (KnobPart::RotateRight, TURN_OFFSET, TURN),
            ] {
                rects.insert(
                    ControlId::Knob(index + 1, part),
                    LayoutRect::new(center.0 + offset - size / 2.0, center.1 - size / 2.0, size, size),
                );
            }
        }
        let min_x = rects.values().map(|rect| rect.x).fold(f64::INFINITY, f64::min);
        let min_y = rects.values().map(|rect| rect.y).fold(f64::INFINITY, f64::min);
        let max_x = rects.values().map(LayoutRect::max_x).fold(f64::NEG_INFINITY, f64::max);
        let max_y = rects.values().map(LayoutRect::max_y).fold(f64::NEG_INFINITY, f64::max);
        let frame =
            LayoutRect::new(min_x - MARGIN, min_y - MARGIN, max_x - min_x + 2.0 * MARGIN, max_y - min_y + 2.0 * MARGIN);
        Self { frame, rects }
    }
}
