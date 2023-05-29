use crate::{
    macros::functions,
    t_funk::{closure::OutputT, Closure},
};

/// A type that can map a function over a wrapped value.
#[functions]
pub trait Fmap<F>: Sized {
    type Fmap;

    fn fmap(self, f: F) -> Self::Fmap;
}

pub type FmapT<T, F> = <T as Fmap<F>>::Fmap;

impl<F> Fmap<F> for () {
    type Fmap = ();

    fn fmap(self, _: F) -> Self::Fmap {
        ()
    }
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
impl<T, F> Fmap<F> for alloc::vec::Vec<T>
where
    F: Clone + Closure<T>,
{
    type Fmap = alloc::vec::Vec<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        self.into_iter().map(|t| f.clone().call(t)).collect()
    }
}
