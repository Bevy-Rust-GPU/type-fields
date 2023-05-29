use super::{Cons, HListRef, Nil};

/// A HList of mutable references
pub trait HListMut<'a>: HListRef<'a> {
    type HeadMut: 'a;
    type TailMut: HListMut<'a>;

    fn head_mut(self) -> Self::HeadMut;
    fn tail_mut(self) -> Self::TailMut;
}

impl<'a, Head, Tail> HListMut<'a> for Cons<&'a mut Head, Tail>
where
    Self: HListRef<'a>,
    Tail: HListMut<'a>,
{
    type HeadMut = &'a mut Head;
    type TailMut = Tail;

    fn head_mut(self) -> Self::HeadMut {
        self.0
    }

    fn tail_mut(self) -> Self::TailMut {
        self.1
    }
}

impl HListMut<'_> for Nil {
    type HeadMut = Self;
    type TailMut = Self;

    fn head_mut(self) -> Self::HeadMut {
        self
    }

    fn tail_mut(self) -> Self::TailMut {
        self
    }
}
