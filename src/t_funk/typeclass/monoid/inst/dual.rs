use crate::macros::{
    applicative::Applicative, foldable::Foldable, functor::Functor, monad::Monad, monoid::Monoid,
    semigroup::Semigroup, Copointed, Pointed,
};

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
    Functor,
    Applicative,
    Monad,
    Semigroup,
    Monoid,
    Foldable,
)]
pub struct Dual<T>(pub T);
