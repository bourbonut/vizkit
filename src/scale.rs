//! This module provides scalers for continuous range of values, color maps, discrete values like
//! `&str` and time values using [`chrono::DateTime`].
//!
//! ```
//! use vizkit::scale::ScaleContinuous;
//!
//! // Dimensions for a chart for instance
//! let width = 1000.;
//! let margin_left = 30.;
//! let margin_right = 20.;
//!
//! // domain of x values
//! let x_min = 20.;
//! let x_max = 50.;
//!
//! let scale = ScaleContinuous::linear()
//!     .domain([x_min, x_max])
//!     .range([margin_left, width - margin_right]);
//!
//! // Start of domain
//! assert_eq!(scale.apply(x_min), margin_left);
//! // Middle of domain
//! assert_eq!(scale.apply((x_max + x_min) * 0.5), (width - margin_right + margin_left) * 0.5);
//! // End of domain
//! assert_eq!(scale.apply(x_max), width - margin_right);
//! ```

mod color;
mod continuous;
mod discrete;
mod linear;
mod log;
mod pow;
mod ticks;
#[cfg(feature = "time")]
mod time;

#[cfg(feature = "time")]
pub use self::time::ScaleTime;

pub use self::{
    color::ScaleColor,
    continuous::{ScaleContinuous, Transformer},
    discrete::{ScaleBand, ScaleOrdinal},
    linear::Linear,
    log::{Ln, Log, Log2, Log10},
    pow::{Power, Sqrt},
    ticks::Tick,
};

/// Axis trait used for generating tick values and tick positions along a direction for axis
/// iterators.
///
/// ## See also
///
/// This trait is used by the following functions:
///
/// - [`axis_top_iter`][`crate::draw::axis_top_iter`]
/// - [`axis_right_iter`][`crate::draw::axis_right_iter`]
/// - [`axis_bottom_iter`][`crate::draw::axis_bottom_iter`]
/// - [`axis_left_iter`][`crate::draw::axis_left_iter`]
pub trait Axis {
    /// Tick type
    type Tick;
    /// Generates tick values with an approximated number of ticks specified by the `count` value.
    ///
    /// If the specified value is `None`, it defaults to 10.
    fn ticks(&self, count: Option<usize>) -> Vec<Self::Tick>;
    /// Generates a tick position along a direction.
    fn tick_position(&self, x: Self::Tick) -> f32;
}
