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

struct PathIterator<I>
where
    I: Iterator<Item = [f32; 2]>,
{
    values: I,
    curve: Curve,
    state: u8,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    last_point: bool,
}

impl<I> PathIterator<I>
where
    I: Iterator<Item = [f32; 2]>,
{
    fn new(values: I, curve: Curve) -> Self {
        Self {
            values,
            curve,
            state: 0,
            x0: f32::NAN,
            y0: f32::NAN,
            x1: f32::NAN,
            y1: f32::NAN,
            x2: f32::NAN,
            y2: f32::NAN,
            last_point: false,
        }
    }

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

impl<I> Iterator for PathIterator<I>
where
    I: Iterator<Item = [f32; 2]>,
{
    type Item = PathCommand;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.values.next() {
            match self.state {
                0 => {
                    self.state = 1;
                    self.update(point);
                    Some(PathCommand::MoveTo(point))
                }
                1 => {
                    self.state = 2;
                    match self.curve {
                        Curve::Linear => Some(PathCommand::LineTo(point)),
                        Curve::Cardinal { tension: _ } => {
                            self.x1 = point[0];
                            self.y1 = point[1];

                            self.update(point);
                            self.next()
                        }
                    }
                }
                2 => {
                    self.state = 3;
                    match self.curve {
                        Curve::Linear => Some(PathCommand::LineTo(point)),
                        Curve::Cardinal { tension } => {
                            let path_command = self.bezier_curve_to(tension, point);
                            self.update(point);
                            Some(path_command)
                        }
                    }
                }
                _ => match self.curve {
                    Curve::Linear => Some(PathCommand::LineTo(point)),
                    Curve::Cardinal { tension } => {
                        let path_command = self.bezier_curve_to(tension, point);
                        self.update(point);
                        Some(path_command)
                    }
                },
            }
        } else if self.last_point {
            None
        } else {
            match self.curve {
                Curve::Linear => None,
                Curve::Cardinal { tension } => match self.state {
                    2 => {
                        self.last_point = true;
                        Some(PathCommand::LineTo([self.x2, self.y2]))
                    }
                    3 => {
                        self.last_point = true;
                        Some(self.bezier_curve_to(tension, [self.x1, self.y1]))
                    }
                    _ => None,
                },
            }
        }
    }
}

pub fn path_iter<'a, Data>(
    values: &'a [Data],
    x: impl Fn(&Data) -> f32 + 'a,
    y: impl Fn(&Data) -> f32 + 'a,
    curve: Curve,
) -> impl Iterator<Item = PathCommand> + 'a {
    let points = values.iter().map(move |value| [x(value), y(value)]);
    PathIterator::new(points, curve)
}
