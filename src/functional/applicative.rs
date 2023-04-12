use core::marker::PhantomData;

use super::Function;

/// A type that can take a wrapped function,
/// map it over a provided value, and wrap the result
///
/// To be definition-correct, `Applicative` types must also implement `Functor`,
/// but this cannot be strongly modeled without higher-ranked type bounds.
pub trait Applicative<T> {
    type Applied;

    /// <*>
    fn apply(self, a: T) -> Self::Applied;
}

impl<T> Applicative<T> for () {
    type Applied = ();

    fn apply(self, _: T) -> Self::Applied {
        ()
    }
}

/// Applicative::apply
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Apply<T>(PhantomData<T>);

impl<U, T> Function<(U, T)> for Apply<T>
where
    U: Applicative<T>,
{
    type Output = U::Applied;

    fn call(self, (u, t): (U, T)) -> Self::Output {
        u.apply(t)
    }
}
