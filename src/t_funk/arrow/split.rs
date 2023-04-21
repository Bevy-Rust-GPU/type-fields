use crate::t_funk::Closure;

pub trait Split<G>: Sized {
    fn split(self, g: G) -> Splitted<Self, G>;
}

impl<T, G> Split<G> for T {
    fn split(self, g: G) -> Splitted<Self, G> {
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

