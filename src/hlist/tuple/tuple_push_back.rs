use crate::hlist::cons::{ConsPushBack, Uncons};

use super::{Cons, TupleList};

pub trait TuplePushBack<Tail, P> {
    type TuplePushBack: TupleList;

    fn tuple_push_back(self, tail: Tail) -> Self::TuplePushBack;
}

impl<T, P, Head> TuplePushBack<Head, P> for T
where
    T: Cons,
    T::Cons: ConsPushBack<Head, P>,
{
    type TuplePushBack = <<T::Cons as ConsPushBack<Head, P>>::ConsPushBack as Uncons>::Uncons;

    fn tuple_push_back(self, tail: Head) -> Self::TuplePushBack {
        self.cons().cons_push_back(tail).uncons()
    }
}

#[cfg(test)]
mod tests {
    use super::TuplePushBack;

    #[test]
    fn test_tuple_push_back() {
        let list = ().tuple_push_back(1).tuple_push_back(2).tuple_push_back(3);
        assert_eq!((1, 2, 3), list);
    }
}
