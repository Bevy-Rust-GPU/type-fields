use crate::hlist::tuple::{TupleGetImpl, TupleRef};

use super::{input_ref_get::InputRefGet, InputMode};

/// Fetch a value and clone it
pub struct InputGet;

impl<'a, C, I, P> InputMode<&'a C, I, P> for InputGet
where
    C: TupleRef + 'a,
    C::TupleRef<'a>: TupleGetImpl<&'a I, P>,
    I: Clone + 'a,
{
    fn fetch(context: &'a C) -> I {
        InputRefGet::fetch(context).clone()
    }
}

