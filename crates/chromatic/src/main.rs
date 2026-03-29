use chromatic::{ColorMap, Rainbow};

fn main() {
    let step = 100;
    let rainbow = Rainbow::default();
    for i in 0..=step {
        let t = i as f32 / step as f32;
        let color: String = rainbow.interpolate(t);
        println!("{:?} - {:?}", t, color)
    }
}
