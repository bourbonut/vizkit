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
        let colors: Vec<[f32; 3]> = (0..=step)
            .map(|i| rainbow(i as f32 / step as f32))
            .collect();
        for color in colors {
            let [r, g, b] = color;
            assert!(0. <= r && r <= 1., "red must be between [0, 1] (rainbow)",);
            assert!(0. <= g && g <= 1., "green must be between [0, 1] (rainbow)",);
            assert!(0. <= b && b <= 1., "blue must be between [0, 1] (rainbow)",);
        }
    }
}
