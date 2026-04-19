mod axis;
mod grid;
mod line;
mod line_attrs;
mod text;
mod text_attrs;

use crate::chromatic::Color;
use crate::scale::{ScaleContinuous, Tick, Transformer};

pub use self::axis::{AxisOptions, axis_bottom, axis_left, axis_right, axis_top};
pub use self::grid::{grid_horizontal, grid_vertical};
pub use self::line::Line;
pub use self::line_attrs::LineAttrs;
pub use self::text::text;
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
    fn draw_line(&mut self, line: LineProperties);
    fn draw_text(&mut self, text: TextProperties);

    fn grid_vertical<Data>(
        &mut self,
        values: &[Data],
        y1: f32,
        y2: f32,
        x: impl Fn(&Data) -> f32,
        line_attrs: &LineAttrs<Data>,
    ) {
        grid_vertical(self, values, y1, y2, x, line_attrs);
    }

    fn grid_horizontal<Data>(
        &mut self,
        values: &[Data],
        x1: f32,
        x2: f32,
        y: impl Fn(&Data) -> f32,
        line_attrs: &LineAttrs<Data>,
    ) {
        grid_horizontal(self, values, x1, x2, y, line_attrs);
    }

    fn text<Data>(
        &mut self,
        values: &[Data],
        x: impl Fn(&Data) -> f32,
        y: impl Fn(&Data) -> f32,
        text_attrs: &TextAttrs<Data>,
    ) {
        text(self, values, x, y, text_attrs)
    }

    fn text_vertical<Data>(
        &mut self,
        values: &[Data],
        x: f32,
        y: impl Fn(&Data) -> f32,
        text_attrs: &TextAttrs<Data>,
    ) {
        text(self, values, |_| x, y, text_attrs);
    }

    fn text_horizontal<Data>(
        &mut self,
        values: &[Data],
        x: impl Fn(&Data) -> f32,
        y: f32,
        text_attrs: &TextAttrs<Data>,
    ) {
        text(self, values, x, |_| y, text_attrs);
    }

    fn axis_top<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        y: f32,
        axis_options: &AxisOptions,
        line_attrbs: &LineAttrs<f32>,
        text_attrbs: &TextAttrs<f32>,
    ) {
        axis_top(self, scaler, y, axis_options, line_attrbs, text_attrbs);
    }

    fn axis_right<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        x: f32,
        axis_options: &AxisOptions,
        line_attrbs: &LineAttrs<f32>,
        text_attrbs: &TextAttrs<f32>,
    ) {
        axis_right(self, scaler, x, axis_options, line_attrbs, text_attrbs);
    }

    fn axis_bottom<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        y: f32,
        axis_options: &AxisOptions,
        line_attrbs: &LineAttrs<f32>,
        text_attrbs: &TextAttrs<f32>,
    ) {
        axis_bottom(self, scaler, y, axis_options, line_attrbs, text_attrbs);
    }

    fn axis_left<T: Transformer + Tick>(
        &mut self,
        scaler: &ScaleContinuous<T>,
        x: f32,
        axis_options: &AxisOptions,
        line_attrbs: &LineAttrs<f32>,
        text_attrbs: &TextAttrs<f32>,
    ) {
        axis_left(self, scaler, x, axis_options, line_attrbs, text_attrbs);
    }
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
