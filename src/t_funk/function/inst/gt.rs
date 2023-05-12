use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::Function,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow)]
pub struct Gt;

impl Function<(f32, f32)> for Gt {
    type Output = bool;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l > r
    }
}
