use crate::t_funk::hlist::{HList, ToTList};

use super::{ToHList, TListLength};

pub trait TList: ToHList + TListLength {
    type Head;
    type Tail: TList;

    fn head(self) -> Self::Head;
    fn tail(self) -> Self::Tail;
}

impl<T> TList for T
where
    T: ToHList,
    <T::HList as HList>::Tail: ToTList,
{
    type Head = <T::HList as HList>::Head;
    type Tail = <<T::HList as HList>::Tail as ToTList>::TList;

    fn head(self) -> Self::Head {
        self.to_hlist().head()
    }

    fn tail(self) -> Self::Tail {
        self.to_hlist().tail().to_tlist()
    }
}

#[cfg(test)]
mod tests {
    use super::TList;

    #[test]
    fn test_tuple_list() {
        let _foo = <(usize, f32, &str) as TList>::Head::default();
        let _bar = <(usize, f32, &str) as TList>::Tail::default();
    }
}
