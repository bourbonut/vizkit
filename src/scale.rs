//! This module provides scalers for continuous range of values, color maps and discrete values like
//! `&str`.
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

pub use self::{
    color::ScaleColor,
    continuous::{Clamper, ScaleContinuous, Transformer},
    discrete::{ScaleBand, ScaleOrdinal},
    linear::Linear,
    log::{Ln, Log, Log2, Log10},
    pow::{Power, Sqrt},
    ticks::Tick,
};
