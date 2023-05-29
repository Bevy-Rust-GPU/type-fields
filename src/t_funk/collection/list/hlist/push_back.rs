use crate::{
    macros::functions,
    t_funk::hlist::{Cons, HList, Next, Nil},
};

/// A `ConsList` that can push a new element to its back.
#[functions]
pub trait PushBack<T> {
    type Path;
    type PushBack: HList;

    fn push_back(self, tail: T) -> Self::PushBack;
}

impl<T, N, PathTail, U> PushBack<U> for Cons<T, N>
where
    Self: HList,
    Cons<T, N::PushBack>: HList,
    N: PushBack<U, Path = PathTail>,
    //PathTail: Path,
{
    type Path = Cons<Next, PathTail>;
    type PushBack = Cons<T, N::PushBack>;

    fn push_back(self, tail: U) -> Self::PushBack {
        Cons(self.0, self.1.push_back(tail))
    }
}

impl<T> PushBack<T> for Nil {
    type Path = Nil;
    type PushBack = Cons<T, Nil>;

    fn push_back(self, tail: T) -> Self::PushBack {
        Cons(tail, Nil)
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::hlist::{Cons, Nil, ToTList};

    use super::PushBack;

    #[test]
    fn test_hlist_push_back() {
        let list: Nil = Nil;
        let list: Cons<usize, Nil> = list.push_back(1);
        let list: Cons<usize, Cons<usize, Nil>> = list.push_back(2);
        let list: Cons<usize, Cons<usize, Cons<usize, Nil>>> = list.push_back(3);
        assert_eq!((1, 2, 3), list.to_tlist());
    }
}
