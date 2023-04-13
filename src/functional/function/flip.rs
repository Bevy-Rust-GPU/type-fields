use crate::{derive_copointed, derive_pointed, functional::Pointed, derive_monad, derive_applicative, derive_functor};

use super::Function;

pub trait Flip: Sized {
    fn flip(self) -> Flipped<Self>;
}

impl<T> Flip for T {
    fn flip(self) -> Flipped<Self> {
        Flipped::point(self)
    }
}

pub struct Flipped<F>(F);

derive_pointed!(Flipped<F>);
derive_copointed!(Flipped<F>);
derive_functor!(Flipped<F>);
derive_applicative!(Flipped<F>);
derive_monad!(Flipped<F>);

impl<F, A, B> Function<(B, A)> for Flipped<F>
where
    F: Function<(A, B)>,
{
    type Output = F::Output;

    fn call(self, (b, a): (B, A)) -> Self::Output {
        self.0.call((a, b))
    }
}
