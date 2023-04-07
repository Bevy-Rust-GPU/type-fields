use crate::hlist::cons::{ConsListMut, Uncons, ConsList};

use super::{tuple_list_ref::TupleListRef, Cons};

pub trait TupleListMut<'a>: TupleListRef<'a> {
    type HeadMut: 'a;
    type TailMut: TupleListMut<'a>;

    fn head_mut(self) -> Self::HeadMut;
    fn tail_mut(self) -> Self::TailMut;
}

impl<'a, T> TupleListMut<'a> for T
where
    T: Cons,
    T::Cons: ConsListMut<'a>,
    <T::Cons as ConsList>::Tail: Uncons,
{
    type HeadMut = <T::Cons as ConsListMut<'a>>::HeadMut;
    type TailMut = <<T::Cons as ConsListMut<'a>>::TailMut as Uncons>::Uncons;

    fn head_mut(self) -> Self::HeadMut {
        self.cons().head_mut()
    }

    fn tail_mut(self) -> Self::TailMut {
        self.cons().tail_mut().uncons()
    }
}

#[cfg(test)]
mod tests {
    use super::TupleListMut;

    #[test]
    fn test_tuple_list_mut() {
        let _foo =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as TupleListMut>::HeadMut>();
        let _bar =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as TupleListMut>::TailMut>();
    }
}
