use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::Function,
};
// Given a square matrix, transpose it
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Transpose;

impl<A, B, C, D> Function<((A, B), (C, D))> for Transpose {
    type Output = ((A, C), (B, D));

    fn call(((a, b), (c, d)): ((A, B), (C, D))) -> Self::Output {
        ((a, c), (b, d))
    }
}
