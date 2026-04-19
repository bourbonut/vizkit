use crate::chromatic::Color;

pub struct CircleAttrs {
    pub fill_color: Option<Color>,
    pub fill_opacity: f32,
    pub stroke_color: Option<Color>,
    pub stroke_width: f32,
    pub stroke_opacity: f32,
}

impl Default for CircleAttrs {
    fn default() -> Self {
        Self {
            fill_color: None,
            fill_opacity: 1.,
            stroke_color: None,
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}

impl CircleAttrs {
    pub fn fill_default() -> Self {
        Self {
            fill_color: Some(Color::default()),
            fill_opacity: 1.,
            stroke_color: None,
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }

    pub fn stroke_default() -> Self {
        Self {
            fill_color: None,
            fill_opacity: 1.,
            stroke_color: Some(Color::default()),
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}

pub struct LineAttrs {
    pub stroke_color: Color,
    pub stroke_width: f32,
    pub stroke_opacity: f32,
}

impl Default for LineAttrs {
    fn default() -> Self {
        Self {
            stroke_color: Color::default(),
            stroke_width: 1.,
            stroke_opacity: 1.,
        }
    }
}

#[derive(Default, Clone)]
pub enum Alignment {
    Start,
    #[default]
    Center,
    End,
}

pub struct TextAttrs {
    pub content: String,
    pub fill_color: Color,
    pub font_size: f32,
    pub align_x: Alignment,
    pub align_y: Alignment,
}

impl Default for TextAttrs {
    fn default() -> Self {
        Self {
            content: String::new(),
            fill_color: Color::default(),
            font_size: 12.,
            align_x: Alignment::default(),
            align_y: Alignment::default(),
        }
    }
}
