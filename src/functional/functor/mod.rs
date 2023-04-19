mod emplace;
mod fmap;
mod laws;
mod replace;

pub use emplace::*;
pub use fmap::*;
pub use laws::*;
pub use replace::*;

pub trait Functor {
    type Mapped<F>
    where
        Self: Fmap<F>;

    type Replaced<V>
    where
        Self: Replace<V>;

    fn fmap<F>(self, f: F) -> Self::Mapped<F>
    where
        Self: Fmap<F>;

    fn replace<V>(self, t: V) -> Self::Replaced<V>
    where
        Self: Replace<V>;
}

impl<T> Functor for T {
    type Mapped<F> = T::Fmap where T: Fmap<F>;
    type Replaced<V> = T::Fmap where T: Replace<V>;

    fn fmap<F>(self, f: F) -> Self::Mapped<F>
    where
        T: Fmap<F>,
    {
        Fmap::<F>::fmap(self, f)
    }

    fn replace<V>(self, t: V) -> Self::Replaced<V>
    where
        T: Replace<V>,
    {
        Replace::<V>::replace(self, t)
    }
}
