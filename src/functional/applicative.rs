use core::marker::PhantomData;

use super::{Function, Functor};

/// A `Functor` type that can take a wrapped function,
/// map it over a provided value, and wrap the result
pub trait Applicative<F>: Functor<F> {
    /// <*>
    fn apply<B, A>(self, a: A) -> B
    where
        F: Function<A, Output = B>;
}

/// Applicative::apply
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Apply<F>(PhantomData<F>);

impl<T, F, A, B> Function<(T, A)> for Apply<F>
where
    T: Applicative<F>,
    F: Function<A, Output = B>,
{
    type Output = B;

    fn call(self, (t, a): (T, A)) -> Self::Output {
        t.apply(a)
    }
}
