use crate::hlist::{
    cons::{ConsRemove, ConsRemoveImpl, Uncons},
    path::Path,
};

use super::{Cons, TupleList};

pub trait TupleRemoveImpl<T, P>: TupleList {
    type TupleRemove: TupleList;

    fn remove_impl(self) -> Self::TupleRemove;
}

impl<T, P, In> TupleRemoveImpl<In, P> for T
where
    T: Cons,
    T::Cons: ConsRemoveImpl<In, P>,
    <T::Cons as ConsRemoveImpl<In, P>>::ConsRemove: Uncons,
    P: Path,
{
    type TupleRemove = <<T::Cons as ConsRemoveImpl<In, P>>::ConsRemove as Uncons>::Uncons;

    fn remove_impl(self) -> Self::TupleRemove {
        self.cons().cons_remove().uncons()
    }
}

pub trait TupleRemove<P>: TupleList {
    fn remove<T>(self) -> Self::TupleRemove
    where
        Self: TupleRemoveImpl<T, P>;
}

impl<T, P> TupleRemove<P> for T
where
    T: TupleList,
{
    fn remove<In>(self) -> <Self as TupleRemoveImpl<In, P>>::TupleRemove
    where
        Self: TupleRemoveImpl<In, P>,
    {
        self.remove_impl()
    }
}

#[cfg(test)]
mod tests {
    use super::TupleRemove;

    #[test]
    fn test_tuple_remove() {
        let list: (usize, f32, &str) = (1, 2.0, "three");
        let list: (usize, f32) = list.remove::<&str>();
        let list: (usize,) = list.remove::<f32>();
        let list: () = list.remove::<usize>();
        assert_eq!((), list);
    }
}

