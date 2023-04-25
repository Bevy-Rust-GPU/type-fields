use crate::t_funk::hlist::{Here, Next, Path};

use super::HList;

/// A `ConsList` that can replace an item by type.
///
/// This is short-circuiting,
/// so only the first found instance of the given type will be replaced.
pub trait Set<T, P>: HList {
    fn set(self, t: T) -> Self;
}

impl<Head, Tail, PathTail, T> Set<T, (Next, PathTail)> for (Head, Tail)
where
    Self: HList,
    Tail: Set<T, PathTail>,
    PathTail: Path,
{
    fn set(self, t: T) -> Self {
        (self.0, self.1.set(t))
    }
}

impl<Tail, T> Set<T, (Here, ())> for (T, Tail)
where
    Self: HList,
{
    fn set(self, t: T) -> Self {
        (t, self.1)
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
