use crate::t_funk::{Branch, Foldr, Leaf, MappendF, Mconcat, Mempty};

impl<T> Mempty for Leaf<T>
where
    T: Mempty,
{
    type Mempty = T::Mempty;

    fn mempty() -> Self::Mempty {
        T::mempty()
    }
}

impl<T> Mconcat for Leaf<T> {
    type Mconcat = Leaf<T>;

    fn mconcat(self) -> Self::Mconcat {
        self
    }
}

impl<L, T, R> Mempty for Branch<L, T, R> {
    type Mempty = Leaf<T>;

    fn mempty() -> Self::Mempty {
        Leaf::default()
    }
}

impl<L, T, R> Mconcat for Branch<L, T, R>
where
    Branch<L, T, R>: Mempty + Foldr<MappendF, <Self as Mempty>::Mempty>,
{
    type Mconcat = <Branch<L, T, R> as Foldr<MappendF, <Self as Mempty>::Mempty>>::Foldr;

    fn mconcat(self) -> Self::Mconcat {
        self.foldr(MappendF::default(), Self::mempty())
    }
}
