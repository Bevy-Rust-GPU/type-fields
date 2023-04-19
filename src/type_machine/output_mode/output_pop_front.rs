use crate::t_funk::tlist::PopFront;

use super::OutputMode;

/// Push outputs to the back of the context.
pub struct OutputPopFront;

impl<C> OutputMode<C, (), ()> for OutputPopFront
where
    C: PopFront,
{
    type Output = C::PopFront;

    fn apply(context: C, _: ()) -> Self::Output {
        context.pop_front()
    }
}

