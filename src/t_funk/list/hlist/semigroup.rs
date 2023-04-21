use crate::t_funk::{Foldr, Mappend, MappendF, Mconcat, Mempty};

impl<Head, Tail, U> Mappend<U> for (Head, Tail)
where
    Tail: Mappend<U>,
{
    type Mappend = (Head, Tail::Mappend);

    fn mappend(self, t: U) -> Self::Mappend {
        (self.0, self.1.mappend(t))
    }
}

impl<U> Mappend<U> for () {
    type Mappend = U;

    fn mappend(self, t: U) -> Self::Mappend {
        t
    }
}

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

#[cfg(test)]
mod test {
    use crate::{
        t_funk::tlist::ToHList,
        t_funk::{Copointed, Fmap, Mconcat, PointF, Sum},
    };

    #[test]
    fn test_cons_semigroup() {
        let concat = (1, 2, 3)
            .to_hlist()
            .fmap(PointF::<Sum<i32>>::default())
            .mconcat();

        assert_eq!(concat.copoint(), 6);
    }
}
