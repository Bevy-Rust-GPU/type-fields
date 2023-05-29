use crate::t_funk::hlist::{HListMut, ToTList, HList};

use super::{tlist_ref::TListRef, ToHList};

pub trait TListMut<'a>: TListRef<'a> {
    type HeadMut: 'a;
    type TailMut: TListMut<'a>;

    fn head_mut(self) -> Self::HeadMut;
    fn tail_mut(self) -> Self::TailMut;
}

impl<'a, T> TListMut<'a> for T
where
    T: ToHList,
    T::HList: HListMut<'a>,
    <T::HList as HList>::Tail: ToTList,
{
    type HeadMut = <T::HList as HListMut<'a>>::HeadMut;
    type TailMut = <<T::HList as HListMut<'a>>::TailMut as ToTList>::TList;

    fn head_mut(self) -> Self::HeadMut {
        self.to_hlist().head_mut()
    }

    fn tail_mut(self) -> Self::TailMut {
        self.to_hlist().tail_mut().to_tlist()
    }
}

#[cfg(test)]
mod tests {
    use super::TListMut;

    #[test]
    fn test_tuple_list_mut() {
        let _foo =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as TListMut>::HeadMut>();
        let _bar =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as TListMut>::TailMut>();
    }
}
