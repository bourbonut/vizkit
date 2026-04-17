use std::marker::PhantomData;

use crate::{
    chromatic::Color,
    generator::{Constant, Function, Generator},
};

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

pub struct TextAttrbs<Data, Formatter, FillColor>
where
    Formatter: Fn(&Data) -> String,
    FillColor: Generator<Data, Output = Color>,
{
    pub(super) formatter: Formatter,
    pub(super) color: FillColor,
    pub(super) font_size: f32,
    pub(super) align_x: Alignment,
    pub(super) align_y: Alignment,
    marker: PhantomData<Data>,
}

impl<Data, Formatter> TextAttrbs<Data, Formatter, Constant<Color>>
where
    Formatter: Fn(&Data) -> String,
{
    pub fn new(formatter: Formatter) -> Self {
        Self {
            formatter,
            color: Constant(Color::default()),
            font_size: 12.,
            align_x: Alignment::Center,
            align_y: Alignment::Center,
            marker: PhantomData,
        }
    }
}

impl<Data, Formatter> From<Formatter> for TextAttrbs<Data, Formatter, Constant<Color>>
where
    Formatter: Fn(&Data) -> String,
{
    fn from(formatter: Formatter) -> Self {
        TextAttrbs::new(formatter)
    }
}

impl<Data, Formatter, FillColor> TextAttrbs<Data, Formatter, FillColor>
where
    Formatter: Fn(&Data) -> String,
    FillColor: Generator<Data, Output = Color>,
{
    pub fn color(self, color: Color) -> TextAttrbs<Data, Formatter, Constant<Color>> {
        TextAttrbs {
            formatter: self.formatter,
            color: Constant(color),
            font_size: self.font_size,
            align_x: self.align_x,
            align_y: self.align_y,
            marker: self.marker,
        }
    }

    pub fn color_with<F>(self, color_fn: F) -> TextAttrbs<Data, Formatter, Function<F, Data, Color>>
    where
        F: Fn(&Data) -> Color,
    {
        TextAttrbs {
            formatter: self.formatter,
            color: Function::new(color_fn),
            font_size: self.font_size,
            align_x: self.align_x,
            align_y: self.align_y,
            marker: self.marker,
        }
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn align_x(mut self, align_x: Alignment) -> Self {
        self.align_x = align_x;
        self
    }

    pub fn align_y(mut self, align_y: Alignment) -> Self {
        self.align_y = align_y;
        self
    }
}
