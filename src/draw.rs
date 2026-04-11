mod grid;
mod line;

use crate::chromatic::Color;

pub use self::grid::{Direction, Grid};
pub use self::line::Line;

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
}

pub trait Draw {
    fn line(&mut self, line: LineProperties);
    fn text(&mut self, text: TextProperties);
}
