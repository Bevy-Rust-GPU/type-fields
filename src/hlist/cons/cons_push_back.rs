use crate::hlist::{
    cons::ConsList,
    path::{Here, Next, Path},
};

/// A `ConsList` that can push a new element to its back.
pub trait ConsPushBack<T, P> {
    type Path: Path;
    type ConsPushBack: ConsList;

    fn cons_push_back(self, tail: T) -> Self::ConsPushBack;
}

impl<Head, Tail, PathTail, T> ConsPushBack<T, (Next, PathTail)> for (Head, Tail)
where
    Self: ConsList,
    (Head, Tail::ConsPushBack): ConsList,
    Tail: ConsPushBack<T, PathTail>,
    PathTail: Path,
{
    type Path = (Next, PathTail);
    type ConsPushBack = (Head, Tail::ConsPushBack);

    fn cons_push_back(self, tail: T) -> Self::ConsPushBack {
        (self.0, self.1.cons_push_back(tail))
    }
}

impl<T, Head> ConsPushBack<T, (Here, ())> for (Head, ()) {
    type Path = (Here, ());
    type ConsPushBack = (Head, (T, ()));

    fn cons_push_back(self, tail: T) -> Self::ConsPushBack {
        (self.0, (tail, ()))
    }
}

impl<T> ConsPushBack<T, ()> for () {
    type Path = (Here, ());
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
        let list = ().cons_push_back(1).cons_push_back(2).cons_push_back(3);
        assert_eq!((1, 2, 3), list.uncons());
    }
}
