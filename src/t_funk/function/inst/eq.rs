use crate::{
    macros::{arrow::arrow, category::category, Closure},
    t_funk::Function,
};

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Eq;

impl Function<(f32, f32)> for Eq {
    type Output = bool;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l.eq(&r)
    }
}
