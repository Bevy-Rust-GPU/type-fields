use crate::derive_closure;

use super::Function;

/// A type that can unwrap a value
pub trait Copointed
where
    Self: Sized,
{
    type Copointed;

    /// Unwrap `Unit` from `Self`
    fn copoint(self) -> Self::Copointed;
}

/// Copointed::copoint
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Copoint;

impl<I> Function<I> for Copoint
where
    I: Copointed,
{
    type Output = I::Copointed;

    fn call(input: I) -> I::Copointed {
        input.copoint()
    }
}

derive_closure!(Copoint);
