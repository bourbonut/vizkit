use scale::ScaleContinuous;

fn main() {
    println!(
        "{:?}",
        ScaleContinuous::linear()
            .domain([0., 10.])
            .range([20., 100.])
            .apply(5.)
    );
}
