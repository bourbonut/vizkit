use crate::{
    continuous::Transformer,
    linear::Linear,
    log::{Ln, Log, Log2, Log10},
    pow::{Power, Sqrt},
};
use vizkit_chromatic::{Color, ColorMap};

pub struct ScaleColor<T: Transformer, C: ColorMap> {
    transformer: T,
    color_map: C,
    t0: f32,
    factor: f32,
    clamp: bool,
}

impl<T: Transformer, C: ColorMap> ScaleColor<T, C> {
    fn new(transformer: T, color_map: C) -> Self {
        let t0 = transformer.transform(0.);
        let t1 = transformer.transform(1.);
        Self {
            transformer,
            color_map,
            t0,
            factor: if t0 == t1 { 0. } else { 1. / (t1 - t0) },
            clamp: false,
        }
    }

    pub fn domain(self, domain: [f32; 2]) -> Self {
        let [x0, x1] = domain;
        let t0 = self.transformer.transform(x0);
        let t1 = self.transformer.transform(x1);
        Self {
            t0,
            factor: if t0 == t1 { 0. } else { 1. / (t1 - t0) },
            ..self
        }
    }

    pub fn clamp(self, clamp: bool) -> Self {
        Self { clamp, ..self }
    }

    pub fn apply<O>(&self, x: f32) -> O
    where
        Color: Into<O>,
    {
        let t = if self.factor == 0. {
            0.5
        } else {
            let value = (self.transformer.transform(x) - self.t0) * self.factor;
            if self.clamp {
                value.clamp(0., 1.)
            } else {
                value
            }
        };
        self.color_map.interpolate(t)
    }
}

impl<C: ColorMap> ScaleColor<Linear, C> {
    pub fn linear(color_map: C) -> Self {
        Self::new(Linear, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Ln, C> {
    pub fn ln(color_map: C) -> Self {
        Self::new(Ln, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Log10, C> {
    pub fn log10(color_map: C) -> Self {
        Self::new(Log10, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Log2, C> {
    pub fn log2(color_map: C) -> Self {
        Self::new(Log2, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Log, C> {
    pub fn log(color_map: C, base: f32) -> Self {
        Self::new(Log { base }, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Power, C> {
    pub fn pow(color_map: C, exponent: f32) -> Self {
        Self::new(Power { exponent }, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Sqrt, C> {
    pub fn sqrt(color_map: C) -> Self {
        Self::new(Sqrt, color_map)
    }
}
