use crate::macros::{arrow::Arrow, category::Category, Closure};

use crate::t_funk::Function;

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct Id;

impl<T> Function<T> for Id {
    type Output = T;

    fn call(input: T) -> Self::Output {
        input
    }
}
