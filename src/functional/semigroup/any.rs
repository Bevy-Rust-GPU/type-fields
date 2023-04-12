use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_monad, derive_monoid,
    derive_pointed,
    functional::{Copointed, Pointed, Semigroup},
};

/// A `Semigroup` wrapper that can append with OR semantics.
pub struct Any<T>(T);

derive_pointed!(Any<T>);
derive_copointed!(Any<T>);
derive_functor!(Any<T>);
derive_applicative!(Any<T>);
derive_monad!(Any<T>);
derive_monoid!(Any<T>);

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
