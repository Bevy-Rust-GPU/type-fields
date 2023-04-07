use crate::hlist::cons::{ConsList, Uncons};

use super::{Cons, TupleLength};

pub trait TupleList: Cons + TupleLength {
    type Head;
    type Tail: TupleList;

    fn head(self) -> Self::Head;
    fn tail(self) -> Self::Tail;
}

impl<T> TupleList for T
where
    T: Cons,
    <T::Cons as ConsList>::Tail: Uncons,
{
    type Head = <T::Cons as ConsList>::Head;
    type Tail = <<T::Cons as ConsList>::Tail as Uncons>::Uncons;

    fn head(self) -> Self::Head {
        self.cons().head()
    }

    fn tail(self) -> Self::Tail {
        self.cons().tail().uncons()
    }
}

#[cfg(test)]
mod tests {
    use super::TupleList;

    #[test]
    fn test_tuple_list() {
        let _foo = <(usize, f32, &str) as TupleList>::Head::default();
        let _bar = <(usize, f32, &str) as TupleList>::Tail::default();
    }
}
