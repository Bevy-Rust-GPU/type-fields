use super::{Cons, Nil};
use crate::t_funk::Closure;

/// A list of closures with chainable I/O is itself a valid closure
impl<T, N, I> Closure<I> for Cons<T, N>
where
    T: Closure<I>,
    N: Closure<T::Output>,
{
    type Output = <N as Closure<T::Output>>::Output;

    fn call(self, input: I) -> Self::Output {
        let Cons(lhs, rhs) = self;
        rhs.call(lhs.call(input))
    }
}

impl<I> Closure<I> for Nil {
    type Output = I;

    fn call(self, input: I) -> Self::Output {
        input
    }
}
