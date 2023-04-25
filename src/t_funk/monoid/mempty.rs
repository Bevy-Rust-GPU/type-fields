use crate::macros::functions;

#[functions]
pub trait Mempty {
    type Mempty;

    fn mempty() -> Self::Mempty;
}

