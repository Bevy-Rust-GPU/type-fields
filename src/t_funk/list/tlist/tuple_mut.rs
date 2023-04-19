use crate::t_funk::hlist::ToTList;

use super::{AsHListMut, TList};

pub trait TupleMut: AsHListMut {
    type TupleMut<'a>: TList
    where
        Self: 'a;

    fn as_mut<'a>(&'a mut self) -> Self::TupleMut<'a>;
}

impl<T> TupleMut for T
where
    T: AsHListMut,
{
    type TupleMut<'a> = <T::HListMut<'a> as ToTList>::TList where T: 'a;

    fn as_mut<'a>(&'a mut self) -> Self::TupleMut<'a> {
        self.as_hlist_mut().to_tlist()
    }
}

#[cfg(test)]
mod tests {
    use crate::t_funk::tlist::TupleMut;

    #[test]
    fn test_tuple_mut() {
        let mut list = (1, 2.0, "three");
        let list_mut = list.as_mut();
        assert_eq!((&mut 1, &mut 2.0, &mut "three"), list_mut);
    }
}
