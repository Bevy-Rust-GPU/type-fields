use crate::hlist::tuple::TuplePushBack;

use super::OutputMode;

/// Push outputs to the back of the context.
pub struct OutputPushBack;

impl<C, O, P> OutputMode<C, O, P> for OutputPushBack
where
    C: TuplePushBack<O, P>,
{
    type Output = C::TuplePushBack;

    fn apply(context: C, output: O) -> Self::Output {
        context.push_back(output)
    }
}

