use crate::hlist::{cons::ConsListRef, tuple::TupleList};

/// Immutably borrow a cons list from a flat tuple.
/// ex. `&(1, 2, 3, 4)` -> `(&1, (&2, (&3, (&4, ()))))`
pub trait ConsRef: TupleList {
    type ConsRef<'a>: ConsListRef<'a> where Self: 'a;

    fn cons_ref<'a>(&'a self) -> Self::ConsRef<'a>;
}

impl ConsRef for () {
    type ConsRef<'a> = ();

    fn cons_ref<'a>(&'a self) -> Self::ConsRef<'a> {
        ()
    }
}
