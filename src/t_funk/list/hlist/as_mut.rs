use super::{HList, HListMut, Cons, Nil};

/// A cons list that can have its items taken by reference
pub trait AsMut<'a>: HList {
    type AsMut: HListMut<'a>;

    fn as_mut(&'a mut self) -> Self::AsMut;
}

impl<'a, T, N> AsMut<'a> for Cons<T, N>
where
    Self: HList,
    T: 'a,
    N: AsMut<'a>,
    (&'a mut T, N::AsMut): HListMut<'a>,
{
    type AsMut = (&'a mut T, N::AsMut);

    fn as_mut(&'a mut self) -> Self::AsMut {
        (&mut self.0, self.1.as_mut())
    }
}

impl<'a> AsMut<'a> for Nil {
    type AsMut = Nil;

    fn as_mut(&'a mut self) -> Self::AsMut {
        *self
    }
}
