use crate::t_funk::hlist::{Here, Next, Path};

use super::{HList, Cons, Nil};

/// A `ConsList` that can retrieve an item by type.
pub trait Get<T, P>: HList {
    fn get(self) -> T;
}

impl<T, N, PathTail, U> Get<U, Cons<Next, PathTail>> for Cons<T, N>
where
    Self: HList,
    N: Get<U, PathTail>,
    PathTail: Path,
{
    fn get(self) -> U {
        self.1.get()
    }
}

impl<Tail, T> Get<T, Cons<Here, Nil>> for Cons<T, Tail>
where
    Self: HList,
{
    fn get(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::{hlist::Get, tlist::ToHList};

    #[test]
    fn test_hlist_get() {
        let list = (1, 2.0, "three").to_hlist();
        let int: usize = list.get();
        let float: f32 = list.get();
        let string: &str = list.get();
        assert_eq!((1, 2.0, "three"), (int, float, string));
    }
}
