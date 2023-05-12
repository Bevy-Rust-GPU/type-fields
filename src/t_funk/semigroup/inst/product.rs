use crate::macros::{
    applicative::Applicative, foldable::Foldable, functor::Functor, monad::Monad, monoid::Monoid,
    Copointed, Pointed,
};

use crate::t_funk::Mappend;
use core::ops::Mul;

/// A `Semigroup` wrapper that can append multiplicatively.
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
    Foldable,
)]
pub struct Product<T>(pub T);

impl<T, U> Mappend<Product<U>> for Product<T>
where
    T: Mul<U>,
{
    type Mappend = Product<T::Output>;

    fn mappend(self, t: Product<U>) -> Self::Mappend {
        Product(self.0 * t.0)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::semigroup::{Mappend, Product};

    #[test]
    fn test_product() {
        assert_eq!(
            Product(48),
            Product(2).mappend(Product(4)).mappend(Product(6))
        )
    }
}
