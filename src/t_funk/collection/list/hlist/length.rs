use super::{Cons, Nil};

/// A `HList` with a known length.
pub trait HListLength {
    const LENGTH: usize;
}

impl<T, N> HListLength for Cons<T, N>
where
    N: HListLength,
{
    const LENGTH: usize = <N as HListLength>::LENGTH + 1;
}

impl HListLength for Nil {
    const LENGTH: usize = 0;
}

#[cfg(test)]
mod tests {
    use crate::t_funk::{hlist::{HListLength, Nil}, tlist::ToHList};

    #[test]
    fn test_cons_length() {
        assert_eq!(0, <Nil as HListLength>::LENGTH);
        assert_eq!(1, <<((),) as ToHList>::HList as HListLength>::LENGTH);
        assert_eq!(2, <<((), ()) as ToHList>::HList as HListLength>::LENGTH);
        assert_eq!(3, <<((), (), ()) as ToHList>::HList as HListLength>::LENGTH);
        assert_eq!(
            4,
            <<((), (), (), ()) as ToHList>::HList as HListLength>::LENGTH
        );
    }
}
