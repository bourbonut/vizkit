use super::common::{RGBInterpolator, Scheme, split_colors};

/// Color space used for [`Diverging`][`super::Diverging`] color map
#[derive(Debug, Clone)]
pub enum DivergingSpace {
    /// <svg height="30" viewBox="0 0 330 30" width="330" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#543005" height="30" width="30" x="0"/>
    /// <rect fill="#8c510a" height="30" width="30" x="30"/>
    /// <rect fill="#bf812d" height="30" width="30" x="60"/>
    /// <rect fill="#dfc27d" height="30" width="30" x="90"/>
    /// <rect fill="#f6e8c3" height="30" width="30" x="120"/>
    /// <rect fill="#f5f5f5" height="30" width="30" x="150"/>
    /// <rect fill="#c7eae5" height="30" width="30" x="180"/>
    /// <rect fill="#80cdc1" height="30" width="30" x="210"/>
    /// <rect fill="#35978f" height="30" width="30" x="240"/>
    /// <rect fill="#01665e" height="30" width="30" x="270"/>
    /// <rect fill="#003c30" height="30" width="30" x="300"/>
    /// </svg>
    BrBg,

    /// <svg height="30" viewBox="0 0 330 30" width="330" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#8e0152" height="30" width="30" x="0"/>
    /// <rect fill="#c51b7d" height="30" width="30" x="30"/>
    /// <rect fill="#de77ae" height="30" width="30" x="60"/>
    /// <rect fill="#f1b6da" height="30" width="30" x="90"/>
    /// <rect fill="#fde0ef" height="30" width="30" x="120"/>
    /// <rect fill="#f7f7f7" height="30" width="30" x="150"/>
    /// <rect fill="#e6f5d0" height="30" width="30" x="180"/>
    /// <rect fill="#b8e186" height="30" width="30" x="210"/>
    /// <rect fill="#7fbc41" height="30" width="30" x="240"/>
    /// <rect fill="#4d9221" height="30" width="30" x="270"/>
    /// <rect fill="#276419" height="30" width="30" x="300"/>
    /// </svg>
    PiYg,

    /// <svg height="30" viewBox="0 0 330 30" width="330" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#40004b" height="30" width="30" x="0"/>
    /// <rect fill="#762a83" height="30" width="30" x="30"/>
    /// <rect fill="#9970ab" height="30" width="30" x="60"/>
    /// <rect fill="#c2a5cf" height="30" width="30" x="90"/>
    /// <rect fill="#e7d4e8" height="30" width="30" x="120"/>
    /// <rect fill="#f7f7f7" height="30" width="30" x="150"/>
    /// <rect fill="#d9f0d3" height="30" width="30" x="180"/>
    /// <rect fill="#a6dba0" height="30" width="30" x="210"/>
    /// <rect fill="#5aae61" height="30" width="30" x="240"/>
    /// <rect fill="#1b7837" height="30" width="30" x="270"/>
    /// <rect fill="#00441b" height="30" width="30" x="300"/>
    /// </svg>
    PrGn,

    /// <svg height="30" viewBox="0 0 330 30" width="330" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#2d004b" height="30" width="30" x="0"/>
    /// <rect fill="#542788" height="30" width="30" x="30"/>
    /// <rect fill="#8073ac" height="30" width="30" x="60"/>
    /// <rect fill="#b2abd2" height="30" width="30" x="90"/>
    /// <rect fill="#d8daeb" height="30" width="30" x="120"/>
    /// <rect fill="#f7f7f7" height="30" width="30" x="150"/>
    /// <rect fill="#fee0b6" height="30" width="30" x="180"/>
    /// <rect fill="#fdb863" height="30" width="30" x="210"/>
    /// <rect fill="#e08214" height="30" width="30" x="240"/>
    /// <rect fill="#b35806" height="30" width="30" x="270"/>
    /// <rect fill="#7f3b08" height="30" width="30" x="300"/>
    /// </svg>
    PuOr,

    /// <svg height="30" viewBox="0 0 330 30" width="330" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#67001f" height="30" width="30" x="0"/>
    /// <rect fill="#b2182b" height="30" width="30" x="30"/>
    /// <rect fill="#d6604d" height="30" width="30" x="60"/>
    /// <rect fill="#f4a582" height="30" width="30" x="90"/>
    /// <rect fill="#fddbc7" height="30" width="30" x="120"/>
    /// <rect fill="#f7f7f7" height="30" width="30" x="150"/>
    /// <rect fill="#d1e5f0" height="30" width="30" x="180"/>
    /// <rect fill="#92c5de" height="30" width="30" x="210"/>
    /// <rect fill="#4393c3" height="30" width="30" x="240"/>
    /// <rect fill="#2166ac" height="30" width="30" x="270"/>
    /// <rect fill="#053061" height="30" width="30" x="300"/>
    /// </svg>
    RdBu,

    /// <svg height="30" viewBox="0 0 330 30" width="330" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#67001f" height="30" width="30" x="0"/>
    /// <rect fill="#b2182b" height="30" width="30" x="30"/>
    /// <rect fill="#d6604d" height="30" width="30" x="60"/>
    /// <rect fill="#f4a582" height="30" width="30" x="90"/>
    /// <rect fill="#fddbc7" height="30" width="30" x="120"/>
    /// <rect fill="#ffffff" height="30" width="30" x="150"/>
    /// <rect fill="#e0e0e0" height="30" width="30" x="180"/>
    /// <rect fill="#bababa" height="30" width="30" x="210"/>
    /// <rect fill="#878787" height="30" width="30" x="240"/>
    /// <rect fill="#4d4d4d" height="30" width="30" x="270"/>
    /// <rect fill="#1a1a1a" height="30" width="30" x="300"/>
    /// </svg>
    RdGy,

