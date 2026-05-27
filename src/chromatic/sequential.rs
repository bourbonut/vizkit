use super::common::{RGBInterpolator, Scheme, split_colors};

/// Color space used for [`Sequential`][`super::Sequential`] color map
#[derive(Debug, Clone)]
pub enum SequentialSpace {
    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#f7fbff" height="30" width="30" x="0"/>
    /// <rect fill="#deebf7" height="30" width="30" x="30"/>
    /// <rect fill="#c6dbef" height="30" width="30" x="60"/>
    /// <rect fill="#9ecae1" height="30" width="30" x="90"/>
    /// <rect fill="#6baed6" height="30" width="30" x="120"/>
    /// <rect fill="#4292c6" height="30" width="30" x="150"/>
    /// <rect fill="#2171b5" height="30" width="30" x="180"/>
    /// <rect fill="#08519c" height="30" width="30" x="210"/>
    /// <rect fill="#08306b" height="30" width="30" x="240"/>
    /// </svg>
    Blues,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#f7fcf5" height="30" width="30" x="0"/>
    /// <rect fill="#e5f5e0" height="30" width="30" x="30"/>
    /// <rect fill="#c7e9c0" height="30" width="30" x="60"/>
    /// <rect fill="#a1d99b" height="30" width="30" x="90"/>
    /// <rect fill="#74c476" height="30" width="30" x="120"/>
    /// <rect fill="#41ab5d" height="30" width="30" x="150"/>
    /// <rect fill="#238b45" height="30" width="30" x="180"/>
    /// <rect fill="#006d2c" height="30" width="30" x="210"/>
    /// <rect fill="#00441b" height="30" width="30" x="240"/>
    /// </svg>
    Greens,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#ffffff" height="30" width="30" x="0"/>
    /// <rect fill="#f0f0f0" height="30" width="30" x="30"/>
    /// <rect fill="#d9d9d9" height="30" width="30" x="60"/>
    /// <rect fill="#bdbdbd" height="30" width="30" x="90"/>
    /// <rect fill="#969696" height="30" width="30" x="120"/>
    /// <rect fill="#737373" height="30" width="30" x="150"/>
    /// <rect fill="#525252" height="30" width="30" x="180"/>
    /// <rect fill="#252525" height="30" width="30" x="210"/>
    /// <rect fill="#000000" height="30" width="30" x="240"/>
    /// </svg>
    Greys,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#fff5eb" height="30" width="30" x="0"/>
    /// <rect fill="#fee6ce" height="30" width="30" x="30"/>
    /// <rect fill="#fdd0a2" height="30" width="30" x="60"/>
    /// <rect fill="#fdae6b" height="30" width="30" x="90"/>
    /// <rect fill="#fd8d3c" height="30" width="30" x="120"/>
    /// <rect fill="#f16913" height="30" width="30" x="150"/>
    /// <rect fill="#d94801" height="30" width="30" x="180"/>
    /// <rect fill="#a63603" height="30" width="30" x="210"/>
    /// <rect fill="#7f2704" height="30" width="30" x="240"/>
    /// </svg>
    Oranges,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#fcfbfd" height="30" width="30" x="0"/>
    /// <rect fill="#efedf5" height="30" width="30" x="30"/>
    /// <rect fill="#dadaeb" height="30" width="30" x="60"/>
    /// <rect fill="#bcbddc" height="30" width="30" x="90"/>
    /// <rect fill="#9e9ac8" height="30" width="30" x="120"/>
    /// <rect fill="#807dba" height="30" width="30" x="150"/>
    /// <rect fill="#6a51a3" height="30" width="30" x="180"/>
    /// <rect fill="#54278f" height="30" width="30" x="210"/>
    /// <rect fill="#3f007d" height="30" width="30" x="240"/>
    /// </svg>
    Purples,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#fff5f0" height="30" width="30" x="0"/>
    /// <rect fill="#fee0d2" height="30" width="30" x="30"/>
    /// <rect fill="#fcbba1" height="30" width="30" x="60"/>
    /// <rect fill="#fc9272" height="30" width="30" x="90"/>
    /// <rect fill="#fb6a4a" height="30" width="30" x="120"/>
    /// <rect fill="#ef3b2c" height="30" width="30" x="150"/>
    /// <rect fill="#cb181d" height="30" width="30" x="180"/>
    /// <rect fill="#a50f15" height="30" width="30" x="210"/>
    /// <rect fill="#67000d" height="30" width="30" x="240"/>
    /// </svg>
    Reds,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#f7fcfd" height="30" width="30" x="0"/>
    /// <rect fill="#e5f5f9" height="30" width="30" x="30"/>
    /// <rect fill="#ccece6" height="30" width="30" x="60"/>
    /// <rect fill="#99d8c9" height="30" width="30" x="90"/>
    /// <rect fill="#66c2a4" height="30" width="30" x="120"/>
    /// <rect fill="#41ae76" height="30" width="30" x="150"/>
    /// <rect fill="#238b45" height="30" width="30" x="180"/>
    /// <rect fill="#006d2c" height="30" width="30" x="210"/>
    /// <rect fill="#00441b" height="30" width="30" x="240"/>
    /// </svg>
    BuGn,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#f7fcfd" height="30" width="30" x="0"/>
    /// <rect fill="#e0ecf4" height="30" width="30" x="30"/>
    /// <rect fill="#bfd3e6" height="30" width="30" x="60"/>
    /// <rect fill="#9ebcda" height="30" width="30" x="90"/>
    /// <rect fill="#8c96c6" height="30" width="30" x="120"/>
    /// <rect fill="#8c6bb1" height="30" width="30" x="150"/>
    /// <rect fill="#88419d" height="30" width="30" x="180"/>
    /// <rect fill="#810f7c" height="30" width="30" x="210"/>
    /// <rect fill="#4d004b" height="30" width="30" x="240"/>
    /// </svg>
    BuPu,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#f7fcf0" height="30" width="30" x="0"/>
    /// <rect fill="#e0f3db" height="30" width="30" x="30"/>
    /// <rect fill="#ccebc5" height="30" width="30" x="60"/>
    /// <rect fill="#a8ddb5" height="30" width="30" x="90"/>
    /// <rect fill="#7bccc4" height="30" width="30" x="120"/>
    /// <rect fill="#4eb3d3" height="30" width="30" x="150"/>
    /// <rect fill="#2b8cbe" height="30" width="30" x="180"/>
    /// <rect fill="#0868ac" height="30" width="30" x="210"/>
    /// <rect fill="#084081" height="30" width="30" x="240"/>
    /// </svg>
    GnBu,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#fff7ec" height="30" width="30" x="0"/>
    /// <rect fill="#fee8c8" height="30" width="30" x="30"/>
    /// <rect fill="#fdd49e" height="30" width="30" x="60"/>
    /// <rect fill="#fdbb84" height="30" width="30" x="90"/>
    /// <rect fill="#fc8d59" height="30" width="30" x="120"/>
    /// <rect fill="#ef6548" height="30" width="30" x="150"/>
    /// <rect fill="#d7301f" height="30" width="30" x="180"/>
    /// <rect fill="#b30000" height="30" width="30" x="210"/>
    /// <rect fill="#7f0000" height="30" width="30" x="240"/>
    /// </svg>
    OrRd,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#fff7fb" height="30" width="30" x="0"/>
    /// <rect fill="#ece7f2" height="30" width="30" x="30"/>
    /// <rect fill="#d0d1e6" height="30" width="30" x="60"/>
    /// <rect fill="#a6bddb" height="30" width="30" x="90"/>
    /// <rect fill="#74a9cf" height="30" width="30" x="120"/>
    /// <rect fill="#3690c0" height="30" width="30" x="150"/>
    /// <rect fill="#0570b0" height="30" width="30" x="180"/>
    /// <rect fill="#045a8d" height="30" width="30" x="210"/>
    /// <rect fill="#023858" height="30" width="30" x="240"/>
    /// </svg>
    PuBu,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#fff7fb" height="30" width="30" x="0"/>
    /// <rect fill="#ece2f0" height="30" width="30" x="30"/>
    /// <rect fill="#d0d1e6" height="30" width="30" x="60"/>
    /// <rect fill="#a6bddb" height="30" width="30" x="90"/>
    /// <rect fill="#67a9cf" height="30" width="30" x="120"/>
    /// <rect fill="#3690c0" height="30" width="30" x="150"/>
    /// <rect fill="#02818a" height="30" width="30" x="180"/>
    /// <rect fill="#016c59" height="30" width="30" x="210"/>
    /// <rect fill="#014636" height="30" width="30" x="240"/>
    /// </svg>
    PuBuGn,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#f7f4f9" height="30" width="30" x="0"/>
    /// <rect fill="#e7e1ef" height="30" width="30" x="30"/>
    /// <rect fill="#d4b9da" height="30" width="30" x="60"/>
    /// <rect fill="#c994c7" height="30" width="30" x="90"/>
    /// <rect fill="#df65b0" height="30" width="30" x="120"/>
    /// <rect fill="#e7298a" height="30" width="30" x="150"/>
    /// <rect fill="#ce1256" height="30" width="30" x="180"/>
    /// <rect fill="#980043" height="30" width="30" x="210"/>
    /// <rect fill="#67001f" height="30" width="30" x="240"/>
    /// </svg>
    PuRd,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#fff7f3" height="30" width="30" x="0"/>
    /// <rect fill="#fde0dd" height="30" width="30" x="30"/>
    /// <rect fill="#fcc5c0" height="30" width="30" x="60"/>
    /// <rect fill="#fa9fb5" height="30" width="30" x="90"/>
    /// <rect fill="#f768a1" height="30" width="30" x="120"/>
    /// <rect fill="#dd3497" height="30" width="30" x="150"/>
    /// <rect fill="#ae017e" height="30" width="30" x="180"/>
    /// <rect fill="#7a0177" height="30" width="30" x="210"/>
    /// <rect fill="#49006a" height="30" width="30" x="240"/>
    /// </svg>
    RdPu,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#ffffe5" height="30" width="30" x="0"/>
    /// <rect fill="#f7fcb9" height="30" width="30" x="30"/>
    /// <rect fill="#d9f0a3" height="30" width="30" x="60"/>
    /// <rect fill="#addd8e" height="30" width="30" x="90"/>
    /// <rect fill="#78c679" height="30" width="30" x="120"/>
    /// <rect fill="#41ab5d" height="30" width="30" x="150"/>
    /// <rect fill="#238443" height="30" width="30" x="180"/>
    /// <rect fill="#006837" height="30" width="30" x="210"/>
    /// <rect fill="#004529" height="30" width="30" x="240"/>
    /// </svg>
    YlGn,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#ffffd9" height="30" width="30" x="0"/>
    /// <rect fill="#edf8b1" height="30" width="30" x="30"/>
    /// <rect fill="#c7e9b4" height="30" width="30" x="60"/>
    /// <rect fill="#7fcdbb" height="30" width="30" x="90"/>
    /// <rect fill="#41b6c4" height="30" width="30" x="120"/>
    /// <rect fill="#1d91c0" height="30" width="30" x="150"/>
    /// <rect fill="#225ea8" height="30" width="30" x="180"/>
    /// <rect fill="#253494" height="30" width="30" x="210"/>
    /// <rect fill="#081d58" height="30" width="30" x="240"/>
    /// </svg>
    YlGnBu,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#ffffe5" height="30" width="30" x="0"/>
    /// <rect fill="#fff7bc" height="30" width="30" x="30"/>
    /// <rect fill="#fee391" height="30" width="30" x="60"/>
    /// <rect fill="#fec44f" height="30" width="30" x="90"/>
    /// <rect fill="#fe9929" height="30" width="30" x="120"/>
    /// <rect fill="#ec7014" height="30" width="30" x="150"/>
    /// <rect fill="#cc4c02" height="30" width="30" x="180"/>
    /// <rect fill="#993404" height="30" width="30" x="210"/>
    /// <rect fill="#662506" height="30" width="30" x="240"/>
    /// </svg>
    YlOrBr,

    /// <svg height="30" viewBox="0 0 270 30" width="270" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#ffffcc" height="30" width="30" x="0"/>
    /// <rect fill="#ffeda0" height="30" width="30" x="30"/>
    /// <rect fill="#fed976" height="30" width="30" x="60"/>
    /// <rect fill="#feb24c" height="30" width="30" x="90"/>
    /// <rect fill="#fd8d3c" height="30" width="30" x="120"/>
    /// <rect fill="#fc4e2a" height="30" width="30" x="150"/>
    /// <rect fill="#e31a1c" height="30" width="30" x="180"/>
    /// <rect fill="#bd0026" height="30" width="30" x="210"/>
    /// <rect fill="#800026" height="30" width="30" x="240"/>
    /// </svg>
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
    pub(crate) fn interpolator(&self) -> RGBInterpolator {
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
