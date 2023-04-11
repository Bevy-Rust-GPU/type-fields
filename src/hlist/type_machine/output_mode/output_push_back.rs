use core::marker::PhantomData;

use crate::{
    functional::{Tagged, Pointed},
    hlist::{tuple::TuplePushBack, type_machine::input_mode::InputMode},
};

use super::OutputMode;

/// Push outputs to the back of the context.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputPushBack<T>(PhantomData<T>);

impl<T, C> InputMode<C, (), ()> for OutputPushBack<T> {
    fn fetch(_: C) -> () {}
}

impl<T, C, O, P> OutputMode<C, O, P> for OutputPushBack<T>
where
    C: TuplePushBack<Tagged<T, O>, P>,
{
    type Output = C::TuplePushBack;

    fn apply(context: C, output: O) -> Self::Output {
        context.push_back(Pointed::point(output))
    }
}
