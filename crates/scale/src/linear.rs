use crate::continuous::Transformer;
use crate::ticks::{Tick, tick_increment, ticks};

pub struct Linear;
impl Transformer for Linear {
    fn transform(&self, x: f32) -> f32 {
        x
    }

    fn untransform(&self, y: f32) -> f32 {
        y
    }
}

impl Tick for Linear {
    fn ticks(&self, domain: &[f32; 2], count: usize) -> Vec<f32> {
        ticks(domain[0], domain[1], count)
    }

    fn nice(&self, domain: &[f32; 2], count: usize) -> [f32; 2] {
        let mut start = domain[0];
        let mut stop = domain[1];

        let mut prestep = None;

        let mut max_iter = 10;
        while max_iter > 0 {
            let step = tick_increment(start, stop, count);
            if Some(step) == prestep {
                return [start, stop];
            } else if step > 0. {
                start = (start / step).floor() * step;
                stop = (stop / step).ceil() * step;
            } else if step < 0. {
                start = (start / step).ceil() * step;
                stop = (stop / step).floor() * step;
            } else {
                break;
            }
            prestep = Some(step);
            max_iter -= 1;
        }
        [start, stop]
    }
}
