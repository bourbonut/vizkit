mod axis;
mod grid;
mod line;
mod line_attrs;
mod text;
mod text_attrs;

use crate::chromatic::Color;

pub use self::axis::Axis;
pub use self::grid::Grid;
pub use self::line::Line;
pub use self::line_attrs::LineAttrs;
pub use self::text::Text;
pub use self::text_attrs::{Alignment, TextAttrs};

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

pub trait Draw {
    fn line(&mut self, line: LineProperties);
    fn text(&mut self, text: TextProperties);
}

pub trait Direction {
    fn direction(coord1: f32, coord2: f32) -> [f32; 2];
}

pub struct Vertical;
pub struct Horizontal;

impl Direction for Vertical {
    fn direction(coord1: f32, coord2: f32) -> [f32; 2] {
        [coord1, coord2]
    }
}

impl Direction for Horizontal {
    fn direction(coord1: f32, coord2: f32) -> [f32; 2] {
        [coord2, coord1]
    }
}

enum Orientation {
    Flip,
    Same,
}

impl Orientation {
    fn apply(&self, x: f32, y: f32) -> [f32; 2] {
        match self {
            Orientation::Same => [x, y],
            Orientation::Flip => [y, x],
        }
    }
}
