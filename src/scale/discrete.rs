use std::collections::HashMap;
use std::hash::Hash;

use super::Axis;

/// Scaler with a discrete domain and a discrete range.
///
/// ```
/// use vizkit::scale::ScaleOrdinal;
///
/// let scaler = ScaleOrdinal::default()
///     .domain(&["a", "b", "c"])
///     .range(&["red", "green", "blue"]);
///
/// for c in "abcdefgh".split("") {
///     match c {
///         "a" => assert_eq!(scaler.scale(c), Some("red").as_ref()),
///         "b" => assert_eq!(scaler.scale(c), Some("green").as_ref()),
///         "c" => assert_eq!(scaler.scale(c), Some("blue").as_ref()),
///         "d" => assert_eq!(scaler.scale(c), None),
///         "e" => assert_eq!(scaler.scale(c), None),
///         "f" => assert_eq!(scaler.scale(c), None),
///         "g" => assert_eq!(scaler.scale(c), None),
///         "h" => assert_eq!(scaler.scale(c), None),
///         "" => (),
///         x => unreachable!("char {} should not exist", x),
///     }
/// }
/// ```
#[derive(Default)]
pub struct ScaleOrdinal<D, R>
where
    D: Hash + Eq,
{
    index: HashMap<D, usize>,
    domain: Vec<D>,
    range: Vec<R>,
}

impl<D, R> ScaleOrdinal<D, R>
where
    D: Hash + Eq,
{
    /// Returns a new [`ScaleOrdinal`] with the specified domain applied.
    pub fn domain(self, domain: &[D]) -> Self
    where
        D: Clone,
    {
        let mut index = HashMap::new();
        let mut next_domain = Vec::new();
        for value in domain.iter() {
            if index.contains_key(value) {
                continue;
            }
            next_domain.push(value.clone());
            index.insert(value.clone(), next_domain.len() - 1);
        }
        Self {
            index,
            domain: next_domain,
            range: self.range,
        }
    }

    /// Returns a new [`ScaleOrdinal`] with the specified range applied.
    pub fn range(self, range: &[R]) -> Self
    where
        R: Clone,
    {
        Self {
            range: range.to_vec(),
            ..self
        }
    }

    /// Given the input, firstly it checks if the value exists in the domain, then it checks if it
    /// has a corresponding range value. If a value has been found, the value is returned. Otherwise
    /// it returns `None` (invalid value or empty range).
    pub fn scale(&self, x: D) -> Option<&R>
    where
        D: Clone,
    {
        match self.index.get(&x) {
            None => {
                // For non deterministic behavior, this code must be uncommented.
                // Howewer it implies to set `self` as `&mut self` which is not flexible.
                // self.domain.push(x.clone());
                // let i = self.domain.len() - 1;
                // self.index.insert(x.clone(), i);
                // if self.range.is_empty() {
                //     return None;
                // }
                // let index = i % self.range.len();
                // self.range.get(index)
                None
            }
            Some(i) => {
                if self.range.is_empty() {
                    return None;
                }
                let index = i % self.range.len();
                self.range.get(index)
            }
        }
    }
}

/// Scaler with a discrete domain and a continous range.
///
/// Additionally, it computes _band_ dimensions used for typically bar charts.
///
/// See [`ScaleDiscrete::step`] and [`ScaleDiscrete::bandwidth`].
///
///
/// ```
/// use vizkit::scale::ScaleDiscrete;
///
/// let scaler = ScaleDiscrete::band()
///     .domain(&["a", "b", "c"])
///     .range([0., 960.]);
///
/// for c in "abcd".split("") {
///     match c {
///         "a" => assert_eq!(scaler.scale(c), Some(0.)),
///         "b" => assert_eq!(scaler.scale(c), Some(320.)),
///         "c" => assert_eq!(scaler.scale(c), Some(640.)),
///         "d" => assert_eq!(scaler.scale(c), None),
///         "" => (),
///         x => unreachable!("char {} should not exist", x),
///     }
/// }
///
/// let scaler = ScaleDiscrete::point()
///     .domain(&["a", "b", "c"])
///     .range([0., 960.]);
///
/// for c in "abcd".split("") {
///     match c {
///         "a" => assert_eq!(scaler.scale(c), Some(0.)),
///         "b" => assert_eq!(scaler.scale(c), Some(480.)),
///         "c" => assert_eq!(scaler.scale(c), Some(960.)),
///         "d" => assert_eq!(scaler.scale(c), None),
///         "" => (),
///         x => unreachable!("char {} should not exist", x),
///     }
/// }
/// ```
pub struct ScaleDiscrete<D>
where
    D: Hash + Eq,
{
    r0: f32,
    r1: f32,
    step: f32,
    bandwidth: f32,
    padding_inner: f32,
    padding_outer: f32,
    align: f32,
    scale_ordinal: ScaleOrdinal<D, f32>,
}

