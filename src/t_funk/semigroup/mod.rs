mod inst;
mod mappend;

pub use inst::*;
pub use mappend::*;

pub trait Semigroup {
    type Appended<V>
    where
        Self: Mappend<V>;

    fn mappend<V>(self, v: V) -> Self::Appended<V>
    where
        Self: Mappend<V>;
}

impl<T> Semigroup for T {
    type Appended<V> = T::Mappend where T: Mappend<V>;

    fn mappend<V>(self, v: V) -> Self::Appended<V>
    where
        T: Mappend<V>,
    {
        self.mappend(v)
    }
}
