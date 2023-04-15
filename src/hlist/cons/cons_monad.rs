use crate::functional::{Closure, Monad};

impl<Head, Tail, F> Monad<F> for (Head, Tail)
where
    F: Clone + Closure<Head>,
    Tail: Monad<F>,
{
    type Chained = (F::Output, Tail::Chained);

    fn chain(self, f: F) -> Self::Chained {
        (f.clone().call(self.0), self.1.chain(f))
    }
}
