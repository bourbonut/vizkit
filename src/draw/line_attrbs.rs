use std::marker::PhantomData;

use crate::{
    chromatic::Color,
    generator::{Constant, Function, Generator},
};

pub struct LineAttrbs<Data, StrokeColor, StrokeWidth, StrokeOpacity>
where
    StrokeColor: Generator<Data, Output = Color>,
    StrokeWidth: Generator<Data, Output = f32>,
    StrokeOpacity: Generator<Data, Output = f32>,
{
    pub(super) color: StrokeColor,
    pub(super) width: StrokeWidth,
    pub(super) opacity: StrokeOpacity,
    marker: PhantomData<Data>,
}

impl<Data> Default for LineAttrbs<Data, Constant<Color>, Constant<f32>, Constant<f32>> {
    fn default() -> Self {
        Self {
            color: Constant(Color::default()),
            width: Constant(1.),
            opacity: Constant(1.),
            marker: PhantomData,
        }
    }
}

impl<Data, StrokeColor, StrokeWidth, StrokeOpacity>
    LineAttrbs<Data, StrokeColor, StrokeWidth, StrokeOpacity>
where
    StrokeColor: Generator<Data, Output = Color>,
    StrokeWidth: Generator<Data, Output = f32>,
    StrokeOpacity: Generator<Data, Output = f32>,
{
    pub fn color(
        self,
        color: Color,
    ) -> LineAttrbs<Data, Constant<Color>, StrokeWidth, StrokeOpacity> {
        LineAttrbs {
            color: Constant(color),
            width: self.width,
            opacity: self.opacity,
            marker: self.marker,
        }
    }

    pub fn color_with<F>(
        self,
        color_fn: F,
    ) -> LineAttrbs<Data, Function<F, Data, Color>, StrokeWidth, StrokeOpacity>
    where
        F: Fn(&Data) -> Color,
    {
        LineAttrbs {
            color: Function::new(color_fn),
            width: self.width,
            opacity: self.opacity,
            marker: self.marker,
        }
    }

    pub fn width(self, width: f32) -> LineAttrbs<Data, StrokeColor, Constant<f32>, StrokeOpacity> {
        LineAttrbs {
            color: self.color,
            width: Constant(width),
            opacity: self.opacity,
            marker: self.marker,
        }
    }

    pub fn width_with<F>(
        self,
        width_fn: F,
    ) -> LineAttrbs<Data, StrokeColor, Function<F, Data, f32>, StrokeOpacity>
    where
        F: Fn(&Data) -> f32,
    {
        LineAttrbs {
            color: self.color,
            width: Function::new(width_fn),
            opacity: self.opacity,
            marker: self.marker,
        }
    }

    pub fn opacity(
        self,
        opacity: f32,
    ) -> LineAttrbs<Data, StrokeColor, StrokeWidth, Constant<f32>> {
        LineAttrbs {
            color: self.color,
            width: self.width,
            opacity: Constant(opacity),
            marker: self.marker,
        }
    }

    pub fn opacity_with<F>(
        self,
        opacity_fn: F,
    ) -> LineAttrbs<Data, StrokeColor, StrokeWidth, Function<F, Data, f32>>
    where
        F: Fn(&Data) -> f32,
    {
        LineAttrbs {
            color: self.color,
            width: self.width,
            opacity: Function::new(opacity_fn),
            marker: self.marker,
        }
    }
}
