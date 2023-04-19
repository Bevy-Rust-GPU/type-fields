use type_fields_macros::functions;

use crate::t_funk::{Replace, Id, Apply, Fmap, CurriedA, Const};

#[functions]
pub trait Then<F> {
    type Then;

    fn then(self, f: F) -> Self::Then;
}

impl<T, F> Then<F> for T
where
    T: Replace<Id>,
    T::Fmap: Apply<F>,
{
    type Then = <<T as Fmap<CurriedA<Const, Id>>>::Fmap as Apply<F>>::Apply;

    fn then(self, f: F) -> Self::Then {
        self.replace(Id).apply(f)
    }
}

