use crate::hlist::tuple::{TupleSet, TupleSets, TuplePushBack, TuplePushFront};

/// The operation used to apply the output of an [`Instruction`] to its context.
pub trait OutputMode<C, O, P> {
    type Output;

    fn apply(context: C, output: O) -> Self::Output;
}

/// Discard the output and do not modify the context.
pub struct OutputNone;

impl<C, O> OutputMode<C, O, ()> for OutputNone {
    type Output = C;

    fn apply(context: C, _: O) -> Self::Output {
        context
    }
}

/// Set each element of the output by type.
pub struct OutputSet;

impl<C, O, P> OutputMode<C, O, P> for OutputSet
where
    C: TupleSet<O, P>,
{
    type Output = C;

    fn apply(context: C, output: O) -> Self::Output {
        context.tuple_set(output)
    }
}

/// Set each element of the output by type.
pub struct OutputSets;

impl<C, O, P> OutputMode<C, O, P> for OutputSets
where
    C: TupleSets<O, P>,
{
    type Output = C;

    fn apply(context: C, output: O) -> Self::Output {
        context.tuple_sets(output)
    }
}

/// Push outputs to the back of the context.
pub struct OutputPushBack;

impl<C, O, P> OutputMode<C, O, P> for OutputPushBack
where
    C: TuplePushBack<O, P>,
{
    type Output = C::TuplePushBack;

    fn apply(context: C, output: O) -> Self::Output {
        context.tuple_push_back(output)
    }
}

/// Push outputs to the back of the context.
pub struct OutputPushFront;

impl<C, O> OutputMode<C, O, ()> for OutputPushFront
where
    C: TuplePushFront<O>,
{
    type Output = C::TuplePushFront;

    fn apply(context: C, output: O) -> Self::Output {
        context.tuple_push_front(output)
    }
}
