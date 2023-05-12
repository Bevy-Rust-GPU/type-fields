use crate::macros::{arrow::Arrow, category::Category};

use crate::{
    macros::{functions, Copointed, Pointed},
    t_funk::Closure,
};

/// Feed one argument into both inputs of an arity 2 function
#[functions]
pub trait Spread: Sized {
    fn spread(self) -> Spreaded<Self>;
}

impl<T> Spread for T {
    fn spread(self) -> Spreaded<Self> {
        Spreaded(self)
    }
}

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
pub struct Spreaded<F>(pub F);

impl<F, A> Closure<A> for Spreaded<F>
where
    F: Closure<(A, A)>,
    A: Clone,
{
    type Output = F::Output;

    fn call(self, a: A) -> Self::Output {
        self.0.call((a.clone(), a))
    }
}
