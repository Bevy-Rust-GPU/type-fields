use type_fields_macros::functions;

use super::{Cons, Nil};
use crate::t_funk::closure::{Compose, ComposeT};

#[functions]
pub trait Chain {
    type Chain;

    fn chain(self) -> Self::Chain;
}

pub type ChainT<T> = <T as Chain>::Chain;

impl<T, N> Chain for Cons<T, N>
where
    N: Chain,
    ChainT<N>: Compose<T>,
{
    type Chain = ComposeT<ChainT<N>, T>;

    fn chain(self) -> Self::Chain {
        self.1.chain().compose(self.0)
    }
}

impl<T> Chain for Cons<T, Nil> {
    type Chain = T;

    fn chain(self) -> Self::Chain {
        self.0
    }
}
