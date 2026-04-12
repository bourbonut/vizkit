pub trait Generator1D {
    type Output;
    fn generate(&self, value: f32) -> Self::Output;
}

pub struct Constant1D<T>(pub T);

impl<T: Clone> Generator1D for Constant1D<T> {
    type Output = T;
    fn generate(&self, _: f32) -> Self::Output {
        self.0.clone()
    }
}

pub struct Function1D<F, T>(pub F)
where
    F: Fn(f32) -> T;

impl<F, T> Generator1D for Function1D<F, T>
where
    F: Fn(f32) -> T,
{
    type Output = T;
    fn generate(&self, value: f32) -> Self::Output {
        (self.0)(value)
    }
}

pub trait Generator2D {
    type Output;
    fn generate(&self, x: f32, y: f32) -> Self::Output;
}

pub struct Constant2D<T>(pub T);

impl<T: Clone> Generator2D for Constant2D<T> {
    type Output = T;
    fn generate(&self, _: f32, _: f32) -> Self::Output {
        self.0.clone()
    }
}

pub struct Function2D<F, T>(pub F)
where
    F: Fn(f32, f32) -> T;

impl<F, T> Generator2D for Function2D<F, T>
where
    F: Fn(f32, f32) -> T,
{
    type Output = T;
    fn generate(&self, x: f32, y: f32) -> Self::Output {
        (self.0)(x, y)
    }
}
