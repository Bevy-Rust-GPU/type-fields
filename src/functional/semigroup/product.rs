use crate::functional::{Copointed, Semigroup, Tagged, Pointed};
use core::ops::Mul;

/// A `Semigroup` wrapper that can append multiplicatively.
pub type Product<T> = Tagged<TagProduct, T>;
pub enum TagProduct {}

impl<T> Semigroup<Product<T>> for Product<T>
where
    T: Mul<T>,
{
    type Appended = Product<T::Output>;

    fn mappend(self, t: Product<T>) -> Self::Appended {
        Pointed::point(self.copoint() * t.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{
        semigroup::{Product, Semigroup},
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
