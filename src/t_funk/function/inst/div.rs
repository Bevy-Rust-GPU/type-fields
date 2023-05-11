use crate::macros::{arrow::arrow, category::category, Closure};

use crate::t_funk::Function;

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Div;

impl<A, B> Function<(A, B)> for Div
where
    A: core::ops::Div<B>,
{
    type Output = A::Output;

    fn call((a, b): (A, B)) -> Self::Output {
        a / b
    }
}
