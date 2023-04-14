use crate::hlist::cons::{ConsPushBack, Uncons};

use super::{Cons, TupleList};

pub trait TuplePushBack<Tail> {
    type Path;
    type TuplePushBack: TupleList;

    fn push_back(self, tail: Tail) -> Self::TuplePushBack;
}

impl<T, Head> TuplePushBack<Head> for T
where
    T: Cons,
    T::Cons: ConsPushBack<Head>,
{
    type Path = <T::Cons as ConsPushBack<Head>>::Path;
    type TuplePushBack = <<T::Cons as ConsPushBack<Head>>::ConsPushBack as Uncons>::Uncons;

    fn push_back(self, tail: Head) -> Self::TuplePushBack {
        self.cons().cons_push_back(tail).uncons()
    }
}

#[cfg(test)]
mod tests {
    use super::TuplePushBack;

    #[test]
    fn test_tuple_push_back() {
        let list = ().push_back(1).push_back(2).push_back(3);
        assert_eq!((1, 2, 3), list);
    }
}
