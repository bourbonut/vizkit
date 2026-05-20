use super::Alignment;
use super::PathCommand;
use crate::chromatic::Color;

/// Line properties used for drawing a line.
#[derive(Clone)]
pub struct LineProperties {
    /// Starting point
    pub start: [f32; 2],
    /// Ending point
    pub end: [f32; 2],
    /// Stroke color
    pub stroke_color: Color,
    /// Stroke width
    pub stroke_width: f32,
    /// Stroke opacity
    pub stroke_opacity: f32,
}

impl Default for LineProperties {
    /// Quantitative values are set to `1.`, stroke color is white and starting and ending points
    /// are set to `[0.; 2]`.
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
    pub content: String,
    pub position: [f32; 2],
    pub fill_color: Color,
    pub font_size: f32,
    pub align_x: Alignment,
    pub align_y: Alignment,
}

impl Default for TextProperties {
    /// Centered empty content with origin position, filled with white color and `font_size` set to
    /// `12.`.
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
    pub center: [f32; 2],
    pub radius: f32,
    pub fill_color: Option<Color>,
    pub fill_opacity: f32,
    pub stroke_color: Option<Color>,
    pub stroke_width: f32,
    pub stroke_opacity: f32,
}

impl Default for CircleProperties {
    /// Default circle centered at the origin, with no color and all quantitative values are set to
    /// `1.`.
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
    pub top_left: [f32; 2],
    pub size: [f32; 2],
    pub corner_radius: Option<f32>,
    pub fill_color: Option<Color>,
    pub fill_opacity: f32,
    pub stroke_color: Option<Color>,
    pub stroke_width: f32,
    pub stroke_opacity: f32,
}

impl Default for RectProperties {
    /// Default rectangle centered at the origin, with null size, no corner radius and no color,
    /// where all quantitative values are set to `1.`.
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
    pub path_commands: Vec<PathCommand>,
    pub stroke_color: Color,
    pub stroke_width: f32,
    pub stroke_opacity: f32,
}

impl Default for ArrowProperties {
    /// Arrow with empty path commands, with white stroke and quantitative values set to `1.`.
    fn default() -> Self {
        Self {
            path_commands: Vec::new(),
            stroke_color: Color::default(),
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}
