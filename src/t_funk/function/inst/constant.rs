use type_fields_macros::{Closure, Compose, First, Id, Second, Arr, Split, Fanout};

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
    Arr,
    First,
    Second,
    Split,
    Fanout,
)]
pub struct Const;

impl<A, B> Function<(A, B)> for Const {
    type Output = A;

    fn call((a, _): (A, B)) -> Self::Output {
        a
    }
}