impl<D> ScaleDiscrete<D>
where
    D: Hash + Eq,
{
    #[doc = concat!(
        "Returns a default [`ScaleDiscrete`] with uniform bands. You should use this default",
        " settings when you want ticks centered on the bandwidth.\n\n",
        include_str!("../../docs/band.svg"),
    )]
    pub fn band() -> Self
    where
        D: Default,
    {
        Self {
            r0: 0.,
            r1: 1.,
            step: 0.,
            bandwidth: 0.,
            padding_inner: 0.,
            padding_outer: 0.,
            align: 0.5,
            scale_ordinal: ScaleOrdinal::default(),
        }
        .rescale()
    }

    #[doc = concat!(
        "Returns a default [`ScaleDiscrete`] where the bandwidth is null.\n\n",
        include_str!("../../docs/point.svg"),
    )]
    pub fn point() -> Self
    where
        D: Default,
    {
        Self {
            r0: 0.,
            r1: 1.,
            step: 0.,
            bandwidth: 0.,
            padding_inner: 1.,
            padding_outer: 0.,
            align: 0.5,
            scale_ordinal: ScaleOrdinal::default(),
        }
        .rescale()
    }

    /// Returns a new [`ScaleDiscrete`] with the specified domain applied.
    pub fn domain(self, domain: &[D]) -> Self
    where
        D: Clone,
    {
        Self {
            scale_ordinal: self.scale_ordinal.domain(domain),
            ..self
        }
        .rescale()
    }

    /// Returns a new [`ScaleDiscrete`] with the specified range applied.
    pub fn range(self, range: [f32; 2]) -> Self {
        let [r0, r1] = range;
        Self { r0, r1, ..self }.rescale()
    }

    #[doc = concat!(
        "Returns a new [`ScaleDiscrete`] in which inner and outer paddings are set to the same",
        " padding value. The inner and outer padding values range from 0.0 to 1.0.\n",
        "# Example\n",
        "padding: 0.4\n\n",
        include_str!("../../docs/band_padding.svg"),
    )]
    pub fn padding(self, padding: f32) -> Self {
        Self {
            padding_outer: padding.clamp(0., 1.),
            padding_inner: padding.clamp(0., 1.),
            ..self
        }
        .rescale()
    }

    #[doc = concat!(
        "Returns a new [`ScaleDiscrete`] in which inner padding is set to the padding value. The",
        " inner padding value ranges from 0.0 to 1.0.\n",
        "# Example\n",
        "padding_inner: 0.4\n\n",
        include_str!("../../docs/band_padding_inner.svg"),
    )]
    pub fn padding_inner(self, padding_inner: f32) -> Self {
        Self {
            padding_inner: padding_inner.clamp(0., 1.),
            ..self
        }
        .rescale()
    }

    #[doc = concat!(
        "Returns a new [`ScaleDiscrete`] with a new outer padding value. The outer padding value",
        " ranges from 0.0 to 1.0.\n",
        "# Example\n",
        "padding_outer: 0.4\n\n",
        include_str!("../../docs/band_padding_outer.svg"),
    )]
    pub fn padding_outer(self, padding_outer: f32) -> Self {
        Self {
            padding_outer: padding_outer.clamp(0., 1.),
            ..self
        }
        .rescale()
    }

    #[doc = concat!(
        "Returns a new [`ScaleDiscrete`] in which align is set to the specified value. The align",
        " value ranges from 0.0 to 1.0.\n",
        "# Example\n",
        "padding: 0.4; align: 0.1\n\n",
        include_str!("../../docs/band_align_1.svg"),
        "\npadding: 0.4; align: 0.9\n\n",
        include_str!("../../docs/band_align_2.svg"),
    )]
    pub fn align(self, align: f32) -> Self {
        Self {
            align: align.clamp(0., 1.),
            ..self
        }
        .rescale()
    }

    fn rescale(self) -> Self {
        let n = self.scale_ordinal.domain.len();
        let reverse = self.r1 < self.r0;
        let start = if reverse { self.r1 } else { self.r0 };
        let stop = if reverse { self.r0 } else { self.r1 };
        let step =
            (stop - start) / 1_f32.max(n as f32 - self.padding_inner + self.padding_outer * 2.);
        let start = start + (stop - start - step * (n as f32 - self.padding_inner)) * self.align;
        let bandwidth = step * (1. - self.padding_inner);
        let mut range: Vec<f32> = (0..n).map(|i| start + step * i as f32).collect();
        if reverse {
            range.reverse();
        }
        Self {
            scale_ordinal: self.scale_ordinal.range(&range),
            step,
            bandwidth,
            ..self
        }
    }

    /// Given the input, firstly it checks if the value exists in the domain, then it checks if it
    /// has a corresponding range value. If a value has been found, the value is returned. Otherwise
    /// it returns `None` (invalid value or empty range).
    pub fn scale(&self, x: D) -> Option<f32> {
        self.scale_ordinal
            .index
            .get(&x)
            .and_then(|i| {
                if self.scale_ordinal.range.is_empty() {
                    return None;
                }
                let index = i % self.scale_ordinal.range.len();
                self.scale_ordinal.range.get(index)
            })
            .copied()
    }

    /// Returns the distance between two adjacent bands.
    pub fn step(&self) -> f32 {
        self.step
    }

    /// Returns the width of each band.
    pub fn bandwidth(&self) -> f32 {
        self.bandwidth
    }
}

