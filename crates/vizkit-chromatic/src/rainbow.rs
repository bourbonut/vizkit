use crate::color::{Color, Cubehelix};

pub fn rainbow<T>(t: f32) -> T
where
    Color: Into<T>,
{
    let mut t = t;
    if t < 0. || t > 1. {
        t -= t.floor();
    }
    let ts = (t - 0.5).abs();
    Color::from(Cubehelix([360. * t - 100., 1.5 - 1.5 * ts, 0.8 - 0.9 * ts])).into()
}

#[cfg(test)]
mod tests {
    use super::rainbow;

    #[test]
    fn test_rainbow() {
        let step = 100;
        let _: Vec<String> = (0..=step)
            .map(|i| rainbow(i as f32 / step as f32))
            .collect();
    }
}
