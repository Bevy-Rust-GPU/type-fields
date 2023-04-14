use crate::{
    derive_applicative, derive_copointed, derive_functor, derive_monad, derive_pointed,
    functional::{Applicative, Copointed, Functor, Monad, Pointed},
};

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

impl<F> Functor<F> for Nothing {
    type Mapped = Nothing;

    fn fmap(self, _: F) -> Self::Mapped {
        Nothing
    }
}

impl<T> Applicative<T> for Nothing {
    type Applied = T;

    fn apply(self, a: T) -> Self::Applied {
        a
    }
}

impl<F> Monad<F> for Nothing {
    type Chained = Nothing;

    fn chain(self, _: F) -> Self::Chained {
        Nothing
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Just<T>(T);

derive_pointed!(Just<T>);
derive_copointed!(Just<T>);
derive_functor!(Just<T>);
derive_applicative!(Just<T>);
derive_monad!(Just<T>);

pub trait Maybe {}
impl Maybe for Nothing {}
impl<T> Maybe for Just<T> {}

#[cfg(test)]
mod test {
    use crate::functional::{Add, CurryN, Function, Functor, Pointed};

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
}
