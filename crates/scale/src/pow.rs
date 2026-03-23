use crate::continuous::Transformer;

pub struct Power {
    pub exponent: f32,
}

impl Transformer for Power {
    fn transform(&self, x: f32) -> f32 {
        x.powf(self.exponent)
    }

    fn untransform(&self, y: f32) -> f32 {
        y.powf(1. / self.exponent)
    }
}

pub struct Sqrt;

impl Transformer for Sqrt {
    fn transform(&self, x: f32) -> f32 {
        x.sqrt()
    }

    fn untransform(&self, y: f32) -> f32 {
        y * y
    }
}
