use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_monad, derive_monoid,
    derive_pointed,
    functional::{Copointed, Pointed, Mappend},
};
use core::ops::Mul;

/// A `Semigroup` wrapper that can append multiplicatively.
pub struct Product<T>(T);

derive_pointed!(Product<T>);
derive_copointed!(Product<T>);
derive_functor!(Product<T>);
derive_applicative!(Product<T>);
derive_monad!(Product<T>);
derive_monoid!(Product<T>);

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
    use crate::functional::{
        semigroup::{Product, Mappend},
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
