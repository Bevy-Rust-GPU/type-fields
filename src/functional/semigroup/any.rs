use crate::functional::{Copointed, Pointed, Semigroup, Tagged};

/// A `Semigroup` wrapper that can append with OR semantics.
pub type Any<T> = Tagged<TagAny, T>;
pub enum TagAny {}

impl<T> Semigroup<Any<T>> for Any<T>
where
    T: core::ops::BitOr<T>,
{
    type Appended = Any<T::Output>;

    fn mappend(self, t: Any<T>) -> Self::Appended {
        Pointed::point(self.copoint() | t.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{
        semigroup::{Any, Semigroup},
        Copointed, Pointed,
    };

    #[test]
    fn test_any() {
        assert_eq!(
            true,
            Any::point(true)
                .mappend(Any::point(false))
                .mappend(Any::point(true))
                .copoint()
        )
    }
}
