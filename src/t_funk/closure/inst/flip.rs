use type_fields_macros::{functions, Compose, Copointed, First, Id, Pointed, Second, Arr, Split, Fanout};

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
    Id,
    Compose,
    Arr,
    First,
    Second,
    Split,
    Fanout,
)]
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
