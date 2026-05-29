use crate::chromatic::Color;

/// Shape attributes for stroke and fill values.
///
/// Please note:
///
/// | Function | `fill_color` | `stroke_color` |
/// | -------- | ------------ | -------------- |
/// | [`ShapeAttrs::default`] | None | None |
/// | [`ShapeAttrs::fill_default`] | white color | None |
/// | [`ShapeAttrs::stroke_default`] | None | white color |
pub struct ShapeAttrs {
    /// Fill color (default: None)
    pub fill_color: Option<Color>,
    /// Fill opacity (default: 1.0)
    pub fill_opacity: f32,
    /// Stroke color (default: None)
    pub stroke_color: Option<Color>,
    /// Stroke width (default: 1.0)
    pub stroke_width: f32,
    /// Stroke opacity (default: 1.0)
    pub stroke_opacity: f32,
}

impl Default for ShapeAttrs {
    fn default() -> Self {
        Self {
            fill_color: None,
            fill_opacity: 1.,
            stroke_color: None,
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}

impl ShapeAttrs {
    /// All default values but `fill_color` is white color.
    pub fn fill_default() -> Self {
        Self {
            fill_color: Some(Color::default()),
            fill_opacity: 1.,
            stroke_color: None,
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }

    /// All default values but `stroke_color` is white color.
    pub fn stroke_default() -> Self {
        Self {
            fill_color: None,
            fill_opacity: 1.,
            stroke_color: Some(Color::default()),
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}

/// Line attributes for stroke values.
pub struct LineAttrs {
    /// Stroke color (default: white color)
    pub stroke_color: Color,
    /// Stroke width (default: 1.0)
    pub stroke_width: f32,
    /// Stroke opacity (default: 1.0)
    pub stroke_opacity: f32,
}

impl Default for LineAttrs {
    fn default() -> Self {
        Self {
            stroke_color: Color::default(),
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}

/// Text alignment variants used in [`TextAttrs`] and
/// [`TextProperties`][`crate::draw::TextProperties`]
#[derive(Default, Clone)]
pub enum Alignment {
    /// Indicates to align the text on the start
    Start,
    /// Indicates to align the text on the center (default variant)
    #[default]
    Center,
    /// Indicates to align the text on the end
    End,
}

/// Text attributes for text values, fill values and alignments.
pub struct TextAttrs {
    /// Fill color (default: white color)
    pub fill_color: Color,
    /// Font size (default: 12)
    pub font_size: f32,
    /// X-oriented text alignment (default: center)
    pub align_x: Alignment,
    /// Y-oriented text alignment (default: center)
    pub align_y: Alignment,
}

impl Default for TextAttrs {
    fn default() -> Self {
        Self {
            fill_color: Color::default(),
            font_size: 12.,
            align_x: Alignment::default(),
            align_y: Alignment::default(),
        }
    }
}

/// Arrow attributes for shape and color values.
///
/// <svg height="300" viewBox="0 0 800 300" width="800" xmlns="http://www.w3.org/2000/svg">
/// <path d="M200,225L200,225Q286.74182,152.20596,400,152.20596Q513.2582,152.20596,600.0145,225.01219L600,225M586.3192,187.4123L600,225L560.60767,218.05408" fill="none" stroke="currentColor" stroke-width="2"/>
/// <line stroke="currentColor" stroke-dasharray="8 8" x1="200" x2="400" y1="225" y2="57.180084"/>
/// <line stroke="currentColor" stroke-dasharray="8 8" x1="600" x2="400" y1="225" y2="57.180084"/>
/// <line stroke="currentColor" stroke-dasharray="8 8" x1="600" x2="403.03833" y1="225" y2="190.27039"/>
/// <circle cx="400" cy="57.180084" fill="var(--link-color)" r="3.5"/>
/// <circle cx="200" cy="225" fill="var(--link-color)" r="3.5"/>
/// <circle cx="600" cy="225" fill="var(--link-color)" r="3.5"/>
/// <circle cx="403.03833" cy="190.27039" fill="var(--link-color)" r="3.5"/>
/// <circle cx="560.60767" cy="218.05408" fill="var(--link-color)" r="3.5"/>
/// <text fill="var(--link-color)" font-size="16" text-anchor="middle" transform="translate(400, 67.180084)" y="0.71em">C</text>
/// <text fill="var(--link-color)" font-size="16" text-anchor="middle" transform="translate(200, 235)" y="0.71em">A</text>
/// <text fill="var(--link-color)" font-size="16" text-anchor="middle" transform="translate(600, 235)" y="0.71em">B</text>
/// <text fill="var(--link-color)" font-size="16" text-anchor="middle" transform="translate(403.03833, 200.27039)" y="0.71em">S</text>
/// <text fill="var(--link-color)" font-size="16" text-anchor="middle" transform="translate(560.60767, 228.05408)" y="0.71em">H</text>
/// </svg>
///
/// - The bend angle is defined by the three points _A_, _C_, and _B_.
/// - The head angle is defined by the three points _S_, _B_, and _C_.
/// - The head length is the distance between points _H_ and _B_.
pub struct ArrowAttrs {
    /// Bend angle in degrees (default: 0.0)
    pub bend_angle: f32,
    /// Head angle in degrees (default: 60.0)
    pub head_angle: f32,
    /// Head length (default: 8.0)
    pub head_length: f32,
    /// Stroke color (default: white color)
    pub stroke_color: Color,
    /// Stroke width (default: 1.0)
    pub stroke_width: f32,
    /// Stroke opacity (default: 1.0)
    pub stroke_opacity: f32,
}

impl Default for ArrowAttrs {
    fn default() -> Self {
        Self {
            bend_angle: 0.,
            head_angle: 60.,
            head_length: 8.,
            stroke_color: Color::default(),
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}
