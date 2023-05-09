use crate::t_funk::hlist::{Path, Paths};

use super::{Cons, Get, HList, Nil};

/// Fetch multiple items by type from a `HList`.
pub trait GetsImpl<T, P>: HList {
    fn gets_impl(self) -> T;
}

impl<T, Head, Tail, PathHead, PathTail> GetsImpl<Cons<Head, Tail>, Cons<PathHead, PathTail>> for T
where
    T: Get<Head, PathHead> + GetsImpl<Tail, PathTail> + Clone,
    PathTail: Paths,
    Cons<PathHead, PathTail>: Paths,
{
    fn gets_impl(self) -> Cons<Head, Tail> {
        Cons(self.clone().get(), self.gets_impl())
    }
}

impl<T, Head, PathHead> GetsImpl<Cons<Head, Nil>, Cons<PathHead, Nil>> for T
where
    T: Get<Head, PathHead>,
    PathHead: Path,
{
    fn gets_impl(self) -> Cons<Head, Nil> {
        Cons(self.get(), Nil)
    }
}

pub trait Gets<P>: HList {
    fn gets<T>(self) -> T
    where
        Self: GetsImpl<T, P>;
}

impl<T, P> Gets<P> for T
where
    T: HList,
{
    fn gets<In>(self) -> In
    where
        Self: GetsImpl<In, P>,
    {
        self.gets_impl()
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::{
        hlist::{GetsImpl, ToTList, Cons, Nil},
        tlist::ToHList,
    };

    #[test]
    fn test_hlist_gets() {
        let list = (1, 2.0, "three").to_hlist();
        let gets: Cons<&str, Cons<f32, Cons<usize, Nil>>> = list.gets_impl();
        assert_eq!(("three", 2.0, 1), gets.to_tlist());
    }
}
