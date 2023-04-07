use crate::hlist::cons::{ConsLength, Uncons};

/// The base cons list type.
/// Describes the Head / CAR, Tail / CDR structure via associated types.
pub trait ConsList: ConsLength + Uncons {
    type Head;
    type Tail: ConsList;

    fn head(self) -> Self::Head;
    fn tail(self) -> Self::Tail;
}

impl<Head, Tail> ConsList for (Head, Tail)
where
    Self: Uncons,
    Tail: ConsList,
{
    type Head = Head;
    type Tail = Tail;

    fn head(self) -> Self::Head {
        self.0
    }

    fn tail(self) -> Self::Tail {
        self.1
    }
}

impl ConsList for () {
    type Head = ();
    type Tail = ();

    fn head(self) -> Self::Head {
        ()
    }

    fn tail(self) -> Self::Tail {
        ()
    }
}
