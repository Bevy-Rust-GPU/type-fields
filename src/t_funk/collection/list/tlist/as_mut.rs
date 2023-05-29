use crate::t_funk::hlist::{AsMut, ToTList};

use super::{TList, TListMut};

pub trait AsTListMut<'a>: TList {
    type AsHListMut: TListMut<'a>;
}

impl<'a, T> AsTListMut<'a> for T
where
    T: TList,
    T::HList: AsMut<'a>,
{
    type AsHListMut = <<T::HList as AsMut<'a>>::AsMut as ToTList>::TList;
}
