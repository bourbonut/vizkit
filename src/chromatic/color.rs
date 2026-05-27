use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

const A: f32 = -0.14861;
const B: f32 = 1.78277;
const C: f32 = -0.29227;
const D: f32 = -0.90649;
const E: f32 = 1.97294;

/// Represents a color in RGB (red, green, blue) where each channel is a value in [0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub [f32; 3]);
/// Represents a color in HSL (hue, saturation, lightness) where each channel is a value in [0.0,
/// 1.0].
pub(crate) struct Cubehelix(pub [f32; 3]);

impl Default for Color {
    fn default() -> Self {
        Self([1.; 3])
    }
}

/// Represents errors that can occur during parsing operations.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// This happens if a character in the color string is not a valid hex digit.
    ParseIntError(ParseIntError),
    /// The input string has an invalid length excluding '#' for a color code.
    InvalidLength(usize),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseIntError(error) => error.fmt(f),
            Self::InvalidLength(length) => {
                f.write_str(&format!(
                    "Expected color string of length 3 or 6 characters excluding '#' (length found: {})",
                    length
                ))
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseIntError(error) => Some(error),
            Self::InvalidLength(_) => None,
        }
    }
}

/// Converts a string formated in hex color to this type.
///
/// # Panics
///
/// This function panics when the specified string does not have 3 or 6 characters excluding '#' or
/// when a character is not in these the subsets:
///
/// - `0-9`
/// - `a-f`
/// - `A-F`
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use vizkit::chromatic::Color;
///
/// assert_eq!(Color::from_str("#000000"), Ok(Color([0., 0., 0.])));
/// assert_eq!(Color::from_str("#666"), Ok(Color([102. / 255.; 3]))); // 6 => 0x66 = 102
/// assert!(Color::from_str("4I9820842908490").is_err());
/// assert!(Color::from_str("#g0000z").is_err());
/// ```
impl FromStr for Color {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.strip_prefix("#").unwrap_or(string);
        match string.len() {
            3 => Ok(Color([
                u8::from_str_radix(&string[0..1].repeat(2), 16)? as f32 / 255.,
                u8::from_str_radix(&string[1..2].repeat(2), 16)? as f32 / 255.,
                u8::from_str_radix(&string[2..3].repeat(2), 16)? as f32 / 255.,
            ])),
            6 => Ok(Color([
                u8::from_str_radix(&string[0..2], 16)? as f32 / 255.,
                u8::from_str_radix(&string[2..4], 16)? as f32 / 255.,
                u8::from_str_radix(&string[4..6], 16)? as f32 / 255.,
            ])),
            size => Err(ParseError::InvalidLength(size)),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b] = self.0;
        write!(
            f,
            "#{:02x}{:02x}{:02x}",
            (255. * r) as u8,
            (255. * g) as u8,
            (255. * b) as u8
        )
    }
}

/// Converts a string into a color in hex format.
///
/// ```
/// use vizkit::chromatic::Color;
///
/// assert_eq!(String::from(Color([1., 1., 1.])), String::from("#ffffff"));
/// ```
impl From<Color> for String {
    fn from(color: Color) -> String {
        let [r, g, b] = color.0;
        let r = r.clamp(0., 1.);
        let g = g.clamp(0., 1.);
        let b = b.clamp(0., 1.);
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
