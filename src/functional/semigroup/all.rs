use crate::functional::{Copointed, Pointed, Semigroup, Tagged};

/// A `Semigroup` wrapper that can append with AND semantics.
pub type All<T> = Tagged<TagAll, T>;
pub enum TagAll {}

impl<T> Semigroup<All<T>> for All<T>
where
    T: core::ops::BitAnd<T>,
{
    type Appended = All<T::Output>;

    fn mappend(self, t: All<T>) -> Self::Appended {
        Pointed::point(self.copoint() & t.copoint())
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{
        semigroup::{All, Semigroup},
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
