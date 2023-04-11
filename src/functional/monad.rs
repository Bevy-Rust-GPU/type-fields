use core::marker::PhantomData;

use super::{Const, Function, Pointed};

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

    fn call(self, (a, f): (A, F)) -> Self::Output {
        a.chain(f)
    }
}

pub trait Then<F>: Sized + Monad<Const<F::Output>>
where
    F: Function<()>,
{
    fn then(self, f: F) -> Self::Chained {
        self.chain(Const::point(f.call(())))
    }
}

impl<T, F> Then<F> for T
where
    T: Monad<Const<F::Output>>,
    F: Function<()>,
{
}
