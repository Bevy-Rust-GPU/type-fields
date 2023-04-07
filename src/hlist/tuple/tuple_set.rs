use crate::hlist::{
    cons::{ConsSet, Uncons},
    path::Path,
};

use super::{Cons, TupleList};

pub trait TupleSet<T, P>: TupleList {
    fn tuple_set(self, t: T) -> Self;
}

impl<T, P, In> TupleSet<In, P> for T
where
    T: Cons,
    T::Cons: ConsSet<In, P> + Uncons<Uncons = T>,
    P: Path,
{
    fn tuple_set(self, t: In) -> T {
        self.cons().cons_set(t).uncons()
    }
}

#[cfg(test)]
mod tests {
    use super::TupleSet;

    #[test]
    fn test_tuple_set() {
        let list = (1, 2.0, "three")
            .tuple_set(6)
            .tuple_set(5.0)
            .tuple_set("four");
        assert_eq!((6, 5.0, "four"), list);
    }
}
