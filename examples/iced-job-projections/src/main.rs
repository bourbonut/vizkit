mod data;
use polars::prelude::Column;
use std::error::Error;
use vizkit::scale::{ScaleContinuous, ScaleOrdinal};

use crate::data::load_transform_data;

#[allow(unused)]
struct Margin {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

fn min(column: &Column) -> Result<f32, Box<dyn std::error::Error>> {
    Ok(column.min_reduce()?.into_value().try_extract::<f32>()?)
}

fn max(column: &Column) -> Result<f32, Box<dyn std::error::Error>> {
    Ok(column.max_reduce()?.into_value().try_extract::<f32>()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let df = load_transform_data()?;

    let width = 600.;
    let height = 300.;
    let margin = Margin {
        top: 10.,
        right: 15.,
        bottom: 40.,
        left: 55.,
    };

    let radius = ScaleContinuous::sqrt()
        .domain([min(&df["openings"])?, max(&df["openings"])?])
        .range([2., 20.]);

    let xmax = df["turnover"]
        .max_reduce()?
        .into_value()
        .try_extract::<f32>()?;

    let x = ScaleContinuous::linear()
        .domain([0., max(&df["turnover"])?])
        .range([margin.left, width - margin.right])
        .nice(None);
    let y = ScaleContinuous::linear()
        .domain([0., 140000.])
        .range([height - margin.bottom, margin.top])
        .nice(None);

    let color = ScaleOrdinal::default()
        .domain(vec![
            "Natural Resources",
            "Construction",
            "Manufacturing",
            "Trade",
            "Services",
            "Healthcare",
            "Education/Government",
        ])
        .range(vec![
            "#1b9e77", "#d95f02", "#7570b3", "#e7298a", "#66a61e", "#e6ab02", "#a6761d",
        ]);

    Ok(())
}
