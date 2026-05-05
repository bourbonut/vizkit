#[derive(Default)]
pub enum Curve {
    #[default]
    Linear,
    Cardinal {
        tension: f32,
    },
}

pub enum PathCommand {
    MoveTo([f32; 2]),
    LineTo([f32; 2]),
    BezierCurveTo([[f32; 2]; 3]),
}

struct PathState {
    state: u8,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Default for PathState {
    fn default() -> Self {
        Self {
            state: 0,
            x0: f32::NAN,
            y0: f32::NAN,
            x1: f32::NAN,
            y1: f32::NAN,
            x2: f32::NAN,
            y2: f32::NAN,
        }
    }
}

impl PathState {
    fn update(&mut self, [x, y]: [f32; 2]) {
        self.x0 = self.x1;
        self.x1 = self.x2;
        self.x2 = x;
        self.y0 = self.y1;
        self.y1 = self.y2;
        self.y2 = y;
    }

    fn bezier_curve_to(&self, tension: f32, [x, y]: [f32; 2]) -> PathCommand {
        let k = (1. - tension) / 6.;
        PathCommand::BezierCurveTo([
            [
                self.x1 + k * (self.x2 - self.x0),
                self.y1 + k * (self.y2 - self.y0),
            ],
            [self.x2 + k * (self.x1 - x), self.y2 + k * (self.y1 - y)],
            [self.x2, self.y2],
        ])
    }
}

pub fn path_iter<Data>(
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    curve: Curve,
) -> impl Iterator<Item = PathCommand> {
    let mut path_state = PathState::default();
    values.iter().filter_map(move |value| {
        let point = [x(value), y(value)];
        match path_state.state {
            0 => {
                path_state.state = 1;
                path_state.update(point);
                Some(PathCommand::MoveTo(point))
            }
            1 => {
                path_state.state = 2;
                match curve {
                    Curve::Linear => Some(PathCommand::LineTo(point)),
                    Curve::Cardinal { tension: _ } => {
                        path_state.x1 = point[0];
                        path_state.y1 = point[1];

                        path_state.update(point);
                        None
                    }
                }
            }
            2 => {
                path_state.state = 3;
                match curve {
                    Curve::Linear => Some(PathCommand::LineTo(point)),
                    Curve::Cardinal { tension } => {
                        let path_command = path_state.bezier_curve_to(tension, point);
                        path_state.update(point);
                        Some(path_command)
                    }
                }
            }
            _ => match curve {
                Curve::Linear => Some(PathCommand::LineTo(point)),
                Curve::Cardinal { tension } => {
                    let path_command = path_state.bezier_curve_to(tension, point);
                    path_state.update(point);
                    Some(path_command)
                }
            },
        }
    })
}
