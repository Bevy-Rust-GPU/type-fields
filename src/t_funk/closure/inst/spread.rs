use crate::macros::arrow::{Fanout, Split};

use crate::{
    macros::{
        arrow::{Arr, First, Second},
        category::{Compose, Id},
        functions, Copointed, Pointed,
    },
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
    Id,
    Compose,
    Arr,
    First,
    Second,
    Split,
    Fanout,
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
