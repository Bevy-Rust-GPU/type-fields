use crate::macros::Closure;

use crate::t_funk::{Apply, Closure, Fmap, Function, LiftA2};

/// Lift a ternary function to actions
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct LiftA3;

impl<F, A, B, C> Function<(F, A, B, C)> for LiftA3
where
    A: Fmap<F>,
    A::Fmap: Apply<B>,
    <A::Fmap as Apply<B>>::Apply: Apply<C>,
{
    type Output = <<A::Fmap as Apply<B>>::Apply as Apply<C>>::Apply;

    fn call((f, a, b, c): (F, A, B, C)) -> Self::Output {
        LiftA2.call((f, a, b)).apply(c)
    }
}

