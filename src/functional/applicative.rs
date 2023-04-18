use core::marker::PhantomData;

use crate::derive_closure;

use super::{CurriedA, Curry, Function, Functor};

/// A type that can take a wrapped function,
/// map it over a provided value, and wrap the result
///
/// To be definition-correct, `Applicative` types must also implement `Functor`,
/// but this cannot be strongly modeled without higher-ranked type bounds.
pub trait Applicative<T> {
    type Applied;

    /// <*>
    fn apply(self, a: T) -> Self::Applied;
}

impl<T> Applicative<T> for () {
    type Applied = ();

    fn apply(self, _: T) -> Self::Applied {
        ()
    }
}

/// Applicative::apply
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Apply;

impl<U, T> Function<(U, T)> for Apply
where
    U: Applicative<T>,
{
    type Output = U::Applied;

    fn call((u, t): (U, T)) -> Self::Output {
        u.apply(t)
    }
}

derive_closure!(Apply);

pub trait Pure {
    type Pure<T>;
    fn pure<T>(t: T) -> Self::Pure<T>;
}

pub struct PureF<T>(PhantomData<T>);

impl<T> Default for PureF<T> {
    fn default() -> Self {
        PureF(PhantomData)
    }
}

derive_closure!(PureF<T>);

impl<T> Clone for PureF<T> {
    fn clone(&self) -> Self {
        PureF(PhantomData)
    }
}

impl<T, U> Function<U> for PureF<T>
where
    T: Pure,
{
    type Output = T::Pure<U>;

    fn call(input: U) -> Self::Output {
        T::pure(input)
    }
}

pub struct LiftA2;

impl<F, X> Function<(F, X)> for LiftA2
where
    X: Functor<F>,
{
    type Output = CurriedA<Apply, X::Mapped>;

    fn call((f, x): (F, X)) -> Self::Output {
        Apply.curry_a(x.fmap(f))
    }
}

derive_closure!(LiftA2);

#[cfg(test)]
mod test {
    use crate::functional::{Closure, Curry, Just, LiftA2, Pointed, Tuple};

    #[test]
    fn test_lift_a2() {
        let foo = LiftA2
            .call((Tuple.curry(), Just::point(3)))
            .call(Just::point(5));
        assert_eq!(foo, Just::point((3, 5)));
    }
}
