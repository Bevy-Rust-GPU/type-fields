use super::HListRef;

/// A HList of mutable references
pub trait HListMut<'a>: HListRef<'a> {
    type HeadMut: 'a;
    type TailMut: HListMut<'a>;

    fn head_mut(self) -> Self::HeadMut;
    fn tail_mut(self) -> Self::TailMut;
}

impl<'a, Head, Tail> HListMut<'a> for (&'a mut Head, Tail)
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

impl HListMut<'_> for () {
    type HeadMut = ();
    type TailMut = ();

    fn head_mut(self) -> Self::HeadMut {
        ()
    }

    fn tail_mut(self) -> Self::TailMut {
        ()
    }
}
