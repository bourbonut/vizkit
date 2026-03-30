use std::f32;

use crate::color::Color;

pub fn sinebow<T>(t: f32) -> T
where
    Color: Into<T>,
{
    let pi = f32::consts::PI;
    let t = (0.5 - t) * pi;

    let x = t.sin();
    let r = x * x;

    let x = (t + pi / 3.).sin();
    let g = x * x;

    let x = (t + pi * 2. / 3.).sin();
    let b = x * x;

    Color([r, g, b]).into()
}

#[cfg(test)]
mod tests {
    use super::sinebow;

    #[test]
    fn test_sinebow() {
        let step = 100;
        let _: Vec<String> = (0..=step)
            .map(|i| sinebow(i as f32 / step as f32))
            .collect();
    }
}
