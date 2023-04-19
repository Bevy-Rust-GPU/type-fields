use type_fields_macros::Closure;

use super::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Const;

impl<A, B> Function<(A, B)> for Const {
    type Output = A;

    fn call((a, _): (A, B)) -> Self::Output {
        a
    }
}
