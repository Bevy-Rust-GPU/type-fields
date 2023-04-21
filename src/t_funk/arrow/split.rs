use type_fields_macros::functions;

use crate::t_funk::Closure;

#[functions]
pub trait Split<G>: Sized {
    type Split;

    fn split(self, g: G) -> Self::Split;
}

impl<T, G> Split<G> for T {
    type Split = Splitted<Self, G>;

    fn split(self, g: G) -> Self::Split {
        Splitted(self, g)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
