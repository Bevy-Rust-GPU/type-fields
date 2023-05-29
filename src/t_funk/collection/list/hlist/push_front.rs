use crate::macros::functions;

use crate::t_funk::hlist::HList;

use super::Cons;

/// A `ConsList` that can push a new element to its front.
#[functions]
pub trait PushFront<Head> {
    type PushFront: HList;

    fn push_front(self, head: Head) -> Self::PushFront;
}

impl<T, Head> PushFront<Head> for T
where
    Cons<Head, T>: HList,
{
    type PushFront = Cons<Head, T>;

    fn push_front(self, head: Head) -> Self::PushFront {
        Cons(head, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::hlist::{Cons, Nil, ToTList};

    use super::PushFront;

    #[test]
    fn test_hlist_push_front() {
        let list: Nil = Nil;
        let list: Cons<usize, Nil> = list.push_front(1);
        let list: Cons<usize, Cons<usize, Nil>> = list.push_front(2);
        let list: Cons<usize, Cons<usize, Cons<usize, Nil>>> = list.push_front(3);
        assert_eq!((3, 2, 1), list.to_tlist());
    }
}
