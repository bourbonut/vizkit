#[derive(Clone)]
pub enum PathCommand {
    MoveTo([f32; 2]),
    LineTo([f32; 2]),
    BezierCurveTo([[f32; 2]; 3]),
    ArcTo(([f32; 2], [f32; 2], f32)),
    ClosePath,
}

pub(crate) trait PathCurve {
    fn point(&mut self, point: [f32; 2]) -> Option<PathCommand>;
    fn end(&mut self) -> Option<PathCommand>;
}

#[derive(Default)]
pub(super) struct Linear {
    state: u8,
}

pub(super) struct Cardinal {
    k: f32,
    state: u8,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl PathCurve for Linear {
    fn point(&mut self, point: [f32; 2]) -> Option<PathCommand> {
        match self.state {
            0 => {
                self.state = 1;
                Some(PathCommand::MoveTo(point))
            }
            1 => {
                self.state = 2;
                Some(PathCommand::LineTo(point))
            }
            _ => Some(PathCommand::LineTo(point)),
        }
    }

    fn end(&mut self) -> Option<PathCommand> {
        self.state = 0;
        None
    }
}

impl Cardinal {
    pub(super) fn new(tension: f32) -> Self {
        Self {
            k: (1. - tension) / 6.,
            state: 0,
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
    fn point(&mut self, point: [f32; 2]) -> Option<PathCommand> {
        match self.state {
            0 => {
                self.state = 1;
                self.update(point);
                Some(PathCommand::MoveTo(point))
            }
            1 => {
                self.state = 2;
                self.x1 = point[0];
                self.y1 = point[1];
                self.update(point);
                None
            }
            2 => {
                self.state = 3;
                let path_command = self.bezier_curve_to(point);
                self.update(point);
                Some(path_command)
            }
            _ => {
                let path_command = self.bezier_curve_to(point);
                self.update(point);
                Some(path_command)
            }
        }
    }

    fn end(&mut self) -> Option<PathCommand> {
        let last_state = self.state;
        self.state = 0;
        match last_state {
            2 => Some(PathCommand::LineTo([self.x2, self.y2])),
            3 => Some(self.bezier_curve_to([self.x1, self.y1])),
            _ => None,
        }
    }
}

#[derive(Default)]
pub enum Curve {
    #[default]
    Linear,
    Cardinal {
        tension: f32,
    },
}

struct PathCurveIterator<I, C>
where
    I: Iterator<Item = [f32; 2]>,
    C: PathCurve,
{
    values: I,
    curve: C,
    is_close: bool,
}

impl<I, C> Iterator for PathCurveIterator<I, C>
where
    I: Iterator<Item = [f32; 2]>,
    C: PathCurve,
{
    type Item = PathCommand;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.values.next() {
            let path_command = self.curve.point(point);
            if path_command.is_none() {
                self.next()
            } else {
                path_command
            }
        } else {
            let path_command = self.curve.end();
            if path_command.is_none() && self.is_close {
                self.is_close = false;
                Some(PathCommand::ClosePath)
            } else {
                path_command
            }
        }
    }
}

enum PathIterator<I>
where
    I: Iterator<Item = [f32; 2]>,
{
    Linear(PathCurveIterator<I, Linear>),
    Cardinal(PathCurveIterator<I, Cardinal>),
}

impl<I> PathIterator<I>
where
    I: Iterator<Item = [f32; 2]>,
{
    fn new(values: I, curve: &Curve, is_closed: bool) -> Self {
        match curve {
            Curve::Linear => Self::Linear(PathCurveIterator {
                values,
                curve: Linear::default(),
                is_close: is_closed,
            }),
            Curve::Cardinal { tension } => Self::Cardinal(PathCurveIterator {
                values,
                curve: Cardinal::new(*tension),
                is_close: is_closed,
            }),
        }
    }
}

impl<I> Iterator for PathIterator<I>
where
    I: Iterator<Item = [f32; 2]>,
{
    type Item = PathCommand;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Linear(it) => it.next(),
            Self::Cardinal(it) => it.next(),
        }
    }
}

pub fn path_iter<'a, Data>(
    values: &'a [Data],
    x: impl Fn(&Data) -> f32 + 'a,
    y: impl Fn(&Data) -> f32 + 'a,
    curve: Curve,
) -> impl Iterator<Item = PathCommand> + 'a {
    PathIterator::new(
        values.iter().map(move |value| [x(value), y(value)]),
        &curve,
        false,
    )
}

pub fn area_iter<'a, Data>(
    values: &'a [Data],
    x0: impl Fn(&Data) -> f32 + 'a,
    y0: impl Fn(&Data) -> f32 + 'a,
    x1: impl Fn(&Data) -> f32 + 'a,
    y1: impl Fn(&Data) -> f32 + 'a,
    curve: Curve,
) -> impl Iterator<Item = PathCommand> + 'a {
    PathIterator::new(
        values.iter().map(move |value| [x1(value), y1(value)]),
        &curve,
        false,
    )
    .chain(PathIterator::new(
        values.iter().map(move |value| [x0(value), y0(value)]).rev(),
        &curve,
        true,
    ))
}
