use super::ConsList;

/// A cons list of immutable references
pub trait ConsListRef<'a>: ConsList {
    type HeadRef: 'a;
    type TailRef: ConsListRef<'a>;

    fn head_ref(self) -> Self::HeadRef;
    fn tail_ref(self) -> Self::TailRef;
}

impl<'a, Head, Tail> ConsListRef<'a> for (&'a Head, Tail)
where
    Self: ConsList,
    Tail: ConsListRef<'a>,
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

impl<'a, Head, Tail> ConsListRef<'a> for (&'a mut Head, Tail)
where
    Self: ConsList,
    Tail: ConsListRef<'a>,
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

impl ConsListRef<'_> for () {
    type HeadRef = ();

    type TailRef = ();

    fn head_ref(self) -> Self::HeadRef {
        ()
    }

    fn tail_ref(self) -> Self::TailRef {
        ()
    }
}
