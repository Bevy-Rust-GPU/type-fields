use type_fields_macros::{Apply, Copointed, Fmap, Monad, Monoid, Pointed};

use crate::t_funk::{Copointed, Mappend, Pointed};

/// A `Semigroup` wrapper that can append with OR semantics.
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
pub struct Any<T>(T);

impl<T> Mappend<Any<T>> for Any<T>
where
    T: core::ops::BitOr<T>,
{
    type Mappend = Any<T::Output>;

    fn mappend(self, t: Any<T>) -> Self::Mappend {
        Pointed::point(self.copoint() | t.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        semigroup::{Any, Mappend},
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
