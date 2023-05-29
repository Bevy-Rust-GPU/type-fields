use type_fields_macros::functions;

use super::{Cons, HList};

#[functions]
pub trait PopFront: HList {
    type PopFront: HList;

    fn pop_front(self) -> Self::PopFront;
}

impl<T, N> PopFront for Cons<T, N>
where
    Cons<T, N>: HList,
    N: HList,
{
    type PopFront = N;

    fn pop_front(self) -> Self::PopFront {
        self.1
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        hlist::{Cons, Nil},
        tlist::ToHList,
    };

    use super::PopFront;

    #[test]
    fn test_hlist_pop_front() {
        let list: Cons<usize, Cons<f32, Cons<&str, Nil>>> = (1, 2.0, "three").to_hlist();
        let list: Cons<f32, Cons<&str, Nil>> = list.pop_front();
        let list: Cons<&str, Nil> = list.pop_front();
        let list: Nil = list.pop_front();
        assert_eq!(Nil, list);
    }
}
