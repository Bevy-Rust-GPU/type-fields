use crate::t_funk::hlist::{Here, Next, Path};

use super::{Cons, HList, Nil};

/// A `ConsList` that can replace an item by type.
///
/// This is short-circuiting,
/// so only the first found instance of the given type will be replaced.
pub trait Set<T, P>: HList {
    fn set(self, t: T) -> Self;
}

impl<T, N, PathTail, U> Set<U, Cons<Next, PathTail>> for Cons<T, N>
where
    Self: HList,
    N: Set<U, PathTail>,
    PathTail: Path,
{
    fn set(self, t: U) -> Self {
        Cons(self.0, self.1.set(t))
    }
}

impl<Tail, T> Set<T, Cons<Here, Nil>> for Cons<T, Tail>
where
    Self: HList,
{
    fn set(self, t: T) -> Self {
        Cons(t, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        t_funk::hlist::{Set, ToTList},
        t_funk::tlist::ToHList,
    };

    #[test]
    fn test_hlist_set() {
        let list = (1, 2.0, "three").to_hlist();
        let list = list.set(6);
        let list = list.set(5.0);
        let list = list.set("four");
        assert_eq!((6, 5.0, "four"), list.to_tlist());
    }
}
