use super::HList;

/// A HList of immutable references
pub trait HListRef<'a>: HList {
    type HeadRef: 'a;
    type TailRef: HListRef<'a>;

    fn head_ref(self) -> Self::HeadRef;
    fn tail_ref(self) -> Self::TailRef;
}

impl<'a, Head, Tail> HListRef<'a> for (&'a Head, Tail)
where
    Self: HList,
    Tail: HListRef<'a>,
{
    type HeadRef = &'a Head;
    type TailRef = Tail;

    fn head_ref(self) -> Self::HeadRef {
        self.0
    }

    fn tail_ref(self) -> Self::TailRef {
        self.1
    }
}

impl<'a, Head, Tail> HListRef<'a> for (&'a mut Head, Tail)
where
    Self: HList,
    Tail: HListRef<'a>,
{
    type HeadRef = &'a Head;
    type TailRef = Tail;

    fn head_ref(self) -> Self::HeadRef {
        self.0
    }

    fn tail_ref(self) -> Self::TailRef {
        self.1
    }
}

impl HListRef<'_> for () {
    type HeadRef = ();

    type TailRef = ();

    fn head_ref(self) -> Self::HeadRef {
        ()
    }

    fn tail_ref(self) -> Self::TailRef {
        ()
    }
}
