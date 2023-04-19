use type_fields_macros::{Applicative, Copointed, Functor, Monad, Monoid, Pointed};

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
    Functor,
    Applicative,
    Monad,
    Monoid,
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
