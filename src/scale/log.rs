use std::f32;
use std::mem::swap;

use super::{continuous::Transformer, ticks::Tick};

/// Logarithm transformation (base `10`)
#[derive(Clone, Copy)]
pub struct Log10;
impl Transformer for Log10 {
    #[inline]
    fn transform(&self, x: f32) -> f32 {
        x.log10()
    }

    #[inline]
    fn untransform(&self, y: f32) -> f32 {
        10_f32.powf(y)
    }
}

impl Log10 {
    #[inline]
    fn base(&self) -> f32 {
        10.
    }
}

/// Logarithm transformation (base `2`)
#[derive(Clone, Copy)]
pub struct Log2;
impl Transformer for Log2 {
    #[inline]
    fn transform(&self, x: f32) -> f32 {
        x.log2()
    }

    #[inline]
    fn untransform(&self, y: f32) -> f32 {
        2_f32.powf(y)
    }
}

impl Log2 {
    #[inline]
    fn base(&self) -> f32 {
        2.
    }
}

/// Natural logarithm transformation (base `e`)
#[derive(Clone, Copy)]
pub struct Ln;
impl Transformer for Ln {
    #[inline]
    fn transform(&self, x: f32) -> f32 {
        x.ln()
    }

    #[inline]
    fn untransform(&self, y: f32) -> f32 {
        y.exp()
    }
}

impl Ln {
    #[inline]
    fn base(&self) -> f32 {
        f32::consts::E
    }
}

/// Logarithm transformation
#[derive(Clone, Copy)]
pub struct Log {
    pub base: f32,
}
impl Transformer for Log {
    #[inline]
    fn transform(&self, x: f32) -> f32 {
        x.log(self.base)
    }

    #[inline]
    fn untransform(&self, y: f32) -> f32 {
        self.base.powf(y)
    }
}

impl Log {
    #[inline]
    fn base(&self) -> f32 {
        self.base
    }
}

fn ticks(trans: &impl Transformer, base: f32, domain: &[f32; 2], count: usize) -> Vec<f32> {
    let &[mut u, mut v] = domain;
    let reverse = v < u;

    if reverse {
        swap(&mut u, &mut v);
    }

    let sign = if u > 0. { 1. } else { -1. };
    let transform = |x| sign * trans.transform(sign * x);
    let untransform = |x| sign * trans.untransform(sign * x);

    let i = transform(u);
    let j = transform(v);
    let n = count;
    let mut z = Vec::new();
    if base % 1. == 0. && j - i < n as f32 {
        let base = base as usize;
        let start = i.floor() as i32;
        let end = j.ceil() as i32;
        if u > 0. {
            for i in start..end + 1 {
                for k in 1..base {
                    let t = if i < 0 {
                        k as f32 / untransform(-(i as f32))
                    } else {
                        k as f32 * untransform(i as f32)
                    };
                    if t < u {
                        continue;
                    }
                    if t > v {
                        break;
                    }
                    z.push(t);
                }
            }
        } else {
            for i in start..end + 1 {
                for k in (1..base).rev() {
                    let t = if i > 0 {
                        k as f32 / untransform(-(i as f32))
                    } else {
                        k as f32 * untransform(i as f32)
                    };
                    if t < u {
                        continue;
                    }
                    if t > v {
                        break;
                    }
                    z.push(t);
                }
            }
        }
        if z.len() * 2 < n {
            z = super::ticks::ticks(u, v, n);
        }
    } else {
        z = super::ticks::ticks(i, j, ((j - i) as usize).min(n))
            .into_iter()
            .map(|y| trans.untransform(y))
            .collect();
    }

    if reverse {
        z.reverse();
    }
    z
}

fn nice(trans: &impl Transformer, domain: &[f32; 2], _: usize) -> [f32; 2] {
    let &[mut x0, mut x1] = domain;
    let reverse = x1 < x0;
    if reverse {
        swap(&mut x0, &mut x1);
    }

    x0 = if x0 == 0. {
        x0
    } else if x0 > 0. {
        trans.untransform(trans.transform(x0).floor())
    } else {
        -trans.untransform(-(-trans.transform(-x0)).floor())
    };
    x1 = if x1 == 0. {
        x1
    } else if x1 > 0. {
        trans.untransform(trans.transform(x1).ceil())
    } else {
        -trans.untransform(-(-trans.transform(-x1)).ceil())
    };
    if reverse { [x1, x0] } else { [x0, x1] }
}

macro_rules! impl_tick {
    ($type_name:ty) => {
        impl Tick for $type_name {
            fn ticks(&self, domain: &[f32; 2], count: usize) -> Vec<f32> {
                ticks(self, self.base(), domain, count)
            }

            fn nice(&self, domain: &[f32; 2], count: usize) -> [f32; 2] {
                nice(self, domain, count)
            }
        }
    };
}

impl_tick!(Log);
impl_tick!(Log2);
impl_tick!(Log10);
impl_tick!(Ln);

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts;

    #[test]
    fn test_log() {
        let log = Log { base: 8. };
        assert_eq!(log.untransform(log.transform(consts::PI)), consts::PI);
        assert_eq!(log.base(), 8.);
    }

    #[test]
    fn test_log10() {
        let log = Log10;
        assert_eq!(log.untransform(log.transform(consts::PI)), consts::PI);
        assert_eq!(log.base(), 10.);
    }

    #[test]
    fn test_log2() {
        let log = Log2;
        assert_eq!(log.untransform(log.transform(consts::PI)), consts::PI);
        assert_eq!(log.base(), 2.);
    }

    #[test]
    fn test_ln() {
        let log = Ln;
        assert!((log.untransform(log.transform(consts::PI)) - consts::PI).abs() < 1e-5);
        assert_eq!(log.base(), consts::E);
    }
}
