use crate::hlist::cons::{ConsClone, Uncons};

use super::{tuple_list_ref::TupleListRef, TupleList};

pub trait TupleClone<'a>: TupleListRef<'a> {
    type TupleClone: TupleList;

    fn tuple_clone(self) -> Self::TupleClone;
}

impl<'a, T> TupleClone<'a> for T
where
    T: TupleListRef<'a>,
    T::Cons: ConsClone<'a>,
{
    type TupleClone = <<T::Cons as ConsClone<'a>>::ConsClone as Uncons>::Uncons;

    fn tuple_clone(self) -> Self::TupleClone {
        self.cons().cons_clone().uncons()
    }
}

