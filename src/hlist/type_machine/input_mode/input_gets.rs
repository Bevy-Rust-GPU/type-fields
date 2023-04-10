use core::marker::PhantomData;

use crate::hlist::tuple::{TupleAsRef, TupleClone, TupleGetsImpl, TupleRef};

use super::{input_ref_gets::InputRefGets, InputMode};

/// Fetch a value and clone it
pub struct InputGets<T>(PhantomData<T>);

impl<'a, T, C, I, P> InputMode<&'a C, I, P> for InputGets<T>
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

