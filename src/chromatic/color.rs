const A: f32 = -0.14861;
const B: f32 = 1.78277;
const C: f32 = -0.29227;
const D: f32 = -0.90649;
const E: f32 = 1.97294;

pub struct Color(pub [f32; 3]);
pub struct Cubehelix(pub [f32; 3]);

impl From<&str> for Color {
    fn from(string: &str) -> Self {
        Color([
            u8::from_str_radix(&string[0..2], 16).unwrap_or_default() as f32 / 255.,
            u8::from_str_radix(&string[2..4], 16).unwrap_or_default() as f32 / 255.,
            u8::from_str_radix(&string[4..6], 16).unwrap_or_default() as f32 / 255.,
        ])
    }
}

impl From<Color> for String {
    fn from(color: Color) -> String {
        let [r, g, b] = color.0;
        format!(
            "#{:02x}{:02x}{:02x}",
            (255. * r) as u8,
            (255. * g) as u8,
            (255. * b) as u8
        )
    }
}

impl From<Color> for [f32; 3] {
    fn from(color: Color) -> [f32; 3] {
        color.0
    }
}

impl From<Cubehelix> for Color {
    fn from(cubehelix: Cubehelix) -> Color {
        let [h, s, l] = cubehelix.0;
        let h = if h.is_nan() {
            0.
        } else {
            (h + 120.).to_radians()
        };
        let a = if s.is_nan() { 0. } else { s * l * (1. - l) };
        let cosh = h.cos();
        let sinh = h.sin();
        Color([
            (l + a * (A * cosh + B * sinh)).clamp(0., 1.),
            (l + a * (C * cosh + D * sinh)).clamp(0., 1.),
            (l + a * (E * cosh)).clamp(0., 1.),
        ])
    }
}

#[derive(Clone)]
pub enum Interpolator {
    Linear { a: f32, d: f32 },
    Constant { value: f32 },
}

impl Interpolator {
    pub fn interpolate(&self, t: f32) -> f32 {
        match self {
            Self::Linear { a, d } => a + t * d,
            Self::Constant { value } => *value,
        }
    }
}

pub fn color(a: f32, b: f32) -> Interpolator {
    let d = b - a;
    if !d.is_nan() && d != 0. {
        Interpolator::Linear { a, d }
    } else {
        Interpolator::Constant {
            value: if a.is_nan() { b } else { a },
        }
    }
}
