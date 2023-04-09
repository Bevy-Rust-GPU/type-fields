use crate::hlist::tuple::{TupleAsRef, TupleClone, TupleGetsImpl, TupleRef};

use super::{input_ref_gets::InputRefGets, InputMode};

/// Fetch a value and clone it
pub struct InputGets;

impl<'a, C, I, P> InputMode<&'a C, I, P> for InputGets
where
    C: TupleRef + 'a,
    C::TupleRef<'a>: TupleGetsImpl<I::TupleAsRef, P>,
    I: TupleAsRef<'a>,
    I::TupleAsRef: TupleClone<'a, TupleClone = I>,
{
    fn fetch(context: &'a C) -> I {
        InputRefGets::fetch(context).tuple_clone()
    }
}

