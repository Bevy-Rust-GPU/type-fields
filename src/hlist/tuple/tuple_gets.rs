use crate::hlist::{
    cons::{ConsGetsImpl, Uncons},
    path::Paths,
};

use super::TupleList;

pub trait TupleGetsImpl<In, P>: TupleList {
    fn gets_impl(self) -> In;
}

impl<T, P, In> TupleGetsImpl<In, P> for T
where
    T: TupleList,
    In: TupleList,
    T::Cons: ConsGetsImpl<In::Cons, P>,
    P: Paths,
{
    fn gets_impl(self) -> In {
        self.cons().cons_gets_impl().uncons()
    }
}

impl<T> TupleGetsImpl<(), ()> for T
where
    T: TupleList,
{
    fn gets_impl(self) -> () {
        ()
    }
}

pub trait TupleGets<P>: TupleList {
    fn gets<T>(self) -> T
    where
        Self: TupleGetsImpl<T, P>;
}

impl<T, P> TupleGets<P> for T
where
    T: TupleList,
{
    fn gets<In>(self) -> In
    where
        Self: TupleGetsImpl<In, P>,
    {
        self.gets_impl()
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::tuple::TupleGetsImpl;

    #[test]
    fn test_tuple_gets() {
        let list = (1, 2.0, "three");
        let (string, float, int) = TupleGetsImpl::<(&str, f32, usize), _>::gets_impl(list);
        assert_eq!((1, 2.0, "three"), (int, float, string));
    }
}
