use crate::{
    macros::{arrow::Arrow, category::Category, functions, Copointed, Pointed},
    t_funk::Closure,
};

#[functions]
pub trait Second {
    type Second;

    fn second(self) -> Self::Second;
}

pub type SecondT<T> = <T as Second>::Second;

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
    Pointed,
    Copointed,
    Category,
    Arrow,
)]
pub struct Seconded<F>(pub F);

impl<F, X, Y> Closure<(X, Y)> for Seconded<F>
where
    F: Closure<Y>,
{
    type Output = (X, F::Output);

    fn call(self, (x, y): (X, Y)) -> Self::Output {
        (x, self.0.call(y))
    }
}
