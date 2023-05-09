use crate::macros::functions;

/// A type that can map a function over a wrapped value.
#[functions]
pub trait Fmap<F>: Sized {
    type Fmap;

    fn fmap(self, f: F) -> Self::Fmap;
}

impl<F> Fmap<F> for () {
    type Fmap = ();

    fn fmap(self, _: F) -> Self::Fmap {
        ()
    }
}
