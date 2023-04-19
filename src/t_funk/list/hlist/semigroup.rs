use crate::t_funk::{Mappend, MappendF, Mempty, SemigroupConcat};

use super::FoldRight;

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

impl<T> SemigroupConcat for T
where
    T: Mempty + FoldRight<T::Mempty, MappendF>,
{
    type Concatenated = T::Folded;

    fn mconcat(self) -> Self::Concatenated {
        self.fold_right(T::mempty(), MappendF::default())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        t_funk::{Copointed, Fmap, PointF, SemigroupConcat, Sum},
        t_funk::tlist::ToHList,
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
