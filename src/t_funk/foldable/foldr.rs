use crate::macros::functions;

#[functions]
pub trait Foldr<F, Z> {
    type Foldr;

    fn foldr(self, f: F, z: Z) -> Self::Foldr;
}
