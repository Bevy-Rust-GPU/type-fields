use crate::hlist::cons::{ConsAsRef, Uncons};

use super::{tuple_list_ref::TupleListRef, TupleList};

pub trait TupleAsRef<'a>: TupleList {
    type TupleAsRef: TupleListRef<'a>;
}

impl<'a, T> TupleAsRef<'a> for T
where
    T: TupleList,
    T::Cons: ConsAsRef<'a>,
{
    type TupleAsRef = <<T::Cons as ConsAsRef<'a>>::ConsAsRef as Uncons>::Uncons;
}
