use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::Function,
};

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Max;

impl Function<(f32, f32)> for Max {
    type Output = f32;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l.max(r)
    }
}
