/// A `ConsList` with a known length.
pub trait ConsLength {
    const LENGTH: usize;
}

impl<Head, Tail> ConsLength for (Head, Tail)
where
    Tail: ConsLength,
{
    const LENGTH: usize = <Tail as ConsLength>::LENGTH + 1;
}

impl ConsLength for () {
    const LENGTH: usize = 0;
}

#[cfg(test)]
mod tests {
    use crate::hlist::{cons::ConsLength, tuple::Cons};

    #[test]
    fn test_cons_length() {
        assert_eq!(0, <() as ConsLength>::LENGTH);
        assert_eq!(1, <<((),) as Cons>::Cons as ConsLength>::LENGTH);
        assert_eq!(2, <<((), ()) as Cons>::Cons as ConsLength>::LENGTH);
        assert_eq!(3, <<((), (), ()) as Cons>::Cons as ConsLength>::LENGTH);
        assert_eq!(4, <<((), (), (), ()) as Cons>::Cons as ConsLength>::LENGTH);
    }
}
