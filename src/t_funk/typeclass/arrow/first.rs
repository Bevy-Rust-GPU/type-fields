use crate::{
    macros::{arrow::Arrow, category::Category, functions, Copointed, Pointed},
    t_funk::Closure,
};

#[functions]
pub trait First {
    type First;

    fn first(self) -> Self::First;
}

pub type FirstT<T> = <T as First>::First;

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
pub struct Firsted<F>(pub F);

impl<F, X, Y> Closure<(X, Y)> for Firsted<F>
where
    F: Closure<X>,
{
    type Output = (F::Output, Y);

    fn call(self, (x, y): (X, Y)) -> Self::Output {
        (self.0.call(x), y)
    }
}
