use crate::{t_funk::Function, macros::Closure};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Eq;

impl Function<(f32, f32)> for Eq {
    type Output = bool;

    fn call((l, r): (f32, f32)) -> Self::Output {
        l.eq(&r)
    }
}

