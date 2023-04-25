use crate::macros::functions;

use crate::t_funk::hlist::HList;

/// A `ConsList` that can push a new element to its front.
#[functions]
pub trait PushFront<Head> {
    type PushFront: HList;

    fn push_front(self, head: Head) -> Self::PushFront;
}

impl<T, Head> PushFront<Head> for T
where
    (Head, T): HList,
{
    type PushFront = (Head, T);

    fn push_front(self, head: Head) -> Self::PushFront {
        (head, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::hlist::ToTList;

    use super::PushFront;

    #[test]
    fn test_hlist_push_front() {
        let list: () = ();
        let list: (usize, ()) = list.push_front(1);
        let list: (usize, (usize, ())) = list.push_front(2);
        let list: (usize, (usize, (usize, ()))) = list.push_front(3);
        assert_eq!((3, 2, 1), list.to_tlist());
    }
}
