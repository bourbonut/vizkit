//! This module provides color maps.
//!
//! ```
//! use vizkit::chromatic::{ColorMap, Scheme, Diverging, DivergingSpace};
//! use vizkit::scale::ScaleColor;
//!
//! let color_map = Diverging::new(DivergingSpace::Spectral);
//!
//! // You can convert to `String` or `[f32; 3]`
//! assert_eq!(color_map.interpolate::<String>(0.), "#9e0042".to_string());
//! assert_eq!(color_map.interpolate::<String>(0.5), "#faf8af".to_string());
//! assert_eq!(color_map.interpolate::<String>(1.), "#5e4ea2".to_string());
//!
//! // For color space with `Scheme` trait
//! assert_eq!(DivergingSpace::Spectral.scheme().len(), 11); // 11 interpolated colors
//! ```

mod categorical;
mod cividis;
mod color;
mod common;
mod diverging;
mod rainbow;
mod sequential;
mod sinebow;
mod turbo;
mod viridis;
mod warm_cold;

pub use self::{
    categorical::CategoricalSpace,
    color::Color,
    common::{ColorMap, Scheme},
    diverging::DivergingSpace,
    sequential::SequentialSpace,
    viridis::ViridisSpace,
};
use self::{
    cividis::cividis, common::RGBInterpolator, rainbow::rainbow, sinebow::sinebow, turbo::turbo,
    viridis::ViridisInterpolator, warm_cold::CubehelixInterpolator,
};

/// Color space used for [`WarmCold`] color map
#[derive(Debug, Clone)]
pub enum WarmColdSpace {
    /// 180° rotation
    Warm,
    /// 0° rotation
    Cold,
}

/// Colors from a Niccoli's perceptual rainbow
#[derive(Clone)]
pub struct WarmCold {
    interpolator: CubehelixInterpolator,
}

impl WarmCold {
    /// Constructs a new color map from a Niccoli's perceptual rainbow
    pub fn new(space: WarmColdSpace) -> Self {
        match space {
            WarmColdSpace::Warm => Self {
                interpolator: CubehelixInterpolator::warm(),
            },
            WarmColdSpace::Cold => Self {
                interpolator: CubehelixInterpolator::cold(),
            },
        }
    }

    /// Sets gamma used as exponent on lightness channel values
    pub fn gamma(self, gamma: f32) -> Self {
        Self {
            interpolator: self.interpolator.gamma(gamma),
        }
    }
}

impl ColorMap for WarmCold {
    fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        self.interpolator.interpolate(t)
    }
}

/// Color map from the "viridis" perceptually-uniform color scheme designed by Van der Walt, Smith
/// and Firing
#[derive(Clone)]
pub struct Viridis<'a> {
    interpolator: ViridisInterpolator<'a>,
}

impl<'a> Viridis<'a> {
    pub fn new(space: &'a ViridisSpace) -> Self {
        Self {
            interpolator: space.interpolator(),
        }
    }
}

impl<'a> ColorMap for Viridis<'a> {
    fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        self.interpolator.interpolate(t)
    }
}

/// Colors using sequential scales used for a color encoding
#[derive(Clone)]
pub struct Sequential {
    interpolator: RGBInterpolator,
}

impl Sequential {
    pub fn new(space: SequentialSpace) -> Self {
        Self {
            interpolator: space.interpolator(),
        }
    }
}

impl ColorMap for Sequential {
    fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        self.interpolator.interpolate(t)
    }
}

/// Colors using diverging scales used for a color encoding
#[derive(Clone)]
pub struct Diverging {
    interpolator: RGBInterpolator,
}

impl Diverging {
    pub fn new(space: DivergingSpace) -> Self {
        Self {
            interpolator: space.interpolator(),
        }
    }
}

impl ColorMap for Diverging {
    fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        self.interpolator.interpolate(t)
    }
}

/// Color map from [`WarmColdSpace::Warm`] in range [0.0, 0.5] followed by the
/// [`WarmColdSpace::Cold`] in range [0.5, 1.0], thus implementing the cyclical less-angry rainbow
/// color scheme.
#[derive(Default, Clone)]
pub struct Rainbow;

impl ColorMap for Rainbow {
    fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        rainbow(t)
    }
}

/// Color map from the "cividis" color vision deficiency-optimized color scheme designed by Nuñez,
/// Anderton, and Renslow.
#[derive(Default, Clone)]
pub struct Cividis;

impl ColorMap for Cividis {
    fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        cividis(t)
    }
}

/// Color map from the "sinebow" color scheme by Jim Bumgardner and Charlie Loyd.
#[derive(Default, Clone)]
pub struct Sinebow;

impl ColorMap for Sinebow {
    fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        sinebow(t)
    }
}

/// Color map from the "turbo" color scheme by Anton Mikhailov.
#[derive(Default, Clone)]
pub struct Turbo;

impl ColorMap for Turbo {
    fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        turbo(t)
    }
}
