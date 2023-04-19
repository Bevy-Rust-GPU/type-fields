use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_monad, derive_monoid,
    derive_pointed,
    t_funk::{Copointed, Pointed, Mappend},
};

/// A `Semigroup` wrapper that can append with AND semantics.
pub struct All<T>(T);

derive_pointed!(All<T>);
derive_copointed!(All<T>);
derive_functor!(All<T>);
derive_applicative!(All<T>);
derive_monad!(All<T>);
derive_monoid!(All<T>);

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