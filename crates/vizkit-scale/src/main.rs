use vizkit_chromatic::{Viridis, ViridisSpace};
use vizkit_scale::ScaleColor;

fn main() {
    println!(
        "{:?}",
        ScaleColor::linear(Viridis::new(&ViridisSpace::Plasma))
            .domain([0., 10.])
            // .range([20., 100.])
            .apply::<String>(5.)
    );
}
