use crate::hlist::{cons::ConsGet, path::Path};

use super::TupleList;

pub trait TupleGetImpl<T, P>: TupleList {
    fn get_impl(self) -> T;
}

impl<T, P, In> TupleGetImpl<In, P> for T
where
    T: TupleList,
    T::Cons: ConsGet<In, P>,
    P: Path,
{
    fn get_impl(self) -> In {
        self.cons().cons_get()
    }
}

pub trait TupleGet<P>: TupleList {
    fn get<T>(self) -> T
    where
        Self: TupleGetImpl<T, P>;
}

impl<T, P> TupleGet<P> for T
where
    T: TupleList,
    P: Path,
{
    fn get<In>(self) -> In
    where
        T: TupleGetImpl<In, P>,
    {
        self.get_impl()
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::tuple::TupleGetImpl;

    #[test]
    fn test_tuple_get() {
        let list = (1, 2.0, "three");
        let int: usize = list.get_impl();
        let float: f32 = list.get_impl();
        let string: &str = list.get_impl();
        assert_eq!((1, 2.0, "three"), (int, float, string));
    }
}
