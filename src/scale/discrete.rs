use std::collections::HashMap;
use std::hash::Hash;

/// Scaler with a discrete domain and a discrete range.
///
/// ```
/// use vizkit::scale::ScaleOrdinal;
///
/// // The method `apply` needs `&mut self`.
/// let mut scale = ScaleOrdinal::default()
///     .domain(&["a", "b", "c"])
///     .range(&["red", "green", "blue"]);
///
/// for c in "abcdefgh".split("") {
///     match c {
///         "a" => assert_eq!(scale.apply(c), Some("red").as_ref()),
///         "b" => assert_eq!(scale.apply(c), Some("green").as_ref()),
///         "c" => assert_eq!(scale.apply(c), Some("blue").as_ref()),
///         "d" => assert_eq!(scale.apply(c), None),
///         "e" => assert_eq!(scale.apply(c), None),
///         "f" => assert_eq!(scale.apply(c), None),
///         "g" => assert_eq!(scale.apply(c), None),
///         "h" => assert_eq!(scale.apply(c), None),
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
            range: range.iter().cloned().collect(),
            ..self
        }
    }

    /// Given the input, firstly it checks if the value exists in the domain, then it checks if it
    /// has a corresponding range value. It creates it a new one if not, and returns it. Otherwise
    /// it returns `None` (invalid value or empty range).
    pub fn apply(&self, x: D) -> Option<&R>
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
/// See [`ScaleBand::step`] and [`ScaleBand::bandwidth`].
///
/// ```
/// use vizkit::scale::ScaleBand;
///
/// // The method `apply` needs `&mut self`.
/// let scale = ScaleBand::default()
///     .domain(&["a", "b", "c"])
///     .range([0., 960.]);
///
/// for c in "abcdefgh".split("") {
///     match c {
///         "a" => assert_eq!(scale.apply(c), Some(0_f32).as_ref()),
///         "b" => assert_eq!(scale.apply(c), Some(320_f32).as_ref()),
///         "c" => assert_eq!(scale.apply(c), Some(640_f32).as_ref()),
///         "d" => assert_eq!(scale.apply(c), None),
///         "e" => assert_eq!(scale.apply(c), None),
///         "f" => assert_eq!(scale.apply(c), None),
///         "g" => assert_eq!(scale.apply(c), None),
///         "h" => assert_eq!(scale.apply(c), None),
///         "" => (),
///         x => unreachable!("char {} should not exist", x),
///     }
/// }
/// ```
pub struct ScaleBand<D>
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

impl<D> Default for ScaleBand<D>
where
    D: Hash + Eq + Default,
{
    fn default() -> Self {
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
    }
}

impl<D> ScaleBand<D>
where
    D: Hash + Eq + Clone,
{
    /// Returns a new [`ScaleBand`] with the specified domain applied.
    pub fn domain(self, domain: &[D]) -> Self {
        Self {
            scale_ordinal: self.scale_ordinal.domain(domain),
            ..self
        }
        .rescale()
    }

    /// Returns a new [`ScaleBand`] with the specified range applied.
    pub fn range(self, range: [f32; 2]) -> Self {
        let [r0, r1] = range;
        Self { r0, r1, ..self }.rescale()
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
    /// has a corresponding range value. It creates it a new one if not, and returns it. Otherwise
    /// it returns `None` (invalid value or empty range).
    pub fn apply(&self, x: D) -> Option<&f32> {
        self.scale_ordinal.index.get(&x).and_then(|i| {
            if self.scale_ordinal.range.is_empty() {
                return None;
            }
            let index = i % self.scale_ordinal.range.len();
            self.scale_ordinal.range.get(index)
        })
    }

    /// Returns the distance between two adjacent bands.
    pub fn step(&self) -> f32 {
        self.step
    }

    /// Returns the width of each band
    pub fn bandwidth(&self) -> f32 {
        self.bandwidth
    }
}
