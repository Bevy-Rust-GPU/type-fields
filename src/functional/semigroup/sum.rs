use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_monad, derive_monoid,
    derive_pointed,
    functional::{Copointed, Pointed, Semigroup},
};
use core::ops::Add;

/// A `Semigroup` wrapper that can append additively.
pub struct Sum<T>(T);

derive_pointed!(Sum<T>);
derive_copointed!(Sum<T>);
derive_functor!(Sum<T>);
derive_applicative!(Sum<T>);
derive_monad!(Sum<T>);
derive_monoid!(Sum<T>);

impl<T> Semigroup<Sum<T>> for Sum<T>
where
    T: Add<T>,
{
    type Appended = Sum<T::Output>;

    fn mappend(self, t: Sum<T>) -> Self::Appended {
        Pointed::point(self.copoint() + t.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{
        semigroup::{sum::Sum, Semigroup},
        Copointed, Pointed,
    };

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
