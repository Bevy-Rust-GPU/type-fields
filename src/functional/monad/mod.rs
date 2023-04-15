mod identity;
mod maybe;
mod pair;
mod state;
mod tagged;

pub use identity::*;
pub use maybe::*;
pub use pair::*;
pub use state::*;
pub use tagged::*;

use core::marker::PhantomData;

use super::{Const, CurriedA, Curry, Function, Closure};

/// A type that can flat-map a function over its wrapped value
///
/// To be definition-correct, `Monad` types must also implement `Applicative`,
/// but this cannot be strongly modeled without higher-ranked type bounds.
pub trait Monad<F> {
    type Chained;

    fn chain(self, f: F) -> Self::Chained;
}

impl<F> Monad<F> for () {
    type Chained = ();

    fn chain(self, _: F) -> Self::Chained {
        self
    }
}

/// Monad::chain
struct Chain<F>(PhantomData<F>);

impl<F, A> Function<(A, F)> for Chain<F>
where
    A: Monad<F>,
{
    type Output = A::Chained;

    fn call((a, f): (A, F)) -> Self::Output {
        a.chain(f)
    }
}

pub trait Then<F>: Sized + Monad<CurriedA<Const, F::Output>>
where
    F: Closure<()>,
{
    fn then(self, f: F) -> Self::Chained;
}

impl<T, F> Then<F> for T
where
    T: Monad<CurriedA<Const, F::Output>>,
    F: Closure<()>,
{
    fn then(self, f: F) -> Self::Chained {
        self.chain(Const.curry_a(f.call(())))
    }
}
