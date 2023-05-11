use crate::macros::{
    applicative::applicative, foldable::foldable, functor::functor, monad::monad, monoid::monoid,
    semigroup::semigroup, Copointed, Pointed,
};

#[functor]
#[applicative]
#[monad]
#[semigroup]
#[monoid]
#[foldable]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct Dual<T>(pub T);
