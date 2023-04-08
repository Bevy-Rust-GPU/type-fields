use crate::hlist::tuple::TuplePopFront;

use super::OutputMode;

/// Push outputs to the back of the context.
pub struct OutputPopFront;

impl<C> OutputMode<C, (), ()> for OutputPopFront
where
    C: TuplePopFront,
{
    type Output = C::TuplePopFront;

    fn apply(context: C, _: ()) -> Self::Output {
        context.pop_front()
    }
}

