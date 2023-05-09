use crate::macros::{monad::Chain, Copointed, Pointed};

use crate::t_funk::{Apply, Closure, Fmap, Mappend, Mempty, Pure};
use core::ops::Add;

/// A `Semigroup` wrapper that can append additively.
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed, Chain,
)]
pub struct Sum<T>(pub T);

impl<T, F> Fmap<F> for Sum<T>
where
    F: Closure<T>,
{
    type Fmap = Sum<F::Output>;

    fn fmap(self, f: F) -> Self::Fmap {
        Sum(f.call(self.0))
    }
}

impl<F, T> Apply<Sum<T>> for Sum<F>
where
    F: Closure<T>,
{
    type Apply = Sum<F::Output>;

    fn apply(self, Sum(t): Sum<T>) -> Self::Apply {
        Sum(self.0.call(t))
    }
}

impl<T> Mappend<Sum<T>> for Sum<T>
where
    T: Add<T>,
{
    type Mappend = Sum<T::Output>;

    fn mappend(self, t: Sum<T>) -> Self::Mappend {
        Sum(self.0 + t.0)
    }
}

impl<T> Mempty for Sum<T>
where
    T: Default,
{
    type Mempty = Sum<T>;

    fn mempty() -> Self::Mempty {
        Sum(Default::default())
    }
}

impl<T> Pure for Sum<T> {
    type Pure<U> = Sum<U>;

    fn pure<U>(t: U) -> Self::Pure<U> {
        Sum(t)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{Mappend, Sum};

    #[test]
    fn test_sum() {
        assert_eq!(Sum(2).mappend(Sum(4)).mappend(Sum(6)), Sum(12))
    }
}
