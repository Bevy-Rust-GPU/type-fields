use crate::macros::{
    arrow::{Arr, Fanout, First, Second, Split},
    category::{Compose, Id},
    functions, Copointed, Pointed,
};

use crate::t_funk::Closure;

#[functions]
pub trait Split<G>: Sized {
    type Split;

    fn split(self, g: G) -> Self::Split;
}

pub type SplitT<T, G> = <T as Split<G>>::Split;

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
pub struct Splitted<F, G>(pub F, pub G);

impl<F, G, X, Y> Closure<(X, Y)> for Splitted<F, G>
where
    F: Closure<X>,
    G: Closure<Y>,
{
    type Output = (F::Output, G::Output);

    fn call(self, (x, y): (X, Y)) -> Self::Output {
        (self.0.call(x), self.1.call(y))
    }
}
