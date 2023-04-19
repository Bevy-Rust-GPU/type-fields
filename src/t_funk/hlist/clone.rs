use super::{HList, HListRef};

/// A cons list of immutable references
pub trait HListClone<'a>: HListRef<'a> {
    type Cloned: HList;

    fn hlist_clone(self) -> Self::Cloned;
}

impl<'a, Head, Tail> HListClone<'a> for (&'a Head, Tail)
where
    Self: HListRef<'a>,
    Head: Clone,
    Tail: HListClone<'a>,
    (Head, Tail::Cloned): HList,
{
    type Cloned = (Head, Tail::Cloned);

    fn hlist_clone(self) -> Self::Cloned {
        (self.0.clone(), self.1.hlist_clone())
    }
}

impl<'a, Head, Tail> HListClone<'a> for (&'a mut Head, Tail)
where
    Self: HListRef<'a>,
    Head: Clone,
    Tail: HListClone<'a>,
    (Head, Tail::Cloned): HList,
{
    type Cloned = (Head, Tail::Cloned);

    fn hlist_clone(self) -> Self::Cloned {
        (self.0.clone(), self.1.hlist_clone())
    }
}

impl HListClone<'_> for () {
    type Cloned = ();

    fn hlist_clone(self) -> Self::Cloned {
        self.clone()
    }
}

