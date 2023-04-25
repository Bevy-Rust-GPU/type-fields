use crate::macros::functions;

#[functions]
pub trait First {
    type First;

    fn first(self) -> Self::First;
}

