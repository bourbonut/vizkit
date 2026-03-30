use std::f32;
use std::mem::swap;
use std::usize;

use crate::continuous::Transformer;
use crate::ticks::{Tick, ticks};

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

impl Log10 {
    fn base(&self) -> f32 {
        10.
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

impl Log2 {
    fn base(&self) -> f32 {
        2.
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

impl Ln {
    fn base(&self) -> f32 {
        f32::consts::E
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

impl Log {
    fn base(&self) -> f32 {
        self.base
    }
}

macro_rules! impl_tick {
    ($type_name:ty) => {
        impl Tick for $type_name {
            fn ticks(&self, domain: &[f32; 2], count: usize) -> Vec<f32> {
                let &[mut u, mut v] = domain;
                let reverse = v < u;

                if reverse {
                    swap(&mut u, &mut v);
                }

                // TODO: Make a structure or enum to avoid Box
                let transform: Box<dyn Fn(f32) -> f32> = if u > 0. {
                    Box::new(|x: f32| self.transform(x))
                } else {
                    Box::new(|x: f32| -self.transform(-x))
                };

                let untransform: Box<dyn Fn(f32) -> f32> = if u > 0. {
                    Box::new(|x: f32| self.untransform(x))
                } else {
                    Box::new(|x: f32| -self.untransform(-x))
                };

                let i = transform(u);
                let j = transform(v);
                let n = count;
                let mut z = Vec::new();
                let base = self.base();
                if self.base() % 1. == 0. && j - i < n as f32 {
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
                        z = ticks(u, v, n);
                    }
                } else {
                    z = ticks(i, j, ((j - i) as usize).min(n))
                        .into_iter()
                        .map(|y| self.untransform(y))
                        .collect();
                }

                if reverse {
                    z.reverse();
                }
                z
            }

            fn nice(&self, domain: &[f32; 2], _: usize) -> [f32; 2] {
                let &[mut x0, mut x1] = domain;
                let reverse = x1 < x0;
                if reverse {
                    swap(&mut x0, &mut x1);
                }

                x0 = if x0 == 0. {
                    x0
                } else if x0 > 0. {
                    self.untransform(self.transform(x0).floor())
                } else {
                    -self.untransform(-(-self.transform(-x0)).floor())
                };
                x1 = if x1 == 0. {
                    x1
                } else if x1 > 0. {
                    self.untransform(self.transform(x1).ceil())
                } else {
                    -self.untransform(-(-self.transform(-x1)).ceil())
                };
                if reverse { [x1, x0] } else { [x0, x1] }
            }
        }
    };
}

impl_tick!(Log);
impl_tick!(Log2);
impl_tick!(Log10);
impl_tick!(Ln);
