use std::f32::consts;

use super::{
    linear::Linear,
    log::{Ln, Log, Log2, Log10},
    pow::{Power, Sqrt},
    ticks::Tick,
};

enum Normalizer {
    Constant { value: f32 },
    Linear { a: f32, b: f32 },
}

impl Normalizer {
    fn new(a: f32, b: f32) -> Self {
        let b = b - a;

        if b.is_nan() {
            Self::Constant { value: f32::NAN }
        } else if b == 0.0 {
            Self::Constant { value: 0.5 }
        } else {
            Self::Linear { a, b }
        }
    }
    fn normalize(&self, x: f32) -> f32 {
        match self {
            Normalizer::Constant { value } => *value,
            Normalizer::Linear { a, b } => (x - a) / b,
        }
    }
}

struct Interpolate {
    a: f32,
    b: f32,
}

impl Interpolate {
    fn interpolate(&self, t: f32) -> f32 {
        self.a * (1. - t) + self.b * t
    }
}

struct BiMap {
    d0: Normalizer,
    r0: Interpolate,
}

impl BiMap {
    fn new(domain: &[f32; 2], range: &[f32; 2]) -> Self {
        let [d0, d1] = domain;
        let [r0, r1] = range;

        if d1 < d0 {
            Self {
                d0: Normalizer::new(*d1, *d0),
                r0: Interpolate { a: *r1, b: *r0 },
            }
        } else {
            Self {
                d0: Normalizer::new(*d0, *d1),
                r0: Interpolate { a: *r0, b: *r1 },
            }
        }
    }

    fn apply(&self, x: f32) -> f32 {
        self.r0.interpolate(self.d0.normalize(x))
    }
}

impl Default for BiMap {
    fn default() -> Self {
        Self {
            d0: Normalizer::new(0., 1.),
            r0: Interpolate { a: 0., b: 1. },
        }
    }
}

/// Trait used for bijection functions (bidirectional transformation)
pub trait Transformer {
    /// Transforms `x` to `y`
    fn transform(&self, x: f32) -> f32;
    /// Untransforms `y` to `x`
    fn untransform(&self, y: f32) -> f32;
}

/// Continuous scaler between two domain values and two range values
///
/// ```
/// use vizkit::scale::ScaleContinuous;
///
/// let scale = ScaleContinuous::linear().domain([20., 30.]).range([100., 400.]);
/// assert_eq!(scale.apply(25.), 250.);
/// assert_eq!(scale.invert(400.), 30.);
/// ```
pub struct ScaleContinuous<T: Transformer + Tick> {
    transformer: T,
    domain: [f32; 2],
    range: [f32; 2],
    output: BiMap,
    input: BiMap,
    clamp: bool,
}

impl<T: Transformer + Tick> ScaleContinuous<T> {
    /// Returns a new [`ScaleContinuous`] with the specified domain applied.
    pub fn domain(self, domain: [f32; 2]) -> Self {
        Self {
            domain,
            input: BiMap::new(&self.range, &domain.map(|x| self.transformer.transform(x))),
            output: BiMap::new(&domain.map(|x| self.transformer.transform(x)), &self.range),
            ..self
        }
    }

    /// Returns a new [`ScaleContinuous`] with the specified range applied.
    pub fn range(self, range: [f32; 2]) -> Self {
        Self {
            range,
            input: BiMap::new(&range, &self.domain.map(|x| self.transformer.transform(x))),
            output: BiMap::new(&self.domain.map(|x| self.transformer.transform(x)), &range),
            ..self
        }
    }

    /// Returns a new [`ScaleContinuous`] with the specified clamp value. If `true`, it clamps the
    /// value passed to the transform step (see [`ScaleContinuous::apply`]) and the returned value
    /// after untransform step (see [`ScaleContinuous::invert`]) with the domain values.
    pub fn clamp(self, clamp: bool) -> Self {
        Self { clamp, ..self }
    }

    /// Given the specified value in the domain, it clamps the value, transforms it and returns the
    /// corresponding value of the range.
    pub fn apply(&self, x: f32) -> f32 {
        self.output.apply(self.transformer.transform(if self.clamp {
            let [a, b] = self.domain;
            x.clamp(a, b)
        } else {
            x
        }))
    }

    /// Given the specified value in the range, it computes the corresponding value of the domain,
    /// untransforms it and returns the clamped value.
    pub fn invert(&self, y: f32) -> f32 {
        let x = self.transformer.untransform(self.input.apply(y));
        if self.clamp {
            let [a, b] = self.domain;
            x.clamp(a, b)
        } else {
            x
        }
    }

