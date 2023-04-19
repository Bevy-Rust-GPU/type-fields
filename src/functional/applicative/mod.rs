mod apply;
mod lift_a2;
mod pure;

pub use apply::*;
pub use lift_a2::*;
pub use pure::*;

use crate::functional::Functor;

pub trait Applicative: Functor {
    type Pure<V>
    where
        Self: Pure;

    type Applied<F>
    where
        Self: Apply<F>;

    fn pure<U>(v: U) -> <Self as Pure>::Pure<U>
    where
        Self: Pure;

    fn apply<U>(self, u: U) -> Self::Applied<U>
    where
        Self: Apply<U>;
}

impl<T> Applicative for T {
    type Applied<U> = T::Apply
    where
        T: Apply<U>;

    type Pure<U> = T::Pure<U>
    where
        T: Pure;

    fn pure<U>(v: U) -> Self::Pure<U>
    where
        T: Pure,
    {
        <T as Pure>::pure(v)
    }

    fn apply<U>(self, u: U) -> Self::Applied<U>
    where
        Self: Apply<U>,
    {
        <T as Apply<U>>::apply(self, u)
    }
}
