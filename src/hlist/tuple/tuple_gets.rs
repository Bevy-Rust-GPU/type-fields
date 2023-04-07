use crate::hlist::{
    cons::{ConsGets, Uncons},
    path::Paths,
};

use super::{Cons, TupleList};

pub trait TupleGets<In, P>: TupleList {
    type Input: Cons;
    type Cons: ConsGets<<Self::Input as Cons>::Cons, P>;

    fn tuple_gets(self) -> Self::Input;
}

impl<T, P, In> TupleGets<In, P> for T
where
    T: TupleList,
    In: TupleList,
    T::Cons: ConsGets<In::Cons, P>,
    P: Paths,
{
    type Input = In;
    type Cons = T::Cons;

    fn tuple_gets(self) -> Self::Input {
        self.cons().cons_gets().uncons()
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
