use crate::color::Color;

pub trait ColorMap {
    fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>;
}

pub trait Scheme {
    fn scheme(&self) -> Vec<&str>;
}

pub(crate) fn split_colors(string: &str) -> Vec<&str> {
    let n = string.len() / 6;
    (0..n).map(|i| &string[i * 6..(i + 1) * 6]).collect()
}

struct InterpolateBasis {
    channel_values: Vec<f32>,
}

impl InterpolateBasis {
    fn new(channel_values: Vec<f32>) -> Self {
        Self { channel_values }
    }

    fn interpolate(&self, t: f32) -> f32 {
        let n = self.channel_values.len() - 1;
        let i = if t <= 0. {
            0
        } else if t >= 1. {
            n - 1
        } else {
            (t * n as f32) as usize
        };
        let v1 = self.channel_values[i];
        let v2 = self.channel_values[i + 1];
        let v0 = if i > 0 {
            self.channel_values[i - 1]
        } else {
            2. * v1 - v2
        };
        let v3 = if i < n - 1 {
            self.channel_values[i + 2]
        } else {
            2. * v2 - v1
        };
        Self::basis((t - i as f32) / n as f32, v0, v1, v2, v3)
    }

    fn basis(t1: f32, v0: f32, v1: f32, v2: f32, v3: f32) -> f32 {
        let t2 = t1 * t1;
        let t3 = t2 * t1;
        ((1. - 3. * t1 + 3. * t2 - t3) * v0
            + (4. - 6. * t2 + 3. * t3) * v1
            + (1. + 3. * t1 + 3. * t2 - 3. * t3) * v2
            + t3 * v3)
            / 6.
    }
}

pub struct RGBInterpolator {
    r_channel: InterpolateBasis,
    g_channel: InterpolateBasis,
    b_channel: InterpolateBasis,
}

impl RGBInterpolator {
    pub(crate) fn new(colors: Vec<&str>) -> Self {
        let (r_values, g_values, b_values) = colors.into_iter().map(Color::from).fold(
            (vec![], vec![], vec![]),
            |(mut r, mut g, mut b), color| {
                r.push(color.0[0]);
                g.push(color.0[1]);
                b.push(color.0[2]);
                (r, g, b)
            },
        );
        RGBInterpolator {
            r_channel: InterpolateBasis::new(r_values),
            g_channel: InterpolateBasis::new(g_values),
            b_channel: InterpolateBasis::new(b_values),
        }
    }

    pub fn interpolate<T>(&self, t: f32) -> T
    where
        Color: Into<T>,
    {
        Color([
            self.r_channel.interpolate(t),
            self.g_channel.interpolate(t),
            self.b_channel.interpolate(t),
        ])
        .into()
    }
}
