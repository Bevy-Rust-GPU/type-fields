use crate::hlist::{
    cons::{ConsPopFront, Uncons},
    tuple::TupleList,
};

pub trait TuplePopFront: TupleList {
    type TuplePopFront: TupleList;

    fn pop_front(self) -> Self::TuplePopFront;
}

impl<T> TuplePopFront for T
where
    T: TupleList,
    T::Cons: ConsPopFront,
    <T::Cons as ConsPopFront>::ConsPopFront: Uncons,
{
    type TuplePopFront = <<T::Cons as ConsPopFront>::ConsPopFront as Uncons>::Uncons;

    fn pop_front(self) -> Self::TuplePopFront {
        self.cons().cons_pop_front().uncons()
    }
}

#[cfg(test)]
mod test {
    use crate::hlist::tuple::tuple_pop_front::TuplePopFront;

    #[test]
    fn test_tuple_pop_back() {
        let list: (usize, f32, &str) = (1, 2.0, "three");
        let list: (f32, &str) = list.pop_front();
        let list: (&str,) = list.pop_front();
        let list: () = list.pop_front();
        assert_eq!((), list);
    }
}

