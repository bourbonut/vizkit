use crate::{color::Color, common::split_colors};

struct InterpolateBasis {
    channel_values: Vec<f32>,
}

impl InterpolateBasis {
    fn new(channel_values: Vec<f32>) -> Self {
        Self { channel_values }
    }

    fn interpolate(&self, t: f32) -> f32 {
        let n = self.channel_values.len();
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

struct RGBInterpolator {
    r_channel: InterpolateBasis,
    g_channel: InterpolateBasis,
    b_channel: InterpolateBasis,
}

impl RGBInterpolator {
    fn new(colors: Vec<&str>) -> Self {
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

    fn interpolate<T>(&self, t: f32) -> T
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

enum Diverging {
    BrBg,
    PiYg,
    PrGn,
    PuOr,
    RdBu,
    RdGy,
    RdYlBu,
    RdYlGn,
    Spectral,
}

impl Diverging {
    fn interpolator(&self) -> RGBInterpolator {
        match self {
            Self::BrBg => RGBInterpolator::new(split_colors(
                "5430058c510abf812ddfc27df6e8c3f5f5f5c7eae580cdc135978f01665e003c30",
            )),
            Self::PiYg => RGBInterpolator::new(split_colors(
                "8e0152c51b7dde77aef1b6dafde0eff7f7f7e6f5d0b8e1867fbc414d9221276419",
            )),
            Self::PrGn => RGBInterpolator::new(split_colors(
                "40004b762a839970abc2a5cfe7d4e8f7f7f7d9f0d3a6dba05aae611b783700441b",
            )),
            Self::PuOr => RGBInterpolator::new(split_colors(
                "2d004b5427888073acb2abd2d8daebf7f7f7fee0b6fdb863e08214b358067f3b08",
            )),
            Self::RdBu => RGBInterpolator::new(split_colors(
                "67001fb2182bd6604df4a582fddbc7f7f7f7d1e5f092c5de4393c32166ac053061",
            )),
            Self::RdGy => RGBInterpolator::new(split_colors(
                "67001fb2182bd6604df4a582fddbc7ffffffe0e0e0bababa8787874d4d4d1a1a1a",
            )),
            Self::RdYlBu => RGBInterpolator::new(split_colors(
                "a50026d73027f46d43fdae61fee090ffffbfe0f3f8abd9e974add14575b4313695",
            )),
            Self::RdYlGn => RGBInterpolator::new(split_colors(
                "a50026d73027f46d43fdae61fee08bffffbfd9ef8ba6d96a66bd631a9850006837",
            )),
            Self::Spectral => RGBInterpolator::new(split_colors(
                "9e0142d53e4ff46d43fdae61fee08bffffbfe6f598abdda466c2a53288bd5e4fa2",
            )),
        }
    }
}
