use super::{ArrowAttrs, ArrowProperties, PathCommand};

fn arrow_builder(x1: f32, y1: f32, x2: f32, y2: f32, arrow_values: ArrowAttrs) -> ArrowProperties {
    let line_length = (x2 - x1).hypot(y2 - y1);
    let line_angle = (y2 - y1).atan2(x2 - x1);

    let bend_angle = arrow_values.bend_angle.to_radians();
    let r = line_length.hypot(line_length / bend_angle.tan()) * 0.5;

    let wing_angle = arrow_values.head_angle.to_radians() * 0.5;
    let wing_scale = arrow_values.head_length / 1.5;

    let head_length = wing_scale.min(line_length / 3.);

    let end_angle = line_angle + bend_angle;
    let left_angle = end_angle + wing_angle;
    let right_angle = end_angle - wing_angle;

    let x3 = x2 - head_length * left_angle.cos();
    let y3 = y2 - head_length * left_angle.sin();
    let x4 = x2 - head_length * right_angle.cos();
    let y4 = y2 - head_length * right_angle.sin();

    // According to `https://html.spec.whatwg.org/multipage/canvas.html#building-paths:dom-context-2d-arcto`
    // I had to compute a third control point which is used for the tangents of the radius circle in order
    // to draw an arc. Then after some maths, I ended up with the following results.
    //
    // Known information:
    // r: arc radius
    // A = (x1, y1)
    // B = (x2, y2)
    //
    // ||AB|| = 2 * m => m = 0.5 ||AB||
    //  median_line_(AB) = t * AB_perp / ||AB|| + I where I = (A + B) / 2
    //  where AB_perp is the perpendicular vector of AB
    //
    //  With intercept theorem, we have t / m = m / z where r^2 = z^2 + m^2
    //  => t = m^2 / sqrt(r^2 - z^2)
    let m = -bend_angle.signum() * (x2 - x1).hypot(y2 - y1) * 0.5;
    let s = (r * r - m * m).sqrt();
    let xi = 0.5 * (m / s * (y1 - y2) + x1 + x2);
    let yi = 0.5 * (m / s * (x2 - x1) + y1 + y2);

    let path_commands = vec![
        PathCommand::MoveTo([x1, y1]),
        if r < 1e5 {
            PathCommand::ArcTo(([xi, yi], [x2, y2], r))
        } else {
            PathCommand::LineTo([x2, y2])
        },
        PathCommand::LineTo([x2, y2]),
        PathCommand::MoveTo([x3, y3]),
        PathCommand::LineTo([x2, y2]),
        PathCommand::LineTo([x4, y4]),
    ];

    ArrowProperties {
        path_commands,
        stroke_color: arrow_values.stroke_color,
        stroke_width: arrow_values.stroke_width,
        stroke_opacity: arrow_values.stroke_opacity,
    }
}

/// Creates an iterator of [`ArrowProperties`] for drawing arrows by indicating the starting point
/// and the ending point of each arrow.
///
/// Arrows start from the point `[x1, y1]` and end at the point `[x2, y2]`. Other shape options such
/// as the head size or the bend angle are defined in [`ArrowAttrs`].
///
/// # Example
///
/// ```
/// use vizkit::{
///     draw::{ArrowAttrs, ArrowProperties, arrow_iter},
///     scale::ScaleContinuous,
/// };
///
/// let x = ScaleContinuous::linear().domain([-100., 100.]);
/// let y = ScaleContinuous::linear().domain([-100., 100.]);
///
/// let data = [
///     [0., 15., 40., 20.],
///     [40., 20., 30., -10.],
/// ];
///
/// let arrows: Vec<ArrowProperties> = arrow_iter(
///     &data,
///     |d| x.apply(d[0]),
///     |d| x.apply(d[1]),
///     |d| x.apply(d[2]),
///     |d| x.apply(d[3]),
///     |d| ArrowAttrs::default(),
/// ).collect();
/// ```
pub fn arrow_iter<Data>(
    values: &[Data],
    x1: impl Fn(&Data) -> f32,
    y1: impl Fn(&Data) -> f32,
    x2: impl Fn(&Data) -> f32,
    y2: impl Fn(&Data) -> f32,
    arrow_attrs: impl Fn(&Data) -> ArrowAttrs,
) -> impl Iterator<Item = ArrowProperties> {
    values.iter().map(move |value| {
        arrow_builder(
            x1(value),
            y1(value),
            x2(value),
            y2(value),
            arrow_attrs(value),
        )
    })
}

/// Creates an iterator of [`ArrowProperties`] for drawing arrows by indicating the position and the
/// length and the rotation of each arrow.
///
/// Vectors start from the point `[x, y]`, have a size of `length` and are rotated given the
/// computed angle in radians from the `rotate` function. Other shape options such as the head size
/// or the bend angle are defined in [`ArrowAttrs`].
///
/// # Example
///
/// ```
/// use vizkit::{
///     draw::{ArrowAttrs, ArrowProperties, vector_iter},
///     scale::ScaleContinuous,
/// };
///
/// let x = ScaleContinuous::linear().domain([-100., 100.]);
/// let y = ScaleContinuous::linear().domain([-100., 100.]);
///
/// let data = [[0., 15.], [40., 20.]];
///
/// let vectors: Vec<ArrowProperties> = vector_iter(
///     &data,
///     |d| x.apply(d[0]),
///     |d| x.apply(d[1]),
///     |_| 5.,
///     |d| d[1].atan2(d[0]),
///     |d| ArrowAttrs::default(),
/// ).collect();
/// ```
pub fn vector_iter<Data>(
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    length: impl Fn(&Data) -> f32,
    rotate: impl Fn(&Data) -> f32,
    arrow_attrs: impl Fn(&Data) -> ArrowAttrs,
) -> impl Iterator<Item = ArrowProperties> {
    values.iter().map(move |value| {
        let line_length = length(value);
        let angle = rotate(value);
        let cos = angle.cos();
        let sin = angle.sin();

        let x = x(value);
        let y = y(value);
        let half = line_length * 0.5;

        arrow_builder(
            half * cos + x,
            -half * sin + y,
            -half * cos + x,
            half * sin + y,
            arrow_attrs(value),
        )
    })
}
