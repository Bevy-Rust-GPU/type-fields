use super::{Cons, Nil};
use crate::t_funk::{Foldr, MappendF, Mconcat, Mempty};

impl<Head, Tail> Mconcat for Cons<Head, Tail>
where
    Self: Mempty + Foldr<MappendF, <Self as Mempty>::Mempty>,
{
    type Mconcat = <Cons<Head, Tail> as Foldr<MappendF, <Self as Mempty>::Mempty>>::Foldr;

    fn mconcat(self) -> Self::Mconcat {
        self.foldr(MappendF::default(), Self::mempty())
    }
}

impl Mconcat for Nil {
    type Mconcat = Self;

    fn mconcat(self) -> Self::Mconcat {
        self
    }
}
