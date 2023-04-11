use crate::functional::{Applicative, Function};

impl<Head, Tail, U> Applicative<U> for (Head, Tail)
where
    Head: Function<U>,
    Tail: Applicative<U>,
    U: Clone,
{
    type Applied = (Head::Output, Tail::Applied);

    fn apply(self, a: U) -> Self::Applied {
        (self.0.call(a.clone()), self.1.apply(a))
    }
}

