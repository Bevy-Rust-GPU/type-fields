use crate::macros::functions;

#[functions]
pub trait Foldl<F, Z> {
    type Foldl;

    fn foldl(self, f: F, z: Z) -> Self::Foldl;
}

