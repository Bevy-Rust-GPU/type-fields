/// A type that can unwrap a value
pub trait Copointed
where
    Self: Sized,
{
    type Copointed;

    /// Unwrap `Unit` from `Self`
    fn unwrap(self) -> Self::Copointed;
}

