use super::{HList, HListRef, Cons, Nil};

/// A cons list that can have its items taken by reference
pub trait AsRef<'a>: HList {
    type AsRef: HListRef<'a>;

    fn as_ref(&'a self) -> Self::AsRef;
}

impl<'a, T, N> AsRef<'a> for Cons<T, N>
where
    Self: HList,
    T: 'a,
    N: AsRef<'a>,
    Cons<&'a T, N::AsRef>: HListRef<'a>,
{
    type AsRef = Cons<&'a T, N::AsRef>;

    fn as_ref(&'a self) -> Self::AsRef {
        Cons(&self.0, self.1.as_ref())
    }
}

impl<'a> AsRef<'a> for Nil {
    type AsRef = Nil;

    fn as_ref(&'a self) -> Self::AsRef {
        Nil
    }
}
