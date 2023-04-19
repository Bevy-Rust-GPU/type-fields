use crate::t_funk::hlist::{HListClone, ToTList};

use super::{tlist_ref::TListRef, TList};

pub trait TListClone<'a>: TListRef<'a> {
    type Cloned: TList;

    fn tlist_clone(self) -> Self::Cloned;
}

impl<'a, T> TListClone<'a> for T
where
    T: TListRef<'a>,
    T::HList: HListClone<'a>,
{
    type Cloned = <<T::HList as HListClone<'a>>::Cloned as ToTList>::TList;

    fn tlist_clone(self) -> Self::Cloned {
        self.to_hlist().hlist_clone().to_tlist()
    }
}

