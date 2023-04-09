use super::{ConsList, ConsListMut};

/// A cons list that can have its items taken by reference
pub trait ConsAsMut<'a>: ConsList {
    type ConsAsMut: ConsListMut<'a>;

    fn cons_as_mut(&'a mut self) -> Self::ConsAsMut;
}

impl<'a, Head, Tail> ConsAsMut<'a> for (Head, Tail)
where
    Self: ConsList,
    Head: 'a,
    Tail: ConsAsMut<'a>,
    (&'a mut Head, Tail::ConsAsMut): ConsListMut<'a>,
{
    type ConsAsMut = (&'a mut Head, Tail::ConsAsMut);

    fn cons_as_mut(&'a mut self) -> Self::ConsAsMut {
        (&mut self.0, self.1.cons_as_mut())
    }
}

impl<'a> ConsAsMut<'a> for () {
    type ConsAsMut = ();

    fn cons_as_mut(&'a mut self) -> Self::ConsAsMut {
        *self
    }
}
