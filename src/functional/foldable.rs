use super::{Functor, SemigroupConcat};

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
