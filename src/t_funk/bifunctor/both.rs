use type_fields_macros::functions;

use crate::t_funk::Closure;

#[functions]
pub trait Both<F1, F2> {
    type Both;

    fn both(self, f1: F1, f2: F2) -> Self::Both;
}

impl<A, B, F1, F2> Both<F1, F2> for (A, B)
where
    F1: Closure<A>,
    F2: Closure<B>,
{
    type Both = (F1::Output, F2::Output);

    fn both(self, f1: F1, f2: F2) -> Self::Both {
        (f1.call(self.0), f2.call(self.1))
    }
}
