use super::{Cons, Nil};
use crate::t_funk::Mempty;

impl<Head, Tail> Mempty for Cons<Head, Tail> {
    type Mempty = Nil;

    fn mempty() -> Self::Mempty {
        Nil
    }
}

impl Mempty for Nil {
    type Mempty = Nil;

    fn mempty() -> Self::Mempty {
        Nil
    }
}
