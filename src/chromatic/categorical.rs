use super::common::{Scheme, split_colors};

/// Color space used for categorical schemes.
pub enum CategoricalSpace {
    /// <svg height="30" viewBox="0 0 240 30" width="240" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#7fc97f" height="30" width="30" x="0"/>
    /// <rect fill="#beaed4" height="30" width="30" x="30"/>
    /// <rect fill="#fdc086" height="30" width="30" x="60"/>
    /// <rect fill="#ffff99" height="30" width="30" x="90"/>
    /// <rect fill="#386cb0" height="30" width="30" x="120"/>
    /// <rect fill="#f0027f" height="30" width="30" x="150"/>
    /// <rect fill="#bf5b17" height="30" width="30" x="180"/>
    /// <rect fill="#666666" height="30" width="30" x="210"/>
    /// </svg>
    Accent,

    /// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#1f77b4" height="30" width="30" x="0"/>
    /// <rect fill="#ff7f0e" height="30" width="30" x="30"/>
    /// <rect fill="#2ca02c" height="30" width="30" x="60"/>
    /// <rect fill="#d62728" height="30" width="30" x="90"/>
    /// <rect fill="#9467bd" height="30" width="30" x="120"/>
    /// <rect fill="#8c564b" height="30" width="30" x="150"/>
    /// <rect fill="#e377c2" height="30" width="30" x="180"/>
    /// <rect fill="#7f7f7f" height="30" width="30" x="210"/>
    /// <rect fill="#bcbd22" height="30" width="30" x="240"/>
    /// <rect fill="#17becf" height="30" width="30" x="270"/>
    /// </svg>
    Category10,

    /// <svg height="30" viewBox="0 0 240 30" width="240" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#1b9e77" height="30" width="30" x="0"/>
    /// <rect fill="#d95f02" height="30" width="30" x="30"/>
    /// <rect fill="#7570b3" height="30" width="30" x="60"/>
    /// <rect fill="#e7298a" height="30" width="30" x="90"/>
    /// <rect fill="#66a61e" height="30" width="30" x="120"/>
    /// <rect fill="#e6ab02" height="30" width="30" x="150"/>
    /// <rect fill="#a6761d" height="30" width="30" x="180"/>
    /// <rect fill="#666666" height="30" width="30" x="210"/>
    /// </svg>
    Dark2,

    /// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#4269d0" height="30" width="30" x="0"/>
    /// <rect fill="#efb118" height="30" width="30" x="30"/>
    /// <rect fill="#ff725c" height="30" width="30" x="60"/>
    /// <rect fill="#6cc5b0" height="30" width="30" x="90"/>
    /// <rect fill="#3ca951" height="30" width="30" x="120"/>
    /// <rect fill="#ff8ab7" height="30" width="30" x="150"/>
    /// <rect fill="#a463f2" height="30" width="30" x="180"/>
    /// <rect fill="#97bbf5" height="30" width="30" x="210"/>
    /// <rect fill="#9c6b4e" height="30" width="30" x="240"/>
    /// <rect fill="#9498a0" height="30" width="30" x="270"/>
    /// </svg>
    Observable10,

    /// <svg height="30" viewBox="0 0 360 30" width="360" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#a6cee3" height="30" width="30" x="0"/>
    /// <rect fill="#1f78b4" height="30" width="30" x="30"/>
    /// <rect fill="#b2df8a" height="30" width="30" x="60"/>
    /// <rect fill="#33a02c" height="30" width="30" x="90"/>
    /// <rect fill="#fb9a99" height="30" width="30" x="120"/>
    /// <rect fill="#e31a1c" height="30" width="30" x="150"/>
    /// <rect fill="#fdbf6f" height="30" width="30" x="180"/>
    /// <rect fill="#ff7f00" height="30" width="30" x="210"/>
    /// <rect fill="#cab2d6" height="30" width="30" x="240"/>
    /// <rect fill="#6a3d9a" height="30" width="30" x="270"/>
    /// <rect fill="#ffff99" height="30" width="30" x="300"/>
    /// <rect fill="#b15928" height="30" width="30" x="330"/>
    /// </svg>
    Paired,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#fbb4ae" height="30" width="30" x="0"/>
    /// <rect fill="#b3cde3" height="30" width="30" x="30"/>
    /// <rect fill="#ccebc5" height="30" width="30" x="60"/>
    /// <rect fill="#decbe4" height="30" width="30" x="90"/>
    /// <rect fill="#fed9a6" height="30" width="30" x="120"/>
    /// <rect fill="#ffffcc" height="30" width="30" x="150"/>
    /// <rect fill="#e5d8bd" height="30" width="30" x="180"/>
    /// <rect fill="#fddaec" height="30" width="30" x="210"/>
    /// <rect fill="#f2f2f2" height="30" width="30" x="240"/>
    /// </svg>
    Pastel1,

