use crate::common::{RGBInterpolator, Scheme, split_colors};

#[derive(Debug)]
pub enum SequentialSpace {
    Blues,
    Greens,
    Greys,
    Oranges,
    Purples,
    Reds,
    BuGn,
    BuPu,
    GnBu,
    OrRd,
    PuBu,
    PuBuGn,
    PuRd,
    RdPu,
    YlGn,
    YlGnBu,
    YlOrBr,
    YlOrRd,
}

impl Scheme for SequentialSpace {
    fn scheme(&self) -> Vec<&str> {
        split_colors(match self {
            Self::Blues => "f7fbffdeebf7c6dbef9ecae16baed64292c62171b508519c08306b",
            Self::Greens => "f7fcf5e5f5e0c7e9c0a1d99b74c47641ab5d238b45006d2c00441b",
            Self::Greys => "fffffff0f0f0d9d9d9bdbdbd969696737373525252252525000000",
            Self::Oranges => "fff5ebfee6cefdd0a2fdae6bfd8d3cf16913d94801a636037f2704",
            Self::Purples => "fcfbfdefedf5dadaebbcbddc9e9ac8807dba6a51a354278f3f007d",
            Self::Reds => "fff5f0fee0d2fcbba1fc9272fb6a4aef3b2ccb181da50f1567000d",
            Self::BuGn => "f7fcfde5f5f9ccece699d8c966c2a441ae76238b45006d2c00441b",
            Self::BuPu => "f7fcfde0ecf4bfd3e69ebcda8c96c68c6bb188419d810f7c4d004b",
            Self::GnBu => "f7fcf0e0f3dbccebc5a8ddb57bccc44eb3d32b8cbe0868ac084081",
            Self::OrRd => "fff7ecfee8c8fdd49efdbb84fc8d59ef6548d7301fb300007f0000",
            Self::PuBu => "fff7fbece7f2d0d1e6a6bddb74a9cf3690c00570b0045a8d023858",
            Self::PuBuGn => "fff7fbece2f0d0d1e6a6bddb67a9cf3690c002818a016c59014636",
            Self::PuRd => "f7f4f9e7e1efd4b9dac994c7df65b0e7298ace125698004367001f",
            Self::RdPu => "fff7f3fde0ddfcc5c0fa9fb5f768a1dd3497ae017e7a017749006a",
            Self::YlGn => "ffffe5f7fcb9d9f0a3addd8e78c67941ab5d238443006837004529",
            Self::YlGnBu => "ffffd9edf8b1c7e9b47fcdbb41b6c41d91c0225ea8253494081d58",
            Self::YlOrBr => "ffffe5fff7bcfee391fec44ffe9929ec7014cc4c02993404662506",
            Self::YlOrRd => "ffffccffeda0fed976feb24cfd8d3cfc4e2ae31a1cbd0026800026",
        })
    }
}

impl SequentialSpace {
    pub fn interpolator(&self) -> RGBInterpolator {
        RGBInterpolator::new(self.scheme())
    }
}

#[cfg(test)]
mod tests {
    use super::SequentialSpace;

    #[test]
    fn test_sequential() {
        let step = 100;
        let variants = [
            SequentialSpace::Blues,
            SequentialSpace::Greens,
            SequentialSpace::Greys,
            SequentialSpace::Oranges,
            SequentialSpace::Purples,
            SequentialSpace::Reds,
            SequentialSpace::BuGn,
            SequentialSpace::BuPu,
            SequentialSpace::GnBu,
            SequentialSpace::OrRd,
            SequentialSpace::PuBu,
            SequentialSpace::PuBuGn,
            SequentialSpace::PuRd,
            SequentialSpace::RdPu,
            SequentialSpace::YlGn,
            SequentialSpace::YlGnBu,
            SequentialSpace::YlOrBr,
            SequentialSpace::YlOrRd,
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
