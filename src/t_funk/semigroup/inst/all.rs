use crate::macros::{
    applicative::applicative, functor::functor, monad::monad,
    Copointed, Pointed,
};

use crate::t_funk::Mappend;

/// A `Semigroup` wrapper that can append with AND semantics.
#[functor]
#[applicative]
#[monad]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct All<T>(pub T);

impl<T, U> Mappend<All<U>> for All<T>
where
    T: core::ops::BitAnd<U>,
{
    type Mappend = All<T::Output>;

    fn mappend(self, t: All<U>) -> Self::Mappend {
        All(self.0 & t.0)
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::semigroup::{All, Mappend};

    #[test]
    fn test_all() {
        assert_eq!(All(false), All(true).mappend(All(false)).mappend(All(true)))
    }
}
