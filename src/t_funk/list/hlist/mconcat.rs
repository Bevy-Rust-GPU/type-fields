use crate::t_funk::{Foldr, MappendF, Mconcat, Mempty};

impl<Head, Tail> Mconcat for (Head, Tail)
where
    Self: Mempty + Foldr<MappendF, <Self as Mempty>::Mempty>,
{
    type Mconcat = <(Head, Tail) as Foldr<MappendF, <Self as Mempty>::Mempty>>::Foldr;

    fn mconcat(self) -> Self::Mconcat {
        self.foldr(MappendF::default(), Self::mempty())
    }
}

impl Mconcat for () {
    type Mconcat = ();

    fn mconcat(self) -> Self::Mconcat {
        ()
    }
}

