use crate::t_funk::hlist::{Path, Paths};

use super::{HList, Set};

/// A `ConsList` that can replace multiple items by type.
pub trait Sets<T, P>: HList {
    fn sets(self, t: T) -> Self;
}

impl<T, Head, Tail, PathHead, PathTail> Sets<(Head, Tail), (PathHead, PathTail)> for T
where
    T: Set<Head, PathHead>,
    T: Sets<Tail, PathTail>,
    (PathHead, PathTail): Paths,
    PathHead: Path,
    PathTail: Paths,
{
    fn sets(self, (head, tail): (Head, Tail)) -> Self {
        self.set(head).sets(tail)
    }
}

impl<T, Head, PathHead> Sets<(Head, ()), (PathHead, ())> for T
where
    Self: HList,
    (PathHead, ()): Paths,
    T: Set<Head, PathHead>,
    PathHead: Path,
{
    fn sets(self, (head, _): (Head, ())) -> Self {
        self.set(head)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        t_funk::hlist::{Sets, ToTList},
        t_funk::tlist::ToHList,
    };

    #[test]
    fn test_hlist_sets() {
        let list = (1, 2.0, "three").to_hlist();
        let list = list.sets(("hello", (7.0, (4, ()))));
        assert_eq!((4, 7.0, "hello"), list.to_tlist());
    }
}
