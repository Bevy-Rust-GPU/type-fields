use crate::t_funk::hlist::{Here, Next, Path};

use super::HList;

/// A `ConsList` that can retrieve an item by type.
pub trait Get<T, P>: HList {
    fn get(self) -> T;
}

impl<Head, Tail, PathTail, T> Get<T, (Next, PathTail)> for (Head, Tail)
where
    Self: HList,
    Tail: Get<T, PathTail>,
    PathTail: Path,
{
    fn get(self) -> T {
        self.1.get()
    }
}

impl<Tail, T> Get<T, (Here, ())> for (T, Tail)
where
    Self: HList,
{
    fn get(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::{hlist::Get, ToHList};

    #[test]
    fn test_cons_get() {
        let list = (1, 2.0, "three").to_hlist();
        let int: usize = list.get();
        let float: f32 = list.get();
        let string: &str = list.get();
        assert_eq!((1, 2.0, "three"), (int, float, string));
    }
}
