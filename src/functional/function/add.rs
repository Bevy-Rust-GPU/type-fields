use super::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Add;

impl<A, B> Function<(A, B)> for Add
where
    A: core::ops::Add<B>,
{
    type Output = A::Output;

    fn call(self, (a, b): (A, B)) -> Self::Output {
        a + b
    }
}
