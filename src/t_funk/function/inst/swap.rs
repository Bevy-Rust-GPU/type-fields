use crate::macros::Closure;

use crate::t_funk::Function;

/// Swap the elements of a 2-tuple
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Swap;

impl<A, B> Function<(A, B)> for Swap {
    type Output = (B, A);

    fn call((a, b): (A, B)) -> Self::Output {
        (b, a)
    }
}
