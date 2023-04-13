use core::marker::PhantomData;

use crate::functional::{Const, Curried, CurriedA, Function, Pointed};

/// A type that can map a function over a wrapped value.
pub trait Functor<F> {
    type Mapped;

    fn fmap(self, f: F) -> Self::Mapped;
}

impl<F> Functor<F> for () {
    type Mapped = ();

    fn fmap(self, _: F) -> Self::Mapped {
        self
    }
}

/// Functor::fmap
pub struct Fmap;

impl<F, A> Function<(A, F)> for Fmap
where
    A: Functor<F>,
{
    type Output = A::Mapped;

    fn call(self, (a, f): (A, F)) -> Self::Output {
        a.fmap(f)
    }
}

pub trait FunctorReplace<T>: Sized + Functor<CurriedA<Const, T>> {
    fn replace(self, t: T) -> Self::Mapped {
        self.fmap(Curried::point(Const).call(t))
    }
}

impl<T, U> FunctorReplace<U> for T where T: Functor<CurriedA<Const, U>> {}

/// Functor::replace
pub struct Replace<T>(PhantomData<T>);

impl<A, T> Function<(A, T)> for Replace<T>
where
    A: Functor<CurriedA<Const, T>>,
{
    type Output = A::Mapped;

    fn call(self, (a, t): (A, T)) -> Self::Output {
        a.fmap(Curried::point(Const).call(t))
    }
}

/// A type that can emplace itself within a functor
pub trait FunctorEmplace<F>: Sized {
    /// `$>`
    fn emplace(self, f: F) -> F::Mapped
    where
        F: Functor<CurriedA<Const, Self>>,
    {
        f.replace(self)
    }
}

impl<T, F> FunctorEmplace<F> for T {}
