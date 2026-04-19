use crate::chromatic::Color;

pub struct LineAttrs<Data> {
    pub(super) color: Box<dyn Fn(&Data) -> Color>,
    pub(super) width: Box<dyn Fn(&Data) -> f32>,
    pub(super) opacity: Box<dyn Fn(&Data) -> f32>,
}

impl<Data> Default for LineAttrs<Data> {
    fn default() -> Self {
        Self {
            color: Box::new(|_| Color::default()),
            width: Box::new(|_| 1.),
            opacity: Box::new(|_| 1.),
        }
    }
}

impl<Data> LineAttrs<Data> {
    pub fn color(mut self, color: Color) -> Self {
        self.color = Box::new(move |_| color);
        self
    }

    pub fn color_with<F>(mut self, color_fn: F) -> Self
    where
        F: Fn(&Data) -> Color + 'static,
    {
        self.color = Box::new(color_fn);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Box::new(move |_| width);
        self
    }

    pub fn width_with<F>(mut self, width_fn: F) -> Self
    where
        F: Fn(&Data) -> f32 + 'static,
    {
        self.width = Box::new(width_fn);
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Box::new(move |_| opacity);
        self
    }

    pub fn opacity_with<F>(mut self, opacity_fn: F) -> Self
    where
        F: Fn(&Data) -> f32 + 'static,
    {
        self.opacity = Box::new(opacity_fn);
        self
    }
}
