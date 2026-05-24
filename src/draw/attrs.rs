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

/// Arrow attributes
pub struct ArrowAttrs {
    /// Bend angle (default: 0.0)
    pub bend_angle: f32,
    /// Head angle (default: 60.0)
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
