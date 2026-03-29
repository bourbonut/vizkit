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

pub use crate::{
    categorical::CategoricalSpace,
    color::Color,
    common::{ColorMap, Scheme},
    diverging::DivergingSpace,
    sequential::SequentialSpace,
    viridis::ViridisSpace,
};
use crate::{
    cividis::cividis, common::RGBInterpolator, rainbow::rainbow, sinebow::sinebow, turbo::turbo,
    viridis::ViridisInterpolator, warm_cold::CubehelixInterpolator,
};

pub enum WarmColdSpace {
    Warm,
    Cold,
}

pub struct WarmCold {
    interpolator: CubehelixInterpolator,
}

impl WarmCold {
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

pub struct Viridis<'a> {
    interpolator: ViridisInterpolator<'a>,
}

impl<'a> Viridis<'a> {
    pub fn new(space: &'a ViridisSpace) -> Self {
        Self {
            interpolator: ViridisInterpolator::new(space.scheme()),
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

macro_rules! empty_color_map {
    ($name:ident, $function:path) => {
        #[derive(Default)]
        pub struct $name;

        impl ColorMap for $name {
            fn interpolate<T>(&self, t: f32) -> T
            where
                Color: Into<T>,
            {
                $function(t)
            }
        }
    };
}

empty_color_map!(Rainbow, rainbow);
empty_color_map!(Cividis, cividis);
empty_color_map!(Sinebow, sinebow);
empty_color_map!(Turbo, turbo);
