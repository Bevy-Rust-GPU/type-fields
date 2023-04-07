use crate::hlist::cons::ConsList;

/// A `ConsList` that can push a new element to its front.
pub trait ConsPushFront<Head>: ConsList {
    type ConsPushFront: ConsList;

    fn cons_push_front(self, head: Head) -> Self::ConsPushFront;
}

impl<T, Head> ConsPushFront<Head> for T
where
    Self: ConsList,
    (Head, T): ConsList,
{
    type ConsPushFront = (Head, T);

    fn cons_push_front(self, head: Head) -> Self::ConsPushFront {
        (head, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::cons::Uncons;

    use super::ConsPushFront;

    #[test]
    fn test_cons_push_front() {
        let list = ().cons_push_front(1).cons_push_front(2).cons_push_front(3);
        assert_eq!((3, 2, 1), list.uncons());
    }
}
