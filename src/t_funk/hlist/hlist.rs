use crate::t_funk::hlist::{HListLength, ToTList};

/// The base cons list type.
/// Describes the Head / CAR, Tail / CDR structure via associated types.
pub trait HList: HListLength + ToTList {
    type Head;
    type Tail: HList;

    fn head(self) -> Self::Head;
    fn tail(self) -> Self::Tail;
}

impl<Head, Tail> HList for (Head, Tail)
where
    Self: ToTList,
    Tail: HList,
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

impl HList for () {
    type Head = ();
    type Tail = ();

    fn head(self) -> Self::Head {
        ()
    }

    fn tail(self) -> Self::Tail {
        ()
    }
}
