use type_fields_macros::{Category, Arrow};

use crate::{macros::Closure, t_funk::Function};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow)]
pub struct FlipTuple;

impl<A, B> Function<(A, B)> for FlipTuple {
    type Output = (B, A);

    fn call((a, b): (A, B)) -> Self::Output {
        (b, a)
    }
}

