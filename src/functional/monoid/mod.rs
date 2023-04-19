mod inst;
mod mempty;

pub use inst::*;
pub use mempty::*;

use crate::functional::Semigroup;

pub trait Monoid: Semigroup {
    type Identity
    where
        Self: Mempty;

    fn mempty() -> Self::Identity
    where
        Self: Mempty;
}

impl<T> Monoid for T {
    type Identity = T::Mempty where T: Mempty;

    fn mempty() -> Self::Identity
    where
        T: Mempty,
    {
        <T as Mempty>::mempty()
    }
}
