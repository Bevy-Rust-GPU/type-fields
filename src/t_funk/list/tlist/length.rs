use crate::t_funk::hlist::HListLength;

use super::ToHList;

pub trait TListLength: ToHList {
    const LENGTH: usize;
}

impl<T> TListLength for T
where
    T: ToHList,
{
    const LENGTH: usize = <T::HList as HListLength>::LENGTH;
}

#[cfg(test)]
mod tests {
    use crate::t_funk::tlist::TListLength;

    #[test]
    fn test_tuple_length() {
        assert_eq!(1, <((),)>::LENGTH);
        assert_eq!(2, <((), ())>::LENGTH);
        assert_eq!(3, <((), (), ())>::LENGTH);
        assert_eq!(4, <((), (), (), ())>::LENGTH);
    }
}
