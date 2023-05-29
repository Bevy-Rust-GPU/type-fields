use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::Function,
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct ResultUnwrap;

impl<T, E> Function<Result<T, E>> for ResultUnwrap
where
    E: core::fmt::Debug,
{
    type Output = T;

    fn call(input: Result<T, E>) -> Self::Output {
        input.unwrap()
    }
}

