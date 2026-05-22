//! This module provides basic functionalities to draw fundamental elements such as axis, grid,
//! circle, rectangle, text, path, area and arrow.
//!
//! It is based on row-oriented data structures and it offers a simple API to change attributes
//! given a specific row of data.
//!
//! ```
//! use vizkit::draw::{
//!     axis_bottom_iter,
//!     AxisOptions,
//!     LineProperties,
//!     TextProperties,
//! };
//! use vizkit::scale::{Axis, ScaleContinuous};
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
//! let (lines, texts): (Vec<LineProperties>, Vec<TextProperties>) = axis_bottom_iter(
//!     &scale,
//!     height - margin_bottom,
//!     |tick: &f32| tick.to_string(),
//!     &AxisOptions::default()
//! ).unzip();
//!
//! assert_eq!(lines.len(), scale.ticks(None).len());
//! assert_eq!(texts.len(), scale.ticks(None).len());
//! ```

mod arrow;
mod attrs;
mod axis;
mod circle;
mod grid;
mod path;
mod properties;
mod rect;
mod text;

pub use self::arrow::{arrow_iter, vector_iter};
pub use self::attrs::{Alignment, ArrowAttrs, LineAttrs, ShapeAttrs, TextAttrs};
pub use self::axis::{
    AxisOptions, axis_bottom_iter, axis_left_iter, axis_right_iter, axis_top_iter,
};
pub use self::circle::circle_iter;
pub use self::grid::{grid_horizontal_iter, grid_vertical_iter};
pub use self::path::{
    Curve, PathCommand, area_horizontal_iter, area_iter, area_vertical_iter, path_iter,
};
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
