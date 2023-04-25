use crate::macros::{functions, Copointed, Pointed};

use crate::t_funk::Closure;

#[functions]
pub trait Split<G>: Sized {
    type Split;

    fn split(self, g: G) -> Self::Split;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
pub struct Splitted<F, G>(F, G);

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
