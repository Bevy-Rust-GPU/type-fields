mod all;
mod any;
mod product;
mod sum;

pub use all::*;
pub use any::*;
pub use product::*;
pub use sum::*;

use crate::derive_closure;

use super::Function;

/// A type with a binary associative function.
pub trait Semigroup<T> {
    type Appended;

    fn mappend(self, t: T) -> Self::Appended;
}

#[derive(Default, Clone)]
pub struct Mappend;

impl<A, B> Function<(A, B)> for Mappend
where
    A: Semigroup<B>,
{
    type Output = A::Appended;

    fn call((a, b): (A, B)) -> Self::Output {
        a.mappend(b)
    }
}

derive_closure!(Mappend);

pub trait SemigroupConcat: Sized {
    type Concatenated;

    fn mconcat(self) -> Self::Concatenated;
}
