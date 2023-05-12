use crate::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::Function,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow)]
pub struct Eq;

impl Function<(f32, f32)> for Eq {
    type Output = bool;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l.eq(&r)
    }
}
