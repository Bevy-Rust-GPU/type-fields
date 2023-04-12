use crate::functional::{
    Compose, Copointed, Foldable, Function, Functor, Id, Point, Pointed, Semigroup,
};

use super::Monoid;

/// The monoid of endomorphisms under composition.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Endo<T>(T);

impl<T> Pointed for Endo<T> {
    type Pointed = T;

    fn point(unit: Self::Pointed) -> Self {
        Endo(unit)
    }
}

impl<T> Copointed for Endo<T> {
    type Copointed = T;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

impl<T> Monoid for Endo<T>
where
    T: Monoid,
{
    type Identity = Endo<Id>;

    fn mempty() -> Self::Identity {
        Endo(Id)
    }
}

impl<T, U> Semigroup<U> for Endo<T> {
    type Appended = Endo<Compose<T, U>>;

    fn mappend(self, u: U) -> Self::Appended {
        Endo::point(Compose::point((self.copoint(), u)))
    }
}

struct FoldComposing;

impl<T, F> Function<(T, F)> for FoldComposing
where
    T: Foldable<Endo<F>>,
{
    type Output = T::Folded;

    fn call(self, (t, f): (T, F)) -> Self::Output {
        t.fold_map(Endo::point(f))
    }
}

