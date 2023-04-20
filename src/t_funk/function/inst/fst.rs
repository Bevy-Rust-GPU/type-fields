use type_fields_macros::Closure;

use crate::t_funk::Function;

/// Extract the first component of a 2-tuple
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Fst;

impl<A, B> Function<(A, B)> for Fst {
    type Output = A;

    fn call((a, _): (A, B)) -> Self::Output {
        a
    }
}

