use crate::common::{RGBInterpolator, Scheme, split_colors};

pub enum DivergingSpace {
    BrBg,
    PiYg,
    PrGn,
    PuOr,
    RdBu,
    RdGy,
    RdYlBu,
    RdYlGn,
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
    pub fn interpolator(&self) -> RGBInterpolator {
        RGBInterpolator::new(self.scheme())
    }
}
