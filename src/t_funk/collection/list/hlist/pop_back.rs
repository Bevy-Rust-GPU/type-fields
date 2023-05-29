use super::{Cons, HList, Nil};

pub trait PopBack: HList {
    type PopBack: HList;

    fn pop_back(self) -> Self::PopBack;
}

impl<T, N> PopBack for Cons<T, N>
where
    Cons<T, N>: HList,
    N: PopBack,
    Cons<T, N::PopBack>: HList,
{
    type PopBack = Cons<T, N::PopBack>;

    fn pop_back(self) -> Self::PopBack {
        Cons(self.0, self.1.pop_back())
    }
}

impl<T> PopBack for Cons<T, Nil> {
    type PopBack = Nil;

    fn pop_back(self) -> Self::PopBack {
        Nil
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::{
        hlist::{Cons, Nil},
        tlist::ToHList,
    };

    use super::PopBack;

    #[test]
    fn test_hlist_pop_back() {
        let list: Cons<usize, Cons<f32, Cons<&str, Nil>>> = (1, 2.0, "three").to_hlist();
        let list: Cons<usize, Cons<f32, Nil>> = list.pop_back();
        let list: Cons<usize, Nil> = list.pop_back();
        let list: Nil = list.pop_back();
        assert_eq!(list, Nil);
    }
}
