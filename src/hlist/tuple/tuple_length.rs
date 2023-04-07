use crate::hlist::cons::ConsLength;

use super::Cons;

pub trait TupleLength: Cons {
    const LENGTH: usize;
}

impl<T> TupleLength for T
where
    T: Cons,
{
    const LENGTH: usize = <T::Cons as ConsLength>::LENGTH;
}

#[cfg(test)]
mod tests {
    use crate::hlist::tuple::TupleLength;

    #[test]
    fn test_tuple_length() {
        assert_eq!(1, <((),)>::LENGTH);
        assert_eq!(2, <((), ())>::LENGTH);
        assert_eq!(3, <((), (), ())>::LENGTH);
        assert_eq!(4, <((), (), (), ())>::LENGTH);
    }
}
