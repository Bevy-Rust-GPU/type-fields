extern crate alloc;

use crate::hlist::tuple::{
    TupleGet, TupleGets, TuplePushBack, TuplePushFront, TupleRef, TupleSet, TupleSets,
};

/// A single operation that takes input and produces output
pub trait Instruction {
    type Input<'a>
    where
        Self: 'a;
    type InputMode;

    type Output;
    type OutputMode;

    fn exec<'a>(self, input: Self::Input<'a>) -> Self::Output;
}

/// The operation used to fetch an [`Instruction`]'s inputs from its context.
pub trait InputMode<C, I, P> {
    fn fetch(context: C) -> I;
}

/// Fetch no input
pub struct InputNone;

impl<C> InputMode<C, (), ()> for InputNone {
    fn fetch(_: C) -> () {
        ()
    }
}

/// Fetch an immutable reference
pub struct InputRefGet;

impl<'a, C, I, P> InputMode<&'a C, I, P> for InputRefGet
where
    C: TupleRef + 'a,
    C::TupleRef<'a>: TupleGet<I, P>,
{
    fn fetch(context: &'a C) -> I {
        context.tuple_ref().tuple_get()
    }
}

/// Fetch a tuple of immutable references
pub struct InputRefGets;

impl<'a, C, I, P> InputMode<&'a C, I, P> for InputRefGets
where
    C: TupleRef + 'a,
    C::TupleRef<'a>: TupleGets<I, P>,
{
    fn fetch(context: &'a C) -> I {
        context.tuple_ref().tuple_gets()
    }
}

/// Fetch a value and clone it
pub struct InputGet;

impl<'a, C, I, P> InputMode<&'a C, I, P> for InputGet
where
    C: TupleRef + 'a,
    C::TupleRef<'a>: TupleGet<&'a I, P>,
    I: Clone + 'a,
{
    fn fetch(context: &'a C) -> I {
        InputRefGet::fetch(context).clone()
    }
}

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
