use crate::macros::{arrow::Arrow, category::Category, Copointed, Pointed};

use crate::t_funk::Closure;

/// Utility trait for currying a binary function
pub trait Curry2: Sized {
    /// Convert `F(A, B) -> *` into `F(A) -> F(B) -> *`
    fn curry2(self) -> Curried2<Self> {
        Curried2(self)
    }

    /// Curry `A` into `F(A, B) -> *` to produce `F(B) -> *`
    fn prefix2<A>(self, a: A) -> Curry2A<Self, A> {
        Curry2A(self, a)
    }

    /// Curry `B` into `F(A, B) -> *` to produce `F(A) -> *`
    fn suffix2<B>(self, b: B) -> Curry2B<Self, B> {
        Curry2B(self, b)
    }
}

impl<T> Curry2 for T {}

/// Curried binary function
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
    Category,
    Arrow,
)]
pub struct Curried2<F>(pub F);

impl<F, A> Closure<A> for Curried2<F> {
    type Output = Curry2A<F, A>;

    fn call(self, input: A) -> Self::Output {
        Curry2A(self.0, input)
    }
}

/// Curried binary function with F, A stored
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct Curry2A<F, A>(pub F, pub A);

impl<F, A, B> Closure<B> for Curry2A<F, A>
where
    F: Closure<(A, B)>,
{
    type Output = F::Output;

    fn call(self, input: B) -> Self::Output {
        self.0.call((self.1, input))
    }
}

/// Curried binary function with F, B stored
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct Curry2B<F, B>(pub F, pub B);

impl<F, B, A> Closure<A> for Curry2B<F, B>
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
    use crate::t_funk::{Add, Closure, Curry2};

    #[test]
    fn test_curry() {
        let curried = Add.curry2();
        let curried = curried.call(1);
        let curried = curried.call(1);
        assert_eq!(curried, 2);
    }
}
