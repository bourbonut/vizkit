use super::common::{Scheme, split_colors};

/// Color space used for categorical schemes.
#[derive(Debug, Clone)]
pub enum CategoricalSpace {
    #[doc = include_str!("../../docs/accent.svg")]
    Accent,

    #[doc = include_str!("../../docs/category10.svg")]
    Category10,

    #[doc = include_str!("../../docs/dark2.svg")]
    Dark2,

    #[doc = include_str!("../../docs/observable10.svg")]
    Observable10,

    #[doc = include_str!("../../docs/paired.svg")]
    Paired,

    #[doc = include_str!("../../docs/pastel1.svg")]
    Pastel1,

    #[doc = include_str!("../../docs/pastel2.svg")]
    Pastel2,

    #[doc = include_str!("../../docs/set1.svg")]
    Set1,

    #[doc = include_str!("../../docs/set2.svg")]
    Set2,

    #[doc = include_str!("../../docs/set3.svg")]
    Set3,

    #[doc = include_str!("../../docs/tableau10.svg")]
    Tableau10,
}

impl Scheme for CategoricalSpace {
    fn scheme(&self) -> Vec<&str> {
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

#[cfg(test)]
mod tests {
    use super::{CategoricalSpace, Scheme};

    #[test]
    fn test_categorical() {
        let variants = [
            CategoricalSpace::Accent,
            CategoricalSpace::Category10,
            CategoricalSpace::Dark2,
            CategoricalSpace::Observable10,
            CategoricalSpace::Paired,
            CategoricalSpace::Pastel1,
            CategoricalSpace::Pastel2,
            CategoricalSpace::Set1,
            CategoricalSpace::Set2,
            CategoricalSpace::Set3,
            CategoricalSpace::Tableau10,
        ];
        for space in variants {
            let _ = space.scheme();
        }
    }
}
