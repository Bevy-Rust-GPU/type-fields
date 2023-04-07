use crate::hlist::path::{Here, Next, Path};

use super::ConsList;

/// A `ConsList` that can retrieve an item by type.
pub trait ConsGet<T, P>: ConsList {
    fn cons_get(self) -> T;
}

impl<Head, Tail, PathTail, T> ConsGet<T, (Next, PathTail)> for (Head, Tail)
where
    Self: ConsList,
    Tail: ConsGet<T, PathTail>,
    PathTail: Path,
{
    fn cons_get(self) -> T {
        self.1.cons_get()
    }
}

impl<Tail, T> ConsGet<T, (Here, ())> for (T, Tail)
where
    Self: ConsList,
{
    fn cons_get(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::{cons::cons_get::ConsGet, tuple::Cons};

    #[test]
    fn test_cons_get() {
        let list = (1, 2.0, "three").cons();
        let int: usize = list.cons_get();
        let float: f32 = list.cons_get();
        let string: &str = list.cons_get();
        assert_eq!((1, 2.0, "three"), (int, float, string));
    }
}
