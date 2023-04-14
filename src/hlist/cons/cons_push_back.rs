use crate::hlist::{cons::ConsList, path::Next};

/// A `ConsList` that can push a new element to its back.
pub trait ConsPushBack<T> {
    type Path;
    type ConsPushBack: ConsList;

    fn cons_push_back(self, tail: T) -> Self::ConsPushBack;
}

impl<Head, Tail, PathTail, T> ConsPushBack<T> for (Head, Tail)
where
    Self: ConsList,
    (Head, Tail::ConsPushBack): ConsList,
    Tail: ConsPushBack<T, Path = PathTail>,
    //PathTail: Path,
{
    type Path = (Next, PathTail);
    type ConsPushBack = (Head, Tail::ConsPushBack);

    fn cons_push_back(self, tail: T) -> Self::ConsPushBack {
        (self.0, self.1.cons_push_back(tail))
    }
}

impl<T> ConsPushBack<T> for () {
    type Path = ();
    type ConsPushBack = (T, ());

    fn cons_push_back(self, tail: T) -> Self::ConsPushBack {
        (tail, ())
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::cons::Uncons;

    use super::ConsPushBack;

    #[test]
    fn test_cons_push_back() {
        let list: () = ();
        let list: (usize, ()) = list.cons_push_back(1);
        let list: (usize, (usize, ())) = list.cons_push_back(2);
        let list: (usize, (usize, (usize, ()))) = list.cons_push_back(3);
        assert_eq!((1, 2, 3), list.uncons());
    }
}
