use crate::hlist::{
    cons::{ConsGets, Uncons},
    path::Paths,
};

use super::TupleList;

pub trait TupleGets<In, P>: TupleList {
    fn tuple_gets(self) -> In;
}

impl<T, P, In> TupleGets<In, P> for T
where
    T: TupleList,
    In: TupleList,
    T::Cons: ConsGets<In::Cons, P>,
    P: Paths,
{
    fn tuple_gets(self) -> In {
        self.cons().cons_gets().uncons()
    }
}

impl<T> TupleGets<(), ()> for T
where
    T: TupleList,
{
    fn tuple_gets(self) -> () {
        ()
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::tuple::TupleGets;

    #[test]
    fn test_tuple_gets() {
        let list = (1, 2.0, "three");
        let (string, float, int) = TupleGets::<(&str, f32, usize), _>::tuple_gets(list);
        assert_eq!((1, 2.0, "three"), (int, float, string));
    }
}
