use crate::macros::functions;

/// A type that can unwrap a value
#[functions]
pub trait Copointed
where
    Self: Sized,
{
    type Copointed;

    /// Unwrap `Unit` from `Self`
    fn copoint(self) -> Self::Copointed;
}
