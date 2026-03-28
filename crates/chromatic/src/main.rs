use chromatic::rainbow;

fn main() {
    let step = 100;
    for i in 0..=step {
        let t = i as f32 / step as f32;
        let color: String = rainbow(t);
        println!("{:?} - {:?}", t, color)
    }
}
