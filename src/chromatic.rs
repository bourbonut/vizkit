//! This module provides color maps.
//!
//! ```
//! use vizkit::chromatic::{ColorMap, Scheme, Diverging, DivergingSpace};
//! use vizkit::scale::ScaleColor;
//!
//! let color_map = Diverging::new(DivergingSpace::Spectral);
//!
//! // You can convert to `String` or `[f32; 3]`
//! assert_eq!(&color_map.interpolate::<String>(0.), "#9e0042");
//! assert_eq!(color_map.interpolate::<[f32; 3]>(0.5), [0.98300654, 0.97320265, 0.68954253]);
//! assert_eq!(&color_map.interpolate::<String>(1.), "#5e4ea2");
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
    color::{Color, ParseError},
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
    ///
    /// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#6d3fa9" height="30" width="30" x="0"/>
    /// <rect fill="#963db2" height="30" width="30" x="30"/>
    /// <rect fill="#bf3cae" height="30" width="30" x="60"/>
    /// <rect fill="#e3409d" height="30" width="30" x="90"/>
    /// <rect fill="#fe4b82" height="30" width="30" x="120"/>
    /// <rect fill="#ff5d63" height="30" width="30" x="150"/>
    /// <rect fill="#ff7746" height="30" width="30" x="180"/>
    /// <rect fill="#fa9633" height="30" width="30" x="210"/>
    /// <rect fill="#e2b72e" height="30" width="30" x="240"/>
    /// <rect fill="#c6d63b" height="30" width="30" x="270"/>
    /// <rect fill="#afef5a" height="30" width="30" x="300"/>
    /// </svg>
    Warm,

    /// 0° rotation
    ///
    /// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#6d3fa9" height="30" width="30" x="0"/>
    /// <rect fill="#6054c7" height="30" width="30" x="30"/>
    /// <rect fill="#4c6edb" height="30" width="30" x="60"/>
    /// <rect fill="#368ce1" height="30" width="30" x="90"/>
    /// <rect fill="#23aad8" height="30" width="30" x="120"/>
    /// <rect fill="#19c7c1" height="30" width="30" x="150"/>
    /// <rect fill="#1ddea3" height="30" width="30" x="180"/>
    /// <rect fill="#30ee82" height="30" width="30" x="210"/>
    /// <rect fill="#52f566" height="30" width="30" x="240"/>
    /// <rect fill="#7ef558" height="30" width="30" x="270"/>
    /// <rect fill="#afef5a" height="30" width="30" x="300"/>
    /// </svg>
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
///
/// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
/// <rect fill="#6d3fa9" height="30" width="30" x="0"/>
/// <rect fill="#bf3cae" height="30" width="30" x="30"/>
/// <rect fill="#fe4b82" height="30" width="30" x="60"/>
/// <rect fill="#ff7746" height="30" width="30" x="90"/>
/// <rect fill="#e2b72e" height="30" width="30" x="120"/>
/// <rect fill="#afef5a" height="30" width="30" x="150"/>
/// <rect fill="#52f566" height="30" width="30" x="180"/>
/// <rect fill="#1ddea3" height="30" width="30" x="210"/>
/// <rect fill="#23aad8" height="30" width="30" x="240"/>
/// <rect fill="#4c6edb" height="30" width="30" x="270"/>
/// <rect fill="#6d3fa9" height="30" width="30" x="300"/>
/// </svg>
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
///
/// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
/// <rect fill="#002020" height="30" width="30" x="0"/>
/// <rect fill="#0a3232" height="30" width="30" x="30"/>
/// <rect fill="#2b4444" height="30" width="30" x="60"/>
/// <rect fill="#4d5656" height="30" width="30" x="90"/>
/// <rect fill="#696969" height="30" width="30" x="120"/>
/// <rect fill="#7f7c7c" height="30" width="30" x="150"/>
/// <rect fill="#948f8f" height="30" width="30" x="180"/>
/// <rect fill="#ada4a4" height="30" width="30" x="210"/>
/// <rect fill="#cababa" height="30" width="30" x="240"/>
/// <rect fill="#ead1d1" height="30" width="30" x="270"/>
/// <rect fill="#fdeaea" height="30" width="30" x="300"/>
/// </svg>
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
///
/// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
/// <rect fill="#ff3f3f" height="30" width="30" x="0"/>
/// <rect fill="#e68c0b" height="30" width="30" x="30"/>
/// <rect fill="#a6d402" height="30" width="30" x="60"/>
/// <rect fill="#58fc2a" height="30" width="30" x="90"/>
/// <rect fill="#18f372" height="30" width="30" x="120"/>
/// <rect fill="#00bfbf" height="30" width="30" x="150"/>
/// <rect fill="#1872f3" height="30" width="30" x="180"/>
/// <rect fill="#582afc" height="30" width="30" x="210"/>
/// <rect fill="#a602d4" height="30" width="30" x="240"/>
/// <rect fill="#e60b8c" height="30" width="30" x="270"/>
/// <rect fill="#ff3f3f" height="30" width="30" x="300"/>
/// </svg>
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
///
/// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
/// <rect fill="#23171b" height="30" width="30" x="0"/>
/// <rect fill="#4a58dd" height="30" width="30" x="30"/>
/// <rect fill="#2f9df5" height="30" width="30" x="60"/>
/// <rect fill="#27d7c4" height="30" width="30" x="90"/>
/// <rect fill="#4df884" height="30" width="30" x="120"/>
/// <rect fill="#95fb51" height="30" width="30" x="150"/>
/// <rect fill="#dedd32" height="30" width="30" x="180"/>
/// <rect fill="#ffa423" height="30" width="30" x="210"/>
/// <rect fill="#f65f18" height="30" width="30" x="240"/>
/// <rect fill="#ba2208" height="30" width="30" x="270"/>
/// <rect fill="#900c00" height="30" width="30" x="300"/>
/// </svg>
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
