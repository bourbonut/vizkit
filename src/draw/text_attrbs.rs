use crate::chromatic::Color;

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

pub struct TextAttrbs<Data> {
    pub(super) formatter: Box<dyn Fn(&Data) -> String>,
    pub(super) color: Box<dyn Fn(&Data) -> Color>,
    pub(super) font_size: f32,
    pub(super) align_x: Alignment,
    pub(super) align_y: Alignment,
}

impl<Data> TextAttrbs<Data> {
    pub fn new<F>(formatter: F) -> Self
    where
        F: Fn(&Data) -> String + 'static,
    {
        Self {
            formatter: Box::new(formatter),
            color: Box::new(|_| Color::default()),
            font_size: 12.,
            align_x: Alignment::Center,
            align_y: Alignment::Center,
        }
    }

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
