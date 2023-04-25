use crate::macros::{functions, Copointed, Pointed};

use crate::t_funk::{Closure, Pointed};

/// Right-to-left composition
/// (.)
#[functions]
pub trait Compose<F>: Sized {
    type Compose;
    fn compose(self, f: F) -> Self::Compose;
}

impl<T, F> Compose<F> for T {
    type Compose = Composed<T, F>;

    /// Compose `F(A) -> B` with `F(B) -> C` to produce `F(A) -> C`
    fn compose(self, f: F) -> Self::Compose {
        Composed::point((self, f))
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed,
)]
pub struct Composed<F1, F2>(F1, F2);

impl<F1, F2, A> Closure<A> for Composed<F1, F2>
where
    F2: Closure<A>,
    F1: Closure<F2::Output>,
{
    type Output = F1::Output;

    fn call(self, a: A) -> Self::Output {
        self.0.call(self.1.call(a))
    }
}

