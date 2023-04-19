use super::{HList, HListMut};

/// A cons list that can have its items taken by reference
pub trait AsMut<'a>: HList {
    type AsMut: HListMut<'a>;

    fn as_mut(&'a mut self) -> Self::AsMut;
}

impl<'a, Head, Tail> AsMut<'a> for (Head, Tail)
where
    Self: HList,
    Head: 'a,
    Tail: AsMut<'a>,
    (&'a mut Head, Tail::AsMut): HListMut<'a>,
{
    type AsMut = (&'a mut Head, Tail::AsMut);

    fn as_mut(&'a mut self) -> Self::AsMut {
        (&mut self.0, self.1.as_mut())
    }
}

impl<'a> AsMut<'a> for () {
    type AsMut = ();

    fn as_mut(&'a mut self) -> Self::AsMut {
        *self
    }
}
