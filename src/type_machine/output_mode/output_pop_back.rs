use crate::t_funk::tlist::PopBack;

use super::OutputMode;

/// Push outputs to the back of the context.
pub struct OutputPopBack;

impl<C> OutputMode<C, (), ()> for OutputPopBack
where
    C: PopBack,
{
    type Output = C::PopBack;

    fn apply(context: C, _: ()) -> Self::Output {
        context.pop_back()
    }
}

