use type_fields_macros::{Closure, Compose, First, Id, Second};

use crate::t_funk::Function;

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Closure,
    Id,
    Compose,
    First,
    Second,
)]
pub struct Const;

impl<A, B> Function<(A, B)> for Const {
    type Output = A;

    fn call((a, _): (A, B)) -> Self::Output {
        a
    }
}
