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

    let [i1, i2, inc] = if power < 0. {
        let inc = 10_f32.powf(-power) / factor as f32;
        let mut i1 = (start * inc).round();
        let mut i2 = (stop * inc).round();
        if i1 / inc < start {
            i1 += 1_f32;
        }
        if i2 / inc > stop {
            i2 -= 1_f32;
        }
        [i1, i2, -inc]
    } else {
        let inc = 10_f32.powf(power) / factor as f32;
        let mut i1 = (start * inc).round();
        let mut i2 = (stop * inc).round();
        if i1 / inc < start {
            i1 += 1_f32;
        }
        if i2 / inc > stop {
            i2 -= 1_f32;
        }
        [i1, i2, inc]
    };
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
        (true, false) => (0..n).map(|i| (i2 - i as f32) / inc).collect(),
        (false, true) => (0..n).map(|i| (i1 + i as f32) / -inc).collect(),
        (false, false) => (0..n).map(|i| (i1 + i as f32) / inc).collect(),
    }
}

pub fn tick_increment(start: f32, stop: f32, count: usize) -> f32 {
    tick_spec(start, stop, count)[2]
}
