use crate::hlist::cons::{ConsAsMut, Uncons};

use super::{TupleList, TupleListMut};

pub trait TupleAsMut<'a>: TupleList {
    type TupleAsMut: TupleListMut<'a>;
}

impl<'a, T> TupleAsMut<'a> for T
where
    T: TupleList,
    T::Cons: ConsAsMut<'a>,
{
    type TupleAsMut = <<T::Cons as ConsAsMut<'a>>::ConsAsMut as Uncons>::Uncons;
}
