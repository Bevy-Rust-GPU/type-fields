use super::{ConsList, ConsListRef};

/// A cons list of immutable references
pub trait ConsClone<'a>: ConsListRef<'a> {
    type ConsClone: ConsList;

    fn cons_clone(self) -> Self::ConsClone;
}

impl<'a, Head, Tail> ConsClone<'a> for (&'a Head, Tail)
where
    Self: ConsListRef<'a>,
    Head: Clone,
    Tail: ConsClone<'a>,
    (Head, Tail::ConsClone): ConsList,
{
    type ConsClone = (Head, Tail::ConsClone);

    fn cons_clone(self) -> Self::ConsClone {
        (self.0.clone(), self.1.cons_clone())
    }
}

impl<'a, Head, Tail> ConsClone<'a> for (&'a mut Head, Tail)
where
    Self: ConsListRef<'a>,
    Head: Clone,
    Tail: ConsClone<'a>,
    (Head, Tail::ConsClone): ConsList,
{
    type ConsClone = (Head, Tail::ConsClone);

    fn cons_clone(self) -> Self::ConsClone {
        (self.0.clone(), self.1.cons_clone())
    }
}

impl ConsClone<'_> for () {
    type ConsClone = ();

    fn cons_clone(self) -> Self::ConsClone {
        self.clone()
    }
}

