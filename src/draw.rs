//! This module provides basic functionalities to draw fundamental elements such as axis, grid,
//! circles, lines ... It is based on row-oriented data structures and it offers a simple API to
//! change attributes given a specific row of data.
//!
//! ```
//! use vizkit::draw::{Draw, AxisOptions, CircleProperties, LineProperties, TextProperties};
//! use vizkit::scale::ScaleContinuous;
//!
//! #[derive(Default)]
//! struct Drawer {
//!     lines: Vec<LineProperties>,
//!     texts: Vec<TextProperties>,
//! }
//
//! impl Draw for Drawer {
//!     fn draw_line(&mut self, line: LineProperties) {
//!         self.lines.push(line);
//!     }
//
//!     fn draw_text(&mut self, text: TextProperties) {
//!         self.texts.push(text);
//!     }
//
//!     fn draw_circle(&mut self, _: CircleProperties) {
//!         todo!()
//!     }
//! }
//!
//! let margin_left = 50.;
//! let margin_right = 10.;
//! let margin_bottom = 40.;
//! let width = 500.;
//! let height = 200.;
//! let scale = ScaleContinuous::linear()
//!     .domain([0., 50.])
//!     .range([margin_left, width - margin_right]);
//!
//! let mut drawer = Drawer::default();
//!
//! drawer.axis_bottom(
//!     &scale,
//!     height - margin_bottom,
//!     |tick: f32| tick.to_string(),
//!     &AxisOptions::default()
//! );
//!
//! assert_eq!(drawer.lines.len(), scale.ticks(None).len());
//! assert_eq!(drawer.texts.len(), scale.ticks(None).len());
//! ```

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
        formatter: impl Fn(f32) -> String,
        axis_options: &AxisOptions,
    ) {
        axis_top(self, scaler, y, formatter, axis_options);
    }

    fn axis_right<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        x: f32,
        formatter: impl Fn(f32) -> String,
        axis_options: &AxisOptions,
    ) {
        axis_right(self, scaler, x, formatter, axis_options);
    }

    fn axis_bottom<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        y: f32,
        formatter: impl Fn(f32) -> String,
        axis_options: &AxisOptions,
    ) {
        axis_bottom(self, scaler, y, formatter, axis_options);
    }

    fn axis_left<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        x: f32,
        formatter: impl Fn(f32) -> String,
        axis_options: &AxisOptions,
    ) {
        axis_left(self, scaler, x, formatter, axis_options);
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
