use crate::macros::{arrow::Arrow, category::Category, Closure};

use crate::t_funk::Function;

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Mul;

impl<A, B> Function<(A, B)> for Mul
where
    A: core::ops::Mul<B>,
{
    type Output = A::Output;

    fn call((a, b): (A, B)) -> Self::Output {
        a * b
    }
}
