use super::color::Color;

pub fn cividis<T>(t: f32) -> T
where
    Color: Into<T>,
{
    let t = t.clamp(0., 1.);
    let r = (-4.54 - t * (35.34 - t * (2381.73 - t * (6402.7 - t * (7024.72 - t * 2710.57)))))
        .round()
        .clamp(0., 255.);

    let g = (32.49 + t * (170.73 + t * (52.82 - t * (131.46 - t * (176.58 - t * 67.37)))))
        .round()
        .clamp(0., 255.);

    let b = (32.49 + t * (170.73 + t * (52.82 - t * (131.46 - t * (176.58 - t * 67.37)))))
        .round()
        .clamp(0., 255.);

    Color([r / 255., g / 255., b / 255.]).into()
}

#[cfg(test)]
mod tests {
    use super::cividis;

    #[test]
    fn test_cividis() {
        let step = 100;
        let colors: Vec<[f32; 3]> = (0..=step)
            .map(|i| cividis(i as f32 / step as f32))
            .collect();
        for color in colors {
            let [r, g, b] = color;
            assert!(0. <= r && r <= 1., "red must be between [0, 1] (cividis)",);
            assert!(0. <= g && g <= 1., "green must be between [0, 1] (cividis)",);
            assert!(0. <= b && b <= 1., "blue must be between [0, 1] (cividis)",);
        }
    }
}
