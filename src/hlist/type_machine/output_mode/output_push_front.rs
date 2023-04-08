use crate::hlist::tuple::TuplePushFront;

use super::OutputMode;

/// Push outputs to the back of the context.
pub struct OutputPushFront;

impl<C, O> OutputMode<C, O, ()> for OutputPushFront
where
    C: TuplePushFront<O>,
{
    type Output = C::TuplePushFront;

    fn apply(context: C, output: O) -> Self::Output {
        context.push_front(output)
    }
}

