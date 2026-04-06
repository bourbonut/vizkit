use polars::prelude::*;
use std::fs::File;

pub fn load_transform_data() -> Result<DataFrame, Box<dyn std::error::Error>> {
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

    let turnover_expr = (col("Annual Exits 2018-2028") + col("Annual Transfers 2018-2028"))
        / col("Total Jobs 2018").alias("turnover");

    let yref = lit(33900).alias("yRef");

    Ok(JsonReader::new(&mut file)
        .finish()?
        .lazy()
        .with_columns([sector_expr, clean_ed_expr])
        .with_columns([ed_level_expr, annual_expr, turnover_expr, yref])
        .filter(
            col("Median Wage 2018")
                .gt(0.)
                .and(col("Median Wage 2018").lt_eq(140000.)),
        )
        .collect()?)
}
