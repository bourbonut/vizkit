use scale::Scale;

fn main() {
    println!(
        "{:?}",
        Scale::linear()
            .domain([0., 10.])
            .range([20., 100.])
            .apply(5.)
    );
}
