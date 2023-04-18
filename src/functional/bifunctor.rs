use crate::derive_closure;

use super::{Closure, Function};

pub trait BifunctorFirst<F> {
    type First;

    fn first(self, f: F) -> Self::First;
}

impl<A, B, F> BifunctorFirst<F> for (A, B)
where
    F: Closure<A>,
{
    type First = (F::Output, B);

    fn first(self, f: F) -> Self::First {
        (f.call(self.0), self.1)
    }
}

pub struct FirstF;

impl<T, F> Function<(T, F)> for FirstF
where
    T: BifunctorFirst<F>,
{
    type Output = T::First;

    fn call((t, f): (T, F)) -> Self::Output {
        t.first(f)
    }
}

derive_closure!(FirstF);

pub trait BifunctorSecond<F> {
    type Second;

    fn second(self, f: F) -> Self::Second;
}

impl<A, B, F> BifunctorSecond<F> for (A, B)
where
    F: Closure<B>,
{
    type Second = (A, F::Output);

    fn second(self, f: F) -> Self::Second {
        (self.0, f.call(self.1))
    }
}

pub struct SecondF;

impl<T, F> Function<(T, F)> for SecondF
where
    T: BifunctorSecond<F>,
{
    type Output = T::Second;

    fn call((t, f): (T, F)) -> Self::Output {
        t.second(f)
    }
}

derive_closure!(SecondF);

