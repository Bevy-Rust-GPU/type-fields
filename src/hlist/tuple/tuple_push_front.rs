use crate::hlist::cons::{ConsPushFront, Uncons};

use super::{Cons, TupleList};

pub trait TuplePushFront<Head> {
    type TuplePushFront: TupleList;

    fn push_front(self, head: Head) -> Self::TuplePushFront;
}

impl<T, Head> TuplePushFront<Head> for T
where
    T: Cons,
    T::Cons: ConsPushFront<Head>,
{
    type TuplePushFront = <<T::Cons as ConsPushFront<Head>>::ConsPushFront as Uncons>::Uncons;

    fn push_front(self, head: Head) -> Self::TuplePushFront {
        self.cons().cons_push_front(head).uncons()
    }
}

#[cfg(test)]
mod tests {
    use super::TuplePushFront;

    #[test]
    fn test_tuple_push_front() {
        let list = ().push_front(3).push_front(2).push_front(1);
        assert_eq!((1, 2, 3), list);
    }
}
