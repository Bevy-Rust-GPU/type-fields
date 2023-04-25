use type_fields_macros::{Fanout, Split};

use crate::{
    macros::{functions, Arr, Compose, Copointed, First, Id, Pointed, Second},
    t_funk::{Closure, Pointed},
};

/// Feed one argument into both inputs of an arity 2 function
#[functions]
pub trait Spread: Sized {
    fn spread(self) -> Spreaded<Self>;
}

impl<T> Spread for T {
    fn spread(self) -> Spreaded<Self> {
        Spreaded::point(self)
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
pub struct Spreaded<F>(F);

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
