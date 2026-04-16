mod axis;
mod grid;
mod line;
mod text;

use crate::chromatic::Color;

pub use self::axis::Axis;
pub use self::grid::Grid;
pub use self::line::Line;
pub use self::text::{Text1D, Text2D};

#[derive(Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Clone)]
pub struct LineProperties {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub color: Color,
    pub width: f32,
    pub opacity: f32,
}

impl Default for LineProperties {
    fn default() -> Self {
        Self {
            start: [0.; 2],
            end: [0.; 2],
            color: Color::default(),
            width: 1.,
            opacity: 1.,
        }
    }
}

#[derive(Default, Clone)]
pub struct TextProperties {
    pub content: String,
    pub position: [f32; 2],
    pub color: Color,
    pub align_x: Alignment,
    pub align_y: Alignment,
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
