use super::{Compose, Composed, Copointed, Endo, Function, Functor, Id, Point, SemigroupConcat};

pub trait Foldable<F> {
    type Folded;

    fn fold_map(self, f: F) -> Self::Folded;
}

impl<T, F> Foldable<F> for T
where
    T: Functor<F>,
    T::Mapped: SemigroupConcat,
{
    type Folded = <T::Mapped as SemigroupConcat>::Concatenated;

    fn fold_map(self, f: F) -> Self::Folded {
        self.fmap(f).mconcat()
    }
}

pub trait Foldr<F, I1, I2> {
    type Foldr;

    fn foldr(self, f: F, z: I2) -> Self::Foldr;
}

impl<T, F, I1, I2> Foldr<F, I1, I2> for T
where
    T: Foldable<Composed<F, Point<Endo<F::Output>>>>,
    F: Function<I1>,
    T::Folded: Copointed,
    <T::Folded as Copointed>::Copointed: Function<I2>,
{
    type Foldr = <<T::Folded as Copointed>::Copointed as Function<I2>>::Output;

    fn foldr(self, f: F, z: I2) -> Self::Foldr {
        self.fold_map(Compose.call((f, Point::<Endo<F::Output>>::default())))
            .copoint()
            .call(z)
    }
}

pub trait Fold: Foldable<Id> {
    fn fold(self) -> Self::Folded;
}

impl<T> Fold for T
where
    T: Foldable<Id>,
{
    fn fold(self) -> Self::Folded {
        self.fold_map(Id)
    }
}
