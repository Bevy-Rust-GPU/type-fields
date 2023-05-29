use crate::t_funk::hlist::{HListRef, ToTList};

use super::{ToHList, TList};

pub trait TListRef<'a>: TList {
    type HeadRef: 'a;
    type TailRef: TListRef<'a>;

    fn head_ref(self) -> Self::HeadRef;
    fn tail_ref(self) -> Self::TailRef;
}

impl<'a, T> TListRef<'a> for T
where
    T: ToHList,
    T::HList: HListRef<'a>,
{
    type HeadRef = <T::HList as HListRef<'a>>::HeadRef;
    type TailRef = <<T::HList as HListRef<'a>>::TailRef as ToTList>::TList;

    fn head_ref(self) -> Self::HeadRef {
        self.to_hlist().head_ref()
    }

    fn tail_ref(self) -> Self::TailRef {
        self.to_hlist().tail_ref().to_tlist()
    }
}

#[cfg(test)]
mod tests {
    use super::TListRef;

    #[test]
    fn test_tuple_list_ref() {
        let _foo = core::any::type_name::<<(&usize, &f32, &&str) as TListRef>::HeadRef>();
        let _bar = core::any::type_name::<<(&usize, &f32, &&str) as TListRef>::TailRef>();

        let _foo =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as TListRef>::HeadRef>();
        let _bar =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as TListRef>::TailRef>();
    }
}
