use crate::macros::{arrow::Arrow, category::Category, Closure};

use crate::t_funk::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow)]
pub struct Sub;

impl<A, B> Function<(A, B)> for Sub
where
    A: core::ops::Sub<B>,
{
    type Output = A::Output;

    fn call((a, b): (A, B)) -> Self::Output {
        a - b
    }
}
