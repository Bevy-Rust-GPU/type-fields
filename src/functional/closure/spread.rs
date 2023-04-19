use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_monad, derive_pointed,
    functional::{Closure, Pointed},
};

/// Feed one argument into both inputs of an arity 2 function
pub trait Spread: Sized {
    fn spread(self) -> Spreaded<Self>;
}

impl<T> Spread for T {
    fn spread(self) -> Spreaded<Self> {
        Spreaded::point(self)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Spreaded<F>(F);

derive_pointed!(Spreaded<F>);
derive_copointed!(Spreaded<F>);
derive_functor!(Spreaded<F>);
derive_applicative!(Spreaded<F>);
derive_monad!(Spreaded<F>);

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
