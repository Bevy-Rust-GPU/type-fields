use crate::hlist::{
    cons::{ConsSet, Uncons},
    path::Path,
};

use super::{Cons, TupleList};

pub trait TupleSet<T, P>: TupleList {
    fn set(self, t: T) -> Self;
}

impl<T, P, In> TupleSet<In, P> for T
where
    T: Cons,
    T::Cons: ConsSet<In, P> + Uncons<Uncons = T>,
    P: Path,
{
    fn set(self, t: In) -> T {
        self.cons().cons_set(t).uncons()
    }
}

#[cfg(test)]
mod tests {
    use super::TupleSet;

    #[test]
    fn test_tuple_set() {
        let list = (1, 2.0, "three")
            .set(6)
            .set(5.0)
            .set("four");
        assert_eq!((6, 5.0, "four"), list);
    }
}
