use crate::hlist::path::{Path, Paths};

use super::{ConsList, ConsSet};

/// A `ConsList` that can replace multiple items by type.
pub trait ConsSets<T, P>: ConsList {
    type ConsSets;

    fn cons_sets(self, t: T) -> Self::ConsSets;
}

impl<T, Head, Tail, PathHead, PathTail> ConsSets<(Head, Tail), (PathHead, PathTail)> for T
where
    T: ConsSet<Head, PathHead>,
    T::ConsSet: ConsSets<Tail, PathTail>,
    (PathHead, PathTail): Paths,
    PathHead: Path,
    PathTail: Paths,
{
    type ConsSets = <T::ConsSet as ConsSets<Tail, PathTail>>::ConsSets;

    fn cons_sets(self, (head, tail): (Head, Tail)) -> Self::ConsSets {
        self.cons_set(head).cons_sets(tail)
    }
}

impl<T, Head, PathHead> ConsSets<(Head, ()), (PathHead, ())> for T
where
    Self: ConsList,
    (PathHead, ()): Paths,
    T: ConsSet<Head, PathHead, ConsSet = T>,
    PathHead: Path,
{
    type ConsSets = T::ConsSet;

    fn cons_sets(self, (head, _): (Head, ())) -> Self::ConsSets {
        self.cons_set(head)
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::{
        cons::{cons_sets::ConsSets, Uncons},
        tuple::Cons,
    };

    #[test]
    fn test_cons_sets() {
        let list = (1, 2.0, "three").cons();
        let list = list.cons_sets(("hello", (7.0, (4, ()))));
        assert_eq!((4, 7.0, "hello"), list.uncons());
    }
}
