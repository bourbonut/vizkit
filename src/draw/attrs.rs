use crate::chromatic::Color;

/// Shape attributes for stroke and fill values.
///
/// Please note:
///
/// | Function | `fill_color` | `stroke_color` |
/// | -------- | ------------ | -------------- |
/// | [`ShapeAttrs::default`] | `None` | `None` |
/// | [`ShapeAttrs::fill_default`] | `Some(Color::default())` | `None` |
/// | [`ShapeAttrs::stroke_default`] | `None` | `Some(Color::default())` |
pub struct ShapeAttrs {
    /// Fill color
    pub fill_color: Option<Color>,
    /// Fill opacity
    pub fill_opacity: f32,
    /// Stroke color
    pub stroke_color: Option<Color>,
    /// Stroke width
    pub stroke_width: f32,
    /// Stroke opacity
    pub stroke_opacity: f32,
}

impl Default for ShapeAttrs {
    /// All quantitative values are set to `1.` and colors are set to `None`.
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
    /// All quantitative values are set to `1.`, `fill_color` is `Some(Color::default())` (white
    /// color) and `stroke_color` is `None`.
    pub fn fill_default() -> Self {
        Self {
            fill_color: Some(Color::default()),
            fill_opacity: 1.,
            stroke_color: None,
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }

    /// All quantitative values are set to `1.`, `stroke_color` is `Some(Color::default())` (white
    /// color) and `fill_color` is `None`.
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
    /// Stroke color
    pub stroke_color: Color,
    /// Stroke width
    pub stroke_width: f32,
    /// Stroke opacity
    pub stroke_opacity: f32,
}

impl Default for LineAttrs {
    /// All quantitative values are set to `1.` and stroke color is white.
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
    /// Text content
    pub content: String,
    /// Fill color
    pub fill_color: Color,
    /// Font size
    pub font_size: f32,
    /// X-oriented text alignment
    pub align_x: Alignment,
    /// Y-oriented text alignment
    pub align_y: Alignment,
}

impl Default for TextAttrs {
    /// Centered empty content filled with white color with `font_size` set to `12.`.
    fn default() -> Self {
        Self {
            content: String::new(),
            fill_color: Color::default(),
            font_size: 12.,
            align_x: Alignment::default(),
            align_y: Alignment::default(),
        }
    }
}

/// Arrow attributes
pub struct ArrowAttrs {
    /// Bend angle
    pub bend_angle: f32,
    /// Head angle
    pub head_angle: f32,
    /// Head length
    pub head_length: f32,
    /// Stroke color
    pub stroke_color: Color,
    /// Stroke width
    pub stroke_width: f32,
    /// Stroke opacity
    pub stroke_opacity: f32,
}

impl Default for ArrowAttrs {
    /// Default values are:
    ///
    /// - `bend_angle`: `0.`
    /// - `head_angle`: `60.`
    /// - `head_length`: `8.`
    /// - `stroke_color`: `Color::default()` (white color)
    /// - `stroke_width`: `1.`
    /// - `stroke_opacity`: `1.`
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
