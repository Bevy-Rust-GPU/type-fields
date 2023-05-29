use crate::t_funk::hlist::{Path, Paths};

use super::{Cons, HList, Nil, Set};

/// A `ConsList` that can replace multiple items by type.
pub trait Sets<T, P>: HList {
    fn sets(self, t: T) -> Self;
}

impl<T, Head, Tail, PathHead, PathTail> Sets<Cons<Head, Tail>, Cons<PathHead, PathTail>> for T
where
    T: Set<Head, PathHead>,
    T: Sets<Tail, PathTail>,
    Cons<PathHead, PathTail>: Paths,
    PathHead: Path,
    PathTail: Paths,
{
    fn sets(self, Cons(head, tail): Cons<Head, Tail>) -> Self {
        self.set(head).sets(tail)
    }
}

impl<T, Head, PathHead> Sets<Cons<Head, Nil>, Cons<PathHead, Nil>> for T
where
    Self: HList,
    Cons<PathHead, Nil>: Paths,
    T: Set<Head, PathHead>,
    PathHead: Path,
{
    fn sets(self, Cons(head, _): Cons<Head, Nil>) -> Self {
        self.set(head)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        t_funk::hlist::{Sets, ToTList},
        t_funk::{
            hlist::{Cons, Nil},
            tlist::ToHList,
        },
    };

    #[test]
    fn test_hlist_sets() {
        let list = (1, 2.0, "three").to_hlist();
        let list = list.sets(Cons("hello", Cons(7.0, Cons(4, Nil))));
        assert_eq!((4, 7.0, "hello"), list.to_tlist());
    }
}
