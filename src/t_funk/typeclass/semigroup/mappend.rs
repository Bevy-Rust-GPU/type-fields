use crate::macros::functions;

/// A type with a binary associative function.
#[functions]
pub trait Mappend<T> {
    type Mappend;

    fn mappend(self, t: T) -> Self::Mappend;
}

