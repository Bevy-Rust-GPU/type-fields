use crate::functional::{Copointed, Pointed};

use super::Function;

pub struct Compose;

impl<F1, F2> Function<(F1, F2)> for Compose {
    type Output = Composed<F1, F2>;

    fn call(self, input: (F1, F2)) -> Self::Output {
        Composed::point(input)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Composed<F1, F2>(F1, F2);

impl<F1, F2> Pointed for Composed<F1, F2> {
    type Pointed = (F1, F2);

    fn point(unit: Self::Pointed) -> Self {
        Composed(unit.0, unit.1)
    }
}

impl<F1, F2> Copointed for Composed<F1, F2> {
    type Copointed = (F1, F2);

    fn copoint(self) -> Self::Copointed {
        (self.0, self.1)
    }
}

impl<F1, F2, A> Function<A> for Composed<F1, F2>
where
    F1: Function<A>,
    F2: Function<F1::Output>,
{
    type Output = F2::Output;

    fn call(self, a: A) -> Self::Output {
        self.1.call(self.0.call(a))
    }
}
