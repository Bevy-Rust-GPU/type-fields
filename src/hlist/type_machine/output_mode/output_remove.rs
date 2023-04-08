use core::marker::PhantomData;

use crate::hlist::tuple::TupleRemoveImpl;

use super::OutputMode;

/// Set each element of the output by type.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputRemove<T>(PhantomData<T>);

impl<C, T, P> OutputMode<C, (), P> for OutputRemove<T>
where
    C: TupleRemoveImpl<T, P>,
{
    type Output = C::TupleRemove;

    fn apply(context: C, _: ()) -> Self::Output {
        context.remove_impl()
    }
}

