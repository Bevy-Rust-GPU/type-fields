use type_fields_macros::functions;

use crate::t_funk::{Apply, Id, Replace};

#[functions]
pub trait Then<F> {
    type Then;

    fn then(self, f: F) -> Self::Then;
}

impl<T, F> Then<F> for T
where
    T: Replace<Id>,
    T::Replace: Apply<F>,
{
    type Then = <<T as Replace<Id>>::Replace as Apply<F>>::Apply;

    fn then(self, f: F) -> Self::Then {
        self.replace(Id).apply(f)
    }
}
