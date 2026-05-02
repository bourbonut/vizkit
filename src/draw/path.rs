pub trait Path {
    fn move_to(&mut self, point: [f32; 2]);
    fn line_to(&mut self, point: [f32; 2]);
    fn close_path(&mut self);
}

pub trait Curve {
    fn area_start(&mut self);
    fn area_end(&mut self);
    fn line_start(&mut self);
    fn line_end(&mut self);
    fn point(&mut self, point: [f32; 2]);
}

pub struct Linear<P: Path> {
    context: P,
    line: bool,
    point: u8,
}

impl<P: Path> Linear<P> {
    pub fn new(context: P) -> Self {
        Self {
            context,
            line: false,
            point: 0,
        }
    }
}

impl<P: Path> Curve for Linear<P> {
    fn area_start(&mut self) {
        self.line = false;
    }

    fn area_end(&mut self) {
        self.line = false;
    }

    fn line_start(&mut self) {
        self.point = 0;
    }

    fn line_end(&mut self) {
        if self.line && self.point == 1 {
            self.context.close_path();
        }
        self.line = !self.line;
    }

    fn point(&mut self, point: [f32; 2]) {
        match self.point {
            0 => {
                self.point = 1;
                if self.line {
                    self.context.line_to(point);
                } else {
                    self.context.move_to(point);
                }
            }
            1 => {
                self.point = 2;
                self.context.line_to(point);
            }
            _ => self.context.line_to(point),
        }
    }
}

pub fn path<Data, C: Curve>(
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    curve: &mut C,
) {
    curve.line_start();
    values.iter().for_each(|value| {
        curve.point([x(value), y(value)]);
    });
    curve.line_end();
}
