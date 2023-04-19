use type_fields_macros::functions;

use crate::t_funk::{Const, CurriedA, Curry, Fmap};

#[functions]
pub trait Replace<T>: Fmap<CurriedA<Const, T>> {
    fn replace(self, t: T) -> Self::Fmap {
        self.fmap(Const.curry_a(t))
    }
}

impl<T, U> Replace<U> for T where T: Fmap<CurriedA<Const, U>> {}
