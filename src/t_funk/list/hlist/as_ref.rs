use super::{HList, HListRef};

/// A cons list that can have its items taken by reference
pub trait AsRef<'a>: HList {
    type AsRef: HListRef<'a>;

    fn as_ref(&'a self) -> Self::AsRef;
}

impl<'a, Head, Tail> AsRef<'a> for (Head, Tail)
where
    Self: HList,
    Head: 'a,
    Tail: AsRef<'a>,
    (&'a Head, Tail::AsRef): HListRef<'a>,
{
    type AsRef = (&'a Head, Tail::AsRef);

    fn as_ref(&'a self) -> Self::AsRef {
        (&self.0, self.1.as_ref())
    }
}

impl<'a> AsRef<'a> for () {
    type AsRef = ();

    fn as_ref(&'a self) -> Self::AsRef {
        ()
    }
}
