use std::collections::VecDeque;

use super::{PathCommand, PathCurve};

pub(super) struct Step {
    state: u8,
    is_closed: bool,
    tension: f32,
    x: f32,
    y: f32,
}

impl Step {
    pub(super) fn new(tension: f32, is_closed: bool) -> Self {
        Self {
            tension: tension.clamp(0., 1.),
            state: 0,
            is_closed,
            x: f32::NAN,
            y: f32::NAN,
        }
    }

    fn update(&mut self, [x, y]: [f32; 2]) {
        self.x = x;
        self.y = y;
    }
}

impl PathCurve for Step {
    fn point(&mut self, point: [f32; 2], buffer: &mut VecDeque<PathCommand>) {
        match self.state {
            0 => {
                self.state = 1;
                if self.is_closed {
                    buffer.push_back(PathCommand::LineTo(point))
                } else {
                    buffer.push_back(PathCommand::MoveTo(point))
                }
            }
            _ => {
                self.state = 2;
                let [x, y] = point;
                if self.tension <= 0. {
                    buffer.push_back(PathCommand::LineTo([self.x, y]));
                    buffer.push_back(PathCommand::LineTo(point))
                } else {
                    let x1 = self.x * (1. - self.tension) + x * self.tension;
                    buffer.push_back(PathCommand::LineTo([x1, self.y]));
                    buffer.push_back(PathCommand::LineTo([x1, y]));
                }
            }
        }
        self.update(point);
    }

    fn end(&mut self, buffer: &mut VecDeque<PathCommand>) {
        if self.state == 2 {
            buffer.push_back(PathCommand::LineTo([self.x, self.y]))
        }
        if self.is_closed {
            buffer.push_back(PathCommand::ClosePath);
        }
        self.tension = 1. - self.tension;
        self.state = 0;
    }
}
