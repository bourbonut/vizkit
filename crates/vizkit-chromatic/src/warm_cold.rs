use crate::color::{Color, Cubehelix, Interpolator, color};

#[derive(Clone)]
pub struct CubehelixInterpolator {
    gamma: f32,
    h: Interpolator,
    s: Interpolator,
    l: Interpolator,
}

impl CubehelixInterpolator {
    pub(crate) fn gamma(self, gamma: f32) -> Self {
        Self { gamma, ..self }
    }

    pub(crate) fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        Color::from(Cubehelix([
            self.h.interpolate(t),
            self.s.interpolate(t),
            self.l.interpolate(t.powf(self.gamma)),
        ]))
        .into()
    }

    pub(crate) fn warm() -> Self {
        let [sh, ss, sl] = [-100., 0.75, 0.35];
        let [eh, es, el] = [80., 1.50, 0.8];
        let h = color(sh, eh);
        let s = color(ss, es);
        let l = color(sl, el);
        Self { gamma: 1., h, s, l }
    }

    pub(crate) fn cold() -> Self {
        let [sh, ss, sl] = [260., 0.75, 0.35];
        let [eh, es, el] = [80., 1.50, 0.8];
        let h = color(sh, eh);
        let s = color(ss, es);
        let l = color(sl, el);
        Self { gamma: 1., h, s, l }
    }
}

impl Default for CubehelixInterpolator {
    fn default() -> Self {
        let [sh, ss, sl] = [300., 0.5, 0.0];
        let [eh, es, el] = [-240., 0.5, 1.0];
        let h = color(sh, eh);
        let s = color(ss, es);
        let l = color(sl, el);
        Self { gamma: 1., h, s, l }
    }
}
