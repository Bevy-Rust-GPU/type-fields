use type_fields_macros::functions;

use crate::t_funk::Id;

use super::FoldMap;

#[functions]
pub trait Fold: FoldMap<Id> {
    fn fold(self) -> Self::FoldMap;
}

impl<T> Fold for T
where
    T: FoldMap<Id>,
{
    fn fold(self) -> Self::FoldMap {
        self.fold_map(Id)
    }
}

