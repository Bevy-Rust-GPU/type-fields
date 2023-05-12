use crate::macros::{arrow::Arrow, category::Category, Copointed, Pointed};

use crate::t_funk::Closure;

/// Utility trait for currying a ternary function
pub trait Curry3: Sized {
    /// Convert `F(A, B) -> *` into `F(A) -> F(B) -> *`
    fn curry3(self) -> Curried3<Self> {
        Curried3(self)
    }

    /// Curry `A` into `F(A, B) -> *` to produce `F(B) -> *`
    fn prefix3<A>(self, a: A) -> Curry3A<Self, A> {
        Curry3A(self, a)
    }
}

impl<T> Curry3 for T {}

/// Curried ternary function
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
pub struct Curried3<F>(pub F);

impl<F, A> Closure<A> for Curried3<F> {
    type Output = Curry3A<F, A>;

    fn call(self, input: A) -> Self::Output {
        Curry3A(self.0, input)
    }
}

/// Curried ternary function with F, A stored
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct Curry3A<F, A>(pub F, pub A);

impl<F, A, B> Closure<B> for Curry3A<F, A> {
    type Output = Curry3AB<F, A, B>;

    fn call(self, input: B) -> Self::Output {
        Curry3AB(self.0, self.1, input)
    }
}

/// Curried ternary function with F, A, B stored
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct Curry3AB<F, A, B>(pub F, pub A, pub B);

impl<F, A, B, C> Closure<C> for Curry3AB<F, A, B>
where
    F: Closure<(A, B, C)>,
{
    type Output = F::Output;

    fn call(self, input: C) -> Self::Output {
        self.0.call((self.1, self.2, input))
    }
}
