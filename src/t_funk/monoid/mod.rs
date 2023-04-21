mod inst;
mod mconcat;
mod mempty;

pub use inst::*;
pub use mconcat::*;
pub use mempty::*;

use crate::t_funk::Semigroup;

/// A type that can provide a neutral element.
///
/// To be definition-correct, `Monoid` types must also implement `Semigroup`,
/// but this cannot be strongly modeled without higher-ranked type bounds.
pub trait Monoid: Semigroup {
    type Identity
    where
        Self: Mempty;

    type Concatenated
    where
        Self: Mconcat;

    fn mempty() -> Self::Identity
    where
        Self: Mempty;

    fn mconcat(self) -> Self::Concatenated
    where
        Self: Mconcat;
}

impl<T> Monoid for T {
    type Identity = T::Mempty where T: Mempty;

    type Concatenated = T::Mconcat
    where
        T: Mconcat;

    fn mempty() -> Self::Identity
    where
        T: Mempty,
    {
        <T as Mempty>::mempty()
    }

    fn mconcat(self) -> Self::Concatenated
    where
        Self: Mconcat,
    {
        <T as Mconcat>::mconcat(self)
    }
}
