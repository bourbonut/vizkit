use std::collections::HashMap;
use std::hash::Hash;

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
    pub fn domain(self, domain: Vec<D>) -> Self
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

    pub fn range(self, range: Vec<R>) -> Self {
        Self { range, ..self }
    }

    pub fn apply(&mut self, x: D) -> Option<&R>
    where
        D: Clone,
    {
        match self.index.get(&x) {
            None => {
                self.domain.push(x.clone());
                let i = self.domain.len() - 1;
                self.index.insert(x.clone(), i);
                if self.range.is_empty() {
                    return None;
                }
                let index = i % self.range.len();
                return self.range.get(index);
            }
            Some(i) => {
                if self.range.is_empty() {
                    return None;
                }
                let index = i % self.range.len();
                return self.range.get(index);
            }
        }
    }
}

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
    pub fn domain(self, domain: Vec<D>) -> Self {
        Self {
            scale_ordinal: self.scale_ordinal.domain(domain),
            ..self
        }
        .rescale()
    }

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
            scale_ordinal: self.scale_ordinal.range(range),
            step,
            bandwidth,
            ..self
        }
    }

    pub fn apply(&mut self, x: D) -> Option<&f32> {
        self.scale_ordinal.index.get(&x).and_then(|i| {
            if self.scale_ordinal.range.is_empty() {
                return None;
            }
            let index = i % self.scale_ordinal.range.len();
            self.scale_ordinal.range.get(index)
        })
    }

    pub fn step(&self) -> f32 {
        self.step
    }

    pub fn bandwidth(&self) -> f32 {
        self.bandwidth
    }
}

mod tests {
    #[test]
    fn test_ordinal() {
        let mut s = super::ScaleOrdinal::default()
            .domain(vec!["a", "b", "c"])
            .range(vec!["red", "green", "blue"]);
        for c in "abcdefgh".split("") {
            match c {
                "a" => assert_eq!(s.apply(c), Some("red").as_ref()),
                "b" => assert_eq!(s.apply(c), Some("green").as_ref()),
                "c" => assert_eq!(s.apply(c), Some("blue").as_ref()),
                "d" => assert_eq!(s.apply(c), Some("red").as_ref()),
                "e" => assert_eq!(s.apply(c), Some("green").as_ref()),
                "f" => assert_eq!(s.apply(c), Some("blue").as_ref()),
                "g" => assert_eq!(s.apply(c), Some("red").as_ref()),
                "h" => assert_eq!(s.apply(c), Some("green").as_ref()),
                "" => (),
                x => unreachable!("char {} should not exist", x),
            }
        }
    }

    #[test]
    fn test_scale() {
        let mut s = super::ScaleBand::default()
            .domain(vec!["a", "b", "c"])
            .range([0., 960.]);
        for c in "abcdefgh".split("") {
            match c {
                "a" => assert_eq!(s.apply(c), Some(0_f32).as_ref()),
                "b" => assert_eq!(s.apply(c), Some(320_f32).as_ref()),
                "c" => assert_eq!(s.apply(c), Some(640_f32).as_ref()),
                "d" => assert_eq!(s.apply(c), None),
                "e" => assert_eq!(s.apply(c), None),
                "f" => assert_eq!(s.apply(c), None),
                "g" => assert_eq!(s.apply(c), None),
                "h" => assert_eq!(s.apply(c), None),
                "" => (),
                x => unreachable!("char {} should not exist", x),
            }
        }
    }
}
