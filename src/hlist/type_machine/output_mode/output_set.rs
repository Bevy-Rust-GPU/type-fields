use crate::hlist::tuple::TupleSet;

use super::OutputMode;

/// Set each element of the output by type.
pub struct OutputSet;

impl<C, O, P> OutputMode<C, O, P> for OutputSet
where
    C: TupleSet<O, P>,
{
    type Output = C;

    fn apply(context: C, output: O) -> Self::Output {
        context.set(output)
    }
}

