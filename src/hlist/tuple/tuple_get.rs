use crate::hlist::{cons::ConsGet, path::Path, tuple::Cons};

pub trait TupleGet<T, P>: Cons
{
    fn tuple_get(self) -> T;
}

impl<T, P, In> TupleGet<In, P> for T
where
    T: Cons,
    T::Cons: ConsGet<In, P>,
    P: Path,
{
    fn tuple_get(self) -> In {
        self.cons().cons_get()
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::tuple::TupleGet;

    #[test]
    fn test_tuple_get() {
        let list = (1, 2.0, "three");
        let int: usize = list.tuple_get();
        let float: f32 = list.tuple_get();
        let string: &str = list.tuple_get();
        assert_eq!((1, 2.0, "three"), (int, float, string));
    }
}
