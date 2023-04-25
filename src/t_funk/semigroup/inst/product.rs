use crate::macros::{
    applicative::Apply, functor::Fmap, monad::Chain, monoid::Mempty, Copointed, Pointed,
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
    Fmap,
    Apply,
    Chain,
    Mempty,
)]
pub struct Product<T>(pub T);

impl<T> Mappend<Product<T>> for Product<T>
where
    T: Mul<T>,
{
    type Mappend = Product<T::Output>;

    fn mappend(self, t: Product<T>) -> Self::Mappend {
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
