use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_monad, derive_pointed,
    t_funk::{Closure, Pointed},
};

/// Flip the arguments of an arity 2 function
pub trait Flip: Sized {
    fn flip(self) -> Flipped<Self>;
}

impl<T> Flip for T {
    fn flip(self) -> Flipped<Self> {
        Flipped::point(self)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Flipped<F>(F);

derive_pointed!(Flipped<F>);
derive_copointed!(Flipped<F>);
derive_functor!(Flipped<F>);
derive_applicative!(Flipped<F>);
derive_monad!(Flipped<F>);

impl<F, A, B> Closure<(B, A)> for Flipped<F>
where
    F: Closure<(A, B)>,
{
    type Output = F::Output;

    fn call(self, (b, a): (B, A)) -> Self::Output {
        self.0.call((a, b))
    }
}
