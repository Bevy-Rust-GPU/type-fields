use core::marker::PhantomData;

use crate::t_funk::tlist::RemoveImpl;

use super::OutputMode;

/// Set each element of the output by type.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputRemove<T>(PhantomData<T>);

impl<C, T, P> OutputMode<C, (), P> for OutputRemove<T>
where
    C: RemoveImpl<T, P>,
{
    type Output = C::Remove;

    fn apply(context: C, _: ()) -> Self::Output {
        context.remove_impl()
    }
}
