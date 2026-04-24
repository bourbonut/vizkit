//! This module provides basic functionalities to draw fundamental elements such as axis, grid,
//! circles, lines ... It is based on row-oriented data structures and it offers a simple API to
//! change attributes given a specific row of data.
//!
//! ```
//! use vizkit::draw::{
//!     Draw,
//!     AxisOptions,
//!     CircleProperties,
//!     LineProperties,
//!     TextProperties,
//!     RectProperties
//! };
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
//!
//!     fn draw_rect(&mut self, _: RectProperties) {
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

mod arrow;
mod attrs;
mod axis;
mod circle;
mod grid;
mod properties;
mod rect;
mod text;

use crate::scale::{ScaleContinuous, Tick, Transformer};

pub use self::arrow::{arrow_iter, vector_iter};
pub use self::attrs::{Alignment, ArrowAttrs, LineAttrs, ShapeAttrs, TextAttrs};
pub use self::axis::{
    AxisOptions, axis_bottom_iter, axis_left_iter, axis_right_iter, axis_top_iter,
};
pub use self::circle::circle_iter;
pub use self::grid::{grid_horizontal_iter, grid_vertical_iter};
pub use self::properties::{
    ArrowProperties, CircleProperties, LineProperties, RectProperties, TextProperties,
};
pub use self::rect::rect_iter;
pub use self::text::text_iter;

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
    fn draw_circle(&mut self, circle: CircleProperties);
    fn draw_line(&mut self, line: LineProperties);
    fn draw_rect(&mut self, rect: RectProperties);
    fn draw_text(&mut self, text: TextProperties);

    fn axis_top<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        y: f32,
        formatter: impl Fn(f32) -> String,
        axis_options: &AxisOptions,
    ) {
        axis_top_iter(scaler, y, formatter, axis_options).for_each(|(line, text)| {
            self.draw_line(line);
            self.draw_text(text);
        });
    }

    fn axis_right<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        x: f32,
        formatter: impl Fn(f32) -> String,
        axis_options: &AxisOptions,
    ) {
        axis_right_iter(scaler, x, formatter, axis_options).for_each(|(line, text)| {
            self.draw_line(line);
            self.draw_text(text);
        });
    }

    fn axis_bottom<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        y: f32,
        formatter: impl Fn(f32) -> String,
        axis_options: &AxisOptions,
    ) {
        axis_bottom_iter(scaler, y, formatter, axis_options).for_each(|(line, text)| {
            self.draw_line(line);
            self.draw_text(text);
        });
    }

    fn axis_left<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        x: f32,
        formatter: impl Fn(f32) -> String,
        axis_options: &AxisOptions,
    ) {
        axis_left_iter(scaler, x, formatter, axis_options).for_each(|(line, text)| {
            self.draw_line(line);
            self.draw_text(text);
        });
    }

    fn circle<Data>(
        &mut self,
        values: &[Data],
        x: impl Fn(&Data) -> f32,
        y: impl Fn(&Data) -> f32,
        r: impl Fn(&Data) -> f32,
        shape_attrs: impl Fn(&Data) -> ShapeAttrs,
    ) {
        self.circle_from_props(circle_iter(values, x, y, r, shape_attrs));
    }

    fn circle_from_props<I: IntoIterator<Item = CircleProperties>>(&mut self, iter: I) {
        iter.into_iter().for_each(|circle| self.draw_circle(circle));
    }

    fn grid_vertical<Data>(
        &mut self,
        values: &[Data],
        y1: f32,
        y2: f32,
        x: impl Fn(&Data) -> f32,
        line_attrs: impl Fn(&Data) -> LineAttrs,
    ) {
        self.line_from_props(grid_vertical_iter(values, y1, y2, x, line_attrs));
    }

    fn grid_horizontal<Data>(
        &mut self,
        values: &[Data],
        x1: f32,
        x2: f32,
        y: impl Fn(&Data) -> f32,
        line_attrs: impl Fn(&Data) -> LineAttrs,
    ) {
        self.line_from_props(grid_horizontal_iter(values, x1, x2, y, line_attrs));
    }

    fn line_from_props<I: IntoIterator<Item = LineProperties>>(&mut self, iter: I) {
        iter.into_iter().for_each(|line| self.draw_line(line));
    }

    fn rect<Data>(
        &mut self,
        values: &[Data],
        x: impl Fn(&Data) -> f32,
        y: impl Fn(&Data) -> f32,
        width: impl Fn(&Data) -> f32,
        height: impl Fn(&Data) -> f32,
        corner_radius: Option<f32>,
        shape_attrs: impl Fn(&Data) -> ShapeAttrs,
    ) {
        self.rect_from_props(rect_iter(
            values,
            x,
            y,
            width,
            height,
            corner_radius,
            shape_attrs,
        ));
    }

    fn rect_from_props<I: IntoIterator<Item = RectProperties>>(&mut self, iter: I) {
        iter.into_iter().for_each(|rect| self.draw_rect(rect));
    }

    fn text<Data>(
        &mut self,
        values: &[Data],
        x: impl Fn(&Data) -> f32,
        y: impl Fn(&Data) -> f32,
        text_attrs: impl Fn(&Data) -> TextAttrs,
    ) {
        self.text_from_props(text_iter(values, x, y, text_attrs))
    }

    fn text_vertical<Data>(
        &mut self,
        values: &[Data],
        x: f32,
        y: impl Fn(&Data) -> f32,
        text_attrs: impl Fn(&Data) -> TextAttrs,
    ) {
        self.text_from_props(text_iter(values, |_| x, y, text_attrs))
    }

    fn text_horizontal<Data>(
        &mut self,
        values: &[Data],
        x: impl Fn(&Data) -> f32,
        y: f32,
        text_attrs: impl Fn(&Data) -> TextAttrs,
    ) {
        self.text_from_props(text_iter(values, x, |_| y, text_attrs))
    }

    fn text_from_props<I: IntoIterator<Item = TextProperties>>(&mut self, iter: I) {
        iter.into_iter().for_each(|text| self.draw_text(text));
    }
}
