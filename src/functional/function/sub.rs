use super::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sub;

impl<A, B> Function<(A, B)> for Sub
where
    A: core::ops::Sub<B>,
{
    type Output = A::Output;

    fn call(self, (a, b): (A, B)) -> Self::Output {
        a - b
    }
}

