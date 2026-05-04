pub enum PathCommand {
    MoveTo([f32; 2]),
    LineTo([f32; 2]),
}

pub fn path_iter<Data>(
    values: &[Data],
    x: impl Fn(&Data) -> f32,
    y: impl Fn(&Data) -> f32,
) -> impl Iterator<Item = PathCommand> {
    let mut first_point = true;
    values.iter().map(move |value| {
        if first_point {
            first_point = false;
            return PathCommand::MoveTo([x(value), y(value)]);
        } else {
            return PathCommand::LineTo([x(value), y(value)]);
        }
    })
}
