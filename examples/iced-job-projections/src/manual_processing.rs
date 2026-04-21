use serde::Deserialize;
use std::{collections::HashMap, fs};

const SECTOR_CATS: [(&str, &str); 16] = [
    ("Marketing", "Services"),
    ("Business Management & Administration", "Services"),
    ("Health Science", "Healthcare"),
    ("Hospitality & Tourism", "Services"),
    ("Architecture & Construction", "Construction"),
    ("Transportation, Distribution & Logistics", "Trade"),
    ("Human Services", "Healthcare"),
    ("Education & Training", "Education/Government"),
    ("Manufacturing", "Manufacturing"),
    ("Finance", "Services"),
    ("Agriculture, Food & Natural Resources", "Natural Resources"),
    ("Law, Public Safety, Corrections & Security", "Services"),
    ("Information Technology", "Services"),
    ("Arts, Audio/Video Technology & Communications", "Services"),
    ("Government & Public Adminstration", "Education/Government"),
    ("Science, Technology, Engineering & Mathematics", "Services"),
];

// Example:
// {
//     "SOCTitle"                   : "Cashiers",
//     "ed_level"                   : "No formal educational credential",
//     "sector"                     : "Marketing",
//     "Median Wage 2018"           : 21990.0,
//     "25th Percentile Wage 2018"  : 19580.0,
//     "75th Percentile Wage 2018"  : 24620.0,
//     "Total Jobs 2018"            : 18060,
//     "Annual Exits 2018-2028"     : 1705.9,
//     "Annual Transfers 2018-2028" : 1664.8,
//     "Annual Chage 2018-2028"     : 21.8,
//     "Annual Openings 2018-2028"  : 3392.5
// }
#[derive(Deserialize)]
struct RawItem {
    #[serde(rename = "SOCTitle")]
    soc_title: String,
    sector: String,
    #[serde(rename = "Median Wage 2018")]
    median_wage: Option<f32>,
    #[serde(rename = "Total Jobs 2018")]
    total_jobs: u32,
    #[serde(rename = "Annual Exits 2018-2028")]
    annual_exits: f32,
    #[serde(rename = "Annual Transfers 2018-2028")]
    annual_transfers: f32,
    #[serde(rename = "Annual Openings 2018-2028")]
    annual_openings: f32,
}

#[derive(Deserialize)]
struct RawData(Vec<RawItem>);

fn load_transform_data() -> Vec<Item> {
    let content = fs::read_to_string("src/trimmed-for-vis.json").unwrap();
    let raw_data = serde_json::from_str::<RawData>(&content).unwrap();

    let sector_cats: HashMap<&str, &str> = HashMap::from_iter(SECTOR_CATS);

    raw_data
        .0
        .into_iter()
        .filter_map(|raw_item| {
            let median_wage = raw_item.median_wage.unwrap_or_default();
            if median_wage <= 0. || median_wage > 140_000. {
                return None;
            }

            let sector = raw_item.sector.as_str();
            let sector_cat = sector_cats.get(&sector).unwrap_or(&sector).to_string();

            let openings = raw_item.annual_openings.round();
            let turnover =
                (raw_item.annual_exits + raw_item.annual_transfers) / (raw_item.total_jobs as f32);
            Some(Item {
                turnover,
                median_wage: median_wage,
                openings,
                sector_cat,
                soc_title: raw_item.soc_title,
                sector: raw_item.sector,
            })
        })
        .collect()
}

struct Item {
    turnover: f32,
    median_wage: f32,
    openings: f32,
    sector_cat: String,
    soc_title: String,
    sector: String,
}

pub struct Data {
    items: Vec<Item>,
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
        if self.index < self.data.len() {
            let item = &self.data.items[self.index];
            let row = Row {
                turnover: item.turnover,
                median_wage: item.median_wage,
                openings: item.openings,
                sector_cat: &item.sector_cat,
                soc_title: &item.soc_title,
                sector: &item.sector,
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
    pub fn new() -> Self {
        let items = load_transform_data();
        let openings: Vec<f32> = items.iter().map(|item| item.openings).collect();
        let turnover: Vec<f32> = items.iter().map(|item| item.turnover).collect();

        Self {
            radius_domain: [
                *openings
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(&0.),
                *openings
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(&1.),
            ],
            x_domain: [
                0.,
                *turnover
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(&1.),
            ],
            items,
        }
    }

    pub fn row(&self, index: usize) -> Row<'_> {
        let item = &self.items[index];
        Row {
            turnover: item.turnover,
            median_wage: item.median_wage,
            openings: item.openings,
            sector_cat: &item.sector_cat,
            soc_title: &item.soc_title,
            sector: &item.sector,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