    /// Returns approximately `count` representative values from the domain where `count` varies
    /// more or fewer the number of values depending on the domain. Default: `10`.
    ///
    /// ```
    /// use vizkit::scale::ScaleContinuous;
    ///
    /// // Default for `count` is `10`
    /// let scale = ScaleContinuous::linear().domain([20., 100.]).range([0., 1.]);
    /// assert_eq!(scale.ticks(None), vec![20., 30., 40., 50., 60., 70., 80., 90., 100.]);
    /// assert_eq!(scale.ticks(Some(5)), vec![20., 40., 60., 80., 100.]);
    /// ```
    pub fn ticks(&self, count: Option<usize>) -> Vec<f32> {
        self.transformer.ticks(&self.domain, count.unwrap_or(10))
    }

    /// Extends the domain so that it starts and ends on nice round values where `count` allows
    /// greater control over the step size used to extend the bounds. Default: `10`.
    ///
    /// ```
    /// use vizkit::scale::ScaleContinuous;
    ///
    /// // Default for `count` is `10`
    /// let scale = ScaleContinuous::linear().domain([12.94728, 16.24782]).range([0., 1.]).nice(None);
    /// assert_eq!(scale.invert(0.), 12.5);
    /// assert_eq!(scale.invert(1.), 16.5);
    ///
    /// let scale = ScaleContinuous::linear().domain([12.94728, 16.24782]).range([0., 1.]).nice(Some(1));
    /// assert_eq!(scale.invert(0.), 10.0);
    /// assert_eq!(scale.invert(1.), 20.0);
    /// ```
    pub fn nice(self, count: Option<usize>) -> Self {
        let domain = self.transformer.nice(&self.domain, count.unwrap_or(10));
        self.domain(domain)
    }
}

impl ScaleContinuous<Linear> {
    /// Linear transformation
    pub fn linear() -> Self {
        Self {
            transformer: Linear,
            domain: [0.0, 1.0],
            range: [0.0, 1.0],
            input: BiMap::default(),
            output: BiMap::default(),
            clamp: false,
        }
    }
}

impl ScaleContinuous<Log10> {
    /// Logarithm transformation (base `10`)
    pub fn log10() -> Self {
        let domain = [1., 10.];
        let range = [0., 1.];
        Self {
            transformer: Log10,
            domain,
            range,
            input: BiMap::new(&range, &domain),
            output: BiMap::new(&domain, &range),
            clamp: false,
        }
    }
}

impl ScaleContinuous<Log2> {
    /// Logarithm transformation (base `2`)
    pub fn log2() -> Self {
        let domain = [1., 2.];
        let range = [0., 1.];
        Self {
            transformer: Log2,
            domain,
            range,
            input: BiMap::new(&range, &domain),
            output: BiMap::new(&domain, &range),
            clamp: false,
        }
    }
}

impl ScaleContinuous<Ln> {
    /// Natural logarithm transformation (base `e`)
    pub fn ln() -> Self {
        let domain = [1., consts::E];
        let range = [0., 1.];
        Self {
            transformer: Ln,
            domain,
            range,
            input: BiMap::new(&range, &domain),
            output: BiMap::new(&domain, &range),
            clamp: false,
        }
    }
}

impl ScaleContinuous<Log> {
    /// Logarithm transformation
    pub fn log(base: f32) -> Self {
        let domain = [1., base];
        let range = [0., 1.];
        Self {
            transformer: Log { base },
            domain,
            range,
            input: BiMap::new(&range, &domain),
            output: BiMap::new(&domain, &range),
            clamp: false,
        }
    }
}

impl ScaleContinuous<Power> {
    /// Power transformation (`x.powf(exponent)` where `x` is the input value used in
    /// [`ScaleContinuous::apply`])
    pub fn pow(exponent: f32) -> Self {
        Self {
            transformer: Power { exponent },
            domain: [0.0, 1.0],
            range: [0.0, 1.0],
            input: BiMap::default(),
            output: BiMap::default(),
            clamp: false,
        }
    }
}