    /// <svg height="30" viewBox="0 0 330 30" width="330" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#a50026" height="30" width="30" x="0"/>
    /// <rect fill="#d73027" height="30" width="30" x="30"/>
    /// <rect fill="#f46d43" height="30" width="30" x="60"/>
    /// <rect fill="#fdae61" height="30" width="30" x="90"/>
    /// <rect fill="#fee090" height="30" width="30" x="120"/>
    /// <rect fill="#ffffbf" height="30" width="30" x="150"/>
    /// <rect fill="#e0f3f8" height="30" width="30" x="180"/>
    /// <rect fill="#abd9e9" height="30" width="30" x="210"/>
    /// <rect fill="#74add1" height="30" width="30" x="240"/>
    /// <rect fill="#4575b4" height="30" width="30" x="270"/>
    /// <rect fill="#313695" height="30" width="30" x="300"/>
    /// </svg>
    RdYlBu,

    /// <svg height="30" viewBox="0 0 330 30" width="330" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#a50026" height="30" width="30" x="0"/>
    /// <rect fill="#d73027" height="30" width="30" x="30"/>
    /// <rect fill="#f46d43" height="30" width="30" x="60"/>
    /// <rect fill="#fdae61" height="30" width="30" x="90"/>
    /// <rect fill="#fee08b" height="30" width="30" x="120"/>
    /// <rect fill="#ffffbf" height="30" width="30" x="150"/>
    /// <rect fill="#d9ef8b" height="30" width="30" x="180"/>
    /// <rect fill="#a6d96a" height="30" width="30" x="210"/>
    /// <rect fill="#66bd63" height="30" width="30" x="240"/>
    /// <rect fill="#1a9850" height="30" width="30" x="270"/>
    /// <rect fill="#006837" height="30" width="30" x="300"/>
    /// </svg>
    RdYlGn,

    /// <svg height="30" viewBox="0 0 330 30" width="330" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#9e0142" height="30" width="30" x="0"/>
    /// <rect fill="#d53e4f" height="30" width="30" x="30"/>
    /// <rect fill="#f46d43" height="30" width="30" x="60"/>
    /// <rect fill="#fdae61" height="30" width="30" x="90"/>
    /// <rect fill="#fee08b" height="30" width="30" x="120"/>
    /// <rect fill="#ffffbf" height="30" width="30" x="150"/>
    /// <rect fill="#e6f598" height="30" width="30" x="180"/>
    /// <rect fill="#abdda4" height="30" width="30" x="210"/>
    /// <rect fill="#66c2a5" height="30" width="30" x="240"/>
    /// <rect fill="#3288bd" height="30" width="30" x="270"/>
    /// <rect fill="#5e4fa2" height="30" width="30" x="300"/>
    /// </svg>
    Spectral,
}

impl Scheme for DivergingSpace {
    fn scheme(&self) -> Vec<&str> {
        split_colors(match self {
            Self::BrBg => "5430058c510abf812ddfc27df6e8c3f5f5f5c7eae580cdc135978f01665e003c30",
            Self::PiYg => "8e0152c51b7dde77aef1b6dafde0eff7f7f7e6f5d0b8e1867fbc414d9221276419",
            Self::PrGn => "40004b762a839970abc2a5cfe7d4e8f7f7f7d9f0d3a6dba05aae611b783700441b",
            Self::PuOr => "2d004b5427888073acb2abd2d8daebf7f7f7fee0b6fdb863e08214b358067f3b08",
            Self::RdBu => "67001fb2182bd6604df4a582fddbc7f7f7f7d1e5f092c5de4393c32166ac053061",
            Self::RdGy => "67001fb2182bd6604df4a582fddbc7ffffffe0e0e0bababa8787874d4d4d1a1a1a",
            Self::RdYlBu => "a50026d73027f46d43fdae61fee090ffffbfe0f3f8abd9e974add14575b4313695",
            Self::RdYlGn => "a50026d73027f46d43fdae61fee08bffffbfd9ef8ba6d96a66bd631a9850006837",
            Self::Spectral => "9e0142d53e4ff46d43fdae61fee08bffffbfe6f598abdda466c2a53288bd5e4fa2",
        })
    }
}

impl DivergingSpace {
    pub(crate) fn interpolator(&self) -> RGBInterpolator {
        RGBInterpolator::new(self.scheme())
    }
}

#[cfg(test)]
mod tests {
    use super::DivergingSpace;

    #[test]
    fn test_sequential() {
        let step = 100;
        let variants = [
            DivergingSpace::BrBg,
            DivergingSpace::PiYg,
            DivergingSpace::PrGn,
            DivergingSpace::PuOr,
            DivergingSpace::RdBu,
            DivergingSpace::RdGy,
            DivergingSpace::RdYlBu,
            DivergingSpace::RdYlGn,
            DivergingSpace::Spectral,
        ];
        for space in variants {
            let interpolator = space.interpolator();
            let colors: Vec<[f32; 3]> = (0..=step)
                .map(|i| interpolator.interpolate(i as f32 / step as f32))
                .collect();
            for color in colors {
                let [r, g, b] = color;
                assert!(
                    0. <= r && r <= 1.,
                    "red must be between [0, 1] (variant: {:?})",
                    space
                );
                assert!(
                    0. <= g && g <= 1.,
                    "green must be between [0, 1] (variant: {:?})",
                    space
                );
                assert!(
                    0. <= b && b <= 1.,
                    "blue must be between [0, 1] (variant: {:?})",
                    space
                );
            }
        }
    }
}
