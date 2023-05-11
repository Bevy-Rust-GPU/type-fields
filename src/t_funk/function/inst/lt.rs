use crate::{
    macros::{arrow::arrow, category::category, Closure},
    t_funk::Function,
};

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Lt;

impl Function<(f32, f32)> for Lt {
    type Output = bool;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l < r
    }
}
