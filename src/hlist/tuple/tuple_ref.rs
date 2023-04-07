use crate::hlist::{
    cons::Uncons,
    tuple::{ConsRef, TupleList},
};

pub trait TupleRef: ConsRef {
    type TupleRef<'a>: TupleList
    where
        Self: 'a;

    fn tuple_ref<'a>(&'a self) -> Self::TupleRef<'a>;
}

impl<T> TupleRef for T
where
    T: ConsRef,
{
    type TupleRef<'a> = <T::ConsRef<'a> as Uncons>::Uncons where T: 'a;

    fn tuple_ref<'a>(&'a self) -> Self::TupleRef<'a> {
        self.cons_ref().uncons()
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::tuple::tuple_ref::TupleRef;

    #[test]
    fn test_tuple_ref() {
        let list = (1, 2.0, "three").tuple_ref();
        assert_eq!((&1, &2.0, &"three"), list);
    }
}
