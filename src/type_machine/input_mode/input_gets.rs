use core::marker::PhantomData;

use crate::t_funk::tlist::{AsTListRef, GetsImpl, TListClone, TupleRef};

use super::{input_ref_gets::InputRefGets, InputMode};

/// Fetch a value and clone it
pub struct InputGets<T>(PhantomData<T>);

impl<'a, T, C, I, P> InputMode<&'a C, I, P> for InputGets<T>
where
    C: TupleRef + 'a,
    C::TupleRef<'a>: GetsImpl<I::AsHListRef, P>,
    I: AsTListRef<'a>,
    I::AsHListRef: TListClone<'a, Cloned = I>,
{
    fn fetch(context: &'a C) -> I {
        InputRefGets::fetch(context).tlist_clone()
    }
}
