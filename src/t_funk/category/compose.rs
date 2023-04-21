use type_fields_macros::functions;

use crate::t_funk::{Closure, Copointed, Pointed};

/// Right-to-left composition
/// (. or <<<)
#[functions]
pub trait Compose<F>: Sized {
    fn compose(self, f: F) -> Composed<Self, F>;
}

/// Left-to-right composition
/// (>>>)
#[functions]
pub trait ComposeL<F>: Sized {
    fn compose_l(self, f: F) -> Composed<F, Self>;
}

impl<T, F> ComposeL<F> for T {
    fn compose_l(self, f: F) -> Composed<F, Self> {
        Composed(f, self)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Composed<F1, F2>(F1, F2);

impl<F1, F2> Pointed for Composed<F1, F2> {
    type Pointed = (F1, F2);

    fn point(unit: Self::Pointed) -> Self {
        Composed(unit.0, unit.1)
    }
}

impl<F1, F2> Copointed for Composed<F1, F2> {
    type Copointed = (F1, F2);

    fn copoint(self) -> Self::Copointed {
        (self.0, self.1)
    }
}

impl<F1, F2, A> Closure<A> for Composed<F1, F2>
where
    F1: Closure<A>,
    F2: Closure<F1::Output>,
{
    type Output = F2::Output;

    fn call(self, a: A) -> Self::Output {
        self.1.call(self.0.call(a))
    }
}
