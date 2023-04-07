use crate::hlist::path::{Here, Next, Path};

use super::ConsList;

/// A `ConsList` that can replace an item by type.
///
/// This is short-circuiting,
/// so only the first found instance of the given type will be replaced.
pub trait ConsSet<T, P>: ConsList {
    type ConsSet;

    fn cons_set(self, t: T) -> Self::ConsSet;
}

impl<Head, Tail, PathTail, T> ConsSet<T, (Next, PathTail)> for (Head, Tail)
where
    Self: ConsList,
    Tail: ConsSet<T, PathTail, ConsSet = Tail>,
    PathTail: Path,
{
    type ConsSet = Self;

    fn cons_set(self, t: T) -> Self::ConsSet {
        (self.0, self.1.cons_set(t))
    }
}

impl<Tail, T> ConsSet<T, (Here, ())> for (T, Tail)
where
    Self: ConsList,
{
    type ConsSet = Self;

    fn cons_set(self, t: T) -> Self::ConsSet {
        (t, self.1)
    }
}

impl<T> ConsSet<T, ()> for () {
    type ConsSet = (T, ());

    fn cons_set(self, t: T) -> Self::ConsSet {
        (t, self)
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

    #[test]
    fn test_cons_set_append() {
        let list = ();
        let list = list.cons_set(6);
        let list = list.cons_set(5.0);
        let list = list.cons_set("four");
        assert_eq!((6, 5.0, "four"), list.uncons());
    }
}
