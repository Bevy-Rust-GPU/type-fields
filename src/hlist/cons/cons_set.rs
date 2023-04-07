use crate::hlist::path::{Here, Next, Path};

use super::ConsList;

/// A `ConsList` that can replace an item by type.
///
/// This is short-circuiting,
/// so only the first found instance of the given type will be replaced.
pub trait ConsSet<T, P>: ConsList {
    type Path: Path;

    fn cons_set(self, t: T) -> Self;
}

impl<Head, Tail, PathTail, T> ConsSet<T, (Next, PathTail)> for (Head, Tail)
where
    Self: ConsList,
    Tail: ConsSet<T, PathTail>,
    PathTail: Path,
{
    type Path = (Next, PathTail);

    fn cons_set(self, t: T) -> Self {
        (self.0, self.1.cons_set(t))
    }
}

impl<Tail, T> ConsSet<T, (Here, ())> for (T, Tail)
where
    Self: ConsList,
{
    type Path = (Here, ());

    fn cons_set(self, t: T) -> Self {
        (t, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::{
        cons::{ConsSet, Uncons},
        tuple::Cons,
    };

    #[test]
    fn test_cons_set() {
        let list = (1, 2.0, "three").cons();
        let list = list.cons_set(6);
        let list = list.cons_set(5.0);
        let list = list.cons_set("four");
        assert_eq!((6, 5.0, "four"), list.uncons());
    }
}
