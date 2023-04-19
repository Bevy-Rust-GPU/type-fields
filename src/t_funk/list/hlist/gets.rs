use crate::t_funk::hlist::{Path, Paths};

use super::{Get, HList};

/// Fetch multiple items by type from a cons list.
pub trait GetsImpl<T, P>: HList {
    fn gets_impl(self) -> T;
}

impl<T, Head, Tail, PathHead, PathTail> GetsImpl<(Head, Tail), (PathHead, PathTail)> for T
where
    T: Get<Head, PathHead> + GetsImpl<Tail, PathTail> + Clone,
    PathTail: Paths,
    (PathHead, PathTail): Paths,
{
    fn gets_impl(self) -> (Head, Tail) {
        (self.clone().get(), self.gets_impl())
    }
}

impl<T, Head, PathHead> GetsImpl<(Head, ()), (PathHead, ())> for T
where
    T: Get<Head, PathHead>,
    PathHead: Path,
{
    fn gets_impl(self) -> (Head, ()) {
        (self.get(), ())
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
        hlist::{GetsImpl, ToTList},
        tlist::ToHList,
    };

    #[test]
    fn test_cons_gets() {
        let list = (1, 2.0, "three").to_hlist();
        let gets: (&str, (f32, (usize, ()))) = list.gets_impl();
        assert_eq!(("three", 2.0, 1), gets.to_tlist());
    }
}
