use std::f32::consts;

use crate::linear::Linear;
use crate::log::{Ln, Log, Log2, Log10};
use crate::pow::{Power, Sqrt};
use crate::ticks::Tick;

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

pub enum Clamper {
    Identity,
    Linear { a: f32, b: f32 },
}

impl Clamper {
    pub fn clamp(&self, x: f32) -> f32 {
        match self {
            Self::Identity => x,
            Self::Linear { a, b } => x.clamp(*a, *b),
        }
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

pub trait Transformer {
    fn transform(&self, x: f32) -> f32;
    fn untransform(&self, y: f32) -> f32;
}

pub struct Scale<T: Transformer + Tick> {
    transformer: T,
    domain: [f32; 2],
    range: [f32; 2],
    output: BiMap,
    input: BiMap,
    clamper: Clamper,
}

impl<T: Transformer + Tick> Scale<T> {
    pub fn domain(self, domain: [f32; 2]) -> Self {
        Self {
            domain,
            input: BiMap::new(&self.range, &domain.map(|x| self.transformer.transform(x))),
            output: BiMap::new(&domain.map(|x| self.transformer.transform(x)), &self.range),
            ..self
        }
    }

    pub fn range(self, range: [f32; 2]) -> Self {
        Self {
            range,
            input: BiMap::new(&range, &self.domain.map(|x| self.transformer.transform(x))),
            output: BiMap::new(&self.domain.map(|x| self.transformer.transform(x)), &range),
            ..self
        }
    }

    pub fn clamper(self, clamper: Clamper) -> Self {
        Self { clamper, ..self }
    }

    pub fn apply(&self, x: f32) -> f32 {
        self.output
            .apply(self.transformer.transform(self.clamper.clamp(x)))
    }

    pub fn invert(&self, y: f32) -> f32 {
        self.clamper
            .clamp(self.transformer.untransform(self.input.apply(y)))
    }

    pub fn ticks(&self, count: Option<usize>) -> Vec<f32> {
        self.transformer.ticks(&self.domain, count.unwrap_or(10))
    }

    pub fn nice(self, count: Option<usize>) -> Self {
        let domain = self.transformer.nice(&self.domain, count.unwrap_or(10));
        self.domain(domain)
    }
}

impl Scale<Linear> {
    pub fn linear() -> Self {
        Self {
            transformer: Linear,
            domain: [0.0, 1.0],
            range: [0.0, 1.0],
            input: BiMap::default(),
            output: BiMap::default(),
            clamper: Clamper::Identity,
        }
    }
}

impl Scale<Log10> {
    pub fn log10() -> Self {
        let domain = [1., 10.];
        let range = [0., 1.];
        Self {
            transformer: Log10,
            domain,
            range,
            input: BiMap::new(&range, &domain),
            output: BiMap::new(&domain, &range),
            clamper: Clamper::Identity,
        }
    }
}

impl Scale<Log2> {
    pub fn log2() -> Self {
        let domain = [1., 2.];
        let range = [0., 1.];
        Self {
            transformer: Log2,
            domain,
            range,
            input: BiMap::new(&range, &domain),
            output: BiMap::new(&domain, &range),
            clamper: Clamper::Identity,
        }
    }
}

impl Scale<Ln> {
    pub fn ln() -> Self {
        let domain = [1., consts::E];
        let range = [0., 1.];
        Self {
            transformer: Ln,
            domain,
            range,
            input: BiMap::new(&range, &domain),
            output: BiMap::new(&domain, &range),
            clamper: Clamper::Identity,
        }
    }
}

impl Scale<Log> {
    pub fn log(base: f32) -> Self {
        let domain = [1., base];
        let range = [0., 1.];
        Self {
            transformer: Log { base },
            domain,
            range,
            input: BiMap::new(&range, &domain),
            output: BiMap::new(&domain, &range),
            clamper: Clamper::Identity,
        }
    }
}

impl Scale<Power> {
    pub fn pow(exponent: f32) -> Self {
        Self {
            transformer: Power { exponent },
            domain: [0.0, 1.0],
            range: [0.0, 1.0],
            input: BiMap::default(),
            output: BiMap::default(),
            clamper: Clamper::Identity,
        }
    }
}

impl Scale<Sqrt> {
    pub fn sqrt() -> Self {
        Self {
            transformer: Sqrt,
            domain: [0.0, 1.0],
            range: [0.0, 1.0],
            input: BiMap::default(),
            output: BiMap::default(),
            clamper: Clamper::Identity,
        }
    }
}

#[cfg(test)]
mod tests {

    #[rustfmt::skip]
    #[test]
    fn test_scale_linear_nice() {
        assert_eq!(super::Scale::linear().domain([0., 0.96]).nice(None).domain, [0., 1.]);
        assert_eq!(super::Scale::linear().domain([0., 96.]).nice(None).domain, [0., 100.]);
        assert_eq!(super::Scale::linear().domain([0., 0.96]).nice(Some(10)).domain, [0., 1.]);
        assert_eq!(super::Scale::linear().domain([0., 96.]).nice(Some(10)).domain, [0., 100.]);
        assert_eq!(super::Scale::linear().domain([0.96, 0.]).nice(Some(10)).domain, [1., 0.]);
        assert_eq!(super::Scale::linear().domain([96., 0.]).nice(Some(10)).domain, [100., 0.]);
        assert_eq!(super::Scale::linear().domain([0., -0.96]).nice(Some(10)).domain, [0., -1.]);
        assert_eq!(super::Scale::linear().domain([0., -96.]).nice(Some(10)).domain, [0., -100.]);
        assert_eq!(super::Scale::linear().domain([-0.96, 0.]).nice(Some(10)).domain, [-1., 0.]);
        assert_eq!(super::Scale::linear().domain([-96., 0.]).nice(Some(10)).domain, [-100., 0.]);
        assert_eq!(super::Scale::linear().domain([-0.1, 51.1]).nice(Some(8)).domain, [-10., 60.]);
        assert_eq!(super::Scale::linear().domain([1.1, 10.9]).nice(Some(10)).domain, [1., 11.]);
        assert_eq!(super::Scale::linear().domain([10.9, 1.1]).nice(Some(10)).domain, [11., 1.]);
        assert_eq!(super::Scale::linear().domain([0.7, 11.001]).nice(Some(10)).domain, [0., 12.]);
        assert_eq!(super::Scale::linear().domain([123.1, 6.7]).nice(Some(10)).domain, [130., 0.]);
        assert_eq!(super::Scale::linear().domain([0., 0.49]).nice(Some(10)).domain, [0., 0.5]);
        assert_eq!(super::Scale::linear().domain([0., 14.1]).nice(Some(5)).domain, [0., 20.]);
        assert_eq!(super::Scale::linear().domain([0., 15.]).nice(Some(5)).domain, [0., 20.]);
        assert_eq!(super::Scale::linear().domain([1.1, 10.9]).nice(Some(10)).domain, [1., 11.]);
        assert_eq!(super::Scale::linear().domain([123.1, -0.9]).nice(Some(10)).domain, [130., -10.]);
        assert_eq!(super::Scale::linear().domain([12., 87.]).nice(Some(5)).domain, [0., 100.]);
        assert_eq!(super::Scale::linear().domain([12., 87.]).nice(Some(10)).domain, [10., 90.]);
        assert_eq!(super::Scale::linear().domain([12., 87.]).nice(Some(100)).domain, [12., 87.]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_scale_linear_ticks() {
        let s = super::Scale::linear();
        let round_epsilon = |vec: Vec<f32>| vec.iter().map(|x| (x * 1e12).round() / 1e12).collect::<Vec<f32>>();
        let reverse = |arr: Vec<f32>| arr.into_iter().rev().collect::<Vec<f32>>();
        assert_eq!(round_epsilon(s.ticks(Some(10))), [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]);
        assert_eq!(round_epsilon(s.ticks(Some(9))), [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]);
        assert_eq!(round_epsilon(s.ticks(Some(8))), [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]);
        assert_eq!(round_epsilon(s.ticks(Some(7))), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(round_epsilon(s.ticks(Some(6))), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(round_epsilon(s.ticks(Some(5))), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(round_epsilon(s.ticks(Some(4))), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(round_epsilon(s.ticks(Some(3))), [0.0, 0.5, 1.0]);
        assert_eq!(round_epsilon(s.ticks(Some(2))), [0.0, 0.5, 1.0]);
        assert_eq!(round_epsilon(s.ticks(Some(1))), [0.0, 1.0]);

        let s = s.domain([-100., 100.]);
        assert_eq!(s.ticks(Some(10)), [-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.]);
        assert_eq!(s.ticks(Some(9)), [-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.]);
        assert_eq!(s.ticks(Some(8)), [-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.]);
        assert_eq!(s.ticks(Some(7)), [-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.]);
        assert_eq!(s.ticks(Some(6)), [-100., -50., 0., 50., 100.]);
        assert_eq!(s.ticks(Some(5)), [-100., -50., 0., 50., 100.]);
        assert_eq!(s.ticks(Some(4)), [-100., -50., 0., 50., 100.]);
        assert_eq!(s.ticks(Some(3)), [-100., -50., 0., 50., 100.]);
        assert_eq!(s.ticks(Some(2)), [-100., 0., 100.]);
        assert_eq!(s.ticks(Some(1)), [0.]);

        let s = super::Scale::linear().domain([1., 0.]);
        assert_eq!(round_epsilon(s.ticks(Some(10))), reverse(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]));
        assert_eq!(round_epsilon(s.ticks(Some(9))), reverse(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]));
        assert_eq!(round_epsilon(s.ticks(Some(8))), reverse(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]));
        assert_eq!(round_epsilon(s.ticks(Some(7))), reverse(vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0]));
        assert_eq!(round_epsilon(s.ticks(Some(6))), reverse(vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0]));
        assert_eq!(round_epsilon(s.ticks(Some(5))), reverse(vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0]));
        assert_eq!(round_epsilon(s.ticks(Some(4))), reverse(vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0]));
        assert_eq!(round_epsilon(s.ticks(Some(3))), reverse(vec![0.0, 0.5, 1.0]));
        assert_eq!(round_epsilon(s.ticks(Some(2))), reverse(vec![0.0, 0.5, 1.0]));
        assert_eq!(round_epsilon(s.ticks(Some(1))), reverse(vec![0.0, 1.0]));

        let s = s.domain([100., -100.]);
        assert_eq!(s.ticks(Some(10)), reverse(vec![-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.]));
        assert_eq!(s.ticks(Some(9)), reverse(vec![-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.]));
        assert_eq!(s.ticks(Some(8)), reverse(vec![-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.]));
        assert_eq!(s.ticks(Some(7)), reverse(vec![-100., -80., -60., -40., -20., 0., 20., 40., 60., 80., 100.]));
        assert_eq!(s.ticks(Some(6)), reverse(vec![-100., -50., 0., 50., 100.]));
        assert_eq!(s.ticks(Some(5)), reverse(vec![-100., -50., 0., 50., 100.]));
        assert_eq!(s.ticks(Some(4)), reverse(vec![-100., -50., 0., 50., 100.]));
        assert_eq!(s.ticks(Some(3)), reverse(vec![-100., -50., 0., 50., 100.]));
        assert_eq!(s.ticks(Some(2)), reverse(vec![-100., 0., 100.]));
        assert_eq!(s.ticks(Some(1)), vec![0.]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_scale_log_nice() {
        assert_eq!(super::Scale::log10().domain([1.1, 10.9]).nice(None).domain, [1., 100.]);
        assert_eq!(super::Scale::log10().domain([10.9, 1.1]).nice(None).domain, [100., 1.]);
        assert_eq!(super::Scale::log10().domain([0.7, 11.001]).nice(None).domain, [0.1, 100.]);
        assert_eq!(super::Scale::log10().domain([123.1, 6.7]).nice(None).domain, [1000., 1.]);
        assert_eq!(super::Scale::log10().domain([0.01, 0.49]).nice(None).domain, [0.01, 1.]);

        let x = super::Scale::log10().domain([1.5, 50.]).nice(None);
        assert_eq!(x.domain, [1., 100.]);
        assert_eq!(x.range, [0., 1.]);
        assert_eq!(x.apply(1.), 0.);
        assert_eq!(x.apply(100.), 1.);

        let x = super::Scale::log10().domain([0., 0.]).nice(None);
        assert_eq!(x.domain, [0., 0.]);
        assert_eq!(x.domain([0.5, 0.5]).nice(None).domain, [0.1, 1.]);

        let x = super::Scale::log10().domain([1.1, 10.9]).nice(None);
        assert_eq!(x.domain, [1., 100.]);
        assert_eq!(x.domain([-123.1, -0.5]).nice(None).domain, [-1000., -0.1])
    }

    #[rustfmt::skip]
    #[test]
    fn test_scale_log_ticks() {
        let round = |vec: Vec<f32>| vec.iter().map(|x| (x * 10.).round() / 10.).collect::<Vec<f32>>();
        let reverse = |arr: Vec<f32>| arr.into_iter().rev().collect::<Vec<f32>>();
        assert_eq!(super::Scale::log10().domain([0.15, 0.68]).ticks(None), [0.2, 0.3, 0.4, 0.5, 0.6]);
        assert_eq!(super::Scale::log10().domain([0.68, 0.15]).ticks(None), [0.6, 0.5, 0.4, 0.3, 0.2]);
        assert_eq!(super::Scale::log10().domain([-0.15, -0.68]).ticks(None), [-0.2, -0.3, -0.4, -0.5, -0.6]);
        assert_eq!(super::Scale::log10().domain([-0.68, -0.15]).ticks(None), [-0.6, -0.5, -0.4, -0.3, -0.2]);

        assert_eq!(
            round(super::Scale::log10().domain([1e-1, 1e1]).ticks(None)),
            [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]
        );
        assert_eq!(
            round(super::Scale::log10().domain([1e-1, 1e0]).ticks(None)),
            [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.]
        );
        assert_eq!(
            round(super::Scale::log10().domain([-1e0, -1e-1]).ticks(None)),
            [-1., -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1]
        );

        assert_eq!(
            round(super::Scale::log10().domain([-1e-1, -1e1]).ticks(None)),
            reverse(vec![-10., -9., -8., -7., -6., -5., -4., -3., -2., -1., -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1])
        );
        assert_eq!(
            round(super::Scale::log10().domain([-1e-1, -1e0]).ticks(None)),
            reverse(vec![-1., -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1])
        );
        assert_eq!(
            round(super::Scale::log10().domain([1e0, 1e-1]).ticks(None)),
            reverse(vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.])
        );

        assert_eq!(super::Scale::log10().domain([1., 5.]).ticks(None), [1., 2., 3., 4., 5.]);
        assert_eq!(super::Scale::log10().domain([5., 1.]).ticks(None), [5., 4., 3., 2., 1.]);
        assert_eq!(super::Scale::log10().domain([-1., -5.]).ticks(None), [-1., -2., -3., -4., -5.]);
        assert_eq!(super::Scale::log10().domain([-5., -1.]).ticks(None), [-5., -4., -3., -2., -1.]);
        assert_eq!(super::Scale::log10().domain([286.9252014, 329.4978332]).ticks(Some(1)), [300.]);
        assert_eq!(super::Scale::log10().domain([286.9252014, 329.4978332]).ticks(Some(2)), [300.]);
        assert_eq!(super::Scale::log10().domain([286.9252014, 329.4978332]).ticks(Some(3)), [300., 320.]);
        assert_eq!(super::Scale::log10().domain([286.9252014, 329.4978332]).ticks(Some(4)), [290., 300., 310., 320.]);
        assert_eq!(super::Scale::log10().domain([286.9252014, 329.4978332]).ticks(None), [290., 295., 300., 305., 310., 315., 320., 325.]);

        assert_eq!(
            super::Scale::log10().domain([41., 42.]).ticks(None),
            [41., 41.1, 41.2, 41.3, 41.4, 41.5, 41.6, 41.7, 41.8, 41.9, 42.]
        );
        assert_eq!(
            super::Scale::log10().domain([42., 41.]).ticks(None),
            [42., 41.9, 41.8, 41.7, 41.6, 41.5, 41.4, 41.3, 41.2, 41.1, 41.]
        );
        assert_eq!(
            super::Scale::log10().domain([1600., 1400.]).ticks(None),
            [1600., 1580., 1560., 1540., 1520., 1500., 1480., 1460., 1440., 1420., 1400.]
        );

        let round = |vec: Vec<f32>| vec.iter().map(|x| (x * 1e12).round() * 1e-12).collect::<Vec<f32>>();
        assert_eq!(
            round(super::Scale::ln().domain([0.1, 100.]).ticks(None)),
            [0.135335283237, 0.367879441171, 1., 2.718281828459, 7.389056098931, 20.085536923188, 54.598150033144]
        );
    }
}
