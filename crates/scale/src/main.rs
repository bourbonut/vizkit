use std::marker::PhantomData;

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

enum Clamper {
    Identity,
    Linear { a: f32, b: f32 },
}

impl Clamper {
    fn clamp(&self, x: f32) -> f32 {
        match self {
            Self::Identity => x,
            Self::Linear { a, b } => a.max(b.min(x)),
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

trait Transformer {
    fn transform(&self, x: f32) -> f32;
    fn untransform(&self, y: f32) -> f32;
}

struct Linear;
impl Transformer for Linear {
    fn transform(&self, x: f32) -> f32 {
        x
    }

    fn untransform(&self, y: f32) -> f32 {
        y
    }
}

struct Scale<T: Transformer> {
    transformer: T,
    domain: [f32; 2],
    range: [f32; 2],
    output: BiMap,
    input: BiMap,
    clamper: Clamper,
}

impl Scale<Linear> {
    fn linear() -> Self {
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

impl<T: Transformer> Scale<T> {
    fn domain(self, domain: [f32; 2]) -> Self {
        Self {
            transformer: self.transformer,
            domain,
            range: self.range,
            input: BiMap::new(&self.range, &domain),
            output: BiMap::new(&domain, &self.range),
            clamper: self.clamper,
        }
    }

    fn range(self, range: [f32; 2]) -> Self {
        Self {
            transformer: self.transformer,
            domain: self.domain,
            range,
            input: BiMap::new(&range, &self.domain),
            output: BiMap::new(&self.domain, &range),
            clamper: self.clamper,
        }
    }

    fn clamper(self, clamper: Clamper) -> Self {
        Self {
            transformer: self.transformer,
            domain: self.domain,
            range: self.range,
            input: self.input,
            output: self.output,
            clamper,
        }
    }

    fn apply(&self, x: f32) -> f32 {
        self.output
            .apply(self.transformer.transform(self.clamper.clamp(x)))
    }

    fn invert(&self, y: f32) -> f32 {
        self.clamper
            .clamp(self.transformer.untransform(self.input.apply(y)))
    }
}

fn main() {
    println!(
        "{:?}",
        Scale::linear()
            .domain([0., 10.])
            .range([20., 100.])
            .apply(5.)
    );
}
