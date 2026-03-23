use crate::continuous::Transformer;

pub struct Linear;
impl Transformer for Linear {
    fn transform(&self, x: f32) -> f32 {
        x
    }

    fn untransform(&self, y: f32) -> f32 {
        y
    }
}
