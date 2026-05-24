use super::Alignment;
use super::PathCommand;
use crate::chromatic::Color;

/// Line properties used for drawing a line.
#[derive(Clone)]
pub struct LineProperties {
    /// Starting point (default: [0.0, 0.0])
    pub start: [f32; 2],
    /// Ending point (default: [0.0, 0.0])
    pub end: [f32; 2],
    /// Stroke color (default: white color)
    pub stroke_color: Color,
    /// Stroke width (default: 1.0)
    pub stroke_width: f32,
    /// Stroke opacity (default: 1.0)
    pub stroke_opacity: f32,
}

impl Default for LineProperties {
    fn default() -> Self {
        Self {
            start: [0.; 2],
            end: [0.; 2],
            stroke_color: Color::default(),
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}

/// Text properties used for drawing some text.
#[derive(Clone)]
pub struct TextProperties {
    /// Text content (default: empty string)
    pub content: String,
    /// Text position (default: [0.0, 0.0])
    pub position: [f32; 2],
    /// Fill color (default: white color)
    pub fill_color: Color,
    /// Font size (default: 12.)
    pub font_size: f32,
    /// X-oriented text alignment (default: center)
    pub align_x: Alignment,
    /// Y-oriented text alignment (default: center)
    pub align_y: Alignment,
}

impl Default for TextProperties {
    fn default() -> Self {
        Self {
            content: String::new(),
            position: [0.; 2],
            fill_color: Color::default(),
            font_size: 12.,
            align_x: Alignment::Center,
            align_y: Alignment::Center,
        }
    }
}

/// Circle properties used for drawing a circle.
#[derive(Clone)]
pub struct CircleProperties {
    /// Circle center position (default: [0.0, 0.0])
    pub center: [f32; 2],
    /// Circle radius (default: 1.)
    pub radius: f32,
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

impl Default for CircleProperties {
    fn default() -> Self {
        Self {
            center: [0.; 2],
            radius: 1.,
            fill_color: None,
            fill_opacity: 1.,
            stroke_color: None,
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}

/// Rectangle properties used for drawing a rectangle.
#[derive(Clone)]
pub struct RectProperties {
    /// Top left corner position (default: [0.0, 0.0])
    pub top_left: [f32; 2],
    /// Rectangle size [width, height] (default: [0.0, 0.0])
    pub size: [f32; 2],
    /// Corner radius (default: None)
    pub corner_radius: Option<f32>,
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

impl Default for RectProperties {
    fn default() -> Self {
        Self {
            top_left: [0.; 2],
            size: [0.; 2],
            corner_radius: None,
            fill_color: None,
            fill_opacity: 1.,
            stroke_color: None,
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}

/// Arrow properties used for drawing an arrow.
#[derive(Clone)]
pub struct ArrowProperties {
    /// Path commands for drawing the arrow (default: empty vector)
    pub path_commands: Vec<PathCommand>,
    /// Stroke color (default: white color)
    pub stroke_color: Color,
    /// Stroke width (default: 1.0)
    pub stroke_width: f32,
    /// Stroke opacity (default: 1.0)
    pub stroke_opacity: f32,
}

impl Default for ArrowProperties {
    fn default() -> Self {
        Self {
            path_commands: Vec::new(),
            stroke_color: Color::default(),
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}
