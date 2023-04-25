use crate::macros::{
    category::{Compose, Id},
    Closure,
};

use crate::t_funk::Function;

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Id, Compose,
)]
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
