use crate::macros::{
    arrow::{Arr, Fanout, First, Second, Split},
    category::{Compose, Id},
    Copointed, Pointed,
};

use crate::t_funk::Closure;

/// Utility trait for constructing a CurryA from a Function<(A, B)>
pub trait Curry: Sized {
    /// Convert `F(A, B) -> *` into `F(A) -> F(B) -> *`
    fn curry(self) -> Curried<Self> {
        Curried(self)
    }

    /// Curry `A` into `F(A, B) -> *` to produce `F(B) -> *`
    fn prefix<A>(self, a: A) -> Prefixed<Self, A> {
        Prefixed(self, a)
    }

    /// Curry `B` into `F(A, B) -> *` to produce `F(A) -> *`
    fn suffix<B>(self, b: B) -> Suffixed<Self, B> {
        Suffixed(self, b)
    }
}

impl<T> Curry for T {}

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
    Arr,
    First,
    Second,
    Split,
    Fanout,
)]
pub struct Curried<F>(pub F);

impl<F, A> Closure<A> for Curried<F> {
    type Output = Prefixed<F, A>;

    fn call(self, input: A) -> Self::Output {
        Prefixed(self.0, input)
    }
}

/// Curry function with F, A stored
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
    Id,
    Compose,
    Arr,
    First,
    Second,
    Split,
    Fanout,
)]
pub struct Prefixed<F, A>(pub F, pub A);

impl<F, A, B> Closure<B> for Prefixed<F, A>
where
    F: Closure<(A, B)>,
{
    type Output = F::Output;

    fn call(self, input: B) -> Self::Output {
        self.0.call((self.1, input))
    }
}

/// Curry function with F, B stored
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
    Id,
    Compose,
    Arr,
    First,
    Second,
    Split,
    Fanout,
)]
pub struct Suffixed<F, B>(pub F, pub B);

impl<F, B, A> Closure<A> for Suffixed<F, B>
where
    F: Closure<(A, B)>,
{
    type Output = F::Output;

    fn call(self, input: A) -> Self::Output {
        self.0.call((input, self.1))
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
