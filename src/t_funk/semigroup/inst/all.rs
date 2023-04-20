use type_fields_macros::{Apply, Copointed, Fmap, Monad, Monoid, Pointed};

use crate::t_funk::{Copointed, Mappend, Pointed};

/// A `Semigroup` wrapper that can append with AND semantics.
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
pub struct All<T>(T);

impl<T> Mappend<All<T>> for All<T>
where
    T: core::ops::BitAnd<T>,
{
    type Mappend = All<T::Output>;

    fn mappend(self, t: All<T>) -> Self::Mappend {
        Pointed::point(self.copoint() & t.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        semigroup::{All, Mappend},
        Copointed, Pointed,
    };

    #[test]
    fn test_all() {
        assert_eq!(
            false,
            All::point(true)
                .mappend(All::point(false))
                .mappend(All::point(true))
                .copoint()
        )
    }
}
