use crate::hlist::{
    cons::{ConsSets, Uncons},
    path::Paths,
};

use super::{Cons, TupleList};

pub trait TupleSets<T, P>: TupleList {
    fn tuple_sets(self, t: T) -> Self;
}

impl<T, P, In> TupleSets<In, P> for T
where
    Self: TupleList,
    Self::Cons: ConsSets<In::Cons, P>,
    In: Cons,
    P: Paths,
{
    fn tuple_sets(self, t: In) -> Self {
        self.cons().cons_sets(t.cons()).uncons()
    }
}

impl<T> TupleSets<(), ()> for T
where
    T: TupleList,
{
    fn tuple_sets(self, _: ()) -> Self {
        self
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
