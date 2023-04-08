use crate::hlist::{
    cons::{ConsPopBack, Uncons},
    tuple::TupleList,
};

pub trait TuplePopBack: TupleList {
    type TuplePopBack: TupleList;

    fn pop_back(self) -> Self::TuplePopBack;
}

impl<T> TuplePopBack for T
where
    T: TupleList,
    T::Cons: ConsPopBack,
    <T::Cons as ConsPopBack>::ConsPopBack: Uncons,
{
    type TuplePopBack = <<T::Cons as ConsPopBack>::ConsPopBack as Uncons>::Uncons;

    fn pop_back(self) -> Self::TuplePopBack {
        self.cons().cons_pop_back().uncons()
    }
}

#[cfg(test)]
mod test {
    use crate::hlist::tuple::tuple_pop_back::TuplePopBack;

    #[test]
    fn test_tuple_pop_back() {
        let list: (usize, f32, &str) = (1, 2.0, "three");
        let list: (usize, f32) = list.pop_back();
        let list: (usize,) = list.pop_back();
        let list: () = list.pop_back();
        assert_eq!((), list);
    }
}

