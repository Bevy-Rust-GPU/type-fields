use crate::macros::functions;

#[functions]
pub trait Second {
    type Second;

    fn second(self) -> Self::Second;
}
