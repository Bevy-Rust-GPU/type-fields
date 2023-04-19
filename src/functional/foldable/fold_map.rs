use type_fields_macros::functions;

use crate::functional::{SemigroupConcat, Fmap};

#[functions]
pub trait FoldMap<F>: Fmap<F> {
    type FoldMap;

    fn fold_map(self, f: F) -> Self::FoldMap;
}

impl<T, F> FoldMap<F> for T
where
    T: Fmap<F>,
    T::Fmap: SemigroupConcat,
{
    type FoldMap = <T::Fmap as SemigroupConcat>::Concatenated;

    fn fold_map(self, f: F) -> Self::FoldMap {
        self.fmap(f).mconcat()
    }
}

