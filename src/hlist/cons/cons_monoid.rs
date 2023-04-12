use crate::functional::Monoid;

impl<Head, Tail> Monoid for (Head, Tail) {
    type Identity = ();

    fn mempty() -> Self::Identity {
        ()
    }
}

