use crate::functional::{Copointed, Pointed};

use super::Function;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Compose<F1, F2>(F1, F2);

impl<F1, F2> Pointed for Compose<F1, F2> {
    type Pointed = (F1, F2);

    fn point(unit: Self::Pointed) -> Self {
        Compose(unit.0, unit.1)
    }
}

impl<F1, F2> Copointed for Compose<F1, F2> {
    type Copointed = (F1, F2);

    fn copoint(self) -> Self::Copointed {
        (self.0, self.1)
    }
}

impl<F1, F2, A> Function<(F1, F2, A)> for Compose<F1, F2>
where
    F1: Function<F2::Output>,
    F2: Function<A>,
{
    type Output = F1::Output;

    fn call(self, (f1, f2, a): (F1, F2, A)) -> Self::Output {
        f1.call(f2.call(a))
    }
}
