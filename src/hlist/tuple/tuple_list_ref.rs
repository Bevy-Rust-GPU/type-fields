use crate::hlist::cons::{ConsListRef, Uncons};

use super::{Cons, TupleList};

pub trait TupleListRef<'a>: TupleList {
    type HeadRef: 'a;
    type TailRef: TupleListRef<'a>;

    fn head_ref(self) -> Self::HeadRef;
    fn tail_ref(self) -> Self::TailRef;
}

impl<'a, T> TupleListRef<'a> for T
where
    T: Cons,
    T::Cons: ConsListRef<'a>,
{
    type HeadRef = <T::Cons as ConsListRef<'a>>::HeadRef;
    type TailRef = <<T::Cons as ConsListRef<'a>>::TailRef as Uncons>::Uncons;

    fn head_ref(self) -> Self::HeadRef {
        self.cons().head_ref()
    }

    fn tail_ref(self) -> Self::TailRef {
        self.cons().tail_ref().uncons()
    }
}

#[cfg(test)]
mod tests {
    use super::TupleListRef;

    #[test]
    fn test_tuple_list_ref() {
        let _foo = core::any::type_name::<<(&usize, &f32, &&str) as TupleListRef>::HeadRef>();
        let _bar = core::any::type_name::<<(&usize, &f32, &&str) as TupleListRef>::TailRef>();

        let _foo =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as TupleListRef>::HeadRef>();
        let _bar =
            core::any::type_name::<<(&mut usize, &mut f32, &mut &str) as TupleListRef>::TailRef>();
    }
}
