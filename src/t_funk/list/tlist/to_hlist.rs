use crate::t_funk::hlist::{HList, Nil};

/// Convert a flat tuple into a cons list.
/// ex. `(1, 2, 3, 4)` -> `(1, (2, (3, (4, ()))))`
pub trait ToHList {
    type HList: HList<TList = Self>;

    fn to_hlist(self) -> Self::HList;
}

impl ToHList for () {
    type HList = Nil;

    fn to_hlist(self) -> Self::HList {
        Nil
    }
}
