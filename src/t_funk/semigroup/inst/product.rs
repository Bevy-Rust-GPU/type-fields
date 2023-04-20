use type_fields_macros::{Apply, Copointed, Fmap, Monad, Monoid, Pointed};

use crate::t_funk::{Copointed, Mappend, Pointed};
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
    Monad,
    Monoid,
)]
pub struct Product<T>(T);

impl<T> Mappend<Product<T>> for Product<T>
where
    T: Mul<T>,
{
    type Mappend = Product<T::Output>;

    fn mappend(self, t: Product<T>) -> Self::Mappend {
        Pointed::point(self.copoint() * t.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        semigroup::{Mappend, Product},
        Copointed, Pointed,
    };

    #[test]
    fn test_product() {
        assert_eq!(
            48,
            Product::point(2)
                .mappend(Product::point(4))
                .mappend(Product::point(6))
                .copoint()
        )
    }
}
