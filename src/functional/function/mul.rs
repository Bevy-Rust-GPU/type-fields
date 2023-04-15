use crate::derive_closure;

use super::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

derive_closure!(Mul);
