use crate::{
    t_funk::hlist::ToTList,
    t_funk::tlist::{AsHListRef, TList},
};

pub trait TupleRef: AsHListRef {
    type TupleRef<'a>: TList
    where
        Self: 'a;

    fn as_ref<'a>(&'a self) -> Self::TupleRef<'a>;
}

impl<T> TupleRef for T
where
    T: AsHListRef,
{
    type TupleRef<'a> = <T::HListRef<'a> as ToTList>::TList where T: 'a;

    fn as_ref<'a>(&'a self) -> Self::TupleRef<'a> {
        self.as_hlist_ref().to_tlist()
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::tlist::tuple_ref::TupleRef;

    #[test]
    fn test_tuple_ref() {
        let list = (1, 2.0, "three").as_ref();
        assert_eq!((&1, &2.0, &"three"), list);
    }
}
