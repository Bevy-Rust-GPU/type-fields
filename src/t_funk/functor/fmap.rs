use core::marker::PhantomData;

use type_fields_macros::Closure;

use crate::t_funk::{Closure, Copointed, Function, Pointed};

/// A type that can map a function over a wrapped value.
pub trait Fmap<F>: Sized {
    type Fmap;

    fn fmap(self, f: F) -> Self::Fmap;
}

impl<F> Fmap<F> for () {
    type Fmap = ();

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

#[derive(Closure)]
pub struct FmapF<M>(PhantomData<M>);

impl<M> Default for FmapF<M> {
    fn default() -> Self {
        FmapF(PhantomData)
    }
}

impl<M> Clone for FmapF<M> {
    fn clone(&self) -> Self {
        FmapF(PhantomData)
    }
}

impl<M> Copy for FmapF<M> {}

impl<M, F, T> Function<(T, F)> for FmapF<M>
where
    T: Copointed,
    F: Closure<T::Copointed>,
    M: Pointed<Pointed = F::Output>,
{
    type Output = M;

    fn call((t, f): (T, F)) -> Self::Output {
        M::point(f.call(Copointed::copoint(t)))
    }
}
