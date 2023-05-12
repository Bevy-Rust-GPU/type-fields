use crate::macros::{arrow::Arrow, category::Category, Closure};

use crate::t_funk::Function;

/// Extract the second component of a 2-tuple
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Snd;

impl<A, B> Function<(A, B)> for Snd {
    type Output = B;

    fn call((_, b): (A, B)) -> Self::Output {
        b
    }
}
