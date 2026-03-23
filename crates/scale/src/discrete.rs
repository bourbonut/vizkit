use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

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
        D: for<'a> Borrow<&'a D> + Clone,
    {
        let mut index = HashMap::new();
        let mut next_domain = Vec::new();
        for value in domain.iter() {
            if index.contains_key(&value) {
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
