use crate::t_funk::hlist::{HList, Next};

/// A `ConsList` that can push a new element to its back.
pub trait PushBack<T> {
    type Path;
    type PushBack: HList;

    fn push_back(self, tail: T) -> Self::PushBack;
}

impl<Head, Tail, PathTail, T> PushBack<T> for (Head, Tail)
where
    Self: HList,
    (Head, Tail::PushBack): HList,
    Tail: PushBack<T, Path = PathTail>,
    //PathTail: Path,
{
    type Path = (Next, PathTail);
    type PushBack = (Head, Tail::PushBack);

    fn push_back(self, tail: T) -> Self::PushBack {
        (self.0, self.1.push_back(tail))
    }
}

impl<T> PushBack<T> for () {
    type Path = ();
    type PushBack = (T, ());

    fn push_back(self, tail: T) -> Self::PushBack {
        (tail, ())
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::hlist::ToTList;

    use super::PushBack;

    #[test]
    fn test_cons_push_back() {
        let list: () = ();
        let list: (usize, ()) = list.push_back(1);
        let list: (usize, (usize, ())) = list.push_back(2);
        let list: (usize, (usize, (usize, ()))) = list.push_back(3);
        assert_eq!((1, 2, 3), list.to_tlist());
    }
}
