use crate::macros::functions;

#[functions]
pub trait FoldMap<F> {
    type FoldMap;

    fn fold_map(self, f: F) -> Self::FoldMap;
}
