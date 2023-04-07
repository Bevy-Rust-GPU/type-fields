use crate::hlist::cons::Uncons;

use super::{ConsMut, TupleList};

pub trait TupleMut: ConsMut {
    type TupleMut<'a>: TupleList
    where
        Self: 'a;

    fn tuple_mut<'a>(&'a mut self) -> Self::TupleMut<'a>;
}

impl<T> TupleMut for T
where
    T: ConsMut,
{
    type TupleMut<'a> = <T::ConsMut<'a> as Uncons>::Uncons where T: 'a;

    fn tuple_mut<'a>(&'a mut self) -> Self::TupleMut<'a> {
        self.cons_mut().uncons()
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist::tuple::TupleMut;

    #[test]
    fn test_tuple_mut() {
        let mut list = (1, 2.0, "three");
        let list_mut = list.tuple_mut();
        assert_eq!((&mut 1, &mut 2.0, &mut "three"), list_mut);
    }
}
