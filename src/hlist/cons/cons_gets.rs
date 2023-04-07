use crate::hlist::path::{Path, Paths};

use super::{ConsGet, ConsList};

/// Fetch multiple items by type from a cons list.
pub trait ConsGets<T, P>: ConsList {
    fn cons_gets(self) -> T;
}

impl<T, Head, Tail, PathHead, PathTail> ConsGets<(Head, Tail), (PathHead, PathTail)> for T
where
    T: ConsGet<Head, PathHead> + ConsGets<Tail, PathTail> + Clone,
    PathTail: Paths,
    (PathHead, PathTail): Paths,
{
    fn cons_gets(self) -> (Head, Tail) {
        (self.clone().cons_get(), self.cons_gets())
    }
}

impl<T, Head, PathHead> ConsGets<(Head, ()), (PathHead, ())> for T
where
    T: ConsGet<Head, PathHead>,
    PathHead: Path,
{
    fn cons_gets(self) -> (Head, ()) {
        (self.cons_get(), ())
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::{
        cons::{cons_gets::ConsGets, Uncons},
        tuple::Cons,
    };

    #[test]
    fn test_cons_gets() {
        let list = (1, 2.0, "three").cons();
        let gets: (&str, (f32, (usize, ()))) = list.cons_gets();
        assert_eq!(("three", 2.0, 1), gets.uncons());
    }
}
