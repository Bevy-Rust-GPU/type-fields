use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::Function,
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Lt;

impl<T, U> Function<(T, U)> for Lt where T: PartialOrd<U> {
    type Output = bool;

    fn call((l, r): (T, U)) -> Self::Output {
        l < r
    }
}
