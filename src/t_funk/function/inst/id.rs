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
pub struct Id;

impl<T> Function<T> for Id {
    type Output = T;

    fn call(input: T) -> Self::Output {
        input
    }
}
