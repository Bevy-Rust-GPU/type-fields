use crate::hlist::{
    cons::{ConsList, ConsSets, Uncons},
    path::Paths,
};

use super::{Cons, TupleList};

pub trait TupleSets<T, P>: TupleList {
    type Input: Cons;
    type InputCons: ConsList;

    fn tuple_sets(self, t: T) -> Self;
}

impl<T, P, In> TupleSets<In, P> for T
where
    Self: TupleList,
    Self::Cons: ConsSets<In::Cons, P, Paths = P>,
    In: Cons,
    P: Paths,
{
    type Input = In;
    type InputCons = In::Cons;

    fn tuple_sets(self, t: In) -> Self {
        self.cons().cons_sets(t.cons()).uncons()
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::tuple::TupleSets;

    #[test]
    fn test_tuple_sets() {
        let list = (1, 2.0, "three");
        let list = TupleSets::<(&str, f32, usize), _>::tuple_sets(list, ("hello", 7.0, 5));
        //let list = list.tuple_sets(("hello", 7.0, 5));
        assert_eq!((5, 7.0, "hello"), list);
    }
}
