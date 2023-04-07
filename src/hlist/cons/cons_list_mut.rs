use super::ConsListRef;

/// A cons list of mutable references
pub trait ConsListMut<'a>: ConsListRef<'a> {
    type HeadMut: 'a;
    type TailMut: ConsListMut<'a>;

    fn head_mut(self) -> Self::HeadMut;
    fn tail_mut(self) -> Self::TailMut;
}

impl<'a, Head, Tail> ConsListMut<'a> for (&'a mut Head, Tail)
where
    Self: ConsListRef<'a>,
    Tail: ConsListMut<'a>,
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

impl ConsListMut<'_> for () {
    type HeadMut = ();
    type TailMut = ();

    fn head_mut(self) -> Self::HeadMut {
        ()
    }

    fn tail_mut(self) -> Self::TailMut {
        ()
    }
}
