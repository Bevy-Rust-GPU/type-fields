use crate::hlist::tuple::{TupleGetImpl, TupleRef};

use super::InputMode;

/// Fetch an immutable reference
pub struct InputRefGet;

impl<'a, C, I, P> InputMode<&'a C, I, P> for InputRefGet
where
    C: TupleRef + 'a,
    C::TupleRef<'a>: TupleGetImpl<I, P>,
{
    fn fetch(context: &'a C) -> I {
        context.as_ref().get_impl()
    }
}

