use crate::chromatic::Color;

pub struct LineAttrs<Data> {
    pub(super) stroke_color: Box<dyn Fn(&Data) -> Color>,
    pub(super) stroke_width: Box<dyn Fn(&Data) -> f32>,
    pub(super) stroke_opacity: Box<dyn Fn(&Data) -> f32>,
}

impl<Data> Default for LineAttrs<Data> {
    fn default() -> Self {
        Self {
            stroke_color: Box::new(|_| Color::default()),
            stroke_width: Box::new(|_| 1.),
            stroke_opacity: Box::new(|_| 1.),
        }
    }
}

impl<Data> LineAttrs<Data> {
    pub fn stroke_color(mut self, color: Color) -> Self {
        self.stroke_color = Box::new(move |_| color);
        self
    }

    pub fn stroke_color_with<F>(mut self, color_fn: F) -> Self
    where
        F: Fn(&Data) -> Color + 'static,
    {
        self.stroke_color = Box::new(color_fn);
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = Box::new(move |_| width);
        self
    }

    pub fn stroke_width_with<F>(mut self, width_fn: F) -> Self
    where
        F: Fn(&Data) -> f32 + 'static,
    {
        self.stroke_width = Box::new(width_fn);
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.stroke_opacity = Box::new(move |_| opacity);
        self
    }

    pub fn stroke_opacity_with<F>(mut self, opacity_fn: F) -> Self
    where
        F: Fn(&Data) -> f32 + 'static,
    {
        self.stroke_opacity = Box::new(opacity_fn);
        self
    }
}
