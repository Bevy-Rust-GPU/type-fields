use crate::t_funk::hlist::{AsRef, ToTList};

use super::{tlist_ref::TListRef, TList};

pub trait AsTListRef<'a>: TList {
    type AsHListRef: TListRef<'a>;
}

impl<'a, T> AsTListRef<'a> for T
where
    T: TList,
    T::HList: AsRef<'a>,
{
    type AsHListRef = <<T::HList as AsRef<'a>>::AsRef as ToTList>::TList;
}
