use core::marker::PhantomData;

use crate::{
    t_funk::{tlist::PushBack, Pointed, Tagged},
    type_machine::input_mode::InputMode,
};

use super::OutputMode;

/// Push outputs to the back of the context.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputPushBack<T>(PhantomData<T>);

impl<T, C> InputMode<C, (), ()> for OutputPushBack<T> {
    fn fetch(_: C) -> () {}
}

impl<T, C, O> OutputMode<C, O, ()> for OutputPushBack<T>
where
    C: PushBack<Tagged<T, O>>,
{
    type Output = C::TuplePushBack;

    fn apply(context: C, output: O) -> Self::Output {
        context.push_back(Tagged::point(output))
    }
}
