use crate::common::{RGBInterpolator, split_colors};

pub enum Sequential {
    Blues,
    Greens,
    Greys,
    Oranges,
    Purples,
    Reds,
}

impl Sequential {
    pub fn scheme(&self) -> Vec<&str> {
        split_colors(match self {
            Self::Blues => "f7fbffdeebf7c6dbef9ecae16baed64292c62171b508519c08306b",
            Self::Greens => "f7fcf5e5f5e0c7e9c0a1d99b74c47641ab5d238b45006d2c00441b",
            Self::Greys => "fffffff0f0f0d9d9d9bdbdbd969696737373525252252525000000",
            Self::Oranges => "fff5ebfee6cefdd0a2fdae6bfd8d3cf16913d94801a636037f2704",
            Self::Purples => "fcfbfdefedf5dadaebbcbddc9e9ac8807dba6a51a354278f3f007d",
            Self::Reds => "fff5f0fee0d2fcbba1fc9272fb6a4aef3b2ccb181da50f1567000d",
        })
    }

    pub fn interpolator(&self) -> RGBInterpolator {
        RGBInterpolator::new(self.scheme())
    }
}
