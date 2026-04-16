use std::marker::PhantomData;

pub trait Generator<D> {
    type Output;
    fn generate(&self, value: &D) -> Self::Output;
}

pub struct Constant<T>(pub T);

impl<T: Clone, D> Generator<D> for Constant<T> {
    type Output = T;

    fn generate(&self, _: &D) -> Self::Output {
        self.0.clone()
    }
}

pub struct Function<F, D, T>
where
    F: Fn(&D) -> T,
{
    f: F,
    _t: PhantomData<T>,
    _d: PhantomData<D>,
}

impl<F, D, T> Function<F, D, T>
where
    F: Fn(&D) -> T,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _t: PhantomData,
            _d: PhantomData,
        }
    }
}

impl<T: Clone, D, F> Generator<D> for Function<F, D, T>
where
    F: Fn(&D) -> T,
{
    type Output = T;

    fn generate(&self, value: &D) -> Self::Output {
        (self.f)(value)
    }
}
