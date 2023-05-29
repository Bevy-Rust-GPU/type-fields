use crate::{macros::Closure, t_funk::Function};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct RShiftTuple;

impl<A, B, C> Function<(A, (B, C))> for RShiftTuple {
    type Output = ((A, B), C);

    fn call((a, (b, c)): (A, (B, C))) -> Self::Output {
        ((a, b), c)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct LShiftTuple;

impl<A, B, C> Function<((A, B), C)> for LShiftTuple {
    type Output = (A, (B, C));

    fn call(((a, b), c): ((A, B), C)) -> Self::Output {
        (a, (b, c))
    }
}

