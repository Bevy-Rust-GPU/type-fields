use crate::t_funk::tlist::{GetsImpl, TupleRef};

use super::InputMode;

/// Fetch a tuple of immutable references
pub struct InputRefGets;

impl<'a, C, I, P> InputMode<&'a C, I, P> for InputRefGets
where
    C: TupleRef + 'a,
    C::TupleRef<'a>: GetsImpl<I, P>,
{
    fn fetch(context: &'a C) -> I {
        context.as_ref().gets_impl()
    }
}

