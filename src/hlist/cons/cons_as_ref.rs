use super::{ConsList, ConsListRef};

/// A cons list that can have its items taken by reference
pub trait ConsAsRef<'a>: ConsList {
    type ConsAsRef: ConsListRef<'a>;

    fn cons_as_ref(&'a self) -> Self::ConsAsRef;
}

impl<'a, Head, Tail> ConsAsRef<'a> for (Head, Tail)
where
    Self: ConsList,
    Head: 'a,
    Tail: ConsAsRef<'a>,
    (&'a Head, Tail::ConsAsRef): ConsListRef<'a>,
{
    type ConsAsRef = (&'a Head, Tail::ConsAsRef);

    fn cons_as_ref(&'a self) -> Self::ConsAsRef {
        (&self.0, self.1.cons_as_ref())
    }
}

impl<'a> ConsAsRef<'a> for () {
    type ConsAsRef = ();

    fn cons_as_ref(&'a self) -> Self::ConsAsRef {
        ()
    }
}
