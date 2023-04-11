use crate::functional::{Function, Monad};

impl<Head, Tail, F> Monad<F> for (Head, Tail)
where
    F: Clone + Function<Head>,
    Tail: Monad<F>,
{
    type Chained = (F::Output, Tail::Chained);

    fn chain(self, f: F) -> Self::Chained {
        (f.clone().call(self.0), self.1.chain(f))
    }
}
