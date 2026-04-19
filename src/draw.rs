mod attrs;
mod axis;
mod circle;
mod grid;
mod line;
mod properties;
mod text;

use crate::scale::{ScaleContinuous, Tick, Transformer};

pub use self::attrs::{Alignment, CircleAttrs, LineAttrs, TextAttrs};
pub use self::axis::{AxisOptions, axis_bottom, axis_left, axis_right, axis_top};
pub use self::circle::circle;
pub use self::grid::{grid_horizontal, grid_vertical};
pub use self::line::Line;
pub use self::properties::{CircleProperties, LineProperties, TextProperties};
pub use self::text::text;

enum Orientation {
    Flip,
    Same,
}

impl Orientation {
    fn apply(&self, x: f32, y: f32) -> [f32; 2] {
        match self {
            Orientation::Same => [x, y],
            Orientation::Flip => [y, x],
        }
    }
}

pub trait Draw {
    fn draw_line(&mut self, line: LineProperties);
    fn draw_text(&mut self, text: TextProperties);
    fn draw_circle(&mut self, circle: CircleProperties);

    fn axis_top<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        y: f32,
        line_attrs: impl Fn(f32) -> LineAttrs,
        text_attrs: impl Fn(f32) -> TextAttrs,
        axis_options: &AxisOptions,
    ) {
        axis_top(self, scaler, y, line_attrs, text_attrs, axis_options);
    }

    fn axis_right<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        x: f32,
        line_attrs: impl Fn(f32) -> LineAttrs,
        text_attrs: impl Fn(f32) -> TextAttrs,
        axis_options: &AxisOptions,
    ) {
        axis_right(self, scaler, x, line_attrs, text_attrs, axis_options);
    }

    fn axis_bottom<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        y: f32,
        line_attrs: impl Fn(f32) -> LineAttrs,
        text_attrs: impl Fn(f32) -> TextAttrs,
        axis_options: &AxisOptions,
    ) {
        axis_bottom(self, scaler, y, line_attrs, text_attrs, axis_options);
    }

    fn axis_left<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        x: f32,
        line_attrs: impl Fn(f32) -> LineAttrs,
        text_attrs: impl Fn(f32) -> TextAttrs,
        axis_options: &AxisOptions,
    ) {
        axis_left(self, scaler, x, line_attrs, text_attrs, axis_options);
    }

    fn circle<Data>(
        &mut self,
        values: &[Data],
        x: impl Fn(&Data) -> f32,
        y: impl Fn(&Data) -> f32,
        r: impl Fn(&Data) -> f32,
        circle_attrs: impl Fn(&Data) -> CircleAttrs,
    ) {
        circle(self, values, x, y, r, circle_attrs)
    }

    fn grid_vertical<Data>(
        &mut self,
        values: &[Data],
        y1: f32,
        y2: f32,
        x: impl Fn(&Data) -> f32,
        line_attrs: impl Fn(&Data) -> LineAttrs,
    ) {
        grid_vertical(self, values, y1, y2, x, line_attrs);
    }

    fn grid_horizontal<Data>(
        &mut self,
        values: &[Data],
        x1: f32,
        x2: f32,
        y: impl Fn(&Data) -> f32,
        line_attrs: impl Fn(&Data) -> LineAttrs,
    ) {
        grid_horizontal(self, values, x1, x2, y, line_attrs);
    }

    fn text<Data>(
        &mut self,
        values: &[Data],
        x: impl Fn(&Data) -> f32,
        y: impl Fn(&Data) -> f32,
        text_attrs: impl Fn(&Data) -> TextAttrs,
    ) {
        text(self, values, x, y, text_attrs)
    }

    fn text_vertical<Data>(
        &mut self,
        values: &[Data],
        x: f32,
        y: impl Fn(&Data) -> f32,
        text_attrs: impl Fn(&Data) -> TextAttrs,
    ) {
        text(self, values, |_| x, y, text_attrs);
    }

    fn text_horizontal<Data>(
        &mut self,
        values: &[Data],
        x: impl Fn(&Data) -> f32,
        y: f32,
        text_attrs: impl Fn(&Data) -> TextAttrs,
    ) {
        text(self, values, x, |_| y, text_attrs);
    }
}
