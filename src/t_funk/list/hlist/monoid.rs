use crate::t_funk::Mempty;

impl<Head, Tail> Mempty for (Head, Tail) {
    type Mempty = ();

    fn mempty() -> Self::Mempty {
        ()
    }
}

