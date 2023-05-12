use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::Function,
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Abs;

impl Function<f32> for Abs {
    type Output = f32;

    fn call(input: f32) -> Self::Output {
        input.abs()
    }
}
