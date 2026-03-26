pub trait Tick {
    fn ticks(&self, domain: &[f32; 2], count: usize) -> Vec<f32>;
    // fn tick_format(&self, count: Option<usize>, specifier: Option<&str>) -> TickFormatter;
    fn nice(&self, domain: &[f32; 2], count: usize) -> [f32; 2];
}

fn tick_spec(start: f32, stop: f32, count: usize) -> [f32; 3] {
    let step = (stop - start) / 1_f32.max(count as f32);
    let power = step.log10().floor();
    let error = step / 10_f32.powf(power);
    let factor = if error >= 50_f32.sqrt() {
        10
    } else if error >= 10_f32.sqrt() {
        5
    } else if error >= 2_f32.sqrt() {
        2
    } else {
        1
    };

    let mut inc;
    let mut i1;
    let mut i2;
    if power < 0. {
        inc = 10_f32.powf(-power) / factor as f32;
        i1 = (start * inc).round();
        i2 = (stop * inc).round();
        if i1 / inc < start {
            i1 += 1.;
        }
        if i2 / inc > stop {
            i2 -= 1.;
        }
        inc = -inc;
    } else {
        inc = 10_f32.powf(power) * factor as f32;
        i1 = (start / inc).round();
        i2 = (stop / inc).round();
        if i1 * inc < start {
            i1 += 1.;
        }
        if i2 * inc > stop {
            i2 -= 1.;
        }
    }
    if i2 < i1 && count < 2 {
        return tick_spec(start, stop, count * 2);
    }
    [i1, i2, inc]
}

pub fn ticks(start: f32, stop: f32, count: usize) -> Vec<f32> {
    if count == 0 {
        return Vec::new();
    }
    if start == stop {
        return vec![start];
    }
    let reverse = stop < start;
    let [i1, i2, inc] = if reverse {
        tick_spec(stop, start, count)
    } else {
        tick_spec(start, stop, count)
    };
    if i2 < i1 {
        return Vec::new();
    }
    let n = (i2 - i1 + 1.) as usize;
    match (reverse, inc < 0.) {
        (true, true) => (0..n).map(|i| (i2 - i as f32) / -inc).collect(),
        (true, false) => (0..n).map(|i| (i2 - i as f32) * inc).collect(),
        (false, true) => (0..n).map(|i| (i1 + i as f32) / -inc).collect(),
        (false, false) => (0..n).map(|i| (i1 + i as f32) * inc).collect(),
    }
}

pub fn tick_increment(start: f32, stop: f32, count: usize) -> f32 {
    tick_spec(start, stop, count)[2]
}

mod tests {

