use super::{HList, HListRef, Cons, Nil};

/// A cons list of immutable references
pub trait HListClone<'a>: HListRef<'a> {
    type Cloned: HList;

    fn hlist_clone(self) -> Self::Cloned;
}

impl<'a, Head, Tail> HListClone<'a> for Cons<&'a Head, Tail>
where
    Self: HListRef<'a>,
    Head: Clone,
    Tail: HListClone<'a>,
    Cons<Head, Tail::Cloned>: HList,
{
    type Cloned = Cons<Head, Tail::Cloned>;

    fn hlist_clone(self) -> Self::Cloned {
        Cons(self.0.clone(), self.1.hlist_clone())
    }
}

impl<'a, Head, Tail> HListClone<'a> for Cons<&'a mut Head, Tail>
where
    Self: HListRef<'a>,
    Head: Clone,
    Tail: HListClone<'a>,
    Cons<Head, Tail::Cloned>: HList,
{
    type Cloned = Cons<Head, Tail::Cloned>;

    fn hlist_clone(self) -> Self::Cloned {
        Cons(self.0.clone(), self.1.hlist_clone())
    }
}

impl HListClone<'_> for Nil {
    type Cloned = Nil;

    fn hlist_clone(self) -> Self::Cloned {
        self
    }
}

