use crate::continuous::Transformer;

// TODO: negative values for domain

pub struct Log10;
impl Transformer for Log10 {
    fn transform(&self, x: f32) -> f32 {
        x.log10()
    }

    fn untransform(&self, y: f32) -> f32 {
        10_f32.powf(y)
    }
}

pub struct Log2;
impl Transformer for Log2 {
    fn transform(&self, x: f32) -> f32 {
        x.log2()
    }

    fn untransform(&self, y: f32) -> f32 {
        2_f32.powf(y)
    }
}

pub struct Ln;
impl Transformer for Ln {
    fn transform(&self, x: f32) -> f32 {
        x.ln()
    }

    fn untransform(&self, y: f32) -> f32 {
        y.exp()
    }
}

pub struct Log {
    pub base: f32,
}
impl Transformer for Log {
    fn transform(&self, x: f32) -> f32 {
        x.log(self.base)
    }

    fn untransform(&self, y: f32) -> f32 {
        self.base.powf(y)
    }
}
