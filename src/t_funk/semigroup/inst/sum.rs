use crate::macros::{
    applicative::Apply, functor::Fmap, monad::Chain, monoid::Mempty, Copointed, Pointed,
};

use crate::t_funk::Mappend;
use core::ops::Add;

/// A `Semigroup` wrapper that can append additively.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Fmap,
    Apply,
    Chain,
    Mempty,
)]
pub struct Sum<T>(pub T);

impl<T> Mappend<Sum<T>> for Sum<T>
where
    T: Add<T>,
{
    type Mappend = Sum<T::Output>;

    fn mappend(self, t: Sum<T>) -> Self::Mappend {
        Sum(self.0 + t.0)
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
