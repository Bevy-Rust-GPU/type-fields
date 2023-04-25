use crate::macros::{
    arrow::{Arr, Fanout, First, Second, Split},
    category::{Compose, Id},
    Closure,
};

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
pub struct Div;

impl<A, B> Function<(A, B)> for Div
where
    A: core::ops::Div<B>,
{
    type Output = A::Output;

    fn call((a, b): (A, B)) -> Self::Output {
        a / b
    }
}
