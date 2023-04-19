use type_fields_macros::functions;

use crate::functional::Id;

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

