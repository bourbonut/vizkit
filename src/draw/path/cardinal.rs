use std::collections::VecDeque;

use super::{PathCommand, PathCurve};

pub(super) struct Cardinal {
    k: f32,
    state: u8,
    is_closed: bool,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Cardinal {
    pub(super) fn new(tension: f32, is_closed: bool) -> Self {
        Self {
            k: (1. - tension.clamp(0., 1.)) / 6.,
            state: 0,
            is_closed,
            x0: f32::NAN,
            y0: f32::NAN,
            x1: f32::NAN,
            y1: f32::NAN,
            x2: f32::NAN,
            y2: f32::NAN,
        }
    }

    fn bezier_curve_to(&self, [x, y]: [f32; 2]) -> PathCommand {
        PathCommand::BezierCurveTo([
            [
                self.x1 + self.k * (self.x2 - self.x0),
                self.y1 + self.k * (self.y2 - self.y0),
            ],
            [
                self.x2 + self.k * (self.x1 - x),
                self.y2 + self.k * (self.y1 - y),
            ],
            [self.x2, self.y2],
        ])
    }

    fn update(&mut self, [x, y]: [f32; 2]) {
        self.x0 = self.x1;
        self.x1 = self.x2;
        self.x2 = x;
        self.y0 = self.y1;
        self.y1 = self.y2;
        self.y2 = y;
    }
}

impl PathCurve for Cardinal {
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
            1 => {
                self.state = 2;
                self.x1 = point[0];
                self.y1 = point[1];
            }
            2 => {
                self.state = 3;
                buffer.push_back(self.bezier_curve_to(point));
            }
            _ => {
                buffer.push_back(self.bezier_curve_to(point));
            }
        }
        self.update(point);
    }

    fn end(&mut self, buffer: &mut VecDeque<PathCommand>) {
        match self.state {
            2 => buffer.push_back(PathCommand::LineTo([self.x2, self.y2])),
            3 => buffer.push_back(self.bezier_curve_to([self.x1, self.y1])),
            _ => (),
        }
        if self.is_closed {
            buffer.push_back(PathCommand::ClosePath);
        }
        self.state = 0;
    }
}
