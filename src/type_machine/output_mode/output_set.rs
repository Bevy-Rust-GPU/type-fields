use core::marker::PhantomData;

use crate::{
    t_funk::{tlist::Set, Pointed, Tagged},
    type_machine::input_mode::InputMode,
};

use super::OutputMode;

/// Set each element of the output by type.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputSet<T>(PhantomData<T>);

impl<T, C> InputMode<C, (), ()> for OutputSet<T> {
    fn fetch(_: C) -> () {}
}

impl<T, C, O, P> OutputMode<C, O, P> for OutputSet<T>
where
    C: Set<Tagged<T, O>, P>,
{
    type Output = C;

    fn apply(context: C, output: O) -> Self::Output {
        context.set(Tagged::point(output))
    }
}
