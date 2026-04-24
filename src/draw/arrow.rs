use super::{ArrowAttrs, ArrowProperties};

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

    ArrowProperties {
        points: [[x1, y1], [xi, yi], [x2, y2], [x3, y3], [x4, y4]],
        radius: r,
        stroke_color: arrow_values.stroke_color,
        stroke_width: arrow_values.stroke_width,
        stroke_opacity: arrow_values.stroke_opacity,
    }
}

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

pub fn vector_iter<Data>(
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
    rotate: impl Fn(&Data) -> f32,
    arrow_attrs: impl Fn(&Data) -> ArrowAttrs,
) -> impl Iterator<Item = ArrowProperties> {
    values.iter().map(move |value| {
        let arrow_values = arrow_attrs(value);
        let head_length = arrow_values.head_length;
        let angle = rotate(value);
        let cos = angle.cos();
        let sin = angle.sin();

        let x = x(value);
        let y = y(value);
        let half = head_length * 0.5;

        arrow_builder(
            half * cos + x,
            -half * sin + y,
            -half * cos + x,
            half * sin + y,
            arrow_values,
        )
    })
}
