use type_fields_macros::{Copointed, Pointed, functions};

use crate::t_funk::{Closure, Pointed};

/// Flip the arguments of an arity 2 function
#[functions]
pub trait Flip: Sized {
    fn flip(self) -> Flipped<Self>;
}

impl<T> Flip for T {
    fn flip(self) -> Flipped<Self> {
        Flipped::point(self)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct Flipped<F>(F);

impl<F, A, B> Closure<(B, A)> for Flipped<F>
where
    F: Closure<(A, B)>,
{
    type Output = F::Output;

    fn call(self, (b, a): (B, A)) -> Self::Output {
        self.0.call((a, b))
    }
}
