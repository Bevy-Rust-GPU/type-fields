use crate::t_funk::Closure;

/// A list of closures with chainable I/O is itself a valid closure
impl<LHS, RHS, I> Closure<I> for (LHS, RHS)
where
    LHS: Closure<I>,
    RHS: Closure<LHS::Output>,
{
    type Output = <RHS as Closure<LHS::Output>>::Output;

    fn call(self, input: I) -> Self::Output {
        let (lhs, rhs) = self;
        rhs.call(lhs.call(input))
    }
}

impl<I> Closure<I> for () {
    type Output = I;

    fn call(self, input: I) -> Self::Output {
        input
    }
}
