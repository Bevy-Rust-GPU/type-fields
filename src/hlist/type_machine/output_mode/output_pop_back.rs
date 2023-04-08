use crate::hlist::tuple::TuplePopBack;

use super::OutputMode;

/// Push outputs to the back of the context.
pub struct OutputPopBack;

impl<C> OutputMode<C, (), ()> for OutputPopBack
where
    C: TuplePopBack,
{
    type Output = C::TuplePopBack;

    fn apply(context: C, _: ()) -> Self::Output {
        context.pop_back()
    }
}

