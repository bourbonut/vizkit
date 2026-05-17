use std::collections::VecDeque;

use super::{PathCommand, PathCurve};

pub(super) struct Linear {
    state: u8,
    is_closed: bool,
}

impl Linear {
    pub(super) fn new(is_closed: bool) -> Self {
        Linear {
            state: 0,
            is_closed,
        }
    }
}

impl PathCurve for Linear {
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
                buffer.push_back(PathCommand::LineTo(point))
            }
            _ => buffer.push_back(PathCommand::LineTo(point)),
        }
    }

    fn end(&mut self, buffer: &mut VecDeque<PathCommand>) {
        if self.is_closed {
            buffer.push_back(PathCommand::ClosePath);
        }
        self.state = 0;
    }
}
