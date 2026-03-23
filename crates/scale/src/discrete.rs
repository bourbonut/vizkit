use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default)]
struct ScaleOrdinal<D, R>
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
    fn domain(self, domain: Vec<D>) -> Self
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

    fn range(self, range: Vec<R>) -> Self {
        Self {
            domain: self.domain,
            index: self.index,
            range,
        }
    }

    fn apply(&mut self, x: D) -> Option<&R>
    where
        D: Clone,
    {
        match self.index.get(&x) {
            None => {
                self.domain.push(x.clone());
                let i = self.domain.len() - 1;
                self.index.insert(x.clone(), i);
                let length = self.range.len();
                if self.range.is_empty() {
                    return None;
                }
                let index = i % length;
                return self.range.get(index);
            }
            Some(i) => {
                let length = self.range.len();
                if self.range.is_empty() {
                    return None;
                }
                let index = i % length;
                return self.range.get(index);
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_ordinal() {
        let mut s = ScaleOrdinal::<&str, &str>::default()
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
}
