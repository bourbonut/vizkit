use crate::chromatic::Color;

pub struct CircleAttrs<Data> {
    pub(crate) fill_color: Box<dyn Fn(&Data) -> Option<Color>>,
    pub(crate) fill_opacity: Box<dyn Fn(&Data) -> f32>,
    pub(crate) stroke_color: Box<dyn Fn(&Data) -> Option<Color>>,
    pub(crate) stroke_width: Box<dyn Fn(&Data) -> f32>,
    pub(crate) stroke_opacity: Box<dyn Fn(&Data) -> f32>,
}

impl<Data> CircleAttrs<Data> {
    pub fn fill_default() -> Self {
        Self {
            fill_color: Box::new(|_| Some(Color::default())),
            fill_opacity: Box::new(|_| 1.),
            stroke_color: Box::new(|_| None),
            stroke_width: Box::new(|_| 1.),
            stroke_opacity: Box::new(|_| 1.),
        }
    }

    pub fn stroke_default() -> Self {
        Self {
            fill_color: Box::new(|_| None),
            fill_opacity: Box::new(|_| 1.),
            stroke_color: Box::new(|_| Some(Color::default())),
            stroke_width: Box::new(|_| 1.),
            stroke_opacity: Box::new(|_| 1.),
        }
    }

    pub fn fill_color(mut self, color: Option<Color>) -> Self {
        self.fill_color = Box::new(move |_| color);
        self
    }

    pub fn fill_color_with<F>(mut self, color_fn: F) -> Self
    where
        F: Fn(&Data) -> Option<Color> + 'static,
    {
        self.fill_color = Box::new(color_fn);
        self
    }

    pub fn fill_opacity(mut self, opacity: f32) -> Self {
        self.fill_opacity = Box::new(move |_| opacity);
        self
    }

    pub fn fill_opacity_with<F>(mut self, opacity_fn: F) -> Self
    where
        F: Fn(&Data) -> f32 + 'static,
    {
        self.fill_opacity = Box::new(opacity_fn);
        self
    }

    pub fn stroke_color(mut self, color: Option<Color>) -> Self {
        self.stroke_color = Box::new(move |_| color);
        self
    }

    pub fn stroke_color_with<F>(mut self, color_fn: F) -> Self
    where
        F: Fn(&Data) -> Option<Color> + 'static,
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
