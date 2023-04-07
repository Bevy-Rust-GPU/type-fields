use crate::hlist::{
    cons::{ConsSet, Uncons},
    path::Path,
};

use super::Cons;

pub trait TupleSet<T, P>: Cons
{
    fn tuple_set(self, t: T) -> Self;
}

impl<T, P, In> TupleSet<In, P> for T
where
    T: Cons,
    T::Cons: ConsSet<In, P>,
    P: Path,
{
    fn tuple_set(self, t: In) -> Self {
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