impl<D> Axis for ScaleDiscrete<D>
where
    D: Hash + Eq + Clone,
{
    type Tick = D;

    fn ticks(&self, _: Option<usize>) -> Vec<Self::Tick> {
        self.scale_ordinal.domain.to_vec()
    }

    fn tick_position(&self, x: Self::Tick) -> f32 {
        self.scale(x).unwrap_or(0.0) + self.bandwidth() * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_scale_ordinal() {
        let s = ScaleOrdinal::default()
            .domain(&[0, 1])
            .range(&["foo", "bar"]);
        assert_eq!(s.scale(0), Some("foo").as_ref());
        assert_eq!(s.scale(1), Some("bar").as_ref());
        assert_eq!(s.scale(2), None);
        assert_eq!(s.scale(-1), None);

        let s = s.range(&["a", "b", "c"]);
        assert_eq!(s.scale(0), Some("a").as_ref());
        assert_eq!(s.scale(1), Some("b").as_ref());
        assert_eq!(s.scale(2), None);

        let s = s.domain(&[0, 1, 2]);
        assert_eq!(s.scale(2), Some("c").as_ref());

        let s = s.domain(&[0, 1]);
        assert_eq!(s.scale(2), None);
    }

    fn band() -> ScaleDiscrete<&'static str> {
        ScaleDiscrete::band()
    }

    fn point() -> ScaleDiscrete<&'static str> {
        ScaleDiscrete::point()
    }

    #[rstest]
    #[case(band, 480., 40., 30., [0., 40., 80.], [7.5, 45., 82.5])]
    #[case(point, 960., 0., 30., [0., 60., 120.], [7.5, 45., 82.5])]
    fn test_scale_discrete(
        #[case] init: impl Fn() -> ScaleDiscrete<&'static str>,
        #[case] bar_value: f32,
        #[case] bandwidth1: f32,
        #[case] bandwidth2: f32,
        #[case] domain1: [f32; 3],
        #[case] domain2: [f32; 3],
    ) {
        let s = init().range([0., 960.]);
        assert_eq!(s.scale("foo"), None);

        let s = init().domain(&["foo", "bar"]).range([0., 960.]);
        assert_eq!(s.scale("foo"), Some(0.));
        assert_eq!(s.scale("bar"), Some(bar_value));

        let s = init().domain(&["a", "b", "c"]).range([0., 120.]);
        let range: Vec<f32> = s.ticks(None).iter().map(|x| s.tick_position(x)).collect();
        let bandwidth = s.bandwidth();
        let expected: Vec<f32> = domain1.map(|x| x + bandwidth * 0.5).into_iter().collect();
        assert_eq!(range, expected);
        assert_eq!(s.bandwidth(), bandwidth1);

        let s = s.padding(0.2);
        let range: Vec<f32> = s.ticks(None).iter().map(|x| s.tick_position(x)).collect();
        let bandwidth = s.bandwidth();
        let expected: Vec<f32> = domain2.map(|x| x + bandwidth * 0.5).into_iter().collect();
        assert_eq!(range, expected);
        assert_eq!(s.bandwidth(), bandwidth2);
    }
}
