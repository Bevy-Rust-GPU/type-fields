use crate::t_funk::Mappend;

use super::{Cons, Nil};

impl<T, N, U> Mappend<U> for Cons<T, N>
where
    N: Mappend<U>,
{
    type Mappend = Cons<T, N::Mappend>;

    fn mappend(self, t: U) -> Self::Mappend {
        Cons(self.0, self.1.mappend(t))
    }
}

impl<U> Mappend<U> for Nil {
    type Mappend = U;

    fn mappend(self, t: U) -> Self::Mappend {
        t
    }
}

#[cfg(test)]
mod test {
    use crate::{
        t_funk::tlist::ToHList,
        t_funk::{Copointed, Fmap, Mconcat, PointF, Sum},
    };

    #[test]
    fn test_hlist_semigroup() {
        let concat = (1, 2, 3)
            .to_hlist()
            .fmap(PointF::<Sum<i32>>::default())
            .mconcat();

        assert_eq!(concat.copoint(), 6);
    }
}
