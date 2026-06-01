use std::mem::swap;

use super::{
    continuous::Transformer,
    ticks::{Tick, tick_increment, ticks},
};

/// Power transformation (`x.powf(exponent)`)
#[derive(Clone, Copy)]
pub struct Power {
    pub exponent: f32,
}

impl Transformer for Power {
    #[inline]
    fn transform(&self, x: f32) -> f32 {
        x.powf(self.exponent)
    }

    #[inline]
    fn untransform(&self, y: f32) -> f32 {
        y.powf(1. / self.exponent)
    }
}

/// Square root transformation
#[derive(Clone, Copy)]
pub struct Sqrt;

impl Transformer for Sqrt {
    #[inline]
    fn transform(&self, x: f32) -> f32 {
        x.sqrt()
    }

    #[inline]
    fn untransform(&self, y: f32) -> f32 {
        y * y
    }
}

fn nice(domain: &[f32; 2], count: usize) -> [f32; 2] {
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

macro_rules! impl_tick {
    ($type_name:ty) => {
        impl Tick for $type_name {
            fn ticks(&self, domain: &[f32; 2], count: usize) -> Vec<f32> {
                ticks(domain[0], domain[1], count)
            }

            fn nice(&self, domain: &[f32; 2], count: usize) -> [f32; 2] {
                nice(domain, count)
            }
        }
    };
}

impl_tick!(Power);
impl_tick!(Sqrt);

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts;

    #[test]
    fn test_power() {
        let power = Power { exponent: 3. };
        assert_eq!(power.untransform(power.transform(consts::PI)), consts::PI);
    }

    #[test]
    fn test_sqrt() {
        let sqrt = Sqrt;
        assert_eq!(sqrt.untransform(sqrt.transform(consts::PI)), consts::PI);
    }
}
