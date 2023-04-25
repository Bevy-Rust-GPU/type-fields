use type_fields_macros::{Apply, Chain, Copointed, Fmap, Mempty, Pointed};

use crate::t_funk::Mappend;

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
    Chain,
    Mempty,
)]
pub struct Any<T>(pub T);

impl<T> Mappend<Any<T>> for Any<T>
where
    T: core::ops::BitOr<T>,
{
    type Mappend = Any<T::Output>;

    fn mappend(self, t: Any<T>) -> Self::Mappend {
        Any(self.0 | t.0)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::semigroup::{Any, Mappend};

    #[test]
    fn test_any() {
        assert_eq!(Any(true), Any(true).mappend(Any(false)).mappend(Any(true)))
    }
}
