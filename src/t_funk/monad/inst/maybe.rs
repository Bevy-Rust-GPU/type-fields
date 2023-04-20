use type_fields_macros::{Apply, Copointed, Fmap, Monad, Pointed};

use crate::t_funk::{Applicative, Apply, Chain, Copointed, Fmap, Functor, Monad, Pointed};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nothing;

impl Pointed for Nothing {
    type Pointed = Nothing;

    fn point(_: Self::Pointed) -> Self {
        Nothing
    }
}

impl Copointed for Nothing {
    type Copointed = Nothing;

    fn copoint(self) -> Self::Copointed {
        Nothing
    }
}

impl<F> Fmap<F> for Nothing {
    type Fmap = Nothing;

    fn fmap(self, _: F) -> Self::Fmap {
        Nothing
    }
}

impl<T> Apply<T> for Nothing {
    type Apply = T;

    fn apply(self, a: T) -> Self::Apply {
        a
    }
}

impl<F> Chain<F> for Nothing {
    type Chain = Nothing;

    fn chain(self, _: F) -> Self::Chain {
        Nothing
    }
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Fmap,
    Apply,
    Monad,
)]
pub struct Just<T>(T);

pub trait Maybe<T>: Pointed + Copointed + Functor + Applicative + Monad {}
impl<T> Maybe<T> for Nothing {}
impl<T> Maybe<T> for Just<T> {}

#[cfg(test)]
mod test {
    use crate::t_funk::{test_functor_laws, Add, Closure, Curry, CurryN, Fmap, Mul, Pointed};

    use super::{Just, Nothing};

    #[test]
    fn test_maybe() {
        let nothing = Nothing;
        let mapped = nothing.fmap(CurryN::<(i32, i32)>::curry_n(Add).call(1));
        assert_eq!(mapped, Nothing);

        let just = Just::point(2);
        let mapped = just.fmap(Add.curry_n().call(1));
        assert_eq!(mapped, Just(3));
    }

    #[test]
    fn test_functor_laws_maybe() {
        test_functor_laws(Nothing, Add.curry_a(2), Mul.curry_a(2));
        test_functor_laws(Just::point(1), Add.curry_a(2), Mul.curry_a(2));
    }
}
