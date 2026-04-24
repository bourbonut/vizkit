use super::Alignment;
use crate::chromatic::Color;

#[derive(Clone)]
pub struct LineProperties {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub stroke_color: Color,
    pub stroke_width: f32,
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

#[derive(Clone)]
pub struct ArrowProperties {
    pub points: [[f32; 2]; 5],
    pub radius: f32,
    pub stroke_color: Color,
    pub stroke_width: f32,
    pub stroke_opacity: f32,
}

impl Default for ArrowProperties {
    fn default() -> Self {
        Self {
            points: [[0.; 2]; 5],
            radius: 1.,
            stroke_color: Color::default(),
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}
