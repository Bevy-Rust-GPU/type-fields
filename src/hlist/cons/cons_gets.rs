use crate::hlist::path::{Path, Paths};

use super::{ConsGet, ConsList};

/// Fetch multiple items by type from a cons list.
pub trait ConsGetsImpl<T, P>: ConsList {
    fn cons_gets_impl(self) -> T;
}

impl<T, Head, Tail, PathHead, PathTail> ConsGetsImpl<(Head, Tail), (PathHead, PathTail)> for T
where
    T: ConsGet<Head, PathHead> + ConsGetsImpl<Tail, PathTail> + Clone,
    PathTail: Paths,
    (PathHead, PathTail): Paths,
{
    fn cons_gets_impl(self) -> (Head, Tail) {
        (self.clone().cons_get(), self.cons_gets_impl())
    }
}

impl<T, Head, PathHead> ConsGetsImpl<(Head, ()), (PathHead, ())> for T
where
    T: ConsGet<Head, PathHead>,
    PathHead: Path,
{
    fn cons_gets_impl(self) -> (Head, ()) {
        (self.cons_get(), ())
    }
}

pub trait ConsGets<P>: ConsList {
    fn cons_gets<T>(self) -> T
    where
        Self: ConsGetsImpl<T, P>;
}

impl<T, P> ConsGets<P> for T
where
    T: ConsList,
{
    fn cons_gets<In>(self) -> In
    where
        Self: ConsGetsImpl<In, P>,
    {
        self.cons_gets_impl()
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::{
        cons::{cons_gets::ConsGetsImpl, Uncons},
        tuple::Cons,
    };

    #[test]
    fn test_cons_gets() {
        let list = (1, 2.0, "three").cons();
        let gets: (&str, (f32, (usize, ()))) = list.cons_gets_impl();
        assert_eq!(("three", 2.0, 1), gets.uncons());
    }
}
