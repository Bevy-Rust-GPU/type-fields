/// A `HList` with a known length.
pub trait HListLength {
    const LENGTH: usize;
}

impl<Head, Tail> HListLength for (Head, Tail)
where
    Tail: HListLength,
{
    const LENGTH: usize = <Tail as HListLength>::LENGTH + 1;
}

impl HListLength for () {
    const LENGTH: usize = 0;
}

#[cfg(test)]
mod tests {
    use crate::t_funk::{hlist::HListLength, tlist::ToHList};

    #[test]
    fn test_cons_length() {
        assert_eq!(0, <() as HListLength>::LENGTH);
        assert_eq!(1, <<((),) as ToHList>::HList as HListLength>::LENGTH);
        assert_eq!(2, <<((), ()) as ToHList>::HList as HListLength>::LENGTH);
        assert_eq!(3, <<((), (), ()) as ToHList>::HList as HListLength>::LENGTH);
        assert_eq!(
            4,
            <<((), (), (), ()) as ToHList>::HList as HListLength>::LENGTH
        );
    }
}
