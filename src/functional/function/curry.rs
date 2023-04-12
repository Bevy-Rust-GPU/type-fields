use core::marker::PhantomData;

use crate::functional::{Copointed, Pointed};

use super::Function;

/// Utility trait for constructing a CurryA from a Function<(A, B)>
pub trait Curry<A, B>: Function<(A, B)> {
    type Curried;

    fn curry(self) -> Self::Curried;
}

impl<T, A, B> Curry<A, B> for T
where
    T: Function<(A, B)>,
{
    type Curried = CurryA<T, A, B>;

    fn curry(self) -> Self::Curried {
        CurryA::point(self)
    }
}

/// Base curry function
pub struct CurryF<F, A, B>(PhantomData<(F, A, B)>);

impl<F, A, B> Default for CurryF<F, A, B> {
    fn default() -> Self {
        CurryF(PhantomData)
    }
}

impl<F, A, B> Clone for CurryF<F, A, B> {
    fn clone(&self) -> Self {
        CurryF(PhantomData)
    }
}

impl<F, A, B> Function<F> for CurryF<F, A, B>
where
    F: Function<(A, B)>,
{
    type Output = CurryA<F, A, B>;

    fn call(self, input: F) -> Self::Output {
        CurryA::point(input)
    }
}

/// Curry function with F stored
pub struct CurryA<F, A, B>(F, PhantomData<(A, B)>);

impl<F, A, B> Default for CurryA<F, A, B>
where
    F: Default,
{
    fn default() -> Self {
        CurryA(F::default(), PhantomData)
    }
}

impl<F, A, B> Clone for CurryA<F, A, B>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        CurryA(self.0.clone(), PhantomData)
    }
}

impl<F, A, B> Pointed for CurryA<F, A, B> {
    type Pointed = F;

    fn point(unit: Self::Pointed) -> Self {
        CurryA(unit, PhantomData)
    }
}

impl<F, A, B> Copointed for CurryA<F, A, B> {
    type Copointed = F;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

impl<F, A, B> Function<A> for CurryA<F, A, B> {
    type Output = CurryB<F, A, B>;

    fn call(self, input: A) -> Self::Output {
        CurryB::point((self.0, input))
    }
}

/// Curry function with F, A stored
pub struct CurryB<F, A, B>(F, A, PhantomData<B>);

impl<F, A, B> Pointed for CurryB<F, A, B> {
    type Pointed = (F, A);

    fn point((f, a): Self::Pointed) -> Self {
        CurryB(f, a, PhantomData)
    }
}

impl<F, A, B> Copointed for CurryB<F, A, B> {
    type Copointed = (F, A);

    fn copoint(self) -> Self::Copointed {
        (self.0, self.1)
    }
}

impl<F, A, B> Function<B> for CurryB<F, A, B>
where
    F: Function<(A, B)>,
{
    type Output = F::Output;

    fn call(self, input: B) -> Self::Output {
        self.0.call((self.1, input))
    }
}

#[cfg(test)]
mod test {
    use crate::functional::{Add, Function};

    use super::Curry;

    #[test]
    fn test_curry() {
        let curried = Add.curry();
        let curried = curried.call(1);
        let curried = curried.call(1);
        assert_eq!(curried, 2);
    }
}
