use std::mem::swap;

use super::{
    continuous::Transformer,
    ticks::{Tick, tick_increment, ticks},
};

/// Linear transformation
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
