use crate::t_funk::hlist::{Path, Remove as HListRemove, RemoveImpl as HListRemoveImpl, ToTList};

use super::{TList, ToHList};

pub trait RemoveImpl<T, P>: TList {
    type Remove: TList;

    fn remove_impl(self) -> Self::Remove;
}

impl<T, P, In> RemoveImpl<In, P> for T
where
    T: ToHList,
    T::HList: HListRemoveImpl<In, P>,
    <T::HList as HListRemoveImpl<In, P>>::Remove: ToTList,
    P: Path,
{
    type Remove = <<T::HList as HListRemoveImpl<In, P>>::Remove as ToTList>::TList;

    fn remove_impl(self) -> Self::Remove {
        self.to_hlist().remove().to_tlist()
    }
}

pub trait TupleRemove<P>: TList {
    fn remove<T>(self) -> Self::Remove
    where
        Self: RemoveImpl<T, P>;
}

impl<T, P> TupleRemove<P> for T
where
    T: TList,
{
    fn remove<In>(self) -> <Self as RemoveImpl<In, P>>::Remove
    where
        Self: RemoveImpl<In, P>,
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
