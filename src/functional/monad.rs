use core::marker::PhantomData;

use super::{Applicative, Const, Function, Pointed};

/// An `Applicative` type that can flat-map a function over its wrapped value
pub trait Monad<T>: Applicative<T> {
    fn chain<M, F>(self, f: F) -> M
    where
        F: Function<T, Output = M>;

    fn then<M, F>(self, f: F) -> M
    where
        Self: Sized,
        F: Function<(), Output = M>,
    {
        self.chain(Const::point(f.call(())))
    }
}

/// Monad::chain
struct Chain<T, M>(PhantomData<(T, M)>);

impl<T, M> Clone for Chain<T, M> {
    fn clone(&self) -> Self {
        Chain(self.0.clone())
    }
}

impl<T, I, F, M> Function<(I, F)> for Chain<T, M>
where
    I: Monad<T>,
    F: Function<T, Output = M>,
{
    type Output = M;

    fn call(self, (i, f): (I, F)) -> M {
        i.chain(f)
    }
}

/// Monad::then
struct Then<T>(PhantomData<T>);

impl<T> Clone for Then<T> {
    fn clone(&self) -> Self {
        Then(self.0.clone())
    }
}

impl<T, I, F, M> Function<(I, F)> for Then<T>
where
    I: Monad<T>,
    F: Function<(), Output = M>,
{
    type Output = M;

    fn call(self, (i, f): (I, F)) -> M {
        i.then(f)
    }
}