impl ScaleContinuous<Sqrt> {
    /// Square root transformation
    pub fn sqrt() -> Self {
        Self {
            transformer: Sqrt,
            domain: [0.0, 1.0],
            range: [0.0, 1.0],
            input: BiMap::default(),
            output: BiMap::default(),
            clamp: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ScaleContinuous;
    use rstest::rstest;

    fn reverse(slice: &[f32]) -> Vec<f32> {
        slice.into_iter().rev().copied().collect()
    }

    fn round(vec: Vec<f32>) -> Vec<f32> {
        vec.iter().map(|x| (x * 10.).round() / 10.).collect()
    }

    fn round_epsilon(vec: Vec<f32>) -> Vec<f32> {
        vec.iter().map(|x| (x * 1e12).round() / 1e12).collect()
    }

    #[rstest]
    #[case([0., 0.96], None, [0., 1.])]
    #[case([0., 96.], None, [0., 100.])]
    #[case([0., 0.96], Some(10), [0., 1.])]
    #[case([0., 96.], Some(10), [0., 100.])]
    #[case([0.96, 0.], Some(10), [1., 0.])]
    #[case([96., 0.], Some(10), [100., 0.])]
    #[case([0., -0.96], Some(10), [0., -1.])]
    #[case([0., -96.], Some(10), [0., -100.])]
    #[case([-0.96, 0.], Some(10), [-1., 0.])]
    #[case([-96., 0.], Some(10), [-100., 0.])]
    #[case([-0.1, 51.1], Some(8), [-10., 60.])]
    #[case([1.1, 10.9], Some(10), [1., 11.])]
    #[case([10.9, 1.1], Some(10), [11., 1.])]
    #[case([0.7, 11.001], Some(10), [0., 12.])]
    #[case([123.1, 6.7], Some(10), [130., 0.])]
    #[case([0., 0.49], Some(10), [0., 0.5])]
    #[case([0., 14.1], Some(5), [0., 20.])]
    #[case([0., 15.], Some(5), [0., 20.])]
    #[case([1.1, 10.9], Some(10), [1., 11.])]
    #[case([123.1, -0.9], Some(10), [130., -10.])]
    #[case([12., 87.], Some(5), [0., 100.])]
    #[case([12., 87.], Some(10), [10., 90.])]
    #[case([12., 87.], Some(100), [12., 87.])]
    fn test_scale_linear_nice(
        #[case] domain: [f32; 2],
        #[case] count: Option<usize>,
        #[case] expected: [f32; 2],
    ) {
        let s = ScaleContinuous::linear().domain(domain).nice(count);
        assert_eq!(s.domain, expected);
    }

    #[rstest]
    #[case(Some(10), &[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0])]
    #[case(Some(9), &[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0])]
    #[case(Some(8), &[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0])]
    #[case(Some(7), &[0.0, 0.2, 0.4, 0.6, 0.8, 1.0])]
    #[case(Some(6), &[0.0, 0.2, 0.4, 0.6, 0.8, 1.0])]
    #[case(Some(5), &[0.0, 0.2, 0.4, 0.6, 0.8, 1.0])]
    #[case(Some(4), &[0.0, 0.2, 0.4, 0.6, 0.8, 1.0])]
    #[case(Some(3), &[0.0, 0.5, 1.0])]
    #[case(Some(2), &[0.0, 0.5, 1.0])]
    #[case(Some(1), &[0.0, 1.0])]
    fn test_scale_linear_ticks_defaut_domain(
        #[case] count: Option<usize>,
        #[case] expected: &[f32],
    ) {
        let s = ScaleContinuous::linear();
        assert_eq!(round_epsilon(s.ticks(count)), expected);

        let s = ScaleContinuous::linear().domain([1., 0.]);
        assert_eq!(round_epsilon(s.ticks(count)), reverse(expected));
    }

    #[rstest]
    #[case(Some(10), &[-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.])]
    #[case(Some(9), &[-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.])]
    #[case(Some(8), &[-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.])]
    #[case(Some(7), &[-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.])]
    #[case(Some(6), &[-100., -50., 0., 50., 100.])]
    #[case(Some(5), &[-100., -50., 0., 50., 100.])]
    #[case(Some(4), &[-100., -50., 0., 50., 100.])]
    #[case(Some(3), &[-100., -50., 0., 50., 100.])]
    #[case(Some(2), &[-100., 0., 100.])]
    #[case(Some(1), &[0.])]
    fn test_scale_linear_ticks_specific_domain(
        #[case] count: Option<usize>,
        #[case] expected: &[f32],
    ) {
        let s = ScaleContinuous::linear().domain([-100., 100.]);
        assert_eq!(s.ticks(count), expected);

        let s = ScaleContinuous::linear().domain([100., -100.]);
        assert_eq!(s.ticks(count), reverse(expected));
    }

    #[rustfmt::skip]
    #[test]
    fn test_scale_log_nice() {
        assert_eq!(ScaleContinuous::log10().domain([1.1, 10.9]).nice(None).domain, [1., 100.]);
        assert_eq!(ScaleContinuous::log10().domain([10.9, 1.1]).nice(None).domain, [100., 1.]);
        assert_eq!(ScaleContinuous::log10().domain([0.7, 11.001]).nice(None).domain, [0.1, 100.]);
        assert_eq!(ScaleContinuous::log10().domain([123.1, 6.7]).nice(None).domain, [1000., 1.]);
        assert_eq!(ScaleContinuous::log10().domain([0.01, 0.49]).nice(None).domain, [0.01, 1.]);

        let x = ScaleContinuous::log10().domain([1.5, 50.]).nice(None);
        assert_eq!(x.domain, [1., 100.]);
        assert_eq!(x.range, [0., 1.]);
        assert_eq!(x.apply(1.), 0.);
        assert_eq!(x.apply(100.), 1.);

        let x = ScaleContinuous::log10().domain([0., 0.]).nice(None);
        assert_eq!(x.domain, [0., 0.]);
        assert_eq!(x.domain([0.5, 0.5]).nice(None).domain, [0.1, 1.]);

        let x = ScaleContinuous::log10().domain([1.1, 10.9]).nice(None);
        assert_eq!(x.domain, [1., 100.]);
        assert_eq!(x.domain([-123.1, -0.5]).nice(None).domain, [-1000., -0.1])
    }

    #[rstest]
    #[case([0.15, 0.68], None, &[0.2, 0.3, 0.4, 0.5, 0.6])]
    #[case([0.68, 0.15], None, &[0.6, 0.5, 0.4, 0.3, 0.2])]
    #[case([-0.15, -0.68], None, &[-0.2, -0.3, -0.4, -0.5, -0.6])]
    #[case([-0.68, -0.15], None, &[-0.6, -0.5, -0.4, -0.3, -0.2])]
    #[case([1., 5.], None, &[1., 2., 3., 4., 5.])]
    #[case([5., 1.], None, &[5., 4., 3., 2., 1.])]
    #[case([-1., -5.], None, &[-1., -2., -3., -4., -5.])]
    #[case([-5., -1.], None, &[-5., -4., -3., -2., -1.])]
    #[case([286.9252014, 329.4978332], Some(1), &[300.])]
    #[case([286.9252014, 329.4978332], Some(2), &[300.])]
    #[case([286.9252014, 329.4978332], Some(3), &[300., 320.])]
    #[case([286.9252014, 329.4978332], Some(4), &[290., 300., 310., 320.])]
    #[case([286.9252014, 329.4978332], None, &[290., 295., 300., 305., 310., 315., 320., 325.])]
    #[case([41., 42.], None, &[41., 41.1, 41.2, 41.3, 41.4, 41.5, 41.6, 41.7, 41.8, 41.9, 42.])]
    #[case([42., 41.], None, &[42., 41.9, 41.8, 41.7, 41.6, 41.5, 41.4, 41.3, 41.2, 41.1, 41.])]
    #[case([1600., 1400.], None, &[1600., 1580., 1560., 1540., 1520., 1500., 1480., 1460., 1440., 1420., 1400.])]
    fn test_scale_log10_ticks(
        #[case] domain: [f32; 2],
        #[case] count: Option<usize>,
        #[case] expected: &[f32],
    ) {
        assert_eq!(
            ScaleContinuous::log10().domain(domain).ticks(count),
            expected
        );
    }

    #[rstest]
    #[case([1e-1, 1e1], &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.])]
    #[case([1e-1, 1e0], &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.])]
    #[case([-1e0, -1e-1], &[-1., -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1])]
    #[case([-1e-1, -1e1], &reverse(&[-10., -9., -8., -7., -6., -5., -4., -3., -2., -1., -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1]))]
    #[case([-1e-1, -1e0], &reverse(&[-1., -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1])
    )]
    #[case([1e0, 1e-1], &reverse(&[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.]))]
    fn test_scale_log_ticks_round(#[case] domain: [f32; 2], #[case] expected: &[f32]) {
        assert_eq!(
            round(ScaleContinuous::log10().domain(domain).ticks(None)),
            expected
        );
    }

    #[test]
    fn test_scale_ln_ticks() {
        assert_eq!(
            round_epsilon(ScaleContinuous::ln().domain([0.1, 100.]).ticks(None)),
            [
                0.135335283237,
                0.367879441171,
                1.,
                2.718281828459,
                7.389056098931,
                20.085536923188,
                54.598150033144
            ]
        );
    }
}
