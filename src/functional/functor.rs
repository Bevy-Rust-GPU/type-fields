use core::marker::PhantomData;

use crate::functional::{Const, Pointed};

use super::Function;

/// A type that can map a function over a contained value.
pub trait Functor<F> {
    type Mapped;

    fn fmap(self, f: F) -> Self::Mapped;
}

impl<F> Functor<F> for () {
    type Mapped = ();

    fn fmap(self, f: F) -> Self::Mapped {
        self
    }
}

/// Functor::fmap
pub struct Fmap<F>(PhantomData<F>);

impl<F, A> Function<(A, F)> for Fmap<F>
where
    A: Functor<F>,
    F: Function<A>,
{
    type Output = A::Mapped;

    fn call(self, (a, f): (A, F)) -> Self::Output {
        a.fmap(f)
    }
}

pub trait FunctorReplace<T>: Sized + Functor<Const<T>> {
    fn replace(self, t: T) -> Self::Mapped {
        self.fmap(Const::point(t))
    }
}

impl<T, U> FunctorReplace<U> for T where T: Functor<Const<U>> {}

/// Functor::replace
pub struct Replace<T>(PhantomData<T>);

impl<A, T> Function<(A, T)> for Replace<T>
where
    A: Functor<Const<T>>,
{
    type Output = A::Mapped;

    fn call(self, (a, t): (A, T)) -> Self::Output {
        a.fmap(Const::point(t))
    }
}

/// A type that can emplace itself within a functor
pub trait FunctorEmplace<F>: Sized {
    /// `$>`
    fn emplace(self, f: F) -> F::Mapped
    where
        F: Functor<Const<Self>>,
    {
        f.replace(self)
    }
}

impl<T, F> FunctorEmplace<F> for T {}
