use std::mem::swap;

use super::{
    continuous::Transformer,
    ticks::{Tick, tick_increment, ticks},
};

/// Power transformation (`x.powf(exponent)`)
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

/// Square root transformation
pub struct Sqrt;

impl Transformer for Sqrt {
    fn transform(&self, x: f32) -> f32 {
        x.sqrt()
    }

    fn untransform(&self, y: f32) -> f32 {
        y * y
    }
}

macro_rules! impl_tick {
    ($type_name:ty) => {
        impl Tick for $type_name {
            fn ticks(&self, domain: &[f32; 2], count: usize) -> Vec<f32> {
                ticks(domain[0], domain[1], count)
            }

            fn nice(&self, domain: &[f32; 2], count: usize) -> [f32; 2] {
                let &[mut start, mut stop] = domain;
                let mut prestep = None;

                let reverse = stop < start;
                if reverse {
                    swap(&mut start, &mut stop);
                }

                let mut max_iter = 10;
                while max_iter > 0 {
                    let step = tick_increment(start, stop, count);
                    if Some(step) == prestep {
                        if reverse {
                            swap(&mut start, &mut stop);
                        }
                        return [start, stop];
                    } else if step > 0. {
                        start = (start / step).floor() * step;
                        stop = (stop / step).ceil() * step;
                    } else if step < 0. {
                        start = (start * step).ceil() / step;
                        stop = (stop * step).floor() / step;
                    } else {
                        break;
                    }
                    prestep = Some(step);
                    max_iter -= 1;
                }
                if reverse {
                    swap(&mut start, &mut stop);
                }
                [start, stop]
            }
        }
    };
}

impl_tick!(Power);
impl_tick!(Sqrt);
