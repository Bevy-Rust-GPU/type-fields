use crate::derive_closure;

use super::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

derive_closure!(Div);
