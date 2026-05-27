//! vizkit is a rendering-agnostic kit for data visualization.
//!
//! It aims to provide basic functionalities for making easier data visualization in GUI such as
//! [iced](https://iced.rs/) or [egui](https://www.egui.rs/) or more specific use cases such as
//! creating your own SVG.
//!
//! # Features
//!
//! Optional features:
//!
//! - `time`: Enable time operations and scales with a temporal domain using
//! [chrono](https://docs.rs/chrono/latest/chrono/).
//!
//! # Overview
//!
//! Most of the time, you want to draw basic elements (circles, rectangles, lines, ...) in a
//! well-defined region with a `width` and `height` (and sometimes margins such as `margin_top`,
//! `margin_right`, `margin_bottom`, `margin_left`).
//!
//! For that, let's imagine the following values:
//! ```
//! let width = 960.;
//! let height = 400.;
//!
//! let margin_top = 10.;
//! let margin_left = 50.;
//! let margin_right = 10.;
//! let margin_bottom = 40.;
//! ```
//!
//! In order to visualize your data, we assume you have processed data ready to be used as a slice
//! `&[T]`. Let's say `T` is the following structure:
//!
//! ```
//! struct Row {
//!     location: String,   // discrete values
//!     hour: u8,           // discrete values {0, 1, 2, ..., 21, 22, 23}
//!     vehicles: u32,      // continuous values between [0, 10_000]
//! }
//! ```
//!
//! For this specific case, we want to draw a _heatmap_ with:
//!
//! - an x-axis where `hour` values represent the ticks.
//! - an y-axis where `location` values represent the ticks.
//! - rectangles positioned at `[hour, location]` coordinates and filled with a color based  on the
//! row's associated `vehicles` value.
//!
//! We are going to use different scalers (see [`scale`][`crate::scale`] for more information):
//!
//! - a first [`ScaleBand`][`crate::scale::ScaleBand`] for mapping `hour` values to a range defined
//! by the region's width.
//! - a second [`ScaleBand`][`crate::scale::ScaleBand`] for mapping `location` values to a range
//! defined by the region's height.
//! - a [`ScaleColor`][`crate::scale::ScaleColor`] for mapping `vehicles` values to a range of
//! colors.
//!
//! ```
//! use std::collections::HashSet;
//! use vizkit::{
//!     chromatic::{ColorMap, Turbo},
//!     draw::{AxisOptions, ShapeAttrs, axis_bottom_iter, axis_left_iter, rect_iter},
//!     scale::{Axis, ScaleBand, ScaleColor},
//! };
//!
//! let width = 960.;
//! let height = 400.;
//!
//! let margin_top = 10.;
//! let margin_left = 50.;
//! let margin_right = 10.;
//! let margin_bottom = 40.;
//!
//! struct Row {
//!     location: String,
//!     hour: u8,
//!     vehicles: u32,
//! }
//!
//! let data = vec![
//!     Row {
//!         location: "Hasborn".to_string(),
//!         hour: 19,
//!         vehicles: 929,
//!     },
//!     Row {
//!         location: "Köln-Nord".to_string(),
//!         hour: 7,
//!         vehicles: 6882,
//!     },
//!     // ...
//! ];
//!
//! let hours: Vec<u8> = (0..24).collect();
//! let x_scale = ScaleBand::default()
//!     .domain(&hours)
//!     .range([margin_left, width - margin_right]);
//!
//! let locations: HashSet<&str> = HashSet::from_iter(
//!     data.iter().map(|row| row.location.as_str())
//! );
//! let y_scale = ScaleBand::default()
//!     .domain(&Vec::from_iter(locations))
//!     .range([height - margin_bottom, margin_top]);
//!
//! let rect_color = ScaleColor::linear(Turbo::default()).domain([0.0, 10_000.0]);
//!
//! let axis_options = AxisOptions::default();
//!
//! let x_axis = axis_bottom_iter(
//!     &x_scale,
//!     height - margin_bottom,
//!     |tick| tick.to_string(),
//!     &axis_options,
//! );
//!
//! let y_axis = axis_left_iter(
//!     &y_scale,
//!     margin_left,
//!     |tick| tick.to_string(),
//!     &axis_options,
//! );
//!
//! let rects = rect_iter(
//!     &data,
//!     |d| [
//!         x_scale.scale(d.hour).unwrap_or_default(),
//!         y_scale.scale(&d.location).unwrap_or_default()
//!     ],
//!     |_| [20., 20.],
//!     None, // corner radius
//!     |d| ShapeAttrs {
//!         fill_color: Some(rect_color.scale(d.vehicles as f32)),
//!         ..Default::default()
//!     }
//! );
//! ```
//!
//! ## Notes
//!
//! Every function in the [`draw`][`crate::draw`] module shares the same API: they return an
//! iterator of type `impl Iterator<Item = T>`. For basic shapes such as circles, rectangles, text,
//! and lines, `T` is a structure containing all properties for each element. For specific curves
//! like paths, areas, or arrows, `T` represents a sequence of
//! [`PathCommand`][`crate::draw::PathCommand`] used to draw the curve.

pub mod chromatic;
pub mod draw;
pub mod scale;
#[cfg(feature = "time")]
pub mod time;
