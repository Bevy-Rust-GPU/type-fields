use crate::macros::{arrow::arrow, category::category, Closure};

use crate::t_funk::Function;

#[category]
#[arrow]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct MakePair;

impl<T> Function<T> for MakePair
where
    T: Clone,
{
    type Output = (T, T);

    fn call(t: T) -> Self::Output {
        (t.clone(), t)
    }
}
