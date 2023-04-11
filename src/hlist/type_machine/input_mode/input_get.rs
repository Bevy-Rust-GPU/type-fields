use core::marker::PhantomData;

use crate::{
    functional::{Copointed, Tagged},
    hlist::{
        tuple::{TupleGetImpl, TupleRef},
    },
};

use super::{input_ref_get::InputRefGet, InputMode};

/// Fetch a value and clone it
pub struct InputGet<T>(PhantomData<T>);

impl<'a, T, C, I, P> InputMode<&'a C, I, P> for InputGet<T>
where
    T: 'a,
    C: TupleRef + 'a,
    C::TupleRef<'a>: TupleGetImpl<&'a Tagged<T, I>, P>,
    I: Clone + 'a,
{
    fn fetch(context: &'a C) -> I {
        InputRefGet::fetch(context).clone().copoint()
    }
}
