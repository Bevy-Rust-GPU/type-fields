use core::marker::PhantomData;

use crate::functional::{Const, Pointed};

use super::Function;

/// A type that can map a function over its wrapped value
pub trait Functor<A> {
    type Mapped<B>: Functor<B>;

    /// `<$>`
    fn fmap<B, F>(self, f: F) -> <Self as Functor<A>>::Mapped<B>
    where
        F: Function<A, Output = B>;

    /// `<$`
    fn replace<B>(self, t: B) -> <Self as Functor<A>>::Mapped<B>
    where
        Self: Sized,
        Const<B>: Function<A, Output = B>,
    {
        self.fmap(Const::point(t))
    }
}

/// Functor::fmap
pub struct Fmap<A, B>(PhantomData<(A, B)>);

impl<A, B> Clone for Fmap<A, B> {
    fn clone(&self) -> Self {
        Fmap(self.0.clone())
    }
}

impl<T, F, A, B, O> Function<(T, F)> for Fmap<A, B>
where
    T: Functor<A, Mapped<B> = O>,
    F: Function<A, Output = B>,
{
    type Output = O;

    fn call(self, (t, f): (T, F)) -> Self::Output {
        t.fmap(f)
    }
}

/// Functor::replace
pub struct Replace<A>(PhantomData<A>);

impl<A> Clone for Replace<A> {
    fn clone(&self) -> Self {
        Replace(self.0)
    }
}

impl<T, I, A, O> Function<(T, I)> for Replace<A>
where
    T: Functor<A, Mapped<I> = O>,
{
    type Output = O;

    fn call(self, (t, i): (T, I)) -> O {
        t.replace(i)
    }
}

/// A type that can emplace itself within a functor
pub trait FunctorEmplace: Sized {
    /// `$>`
    fn emplace<A, F>(self, f: F) -> <F as Functor<A>>::Mapped<Self>
    where
        F: Functor<A>,
    {
        f.fmap(Const::point(self))
    }
}

impl<T> FunctorEmplace for T {}