    /// <svg height="30" viewBox="0 0 240 30" width="240" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#b3e2cd" height="30" width="30" x="0"/>
    /// <rect fill="#fdcdac" height="30" width="30" x="30"/>
    /// <rect fill="#cbd5e8" height="30" width="30" x="60"/>
    /// <rect fill="#f4cae4" height="30" width="30" x="90"/>
    /// <rect fill="#e6f5c9" height="30" width="30" x="120"/>
    /// <rect fill="#fff2ae" height="30" width="30" x="150"/>
    /// <rect fill="#f1e2cc" height="30" width="30" x="180"/>
    /// <rect fill="#cccccc" height="30" width="30" x="210"/>
    /// </svg>
    Pastel2,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#e41a1c" height="30" width="30" x="0"/>
    /// <rect fill="#377eb8" height="30" width="30" x="30"/>
    /// <rect fill="#4daf4a" height="30" width="30" x="60"/>
    /// <rect fill="#984ea3" height="30" width="30" x="90"/>
    /// <rect fill="#ff7f00" height="30" width="30" x="120"/>
    /// <rect fill="#ffff33" height="30" width="30" x="150"/>
    /// <rect fill="#a65628" height="30" width="30" x="180"/>
    /// <rect fill="#f781bf" height="30" width="30" x="210"/>
    /// <rect fill="#999999" height="30" width="30" x="240"/>
    /// </svg>
    Set1,

    /// <svg height="30" viewBox="0 0 240 30" width="240" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#66c2a5" height="30" width="30" x="0"/>
    /// <rect fill="#fc8d62" height="30" width="30" x="30"/>
    /// <rect fill="#8da0cb" height="30" width="30" x="60"/>
    /// <rect fill="#e78ac3" height="30" width="30" x="90"/>
    /// <rect fill="#a6d854" height="30" width="30" x="120"/>
    /// <rect fill="#ffd92f" height="30" width="30" x="150"/>
    /// <rect fill="#e5c494" height="30" width="30" x="180"/>
    /// <rect fill="#b3b3b3" height="30" width="30" x="210"/>
    /// </svg>
    Set2,

    /// <svg height="30" viewBox="0 0 360 30" width="360" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#8dd3c7" height="30" width="30" x="0"/>
    /// <rect fill="#ffffb3" height="30" width="30" x="30"/>
    /// <rect fill="#bebada" height="30" width="30" x="60"/>
    /// <rect fill="#fb8072" height="30" width="30" x="90"/>
    /// <rect fill="#80b1d3" height="30" width="30" x="120"/>
    /// <rect fill="#fdb462" height="30" width="30" x="150"/>
    /// <rect fill="#b3de69" height="30" width="30" x="180"/>
    /// <rect fill="#fccde5" height="30" width="30" x="210"/>
    /// <rect fill="#d9d9d9" height="30" width="30" x="240"/>
    /// <rect fill="#bc80bd" height="30" width="30" x="270"/>
    /// <rect fill="#ccebc5" height="30" width="30" x="300"/>
    /// <rect fill="#ffed6f" height="30" width="30" x="330"/>
    /// </svg>
    Set3,

    /// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#4e79a7" height="30" width="30" x="0"/>
    /// <rect fill="#f28e2c" height="30" width="30" x="30"/>
    /// <rect fill="#e15759" height="30" width="30" x="60"/>
    /// <rect fill="#76b7b2" height="30" width="30" x="90"/>
    /// <rect fill="#59a14f" height="30" width="30" x="120"/>
    /// <rect fill="#edc949" height="30" width="30" x="150"/>
    /// <rect fill="#af7aa1" height="30" width="30" x="180"/>
    /// <rect fill="#ff9da7" height="30" width="30" x="210"/>
    /// <rect fill="#9c755f" height="30" width="30" x="240"/>
    /// <rect fill="#bab0ab" height="30" width="30" x="270"/>
    /// </svg>
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
