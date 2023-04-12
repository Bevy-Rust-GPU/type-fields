use crate::functional::{Mappend, Monoid, Semigroup, SemigroupConcat};

use super::ConsFoldRight;

impl<Head, Tail, U> Semigroup<U> for (Head, Tail)
where
    Tail: Semigroup<U>,
{
    type Appended = (Head, Tail::Appended);

    fn mappend(self, t: U) -> Self::Appended {
        (self.0, self.1.mappend(t))
    }
}

impl<U> Semigroup<U> for () {
    type Appended = U;

    fn mappend(self, t: U) -> Self::Appended {
        t
    }
}

impl<T> SemigroupConcat for T
where
    T: Monoid + ConsFoldRight<T::Identity, Mappend>,
{
    type Concatenated = T::Folded;

    fn mconcat(self) -> Self::Concatenated {
        self.cons_fold_right(T::mempty(), Mappend::default())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        functional::{Copointed, Functor, Point, SemigroupConcat, Sum},
        hlist::tuple::Cons,
    };

    #[test]
    fn test_cons_semigroup() {
        let concat = (1, 2, 3)
            .cons()
            .fmap(Point::<Sum<i32>>::default())
            .mconcat();

        assert_eq!(concat.copoint(), 6);
    }
}

