use crate::t_funk::tlist::Sets;

use super::OutputMode;

/// Set each element of the output by type.
pub struct OutputSets;

impl<C, O, P> OutputMode<C, O, P> for OutputSets
where
    C: Sets<O, P>,
{
    type Output = C;

    fn apply(context: C, output: O) -> Self::Output {
        context.sets(output)
    }
}

