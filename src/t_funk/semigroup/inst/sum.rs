use crate::macros::{
    applicative::Apply, functor::Fmap, monad::Chain, monoid::Mempty, Copointed, Pointed,
};

use crate::t_funk::{Copointed, Mappend, Pointed};
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
pub struct Sum<T>(T);

impl<T> Mappend<Sum<T>> for Sum<T>
where
    T: Add<T>,
{
    type Mappend = Sum<T::Output>;

    fn mappend(self, t: Sum<T>) -> Self::Mappend {
        Pointed::point(self.copoint() + t.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{Copointed, Mappend, Pointed, Sum};

    #[test]
    fn test_sum() {
        assert_eq!(
            12,
            Sum::point(2)
                .mappend(Sum::point(4))
                .mappend(Sum::point(6))
                .copoint()
        )
    }
}
