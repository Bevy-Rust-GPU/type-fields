use crate::functional::{Copointed, Pointed};

use super::Function;

/// Utility trait for constructing a CurryA from a Function<(A, B)>
pub trait Curry {
    type Curried;

    fn curry(self) -> Self::Curried;
}

impl<T> Curry for T {
    type Curried = Curried<T>;

    fn curry(self) -> Self::Curried {
        Curried::point(self)
    }
}

/// Curry function with F stored
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Curried<F>(F);

impl<F> Pointed for Curried<F> {
    type Pointed = F;

    fn point(unit: Self::Pointed) -> Self {
        Curried(unit)
    }
}

impl<F> Copointed for Curried<F> {
    type Copointed = F;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

impl<F, A> Function<A> for Curried<F> {
    type Output = CurriedA<F, A>;

    fn call(self, input: A) -> Self::Output {
        CurriedA::point((self.0, input))
    }
}

/// Curry function with F, A stored
pub struct CurriedA<F, A>(F, A);

impl<F, A> Pointed for CurriedA<F, A> {
    type Pointed = (F, A);

    fn point((f, a): Self::Pointed) -> Self {
        CurriedA(f, a)
    }
}

impl<F, A> Copointed for CurriedA<F, A> {
    type Copointed = (F, A);

    fn copoint(self) -> Self::Copointed {
        (self.0, self.1)
    }
}

impl<F, A, B> Function<B> for CurriedA<F, A>
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
    use crate::functional::{Add, Curried, Function, Pointed};

    #[test]
    fn test_curry() {
        let curried = Curried::point(Add);
        let curried = curried.call(1);
        let curried = curried.call(1);
        assert_eq!(curried, 2);
    }
}
