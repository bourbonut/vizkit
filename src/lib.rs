//! vizkit is an agnostic kit for data visualization.
//!
//! It aims to provide basic functionalities for making easier data visualization in GUI such as
//! [iced](https://iced.rs/) or [egui](https://www.egui.rs/) or more specific use cases such as
//! creating your own SVG.
//!
//! The main structures to use are:
//! - [`ScaleContinuous`][`crate::scale::ScaleContinuous`] for continuous domain with continuous
//! values. It supports:
//!     - [linear][`crate::scale::ScaleContinuous::linear`] transformation
//!     - [natural logarithm][`crate::scale::ScaleContinuous::ln`] transformation
//!     - [logarithm in base 2][`crate::scale::ScaleContinuous::log2`] transformation
//!     - [logarithm in base 10][`crate::scale::ScaleContinuous::log10`] transformation
//!     - [logarithm with specified base][`crate::scale::ScaleContinuous::log`] transformation
//!     - [power with specified exponent][`crate::scale::ScaleContinuous::pow`] transformation
//!     - [square root][`crate::scale::ScaleContinuous::sqrt`] transformation
//! - [`ScaleOrdinal`][`crate::scale::ScaleOrdinal`] for discrete domain with discrete range
//! - [`ScaleBand`][`crate::scale::ScaleOrdinal`] for discrete domain with continuous range
//! - [`ScaleColor`][`crate::scale::ScaleColor`] for color maps which maps a continuous domain to a
//! continuous color range. It supports:
//!     - [linear][`crate::scale::ScaleColor::linear`] transformation
//!     - [natural logarithm][`crate::scale::ScaleColor::ln`] transformation
//!     - [logarithm in base 2][`crate::scale::ScaleColor::log2`] transformation
//!     - [logarithm in base 10][`crate::scale::ScaleColor::log10`] transformation
//!     - [logarithm with specified base][`crate::scale::ScaleColor::log`] transformation
//!     - [power with specified exponent][`crate::scale::ScaleColor::pow`] transformation
//!     - [square root][`crate::scale::ScaleColor::sqrt`] transformation
//! - [`TimeInterval`][`crate::time::TimeInterval`] for manipulating [`chrono::DateTime`] with operations:
//!     - [floor][`crate::time::TimeInterval::interval`]
//!     - [ceil][`crate::time::TimeInterval::ceil`]
//!     - [round][`crate::time::TimeInterval::round`]
//!     - [range][`crate::time::TimeInterval::range`]
//!     - [every][`crate::time::TimeInterval::every`]
//! - [`ColorMap`][`crate::chromatic::ColorMap`]
//!     - [`WarmColdSpace`][`crate::chromatic::WarmColdSpace`] with 2 variants
//!     - [`ViridisSpace`][`crate::chromatic::ViridisSpace`] with 4 variants
//!     - [`DivergingSpace`][`crate::chromatic::DivergingSpace`] with 9 variants
//!     - [`SequentialSpace`][`crate::chromatic::SequentialSpace`] with 18 variants
//!     - [`Rainbow`][`crate::chromatic::Rainbow`]
//!     - [`Cividis`][`crate::chromatic::Cividis`]
//!     - [`Turbo`][`crate::chromatic::Turbo`]
//!     - [`Sinebow`][`crate::chromatic::Sinebow`]

pub mod axis;
pub mod chromatic;
pub mod draw;
pub mod scale;
pub mod time;
