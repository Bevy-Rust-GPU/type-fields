use type_fields_macros::functions;

use crate::t_funk::Fmap;

#[functions]
pub trait FoldMap<F>: Fmap<F> {
    type FoldMap;

    fn fold_map(self, f: F) -> Self::FoldMap;
}
