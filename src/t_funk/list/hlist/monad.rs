use crate::t_funk::{monad::Chain, Closure};

impl<Head, Tail, F> Chain<F> for (Head, Tail)
where
    F: Clone + Closure<Head>,
    Tail: Chain<F>,
{
    type Chain = (F::Output, Tail::Chain);

    fn chain(self, f: F) -> Self::Chain {
        (f.clone().call(self.0), self.1.chain(f))
    }
}

impl<F> Chain<F> for () {
    type Chain = ();

    fn chain(self, _: F) -> Self::Chain {
        self
    }
}
