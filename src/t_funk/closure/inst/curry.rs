use type_fields_macros::{Compose, First, Id, Second, Pointed, Copointed};

use crate::t_funk::{Closure, Flip, Flipped};

/// Utility trait for constructing a CurryA from a Function<(A, B)>
pub trait Curry: Sized {
    fn curry(self) -> Curried<Self>;

    fn curry_a<A>(self, a: A) -> CurriedA<Self, A> {
        self.curry().call(a)
    }

    fn curry_b<B>(self, a: B) -> CurriedA<Flipped<Self>, B> {
        self.flip().curry_a(a)
    }
}

impl<T> Curry for T {
    fn curry(self) -> Curried<Self> {
        Curried(self)
    }
}

/// Curry function with F stored
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
    Id,
    Compose,
    First,
    Second,
)]
pub struct Curried<F>(pub F);

impl<F, A> Closure<A> for Curried<F> {
    type Output = CurriedA<F, A>;

    fn call(self, input: A) -> Self::Output {
        CurriedA(self.0, input)
    }
}

/// Curry function with F, A stored
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Id, Compose, First, Second,
)]
pub struct CurriedA<F, A>(F, A);

impl<F, A, B> Closure<B> for CurriedA<F, A>
where
    F: Closure<(A, B)>,
{
    type Output = F::Output;

    fn call(self, input: B) -> Self::Output {
        self.0.call((self.1, input))
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{Add, Closure, Curry};

    #[test]
    fn test_curry() {
        let curried = Add.curry();
        let curried = curried.call(1);
        let curried = curried.call(1);
        assert_eq!(curried, 2);
    }
}
