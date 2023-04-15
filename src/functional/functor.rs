use core::marker::PhantomData;

use crate::{functional::{Compose, Const, CurriedA, Curry, Id, Closure}, derive_closure};

use super::{Composed, Function};

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
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fmap;

impl<F, A> Function<(A, F)> for Fmap
where
    A: Functor<F>,
{
    type Output = A::Mapped;

    fn call((a, f): (A, F)) -> Self::Output {
        a.fmap(f)
    }
}

derive_closure!(Fmap);

pub trait FunctorReplace<T>: Sized + Functor<CurriedA<Const, T>> {
    fn replace(self, t: T) -> Self::Mapped {
        self.fmap(Const.curry_a(t))
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

    fn call((a, t): (A, T)) -> Self::Output {
        a.fmap(Const.curry_a(t))
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

pub fn test_functor_laws<F, F1, F2, G>(f: F, f1: F1, f2: F2)
where
    F: core::fmt::Debug
        + Clone
        + PartialEq
        + Functor<Id, Mapped = F>
        + Functor<F1>
        + Functor<F2>
        + Functor<Composed<F1, F2>, Mapped = G>,
    <F as Functor<Composed<F1, F2>>>::Mapped: core::fmt::Debug,
    <F as Functor<F1>>::Mapped: core::fmt::Debug + Functor<F2, Mapped = G>,
    <<F as Functor<F1>>::Mapped as Functor<F2>>::Mapped: core::fmt::Debug + PartialEq,
    F1: Clone,
    F2: Clone,
{
    test_functor_identity(f.clone());
    test_functor_composition(f, f1, f2)
}

pub fn test_functor_identity<F>(f: F)
where
    F: core::fmt::Debug + Clone + Functor<Id, Mapped = F>,
    F::Mapped: core::fmt::Debug + PartialEq,
{
    assert_eq!(f.clone().fmap(Id), Closure::call(Id, f));
}

pub fn test_functor_composition<F, F1, F2, G>(f: F, f1: F1, f2: F2)
where
    F: Clone + Functor<F1> + Functor<F2> + Functor<Composed<F1, F2>, Mapped = G>,
    F1: Clone,
    F2: Clone,
    <F as Functor<F1>>::Mapped: Functor<F2, Mapped = G>,
    <<F as Functor<F1>>::Mapped as Functor<F2>>::Mapped: core::fmt::Debug + PartialEq,
    <F as Functor<Composed<F1, F2>>>::Mapped: core::fmt::Debug,
{
    assert_eq!(
        f.clone().fmap(f1.clone()).fmap(f2.clone()),
        f.fmap(f1.compose(f2))
    )
}
