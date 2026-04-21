use polars::prelude::*;
use std::fs::File;

fn load_transform_data() -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut file = File::open("src/trimmed-for-vis.json").unwrap();

    let sector_cats = [
        ["Marketing", "Services"],
        ["Business Management & Administration", "Services"],
        ["Health Science", "Healthcare"],
        ["Hospitality & Tourism", "Services"],
        ["Architecture & Construction", "Construction"],
        ["Transportation, Distribution & Logistics", "Trade"],
        ["Human Services", "Healthcare"],
        ["Education & Training", "Education/Government"],
        ["Manufacturing", "Manufacturing"],
        ["Finance", "Services"],
        ["Agriculture, Food & Natural Resources", "Natural Resources"],
        ["Law, Public Safety, Corrections & Security", "Services"],
        ["Information Technology", "Services"],
        ["Arts, Audio/Video Technology & Communications", "Services"],
        ["Government & Public Adminstration", "Education/Government"],
        ["Science, Technology, Engineering & Mathematics", "Services"],
    ];

    let clean_ed_level = [
        [
            "No formal educational credential",
            "High school diploma or less",
        ],
        [
            "High school diploma or equivalent",
            "High school diploma or less",
        ],
        [
            "Some college, post-HS training or Associate's degree",
            "Some college or two-year degree",
        ],
        ["Bachelor's degree", "Four-year degree"],
        ["Master's degree", "Graduate degree"],
        ["Doctoral or professional degree", "Graduate degree"],
    ];

    let ed_order = [
        ("High school diploma or less", 0),
        ("Some college or two-year degree", 2),
        ("Four-year degree", 3),
        ("Graduate degree", 5),
    ];

    let sector_expr = sector_cats
        .iter()
        .fold(col("sector"), |c, &[old, new]| {
            c.replace(lit(old), lit(new))
        })
        .alias("sector_cat");

    let clean_ed_expr = clean_ed_level
        .iter()
        .fold(col("ed_level"), |c, &[old, new]| {
            c.replace(lit(old), lit(new))
        })
        .alias("ed_level");

    let ed_level_expr = ed_order
        .iter()
        .fold(col("ed_level"), |c, &(old, new)| {
            c.replace(lit(old), lit(new))
        })
        .alias("ed_level_order");

    let annual_expr = col("Annual Openings 2018-2028")
        .round(0, RoundMode::default())
        .alias("openings");

    let turnover_expr = ((col("Annual Exits 2018-2028") + col("Annual Transfers 2018-2028"))
        / col("Total Jobs 2018"))
    .alias("turnover");

    let yref = lit(33_900).alias("yRef");

    Ok(JsonReader::new(&mut file)
        .finish()?
        .lazy()
        .with_columns([sector_expr, clean_ed_expr])
        .with_columns([ed_level_expr, annual_expr, turnover_expr, yref])
        .filter(
            col("Median Wage 2018")
                .gt(0.)
                .and(col("Median Wage 2018").lt_eq(140_000.)),
        )
        .collect()?)
}

/// Converts a `ChunkedArray` into `Vec<T>`
fn into_vec<'a, T: std::default::Default, U: PolarsDataType>(
    chunk_arr: &'a ChunkedArray<U>,
    f: impl Fn(U::Physical<'a>) -> T,
) -> Vec<T> {
    chunk_arr
        .iter()
        .map(|x| x.map(|v| f(v)).unwrap_or_default())
        .collect()
}

pub struct Data {
    turnover: Vec<f32>,
    median_wage: Vec<f32>,
    openings: Vec<f32>,
    sector_cat: Vec<String>,
    soc_title: Vec<String>,
    sector: Vec<String>,
    pub radius_domain: [f32; 2],
    pub x_domain: [f32; 2],
}

pub struct Row<'a> {
    pub turnover: f32,
    pub median_wage: f32,
    pub openings: f32,
    pub sector_cat: &'a str,
    pub soc_title: &'a str,
    pub sector: &'a str,
}

pub struct DataIterator<'a> {
    data: &'a Data,
    index: usize,
}

impl<'a> Iterator for DataIterator<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.turnover.len() {
            let row = Row {
                turnover: self.data.turnover[self.index],
                median_wage: self.data.median_wage[self.index],
                openings: self.data.openings[self.index],
                sector_cat: &self.data.sector_cat[self.index],
                soc_title: &self.data.soc_title[self.index],
                sector: &self.data.sector[self.index],
            };
            self.index += 1;
            Some(row)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Data {
    type Item = Row<'a>;
    type IntoIter = DataIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DataIterator {
            data: self,
            index: 0,
        }
    }
}

impl Data {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let df = load_transform_data()?;
        let openings = into_vec(df["openings"].f64()?, |v| v as f32);
        let turnover = into_vec(df["turnover"].f64()?, |v| v as f32);
        Ok(Self {
            radius_domain: Self::radius_domain(&openings),
            x_domain: Self::x_domain(&turnover),
            turnover,
            median_wage: into_vec(df["Median Wage 2018"].f64()?, |v| v as f32),
            openings,
            sector_cat: into_vec(df["sector_cat"].str()?, |v| v.to_string()),
            soc_title: into_vec(df["SOCTitle"].str()?, |v| v.to_string()),
            sector: into_vec(df["sector"].str()?, |v| v.to_string()),
        })
    }

    pub fn row(&self, index: usize) -> Row<'_> {
        Row {
            turnover: self.turnover[index],
            median_wage: self.median_wage[index],
            openings: self.openings[index],
            sector_cat: &self.sector_cat[index],
            soc_title: &self.soc_title[index],
            sector: &self.sector[index],
        }
    }

    pub fn len(&self) -> usize {
        self.turnover.len()
    }

    fn radius_domain(openings: &[f32]) -> [f32; 2] {
        [
            *openings
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(&0.),
            *openings
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(&1.),
        ]
    }
    fn x_domain(turnover: &[f32]) -> [f32; 2] {
        [
            0.,
            *turnover
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(&1.),
        ]
    }
}
