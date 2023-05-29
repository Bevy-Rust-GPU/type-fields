use type_fields::macros::Closure;

use crate::t_funk::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Neg;

impl<T> Function<T> for Neg
where
    T: core::ops::Neg,
{
    type Output = T::Output;

    fn call(input: T) -> Self::Output {
        input.neg()
    }
}

