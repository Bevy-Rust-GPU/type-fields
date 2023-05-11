use crate::{
    macros::{arrow::arrow, category::category, Closure},
    t_funk::Function,
};

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Min;

impl Function<(f32, f32)> for Min {
    type Output = f32;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l.min(r)
    }
}
