use crate::hlist::cons::{ConsPushFront, Uncons};

use super::{Cons, TupleList};

pub trait TuplePushFront<Head> {
    type TuplePushFront: TupleList;

    fn tuple_push_front(self, head: Head) -> Self::TuplePushFront;
}

impl<T, Head> TuplePushFront<Head> for T
where
    T: Cons,
    T::Cons: ConsPushFront<Head>,
{
    type TuplePushFront = <<T::Cons as ConsPushFront<Head>>::ConsPushFront as Uncons>::Uncons;

    fn tuple_push_front(self, head: Head) -> Self::TuplePushFront {
        self.cons().cons_push_front(head).uncons()
    }
}

#[cfg(test)]
mod tests {
    use super::TuplePushFront;

    #[test]
    fn test_tuple_push_front() {
        let list = ().tuple_push_front(3).tuple_push_front(2).tuple_push_front(1);
        assert_eq!((1, 2, 3), list);
    }
}
