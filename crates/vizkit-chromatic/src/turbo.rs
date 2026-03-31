use crate::color::Color;

pub fn turbo<T>(t: f32) -> T
where
    Color: Into<T>,
{
    let t = t.clamp(0., 1.);
    let r = (34.61
        + t * (1172.33 - t * (10793.56 - t * (33300.12 - t * (38394.49 - t * 14825.05)))))
        .round()
        .clamp(0., 255.);

    let g = (23.31 + t * (557.33 + t * (1225.33 - t * (3574.96 - t * (1073.77 + t * 707.56)))))
        .round()
        .clamp(0., 255.);

    let b = (27.2 + t * (3211.1 - t * (15327.97 - t * (27814. - t * (22569.18 - t * 6838.66)))))
        .round()
        .clamp(0., 255.);

    Color([r / 255., g / 255., b / 255.]).into()
}

#[cfg(test)]
mod tests {
    use super::turbo;

    #[test]
    fn test_turbo() {
        let step = 100;
        let colors: Vec<[f32; 3]> = (0..=step).map(|i| turbo(i as f32 / step as f32)).collect();
        for color in colors {
            let [r, g, b] = color;
            assert!(0. <= r && r <= 1., "red must be between [0, 1] (sinebow)",);
            assert!(0. <= g && g <= 1., "green must be between [0, 1] (sinebow)",);
            assert!(0. <= b && b <= 1., "blue must be between [0, 1] (sinebow)",);
        }
    }
}
