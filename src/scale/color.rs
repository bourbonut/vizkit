use super::{
    continuous::Transformer,
    linear::Linear,
    log::{Ln, Log, Log2, Log10},
    pow::{Power, Sqrt},
};
use crate::chromatic::{Color, ColorMap};

/// Scaler for color maps which maps a continuous domain to a continuous color range.
///
/// ```
/// use vizkit::chromatic::{ColorMap, Diverging, DivergingSpace};
/// use vizkit::scale::ScaleColor;
///
/// let color_map = Diverging::new(DivergingSpace::Spectral);
///
/// let width = 1000.;
/// let scale_color = ScaleColor::linear(color_map).domain([0., width]);
///
/// // You can convert to `String` or `[f32; 3]`
/// assert_eq!(scale_color.apply::<String>(0.), "#9e0042".to_string());
/// assert_eq!(scale_color.apply::<String>(width * 0.5), "#faf8af".to_string());
/// assert_eq!(scale_color.apply::<String>(width), "#5e4ea2".to_string());
/// ```
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

    /// Returns a new [`ScaleColor`] with the specified domain applied.
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

    /// Returns a new [`ScaleColor`] with the specified clamp applied which means that the
    /// transformed value will be clamped in [0., 1.]. See also [`ScaleColor::apply`].
    pub fn clamp(self, clamp: bool) -> Self {
        Self { clamp, ..self }
    }

    /// Given the specified value in the domain, it transforms the value, clamps it if enabled, and
    /// returns the corresponding color.
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
    /// Linear transformation
    pub fn linear(color_map: C) -> Self {
        Self::new(Linear, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Ln, C> {
    /// Natural logarithm transformation (base `e`)
    pub fn ln(color_map: C) -> Self {
        Self::new(Ln, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Log10, C> {
    /// Logarithm transformation (base `10`)
    pub fn log10(color_map: C) -> Self {
        Self::new(Log10, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Log2, C> {
    /// Logarithm transformation (base `2`)
    pub fn log2(color_map: C) -> Self {
        Self::new(Log2, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Log, C> {
    /// Logarithm transformation
    pub fn log(color_map: C, base: f32) -> Self {
        Self::new(Log { base }, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Power, C> {
    /// Power transformation (`x.powf(exponent)` where `x` is the input value used in
    /// [`ScaleColor::apply`])
    pub fn pow(color_map: C, exponent: f32) -> Self {
        Self::new(Power { exponent }, color_map)
    }
}

impl<C: ColorMap> ScaleColor<Sqrt, C> {
    /// Square root transformation
    pub fn sqrt(color_map: C) -> Self {
        Self::new(Sqrt, color_map)
    }
}
