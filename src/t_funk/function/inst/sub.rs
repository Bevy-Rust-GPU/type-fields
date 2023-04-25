use type_fields_macros::{Arr, Closure, Compose, Fanout, First, Id, Second, Split};

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
pub struct Sub;

impl<A, B> Function<(A, B)> for Sub
where
    A: core::ops::Sub<B>,
{
    type Output = A::Output;

    fn call((a, b): (A, B)) -> Self::Output {
        a - b
    }
}