    #[rustfmt::skip]
    #[test]
    fn test_ticks_1() {
        assert_eq!(super::ticks(0., 2.2, 3), [0., 1., 2.]);
        assert_eq!(super::ticks(0., 1., 10), [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]);
        assert_eq!(super::ticks(0., 1., 9), [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]);
        assert_eq!(super::ticks(0., 1., 8), [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]);
        assert_eq!(super::ticks(0., 1., 7), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(super::ticks(0., 1., 6), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(super::ticks(0., 1., 5), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(super::ticks(0., 1., 4), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(super::ticks(0., 1., 3), [0.0, 0.5, 1.0]);
        assert_eq!(super::ticks(0., 1., 2), [0.0, 0.5, 1.0]);
        assert_eq!(super::ticks(0., 1., 1), [0.0, 1.0]);
        assert_eq!(super::ticks(0., 10., 10), [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);
        assert_eq!(super::ticks(0., 10., 9), [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);
        assert_eq!(super::ticks(0., 10., 8), [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);
        assert_eq!(super::ticks(0., 10., 7), [0., 2., 4., 6., 8., 10.]);
        assert_eq!(super::ticks(0., 10., 6), [0., 2., 4., 6., 8., 10.]);
        assert_eq!(super::ticks(0., 10., 5), [0., 2., 4., 6., 8., 10.]);
        assert_eq!(super::ticks(0., 10., 4), [0., 2., 4., 6., 8., 10.]);
        assert_eq!(super::ticks(0., 10., 3), [0., 5., 10.]);
        assert_eq!(super::ticks(0., 10., 2), [0., 5., 10.]);
        assert_eq!(super::ticks(0., 10., 1), [0., 10.]);
        assert_eq!(super::ticks(-10., 10., 10), [-10., -8., -6., -4., -2., 0., 2., 4., 6., 8., 10.]);
        assert_eq!(super::ticks(-10., 10., 9), [-10., -8., -6., -4., -2., 0., 2., 4., 6., 8., 10.]);
        assert_eq!(super::ticks(-10., 10., 8), [-10., -8., -6., -4., -2., 0., 2., 4., 6., 8., 10.]);
        assert_eq!(super::ticks(-10., 10., 7), [-10., -8., -6., -4., -2., 0., 2., 4., 6., 8., 10.]);
        assert_eq!(super::ticks(-10., 10., 6), [-10., -5., 0., 5., 10.]);
        assert_eq!(super::ticks(-10., 10., 5), [-10., -5., 0., 5., 10.]);
        assert_eq!(super::ticks(-10., 10., 4), [-10., -5., 0., 5., 10.]);
        assert_eq!(super::ticks(-10., 10., 3), [-10., -5., 0., 5., 10.]);
        assert_eq!(super::ticks(-10., 10., 2), [-10., 0., 10.]);
        assert_eq!(super::ticks(-10., 10., 1), [0.]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_ticks_2() {
        assert_eq!(super::ticks(1., 0., 10), super::ticks(0., 1., 10).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(1., 0., 9), super::ticks(0., 1., 9).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(1., 0., 8), super::ticks(0., 1., 8).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(1., 0., 7), super::ticks(0., 1., 7).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(1., 0., 6), super::ticks(0., 1., 6).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(1., 0., 5), super::ticks(0., 1., 5).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(1., 0., 4), super::ticks(0., 1., 4).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(1., 0., 3), super::ticks(0., 1., 3).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(1., 0., 2), super::ticks(0., 1., 2).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(1., 0., 1), super::ticks(0., 1., 1).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 10), super::ticks(0., 10., 10).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 9), super::ticks(0., 10., 9).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 8), super::ticks(0., 10., 8).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 7), super::ticks(0., 10., 7).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 6), super::ticks(0., 10., 6).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 5), super::ticks(0., 10., 5).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 4), super::ticks(0., 10., 4).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 3), super::ticks(0., 10., 3).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 2), super::ticks(0., 10., 2).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., 0., 1), super::ticks(0., 10., 1).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 10), super::ticks(-10., 10., 10).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 9), super::ticks(-10., 10., 9).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 8), super::ticks(-10., 10., 8).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 7), super::ticks(-10., 10., 7).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 6), super::ticks(-10., 10., 6).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 5), super::ticks(-10., 10., 5).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 4), super::ticks(-10., 10., 4).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 3), super::ticks(-10., 10., 3).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 2), super::ticks(-10., 10., 2).into_iter().rev().collect::<Vec<f32>>());
        assert_eq!(super::ticks(10., -10., 1), super::ticks(-10., 10., 1).into_iter().rev().collect::<Vec<f32>>());
    }

    #[rustfmt::skip]
    #[test]
    fn test_ticks_3() {
        assert_eq!(super::ticks(0.98, 1.14, 10), [0.98, 1., 1.02, 1.04, 1.06, 1.08, 1.1, 1.12, 1.14]);
        assert_eq!(super::ticks(1., 364., 1), [200.]);
        assert_eq!(super::ticks(1., 499., 1), [200., 400.]);
        assert_eq!(super::ticks(364., 1., 1), [200.]);
        assert_eq!(super::ticks(0.001, 0.364, 1), [0.2]);
        assert_eq!(super::ticks(0.364, 0.001, 1), [0.2]);
        assert_eq!(super::ticks(-1., -364., 1), [-200.]);
        assert_eq!(super::ticks(-364., -1., 1), [-200.]);
        assert_eq!(super::ticks(-0.001, -0.364, 1), [-0.2]);
        assert_eq!(super::ticks(-0.364, -0.001, 1), [-0.2]);
    }
}
