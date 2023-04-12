use crate::functional::{Copointed, Pointed, Semigroup, Tagged};
use core::ops::Add;

/// A `Semigroup` wrapper that can append additively.
pub type Sum<T> = Tagged<TagSum, T>;
pub enum TagSum {}

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
