use crate::{t_funk::hlist::HListRef, t_funk::{tlist::TList, hlist::Nil}};

/// Immutably borrow a HList from a flat tuple.
/// ex. `&(1, 2, 3, 4)` -> `(&1, (&2, (&3, (&4, ()))))`
pub trait AsHListRef: TList {
    type HListRef<'a>: HListRef<'a>
    where
        Self: 'a;

    fn as_hlist_ref<'a>(&'a self) -> Self::HListRef<'a>;
}

impl AsHListRef for () {
    type HListRef<'a> = Nil;

    fn as_hlist_ref<'a>(&'a self) -> Self::HListRef<'a> {
        Nil
    }
}
