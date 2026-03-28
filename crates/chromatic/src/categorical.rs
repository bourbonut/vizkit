use crate::common::split_colors;

pub enum Categorical {
    Accent,
    Category10,
    Dark2,
    Observable10,
    Paired,
    Pastel1,
    Pastel2,
    Set1,
    Set2,
    Set3,
    Tableau10,
}

impl Categorical {
    pub fn scheme(&self) -> Vec<&str> {
        split_colors(match self {
            Self::Accent => "7fc97fbeaed4fdc086ffff99386cb0f0027fbf5b17666666",
            Self::Category10 => "1f77b4ff7f0e2ca02cd627289467bd8c564be377c27f7f7fbcbd2217becf",
            Self::Dark2 => "1b9e77d95f027570b3e7298a66a61ee6ab02a6761d666666",
            Self::Observable10 => "4269d0efb118ff725c6cc5b03ca951ff8ab7a463f297bbf59c6b4e9498a0",
            Self::Paired => {
                "a6cee31f78b4b2df8a33a02cfb9a99e31a1cfdbf6fff7f00cab2d66a3d9affff99b15928"
            }
            Self::Pastel1 => "fbb4aeb3cde3ccebc5decbe4fed9a6ffffcce5d8bdfddaecf2f2f2",
            Self::Pastel2 => "b3e2cdfdcdaccbd5e8f4cae4e6f5c9fff2aef1e2cccccccc",
            Self::Set1 => "e41a1c377eb84daf4a984ea3ff7f00ffff33a65628f781bf999999",
            Self::Set2 => "66c2a5fc8d628da0cbe78ac3a6d854ffd92fe5c494b3b3b3",
            Self::Set3 => {
                "8dd3c7ffffb3bebadafb807280b1d3fdb462b3de69fccde5d9d9d9bc80bdccebc5ffed6f"
            }
            Self::Tableau10 => "4e79a7f28e2ce1575976b7b259a14fedc949af7aa1ff9da79c755fbab0ab",
        })
    }
}
