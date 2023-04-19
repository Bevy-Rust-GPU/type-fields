use super::HList;

pub trait PopFront: HList {
    type PopFront: HList;

    fn cons_pop_front(self) -> Self::PopFront;
}

impl<Head, Tail> PopFront for (Head, Tail)
where
    (Head, Tail): HList,
    Tail: HList,
{
    type PopFront = Tail;

    fn cons_pop_front(self) -> Self::PopFront {
        self.1
    }
}

#[cfg(test)]
mod test {
    use crate::t_funk::tlist::ToHList;

    use super::PopFront;

    #[test]
    fn test_cons_pop_front() {
        let list: (usize, (f32, (&str, ()))) = (1, 2.0, "three").to_hlist();
        let list: (f32, (&str, ())) = list.cons_pop_front();
        let list: (&str, ()) = list.cons_pop_front();
        let list: () = list.cons_pop_front();
        assert_eq!((), list);
    }
}
