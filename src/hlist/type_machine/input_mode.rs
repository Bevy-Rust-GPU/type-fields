use crate::hlist::tuple::{TupleGet, TupleRef, TupleGets};

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

