use crate::{functional::Function, hlist::cons::ConsList};

/// A `ConsList` that can push a new element to its front.
pub trait ConsPushFront<Head> {
    type ConsPushFront: ConsList;

    fn cons_push_front(self, head: Head) -> Self::ConsPushFront;
}

impl<T, Head> ConsPushFront<Head> for T
where
    (Head, T): ConsList,
{
    type ConsPushFront = (Head, T);

    fn cons_push_front(self, head: Head) -> Self::ConsPushFront {
        (head, self)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PushFront;

impl<L, I> Function<(L, I)> for PushFront
where
    L: ConsPushFront<I>,
{
    type Output = <L as ConsPushFront<I>>::ConsPushFront;

    fn call(self, (l, i): (L, I)) -> Self::Output {
        l.cons_push_front(i)
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::cons::Uncons;

    use super::ConsPushFront;

    #[test]
    fn test_cons_push_front() {
        let list: () = ();
        let list: (usize, ()) = list.cons_push_front(1);
        let list: (usize, (usize, ())) = list.cons_push_front(2);
        let list: (usize, (usize, (usize, ()))) = list.cons_push_front(3);
        assert_eq!((3, 2, 1), list.uncons());
    }
}
